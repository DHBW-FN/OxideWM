use log::LevelFilter;

pub const LOG_LEVEL_ENV: &str = "RUST_LOG";
pub const LOG_LEVEL_DEFAULT: LevelFilter = LevelFilter::Info;
pub const LOG_FILE_LOCATION_PROD: &str = "/var/logs";
pub const LOG_FILE_LOCATION_DEV: &str = "/var/logs";

type ExitCode = i32;

pub const EXIT_CODE_LOGGER_CONFIG_FAIL: ExitCode = 1;
