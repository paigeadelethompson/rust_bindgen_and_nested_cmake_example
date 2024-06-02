use libhello_bindings;
pub struct HelloLib {

}

impl HelloLib {
    pub fn say_hello() -> i64 {
        unsafe {
            libhello_bindings::ohai2u().into()
        }
    }
}

#[test]
fn is_42() {
    assert_eq!(HelloLib::say_hello(), 42);
}