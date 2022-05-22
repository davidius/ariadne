mod chef;
mod command_parse;
mod config;
mod environment;
mod log_annotations;
mod logs;
mod read_from_buffer;
mod run_commands;
mod run_recipe;
mod run_task;
mod tasks;
mod types {
    pub mod types;
}
mod tests {
    pub mod unit_tests;
}
mod yaml_parse;
extern crate clap;
use crate::config::create_tasks_config_file;
use crate::config::get_log_annotations;
use crate::config::get_tasks_config;
use crate::config::get_user_config;
use crate::logs::log_list_of_tasks_and_recipes;
use crate::types::types::*;
use clap::{App, Arg, SubCommand};
use read_from_buffer::*;
use run_recipe::*;
use tasks::*;
use yaml_parse::{parse_log_annotations_yaml, parse_tasks_yaml, parse_user_yaml};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let matches = App::new("ariadne")
        .author("Me, davidmichael4d@gmail.com")
        .version("0.2.0")
        .about("A command line app to automate the command line")
        .subcommand(
            SubCommand::with_name("run")
                .about("Runs a single task")
                .arg(
                    Arg::with_name("task")
                        .short("s")
                        .help("The name of the task to run")
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
        .subcommand(SubCommand::with_name("ls").about("Lists all tasks and recipes"))
        .subcommand(
            SubCommand::with_name("setup").about("Creates config files needed for ariadne to run"),
        )
        .get_matches();

    let tasks_config = get_tasks_config();
    let user_config = get_user_config();
    let log_annotations = get_log_annotations();
    let log_annotations_clone = log_annotations.clone();

    if let Some(ref matches) = matches.subcommand_matches("run") {
        let task_name = matches.value_of("task").expect("No task name provided");
        let task = get_task_by_name(task_name.to_string(), tasks_config.tasks);
        let foreground_str = String::from("foreground");

        let recipe_task = RecipeTask {
            name: task.name.to_string(),
            runtype: foreground_str.to_string(),
            continue_on_log_regex: None,
        };

        let recipe_tasks = vec![recipe_task];

        let recipe = Recipe {
            name: "default".to_string(),
            description: None,
            tasks: recipe_tasks,
        };

        cook_recipe(recipe, vec![task], log_annotations_clone.annotations).await;
    } else if let Some(ref matches) = matches.subcommand_matches("cook") {
        let recipe_name = matches.value_of("recipe").expect("No recipe name provided");
        let recipe = get_recipe_by_name(recipe_name.to_string(), tasks_config.recipes);
        cook_recipe(recipe, tasks_config.tasks, log_annotations.annotations).await;
        println!("Recipe {} has been cooked", recipe_name);
    } else if let Some(_) = matches.subcommand_matches("ls") {
        log_list_of_tasks_and_recipes(tasks_config.tasks.clone(), tasks_config.recipes.clone());
    } else if let Some(_) = matches.subcommand_matches("setup") {
        if tasks_config.is_empty() {
            create_tasks_config_file();
        } else {
            println!("You already have ariadne configured. From this point on, you'll have to manually add tasks and recipes to the tasks.yaml file.");
        }
    }

    println!("\nAll done! Hope that worked 😅");
    Ok(())
}
