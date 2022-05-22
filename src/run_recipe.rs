use crate::chef::*;
use crate::environment::*;
use crate::run_task::*;
use crate::tasks::*;
use crate::types::*;
use crate::watch_logs_for_task;
use std::iter::*;
use tokio::sync::mpsc;
use tokio::sync::oneshot;
use tokio::task::JoinHandle;

pub async fn cook_recipe(
    recipe: Recipe,
    all_tasks: Vec<Task>,
    log_annotations: Vec<LogAnnotation>,
) {
    let (chef_sender, rx) = mpsc::channel(32);
    let chef_sender_clone = chef_sender.clone();

    let recipe_for_chef = recipe.clone();

    // Spawn a task that will use a channel to communicate with other tasks and coordinate the recipe.
    tokio::spawn(async move {
        chef_coordination_task(recipe_for_chef, rx).await;
    });

    let mut running_task_handles: Vec<JoinHandle<()>> = Vec::new();

    for recipe_task in recipe.tasks.clone().into_iter() {
        let log_annotations_clone = log_annotations.clone();
        let all_tasks_clone = all_tasks.clone();

        let task_name = recipe_task.name.clone();
        let task_name_clone = task_name.clone();
        let runtype = recipe_task.runtype.clone();
        let continue_on_log_regex = recipe_task.continue_on_log_regex.clone();
        let continue_on_log_regex_clone = continue_on_log_regex.clone();
        let continue_on_log_regex_clone_2 = continue_on_log_regex_clone.clone();

        let task = get_task_by_name(task_name, all_tasks_clone.clone());

        let recipe_clone = recipe.clone();

        // Identify the index of this task in the recipe
        let task_recipe_index = recipe_clone
            .tasks
            .iter()
            .position(|x| x.name == task_name_clone)
            .unwrap();

        let chef_sender_clone = chef_sender_clone.clone();

        // For each task in the recipe, spin up an asynchronous green thread that starts the task when it is ready
        let task_handle = tokio::spawn(async move {
            loop {
                let (resp_tx, resp_rx) = oneshot::channel();
                let (resp_tx2, resp_rx2) = oneshot::channel();

                let prev_task_recipe_index = task_recipe_index.checked_sub(1);

                // Send a command to the chef task to find the status of the task just before this one in the recipe
                chef_sender_clone
                    .send(RecipeCommand::GetTaskStatusByRecipeIndex {
                        recipe_index_opt: prev_task_recipe_index,
                        resp: resp_tx,
                    })
                    .await
                    .unwrap();
                let prev_status = resp_rx.await.unwrap();

                // Send a command to the chef task to find out if the previous task is currently logging
                chef_sender_clone
                    .send(RecipeCommand::GetIsTaskSupersededByRecipeIndex {
                        recipe_index_opt: prev_task_recipe_index,
                        resp: resp_tx2,
                    })
                    .await
                    .unwrap();
                let prev_is_superseded = resp_rx2.await.unwrap();

                if task_recipe_index == 0
                    || (prev_status == TaskStatus::Complete
                        || prev_status == TaskStatus::Running && prev_is_superseded)
                {
                    // The previous task has finished running successfully.
                    // Now we can start this task.
                    let log_annotation_for_task: Vec<LogAnnotation> = log_annotations_clone
                        .into_iter()
                        .filter(|err| err.affected_tasks.contains(&task_name_clone.to_string()))
                        .collect();

                    let log_annotation_for_task_clone = log_annotation_for_task.clone();

                    prepare_env(&task);

                    let running_task = run_task(
                        &task,
                        runtype,
                        log_annotation_for_task,
                        continue_on_log_regex_clone,
                    )
                    .await;

                    // Send a command to the chef task to add this task to the list of running tasks
                    chef_sender_clone
                        .send(RecipeCommand::AddRunningTask {
                            running_task: running_task.clone(),
                        })
                        .await
                        .unwrap();

                    let task_name = &running_task.clone().task_name;

                    watch_logs_for_task(
                        chef_sender_clone.clone(),
                        task_name,
                        true,
                        log_annotation_for_task_clone,
                        continue_on_log_regex_clone_2,
                    )
                    .await;

                    break;
                } else {
                    tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
                }
            }
        });
        running_task_handles.push(task_handle);
    }

    let mut results = Vec::with_capacity(all_tasks.len());
    for task in running_task_handles {
        results.push(task.await.unwrap());
    }
}
