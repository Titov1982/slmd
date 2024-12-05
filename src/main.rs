use std::fs::File;
use std::io::Write;
use std::time::Duration;
use chrono::Local;
use daemonize::{Daemonize, Group, User};
use sysinfo::System;
use crate::config::Config;
use crate::serialize_obj::SerializeObj;

mod config;
mod serialize_obj;

pub struct Daemon<'a> {
    daemon: Daemonize<&'a str>,
    tick_count: usize,
    tick_rate: u64,
    data_file_path: String,
}

impl Daemon<'_> {
    pub fn new(config: Config)  -> Daemon<'static>{

        let stdout = File::create(config.out_file_path).unwrap_or_else(|error| {
            panic!("{}", format!("{} Error: {}", Local::now(), error));
        });
        let stderr = File::create(config.err_file_path).unwrap_or_else(|error| {
            panic!("{}", format!("{} Error: {}", Local::now(), error));
        });

        let daemonize = Daemonize::new()
            .pid_file(format!("{}/slmd.pid", config.working_dir))
            .chown_pid_file(true)
            .working_directory(config.working_dir)
            .user(User::from(config.user_name.as_str()))
            .group(Group::from(config.group_name.as_str()))
            .chroot(config.root_dir)
            .umask(0o022)
            .stdout(stdout)
            .stderr(stderr)
            .privileged_action(|| "Executed before drop privileges");

        Daemon {
            daemon: daemonize,
            tick_count: config.tick_count,
            tick_rate: config.tick_rate,
            data_file_path: config.data_file_path,
        }
    }

    pub fn run(self) {
        match &self.daemon.start() {
            Ok(_) => Daemon::daemon_main(&self.data_file_path, self.tick_count, self.tick_rate),
            Err(e) => eprintln!("Error, {}", e),
        }
    }

    fn daemon_main(data_file_path: &String, tick_count: usize, tick_rate: u64) {
        println!("{}", format!("{} SLM daemon start successful.", Local::now()));

        let mut tick: usize = 0;
        let mut system_state = System::new();

        let mut data_fd = File::create(&data_file_path).unwrap_or_else(|error| {
            panic!("{}", format!("{} Error: {}", Local::now(), error));
        });

        loop {
            if tick < tick_count {
                system_state.refresh_all();
                let obj = SerializeObj::new(tick, system_state.global_cpu_usage(), system_state.used_memory());

                data_fd.write(format!("{},{},{}\n", obj.tick, obj.cpu_usage, obj.mem_used).as_bytes()).unwrap_or_else(|error| {
                    panic!("{}", format!("{} Error: {}", Local::now(), error));
                });
                data_fd.flush().unwrap_or_else(|error| {
                    panic!("{}", format!("{} Error: {}", Local::now(), error));
                });

                tick += 1;
                std::thread::sleep(Duration::from_millis(tick_rate));
            } else {
                tick = 0;

                data_fd = std::fs::OpenOptions::new().write(true).truncate(true).open(&data_file_path).unwrap_or_else(|error| {
                    panic!("{}", format!("{} Error: {}", Local::now(), error));
                });
            }
        }
    }

}

fn main() {

    let cfg: Config = confy::load_path("./slmd.toml").unwrap_or_else(|error| {
        panic!("{}", format!("{} Error: {}", Local::now(), error));
    });

    let daemon = Daemon::new(cfg);
    daemon.run();
}
