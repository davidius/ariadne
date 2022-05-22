use crate::types::*;
use tokio::sync::mpsc::Receiver;

pub async fn chef_coordination_task(recipe: Recipe, mut receiver: Receiver<RecipeCommand>) {
    let mut chef_running_tasks: Vec<RunningTask> = Vec::new();
    let recipe_tasks = recipe.tasks;

    while let Some(cmd) = receiver.recv().await {
        match cmd {
            RecipeCommand::GetRunningTaskByName { task_name, resp } => {
                let running_task = chef_running_tasks
                    .iter()
                    .find(|task| task.task_name == task_name);
                match running_task {
                    Some(task) => {
                        resp.send(Some(task.clone()));
                    }
                    None => {
                        resp.send(None);
                    }
                }
            }
            RecipeCommand::SetTaskStatus {
                task_name,
                status,
                resp,
            } => {
                let running_task = chef_running_tasks
                    .iter()
                    .find(|task: &&RunningTask| task.task_name == task_name);
                match running_task {
                    Some(task) => {
                        let task_clone = task.clone();
                        let updated_task = RunningTask {
                            status: status,
                            ..task_clone
                        };
                        let running_task_index = chef_running_tasks
                            .iter()
                            .position(|task| task.task_name == task_name)
                            .unwrap();
                        chef_running_tasks[running_task_index] = updated_task;
                        resp.send(());
                    }
                    None => {
                        // TODO: handle this error
                        resp.send(());
                    }
                }
            }
            RecipeCommand::GetTaskStatusByRecipeIndex {
                recipe_index_opt,
                resp,
            } => {
                if let Some(recipe_index) = recipe_index_opt {
                    let running_task = chef_running_tasks.iter().find(|task: &&RunningTask| {
                        task.task_name == recipe_tasks[recipe_index].name
                    });
                    match running_task {
                        Some(task) => {
                            resp.send(task.status.clone());
                        }
                        None => {
                            resp.send(TaskStatus::NotStarted);
                        }
                    }
                } else {
                    resp.send(TaskStatus::NotStarted);
                }
            }
            RecipeCommand::GetIsTaskSupersededByRecipeIndex {
                recipe_index_opt,
                resp,
            } => {
                if let Some(recipe_index) = recipe_index_opt {
                    let running_task = chef_running_tasks.iter().find(|task: &&RunningTask| {
                        task.task_name == recipe_tasks[recipe_index].name
                    });
                    match running_task {
                        Some(task) => {
                            resp.send(task.is_superseded);
                        }
                        None => {
                            resp.send(false);
                        }
                    }
                } else {
                    resp.send(false);
                }
            }
            RecipeCommand::SetIsTaskSuperseded {
                task_name,
                is_superseded,
                resp,
            } => {
                let running_task = &mut chef_running_tasks
                    .iter()
                    .find(|task: &&RunningTask| task.task_name == task_name);
                match running_task {
                    Some(task) => {
                        let task_clone = task.clone();
                        let updated_task = RunningTask {
                            is_superseded: is_superseded,
                            ..task_clone
                        };
                        let running_task_index = chef_running_tasks
                            .iter()
                            .position(|task| task.task_name == task_name)
                            .unwrap();
                        chef_running_tasks[running_task_index] = updated_task;
                        resp.send(());
                    }
                    None => {
                        resp.send(());
                    }
                }
            }
            RecipeCommand::AddRunningTask { running_task } => {
                chef_running_tasks.push(running_task);
            }
        }
    }
}
