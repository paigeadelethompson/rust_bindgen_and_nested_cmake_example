use libhello_bindings;
pub struct HelloLib {

}

impl HelloLib {
    pub fn say_hello() {
        unsafe {
            libhello_bindings::ohai2u();
        }
    }
}