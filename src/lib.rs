#![no_std]

use core::panic::PanicInfo;

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

use windows_kernel_sys::base::{DRIVER_OBJECT, NTSTATUS, UNICODE_STRING, STATUS_SUCCESS};
use windows_kernel_sys::ntoskrnl::DbgPrint;

#[no_mangle]
pub extern "system" fn driver_entry(driver: &mut DRIVER_OBJECT, _: *const UNICODE_STRING) -> NTSTATUS {
    unsafe {
        DbgPrint("Hello, world!\0".as_ptr() as _);
    }

    driver.DriverUnload = Some(driver_exit);

    STATUS_SUCCESS
}

#[no_mangle]
pub unsafe extern "C" fn driver_exit(driver: *mut DRIVER_OBJECT) {
    DbgPrint("Bye bye!\0".as_ptr() as _);
}