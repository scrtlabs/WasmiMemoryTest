#[no_mangle]
pub extern "C" fn get_first_i32() -> i32 {
    unsafe { (1 as *const i32).read() }
}
