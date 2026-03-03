use std::collections::HashMap;
use std::time::Duration;
use anyhow::Result;
use serde::{Deserialize, Serialize};
use crate::PromptStepType;

#[derive(Debug, Deserialize, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ProgressItem {
    pub message: String,
    pub increment: Option<u64>,
    pub delay_ms: Option<u64>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
#[serde(rename_all = "camelCase", tag = "type")]
pub enum ProgressType {
    #[serde(rename = "bar")]
    Bar { 
        total: u64,
        #[serde(default)]
        use_download_template: bool,
    },
    #[serde(rename = "spinner")]
    Spinner,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ProgressBarConfig {
    pub progress_type: ProgressType,
    pub start_message: String,
    pub stop_message: String,
    pub items: Option<Vec<ProgressItem>>,
}

/// Handle Progress prompt step
pub fn handle_progress(
    step_type: &PromptStepType,
    context: &mut HashMap<String, String>,
) -> Result<()> {
    if let PromptStepType::Progress { 
        progress_type, 
        start_message, 
        stop_message, 
        items,
        output 
    } = step_type {
        match progress_type {
            ProgressType::Bar { total, use_download_template } => {
                let mut progress = cliclack::progress_bar(*total);
                
                if *use_download_template {
                    progress = progress.with_download_template();
                }
                
                progress.start(start_message);
                
                // Process items if provided
                if let Some(progress_items) = items {
                    for item in progress_items {
                        if let Some(delay) = item.delay_ms {
                            std::thread::sleep(Duration::from_millis(delay));
                        }
                        
                        let increment = item.increment.unwrap_or(1);
                        progress.inc(increment);
                        
                        if !item.message.is_empty() {
                            progress.set_message(&item.message);
                        }
                    }
                } else {
                    // Simple automatic progress
                    for _ in 0..*total {
                        std::thread::sleep(Duration::from_millis(50));
                        progress.inc(1);
                    }
                }
                
                progress.stop(stop_message);
            },
            ProgressType::Spinner => {
                let spinner = cliclack::spinner();
                spinner.start(start_message);
                
                // Process items if provided
                if let Some(progress_items) = items {
                    for item in progress_items {
                        if let Some(delay) = item.delay_ms {
                            std::thread::sleep(Duration::from_millis(delay));
                        }
                        
                        if !item.message.is_empty() {
                            spinner.set_message(&item.message);
                        }
                    }
                } else {
                    // Default spinner duration
                    std::thread::sleep(Duration::from_secs(2));
                }
                
                spinner.stop(stop_message);
            }
        }
        
        // Store output in context if specified
        if let Some(output_key) = output {
            context.insert(output_key.clone(), "completed".to_string());
        }
    }
    Ok(())
}

/// Handle MultiProgress prompt step
pub fn handle_multi_progress(
    step_type: &PromptStepType,
    context: &mut HashMap<String, String>,
) -> Result<()> {
    if let PromptStepType::MultiProgress { 
        title,
        progress_bars,
        output 
    } = step_type {
        let multi = cliclack::multi_progress(title);
        
        // Create a vector to store progress bars and spinners with their configs
        let mut bars = Vec::new();
        let mut spinners = Vec::new();
        
        // Create all progress indicators
        for (idx, bar_config) in progress_bars.iter().enumerate() {
            match &bar_config.progress_type {
                ProgressType::Bar { total, use_download_template } => {
                    let mut progress = multi.add(cliclack::progress_bar(*total));
                    if *use_download_template {
                        progress = progress.with_download_template();
                    }
                    bars.push((idx, progress, bar_config));
                },
                ProgressType::Spinner => {
                    let spinner = multi.add(cliclack::spinner());
                    spinners.push((idx, spinner, bar_config));
                }
            }
        }
        
        // Start all progress indicators
        for (_, bar, config) in &bars {
            bar.start(&config.start_message);
        }
        for (_, spinner, config) in &spinners {
            spinner.start(&config.start_message);
        }
        
        // Process items for progress bars
        for (_idx, bar, config) in &bars {
            if let Some(progress_items) = &config.items {
                for item in progress_items {
                    if let Some(delay) = item.delay_ms {
                        std::thread::sleep(Duration::from_millis(delay));
                    }
                    
                    let increment = item.increment.unwrap_or(1);
                    bar.inc(increment);
                    
                    if !item.message.is_empty() {
                        bar.set_message(&item.message);
                    }
                }
            } else {
                // Default behavior - fill the progress bar
                if let ProgressType::Bar { total, .. } = config.progress_type {
                    for _ in 0..total {
                        std::thread::sleep(Duration::from_millis(20));
                        bar.inc(1);
                    }
                }
            }
        }
        
        // Process items for spinners
        for (_idx, spinner, config) in &spinners {
            if let Some(progress_items) = &config.items {
                for item in progress_items {
                    if let Some(delay) = item.delay_ms {
                        std::thread::sleep(Duration::from_millis(delay));
                    }
                    
                    if !item.message.is_empty() {
                        spinner.set_message(&item.message);
                    }
                }
            } else {
                // Default spinner duration
                std::thread::sleep(Duration::from_secs(1));
            }
        }
        
        // Stop all progress indicators
        for (_, bar, config) in &bars {
            bar.stop(&config.stop_message);
        }
        for (_, spinner, config) in &spinners {
            spinner.stop(&config.stop_message);
        }
        
        multi.stop();
        
        // Store output in context if specified
        if let Some(output_key) = output {
            context.insert(output_key.clone(), "completed".to_string());
        }
    }
    Ok(())
}
