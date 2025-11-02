/* MAIN.RS */

use better_logger::{LoggerSettings, NetworkFormat, logger};
use std::process::Command;
use std::process::exit;
use std::thread::sleep;
use std::time::Duration;
use std::fs::read_to_string;
use std::error::Error;
use chrono::Local;
use serde::Deserialize;

#[derive(Deserialize)]
#[serde(tag = "type")]
enum ConfigNetworkFormat {
    PlainText,
    JsonText { field: String },
}

impl From<ConfigNetworkFormat> for NetworkFormat {
    fn from(v: ConfigNetworkFormat) -> Self {
        match v {
            ConfigNetworkFormat::PlainText => NetworkFormat::PlainText,
            ConfigNetworkFormat::JsonText { field } => NetworkFormat::JsonText { field },
        }
    }
}

#[derive(Deserialize)]
struct Config {
    terminal_logs: bool,
    terminal_log_lvl: String,
    wasm_logging: bool,
    file_logs: bool,
    file_log_lvl: String,
    log_file_path: String,
    network_logs: bool,
    network_log_lvl: String,
    network_endpoint_url: String,
    network_format: ConfigNetworkFormat,
    debug_extra: bool,
    async_logging: bool,
    machine_name: String,
}

fn load_config(path: &str) -> Result<Config, Box<dyn Error>> {
    let raw = read_to_string(path)?;
    return Ok(toml::from_str(&raw)?);
}

