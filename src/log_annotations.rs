use crate::logs::*;
use crate::types::*;

pub fn parse_for_matching_log_annotations(
    log_line: &String,
    log_annotations_for_task: Vec<LogAnnotation>,
) {
    let annotations: Vec<LogAnnotation> = log_annotations_for_task
        .into_iter()
        .filter(|annotation| log_line.find(annotation.regex.as_str()).is_some())
        .collect();
    annotations
        .iter()
        .for_each(|x| log_matched_annotation(x.to_owned()));
}
