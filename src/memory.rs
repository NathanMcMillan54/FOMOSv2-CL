/* src/memory.rs
 *
 * Build/compile:
 * cargo build
 *
 * Description:
 * Alloc memory management
 */

use core::alloc::{GlobalAlloc, Layout};
use core::ptr::{null_mut};

struct MManagement;

unsafe impl GlobalAlloc for MManagement {
    unsafe fn alloc(&self, _layout: Layout) -> *mut u8 {
        null_mut()
    }
    unsafe fn dealloc(&self, _ptr: *mut u8, _layout: Layout) {
        loop {  }
    }
}

#[global_allocator]
static ALLOCATOR: MManagement = MManagement;

