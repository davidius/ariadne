use tokio::time::timeout;
use std::time::Duration;
use std::thread::sleep;
use crate::log_annotations::*;
use crate::logs::*;
use crate::types::*;
use regex::Regex;
use std::io::Error;
use tokio::io::{AsyncBufReadExt, AsyncRead, BufReader};

/// Reads from a buffer and logs the output
pub async fn log_output_from_reader(
    log_reader: BufReader<impl AsyncRead + std::marker::Unpin>,
    is_process_complete_mutex_wrapper: &MutexWrapper,
    with_logs: bool,
    process_name: &String,
    log_annotations_for_service: &Vec<LogAnnotation>,
    continue_on_log_regex: Option<String>,
    log_type: LogType
) -> Result<(), Error> {
    let mut lines = log_reader.lines();

    let mut result: Result<(), Error> = Ok(());

    loop {    
        let is_process_complete = is_process_complete_mutex_wrapper.get_is_process_complete();

        if is_process_complete {
            break;
        }

        if let Err(_) = timeout(Duration::from_millis(1), lines.next_line()).await {
            continue;
        }

        while let Some(log_line) = lines.next_line().await? {
            if let Ok(mut is_process_complete) = is_process_complete_mutex_wrapper.is_process_complete_mutex.try_lock() {
                if with_logs {
                    log_output(&log_line, &log_type, process_name);
                }

                parse_for_matching_log_annotations(&log_line, log_annotations_for_service.to_owned());

                if continue_on_log_regex.is_some() {
                    let regex_string = format!("{}", continue_on_log_regex.as_ref().unwrap());
                    // TODO: use the static syntax below to prevent re-compiling the regex
                    let re = Regex::new(&regex_string).unwrap(); // TODO: should handle errors (i.e. if provided regex is invalid)
                    if re.is_match(&log_line) {
                        log_continue_to_next_process();
                        *is_process_complete = true;
                        result = Ok(());
                        break;
                    }
                }
            } else {
                // send thread to sleep for a tiny period, while we wait to acquire the lock
                sleep(Duration::from_millis(1));
                break;
            }
        }
        break;
    }

    return result;
}