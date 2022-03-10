use crate::environment::*;
use crate::run_service::*;
use crate::services::*;
use crate::types::*;
use std::iter::*;

pub async fn cook_recipe(
    recipe: Recipe,
    all_services: Vec<Service>,
    log_annotations: &Vec<LogAnnotation>,
) {
    let iterator = recipe.services.into_iter();
    for recipe_service in iterator {
        let service_name = recipe_service.name;
        let service_name_clone = service_name.clone();
        let runtype = recipe_service.runtype;
        let all_services_clone = all_services.to_owned();
        let service = get_service_by_name(service_name, all_services_clone);
        let continue_on_log_regex = recipe_service.continue_on_log_regex;

        prepare_env(&service);

        let log_annotations_for_service: Vec<LogAnnotation> = log_annotations
            .clone()
            .into_iter()
            .filter(|err| {
                err.affected_services
                    .contains(&service_name_clone.to_string())
            })
            .collect();

        run_service(
            &service,
            runtype,
            log_annotations_for_service,
            continue_on_log_regex,
        ).await;
    }
}