fn main() {
    let now = Local::now();
    let now_formatted = format!("{}", now.format("%Y-%m-%d_%H:%M:%S"));

    let config_path = "config.toml";
    let config = match load_config(&config_path) {
        Ok(config) => config,
        Err(err) => {
            eprintln!("{}: {:?}", now_formatted, err);
            exit(1);
        }
    };

    let settings = LoggerSettings {
        terminal_logs: config.terminal_logs,
        terminal_log_lvl: config.terminal_log_lvl,
        wasm_logging: config.wasm_logging,
        file_logs: config.file_logs,
        file_log_lvl: config.file_log_lvl,
        log_file_path: config.log_file_path,
        network_logs: config.network_logs,
        network_log_lvl: config.network_log_lvl,
        network_endpoint_url: config.network_endpoint_url,
        network_format: config.network_format.into(),
        debug_extra: config.debug_extra,
        async_logging: config.async_logging,
    };

    if let Err(err) = logger::init(settings) {
        eprintln!("{}: {:?}", now_formatted, err);
        exit(1);
    }

    let apt_update_message = {
        let start = "START - APT-GET UPDATE";
        let end = "END - APT-GET UPDATE";
        let output = match Command::new("/usr/bin/apt-get").args(["update", "-y"]).output() {
            Ok(output) => output,
            Err(error) => {
                logger::error!("{}\n{}\n{:?}\n{}", now_formatted, start, error, end);
                exit(1);
            }
        };

        let stdout = String::from_utf8_lossy(&output.stdout);
        let stderr = String::from_utf8_lossy(&output.stderr);

        let stdout_last_20_lines = {
            let last_20 = stdout
            .lines()
            .rev()
            .take(20)
            .collect::<Vec<_>>()
            .into_iter()
            .rev()
            .collect::<Vec<_>>()
            .join("\n");
            
            if last_20.is_empty() {
                logger::error!("{}\n{}\n{:?}\n{}", now_formatted, start, "stdout is empty", end);
                exit(1);
            } else {
                format!("STDOUT last 20 lines:\n{}", last_20)
            }
        };
        let stderr_last_20_lines = {
            let last_20 = stderr
            .lines()
            .rev()
            .take(20)
            .collect::<Vec<_>>()
            .into_iter()
            .rev()
            .collect::<Vec<_>>()
            .join("\n"); 

            if last_20.is_empty() {
                "STDERR is empty".to_string()
            } else {
                format!("STDERR last 20 lines:\n{}", last_20)
            }
        };

        let update_status = {
            if output.status.success() {
                "Status: SUCCESS".to_string()
            } else {
                format!("Status: FAILED: {:?}", output.status)
            }
        };

        format!("{}\n{}\n{}\n{}\n{}\n{}\n{}\n{}\n{}", start, "", update_status, "", stdout_last_20_lines, "", stderr_last_20_lines, "", end)
    };

    let apt_upgrade_message = {
        let start = "START - APT-GET UPGRADE";
        let end = "END - APT-GET UPGRADE";
        let output = match Command::new("/usr/bin/apt-get").args(["upgrade", "-y"]).output() {
            Ok(output) => output,
            Err(error) => {
                logger::error!("{}\n{}\n{:?}\n{}", now_formatted, start, error, end);
                exit(1);
            }
        };

        let stdout = String::from_utf8_lossy(&output.stdout);
        let stderr = String::from_utf8_lossy(&output.stderr);

        let stdout_last_20_lines = {
            let last_20 = stdout
            .lines()
            .rev()
            .take(20)
            .collect::<Vec<_>>()
            .into_iter()
            .rev()
            .collect::<Vec<_>>()
            .join("\n");
            
            if last_20.is_empty() {
                logger::error!("{}\n{}\n{:?}\n{}", now_formatted, start, "stdout is empty", end);
                exit(1);
            } else {
                format!("STDOUT last 20 lines:\n{}", last_20)
            }
        };
        let stderr_last_20_lines = {
            let last_20 = stderr
            .lines()
            .rev()
            .take(20)
            .collect::<Vec<_>>()
            .into_iter()
            .rev()
            .collect::<Vec<_>>()
            .join("\n"); 

            if last_20.is_empty() {
                "STDERR is empty".to_string()
            } else {
                format!("STDERR last 20 lines:\n{}", last_20)
            }
        };

        let update_status = {
            if output.status.success() {
                "Status: SUCCESS".to_string()
            } else {
                format!("Status: FAILED: {:?}", output.status)
            }
        };

        format!("{}\n{}\n{}\n{}\n{}\n{}\n{}\n{}\n{}", start, "", update_status, "", stdout_last_20_lines, "", stderr_last_20_lines, "", end)
    };

    let apt_full_upgrade_message = {
        let start = "START - APT-GET FULL-UPGRADE";
        let end = "END - APT-GET FULL-UPGRADE";
        let output = match Command::new("/usr/bin/apt-get").args(["full-upgrade", "-y"]).output() {
            Ok(output) => output,
            Err(error) => {
                logger::error!("{}\n{}\n{:?}\n{}", now_formatted, start, error, end);
                exit(1);
            }
        };

        let stdout = String::from_utf8_lossy(&output.stdout);
        let stderr = String::from_utf8_lossy(&output.stderr);

        let stdout_last_20_lines = {
            let last_20 = stdout
            .lines()
            .rev()
            .take(20)
            .collect::<Vec<_>>()
            .into_iter()
            .rev()
            .collect::<Vec<_>>()
            .join("\n");
            
            if last_20.is_empty() {
                logger::error!("{}\n{}\n{:?}\n{}", now_formatted, start, "stdout is empty", end);
                exit(1);
            } else {
                format!("STDOUT last 20 lines:\n{}", last_20)
            }
        };
        let stderr_last_20_lines = {
            let last_20 = stderr
            .lines()
            .rev()
            .take(20)
            .collect::<Vec<_>>()
            .into_iter()
            .rev()
            .collect::<Vec<_>>()
            .join("\n"); 

            if last_20.is_empty() {
                "STDERR is empty".to_string()
            } else {
                format!("STDERR last 20 lines:\n{}", last_20)
            }
        };

        let update_status = {
            if output.status.success() {
                "Status: SUCCESS".to_string()
            } else {
                format!("Status: FAILED: {:?}", output.status)
            }
        };

        format!("{}\n{}\n{}\n{}\n{}\n{}\n{}\n{}\n{}", start, "", update_status, "", stdout_last_20_lines, "", stderr_last_20_lines, "", end)
    };


    println!("{}{}{}\n{}\n{}\n{}\n{}\n{}", now_formatted, "Update / upgrade report for ", config.machine_name, apt_update_message, "", apt_upgrade_message, "", apt_full_upgrade_message);

    sleep(Duration::from_secs(30));
    exit(0);
}
