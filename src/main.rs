mod chef;
mod command_parse;
mod environment;
mod get_config;
mod json_parse;
mod log_annotations;
mod logs;
mod read_from_buffer;
mod run_commands;
mod run_recipe;
mod run_service;
mod services;
mod types;
mod unit_tests;
extern crate clap;
use crate::get_config::create_settings_config_file;
use crate::get_config::get_log_annotations;
use crate::get_config::get_services_config;
use crate::get_config::get_user_config;
use crate::types::*;
use clap::{App, Arg, SubCommand};
use json_parse::{parse_log_annotations_json, parse_services_json, parse_user_json};
use read_from_buffer::*;
use run_recipe::*;
use services::*;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let matches = App::new("ariadne")
        .author("Me, davidmichael4d@gmail.com")
        .version("0.2.0")
        .about("A command line app to automate the command line")
        .subcommand(
            SubCommand::with_name("run")
                .about("Runs a single service")
                .arg(
                    Arg::with_name("service")
                        .short("s")
                        .help("The name of the service to run")
                        .required(false)
                        .index(1),
                ),
        )
        .subcommand(
            SubCommand::with_name("cook").about("Cooks a recipe").arg(
                Arg::with_name("recipe")
                    .short("r")
                    .help("The name of the recipe to cook")
                    .required(false)
                    .index(1),
            ),
        )
        .subcommand(
            SubCommand::with_name("setup").about("Creates config files needed for ariadne to run"),
        )
        .get_matches();

    let services_config = get_services_config();
    let user_config = get_user_config();
    let log_annotations = get_log_annotations();
    let log_annotations_clone = log_annotations.clone();

    if let Some(ref matches) = matches.subcommand_matches("run") {
        let service_name = matches
            .value_of("service")
            .expect("No service name provided");
        let service = get_service_by_name(service_name.to_string(), services_config.services);
        let foreground_str = String::from("foreground");

        let recipe_service = RecipeService {
            name: service.name.to_string(),
            runtype: foreground_str.to_string(),
            continue_on_log_regex: None,
        };

        let recipe_services = vec![recipe_service];

        let recipe = Recipe {
            name: "default".to_string(),
            services: recipe_services,
        };

        cook_recipe(recipe, vec![service], log_annotations_clone.annotations).await;
    } else if let Some(ref matches) = matches.subcommand_matches("cook") {
        let recipe_name = matches.value_of("recipe").expect("No recipe name provided");
        let recipe = get_recipe_by_name(recipe_name.to_string(), services_config.recipes);
        cook_recipe(
            recipe,
            services_config.services,
            log_annotations.annotations,
        )
        .await;
        println!("Recipe {} has been cooked", recipe_name);
    } else if let Some(_) = matches.subcommand_matches("setup") {
        if services_config.is_empty() {
            create_settings_config_file();
        } else {
            println!("Settings file already exists");
        }
    }

    println!("\nAll done! Hope that worked 😅");
    Ok(())
}
