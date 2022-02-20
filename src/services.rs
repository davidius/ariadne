use crate::types::*;

pub fn get_service_by_name(service_name: String, all_services: Vec<Service>) -> Service {
    let mut services_iterator = all_services.into_iter();
    let mut service_opt = services_iterator.find(|service| service.name == service_name);

    if service_opt.is_some() {
        return service_opt.take().unwrap();
    } else {
        panic!("Could not find that service in the services.json file!");
    }
}

pub fn get_recipe_by_name(recipe_name: String, all_recipes: Vec<Recipe>) -> Recipe {
    let mut recipes_iterator = all_recipes.into_iter();
    let mut recipe_opt = recipes_iterator.find(|recipe| recipe.name == recipe_name);

    if recipe_opt.is_some() {
        return recipe_opt.take().unwrap();
    } else {
        panic!("Could not find that recipe in the services.json file!");
    }
}
