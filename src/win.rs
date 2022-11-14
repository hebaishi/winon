#[cfg(windows)]
extern crate user32;

use crate::util;
use widestring::Utf16String;

pub fn get_active_title() -> String {
    let result: Utf16String;
    unsafe {
        let handle = user32::GetForegroundWindow();
        let buffer_size: usize = user32::GetWindowTextLengthW(handle).try_into().unwrap();
        let mut window_text = util::zeroes(buffer_size + 1);
        user32::GetWindowTextW(
            handle,
            window_text.as_mut_ptr(),
            window_text.len().try_into().unwrap(),
        );
        result = Utf16String::from_vec_unchecked(window_text);
    }
    result.to_string()
}
