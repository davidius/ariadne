use crate::chef::*;
use crate::environment::*;
use crate::run_service::*;
use crate::services::*;
use crate::types::*;
use crate::watch_logs_for_service;
use std::iter::*;
use tokio::sync::mpsc;
use tokio::sync::oneshot;
use tokio::task::JoinHandle;

pub async fn cook_recipe(
    recipe: Recipe,
    all_services: Vec<Service>,
    log_annotations: Vec<LogAnnotation>,
) {
    let (chef_sender, rx) = mpsc::channel(32);
    let chef_sender_clone = chef_sender.clone();

    let recipe_for_chef = recipe.clone();

    // Spawn a task that will use a channel to communicate with other tasks and coordinate the recipe.
    tokio::spawn(async move {
        chef_coordination_task(recipe_for_chef, rx).await;
    });

    let mut all_tasks: Vec<JoinHandle<()>> = Vec::new();

    for recipe_service in recipe.services.clone().into_iter() {
        let log_annotations_clone = log_annotations.clone();
        let all_services_clone = all_services.clone();

        let service_name = recipe_service.name.clone();
        let service_name_clone = service_name.clone();
        let runtype = recipe_service.runtype.clone();
        let continue_on_log_regex = recipe_service.continue_on_log_regex.clone();
        let continue_on_log_regex_clone = continue_on_log_regex.clone();
        let continue_on_log_regex_clone_2 = continue_on_log_regex_clone.clone();

        let service = get_service_by_name(service_name, all_services_clone.clone());

        let recipe_clone = recipe.clone();

        // Identify the index of this service in the recipe
        let service_recipe_index = recipe_clone
            .services
            .iter()
            .position(|x| x.name == service_name_clone)
            .unwrap();

        let chef_sender_clone = chef_sender_clone.clone();

        // For each service in the recipe, spin up an asynchronous green thread that starts the service when it is ready
        let task_handle = tokio::spawn(async move {
            loop {
                let (resp_tx, resp_rx) = oneshot::channel();
                let (resp_tx2, resp_rx2) = oneshot::channel();

                let prev_service_recipe_index = service_recipe_index.checked_sub(1);

                // Send a command to the chef task to find the status of the service just before this one in the recipe
                chef_sender_clone
                    .send(RecipeCommand::GetServiceStatusByRecipeIndex {
                        recipe_index_opt: prev_service_recipe_index,
                        resp: resp_tx,
                    })
                    .await
                    .unwrap();
                let prev_status = resp_rx.await.unwrap();

                // Send a command to the chef task to find out if the previous service is currently logging
                chef_sender_clone
                    .send(RecipeCommand::GetIsServiceSupersededByRecipeIndex {
                        recipe_index_opt: prev_service_recipe_index,
                        resp: resp_tx2,
                    })
                    .await
                    .unwrap();
                let prev_is_superseded = resp_rx2.await.unwrap();

                if service_recipe_index == 0
                    || (prev_status == ServiceStatus::Complete
                        || prev_status == ServiceStatus::Running && prev_is_superseded)
                {
                    // The previous service has finished running successfully.
                    // Now we can start this service.
                    let log_annotations_for_service: Vec<LogAnnotation> = log_annotations_clone
                        .into_iter()
                        .filter(|err| {
                            err.affected_services
                                .contains(&service_name_clone.to_string())
                        })
                        .collect();

                    let log_annotations_for_service_clone = log_annotations_for_service.clone();

                    prepare_env(&service);

                    let running_service = run_service(
                        &service,
                        runtype,
                        log_annotations_for_service,
                        continue_on_log_regex_clone,
                    )
                    .await;

                    // Send a command to the chef task to add this service to the list of running services
                    chef_sender_clone
                        .send(RecipeCommand::AddRunningService {
                            running_service: running_service.clone(),
                        })
                        .await
                        .unwrap();

                    let service_name = &running_service.clone().service_name;

                    watch_logs_for_service(
                        chef_sender_clone.clone(),
                        service_name,
                        true,
                        log_annotations_for_service_clone,
                        continue_on_log_regex_clone_2,
                    )
                    .await;

                    break;
                } else {
                    tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
                }
            }
        });
        all_tasks.push(task_handle);
    }

    let mut results = Vec::with_capacity(all_tasks.len());
    for task in all_tasks {
        results.push(task.await.unwrap());
    }
}
