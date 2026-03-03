use cliclack_yaml::{PromptStep, PromptStepType, MultiSelectItem, MultiSelectItems};

#[cfg(test)]
mod multi_select_tests {
    use super::*;

    #[test]
    fn test_multi_select_step_creation() {
        let items = vec![
            MultiSelectItem {
                value: "eslint".to_string(),
                label: "ESLint".to_string(),
                hint: Some("JavaScript linter".to_string()),
            },
            MultiSelectItem {
                value: "prettier".to_string(),
                label: "Prettier".to_string(),
                hint: Some("Code formatter".to_string()),
            },
            MultiSelectItem {
                value: "jest".to_string(),
                label: "Jest".to_string(),
                hint: None,
            },
        ];

        let step = PromptStep {
            step_type: PromptStepType::MultiSelect {
                prompt: "Select tools:".to_string(),
                items: MultiSelectItems::Static(items.clone()),
                output: Some("selected_tools".to_string()),
                required: Some(true),
            },
            output: None,
            step_name: None,
            condition: None,
        };

        // Verify the step was created correctly
        match &step.step_type {
            PromptStepType::MultiSelect { prompt, items, output, required } => {
                assert_eq!(prompt, "Select tools:");
                match items {
                    MultiSelectItems::Static(static_items) => {
                        assert_eq!(static_items.len(), 3);
                        assert_eq!(static_items[0].value, "eslint");
                        assert_eq!(static_items[0].label, "ESLint");
                        assert_eq!(static_items[0].hint, Some("JavaScript linter".to_string()));
                    },
                    _ => panic!("Expected Static items"),
                }
                assert_eq!(output, &Some("selected_tools".to_string()));
                assert_eq!(required, &Some(true));
            },
            _ => panic!("Expected MultiSelect step type"),
        }
    }

    #[test]
    fn test_multi_select_yaml_parsing() {
        let yaml_content = r#"
- type: multiselect
  prompt: "Select additional tools:"
  output: "tools"
  required: true
  items:
    - value: "eslint"
      label: "ESLint"
      hint: "JavaScript linter"
    - value: "prettier"
      label: "Prettier"
      hint: "Code formatter"
"#;

        let steps: Vec<PromptStep> = serde_saphyr::from_str(yaml_content).unwrap();
        assert_eq!(steps.len(), 1);

        match &steps[0].step_type {
            PromptStepType::MultiSelect { prompt, items, output, required } => {
                assert_eq!(prompt, "Select additional tools:");
                match items {
                    MultiSelectItems::Static(static_items) => {
                        assert_eq!(static_items.len(), 2);
                        assert_eq!(static_items[0].value, "eslint");
                        assert_eq!(static_items[0].label, "ESLint");
                        assert_eq!(static_items[0].hint, Some("JavaScript linter".to_string()));
                        assert_eq!(static_items[1].value, "prettier");
                        assert_eq!(static_items[1].label, "Prettier");
                    },
                    _ => panic!("Expected Static items"),
                }
                assert_eq!(output, &Some("tools".to_string()));
                assert_eq!(required, &Some(true));
            },
            _ => panic!("Expected MultiSelect step type"),
        }
    }

    #[test]
    fn test_multi_select_yaml_without_required() {
        let yaml_content = r#"
- type: multiselect
  prompt: "Select tools:"
  output: "selected"
  items:
    - value: "tool1"
      label: "Tool 1"
    - value: "tool2"
      label: "Tool 2"
      hint: "Optional tool"
"#;

        let steps: Vec<PromptStep> = serde_saphyr::from_str(yaml_content).unwrap();
        assert_eq!(steps.len(), 1);

        match &steps[0].step_type {
            PromptStepType::MultiSelect { prompt, items, output, required } => {
                assert_eq!(prompt, "Select tools:");
                match items {
                    MultiSelectItems::Static(static_items) => {
                        assert_eq!(static_items.len(), 2);
                        assert_eq!(static_items[0].hint, None);
                        assert_eq!(static_items[1].hint, Some("Optional tool".to_string()));
                    },
                    _ => panic!("Expected Static items"),
                }
                assert_eq!(output, &Some("selected".to_string()));
                assert_eq!(required, &None);
            },
            _ => panic!("Expected MultiSelect step type"),
        }
    }

    #[test]
    fn test_multi_select_items_serialization() {
        let item = MultiSelectItem {
            value: "test_value".to_string(),
            label: "Test Label".to_string(),
            hint: Some("Test hint".to_string()),
        };

        let serialized = serde_saphyr::to_string(&item).unwrap();
        let deserialized: MultiSelectItem = serde_saphyr::from_str(&serialized).unwrap();

        assert_eq!(deserialized.value, "test_value");
        assert_eq!(deserialized.label, "Test Label");
        assert_eq!(deserialized.hint, Some("Test hint".to_string()));
    }

    #[test]
    fn test_multi_select_items_without_hint() {
        let item = MultiSelectItem {
            value: "test_value".to_string(),
            label: "Test Label".to_string(),
            hint: None,
        };

        let serialized = serde_saphyr::to_string(&item).unwrap();
        let deserialized: MultiSelectItem = serde_saphyr::from_str(&serialized).unwrap();

        assert_eq!(deserialized.value, "test_value");
        assert_eq!(deserialized.label, "Test Label");
        assert_eq!(deserialized.hint, None);
    }

    #[test]
    fn test_multi_select_step_without_output() {
        let yaml_content = r#"
- type: multiselect
  prompt: "Choose options:"
  items:
    - value: "option1"
      label: "Option 1"
"#;

        let steps: Vec<PromptStep> = serde_saphyr::from_str(yaml_content).unwrap();
        assert_eq!(steps.len(), 1);

        match &steps[0].step_type {
            PromptStepType::MultiSelect { prompt, items, output, required } => {
                assert_eq!(prompt, "Choose options:");
                match items {
                    MultiSelectItems::Static(static_items) => {
                        assert_eq!(static_items.len(), 1);
                    },
                    _ => panic!("Expected Static items"),
                }
                assert_eq!(output, &None);
                assert_eq!(required, &None);
            },
            _ => panic!("Expected MultiSelect step type"),
        }
    }

    // Test the multi_select handling logic
    #[test]
    fn test_multi_select_context_storage() {
        // This test would need to be adapted based on your specific testing setup
        // since it involves the actual cliclack interaction which may not be easily testable
        // in a unit test environment. For now, we'll test the structure and parsing.
    }
}
