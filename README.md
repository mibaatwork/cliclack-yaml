# cliclack-yaml

A Rust crate that lets you configure [cliclack](https://crates.io/crates/cliclack) interactive CLI prompts using YAML files instead of code.

## Features

- Define CLI prompts entirely in YAML — no Rust code needed per prompt
- 11 step types: `intro`, `outro`, `input`, `password`, `confirm`, `select`, `multiselect`, `spinner`, `print`, `progress`, `clearscreen`
- Variable interpolation with `{variable_name}` syntax
- Conditional step execution based on previous step results
- Text styling (colors, backgrounds) for intro/outro steps
- Input validation (not-empty, regex, min-length)

## Installation

Add to your `Cargo.toml`:

```toml
[dependencies]
cliclack_yaml = "0.1.1"
```

## Quick Start

### 1. Set up a prompts folder

Create a folder in your project to hold YAML prompt files, for example:

```
src/
  prompts/
    auth/
      login.yaml
    init/
      setup.yaml
    greeting.yaml
```

### 2. Configure cliclack_yaml in `main.rs`

```rust
use std::env;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Set the application root (typically the current working directory)
    cliclack_yaml::set_app_root(env::current_dir()?)?;

    // Set the prompt folder relative to app root
    cliclack_yaml::set_prompt_folder("src/prompts")?;

    // Run a prompt defined in src/prompts/greeting.yaml
    let results = cliclack_yaml::render_prompt_interaction("greeting")?;
    println!("User said: {:?}", results);

    Ok(())
}
```

### 3. Write YAML prompt files

**Simple greeting (`src/prompts/greeting.yaml`):**

```yaml
- type: intro
  text: " Welcome to My CLI "
  style:
    color: black
    background: cyan

- type: input
  prompt: "What is your name?"
  output: "name"

- type: outro
  text: " Hello, {name}! "
  style:
    color: black
    background: green
```

**Login flow with password (`src/prompts/auth/login.yaml`):**

```yaml
- type: intro
  text: " 🔐 Login "
  style:
    color: black
    background: cyan

- type: input
  prompt: "Email address"
  output: "email"

- type: password
  prompt: "Password"
  output: "password"

- type: outro
  text: " Authenticating... "
  style:
    color: black
    background: blue
```

Run nested prompts using the folder path:

```rust
let results = cliclack_yaml::render_prompt_interaction("auth/login")?;
let email = results.get("email").unwrap();
```

## Passing Variables

You can pass variables into prompts and reference them with `{variable_name}`:

```rust
let results = cliclack_yaml::render_prompt_interaction_with_vars(
    "init/setup",
    vec![("config_path", "/home/user/.config/myapp")],
)?;
```

```yaml
- type: print
  text: "Config will be created at: {config_path}"
```

## YAML Step Reference

### clearscreen

Clears the terminal screen.

```yaml
- type: clearscreen
```

### intro

Displays a styled introduction banner.

```yaml
- type: intro
  text: " My Application "
  style:
    color: black        # black, white, red, green, blue
    background: cyan    # cyan, green, red, yellow
```

### input

Prompts for text input with optional validation.

```yaml
- type: input
  prompt: "Project name"
  placeholder: "my-project"
  output: "project_name"
  validate:
    - type: not-empty
      message: "Project name is required"
    - type: must-start-with-letter
      message: "Must start with a letter"
    - type: regex
      pattern: "^[a-z][a-z0-9-]*$"
      message: "Only lowercase letters, numbers, and hyphens"
```

### password

Prompts for password input with masking.

```yaml
- type: password
  prompt: "Enter password"
  output: "password"
  mask: '*'
  confirm_password: true
  confirm_prompt: "Confirm password"
  validate:
    - type: not-empty
      message: "Password is required"
    - type: min-length
      length: 8
      message: "Must be at least 8 characters"
```

### confirm

Prompts for yes/no confirmation. Returns `true`/`false` in the output.

```yaml
- type: confirm
  prompt: "Continue?"
  output: "confirmed"
  default: true
```

### select

Prompts the user to select one option from a list.

```yaml
- type: select
  prompt: "Choose a framework"
  output: "framework"
  items:
    - label: "React"
      value: "react"
    - label: "Vue"
      value: "vue"
      hint: "Recommended"
    - label: "Angular"
      value: "angular"
```

### multiselect

Prompts the user to select multiple options.

```yaml
- type: multiselect
  prompt: "Select features"
  output: "features"
  required: true
  items:
    - label: "Authentication"
      value: "auth"
    - label: "Database"
      value: "db"
      is_selected: true
    - label: "API"
      value: "api"
```

### spinner

Displays a spinner while executing a named function.

```yaml
- type: spinner
  start_text: "Installing dependencies..."
  stop_text: "Dependencies installed!"
  run_fn: "install_deps"
  output: "install_result"
```

> **Note:** The `run_fn` value must match a function name registered in your [`execute_function_by_name`] handler.

### print

Prints a message within the CLI frame. The `loglevel` field controls the visual style:

| `loglevel`       | Symbol | Description                                     |
|------------------|--------|-------------------------------------------------|
| *(omitted)*      | `│`    | Bar continuation — blends into the frame        |
| `none`           | `├`    | Remark — connect-left, no marker icon           |
| `info`           | `●`    | Info marker (blue)                              |
| `warning`/`warn` | `▲`    | Warning marker (yellow)                         |
| `error`          | `■`    | Error marker (red)                              |

**Plain text (no loglevel) — blends into the frame:**

```yaml
- type: print
  text: "This text appears as part of the frame"
```

**With log level marker:**

```yaml
- type: print
  loglevel: info
  text: "Config path: {config_path}"
  input: config_path  # optional: variable to read
```

**Remark style (connect-left, no icon):**

```yaml
- type: print
  loglevel: none
  text: "A remark without a marker icon"
```

> **Tip:** Use no `loglevel` for content like tables or formatted output that should blend seamlessly into the cliclack frame. Use `loglevel: none` when you want a visual separator (`├`) without an icon. Use `info`/`warning`/`error` for messages that need attention markers.

### progress

Displays a progress bar or spinner for long-running tasks.

```yaml
- type: progress
  progress_type: Bar
  start_message: "Downloading..."
  stop_message: "Download complete!"
  items:
    - name: "file1.zip"
      duration_ms: 2000
    - name: "file2.zip"
      duration_ms: 1500
```

### multi-progress

Displays multiple progress bars simultaneously.

```yaml
- type: multi-progress
  title: "Installing components"
  progress_bars:
    - label: "Core"
      progress_type: Bar
      duration_ms: 3000
    - label: "Plugins"
      progress_type: Spinner
      duration_ms: 2000
```

### outro

Displays a styled closing message.

```yaml
- type: outro
  text: " All done! "
  input: result_var   # optional: variable to display
  style:
    color: black
    background: green
```

## Conditional Steps

Steps can be conditionally executed based on previous step results:

```yaml
- type: confirm
  prompt: "Enable advanced mode?"
  output: "advanced"
  step_name: "advanced_check"

- type: input
  prompt: "Enter API key"
  output: "api_key"
  condition:
    parent: "advanced_check"
    value: true
```

The `input` step only runs if the user confirmed "advanced_check".

## Advanced: Embedded Prompts with Fallback

For distributing your CLI as a single binary, you can embed YAML files at compile time using [`include_dir`](https://crates.io/crates/include_dir) and fall back to them when no external prompt folder is found:

```rust
use std::collections::HashMap;
use anyhow::Result;
use include_dir::{include_dir, Dir};

// Embed all YAML files from src/prompts/ into the binary
static EMBEDDED_PROMPTS: Dir = include_dir!("src/prompts");

fn get_embedded_yaml(prompt_name: &str) -> Result<String> {
    let file_path = format!("{}.yaml", prompt_name);
    EMBEDDED_PROMPTS
        .get_file(&file_path)
        .and_then(|f| f.contents_utf8())
        .map(|s| s.to_string())
        .ok_or_else(|| anyhow::anyhow!("Prompt '{}' not found", prompt_name))
}

/// Try external folder first, fall back to embedded prompts
pub fn run_prompt(
    prompt_name: &str,
    vars: Option<HashMap<String, String>>,
) -> Result<HashMap<String, String>> {
    let yaml_str = match cliclack_yaml::get_yaml_content(prompt_name) {
        Ok(content) => content,
        Err(_) => get_embedded_yaml(prompt_name)?,
    };

    let steps: Vec<cliclack_yaml::PromptStep> = serde_yml::from_str(&yaml_str)?;
    cliclack_yaml::create_prompt_with_vars_custom(steps, vars)
}
```

This pattern lets users override prompts by placing YAML files in the configured prompt folder, while keeping defaults baked into the binary.

## License

MIT

## Examples

See the [`examples/config/`](examples/config/) folder for a complete set of white-label YAML configurations covering auth flows, initialization, config management, CRUD operations, installation progress, system checks, and showcases for every feature (conditionals, variables, validation, styling).

For a full working CLI application that integrates `cliclack_yaml` with [clap](https://crates.io/crates/clap), see the **[cliclack-yaml-cli-example](https://github.com/mibaatwork/cliclack-yaml-cli-example)** repository. It demonstrates embedded prompts, configuration management, and how to structure a white-label CLI project using this crate.
