use crate::ffi::CStr;
use crate::io;
use crate::num::NonZeroUsize;
use crate::sys::twizzler::thread_local_dtor::run_dtors;
use crate::time::Duration;
use twizzler_abi::syscall::sys_thread_sync;

pub struct Thread {
    internal_id: u32,
}

unsafe impl Send for Thread {}
unsafe impl Sync for Thread {}

pub const DEFAULT_MIN_STACK_SIZE: usize = (1 << 20) * 2;

impl Thread {
    // unsafe: see thread::Builder::spawn_unchecked for safety requirements
    pub unsafe fn new(stack: usize, p: Box<dyn FnOnce()>) -> io::Result<Thread> {
        let p = Box::into_raw(box p);
        let internal_id = twizzler_abi::thread::spawn(stack, thread_start as usize, p as usize);

        return if let Some(internal_id) = internal_id {
            Ok(Thread { internal_id })
        } else {
            drop(Box::from_raw(p));
            Err(io::const_io_error!(io::ErrorKind::Uncategorized, &"Unable to create thread!"))
        };

        unsafe extern "C" fn thread_start(main: usize) -> ! {
            // TODO (urgent): does this leak p?
            Box::from_raw(main as *mut Box<dyn FnOnce()>)();
            run_dtors();
            twizzler_abi::thread::exit();
        }
    }

    pub fn yield_now() {
        twizzler_abi::syscall::sys_thread_yield();
    }

    pub fn set_name(_name: &CStr) {
        //twizzler_abi::thread::set_name(name)
    }

    pub fn sleep(dur: Duration) {
        let _ = sys_thread_sync(&mut [], Some(dur));
    }

    pub fn join(self) {
        let _ = unsafe { twizzler_abi::thread::join(self.internal_id) };
    }

    #[allow(dead_code)]
    #[inline]
    pub fn id(&self) -> u32 {
        self.internal_id
    }
}

pub fn available_parallelism() -> io::Result<NonZeroUsize> {
    Ok(twizzler_abi::syscall::sys_info().cpu_count())
}

pub mod guard {
    pub type Guard = !;
    pub unsafe fn current() -> Option<Guard> {
        None
    }
    pub unsafe fn init() -> Option<Guard> {
        None
    }
}
