use crate::types::*;
use ansi_term::{Colour, Style};

const ERROR_COLOUR: Colour = Colour::Red;
const GOING_WELL_COLOUR: Colour = Colour::Cyan;
const SERVICE_NAME_COLOUR: Colour = Colour::Purple;
const INTERESTING_COLOUR: Colour = Colour::Blue;

pub fn log_status_update(stage: &String, service_name: &String) {
    println!(
        "=== Service: {}, Stage: {} ===",
        Style::new()
            .bold()
            .fg(SERVICE_NAME_COLOUR)
            .paint(service_name),
        Style::new().bold().fg(GOING_WELL_COLOUR).paint(stage)
    );
}

pub fn log_new_env_variable(env_var_name: &str, env_var_value: &String) {
    println!("  ☀︎ {} = {}", env_var_name, env_var_value);
}

pub fn log_running_command(command: &String) {
    println!(
        "=== Running command: {} ===",
        Style::new().bold().fg(GOING_WELL_COLOUR).paint(command)
    );
}

pub fn log_continue_to_next_process() {
    println!("----------- Ariadne identified the place to continue! --------------------");
}

pub fn log_output(log_line: &String, log_type: &LogType, process_name: &String) {
    match log_type {
        LogType::ProcessStdout => {
            println!(
                "+++ {} from {} : {}",
                Style::new()
                    .bold()
                    .fg(GOING_WELL_COLOUR)
                    .paint(&String::from("stdout")),
                process_name,
                log_line
            );
        }
        LogType::ProcessStderr => {
            println!(
                "--- {} from {} : {}",
                Style::new()
                    .bold()
                    .fg(ERROR_COLOUR)
                    .paint(&String::from("stderr")),
                process_name,
                log_line
            );
        }
    }
}

pub fn log_matched_annotation(log_annotation: LogAnnotation) {
    match log_annotation.annotation_type.as_str() {
        "ERROR" => {
            println!("===========================================================================");
            println!(
                "{}",
                Style::new()
                    .bold()
                    .fg(ERROR_COLOUR)
                    .paint("💡 Ariadne identified a common error:")
            );
            println!("Error: {}", log_annotation.regex);
            println!("Hint: {}", log_annotation.hint);
            if log_annotation.links.len() > 0 {
                println!("Some useful links:");
                log_annotation
                    .links
                    .iter()
                    .for_each(|url| println!(" - {}", url));
            }
            println!("===========================================================================");
        }
        "INFO" => {
            println!("===========================================================================");
            println!(
                "{}",
                Style::new()
                    .bold()
                    .fg(INTERESTING_COLOUR)
                    .paint("💡 Ariadne would like to make an interesting observation:")
            );
            println!("Matched text: {}", log_annotation.regex);
            println!("Hint: {}", log_annotation.hint);
            if log_annotation.links.len() > 0 {
                println!("Some useful links:");
                log_annotation
                    .links
                    .iter()
                    .for_each(|url| println!(" - {}", url));
            }
            println!("===========================================================================");
        }
        &_ => {
            // Do nothing
        }
    }
}
