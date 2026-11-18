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
        #[doc = "desc WDTR."]
        #[must_use]
        #[inline(always)]
        pub const fn wdtr(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "desc WDTR."]
        #[inline(always)]
        pub const fn set_wdtr(&mut self, val: bool) {
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
        #[doc = "desc WDINT."]
        #[must_use]
        #[inline(always)]
        pub const fn wdint(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "desc WDINT."]
        #[inline(always)]
        pub const fn set_wdint(&mut self, val: bool) {
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
                .field("wdtr", &self.wdtr())
                .field("wint_en", &self.wint_en())
                .field("wdint", &self.wdint())
                .field("wcntl", &self.wcntl())
                .finish()
        }
    }
    #[doc = "desc RST."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Rst(pub u32);
    impl Rst {
        #[doc = "desc WDTRST."]
        #[must_use]
        #[inline(always)]
        pub const fn wdtrst(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[doc = "desc WDTRST."]
        #[inline(always)]
        pub const fn set_wdtrst(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
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
                .field("wdtrst", &self.wdtrst())
                .finish()
        }
    }
}
