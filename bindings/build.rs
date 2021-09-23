fn main() {
    windows::build! {
        Windows::Win32::System::Diagnostics::Debug::GetLastError,
        Windows::Win32::UI::KeyboardAndMouseInput::SendInput,
        Windows::Win32::Graphics::Gdi::{
            BI_RGB,
            GetDC,
            ReleaseDC,
            CreateCompatibleDC,
            DeleteDC,
            CreateCompatibleBitmap,
            DeleteObject,
            SelectObject,
            BitBlt,
            GetDIBits,
        },
    };
}