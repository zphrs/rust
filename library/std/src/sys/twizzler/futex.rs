use crate::sync::atomic::AtomicU32;
use crate::time::Duration;
use twizzler_abi::syscall::{
    sys_thread_sync, ThreadSync, ThreadSyncError, ThreadSyncFlags, ThreadSyncOp,
    ThreadSyncReference, ThreadSyncSleep, ThreadSyncWake,
};

/// Wait for a futex_wake operation to wake us.
///
/// Returns directly if the futex doesn't hold the expected value.
///
/// Returns false on timeout, and true in all other cases.
pub fn futex_wait(futex: &AtomicU32, expected: u32, timeout: Option<Duration>) -> bool {
    use crate::ptr::null;
    use crate::sync::atomic::Ordering::Relaxed;

    // No need to wait if the value already changed.
    if futex.load(Relaxed) != expected {
        return true;
    }

    let sleep = ThreadSync::new_sleep(ThreadSyncSleep::new(
        ThreadSyncReference::Virtual32(futex),
        expected.into(),
        ThreadSyncOp::Equal,
        ThreadSyncFlags::empty(),
    ));
    let r = sys_thread_sync(&mut [sleep], timeout);

    match r {
        Err(ThreadSyncError::Timeout) => return false,
        _ => return true,
    }
}

/// Wake up one thread that's blocked on futex_wait on this futex.
///
/// Returns true if this actually woke up such a thread,
/// or false if no thread was waiting on this futex.
///
/// On some platforms, this always returns false.
pub fn futex_wake(futex: &AtomicU32) -> bool {
    let wake = ThreadSync::new_wake(ThreadSyncWake::new(ThreadSyncReference::Virtual32(futex), 1));
    let _ = sys_thread_sync(&mut [wake], None);
    // TODO
    false
}

/// Wake up all threads that are waiting on futex_wait on this futex.
pub fn futex_wake_all(futex: &AtomicU32) {
    let wake = ThreadSync::new_wake(ThreadSyncWake::new(
        ThreadSyncReference::Virtual32(futex),
        usize::MAX,
    ));
    let _ = sys_thread_sync(&mut [wake], None);
}
