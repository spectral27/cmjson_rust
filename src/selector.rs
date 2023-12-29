use std::process;

use crate::{file_read_write, id_generator};
use crate::distribution::Distribution;

pub fn select_all() {
    let distributions_json = file_read_write::read_from_file();
    println!("{}", distributions_json);
}

pub fn select(arg0: &str) {
    let id = match arg0.parse::<i32>() {
        Ok(result) => result,
        Err(e) => panic!("{:?}", e)
    };

    let distributions_json = file_read_write::read_from_file();

    let distributions: Vec<Distribution> = match serde_json::from_str(&distributions_json) {
        Ok(result) => result,
        Err(e) => panic!("Error mapping json to structs: {:?}", e)
    };

    let distribution = distributions.iter()
        .find(|dist| dist.id() == id);

    if distribution.is_some() {
        let distribution_json = match serde_json::to_string_pretty(distribution.unwrap()) {
            Ok(result) => result,
            Err(e) => panic!("Error mapping struct to json: {}", e)
        };
        println!("{}", distribution_json);
    } else {
        println!("Distribution with id {} not found.", id);
    }
}

pub fn insert(arg0: &str) {
    let distributions_json = file_read_write::read_from_file();

    let mut distributions: Vec<Distribution> = match serde_json::from_str(&distributions_json) {
        Ok(result) => result,
        Err(e) => panic!("Error mapping json to structs: {:?}", e)
    };

    if arg0.starts_with("{") {
        let mut new_distribution: Distribution = match serde_json::from_str(&arg0) {
            Ok(result) => result,
            Err(e) => panic!("Error mapping json to struct: {}", e)
        };

        new_distribution.set_id(id_generator::get_new_id(&distributions));

        distributions.push(new_distribution);

        file_read_write::write_to_file(&distributions);

        println!("Insert successful.");
    }
}

pub fn update(arg0: &str) {
    let distributions_json = file_read_write::read_from_file();

    let mut distributions: Vec<Distribution> = match serde_json::from_str(&distributions_json) {
        Ok(result) => result,
        Err(e) => panic!("Error mapping json to structs: {:?}", e)
    };

    if arg0.starts_with("{") {
        let updated: Distribution = match serde_json::from_str(&arg0) {
            Ok(result) => result,
            Err(e) => panic!("Error mapping json to struct: {}", e)
        };

        if updated.id() == 0 {
            println!("Distribution to update id must be present.");
            process::exit(1);
        }

        if updated.name().is_none() {
            println!("Distribution to update name must be present.");
            process::exit(1);
        }

        let to_update_index = distributions.iter()
            .position(|d| d.id() == updated.id());

        if to_update_index.is_some() {
            let to_update_index = to_update_index.unwrap();

            if distributions[to_update_index].name().ne(updated.name()) {
                distributions[to_update_index].set_name(updated.name().clone());

                file_read_write::write_to_file(&distributions);

                println!("Update successful.");
            }
        } else {
            println!("Distribution with id {} not found", updated.id());
        }
    }
}

pub fn delete(arg0: &str) {
    let id = match arg0.parse::<i32>() {
        Ok(result) => result,
        Err(e) => panic!("{:?}", e)
    };

    let distributions_json = file_read_write::read_from_file();

    let mut distributions: Vec<Distribution> = match serde_json::from_str(&distributions_json) {
        Ok(result) => result,
        Err(e) => panic!("Error mapping json to structs: {:?}", e)
    };

    let distribution_to_delete_index = distributions.iter()
        .position(|d| d.id() == id);

    if distribution_to_delete_index.is_some() {
        let distribution_to_delete_index = distribution_to_delete_index.unwrap();

        distributions.remove(distribution_to_delete_index);

        file_read_write::write_to_file(&distributions);

        println!("Delete successful.");
    } else {
        println!("Distribution with id {} not found.", id);
    }
}
