//! Shared by the test binaries that take a lock right after releasing it.
use std::time::{Duration, Instant};

/// Retries `attempt` for a short, bounded time and returns its last result.
///
/// A `flock` belongs to the open file description, not to the descriptor. A child shares that
/// description from `fork` until its `exec` closes the close-on-exec copy, and other tests in the
/// same binary start processes on other threads. So a lock that a test has just released can
/// stay held for an instant, and an attempt made in that instant fails, at times, on Linux. The
/// provider and its drainer wait for the lock in real time, so they are not affected. A test is,
/// where it takes the lock in one attempt, or under a paused clock, where every retry of a wait
/// happens at once. A lock still held when the time is up is a real failure, and is returned.
pub fn settled<T, E>(mut attempt: impl FnMut() -> Result<T, E>) -> Result<T, E> {
    let until = Instant::now() + Duration::from_secs(5);
    loop {
        match attempt() {
            Err(_) if Instant::now() < until => std::thread::sleep(Duration::from_millis(10)),
            result => return result,
        }
    }
}
