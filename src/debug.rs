static mut DEBUG: bool = false;

pub fn set() {
    unsafe { DEBUG = true };
}

pub fn get() -> bool {
    unsafe { DEBUG }
}