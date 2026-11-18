#![allow(clippy::missing_safety_doc)]
#![allow(clippy::identity_op)]
#![allow(clippy::unnecessary_cast)]
#![allow(clippy::erasing_op)]

#[doc = "desc I2C."]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct I2c {
    ptr: *mut u8,
}
unsafe impl Send for I2c {}
unsafe impl Sync for I2c {}
impl I2c {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "desc TMRUN."]
    #[inline(always)]
    pub const fn tmrun(self) -> crate::common::Reg<regs::Tmrun, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0usize) as _) }
    }
    #[doc = "desc TM."]
    #[inline(always)]
    pub const fn tm(self) -> crate::common::Reg<regs::Tm, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x04usize) as _) }
    }
    #[doc = "desc CR."]
    #[inline(always)]
    pub const fn cr(self) -> crate::common::Reg<regs::Cr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x08usize) as _) }
    }
    #[doc = "desc DATA."]
    #[inline(always)]
    pub const fn data(self) -> crate::common::Reg<regs::Data, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0cusize) as _) }
    }
    #[doc = "desc ADDR."]
    #[inline(always)]
    pub const fn addr(self) -> crate::common::Reg<regs::Addr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x10usize) as _) }
    }
    #[doc = "desc STAT."]
    #[inline(always)]
    pub const fn stat(self) -> crate::common::Reg<regs::Stat, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x14usize) as _) }
    }
}
pub mod regs {
    #[doc = "desc ADDR."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Addr(pub u32);
    impl Addr {
        #[doc = "desc GC."]
        #[must_use]
        #[inline(always)]
        pub const fn gc(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "desc GC."]
        #[inline(always)]
        pub const fn set_gc(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "desc I2CADR."]
        #[must_use]
        #[inline(always)]
        pub const fn i2cadr(&self) -> u8 {
            let val = (self.0 >> 1usize) & 0x7f;
            val as u8
        }
        #[doc = "desc I2CADR."]
        #[inline(always)]
        pub const fn set_i2cadr(&mut self, val: u8) {
            self.0 = (self.0 & !(0x7f << 1usize)) | (((val as u32) & 0x7f) << 1usize);
        }
    }
    impl Default for Addr {
        #[inline(always)]
        fn default() -> Addr {
            Addr(0)
        }
    }
    impl core::fmt::Debug for Addr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Addr")
                .field("gc", &self.gc())
                .field("i2cadr", &self.i2cadr())
                .finish()
        }
    }
    #[doc = "desc CR."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Cr(pub u32);
    impl Cr {
        #[doc = "desc H1M."]
        #[must_use]
        #[inline(always)]
        pub const fn h1m(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "desc H1M."]
        #[inline(always)]
        pub const fn set_h1m(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "desc AA."]
        #[must_use]
        #[inline(always)]
        pub const fn aa(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "desc AA."]
        #[inline(always)]
        pub const fn set_aa(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "desc SI."]
        #[must_use]
        #[inline(always)]
        pub const fn si(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "desc SI."]
        #[inline(always)]
        pub const fn set_si(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "desc STO."]
        #[must_use]
        #[inline(always)]
        pub const fn sto(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "desc STO."]
        #[inline(always)]
        pub const fn set_sto(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "desc STA."]
        #[must_use]
        #[inline(always)]
        pub const fn sta(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "desc STA."]
        #[inline(always)]
        pub const fn set_sta(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "desc ENS."]
        #[must_use]
        #[inline(always)]
        pub const fn ens(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "desc ENS."]
        #[inline(always)]
        pub const fn set_ens(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
    }
    impl Default for Cr {
        #[inline(always)]
        fn default() -> Cr {
            Cr(0)
        }
    }
    impl core::fmt::Debug for Cr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Cr")
                .field("h1m", &self.h1m())
                .field("aa", &self.aa())
                .field("si", &self.si())
                .field("sto", &self.sto())
                .field("sta", &self.sta())
                .field("ens", &self.ens())
                .finish()
        }
    }
    #[doc = "desc DATA."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Data(pub u32);
    impl Data {
        #[doc = "desc I2CDAT."]
        #[must_use]
        #[inline(always)]
        pub const fn i2cdat(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[doc = "desc I2CDAT."]
        #[inline(always)]
        pub const fn set_i2cdat(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
        }
    }
    impl Default for Data {
        #[inline(always)]
        fn default() -> Data {
            Data(0)
        }
    }
    impl core::fmt::Debug for Data {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Data")
                .field("i2cdat", &self.i2cdat())
                .finish()
        }
    }
    #[doc = "desc STAT."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Stat(pub u32);
    impl Stat {
        #[doc = "desc I2CSTA."]
        #[must_use]
        #[inline(always)]
        pub const fn i2csta(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[doc = "desc I2CSTA."]
        #[inline(always)]
        pub const fn set_i2csta(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
        }
    }
    impl Default for Stat {
        #[inline(always)]
        fn default() -> Stat {
            Stat(0)
        }
    }
    impl core::fmt::Debug for Stat {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Stat")
                .field("i2csta", &self.i2csta())
                .finish()
        }
    }
    #[doc = "desc TM."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Tm(pub u32);
    impl Tm {
        #[doc = "desc TM."]
        #[must_use]
        #[inline(always)]
        pub const fn tm(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[doc = "desc TM."]
        #[inline(always)]
        pub const fn set_tm(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
        }
    }
    impl Default for Tm {
        #[inline(always)]
        fn default() -> Tm {
            Tm(0)
        }
    }
    impl core::fmt::Debug for Tm {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Tm").field("tm", &self.tm()).finish()
        }
    }
    #[doc = "desc TMRUN."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Tmrun(pub u32);
    impl Tmrun {
        #[doc = "desc TME."]
        #[must_use]
        #[inline(always)]
        pub const fn tme(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "desc TME."]
        #[inline(always)]
        pub const fn set_tme(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
    }
    impl Default for Tmrun {
        #[inline(always)]
        fn default() -> Tmrun {
            Tmrun(0)
        }
    }
    impl core::fmt::Debug for Tmrun {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Tmrun").field("tme", &self.tme()).finish()
        }
    }
}
