extern crate alloc;
use core::{
    alloc::{GlobalAlloc, Layout},
    cmp,
    panic::PanicInfo,
};
use core::ffi::{c_void};
use core::ffi::CStr;
use core::hint::unreachable_unchecked;
use crate::stack_c_string::StackCString;

// Default allocator implementation
pub struct MEMDefaultHeapAllocator;

unsafe impl GlobalAlloc for MEMDefaultHeapAllocator {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        unsafe { cafeos_sys::MEMAllocFromDefaultHeapEx.unwrap()(layout.size() as u32, cmp::max(layout.align() as i32, 4))
            as *mut u8 }
    }

    unsafe fn dealloc(&self, ptr: *mut u8, _layout: Layout) {
        unsafe { cafeos_sys::MEMFreeToDefaultHeap.unwrap()(ptr as *mut c_void); }
    }
}

#[panic_handler]
fn panic_handler(info: &PanicInfo) -> ! {
    let payload = info.message();
    let message = if let Some(s) = payload.as_str() {
        s
    } else {
        "Unhandled rust panic payload!"
    };
    let (filename, line) = if let Some(loc) = info.location() {
        (loc.file(), loc.line())
    } else {
        ("unknown.rs", 0)
    };

    // Copy the message and filename to the stack in order to safely add
    // a terminating nul character (since rust strings don't come with one).
    let message = StackCString::<256>::from(message);
    let filename = StackCString::<128>::from(filename);
    unsafe {
        cafeos_sys::OSPanic(
            filename.c_str().as_ptr(),
            line,
            message.c_str().as_ptr(),
        );
        unreachable_unchecked()
    };
}

pub fn fatal(msg: &CStr) -> ! {
    unsafe {
        cafeos_sys::OSFatal(msg.as_ptr());
        unreachable_unchecked()
    }
}

#[alloc_error_handler]
fn alloc_error(_layout: Layout) -> ! {
    fatal(c"alloc_error")
}
