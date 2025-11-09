#![no_std]
#![no_main]

use cafeos as _;
use cafeos::log;
use cafeos::screen::OSScreen;

extern "C" fn save_callback() {
    unsafe { cafeos_sys::OSSavesDone_ReadyToRelease(); }
}

#[unsafe(no_mangle)]
fn entry(_argc: isize, _argv: *const *const u8) -> isize {
    unsafe {
        cafeos_sys::ProcUIInit(Some(save_callback));
    }
    log::report(c"hello!");

    let screen = OSScreen::init().unwrap();

    loop {
        unsafe {
            let status = cafeos_sys::ProcUIProcessMessages(1);
            match status {
                cafeos_sys::ProcUIStatus::PROCUI_STATUS_IN_FOREGROUND => {}
                cafeos_sys::ProcUIStatus::PROCUI_STATUS_RELEASE_FOREGROUND => {
                    cafeos_sys::ProcUIDrawDoneRelease();
                    continue;
                }
                cafeos_sys::ProcUIStatus::PROCUI_STATUS_EXITING => { break; }
                _ => { continue; }
            }
        }

        screen.text("hello world!", 0, 0);
        screen.flip();
    }


    unsafe {
        cafeos_sys::ProcUIShutdown();
    }

    log::report(c"bye!");

    0
}
