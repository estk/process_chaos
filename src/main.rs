use std::{
    env,
    process::{self, Command},
    thread::{spawn, JoinHandle},
};

use clap::Parser;
use nix::{sys::wait::waitpid, unistd::Pid};
use tracing::{error, info};
use tracing_subscriber::{fmt, prelude::*, EnvFilter};

#[derive(Parser, Debug)]
struct Cli {
    #[clap(env = "CHAOS_RUNNER", short, long)]
    runner: bool,
}

fn main() {
    tracing_subscriber::registry()
        .with(fmt::layer())
        .with(EnvFilter::from_default_env())
        .init();

    let cli = Cli::parse();
    info!("cli: {cli:?}");

    if cli.runner {
        chaos::runner::run()
    } else {
        info!("starting manager");
        let m = manage();
        exec_runner();
        info!("runner_done");
        let success = m.join().unwrap();
        if !success {
            error!("Manager did not exit successfully");
            process::exit(1);
        }
        info!("ps:");
        Command::new("ps").spawn().unwrap().wait();
    }
}
fn manage() -> JoinHandle<bool> {
    nix::sys::prctl::set_child_subreaper(true);
    let h = spawn(|| loop {
        match waitpid(Pid::from_raw(-1), None) {
            Ok(status) => {
                info!("collected {status:?}")
            }
            Err(e) => {
                if e == nix::errno::Errno::ECHILD {
                    return true;
                } else {
                    error!("waitpid exited: {e}");
                    return false;
                }
            }
        }
    });
    h
}
fn exec_runner() {
    let mut cmd = Command::new(env::current_exe().unwrap());
    cmd.env("CHAOS_RUNNER", "true");
    cmd.output();
}
