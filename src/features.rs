use serde::Deserialize;

#[derive(Deserialize, Clone, PartialEq, Eq, Debug)]
pub struct Feature {
    pub name: String,
    pub slug: String,
    pub enabled_command: Option<String>,
    pub target: Option<String>,
}

impl Feature {
    pub fn enabled(&self) -> bool {
        if self.enabled_command.is_none() {
            return true;
        }

        let output = std::process::Command::new("sh")
            .arg("-c")
            .arg(format!("eval {} && echo true || echo false", self.enabled_command.clone().unwrap()))
            .output()
            .expect("Failed to execute command");

        let output = String::from_utf8(output.stdout).unwrap_or("".to_string());

        output.trim() == "true"
    }
}

pub fn features_from_name(features: &[Feature], name: &str) -> Option<Vec<Feature>> {
    let start = name.find('{');
    let end = name.find('}');
    
    // if start is none or end is none, return none
    if start.is_none() || end.is_none()  || start.unwrap() > end.unwrap() {
        return None;
    }

    let feature_string = &name[start.unwrap()+1..end.unwrap()];
    let features = feature_string
        .split(',')
        .filter_map(|feature_name| {
            features 
                .iter()
                .find(|f| f.slug == feature_name)
                .cloned()
        })
        .collect::<Vec<Feature>>();

        if features.len() == 0 {
            return None;
        }
        Some(features)
}

pub fn get_target_override(features: &[Feature]) -> Option<String> {
    let target_overrides = features
        .iter()
        .filter_map(|f| f.target.clone())
        .collect::<Vec<String>>();

    if target_overrides.len() > 1 || target_overrides.len() == 0{
        return None;
    }

    Some(target_overrides.get(0).unwrap().clone())
}
