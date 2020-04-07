#[no_mangle]
pub extern "C" fn get_first_i32() -> i32 {
    unsafe {
        // cannot read from address 0
        // we read from address 4 because rust wants i32 to be aligned to 4
        // we tried to read from 1 and it also worked,
        // but technically it's undefined in rust and we could get errors
        (4 as *const i32).read()
    }
}
