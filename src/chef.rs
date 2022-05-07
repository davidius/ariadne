use crate::types::*;
use tokio::sync::mpsc::Receiver;

pub async fn chef_coordination_task(recipe: Recipe, mut receiver: Receiver<RecipeCommand>) {
    let mut chef_running_services: Vec<RunningService> = Vec::new();
    let recipe_services = recipe.services;

    while let Some(cmd) = receiver.recv().await {
        match cmd {
            RecipeCommand::GetRunningServiceByName { service_name, resp } => {
                let running_service = chef_running_services
                    .iter()
                    .find(|service| service.service_name == service_name);
                match running_service {
                    Some(service) => {
                        resp.send(Some(service.clone()));
                    }
                    None => {
                        resp.send(None);
                    }
                }
            }
            RecipeCommand::SetServiceStatus {
                service_name,
                status,
                resp,
            } => {
                let running_service = chef_running_services
                    .iter()
                    .find(|service: &&RunningService| service.service_name == service_name);
                match running_service {
                    Some(service) => {
                        let service_clone = service.clone();
                        let updated_service = RunningService {
                            status: status,
                            ..service_clone
                        };
                        let running_service_index = chef_running_services
                            .iter()
                            .position(|service| service.service_name == service_name)
                            .unwrap();
                        chef_running_services[running_service_index] = updated_service;
                        resp.send(());
                    }
                    None => {
                        // TODO: handle this error
                        resp.send(());
                    }
                }
            }
            RecipeCommand::GetServiceStatusByRecipeIndex {
                recipe_index_opt,
                resp,
            } => {
                if let Some(recipe_index) = recipe_index_opt {
                    let running_service =
                        chef_running_services
                            .iter()
                            .find(|service: &&RunningService| {
                                service.service_name == recipe_services[recipe_index].name
                            });
                    match running_service {
                        Some(service) => {
                            resp.send(service.status.clone());
                        }
                        None => {
                            resp.send(ServiceStatus::NotStarted);
                        }
                    }
                } else {
                    resp.send(ServiceStatus::NotStarted);
                }
            }
            RecipeCommand::GetIsServiceSupersededByRecipeIndex {
                recipe_index_opt,
                resp,
            } => {
                if let Some(recipe_index) = recipe_index_opt {
                    let running_service =
                        chef_running_services
                            .iter()
                            .find(|service: &&RunningService| {
                                service.service_name == recipe_services[recipe_index].name
                            });
                    match running_service {
                        Some(service) => {
                            resp.send(service.is_superseded);
                        }
                        None => {
                            resp.send(false);
                        }
                    }
                } else {
                    resp.send(false);
                }
            }
            RecipeCommand::SetIsServiceSuperseded {
                service_name,
                is_superseded,
                resp,
            } => {
                let running_service = &mut chef_running_services
                    .iter()
                    .find(|service: &&RunningService| service.service_name == service_name);
                match running_service {
                    Some(service) => {
                        let service_clone = service.clone();
                        let updated_service = RunningService {
                            is_superseded: is_superseded,
                            ..service_clone
                        };
                        let running_service_index = chef_running_services
                            .iter()
                            .position(|service| service.service_name == service_name)
                            .unwrap();
                        chef_running_services[running_service_index] = updated_service;
                        resp.send(());
                    }
                    None => {
                        resp.send(());
                    }
                }
            }
            RecipeCommand::AddRunningService { running_service } => {
                chef_running_services.push(running_service);
            }
        }
    }
}
