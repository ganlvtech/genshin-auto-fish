use std::mem::size_of;

use windows::Handle;

use bindings::Windows::Win32::Graphics::Gdi::{BI_RGB, BitBlt, BITMAPINFO, BITMAPINFOHEADER, CreateCompatibleBitmap, CreateCompatibleDC, CreatedHDC, DeleteDC, DeleteObject, DIB_RGB_COLORS, GetDC, GetDIBits, HBITMAP, HDC, HGDIOBJ, ReleaseDC, RGBQUAD, SelectObject, SRCCOPY};
use bindings::Windows::Win32::System::Diagnostics::Debug::GetLastError;

use crate::Color;

pub struct ScreenCaptureState {
    left: i32,
    top: i32,
    right: i32,
    bottom: i32,
    h_screen_dc: HDC,
    h_mem_dc: CreatedHDC,
    h_old_bitmap: HGDIOBJ,
    h_bitmap: HBITMAP,
    bitmap_info: BITMAPINFO,
    buffer: Vec<u8>,
}

impl ScreenCaptureState {
    pub fn new(left: i32, top: i32, right: i32, bottom: i32) -> Result<ScreenCaptureState, String> {
        let cx = right - left;
        let cy = bottom - top;
        let bitmap_info = BITMAPINFO {
            bmiHeader: BITMAPINFOHEADER {
                biSize: size_of::<BITMAPINFOHEADER>() as u32,
                biWidth: cx,
                biHeight: cy,
                biPlanes: 1,
                biBitCount: 32,
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
            }; 1],
        };
        let size = (((cx * (bitmap_info.bmiHeader.biBitCount as i32) + 31) / 32) * 4 * cy) as usize;
        let buffer = vec![0u8; size];
        unsafe {
            let result;
            let h_screen_dc = GetDC(None);
            if !h_screen_dc.is_invalid() {
                let h_mem_dc = CreateCompatibleDC(h_screen_dc);
                if !h_mem_dc.is_invalid() {
                    let h_bitmap = CreateCompatibleBitmap(h_screen_dc, cx, cy);
                    if !h_bitmap.is_invalid() {
                        let h_old_bitmap = SelectObject(HDC(h_mem_dc.0), HGDIOBJ(h_bitmap.0));
                        if !h_old_bitmap.is_invalid() {
                            return Ok(ScreenCaptureState {
                                left,
                                top,
                                right,
                                bottom,
                                h_screen_dc,
                                h_mem_dc,
                                h_bitmap,
                                h_old_bitmap,
                                bitmap_info,
                                buffer,
                            });
                        } else {
                            result = Err("Select compatible bitmap error".to_string());
                        }
                        if !DeleteObject(HGDIOBJ(h_bitmap.0)).as_bool() {
                            panic!("Delete HBITMAP object error")
                        }
                    } else {
                        result = Err("Create compatible bitmap error".to_string());
                    }
                    if !DeleteDC(h_mem_dc).as_bool() {
                        panic!("Delete memory DC error")
                    }
                } else {
                    result = Err("Create compatible DC error".to_string());
                }
                if ReleaseDC(None, h_screen_dc) != 1 {
                    panic!("Release screen DC error")
                }
            } else {
                result = Err("Get screen DC error".to_string());
            }
            result
        }
    }

    pub fn capture(&mut self) -> Result<(), String> {
        let cx = self.bitmap_info.bmiHeader.biWidth;
        let cy = self.bitmap_info.bmiHeader.biHeight;
        unsafe {
            let result = BitBlt(HDC(self.h_mem_dc.0), 0, 0, cx, cy, self.h_screen_dc, self.left, self.top, SRCCOPY);
            if result.as_bool() {
                let ptr = self.buffer.as_mut_ptr().cast();
                let result = GetDIBits(HDC(self.h_mem_dc.0), self.h_bitmap, 0, cy as u32, ptr, &mut self.bitmap_info, DIB_RGB_COLORS);
                if result != 0 {
                    Ok(())
                } else {
                    Err("GetDIBits error".to_string())
                }
            } else {
                let error = GetLastError();
                Err(format!("BitBlt error, error code = {:?}", error))
            }
        }
    }

    pub fn get_color(&self, screen_x: i32, screen_y: i32) -> Result<Color, String> {
        if screen_x >= self.left && screen_x < self.right && screen_y >= self.top && screen_y < self.bottom {
            let cx = self.bitmap_info.bmiHeader.biWidth;
            let bytes_per_pixel = (self.bitmap_info.bmiHeader.biBitCount / 8) as i32;
            let offset = (((self.bottom - 1 - screen_y) * cx + (screen_x - self.left)) * bytes_per_pixel) as usize;
            let b = self.buffer[offset];
            let g = self.buffer[offset + 1];
            let r = self.buffer[offset + 2];
            Ok(Color(r, g, b))
        } else {
            Err("Not in range".to_string())
        }
    }
}

impl Drop for ScreenCaptureState {
    fn drop(&mut self) {
        unsafe {
            SelectObject(HDC(self.h_mem_dc.0), self.h_old_bitmap);
            DeleteObject(HGDIOBJ(self.h_bitmap.0));
            DeleteDC(self.h_mem_dc);
            ReleaseDC(None, self.h_screen_dc);
        }
    }
}