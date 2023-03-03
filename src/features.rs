use serde::Deserialize;

#[derive(Deserialize, Clone, PartialEq, Eq, Debug)]
pub struct Feature {
    pub name: String,
    pub slug: String,
    pub enabled_command: Option<String>,
}

impl Feature {
    pub fn parse_features(all_features: &Vec<Feature>, name: &String) -> Option<Vec<Feature>> {
        let start = name.find("{");
        let end = name.find("}");
        if start.is_none() || end.is_none() {
            return None;
        }

        let feature_string = &name[(start.unwrap() + 1)..end.unwrap()];
        let feature_names: Vec<&str> = feature_string.split(",").collect();

        let mut features: Vec<Feature> = vec![];

        feature_names.iter().for_each(|feature_name| {
            let feature = all_features.iter().find(|f| f.slug == *feature_name);
            if feature.is_none() {
                println!("Error: Feature {} not found", feature_name);
            }
            features.push(feature.unwrap().clone())
        });
        Some(features)
    }
    pub fn want_features(all_features: &Vec<Feature>) -> Vec<Feature> {
        let mut features: Vec<Feature> = vec![];
        all_features.iter().for_each(|feature| {
            if feature.enabled_command.is_none() {
                features.push(feature.clone());
            }
            if feature.enabled_command.is_some()
                && feature_enabled(&feature.enabled_command.clone().unwrap())
            {
                features.push(feature.clone());
            }
        });
        features
    }
}

fn feature_enabled(command: &str) -> bool {
    let output = std::process::Command::new("sh")
        .arg("-c")
        .arg(format!("eval {} && echo true || echo false", command))
        .output()
        .expect("Failed to execute command");
    let output = String::from_utf8(output.stdout).unwrap_or("".to_string());
    output.trim() == "true"
}
