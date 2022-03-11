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
use crate::get_config::get_log_annotations;
use crate::get_config::get_user_config;
use crate::get_config::get_services_config;
use crate::types::*;
use clap::{App, Arg, SubCommand};
use environment::*;
use json_parse::{parse_log_annotations_json, parse_services_json, parse_user_json};
use run_recipe::*;
use run_service::*;
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
        .subcommand(SubCommand::with_name("list").about("Does some stuff"))
        // .after_help(
        //   "Longer explanation to appear after the options when \
        //              displaying the help information from --help or -h",
        // )
        .get_matches();

    let services_config = get_services_config();
    let user_config = get_user_config();
    let log_annotations = get_log_annotations();

    if let Some(ref matches) = matches.subcommand_matches("run") {
        let service_name = matches.value_of("service").expect("No service name provided");
        let service = get_service_by_name(service_name.to_string(), services_config.services);
        let foreground_str = String::from("foreground");
        prepare_env(&service);

        let log_annotations_for_service: Vec<LogAnnotation> = log_annotations
            .annotations
            .into_iter()
            .filter(|err| err.affected_services.contains(&service_name.to_string()))
            .collect();

        run_service(
            &service,
            foreground_str,
            log_annotations_for_service,
            None,
        ).await;
    } else if let Some(ref matches) = matches.subcommand_matches("cook") {
        let recipe_name = matches.value_of("recipe").expect("No recipe name provided");
        let recipe = get_recipe_by_name(recipe_name.to_string(), services_config.recipes);
        cook_recipe(
            recipe,
            services_config.services,
            &log_annotations.annotations,
        ).await;
    }

    println!("\nAll done! Hope that worked 😅");
    Ok(())
}
