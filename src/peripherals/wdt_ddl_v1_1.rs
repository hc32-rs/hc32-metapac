#![allow(clippy::missing_safety_doc)]
#![allow(clippy::identity_op)]
#![allow(clippy::unnecessary_cast)]
#![allow(clippy::erasing_op)]

#[doc = "desc WDT."]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Wdt {
    ptr: *mut u8,
}
unsafe impl Send for Wdt {}
unsafe impl Sync for Wdt {}
impl Wdt {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "desc RST."]
    #[inline(always)]
    pub const fn rst(self) -> crate::common::Reg<regs::Rst, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x80usize) as _) }
    }
    #[doc = "desc CON."]
    #[inline(always)]
    pub const fn con(self) -> crate::common::Reg<regs::Con, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x84usize) as _) }
    }
}
pub mod regs {
    #[doc = "desc CON."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Con(pub u32);
    impl Con {
        #[doc = "desc WOV."]
        #[must_use]
        #[inline(always)]
        pub const fn wov(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x0f;
            val as u8
        }
        #[doc = "desc WOV."]
        #[inline(always)]
        pub const fn set_wov(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
        }
        #[doc = "desc R."]
        #[must_use]
        #[inline(always)]
        pub const fn r(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "desc R."]
        #[inline(always)]
        pub const fn set_r(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "desc WINT_EN."]
        #[must_use]
        #[inline(always)]
        pub const fn wint_en(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "desc WINT_EN."]
        #[inline(always)]
        pub const fn set_wint_en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "desc INT."]
        #[must_use]
        #[inline(always)]
        pub const fn int(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "desc INT."]
        #[inline(always)]
        pub const fn set_int(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "desc WCNTL."]
        #[must_use]
        #[inline(always)]
        pub const fn wcntl(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0xff;
            val as u8
        }
        #[doc = "desc WCNTL."]
        #[inline(always)]
        pub const fn set_wcntl(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
        }
        #[doc = "desc RSV."]
        #[must_use]
        #[inline(always)]
        pub const fn rsv(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[doc = "desc RSV."]
        #[inline(always)]
        pub const fn set_rsv(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for Con {
        #[inline(always)]
        fn default() -> Con {
            Con(0)
        }
    }
    impl core::fmt::Debug for Con {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Con")
                .field("wov", &self.wov())
                .field("r", &self.r())
                .field("wint_en", &self.wint_en())
                .field("int", &self.int())
                .field("wcntl", &self.wcntl())
                .field("rsv", &self.rsv())
                .finish()
        }
    }
    #[doc = "desc RST."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Rst(pub u32);
    impl Rst {
        #[doc = "desc RST."]
        #[must_use]
        #[inline(always)]
        pub const fn rst(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[doc = "desc RST."]
        #[inline(always)]
        pub const fn set_rst(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
        }
        #[doc = "desc RSV."]
        #[must_use]
        #[inline(always)]
        pub const fn rsv(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[doc = "desc RSV."]
        #[inline(always)]
        pub const fn set_rsv(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for Rst {
        #[inline(always)]
        fn default() -> Rst {
            Rst(0)
        }
    }
    impl core::fmt::Debug for Rst {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Rst")
                .field("rst", &self.rst())
                .field("rsv", &self.rsv())
                .finish()
        }
    }
}
