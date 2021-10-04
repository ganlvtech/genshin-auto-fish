use std::mem::size_of;
use std::ptr::null_mut;
use std::thread;

use windows::Handle;

use bindings::Windows::Win32::Foundation::{HWND, RECT};
use bindings::Windows::Win32::Graphics::Gdi::{BI_RGB, BITMAPINFO, BITMAPINFOHEADER, DIB_RGB_COLORS, GetCurrentObject, GetDC, GetDIBits, HBITMAP, HDC, OBJ_BITMAP, ReleaseDC, RGBQUAD};
use bindings::Windows::Win32::System::Diagnostics::Debug::{Beep, GetLastError};
use bindings::Windows::Win32::UI::HiDpi::{PROCESS_PER_MONITOR_DPI_AWARE, SetProcessDpiAwareness};
use bindings::Windows::Win32::UI::KeyboardAndMouseInput::{INPUT, INPUT_0, INPUT_MOUSE, MOUSEEVENTF_LEFTDOWN, MOUSEEVENTF_LEFTUP, MOUSEINPUT, SendInput};
use bindings::Windows::Win32::UI::WindowsAndMessaging::{FindWindowW, GetClientRect};

use crate::Color;

struct ManagedHDC(HWND, HDC);

impl Drop for ManagedHDC {
    fn drop(&mut self) {
        unsafe { ReleaseDC(self.0, self.1); }
    }
}

pub struct WindowCapture {
    class_name: String,
    window_name: String,
    bytes_per_pixel: usize,
    hwnd: Option<HWND>,
    bitmap_info: BITMAPINFO,
    buffer: Vec<u8>,
    hdc: Option<ManagedHDC>,
    hbm: Option<HBITMAP>,
}

impl WindowCapture {
    const BYTES_PER_PIXEL: usize = 4;

    pub fn new(class_name: String, window_name: String) -> Self {
        WindowCapture {
            class_name,
            window_name,
            bytes_per_pixel: Self::BYTES_PER_PIXEL,
            hwnd: None,
            bitmap_info: BITMAPINFO {
                bmiHeader: BITMAPINFOHEADER {
                    biSize: size_of::<BITMAPINFOHEADER>() as u32,
                    biWidth: 0,
                    biHeight: 0,
                    biPlanes: 1,
                    biBitCount: (Self::BYTES_PER_PIXEL * 8) as u16,
                    biCompression: BI_RGB as u32,
                    biSizeImage: 0,
                    biXPelsPerMeter: 0,
                    biYPelsPerMeter: 0,
                    biClrUsed: 0,
                    biClrImportant: 0,
                },
                bmiColors: [RGBQUAD {
                    rgbBlue: 0,
                    rgbGreen: 0,
                    rgbRed: 0,
                    rgbReserved: 0,
                }],
            },
            hdc: None,
            hbm: None,
            buffer: Vec::new(),
        }
    }
    #[inline]
    pub fn is_window_found(&self) -> bool {
        self.hwnd.is_some()
    }
    #[inline]
    pub fn get_width(&self) -> usize {
        self.bitmap_info.bmiHeader.biWidth as usize
    }
    #[inline]
    pub fn get_height(&self) -> usize {
        self.bitmap_info.bmiHeader.biHeight as usize
    }

