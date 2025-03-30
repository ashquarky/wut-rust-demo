use core::ffi::CStr;

pub fn report(message: &CStr) {
    unsafe { cafeos_sys::OSReport(message.as_ptr()); }
}

pub fn verbose(message: &CStr) {
    unsafe { cafeos_sys::OSReportVerbose(message.as_ptr()); }
}

pub fn info(message: &CStr) {
    unsafe { cafeos_sys::OSReportInfo(message.as_ptr()); }
}

pub fn warn(message: &CStr) {
    unsafe { cafeos_sys::OSReportWarn(message.as_ptr()); }
}