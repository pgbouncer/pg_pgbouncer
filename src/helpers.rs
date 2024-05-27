use crate::*;

pub fn postgres_port() -> i32 {
    unsafe { pg_sys::PostPortNumber }
}

pub fn postgres_socket_dirs() -> Result<String> {
    unsafe {
        let dir = pg_sys::Unix_socket_directories;
        let dir_str = std::ffi::CStr::from_ptr(dir).to_str()?;
        Ok(dir_str.to_string())
    }
}

pub fn escape_double_quotes(s: &str) -> String {
    s.replace('\"', "\"\"")
}

pub fn contains_only_ascii_alphanumeric_and_underscore(s: &str) -> bool {
    s.chars().all(|c| c.is_ascii_alphanumeric() || c == '_')
}