    fn update_hwnd(&mut self) -> Result<(), ()> {
        if self.hwnd.is_none() {
            let hwnd = unsafe { FindWindowW(self.class_name.clone(), self.window_name.clone()) };
            if !hwnd.is_invalid() {
                self.hwnd = Some(hwnd);
                Ok(())
            } else {
                Err(())
            }
        } else {
            Ok(())
        }
    }
    fn update_bitmap_info(&mut self) -> Result<(), ()> {
        if let Some(hwnd) = self.hwnd {
            let mut rect = RECT {
                left: 0,
                top: 0,
                right: 0,
                bottom: 0,
            };
            let result = unsafe { GetClientRect(hwnd, &mut rect as *mut RECT) };
            if result.as_bool() {
                self.bitmap_info.bmiHeader.biWidth = rect.right - rect.left;
                self.bitmap_info.bmiHeader.biHeight = rect.bottom - rect.top;
                if self.bitmap_info.bmiHeader.biWidth >= 0 || self.bitmap_info.bmiHeader.biHeight >= 0 {
                    let len = (self.bitmap_info.bmiHeader.biWidth * self.bitmap_info.bmiHeader.biHeight * (self.bytes_per_pixel as i32)) as usize;
                    self.buffer.resize(len, 0);
                }
                Ok(())
            } else {
                self.hwnd = None;
                Err(())
            }
        } else {
            Err(())
        }
    }
    fn update_gdi_objects(&mut self) -> Result<(), ()> {
        if self.hdc.is_none() || self.hbm.is_none() {
            if let Some(hwnd) = self.hwnd {
                let hdc = unsafe { GetDC(hwnd) };
                if !hdc.is_invalid() {
                    let hgdiobj = unsafe { GetCurrentObject(hdc, OBJ_BITMAP) };
                    if !hgdiobj.is_invalid() {
                        let hbm = HBITMAP(hgdiobj.0);
                        self.hdc = Some(ManagedHDC(hwnd, hdc));
                        self.hbm = Some(hbm);
                        Ok(())
                    } else {
                        self.hwnd = None;
                        Err(())
                    }
                } else {
                    self.hwnd = None;
                    Err(())
                }
            } else {
                Err(())
            }
        } else {
            Ok(())
        }
    }
    fn update_buffer(&mut self) -> Result<(), ()> {
        if let Some(ManagedHDC(_, hdc)) = self.hdc {
            if let Some(hbm) = self.hbm {
                let cy = self.get_height();
                if cy > 0 {
                    let ptr = self.buffer.as_mut_ptr().cast();
                    let result = unsafe { GetDIBits(hdc, hbm, 0, cy as u32, ptr, &mut self.bitmap_info as *mut BITMAPINFO, DIB_RGB_COLORS) };
                    if result != 0 {
                        Ok(())
                    } else {
                        let err = unsafe { GetLastError() };
                        println!("{:?}", err);
                        self.hdc = None;
                        self.hbm = None;
                        self.hwnd = None;
                        Err(())
                    }
                } else {
                    self.hdc = None;
                    self.hbm = None;
                    self.hwnd = None;
                    Err(())
                }
            } else {
                Err(())
            }
        } else {
            Err(())
        }
    }
    pub fn capture(&mut self) -> Result<(), ()> {
        self.update_hwnd()?;
        self.update_bitmap_info()?;
        self.update_gdi_objects()?;
        self.update_buffer()?;
        Ok(())
    }

    #[inline]
    unsafe fn get_pixel_offset_unchecked(&self, x: usize, y: usize) -> usize {
        let cx = self.get_width();
        let cy = self.get_height();
        ((cy - 1 - y) * cx + x) * self.bytes_per_pixel
    }

    pub unsafe fn get_color_unchecked(&self, x: usize, y: usize) -> Color {
        let offset = self.get_pixel_offset_unchecked(x, y);
        let b = self.buffer[offset];
        let g = self.buffer[offset + 1];
        let r = self.buffer[offset + 2];
        Color(r, g, b)
    }
}

pub fn set_mouse_state(is_down: bool) -> Result<(), ()> {
    let dwflags = if is_down { MOUSEEVENTF_LEFTDOWN } else { MOUSEEVENTF_LEFTUP };
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
            Err(())
        } else {
            Ok(())
        }
    }
}

pub fn set_dpi_aware() {
    unsafe { SetProcessDpiAwareness(PROCESS_PER_MONITOR_DPI_AWARE); }
}

pub fn beep(freq: u32, millis: u32) {
    thread::spawn(move || {
        unsafe { Beep(freq, millis); }
    });
}












