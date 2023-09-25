//! Rust integration tests don't load modules by default.
//! This file is picked up by the integration tester and directs it to integration test submodules.
use anyhow::{Context, Result};
use bottlerocket_settings_sdk::model::AsModel;
use bottlerocket_settings_sdk::{GenerateResult, Migrator, SettingsExtension};
pub use helpers::*;
use log::LevelFilter;
use serde::de::DeserializeOwned;

#[ctor::ctor]
fn setup_logging() {
    env_logger::builder()
        .filter_level(LevelFilter::Trace)
        .is_test(true)
        .init();
}

mod colliding_versions;
mod motd;

/// We also define some helpers for invoking the CLI interface generated by the SDK.
mod helpers {
    use super::*;

    /// Wrapper around "extension.set" which uses the CLI.
    pub fn set_cli<Mi, Mo>(
        extension: SettingsExtension<Mi, Mo>,
        version: &str,
        value: serde_json::Value,
    ) -> Result<serde_json::Value>
    where
        Mi: Migrator<ModelKind = Mo>,
        Mo: AsModel,
    {
        extension
            .try_run_with_args(&[
                "extension",
                "proto1",
                "set",
                "--setting-version",
                version,
                "--value",
                value.to_string().as_str(),
            ])
            .context("Failed to run settings extension CLI")
            .and_then(|s| {
                serde_json::from_str(s.as_str()).context("Failed to parse CLI result as JSON")
            })
    }

    /// Wrapper around "extension.generate" which uses the CLI.
    pub fn generate_cli<Mi, Mo, P, C>(
        extension: SettingsExtension<Mi, Mo>,
        version: &str,
        existing_partial: Option<serde_json::Value>,
        required_settings: Option<serde_json::Value>,
    ) -> Result<GenerateResult<P, C>>
    where
        Mi: Migrator<ModelKind = Mo>,
        Mo: AsModel,
        P: DeserializeOwned,
        C: DeserializeOwned,
    {
        let mut args: Vec<String> = vec![
            "extension",
            "proto1",
            "generate",
            "--setting-version",
            version,
        ]
        .into_iter()
        .map(str::to_string)
        .collect();

        if let Some(existing_partial) = &existing_partial {
            args.append(&mut vec![
                "--existing-partial".to_string(),
                existing_partial.to_string(),
            ]);
        }
        if let Some(required_settings) = required_settings {
            args.append(&mut vec![
                "--required-settings".to_string(),
                required_settings.to_string(),
            ]);
        }

        extension
            .try_run_with_args(args)
            .context("Failed to run settings extension CLI")
            .and_then(|s| serde_json::from_str(s.as_str()).context("Failed to parse CLI result"))
    }

    /// Wrapper around "extension.validate" which uses the CLI.
    pub fn validate_cli<Mi, Mo>(
        extension: SettingsExtension<Mi, Mo>,
        version: &str,
        value: serde_json::Value,
        required_settings: Option<serde_json::Value>,
    ) -> Result<serde_json::Value>
    where
        Mi: Migrator<ModelKind = Mo>,
        Mo: AsModel,
    {
        let mut args: Vec<String> = vec![
            "extension",
            "proto1",
            "validate",
            "--setting-version",
            version,
            "--value",
            value.to_string().as_str(),
        ]
        .into_iter()
        .map(str::to_string)
        .collect();

        if let Some(required_settings) = required_settings {
            args.append(&mut vec![
                "--required-settings".to_string(),
                required_settings.to_string(),
            ]);
        }

        extension
            .try_run_with_args(args)
            .context("Failed to run settings extension CLI")
            .and_then(|s| {
                serde_json::from_str(s.as_str()).context("Failed to parse CLI result as JSON")
            })
    }

    /// Wrapper around "extension.migrate" which uses the CLI.
    pub fn migrate_cli<Mi, Mo>(
        extension: SettingsExtension<Mi, Mo>,
        value: serde_json::Value,
        from_version: &str,
        target_version: &str,
    ) -> Result<serde_json::Value>
    where
        Mi: Migrator<ModelKind = Mo>,
        Mo: AsModel,
    {
        let value = value.to_string();
        let args = vec![
            "extension",
            "proto1",
            "migrate",
            "--value",
            &value,
            "--from-version",
            from_version,
            "--target-version",
            target_version,
        ];

        extension
            .try_run_with_args(args)
            .context("Failed to run settings extension CLI")
            .and_then(|s| {
                serde_json::from_str(s.as_str()).context("Failed to parse CLI result as JSON")
            })
    }
}
