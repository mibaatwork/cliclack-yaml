use cliclack_yaml::{PromptStep, PromptStepType, SelectItem, SelectItems};

#[cfg(test)]
mod select_tests {
    use super::*;

    #[test]
    fn test_select_step_creation() {
        let items = vec![
            SelectItem {
                value: "react".to_string(),
                label: "React".to_string(),
                hint: Some("JavaScript library".to_string()),
                execute_yaml: None,
            },
            SelectItem {
                value: "vue".to_string(),
                label: "Vue.js".to_string(),
                hint: Some("Progressive framework".to_string()),
                execute_yaml: None,
            },
            SelectItem {
                value: "angular".to_string(),
                label: "Angular".to_string(),
                hint: None,
                execute_yaml: None,
            },
        ];

        let step = PromptStep {
            step_type: PromptStepType::Select {
                prompt: "Choose a framework:".to_string(),
                items: SelectItems::Static(items.clone()),
                output: Some("selected_framework".to_string()),
                initial: Some("react".to_string()),
            },
            output: None,
            step_name: None,
            condition: None,
        };

        // Verify the step was created correctly
        match &step.step_type {
            PromptStepType::Select { prompt, items, output, initial } => {
                assert_eq!(prompt, "Choose a framework:");
                match items {
                    SelectItems::Static(static_items) => {
                        assert_eq!(static_items.len(), 3);
                        assert_eq!(static_items[0].value, "react");
                        assert_eq!(static_items[0].label, "React");
                        assert_eq!(static_items[0].hint, Some("JavaScript library".to_string()));
                    },
                    _ => panic!("Expected Static items"),
                }
                assert_eq!(output, &Some("selected_framework".to_string()));
                assert_eq!(initial, &Some("react".to_string()));
            },
            _ => panic!("Expected Select step type"),
        }
    }

    #[test]
    fn test_select_yaml_parsing() {
        let yaml_content = r#"
- type: select
  prompt: "Choose your deployment target:"
  output: "deployment"
  initial: "staging"
  items:
    - value: "development"
      label: "Development"
      hint: "Local development environment"
    - value: "staging"
      label: "Staging"
      hint: "Pre-production environment"
    - value: "production"
      label: "Production"
      hint: "Live environment"
"#;

        let steps: Vec<PromptStep> = serde_saphyr::from_str(yaml_content).unwrap();
        assert_eq!(steps.len(), 1);

        match &steps[0].step_type {
            PromptStepType::Select { prompt, items, output, initial } => {
                assert_eq!(prompt, "Choose your deployment target:");
                match items {
                    SelectItems::Static(static_items) => {
                        assert_eq!(static_items.len(), 3);
                        assert_eq!(static_items[0].value, "development");
                        assert_eq!(static_items[0].label, "Development");
                        assert_eq!(static_items[0].hint, Some("Local development environment".to_string()));
                        assert_eq!(static_items[1].value, "staging");
                        assert_eq!(static_items[1].label, "Staging");
                    },
                    _ => panic!("Expected Static items"),
                }
                assert_eq!(output, &Some("deployment".to_string()));
                assert_eq!(initial, &Some("staging".to_string()));
            },
            _ => panic!("Expected Select step type"),
        }
    }

    #[test]
    fn test_select_yaml_without_initial() {
        let yaml_content = r#"
- type: select
  prompt: "Pick an option:"
  output: "choice"
  items:
    - value: "option1"
      label: "Option 1"
    - value: "option2"
      label: "Option 2"
      hint: "Alternative choice"
"#;

        let steps: Vec<PromptStep> = serde_saphyr::from_str(yaml_content).unwrap();
        assert_eq!(steps.len(), 1);

        match &steps[0].step_type {
            PromptStepType::Select { prompt, items, output, initial } => {
                assert_eq!(prompt, "Pick an option:");
                match items {
                    SelectItems::Static(static_items) => {
                        assert_eq!(static_items.len(), 2);
                        assert_eq!(static_items[0].hint, None);
                        assert_eq!(static_items[1].hint, Some("Alternative choice".to_string()));
                    },
                    _ => panic!("Expected Static items"),
                }
                assert_eq!(output, &Some("choice".to_string()));
                assert_eq!(initial, &None);
            },
            _ => panic!("Expected Select step type"),
        }
    }

    #[test]
    fn test_select_items_serialization() {
        let item = SelectItem {
            value: "test_value".to_string(),
            label: "Test Label".to_string(),
            hint: Some("Test hint".to_string()),
            execute_yaml: None,
        };

        let serialized = serde_saphyr::to_string(&item).unwrap();
        let deserialized: SelectItem = serde_saphyr::from_str(&serialized).unwrap();

        assert_eq!(deserialized.value, "test_value");
        assert_eq!(deserialized.label, "Test Label");
        assert_eq!(deserialized.hint, Some("Test hint".to_string()));
    }

    #[test]
    fn test_select_items_without_hint() {
        let item = SelectItem {
            value: "test_value".to_string(),
            label: "Test Label".to_string(),
            hint: None,
            execute_yaml: None,
        };

        let serialized = serde_saphyr::to_string(&item).unwrap();
        let deserialized: SelectItem = serde_saphyr::from_str(&serialized).unwrap();

        assert_eq!(deserialized.value, "test_value");
        assert_eq!(deserialized.label, "Test Label");
        assert_eq!(deserialized.hint, None);
    }

    #[test]
    fn test_select_step_without_output() {
        let yaml_content = r#"
- type: select
  prompt: "Choose an item:"
  items:
    - value: "item1"
      label: "Item 1"
"#;

        let steps: Vec<PromptStep> = serde_saphyr::from_str(yaml_content).unwrap();
        assert_eq!(steps.len(), 1);

        match &steps[0].step_type {
            PromptStepType::Select { prompt, items, output, initial } => {
                assert_eq!(prompt, "Choose an item:");
                match items {
                    SelectItems::Static(static_items) => {
                        assert_eq!(static_items.len(), 1);
                    },
                    _ => panic!("Expected Static items"),
                }
                assert_eq!(output, &None);
                assert_eq!(initial, &None);
            },
            _ => panic!("Expected Select step type"),
        }
    }
}
