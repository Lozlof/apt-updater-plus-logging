# apt-updater-plus-logging
#### Runs all the apt update / upgrade / clean commands with built in logging
#### See: https://crates.io/crates/better-logger for more details
## config.toml
#### Put in same directory as the executable
```toml
terminal_logs = true
terminal_log_lvl = "error"
wasm_logging = false
file_logs = false
file_log_lvl = ""
log_file_path = ""
network_logs = true
network_log_lvl = "debug"
network_endpoint_url = "https://test.com"
debug_extra = false
async_logging = false
machine_name = "testing-01"
[network_format]
type = "JsonText"
field = "text"
```
### This executable is committed:
- I do this so I don't have to build on the production VM.
- You should clean and rebuild your own executable.
```bash
cargo +stable build --release --target x86_64-unknown-linux-gnu
```     
