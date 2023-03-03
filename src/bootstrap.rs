use std::fs;

use crate::{config, features, stow};

pub fn bootstrap(config: &config::Config, unstow: bool, want_features: &Vec<features::Feature>) {
    let source = config.source.clone().unwrap_or("./".to_string());
    let ignore = config.ignore.clone().unwrap_or(vec![]);
    let paths = fs::read_dir(source).unwrap();
    let default_target = shellexpand::full(&config.target).unwrap().to_string();

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

        let features_on_file = features::Feature::parse_features(&config.features, &file_name);
        // if no features exist on the file, stow it
        if features_on_file.is_none() {
            match stow::stow(&default_target, &file_name, Some(unstow)) {
                Ok(_) => {}
                Err(e) => {
                    println!("could not stow {}: {}", file_name, e.error);
                }
            }
            return;
        }

        let features_on_file = features_on_file.unwrap();
        let matching_features = features_on_file
            .iter()
            .filter(|feature| want_features.contains(feature))
            .collect::<Vec<_>>();

        if matching_features.len() == 0 {
            return;
        }

        let target_overrides = matching_features
            .iter()
            .map(|feature| feature.target.clone())
            .filter(|target_override| target_override.is_some())
            .map(|target_override| target_override.unwrap())
            .collect::<Vec<String>>();

        if target_overrides.len() > 1 {
            println!(
                "multiple target overrides found for {}; skipping",
                file_name
            );
            return;
        }

        let target = match target_overrides.get(0) {
            Some(target_override) => {
                let target_override = shellexpand::full(target_override).unwrap().to_string();
                let metadata = fs::metadata(&target_override);
                if !metadata.is_ok() || metadata.unwrap().permissions().readonly() {
                    println!(
                        "target override {} does not exist or no permission to write; skipping",
                        target_override
                    );
                    return;
                }
                target_override
            }
            None => default_target.clone(),
        };

        match stow::stow(&target, &file_name, Some(unstow)) {
            Ok(_) => {}
            Err(e) => {
                println!("could not stow {}: {}", file_name, e.error);
            }
        }
    })
}
