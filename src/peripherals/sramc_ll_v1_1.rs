#![allow(clippy::missing_safety_doc)]
#![allow(clippy::identity_op)]
#![allow(clippy::unnecessary_cast)]
#![allow(clippy::erasing_op)]

#[doc = "desc SRAMC."]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sramc {
    ptr: *mut u8,
}
unsafe impl Send for Sramc {}
unsafe impl Sync for Sramc {}
impl Sramc {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "desc WTCR."]
    #[inline(always)]
    pub const fn wtcr(self) -> crate::common::Reg<regs::Wtcr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0usize) as _) }
    }
    #[doc = "desc WTPR."]
    #[inline(always)]
    pub const fn wtpr(self) -> crate::common::Reg<regs::Wtpr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x04usize) as _) }
    }
    #[doc = "desc CKCR."]
    #[inline(always)]
    pub const fn ckcr(self) -> crate::common::Reg<regs::Ckcr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x08usize) as _) }
    }
    #[doc = "desc CKPR."]
    #[inline(always)]
    pub const fn ckpr(self) -> crate::common::Reg<regs::Ckpr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0cusize) as _) }
    }
    #[doc = "desc CKSR."]
    #[inline(always)]
    pub const fn cksr(self) -> crate::common::Reg<regs::Cksr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x10usize) as _) }
    }
}
pub mod regs {
    #[doc = "desc CKCR."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ckcr(pub u32);
    impl Ckcr {
        #[doc = "desc PYOAD."]
        #[must_use]
        #[inline(always)]
        pub const fn pyoad(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "desc PYOAD."]
        #[inline(always)]
        pub const fn set_pyoad(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "desc ECCOAD."]
        #[must_use]
        #[inline(always)]
        pub const fn eccoad(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "desc ECCOAD."]
        #[inline(always)]
        pub const fn set_eccoad(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "desc BECCOAD."]
        #[must_use]
        #[inline(always)]
        pub const fn beccoad(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "desc BECCOAD."]
        #[inline(always)]
        pub const fn set_beccoad(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[doc = "desc ECCMOD."]
        #[must_use]
        #[inline(always)]
        pub const fn eccmod(&self) -> u8 {
            let val = (self.0 >> 24usize) & 0x03;
            val as u8
        }
        #[doc = "desc ECCMOD."]
        #[inline(always)]
        pub const fn set_eccmod(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 24usize)) | (((val as u32) & 0x03) << 24usize);
        }
        #[doc = "desc BECCMOD."]
        #[must_use]
        #[inline(always)]
        pub const fn beccmod(&self) -> u8 {
            let val = (self.0 >> 26usize) & 0x03;
            val as u8
        }
        #[doc = "desc BECCMOD."]
        #[inline(always)]
        pub const fn set_beccmod(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 26usize)) | (((val as u32) & 0x03) << 26usize);
        }
    }
    impl Default for Ckcr {
        #[inline(always)]
        fn default() -> Ckcr {
            Ckcr(0)
        }
    }
    impl core::fmt::Debug for Ckcr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Ckcr")
                .field("pyoad", &self.pyoad())
                .field("eccoad", &self.eccoad())
                .field("beccoad", &self.beccoad())
                .field("eccmod", &self.eccmod())
                .field("beccmod", &self.beccmod())
                .finish()
        }
    }
    #[doc = "desc CKPR."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ckpr(pub u32);
    impl Ckpr {
        #[doc = "desc CKPRC."]
        #[must_use]
        #[inline(always)]
        pub const fn ckprc(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "desc CKPRC."]
        #[inline(always)]
        pub const fn set_ckprc(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "desc CKPRKW."]
        #[must_use]
        #[inline(always)]
        pub const fn ckprkw(&self) -> u8 {
            let val = (self.0 >> 1usize) & 0x7f;
            val as u8
        }
        #[doc = "desc CKPRKW."]
        #[inline(always)]
        pub const fn set_ckprkw(&mut self, val: u8) {
            self.0 = (self.0 & !(0x7f << 1usize)) | (((val as u32) & 0x7f) << 1usize);
        }
    }
    impl Default for Ckpr {
        #[inline(always)]
        fn default() -> Ckpr {
            Ckpr(0)
        }
    }
    impl core::fmt::Debug for Ckpr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Ckpr")
                .field("ckprc", &self.ckprc())
                .field("ckprkw", &self.ckprkw())
                .finish()
        }
    }
    #[doc = "desc CKSR."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Cksr(pub u32);
    impl Cksr {
        #[doc = "desc SRAM1_PYERR."]
        #[must_use]
        #[inline(always)]
        pub const fn sram1_pyerr(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "desc SRAM1_PYERR."]
        #[inline(always)]
        pub const fn set_sram1_pyerr(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "desc SRAM2_PYERR."]
        #[must_use]
        #[inline(always)]
        pub const fn sram2_pyerr(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "desc SRAM2_PYERR."]
        #[inline(always)]
        pub const fn set_sram2_pyerr(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "desc SRAM3_PYERR."]
        #[must_use]
        #[inline(always)]
        pub const fn sram3_pyerr(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "desc SRAM3_PYERR."]
        #[inline(always)]
        pub const fn set_sram3_pyerr(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "desc SRAMH_PYERR."]
        #[must_use]
        #[inline(always)]
        pub const fn sramh_pyerr(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "desc SRAMH_PYERR."]
        #[inline(always)]
        pub const fn set_sramh_pyerr(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "desc SRAM4_1ERR."]
        #[must_use]
        #[inline(always)]
        pub const fn sram4_1err(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "desc SRAM4_1ERR."]
        #[inline(always)]
        pub const fn set_sram4_1err(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "desc SRAM4_2ERR."]
        #[must_use]
        #[inline(always)]
        pub const fn sram4_2err(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "desc SRAM4_2ERR."]
        #[inline(always)]
        pub const fn set_sram4_2err(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "desc SRAMB_1ERR."]
        #[must_use]
        #[inline(always)]
        pub const fn sramb_1err(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "desc SRAMB_1ERR."]
        #[inline(always)]
        pub const fn set_sramb_1err(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "desc SRAMB_2ERR."]
        #[must_use]
        #[inline(always)]
        pub const fn sramb_2err(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "desc SRAMB_2ERR."]
        #[inline(always)]
        pub const fn set_sramb_2err(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "desc CACHE_PYERR."]
        #[must_use]
        #[inline(always)]
        pub const fn cache_pyerr(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "desc CACHE_PYERR."]
        #[inline(always)]
        pub const fn set_cache_pyerr(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
    }
    impl Default for Cksr {
        #[inline(always)]
        fn default() -> Cksr {
            Cksr(0)
        }
    }
    impl core::fmt::Debug for Cksr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Cksr")
                .field("sram1_pyerr", &self.sram1_pyerr())
                .field("sram2_pyerr", &self.sram2_pyerr())
                .field("sram3_pyerr", &self.sram3_pyerr())
                .field("sramh_pyerr", &self.sramh_pyerr())
                .field("sram4_1err", &self.sram4_1err())
                .field("sram4_2err", &self.sram4_2err())
                .field("sramb_1err", &self.sramb_1err())
                .field("sramb_2err", &self.sramb_2err())
                .field("cache_pyerr", &self.cache_pyerr())
                .finish()
        }
    }
    #[doc = "desc WTCR."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Wtcr(pub u32);
    impl Wtcr {
        #[doc = "desc SRAM123RWT."]
        #[must_use]
        #[inline(always)]
        pub const fn sram123rwt(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x07;
            val as u8
        }
        #[doc = "desc SRAM123RWT."]
        #[inline(always)]
        pub const fn set_sram123rwt(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 0usize)) | (((val as u32) & 0x07) << 0usize);
        }
        #[doc = "desc SRAM123WWT."]
        #[must_use]
        #[inline(always)]
        pub const fn sram123wwt(&self) -> u8 {
            let val = (self.0 >> 4usize) & 0x07;
            val as u8
        }
        #[doc = "desc SRAM123WWT."]
        #[inline(always)]
        pub const fn set_sram123wwt(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 4usize)) | (((val as u32) & 0x07) << 4usize);
        }
        #[doc = "desc SRAM4RWT."]
        #[must_use]
        #[inline(always)]
        pub const fn sram4rwt(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0x07;
            val as u8
        }
        #[doc = "desc SRAM4RWT."]
        #[inline(always)]
        pub const fn set_sram4rwt(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 8usize)) | (((val as u32) & 0x07) << 8usize);
        }
        #[doc = "desc SRAM4WWT."]
        #[must_use]
        #[inline(always)]
        pub const fn sram4wwt(&self) -> u8 {
            let val = (self.0 >> 12usize) & 0x07;
            val as u8
        }
        #[doc = "desc SRAM4WWT."]
        #[inline(always)]
        pub const fn set_sram4wwt(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 12usize)) | (((val as u32) & 0x07) << 12usize);
        }
        #[doc = "desc SRAMHRWT."]
        #[must_use]
        #[inline(always)]
        pub const fn sramhrwt(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x07;
            val as u8
        }
        #[doc = "desc SRAMHRWT."]
        #[inline(always)]
        pub const fn set_sramhrwt(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 16usize)) | (((val as u32) & 0x07) << 16usize);
        }
        #[doc = "desc SRAMHWWT."]
        #[must_use]
        #[inline(always)]
        pub const fn sramhwwt(&self) -> u8 {
            let val = (self.0 >> 20usize) & 0x07;
            val as u8
        }
        #[doc = "desc SRAMHWWT."]
        #[inline(always)]
        pub const fn set_sramhwwt(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 20usize)) | (((val as u32) & 0x07) << 20usize);
        }
        #[doc = "desc SRAMBRWT."]
        #[must_use]
        #[inline(always)]
        pub const fn srambrwt(&self) -> u8 {
            let val = (self.0 >> 24usize) & 0x07;
            val as u8
        }
        #[doc = "desc SRAMBRWT."]
        #[inline(always)]
        pub const fn set_srambrwt(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 24usize)) | (((val as u32) & 0x07) << 24usize);
        }
        #[doc = "desc SRAMBWWT."]
        #[must_use]
        #[inline(always)]
        pub const fn srambwwt(&self) -> u8 {
            let val = (self.0 >> 28usize) & 0x07;
            val as u8
        }
        #[doc = "desc SRAMBWWT."]
        #[inline(always)]
        pub const fn set_srambwwt(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 28usize)) | (((val as u32) & 0x07) << 28usize);
        }
    }
    impl Default for Wtcr {
        #[inline(always)]
        fn default() -> Wtcr {
            Wtcr(0)
        }
    }
    impl core::fmt::Debug for Wtcr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Wtcr")
                .field("sram123rwt", &self.sram123rwt())
                .field("sram123wwt", &self.sram123wwt())
                .field("sram4rwt", &self.sram4rwt())
                .field("sram4wwt", &self.sram4wwt())
                .field("sramhrwt", &self.sramhrwt())
                .field("sramhwwt", &self.sramhwwt())
                .field("srambrwt", &self.srambrwt())
                .field("srambwwt", &self.srambwwt())
                .finish()
        }
    }
    #[doc = "desc WTPR."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Wtpr(pub u32);
    impl Wtpr {
        #[doc = "desc WTPRC."]
        #[must_use]
        #[inline(always)]
        pub const fn wtprc(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "desc WTPRC."]
        #[inline(always)]
        pub const fn set_wtprc(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "desc WTPRKW."]
        #[must_use]
        #[inline(always)]
        pub const fn wtprkw(&self) -> u8 {
            let val = (self.0 >> 1usize) & 0x7f;
            val as u8
        }
        #[doc = "desc WTPRKW."]
        #[inline(always)]
        pub const fn set_wtprkw(&mut self, val: u8) {
            self.0 = (self.0 & !(0x7f << 1usize)) | (((val as u32) & 0x7f) << 1usize);
        }
    }
    impl Default for Wtpr {
        #[inline(always)]
        fn default() -> Wtpr {
            Wtpr(0)
        }
    }
    impl core::fmt::Debug for Wtpr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Wtpr")
                .field("wtprc", &self.wtprc())
                .field("wtprkw", &self.wtprkw())
                .finish()
        }
    }
}
