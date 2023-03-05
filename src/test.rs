#[cfg(test)]
mod tests {
    use crate::features::{features_from_name, Feature};
    fn get_test_features() -> Vec<Feature> {
        vec![
            Feature {
                name: "macos".to_string(),
                slug: "macos".to_string(),
                target: None,
                enabled_command: None,
            },
            Feature {
                name: "linux".to_string(),
                slug: "linux".to_string(),
                target: None,
                enabled_command: Some("true".to_string()),
            },
            Feature {
                name: "windows".to_string(),
                slug: "windows".to_string(),
                target: None,
                enabled_command: Some("[[ 1 == 2 ]]".to_string()),
            },
        ]
    }

    #[test]
    fn test_features_enabled() {
        let all_features = get_test_features();
        assert_eq!(all_features[0].enabled(), true);
        assert_eq!(all_features[1].enabled(), true);
        assert_eq!(all_features[2].enabled(), false);
    }

    #[test]
    fn test_features_from_name() {
        let all_features = get_test_features();
        struct Test {
            name: String,
            expected_output: Option<Vec<Feature>>,
        }

        let tests: Vec<Test> = vec![
            Test {
                name: "{macos,linux}test".to_string(),
                expected_output: Some(vec![all_features[0].clone(), all_features[1].clone()]),
            },
            Test {
                name: "{macos,windows}test".to_string(),
                expected_output: Some(vec![all_features[0].clone(), all_features[2].clone()]),
            },
            Test {
                name: "}macos,linux{test".to_string(),
                expected_output: None,
            },
            Test {
                name: "{macos,linux".to_string(),
                expected_output: None,
            },
            Test {
                name: "foldername".to_string(),
                expected_output: None,
            },
        ];

        for test in tests {
            println!("testing: {}", test.name);
            let result = features_from_name(&all_features, &test.name);
            assert_eq!(result, test.expected_output);
        }
    }
}
