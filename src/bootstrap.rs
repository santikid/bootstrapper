use std::fs;

use crate::{config, features, stow};

pub fn bootstrap(config: &config::Config, unstow: bool, want_features: &Vec<features::Feature>) {
    let source = config.source.clone().unwrap_or("./".to_string());
    let ignore = config.ignore.clone().unwrap_or(vec![]);
    let paths = fs::read_dir(source).unwrap();
    let target = shellexpand::full(&config.target).unwrap().to_string();

    paths.for_each(|file| {
        let file = file.unwrap();

        let md = file.metadata().unwrap();
        if !md.is_dir() {
            return;
        }

        let file_name = file.file_name().into_string().unwrap();
        if ignore.contains(&file_name) {
            return;
        }

        let features = features::Feature::parse_features(&config.features, &file_name);
        if features.is_none() {
            match stow::stow(&target, &file_name, Some(unstow)) {
                Ok(_) => {}
                Err(e) => {
                    println!("could not stow {}: {}", file_name, e.error);
                }
            }
            return;
        }
        let features = features.unwrap();
        let enabled = features
            .iter()
            .map(|feature| want_features.contains(feature))
            .reduce(|acc, curr| acc || curr);
        if enabled.unwrap() {
            match stow::stow(&target, &file_name, Some(unstow)) {
                Ok(_) => {}
                Err(e) => {
                    println!("could not stow {}: {}", file_name, e.error);
                }
            }
        }
    })
}
