use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    pub out_file_path: String,
    pub err_file_path: String,
    pub data_file_path: String,
    pub working_dir: String,
    pub root_dir: String,
    pub user_name: String,
    pub group_name: String,
    pub tick_count: usize,
    pub tick_rate: u64,
}

impl Default for Config {
    fn default() -> Self {
        Config {
            out_file_path: "/tmp/slmd.out".to_string(),
            err_file_path: "/tmp/slmd.err".to_string(),
            data_file_path: "/tmp/slmd.dat".to_string(),
            working_dir: "/tmp".to_string(),
            root_dir: "/".to_string(),
            user_name: "root".to_string(),
            group_name: "wheel".to_string(),
            tick_count: 3600,
            tick_rate: 1000,
        }
    }
}