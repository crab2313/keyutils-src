use std::{ffi::CStr, os::raw::c_char};

extern "C" {
    static keyutils_version_string: c_char;
}

pub fn version() -> &'static str {
    // https://github.com/rust-lang/rust/issues/54450
    unsafe {
        let address = &keyutils_version_string as *const c_char;
        CStr::from_ptr(address).to_str().unwrap()
    }
}

#[cfg(test)]
mod test {
    use super::version;

    #[test]
    fn version_works() {
        let version = version();
        println!("{:?}", version);
        assert_eq!(version, "keyutils-1.6.3");
    }
}
