use std::fs;

use crate::{config, features, stow};

fn target_valid(path: &str) -> bool {
    let metadata = fs::metadata(path);
    metadata.is_ok() && metadata.unwrap().permissions().readonly()
}

pub fn bootstrap(config: &config::Config, unstow: bool) {
    // get features that need to be bootstrapped
    let enabled_features = config
        .features
        .iter()
        .filter(|f| f.enabled())
        .map(|f| f.clone())
        .collect::<Vec<features::Feature>>();

    println!(
        "stowing features: {:?}",
        enabled_features
            .iter()
            .map(|f| format!("{} ({})", f.name, f.slug))
            .collect::<Vec<String>>()
    );

    let source = config.source.clone().unwrap_or("./".to_string());
    let paths = fs::read_dir(source).unwrap();
    let default_target = shellexpand::full(&config.target).unwrap().to_string();

    // iterate over all files in the source directory
    paths.for_each(|file| {
        let file = file.unwrap();

        // if the file is not a directory, skip it
        let md = file.metadata().unwrap();
        if !md.is_dir() {
            return;
        }

        let file_name = file.file_name().into_string().unwrap();

        // get features from the file name
        let features = features::features_from_name(&enabled_features, &file_name);
        // if directory has no enabled features, skip it
        if features.is_none() {
            return;
        }

        // get target override from features
        let target_override = features::get_target_override(&features.unwrap());

        let target = match target_override {
            Some(t) => {
                let t = shellexpand::full(&t).unwrap().to_string();
                if !target_valid(&t) {
                    println!(
                        "target directory {} is not valid; permission denied or does not exist",
                        t
                    );
                    return;
                }
                t
            }
            None => default_target.clone(),
        };

        match stow::stow(&target, &file_name, unstow) {
            Ok(_) if unstow => {
                println!("unstowed {} from {}", file_name, target);
            }
            Ok(_) => {
                println!("stowed {} to {}", file_name, target);
            }
            Err(e) => {
                println!("could not stow {}: {}", file_name, e.error);
            }
        }
    })
}
