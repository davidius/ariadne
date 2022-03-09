mod command_parse;
mod environment;
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
use crate::types::*;
use clap::{App, Arg, SubCommand};
use dirs::home_dir;
use environment::*;
use json_parse::{parse_log_annotations_json, parse_services_json, parse_user_json};
use run_recipe::*;
use run_service::*;
use services::*;
use std::fs;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let matches = App::new("ariadne")
        .author("Me, davidmichael4d@gmail.com")
        .version("0.2.0")
        .about("Helps developers automate away all the boring/time-consuming stuff.")
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

    // BEGIN put below into separate function (which retrieves all the things before starting to process) -----------------------------------
    let user_home_path_buf = home_dir().unwrap();
    let user_home = user_home_path_buf.to_str().unwrap();

    let service_json_path = format!("{}/.ariadne/services.json", user_home);
    let user_json_path = format!("{}/.ariadne/user.json", user_home);
    let log_annotations_json_path = format!("{}/.ariadne/log_annotations.json", user_home);

    let raw_services_json = fs::read_to_string(&service_json_path);
    let raw_user_json = fs::read_to_string(&user_json_path);
    let raw_log_annotations_json = fs::read_to_string(&log_annotations_json_path);

    let raw_services_json = match raw_services_json {
        Ok(raw_json) => raw_json,
        Err(_) => {
            panic!("Hmm. Couldn't find a services config file. This should be stored in {}. You can create one with `ariadne setup`.", &service_json_path);
        }
    };

    let raw_user_json = match raw_user_json {
        Ok(raw_json) => raw_json,
        Err(_) => {
            panic!("Hmm. Couldn't find a user config file. This should be stored in {}. You can create one with `ariadne setup`.", &user_json_path);
        }
    };

    // TODO: should not fail if the file does not exist (not necessary to run)
    let raw_log_annotations_json = match raw_log_annotations_json {
        Ok(raw_json) => raw_json,
        Err(_) => {
            panic!("Hmm. Couldn't find a log annotations file. This should be stored in {}. You can create one with `ariadne setup`.", &log_annotations_json_path);
        }
    };

    let user_config = parse_user_json(raw_user_json);
    let services_config = parse_services_json(raw_services_json);
    let log_annotations = parse_log_annotations_json(raw_log_annotations_json);

    // END ------------------------------------------------------------------------------------------

    if let Some(ref matches) = matches.subcommand_matches("run") {
        let service_name = matches.value_of("service").unwrap();
        let service = get_service_by_name(service_name.to_string(), services_config.services);
        let foreground_str = String::from("foreground");
        prepare_env(&service, &user_config);

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
        let recipe_name = matches.value_of("recipe").unwrap();
        let recipe = get_recipe_by_name(recipe_name.to_string(), services_config.recipes);
        cook_recipe(
            recipe,
            services_config.services,
            user_config,
            &log_annotations.annotations,
        ).await;
    }

    println!("\nAll done! Hope that worked 😅");
    Ok(())
}
