use crate::types::*;

pub fn get_task_by_name(task_name: String, all_tasks: Vec<Task>) -> Task {
    let mut tasks_iterator = all_tasks.into_iter();
    let mut task_opt = tasks_iterator.find(|task| task.name == task_name);

    if task_opt.is_some() {
        return task_opt.take().unwrap();
    } else {
        panic!("Could not find that task in the tasks.yaml file!");
    }
}

pub fn get_recipe_by_name(recipe_name: String, all_recipes: Vec<Recipe>) -> Recipe {
    let mut recipes_iterator = all_recipes.into_iter();
    let mut recipe_opt = recipes_iterator.find(|recipe| recipe.name == recipe_name);

    if recipe_opt.is_some() {
        return recipe_opt.take().unwrap();
    } else {
        panic!("Could not find that recipe in the tasks.yaml file!");
    }
}
