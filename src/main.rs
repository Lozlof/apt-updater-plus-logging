/* MAIN.RS */

use better_logger::{LoggerSettings, NetworkFormat, logger};
use std::process::Command;
use std::process::exit;
use std::thread::sleep;
use std::time::Duration;
use chrono::Local;

fn main() {
    let now = Local::now();
    let now_formatted = format!("{}", now.format("%Y-%m-%d_%H:%M:%S"));

    let settings = LoggerSettings {
        terminal_logs: true,
        terminal_log_lvl: "trace".to_string(),
        wasm_logging: false,
        file_logs: true,
        file_log_lvl: "debug".to_string(),
        log_file_path: format!("../logs/{}", now_formatted),
        network_logs: false,
        network_log_lvl: "".to_string(),
        network_endpoint_url: "".to_string(),
        network_format: NetworkFormat::JsonText { field: "text".into() },
        debug_extra: true,
        async_logging: false,
    };

    if let Err(err) = logger::init(settings) {
        eprintln!("{:?}", err);
        exit(1);
    }

    let apt_update_message = {
        let start = "START - APT-GET UPDATE";
        let end = "END - APT-GET UPDATE";
        let output = match Command::new("/usr/bin/apt-get").args(["update", "-y"]).output() {
            Ok(output) => output,
            Err(error) => {
                logger::error!("{}\n{:?}\n{}", start, error, end);
                exit(1);
            }
        };

        let stdout = String::from_utf8_lossy(&output.stdout);
        let stderr = String::from_utf8_lossy(&output.stderr);

        let stdout_last_10_lines = {
            let last_10 = stdout
            .lines()
            .rev()
            .take(10)
            .collect::<Vec<_>>()
            .into_iter()
            .rev()
            .collect::<Vec<_>>()
            .join("\n");
            
            if last_10.is_empty() {
                logger::error!("{}\n{:?}\n{}", start, "stdout is empty", end);
                exit(1);
            } else {
                format!("stdout last 10 lines:\n{}", last_10)
            }
        };
        let stderr_last_10_lines = {
            let last_10 = stderr
            .lines()
            .rev()
            .take(10)
            .collect::<Vec<_>>()
            .into_iter()
            .rev()
            .collect::<Vec<_>>()
            .join("\n"); 

            if last_10.is_empty() {
                "stderr is empty".to_string()
            } else {
                last_10
            }
        };

        let update_status = {
            if output.status.success() {
                "Status: SUCCESS".to_string()
            } else {
                format!("Status: FAILED: {:?}", output.status)
            }
        };

        format!("{}\n{}\n{}\n{}\n{}\n{}\n{}\n{}\n{}", start, "", update_status, "", stdout_last_10_lines, "", stderr_last_10_lines, "", end)
    };

    let apt_upgrade_message = {
        let start = "START - APT-GET UPGRADE";
        let end = "END - APT-GET UPGRADE";
        let output = match Command::new("/usr/bin/apt-get").args(["upgrade", "-y"]).output() {
            Ok(output) => output,
            Err(error) => {
                logger::error!("{}\n{:?}\n{}", start, error, end);
                exit(1);
            }
        };

        let stdout = String::from_utf8_lossy(&output.stdout);
        let stderr = String::from_utf8_lossy(&output.stderr);

        let stdout_last_10_lines = {
            let last_10 = stdout
            .lines()
            .rev()
            .take(10)
            .collect::<Vec<_>>()
            .into_iter()
            .rev()
            .collect::<Vec<_>>()
            .join("\n");
            
            if last_10.is_empty() {
                logger::error!("{}\n{:?}\n{}", start, "stdout is empty", end);
                exit(1);
            } else {
                format!("stdout last 10 lines:\n{}", last_10)
            }
        };
        let stderr_last_10_lines = {
            let last_10 = stderr
            .lines()
            .rev()
            .take(10)
            .collect::<Vec<_>>()
            .into_iter()
            .rev()
            .collect::<Vec<_>>()
            .join("\n"); 

            if last_10.is_empty() {
                "stderr is empty".to_string()
            } else {
                last_10
            }
        };

        let update_status = {
            if output.status.success() {
                "Status: SUCCESS".to_string()
            } else {
                format!("Status: FAILED: {:?}", output.status)
            }
        };

        format!("{}\n{}\n{}\n{}\n{}\n{}\n{}\n{}\n{}", start, "", update_status, "", stdout_last_10_lines, "", stderr_last_10_lines, "", end)
    };


    println!("{}\n{}\n{}",apt_update_message, "", apt_upgrade_message);

    sleep(Duration::from_secs(30));
    exit(0);
}
