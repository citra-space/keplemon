use crate::GETSETSTRLEN;
use std::ffi::CStr;
use std::os::raw::c_char;

#[derive(Debug, Default)]
pub struct GetSetString {
    buffer: Vec<u8>,
}

impl GetSetString {
    pub fn new() -> Self {
        GetSetString {
            buffer: vec![0u8; GETSETSTRLEN + 1],
        }
    }

    pub fn set(&mut self, index: usize, value: &str) -> Result<(), String> {
        let value = value.as_bytes();
        let last_index = index + value.len();
        if last_index <= GETSETSTRLEN {
            self.buffer[index..last_index].copy_from_slice(value);
            Ok(())
        } else {
            Err("Value exceeds buffer length".to_string())
        }
    }

    pub fn pointer(&mut self) -> *mut c_char {
        self.buffer.as_mut_ptr() as *mut c_char
    }

    pub fn value(&self) -> String {
        let c_str = unsafe { CStr::from_ptr(self.buffer.as_ptr() as *const c_char) };
        c_str.to_string_lossy().to_string()
    }
}

impl From<&str> for GetSetString {
    fn from(value: &str) -> Self {
        let mut buffer = vec![0u8; GETSETSTRLEN + 1];
        let value = value.as_bytes();
        let len = std::cmp::min(GETSETSTRLEN, value.len());
        buffer[..len].copy_from_slice(&value[..len]);
        GetSetString { buffer }
    }
}
