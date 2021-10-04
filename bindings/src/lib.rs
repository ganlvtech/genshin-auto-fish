#[allow(
    unused_variables,
    non_upper_case_globals,
    non_snake_case,
    unused_unsafe,
    non_camel_case_types,
    dead_code,
    clippy::all
)]
pub mod Windows {
    #[allow(
        unused_variables,
        non_upper_case_globals,
        non_snake_case,
        unused_unsafe,
        non_camel_case_types,
        dead_code,
        clippy::all
    )]
    pub mod Win32 {
        #[allow(
            unused_variables,
            non_upper_case_globals,
            non_snake_case,
            unused_unsafe,
            non_camel_case_types,
            dead_code,
            clippy::all
        )]
        pub mod Foundation {
            #[repr(transparent)]
            #[derive(
                :: std :: default :: Default,
                :: std :: clone :: Clone,
                :: std :: marker :: Copy,
                :: std :: cmp :: PartialEq,
                :: std :: cmp :: Eq,
                :: std :: fmt :: Debug,
            )]
            pub struct BOOL(pub i32);
            unsafe impl ::windows::Abi for BOOL {
                type Abi = Self;
                type DefaultType = Self;
            }
            impl BOOL {
                #[inline]
                pub fn as_bool(self) -> bool {
                    !(self.0 == 0)
                }
                #[inline]
                pub fn ok(self) -> ::windows::Result<()> {
                    if self.as_bool() {
                        Ok(())
                    } else {
                        Err(::windows::Error::from_win32())
                    }
                }
                #[inline]
                #[track_caller]
                pub fn unwrap(self) {
                    self.ok().unwrap();
                }
                #[inline]
                #[track_caller]
                pub fn expect(self, msg: &str) {
                    self.ok().expect(msg);
                }
            }
            impl ::std::convert::From<BOOL> for bool {
                fn from(value: BOOL) -> Self {
                    value.as_bool()
                }
            }
            impl ::std::convert::From<&BOOL> for bool {
                fn from(value: &BOOL) -> Self {
                    value.as_bool()
                }
            }
            impl ::std::convert::From<bool> for BOOL {
                fn from(value: bool) -> Self {
                    if value {
                        BOOL(1)
                    } else {
                        BOOL(0)
                    }
                }
            }
            impl ::std::convert::From<&bool> for BOOL {
                fn from(value: &bool) -> Self {
                    (*value).into()
                }
            }
            impl ::std::cmp::PartialEq<bool> for BOOL {
                fn eq(&self, other: &bool) -> bool {
                    self.as_bool() == *other
                }
            }
            impl ::std::cmp::PartialEq<BOOL> for bool {
                fn eq(&self, other: &BOOL) -> bool {
                    *self == other.as_bool()
                }
            }
            impl std::ops::Not for BOOL {
                type Output = Self;
                fn not(self) -> Self::Output {
                    if self.as_bool() {
                        BOOL(0)
                    } else {
                        BOOL(1)
                    }
                }
            }
            impl<'a> ::windows::IntoParam<'a, BOOL> for bool {
                fn into_param(self) -> ::windows::Param<'a, BOOL> {
                    ::windows::Param::Owned(self.into())
                }
            }
            #[derive(
                :: std :: clone :: Clone,
                :: std :: marker :: Copy,
                :: std :: fmt :: Debug,
                :: std :: cmp :: PartialEq,
                :: std :: cmp :: Eq,
            )]
            #[repr(transparent)]
            pub struct HWND(pub isize);
            impl ::std::default::Default for HWND {
                fn default() -> Self {
                    unsafe { ::std::mem::zeroed() }
                }
            }
            unsafe impl ::windows::Handle for HWND {}
            unsafe impl ::windows::Abi for HWND {
                type Abi = Self;
                type DefaultType = Self;
            }
            #[derive(
                :: std :: clone :: Clone,
                :: std :: marker :: Copy,
                :: std :: fmt :: Debug,
                :: std :: cmp :: PartialEq,
                :: std :: cmp :: Eq,
            )]
            #[repr(transparent)]
            pub struct PWSTR(pub *mut u16);
            impl ::std::default::Default for PWSTR {
                fn default() -> Self {
                    Self(::std::ptr::null_mut())
                }
            }
            unsafe impl ::windows::Abi for PWSTR {
                type Abi = Self;
                type DefaultType = Self;
                unsafe fn drop_param(param: &mut ::windows::Param<'_, Self>) {
                    if let ::windows::Param::Boxed(value) = param {
                        if !value.0.is_null() {
                            unsafe {
                                ::std::boxed::Box::from_raw(value.0);
                            }
                        }
                    }
                }
            }
            impl<'a> ::windows::IntoParam<'a, PWSTR> for &str {
                fn into_param(self) -> ::windows::Param<'a, PWSTR> {
                    ::windows::Param::Boxed(PWSTR(::std::boxed::Box::<[u16]>::into_raw(
                        self.encode_utf16()
                            .chain(::std::iter::once(0))
                            .collect::<std::vec::Vec<u16>>()
                            .into_boxed_slice(),
                    ) as _))
                }
            }
            impl<'a> ::windows::IntoParam<'a, PWSTR> for String {
                fn into_param(self) -> ::windows::Param<'a, PWSTR> {
                    ::windows::Param::Boxed(PWSTR(::std::boxed::Box::<[u16]>::into_raw(
                        self.encode_utf16()
                            .chain(::std::iter::once(0))
                            .collect::<std::vec::Vec<u16>>()
                            .into_boxed_slice(),
                    ) as _))
                }
            }
            #[cfg(windows)]
            impl<'a> ::windows::IntoParam<'a, PWSTR> for &::std::ffi::OsStr {
                fn into_param(self) -> ::windows::Param<'a, PWSTR> {
                    use std::os::windows::ffi::OsStrExt;
                    ::windows::Param::Boxed(PWSTR(::std::boxed::Box::<[u16]>::into_raw(
                        self.encode_wide()
                            .chain(::std::iter::once(0))
                            .collect::<std::vec::Vec<u16>>()
                            .into_boxed_slice(),
                    ) as _))
                }
            }
            #[cfg(windows)]
            impl<'a> ::windows::IntoParam<'a, PWSTR> for ::std::ffi::OsString {
                fn into_param(self) -> ::windows::Param<'a, PWSTR> {
                    use std::os::windows::ffi::OsStrExt;
                    ::windows::Param::Boxed(PWSTR(::std::boxed::Box::<[u16]>::into_raw(
                        self.encode_wide()
                            .chain(::std::iter::once(0))
                            .collect::<std::vec::Vec<u16>>()
                            .into_boxed_slice(),
                    ) as _))
                }
            }
            #[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
            #[repr(C)]
            pub struct RECT {
                pub left: i32,
                pub top: i32,
                pub right: i32,
                pub bottom: i32,
            }
            impl RECT {}
            impl ::std::default::Default for RECT {
                fn default() -> Self {
                    unsafe { ::std::mem::zeroed() }
                }
            }
            impl ::std::fmt::Debug for RECT {
                fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                    fmt.debug_struct("RECT")
                        .field("left", &self.left)
                        .field("top", &self.top)
                        .field("right", &self.right)
                        .field("bottom", &self.bottom)
                        .finish()
                }
            }
            impl ::std::cmp::PartialEq for RECT {
                fn eq(&self, other: &Self) -> bool {
                    self.left == other.left
                        && self.top == other.top
                        && self.right == other.right
                        && self.bottom == other.bottom
                }
            }
            impl ::std::cmp::Eq for RECT {}
            unsafe impl ::windows::Abi for RECT {
                type Abi = Self;
                type DefaultType = Self;
            }
        }
        #[allow(
            unused_variables,
            non_upper_case_globals,
            non_snake_case,
            unused_unsafe,
            non_camel_case_types,
            dead_code,
            clippy::all
        )]
        pub mod Graphics {
            #[allow(
                unused_variables,
                non_upper_case_globals,
                non_snake_case,
                unused_unsafe,
                non_camel_case_types,
                dead_code,
                clippy::all
            )]
            pub mod Gdi {
                #[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
                #[repr(C)]
                pub struct BITMAPINFO {
                    pub bmiHeader: BITMAPINFOHEADER,
                    pub bmiColors: [RGBQUAD; 1],
                }
                impl BITMAPINFO {}
                impl ::std::default::Default for BITMAPINFO {
                    fn default() -> Self {
                        unsafe { ::std::mem::zeroed() }
                    }
                }
                impl ::std::fmt::Debug for BITMAPINFO {
                    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                        fmt.debug_struct("BITMAPINFO")
                            .field("bmiHeader", &self.bmiHeader)
                            .field("bmiColors", &self.bmiColors)
                            .finish()
                    }
                }
                impl ::std::cmp::PartialEq for BITMAPINFO {
                    fn eq(&self, other: &Self) -> bool {
                        self.bmiHeader == other.bmiHeader && self.bmiColors == other.bmiColors
                    }
                }
                impl ::std::cmp::Eq for BITMAPINFO {}
                unsafe impl ::windows::Abi for BITMAPINFO {
                    type Abi = Self;
                    type DefaultType = Self;
                }
                #[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
                #[repr(C)]
                pub struct BITMAPINFOHEADER {
                    pub biSize: u32,
                    pub biWidth: i32,
                    pub biHeight: i32,
                    pub biPlanes: u16,
                    pub biBitCount: u16,
                    pub biCompression: u32,
                    pub biSizeImage: u32,
                    pub biXPelsPerMeter: i32,
                    pub biYPelsPerMeter: i32,
                    pub biClrUsed: u32,
                    pub biClrImportant: u32,
                }
                impl BITMAPINFOHEADER {}
                impl ::std::default::Default for BITMAPINFOHEADER {
                    fn default() -> Self {
                        unsafe { ::std::mem::zeroed() }
                    }
                }
                impl ::std::fmt::Debug for BITMAPINFOHEADER {
                    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                        fmt.debug_struct("BITMAPINFOHEADER")
                            .field("biSize", &self.biSize)
                            .field("biWidth", &self.biWidth)
                            .field("biHeight", &self.biHeight)
                            .field("biPlanes", &self.biPlanes)
                            .field("biBitCount", &self.biBitCount)
                            .field("biCompression", &self.biCompression)
                            .field("biSizeImage", &self.biSizeImage)
                            .field("biXPelsPerMeter", &self.biXPelsPerMeter)
                            .field("biYPelsPerMeter", &self.biYPelsPerMeter)
                            .field("biClrUsed", &self.biClrUsed)
                            .field("biClrImportant", &self.biClrImportant)
                            .finish()
                    }
                }
                impl ::std::cmp::PartialEq for BITMAPINFOHEADER {
                    fn eq(&self, other: &Self) -> bool {
                        self.biSize == other.biSize
                            && self.biWidth == other.biWidth
                            && self.biHeight == other.biHeight
                            && self.biPlanes == other.biPlanes
                            && self.biBitCount == other.biBitCount
                            && self.biCompression == other.biCompression
                            && self.biSizeImage == other.biSizeImage
                            && self.biXPelsPerMeter == other.biXPelsPerMeter
                            && self.biYPelsPerMeter == other.biYPelsPerMeter
                            && self.biClrUsed == other.biClrUsed
                            && self.biClrImportant == other.biClrImportant
                    }
                }
                impl ::std::cmp::Eq for BITMAPINFOHEADER {}
                unsafe impl ::windows::Abi for BITMAPINFOHEADER {
                    type Abi = Self;
                    type DefaultType = Self;
                }
                pub const BI_RGB: i32 = 0i32;
                #[derive(
                    :: std :: cmp :: PartialEq,
                    :: std :: cmp :: Eq,
                    :: std :: marker :: Copy,
                    :: std :: clone :: Clone,
                    :: std :: default :: Default,
                    :: std :: fmt :: Debug,
                )]
                #[repr(transparent)]
                pub struct DIB_USAGE(pub u32);
                pub const DIB_RGB_COLORS: DIB_USAGE = DIB_USAGE(0u32);
                pub const DIB_PAL_COLORS: DIB_USAGE = DIB_USAGE(1u32);
                impl ::std::convert::From<u32> for DIB_USAGE {
                    fn from(value: u32) -> Self {
                        Self(value)
                    }
                }
                unsafe impl ::windows::Abi for DIB_USAGE {
                    type Abi = Self;
                    type DefaultType = Self;
                }
                impl ::std::ops::BitOr for DIB_USAGE {
                    type Output = Self;
                    fn bitor(self, rhs: Self) -> Self {
                        Self(self.0 | rhs.0)
                    }
                }
                impl ::std::ops::BitAnd for DIB_USAGE {
                    type Output = Self;
                    fn bitand(self, rhs: Self) -> Self {
                        Self(self.0 & rhs.0)
                    }
                }
                impl ::std::ops::BitOrAssign for DIB_USAGE {
                    fn bitor_assign(&mut self, rhs: Self) {
                        self.0.bitor_assign(rhs.0)
                    }
                }
                impl ::std::ops::BitAndAssign for DIB_USAGE {
                    fn bitand_assign(&mut self, rhs: Self) {
                        self.0.bitand_assign(rhs.0)
                    }
                }
                impl ::std::ops::Not for DIB_USAGE {
                    type Output = Self;
                    fn not(self) -> Self {
                        Self(self.0.not())
                    }
                }
                pub unsafe fn GetCurrentObject<'a>(
                    hdc: impl ::windows::IntoParam<'a, HDC>,
                    r#type: OBJ_TYPE,
                ) -> HGDIOBJ {
                    #[cfg(windows)]
                    {
                        #[link(name = "gdi32")]
                        extern "system" {
                            fn GetCurrentObject(hdc: HDC, r#type: OBJ_TYPE) -> HGDIOBJ;
                        }
                        ::std::mem::transmute(GetCurrentObject(
                            hdc.into_param().abi(),
                            ::std::mem::transmute(r#type),
                        ))
                    }
                    #[cfg(not(windows))]
                    unimplemented!("Unsupported target OS");
                }
                pub unsafe fn GetDC<'a>(
                    hwnd: impl ::windows::IntoParam<'a, super::super::Foundation::HWND>,
                ) -> HDC {
                    #[cfg(windows)]
                    {
                        #[link(name = "user32")]
                        extern "system" {
                            fn GetDC(hwnd: super::super::Foundation::HWND) -> HDC;
                        }
                        ::std::mem::transmute(GetDC(hwnd.into_param().abi()))
                    }
                    #[cfg(not(windows))]
                    unimplemented!("Unsupported target OS");
                }
                pub unsafe fn GetDIBits<'a>(
                    hdc: impl ::windows::IntoParam<'a, HDC>,
                    hbm: impl ::windows::IntoParam<'a, HBITMAP>,
                    start: u32,
                    clines: u32,
                    lpvbits: *mut ::std::ffi::c_void,
                    lpbmi: *mut BITMAPINFO,
                    usage: DIB_USAGE,
                ) -> i32 {
                    #[cfg(windows)]
                    {
                        #[link(name = "gdi32")]
                        extern "system" {
                            fn GetDIBits(
                                hdc: HDC,
                                hbm: HBITMAP,
                                start: u32,
                                clines: u32,
                                lpvbits: *mut ::std::ffi::c_void,
                                lpbmi: *mut BITMAPINFO,
                                usage: DIB_USAGE,
                            ) -> i32;
                        }
                        ::std::mem::transmute(GetDIBits(
                            hdc.into_param().abi(),
                            hbm.into_param().abi(),
                            ::std::mem::transmute(start),
                            ::std::mem::transmute(clines),
                            ::std::mem::transmute(lpvbits),
                            ::std::mem::transmute(lpbmi),
                            ::std::mem::transmute(usage),
                        ))
                    }
                    #[cfg(not(windows))]
                    unimplemented!("Unsupported target OS");
                }
                #[derive(
                    :: std :: clone :: Clone,
                    :: std :: marker :: Copy,
                    :: std :: fmt :: Debug,
                    :: std :: cmp :: PartialEq,
                    :: std :: cmp :: Eq,
                )]
                #[repr(transparent)]
                pub struct HBITMAP(pub isize);
                impl ::std::default::Default for HBITMAP {
                    fn default() -> Self {
                        unsafe { ::std::mem::zeroed() }
                    }
                }
                unsafe impl ::windows::Handle for HBITMAP {}
                unsafe impl ::windows::Abi for HBITMAP {
                    type Abi = Self;
                    type DefaultType = Self;
                }
                #[derive(
                    :: std :: clone :: Clone,
                    :: std :: marker :: Copy,
                    :: std :: fmt :: Debug,
                    :: std :: cmp :: PartialEq,
                    :: std :: cmp :: Eq,
                )]
                #[repr(transparent)]
                pub struct HDC(pub isize);
                impl ::std::default::Default for HDC {
                    fn default() -> Self {
                        unsafe { ::std::mem::zeroed() }
                    }
                }
                unsafe impl ::windows::Handle for HDC {}
                unsafe impl ::windows::Abi for HDC {
                    type Abi = Self;
                    type DefaultType = Self;
                }
                #[derive(
                    :: std :: clone :: Clone,
                    :: std :: marker :: Copy,
                    :: std :: fmt :: Debug,
                    :: std :: cmp :: PartialEq,
                    :: std :: cmp :: Eq,
                )]
                #[repr(transparent)]
                pub struct HGDIOBJ(pub isize);
                impl ::std::default::Default for HGDIOBJ {
                    fn default() -> Self {
                        unsafe { ::std::mem::zeroed() }
                    }
                }
                unsafe impl ::windows::Handle for HGDIOBJ {}
                unsafe impl ::windows::Abi for HGDIOBJ {
                    type Abi = Self;
                    type DefaultType = Self;
                }
                #[derive(
                    :: std :: cmp :: PartialEq,
                    :: std :: cmp :: Eq,
                    :: std :: marker :: Copy,
                    :: std :: clone :: Clone,
                    :: std :: default :: Default,
                    :: std :: fmt :: Debug,
                )]
                #[repr(transparent)]
                pub struct OBJ_TYPE(pub i32);
                pub const OBJ_PEN: OBJ_TYPE = OBJ_TYPE(1i32);
                pub const OBJ_BRUSH: OBJ_TYPE = OBJ_TYPE(2i32);
                pub const OBJ_DC: OBJ_TYPE = OBJ_TYPE(3i32);
                pub const OBJ_METADC: OBJ_TYPE = OBJ_TYPE(4i32);
                pub const OBJ_PAL: OBJ_TYPE = OBJ_TYPE(5i32);
                pub const OBJ_FONT: OBJ_TYPE = OBJ_TYPE(6i32);
                pub const OBJ_BITMAP: OBJ_TYPE = OBJ_TYPE(7i32);
                pub const OBJ_REGION: OBJ_TYPE = OBJ_TYPE(8i32);
                pub const OBJ_METAFILE: OBJ_TYPE = OBJ_TYPE(9i32);
                pub const OBJ_MEMDC: OBJ_TYPE = OBJ_TYPE(10i32);
                pub const OBJ_EXTPEN: OBJ_TYPE = OBJ_TYPE(11i32);
                pub const OBJ_ENHMETADC: OBJ_TYPE = OBJ_TYPE(12i32);
                pub const OBJ_ENHMETAFILE: OBJ_TYPE = OBJ_TYPE(13i32);
                pub const OBJ_COLORSPACE: OBJ_TYPE = OBJ_TYPE(14i32);
                impl ::std::convert::From<i32> for OBJ_TYPE {
                    fn from(value: i32) -> Self {
                        Self(value)
                    }
                }
                unsafe impl ::windows::Abi for OBJ_TYPE {
                    type Abi = Self;
                    type DefaultType = Self;
                }
                #[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
                #[repr(C)]
                pub struct RGBQUAD {
                    pub rgbBlue: u8,
                    pub rgbGreen: u8,
                    pub rgbRed: u8,
                    pub rgbReserved: u8,
                }
                impl RGBQUAD {}
                impl ::std::default::Default for RGBQUAD {
                    fn default() -> Self {
                        unsafe { ::std::mem::zeroed() }
                    }
                }
                impl ::std::fmt::Debug for RGBQUAD {
                    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                        fmt.debug_struct("RGBQUAD")
                            .field("rgbBlue", &self.rgbBlue)
                            .field("rgbGreen", &self.rgbGreen)
                            .field("rgbRed", &self.rgbRed)
                            .field("rgbReserved", &self.rgbReserved)
                            .finish()
                    }
                }
                impl ::std::cmp::PartialEq for RGBQUAD {
                    fn eq(&self, other: &Self) -> bool {
                        self.rgbBlue == other.rgbBlue
                            && self.rgbGreen == other.rgbGreen
                            && self.rgbRed == other.rgbRed
                            && self.rgbReserved == other.rgbReserved
                    }
                }
                impl ::std::cmp::Eq for RGBQUAD {}
                unsafe impl ::windows::Abi for RGBQUAD {
                    type Abi = Self;
                    type DefaultType = Self;
                }
                pub unsafe fn ReleaseDC<'a>(
                    hwnd: impl ::windows::IntoParam<'a, super::super::Foundation::HWND>,
                    hdc: impl ::windows::IntoParam<'a, HDC>,
                ) -> i32 {
                    #[cfg(windows)]
                    {
                        #[link(name = "user32")]
                        extern "system" {
                            fn ReleaseDC(hwnd: super::super::Foundation::HWND, hdc: HDC) -> i32;
                        }
                        ::std::mem::transmute(ReleaseDC(
                            hwnd.into_param().abi(),
                            hdc.into_param().abi(),
                        ))
                    }
                    #[cfg(not(windows))]
                    unimplemented!("Unsupported target OS");
                }
            }
        }
        #[allow(
            unused_variables,
            non_upper_case_globals,
            non_snake_case,
            unused_unsafe,
            non_camel_case_types,
            dead_code,
            clippy::all
        )]
        pub mod System {
            #[allow(
                unused_variables,
                non_upper_case_globals,
                non_snake_case,
                unused_unsafe,
                non_camel_case_types,
                dead_code,
                clippy::all
            )]
            pub mod Diagnostics {
                #[allow(
                    unused_variables,
                    non_upper_case_globals,
                    non_snake_case,
                    unused_unsafe,
                    non_camel_case_types,
                    dead_code,
                    clippy::all
                )]
                pub mod Debug {
                    pub unsafe fn Beep(
                        dwfreq: u32,
                        dwduration: u32,
                    ) -> super::super::super::Foundation::BOOL {
                        #[cfg(windows)]
                        {
                            #[link(name = "kernel32")]
                            extern "system" {
                                fn Beep(
                                    dwfreq: u32,
                                    dwduration: u32,
                                ) -> super::super::super::Foundation::BOOL;
                            }
                            ::std::mem::transmute(Beep(
                                ::std::mem::transmute(dwfreq),
                                ::std::mem::transmute(dwduration),
                            ))
                        }
                        #[cfg(not(windows))]
                        unimplemented!("Unsupported target OS");
                    }
                    pub unsafe fn GetLastError() -> WIN32_ERROR {
                        #[cfg(windows)]
                        {
                            #[link(name = "kernel32")]
                            extern "system" {
                                fn GetLastError() -> WIN32_ERROR;
                            }
                            ::std::mem::transmute(GetLastError())
                        }
                        #[cfg(not(windows))]
                        unimplemented!("Unsupported target OS");
                    }
                    #[derive(
                        :: std :: cmp :: PartialEq,
                        :: std :: cmp :: Eq,
                        :: std :: marker :: Copy,
                        :: std :: clone :: Clone,
                        :: std :: default :: Default,
                        :: std :: fmt :: Debug,
                    )]
                    #[repr(transparent)]
                    pub struct WIN32_ERROR(pub u32);
                    impl ::std::convert::From<u32> for WIN32_ERROR {
                        fn from(value: u32) -> Self {
                            Self(value)
                        }
                    }
                    unsafe impl ::windows::Abi for WIN32_ERROR {
                        type Abi = Self;
                        type DefaultType = Self;
                    }
                    impl ::std::ops::BitOr for WIN32_ERROR {
                        type Output = Self;
                        fn bitor(self, rhs: Self) -> Self {
                            Self(self.0 | rhs.0)
                        }
                    }
                    impl ::std::ops::BitAnd for WIN32_ERROR {
                        type Output = Self;
                        fn bitand(self, rhs: Self) -> Self {
                            Self(self.0 & rhs.0)
                        }
                    }
                    impl ::std::ops::BitOrAssign for WIN32_ERROR {
                        fn bitor_assign(&mut self, rhs: Self) {
                            self.0.bitor_assign(rhs.0)
                        }
                    }
                    impl ::std::ops::BitAndAssign for WIN32_ERROR {
                        fn bitand_assign(&mut self, rhs: Self) {
                            self.0.bitand_assign(rhs.0)
                        }
                    }
                    impl ::std::ops::Not for WIN32_ERROR {
                        type Output = Self;
                        fn not(self) -> Self {
                            Self(self.0.not())
                        }
                    }
                    impl ::std::convert::From<WIN32_ERROR> for ::windows::HRESULT {
                        fn from(value: WIN32_ERROR) -> Self {
                            Self(if value.0 as i32 <= 0 {
                                value.0
                            } else {
                                (value.0 & 0x0000_FFFF) | (7 << 16) | 0x8000_0000
                            })
                        }
                    }
                }
            }
        }
        #[allow(
            unused_variables,
            non_upper_case_globals,
            non_snake_case,
            unused_unsafe,
            non_camel_case_types,
            dead_code,
            clippy::all
        )]
        pub mod UI {
            #[allow(
                unused_variables,
                non_upper_case_globals,
                non_snake_case,
                unused_unsafe,
                non_camel_case_types,
                dead_code,
                clippy::all
            )]
            pub mod HiDpi {
                #[derive(
                    :: std :: cmp :: PartialEq,
                    :: std :: cmp :: Eq,
                    :: std :: marker :: Copy,
                    :: std :: clone :: Clone,
                    :: std :: default :: Default,
                    :: std :: fmt :: Debug,
                )]
                #[repr(transparent)]
                pub struct PROCESS_DPI_AWARENESS(pub i32);
                pub const PROCESS_DPI_UNAWARE: PROCESS_DPI_AWARENESS = PROCESS_DPI_AWARENESS(0i32);
                pub const PROCESS_SYSTEM_DPI_AWARE: PROCESS_DPI_AWARENESS =
                    PROCESS_DPI_AWARENESS(1i32);
                pub const PROCESS_PER_MONITOR_DPI_AWARE: PROCESS_DPI_AWARENESS =
                    PROCESS_DPI_AWARENESS(2i32);
                impl ::std::convert::From<i32> for PROCESS_DPI_AWARENESS {
                    fn from(value: i32) -> Self {
                        Self(value)
                    }
                }
                unsafe impl ::windows::Abi for PROCESS_DPI_AWARENESS {
                    type Abi = Self;
                    type DefaultType = Self;
                }
                pub unsafe fn SetProcessDpiAwareness(
                    value: PROCESS_DPI_AWARENESS,
                ) -> ::windows::Result<()> {
                    #[cfg(windows)]
                    {
                        #[link(name = "onecoreuap")]
                        extern "system" {
                            fn SetProcessDpiAwareness(
                                value: PROCESS_DPI_AWARENESS,
                            ) -> ::windows::HRESULT;
                        }
                        SetProcessDpiAwareness(::std::mem::transmute(value)).ok()
                    }
                    #[cfg(not(windows))]
                    unimplemented!("Unsupported target OS");
                }
            }
            #[allow(
                unused_variables,
                non_upper_case_globals,
                non_snake_case,
                unused_unsafe,
                non_camel_case_types,
                dead_code,
                clippy::all
            )]
            pub mod KeyboardAndMouseInput {
                #[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
                #[repr(C)]
                pub struct HARDWAREINPUT {
                    pub uMsg: u32,
                    pub wParamL: u16,
                    pub wParamH: u16,
                }
                impl HARDWAREINPUT {}
                impl ::std::default::Default for HARDWAREINPUT {
                    fn default() -> Self {
                        unsafe { ::std::mem::zeroed() }
                    }
                }
                impl ::std::fmt::Debug for HARDWAREINPUT {
                    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                        fmt.debug_struct("HARDWAREINPUT")
                            .field("uMsg", &self.uMsg)
                            .field("wParamL", &self.wParamL)
                            .field("wParamH", &self.wParamH)
                            .finish()
                    }
                }
                impl ::std::cmp::PartialEq for HARDWAREINPUT {
                    fn eq(&self, other: &Self) -> bool {
                        self.uMsg == other.uMsg
                            && self.wParamL == other.wParamL
                            && self.wParamH == other.wParamH
                    }
                }
                impl ::std::cmp::Eq for HARDWAREINPUT {}
                unsafe impl ::windows::Abi for HARDWAREINPUT {
                    type Abi = Self;
                    type DefaultType = Self;
                }
                #[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
                #[repr(C)]
                pub struct INPUT {
                    pub r#type: INPUT_TYPE,
                    pub Anonymous: INPUT_0,
                }
                impl INPUT {}
                impl ::std::default::Default for INPUT {
                    fn default() -> Self {
                        unsafe { ::std::mem::zeroed() }
                    }
                }
                impl ::std::cmp::PartialEq for INPUT {
                    fn eq(&self, _other: &Self) -> bool {
                        unimplemented!()
                    }
                }
                impl ::std::cmp::Eq for INPUT {}
                unsafe impl ::windows::Abi for INPUT {
                    type Abi = Self;
                    type DefaultType = Self;
                }
                #[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
                #[repr(C)]
                pub union INPUT_0 {
                    pub mi: MOUSEINPUT,
                    pub ki: KEYBDINPUT,
                    pub hi: HARDWAREINPUT,
                }
                impl INPUT_0 {}
                impl ::std::default::Default for INPUT_0 {
                    fn default() -> Self {
                        unsafe { ::std::mem::zeroed() }
                    }
                }
                impl ::std::cmp::PartialEq for INPUT_0 {
                    fn eq(&self, _other: &Self) -> bool {
                        unimplemented!()
                    }
                }
                impl ::std::cmp::Eq for INPUT_0 {}
                unsafe impl ::windows::Abi for INPUT_0 {
                    type Abi = Self;
                    type DefaultType = Self;
                }
                #[derive(
                    :: std :: cmp :: PartialEq,
                    :: std :: cmp :: Eq,
                    :: std :: marker :: Copy,
                    :: std :: clone :: Clone,
                    :: std :: default :: Default,
                    :: std :: fmt :: Debug,
                )]
                #[repr(transparent)]
                pub struct INPUT_TYPE(pub u32);
                pub const INPUT_MOUSE: INPUT_TYPE = INPUT_TYPE(0u32);
                pub const INPUT_KEYBOARD: INPUT_TYPE = INPUT_TYPE(1u32);
                pub const INPUT_HARDWARE: INPUT_TYPE = INPUT_TYPE(2u32);
                impl ::std::convert::From<u32> for INPUT_TYPE {
                    fn from(value: u32) -> Self {
                        Self(value)
                    }
                }
                unsafe impl ::windows::Abi for INPUT_TYPE {
                    type Abi = Self;
                    type DefaultType = Self;
                }
                impl ::std::ops::BitOr for INPUT_TYPE {
                    type Output = Self;
                    fn bitor(self, rhs: Self) -> Self {
                        Self(self.0 | rhs.0)
                    }
                }
                impl ::std::ops::BitAnd for INPUT_TYPE {
                    type Output = Self;
                    fn bitand(self, rhs: Self) -> Self {
                        Self(self.0 & rhs.0)
                    }
                }
                impl ::std::ops::BitOrAssign for INPUT_TYPE {
                    fn bitor_assign(&mut self, rhs: Self) {
                        self.0.bitor_assign(rhs.0)
                    }
                }
                impl ::std::ops::BitAndAssign for INPUT_TYPE {
                    fn bitand_assign(&mut self, rhs: Self) {
                        self.0.bitand_assign(rhs.0)
                    }
                }
                impl ::std::ops::Not for INPUT_TYPE {
                    type Output = Self;
                    fn not(self) -> Self {
                        Self(self.0.not())
                    }
                }
                #[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
                #[repr(C)]
                pub struct KEYBDINPUT {
                    pub wVk: VIRTUAL_KEY,
                    pub wScan: u16,
                    pub dwFlags: KEYBD_EVENT_FLAGS,
                    pub time: u32,
                    pub dwExtraInfo: usize,
                }
                impl KEYBDINPUT {}
                impl ::std::default::Default for KEYBDINPUT {
                    fn default() -> Self {
                        unsafe { ::std::mem::zeroed() }
                    }
                }
                impl ::std::fmt::Debug for KEYBDINPUT {
                    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                        fmt.debug_struct("KEYBDINPUT")
                            .field("wVk", &self.wVk)
                            .field("wScan", &self.wScan)
                            .field("dwFlags", &self.dwFlags)
                            .field("time", &self.time)
                            .field("dwExtraInfo", &self.dwExtraInfo)
                            .finish()
                    }
                }
                impl ::std::cmp::PartialEq for KEYBDINPUT {
                    fn eq(&self, other: &Self) -> bool {
                        self.wVk == other.wVk
                            && self.wScan == other.wScan
                            && self.dwFlags == other.dwFlags
                            && self.time == other.time
                            && self.dwExtraInfo == other.dwExtraInfo
                    }
                }
                impl ::std::cmp::Eq for KEYBDINPUT {}
                unsafe impl ::windows::Abi for KEYBDINPUT {
                    type Abi = Self;
                    type DefaultType = Self;
                }
                #[derive(
                    :: std :: cmp :: PartialEq,
                    :: std :: cmp :: Eq,
                    :: std :: marker :: Copy,
                    :: std :: clone :: Clone,
                    :: std :: default :: Default,
                    :: std :: fmt :: Debug,
                )]
                #[repr(transparent)]
                pub struct KEYBD_EVENT_FLAGS(pub u32);
                pub const KEYEVENTF_EXTENDEDKEY: KEYBD_EVENT_FLAGS = KEYBD_EVENT_FLAGS(1u32);
                pub const KEYEVENTF_KEYUP: KEYBD_EVENT_FLAGS = KEYBD_EVENT_FLAGS(2u32);
                pub const KEYEVENTF_SCANCODE: KEYBD_EVENT_FLAGS = KEYBD_EVENT_FLAGS(8u32);
                pub const KEYEVENTF_UNICODE: KEYBD_EVENT_FLAGS = KEYBD_EVENT_FLAGS(4u32);
                impl ::std::convert::From<u32> for KEYBD_EVENT_FLAGS {
                    fn from(value: u32) -> Self {
                        Self(value)
                    }
                }
                unsafe impl ::windows::Abi for KEYBD_EVENT_FLAGS {
                    type Abi = Self;
                    type DefaultType = Self;
                }
                impl ::std::ops::BitOr for KEYBD_EVENT_FLAGS {
                    type Output = Self;
                    fn bitor(self, rhs: Self) -> Self {
                        Self(self.0 | rhs.0)
                    }
                }
                impl ::std::ops::BitAnd for KEYBD_EVENT_FLAGS {
                    type Output = Self;
                    fn bitand(self, rhs: Self) -> Self {
                        Self(self.0 & rhs.0)
                    }
                }
                impl ::std::ops::BitOrAssign for KEYBD_EVENT_FLAGS {
                    fn bitor_assign(&mut self, rhs: Self) {
                        self.0.bitor_assign(rhs.0)
                    }
                }
                impl ::std::ops::BitAndAssign for KEYBD_EVENT_FLAGS {
                    fn bitand_assign(&mut self, rhs: Self) {
                        self.0.bitand_assign(rhs.0)
                    }
                }
                impl ::std::ops::Not for KEYBD_EVENT_FLAGS {
                    type Output = Self;
                    fn not(self) -> Self {
                        Self(self.0.not())
                    }
                }
                #[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
                #[repr(C)]
                pub struct MOUSEINPUT {
                    pub dx: i32,
                    pub dy: i32,
                    pub mouseData: u32,
                    pub dwFlags: MOUSE_EVENT_FLAGS,
                    pub time: u32,
                    pub dwExtraInfo: usize,
                }
                impl MOUSEINPUT {}
                impl ::std::default::Default for MOUSEINPUT {
                    fn default() -> Self {
                        unsafe { ::std::mem::zeroed() }
                    }
                }
                impl ::std::fmt::Debug for MOUSEINPUT {
                    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                        fmt.debug_struct("MOUSEINPUT")
                            .field("dx", &self.dx)
                            .field("dy", &self.dy)
                            .field("mouseData", &self.mouseData)
                            .field("dwFlags", &self.dwFlags)
                            .field("time", &self.time)
                            .field("dwExtraInfo", &self.dwExtraInfo)
                            .finish()
                    }
                }
                impl ::std::cmp::PartialEq for MOUSEINPUT {
                    fn eq(&self, other: &Self) -> bool {
                        self.dx == other.dx
                            && self.dy == other.dy
                            && self.mouseData == other.mouseData
                            && self.dwFlags == other.dwFlags
                            && self.time == other.time
                            && self.dwExtraInfo == other.dwExtraInfo
                    }
                }
                impl ::std::cmp::Eq for MOUSEINPUT {}
                unsafe impl ::windows::Abi for MOUSEINPUT {
                    type Abi = Self;
                    type DefaultType = Self;
                }
                #[derive(
                    :: std :: cmp :: PartialEq,
                    :: std :: cmp :: Eq,
                    :: std :: marker :: Copy,
                    :: std :: clone :: Clone,
                    :: std :: default :: Default,
                    :: std :: fmt :: Debug,
                )]
                #[repr(transparent)]
                pub struct MOUSE_EVENT_FLAGS(pub u32);
                pub const MOUSEEVENTF_ABSOLUTE: MOUSE_EVENT_FLAGS = MOUSE_EVENT_FLAGS(32768u32);
                pub const MOUSEEVENTF_LEFTDOWN: MOUSE_EVENT_FLAGS = MOUSE_EVENT_FLAGS(2u32);
                pub const MOUSEEVENTF_LEFTUP: MOUSE_EVENT_FLAGS = MOUSE_EVENT_FLAGS(4u32);
                pub const MOUSEEVENTF_MIDDLEDOWN: MOUSE_EVENT_FLAGS = MOUSE_EVENT_FLAGS(32u32);
                pub const MOUSEEVENTF_MIDDLEUP: MOUSE_EVENT_FLAGS = MOUSE_EVENT_FLAGS(64u32);
                pub const MOUSEEVENTF_MOVE: MOUSE_EVENT_FLAGS = MOUSE_EVENT_FLAGS(1u32);
                pub const MOUSEEVENTF_RIGHTDOWN: MOUSE_EVENT_FLAGS = MOUSE_EVENT_FLAGS(8u32);
                pub const MOUSEEVENTF_RIGHTUP: MOUSE_EVENT_FLAGS = MOUSE_EVENT_FLAGS(16u32);
                pub const MOUSEEVENTF_WHEEL: MOUSE_EVENT_FLAGS = MOUSE_EVENT_FLAGS(2048u32);
                pub const MOUSEEVENTF_XDOWN: MOUSE_EVENT_FLAGS = MOUSE_EVENT_FLAGS(128u32);
                pub const MOUSEEVENTF_XUP: MOUSE_EVENT_FLAGS = MOUSE_EVENT_FLAGS(256u32);
                pub const MOUSEEVENTF_HWHEEL: MOUSE_EVENT_FLAGS = MOUSE_EVENT_FLAGS(4096u32);
                pub const MOUSEEVENTF_MOVE_NOCOALESCE: MOUSE_EVENT_FLAGS =
                    MOUSE_EVENT_FLAGS(8192u32);
                pub const MOUSEEVENTF_VIRTUALDESK: MOUSE_EVENT_FLAGS = MOUSE_EVENT_FLAGS(16384u32);
                impl ::std::convert::From<u32> for MOUSE_EVENT_FLAGS {
                    fn from(value: u32) -> Self {
                        Self(value)
                    }
                }
                unsafe impl ::windows::Abi for MOUSE_EVENT_FLAGS {
                    type Abi = Self;
                    type DefaultType = Self;
                }
                impl ::std::ops::BitOr for MOUSE_EVENT_FLAGS {
                    type Output = Self;
                    fn bitor(self, rhs: Self) -> Self {
                        Self(self.0 | rhs.0)
                    }
                }
                impl ::std::ops::BitAnd for MOUSE_EVENT_FLAGS {
                    type Output = Self;
                    fn bitand(self, rhs: Self) -> Self {
                        Self(self.0 & rhs.0)
                    }
                }
                impl ::std::ops::BitOrAssign for MOUSE_EVENT_FLAGS {
                    fn bitor_assign(&mut self, rhs: Self) {
                        self.0.bitor_assign(rhs.0)
                    }
                }
                impl ::std::ops::BitAndAssign for MOUSE_EVENT_FLAGS {
                    fn bitand_assign(&mut self, rhs: Self) {
                        self.0.bitand_assign(rhs.0)
                    }
                }
                impl ::std::ops::Not for MOUSE_EVENT_FLAGS {
                    type Output = Self;
                    fn not(self) -> Self {
                        Self(self.0.not())
                    }
                }
                pub unsafe fn SendInput(cinputs: u32, pinputs: *const INPUT, cbsize: i32) -> u32 {
                    #[cfg(windows)]
                    {
                        #[link(name = "user32")]
                        extern "system" {
                            fn SendInput(cinputs: u32, pinputs: *const INPUT, cbsize: i32) -> u32;
                        }
                        ::std::mem::transmute(SendInput(
                            ::std::mem::transmute(cinputs),
                            ::std::mem::transmute(pinputs),
                            ::std::mem::transmute(cbsize),
                        ))
                    }
                    #[cfg(not(windows))]
                    unimplemented!("Unsupported target OS");
                }
                #[derive(
                    :: std :: cmp :: PartialEq,
                    :: std :: cmp :: Eq,
                    :: std :: marker :: Copy,
                    :: std :: clone :: Clone,
                    :: std :: default :: Default,
                    :: std :: fmt :: Debug,
                )]
                #[repr(transparent)]
                pub struct VIRTUAL_KEY(pub u16);
                impl ::std::convert::From<u16> for VIRTUAL_KEY {
                    fn from(value: u16) -> Self {
                        Self(value)
                    }
                }
                unsafe impl ::windows::Abi for VIRTUAL_KEY {
                    type Abi = Self;
                    type DefaultType = Self;
                }
            }
            #[allow(
                unused_variables,
                non_upper_case_globals,
                non_snake_case,
                unused_unsafe,
                non_camel_case_types,
                dead_code,
                clippy::all
            )]
            pub mod WindowsAndMessaging {
                pub unsafe fn FindWindowW<'a>(
                    lpclassname: impl ::windows::IntoParam<'a, super::super::Foundation::PWSTR>,
                    lpwindowname: impl ::windows::IntoParam<'a, super::super::Foundation::PWSTR>,
                ) -> super::super::Foundation::HWND {
                    #[cfg(windows)]
                    {
                        #[link(name = "user32")]
                        extern "system" {
                            fn FindWindowW(
                                lpclassname: super::super::Foundation::PWSTR,
                                lpwindowname: super::super::Foundation::PWSTR,
                            ) -> super::super::Foundation::HWND;
                        }
                        ::std::mem::transmute(FindWindowW(
                            lpclassname.into_param().abi(),
                            lpwindowname.into_param().abi(),
                        ))
                    }
                    #[cfg(not(windows))]
                    unimplemented!("Unsupported target OS");
                }
                pub unsafe fn GetClientRect<'a>(
                    hwnd: impl ::windows::IntoParam<'a, super::super::Foundation::HWND>,
                    lprect: *mut super::super::Foundation::RECT,
                ) -> super::super::Foundation::BOOL {
                    #[cfg(windows)]
                    {
                        #[link(name = "user32")]
                        extern "system" {
                            fn GetClientRect(
                                hwnd: super::super::Foundation::HWND,
                                lprect: *mut super::super::Foundation::RECT,
                            ) -> super::super::Foundation::BOOL;
                        }
                        ::std::mem::transmute(GetClientRect(
                            hwnd.into_param().abi(),
                            ::std::mem::transmute(lprect),
                        ))
                    }
                    #[cfg(not(windows))]
                    unimplemented!("Unsupported target OS");
                }
            }
        }
    }
}
