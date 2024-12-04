#![allow(unused)]

use std::path::PathBuf;

const APP_DIR: &str = ".hn";
const LOG_DIR: &str = "log";
const CACHE_DIR: &str = "cache";
const DB_DIR: &str = "db";
const CONF_DIR: &str = "config";

#[inline]
pub(crate) fn app_dir() -> PathBuf {
    dirs::home_dir()
        .expect("Failed to get home directory")
        .join(APP_DIR)
}

#[inline]
pub(crate) fn log_dir() -> PathBuf {
    app_dir().join(LOG_DIR)
}

#[inline]
pub(crate) fn cache_dir() -> PathBuf {
    app_dir().join(CACHE_DIR)
}

#[inline]
pub(crate) fn db_dir() -> PathBuf {
    app_dir().join(DB_DIR)
}

#[inline]
pub(crate) fn conf_dir() -> PathBuf {
    app_dir().join(CONF_DIR)
}
