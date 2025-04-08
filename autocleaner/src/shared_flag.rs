#![allow(dead_code)]
#![allow(unused_imports)]

use once_cell::sync::OnceCell;
use shared_memory::*;
use std::{mem::size_of, sync::atomic::AtomicBool};

static FLAG: OnceCell<&'static AtomicBool> = OnceCell::new();
static mut SHMEM_HANDLE: Option<Shmem> = None;

/// Returns a reference to a shared `AtomicBool` used for inter-process signaling.
///
/// # Parameters
///
/// - `name`: A unique name used as the OS identifier for the shared memory region.
///
/// # Returns
///
/// - A `'static` reference to an `AtomicBool` stored in shared memory.
///
pub fn get_or_create_shared_flag_tray(name: &str) -> &'static AtomicBool {
    FLAG.get_or_init(|| {
        let shmem = ShmemConf::new()
            .os_id(name)
            .open()
            .or_else(|_| {
                ShmemConf::new()
                    .size(size_of::<AtomicBool>())
                    .os_id(name)
                    .create()
            })
            .expect("Failed to open/create shared memory");

        let ptr = shmem.as_ptr() as *mut AtomicBool;

        unsafe {
            if shmem.is_owner() {
                ptr.write(AtomicBool::new(false));
            }
            SHMEM_HANDLE = Some(shmem);
            &*ptr
        }
    })
}
