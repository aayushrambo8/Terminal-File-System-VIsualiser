use std::fs;
use std::path::PathBuf;

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct OpenersConfig {
    pub image: String,
    pub pdf: String,
    pub text: String,
    pub default: String,
}

impl OpenersConfig {
    fn editor_default() -> String {
        std::env::var("EDITOR").unwrap_or_else(|_| "nano".to_string())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct Settings {
    pub start_dir: PathBuf,
    pub show_hidden: bool,
    pub theme_name: String,
    pub openers: OpenersConfig,
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            start_dir: dirs::home_dir().unwrap_or_else(|| PathBuf::from("/")),
            show_hidden: false,
            theme_name: "default".to_string(),
            openers: OpenersConfig {
                image: "xdg-open".to_string(),
                pdf: "xdg-open".to_string(),
                text: OpenersConfig::editor_default(),
                default: "xdg-open".to_string(),
            },
        }
    }
}

impl Settings {
    pub fn config_dir() -> PathBuf {
        if let Ok(path) = std::env::var("TFSV_CONFIG_DIR") {
            return PathBuf::from(path);
        }

        if let Some(path) = dirs::config_dir() {
            path.join("tfsv")
        } else {
            PathBuf::from("/tmp")
        }
    }

    pub fn config_path() -> PathBuf {
        Self::config_dir().join("settings.toml")
    }

    pub fn load() -> Self {
        let path = Self::config_path();
        if !path.exists() {
            return Self::default();
        }
        match fs::read_to_string(&path) {
            Ok(content) => match toml::from_str::<Self>(&content) {
                Ok(settings) => settings,
                Err(err) => {
                    eprintln!(
                        "Warning: failed to parse settings at {}: {err}. Using default settings.",
                        path.display()
                    );
                    Self::default()
                }
            },
            Err(_) => Self::default(),
        }
    }

    pub fn save(&self) -> Result<(), Box<dyn std::error::Error>> {
        let dir = Self::config_dir();
        fs::create_dir_all(&dir)?;
        let content = toml::to_string_pretty(self)?;
        fs::write(Self::config_path(), content)?;
        Ok(())
    }
}

#[cfg(test)]
pub(crate) static TEST_CONFIG_MUTEX: std::sync::Mutex<()> = std::sync::Mutex::new(());

#[cfg(test)]
mod tests {
    use super::Settings;
    use super::TEST_CONFIG_MUTEX;

    #[test]
    fn settings_save_and_load_roundtrip() {
        let _guard = TEST_CONFIG_MUTEX.lock().unwrap();
        let key = "TFSV_CONFIG_DIR";
        let previous = std::env::var(key).ok();
        let unique = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_nanos();
        let target = std::env::temp_dir().join(format!(
            "tfsv-settings-test-{}-{unique}",
            std::process::id()
        ));
        let _ = std::fs::remove_dir_all(&target);

        unsafe {
            std::env::set_var(key, &target);
        }

        let settings = Settings {
            show_hidden: true,
            ..Default::default()
        };
        settings.save().unwrap();

        let loaded = Settings::load();
        assert!(loaded.show_hidden);

        // Verify malformed settings fallback to default without crashing
        std::fs::write(Settings::config_path(), "invalid toml syntax [[[").unwrap();
        let fallback = Settings::load();
        assert!(!fallback.show_hidden);

        unsafe {
            match previous {
                Some(value) => std::env::set_var(key, value),
                None => std::env::remove_var(key),
            }
        }
        let _ = std::fs::remove_dir_all(&target);
    }

    #[test]
    fn config_dir_honors_environment_override() {
        let _guard = TEST_CONFIG_MUTEX.lock().unwrap();
        let key = "TFSV_CONFIG_DIR";
        let previous = std::env::var(key).ok();
        let unique = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_nanos();
        let target = std::env::temp_dir().join(format!(
            "tfsv-config-override-{}-{unique}",
            std::process::id()
        ));

        unsafe {
            std::env::set_var(key, &target);
        }
        assert_eq!(Settings::config_dir(), target);

        unsafe {
            match previous {
                Some(value) => std::env::set_var(key, value),
                None => std::env::remove_var(key),
            }
        }
    }
}
