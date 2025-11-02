# apt-updater-plus-logging
## config.toml
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