use std::mem::size_of;

use bindings::Windows::Win32::System::Diagnostics::Debug::GetLastError;
use bindings::Windows::Win32::UI::KeyboardAndMouseInput::{INPUT, INPUT_0, INPUT_MOUSE, MOUSEEVENTF_LEFTDOWN, MOUSEEVENTF_LEFTUP, MOUSEINPUT, SendInput};

pub fn set_mouse_state(is_mouse_down: bool) -> Result<(), String> {
    let dwflags = if is_mouse_down { MOUSEEVENTF_LEFTDOWN } else { MOUSEEVENTF_LEFTUP };
    let input = [INPUT {
        r#type: INPUT_MOUSE,
        Anonymous: INPUT_0 {
            mi: MOUSEINPUT {
                dx: 0,
                dy: 0,
                mouseData: 0,
                dwFlags: dwflags,
                time: 0,
                dwExtraInfo: 0,
            }
        },
    }; 1];
    unsafe {
        let result = SendInput(input.len() as u32, input.as_ptr(), (size_of::<INPUT>() * input.len()) as i32);
        if result == 0 {
            let error = GetLastError();
            Err(format!("SendInput error {:?}", error))
        } else {
            Ok(())
        }
    }
}