use once_cell::sync::Lazy;
use serde::Deserialize;
use std::fs;
use std::path::PathBuf;
use std::env;
#[derive(Debug, Deserialize)]
pub struct CoreConfig {
    pub git_dir: Option<String>,
    pub default_branch: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct Config {
    pub core: Option<CoreConfig>,
}

fn load_config() -> Config {
    let config_content = fs::read_to_string("config.toml").unwrap_or_default();
    toml::from_str(&config_content).unwrap_or(Config { core: None })
}

// 👇 全局配置变量
pub static CONFIG: Lazy<Config> = Lazy::new(load_config);

// 👇 全局 git 目录（默认为 ".mygit"）
pub static GIT_DIR: Lazy<PathBuf> = Lazy::new(|| {
    let dir = CONFIG
        .core
        .as_ref()
        .and_then(|c| c.git_dir.as_ref())
        .cloned()
        .unwrap_or_else(|| ".mygit".to_string());
    PathBuf::from(dir)
});

// 👇 全局默认分支名（默认为 "master"）
pub static DEFAULT_BRANCH: Lazy<String> = Lazy::new(|| {
    CONFIG
        .core
        .as_ref()
        .and_then(|c| c.default_branch.as_ref())
        .cloned()
        .unwrap_or_else(|| "master".to_string())
});

/// 是否启用详细输出模式（由环境变量控制）
pub static IS_VERBOSE: Lazy<bool> = Lazy::new(|| {
    env::var("RUST_GIT_VERBOSE")
        .map(|v| v == "1" || v.to_lowercase() == "true")
        .unwrap_or(false)
});
