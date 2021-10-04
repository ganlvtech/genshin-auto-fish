fn main() {
    windows::build! {
        Windows::Win32::System::Diagnostics::Debug::GetLastError,
        Windows::Win32::System::Diagnostics::Debug::Beep,
        Windows::Win32::UI::KeyboardAndMouseInput::SendInput,
        Windows::Win32::UI::HiDpi::SetProcessDpiAwareness,
        Windows::Win32::UI::WindowsAndMessaging::{
            FindWindowW,
            GetClientRect,
        },
        Windows::Win32::Graphics::Gdi::{
            BI_RGB,
            GetDC,
            ReleaseDC,
            GetCurrentObject,
            GetDIBits,
        },
    };
}