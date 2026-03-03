# Example YAML Configurations

This folder contains white-label example configurations demonstrating all
`cliclack_yaml` step types and patterns. Copy and adapt them for your own CLI.

## Folder Structure

```
examples/config/
  auth/
    login.yaml           # Email + password login flow
    register.yaml        # Account registration with password confirmation
    api-key.yaml         # API key input with expiration select
    logout.yaml          # Logout confirmation with dynamic messages
    status.yaml          # Display auth status info
  init/
    setup-success.yaml   # Successful initialization output
    setup-force.yaml     # Force-reinitialize with overwrite warning
    already-exists.yaml  # Config already exists notification
    setup-error.yaml     # Error during initialization
  config/
    add.yaml             # Add a key-value config entry
    list.yaml            # Display current configuration
    remove.yaml          # Multi-select removal with confirmation
  items/
    add-form.yaml        # Create item with name, description, type select
    add-scoped.yaml      # Create item scoped to a group (e.g. org, team)
    add-space-select.yaml # Choose scope before creating item
    list.yaml            # Display a list of items
    remove.yaml          # Multi-select removal with destructive confirm
  install/
    progress.yaml        # Progress bar installation flow
  check/
    system.yaml          # System requirements check output
  showcase/
    all-steps.yaml       # Every step type in one file
    print-plain.yaml     # Print step — all loglevel variants
    conditional.yaml     # Conditional step execution patterns
    variables.yaml       # Variable interpolation examples
    validation.yaml      # Input and password validation examples
    styling.yaml         # All text styling options
```

## Usage

Point `cliclack_yaml` at this folder to try the examples:

```rust
cliclack_yaml::set_app_root(env::current_dir()?)?;
cliclack_yaml::set_prompt_folder("examples/config")?;

let results = cliclack_yaml::render_prompt_interaction("auth/login")?;
```
