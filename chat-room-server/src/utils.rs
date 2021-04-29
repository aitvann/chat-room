use jemalloc_ctl::{stats, epoch};

pub fn memory_usage() -> usize {
    epoch::advance().expect("failed to advance");

    stats::allocated::read()
        .expect("failed to get allocated memory") as usize
}
