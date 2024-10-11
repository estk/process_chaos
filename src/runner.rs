use rand::Rng;
use std::{env, os::unix::process::CommandExt, process::Command, thread::sleep, time::Duration};
use tracing::info;

const DEPTH_ENV: &str = "CHAOS_DEPTH";
const CHILD_COUNT_ENV: &str = "CHAOS_CHILD_COUNT_MAX";
const DEFAULT_CHILD_COUNT: usize = 10;
const MAX_SLEEP_MS: u64 = 500;

pub fn run() {
    let mut rng = rand::thread_rng();

    let child_count_max = env::var(CHILD_COUNT_ENV)
        .as_deref()
        .map(|v| v.parse().expect("unable to parse"))
        .unwrap_or(DEFAULT_CHILD_COUNT);
    let depth: usize = env::var(DEPTH_ENV)
        .as_deref()
        .map(|v| v.parse().expect("unable to parse"))
        .unwrap_or_default();

    info!("depth: {depth}, child_max: {child_count_max}");

    let mut handles = vec![];

    let child_count = rng.gen_range(0..=child_count_max);
    for _ in 0..child_count {
        let new_group = rng.gen::<bool>();
        let mut cmd = Command::new(env::current_exe().unwrap());
        if new_group {
            cmd.process_group(0);
        }
        let h = cmd
            .env(DEPTH_ENV, (depth + 1).to_string())
            .env(CHILD_COUNT_ENV, (child_count - 2).to_string())
            .env("CHAOS_RUNNER", "true")
            .spawn()
            .expect("Failed to spawn process");
        handles.push(h);
    }

    let exit_early = rng.gen::<bool>();
    if exit_early {
        let code = rng.gen::<u8>();
        std::process::exit(code as i32);
    }

    let sleep_millis = rng.gen_range(0..=MAX_SLEEP_MS);
    let dur = Duration::from_millis(sleep_millis);
    sleep(dur);

    for mut h in handles {
        let should_wait = rng.gen::<bool>();
        if should_wait {
            if let Err(res) = h.wait() {
                info!("child exited in error: {res:?}")
            }
        }
    }
}
