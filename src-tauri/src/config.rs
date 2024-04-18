

use lazy_static::lazy_static;
use tauri::api::cli::Matches;
use tracing::info;
use std::sync::Mutex;

pub struct BlackCatConfig {
    pub host: String,
    pub port: u16,
    pub user: String,
    pub pass: String,
    pub keyfile: String,
    pub debug: bool,
}
impl BlackCatConfig {
    fn new() -> BlackCatConfig {
        BlackCatConfig {
            host: String::new(),
            port: 0,
            user: String::new(),
            pass: String::new(),
            keyfile: String::new(),
            debug: false,
        }
    }
}

lazy_static! {
    pub static ref CONFIG: BlackCatConfig = BlackCatConfig::new();
}
pub fn update_config(matches: Matches) {
    lazy_static! {
        pub static ref CONFIG: Mutex<BlackCatConfig> = Mutex::new(BlackCatConfig::new());
    }

    // ...

    match matches.args.get("debug") {
        Some(arg) => match arg.value.as_bool() {
            Some(val) => {
                CONFIG.lock().unwrap().debug = val;
            }
            None => {}
        },
        None => {}
    };

    match matches.args.get("host") {
        Some(arg) => match arg.value.as_str() {
            Some(val) => {
                CONFIG.lock().unwrap().host = val.to_string();
            }
            None => {}
        },
        None => {}
    };
    match matches.args.get("port") {
        Some(arg) => match arg.value.as_u64() {
            Some(val) => {
                CONFIG.lock().unwrap().port = val as u16;
            }
            None => {}
        },
        None => {}
    };
    match matches.args.get("user") {
        Some(arg) => match arg.value.as_str() {
            Some(val) => {
                CONFIG.lock().unwrap().user = val.to_string();
            }
            None => {}
        },
        None => {}
    };
    match matches.args.get("pass") {
        Some(arg) => match arg.value.as_str() {
            Some(val) => {
                CONFIG.lock().unwrap().pass = val.to_string();
            }
            None => {}
        },
        None => {}
    };
    match matches.args.get("keyfile") {
        Some(arg) => match arg.value.as_str() {
            Some(val) => {
                CONFIG.lock().unwrap().keyfile = val.to_string();
            }
            None => {}
        },
        None => {}
    };
}

pub fn check_config() -> bool {
    if CONFIG.host.is_empty() {
        info!("Server host not set");
        return false;
    }
    if CONFIG.port == 0 {
        info!("Server port not set");
        return false;
    }
    if CONFIG.user.is_empty() {
        info!("Server user not set");
        return false;
    }
    if CONFIG.pass.is_empty() && CONFIG.keyfile.is_empty() {
        info!("Server password or keyfile not set");
        return false;
    }
    true
}
