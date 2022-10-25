pub mod util {

    use std::ffi::CString;

    pub fn cstr_whitespace(len: usize) -> CString {
        let mut buffer: Vec<u8> = Vec::with_capacity(len);
        buffer.extend([b' '].iter().cycle().take(len));
        unsafe {CString::from_vec_unchecked(buffer)}
    }

}