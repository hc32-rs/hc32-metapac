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
    #[doc = "desc SRAM0_EIEN."]
    #[inline(always)]
    pub const fn sram0_eien(self) -> crate::common::Reg<regs::Sram0Eien, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x14usize) as _) }
    }
    #[doc = "desc SRAM0_EIBIT0."]
    #[inline(always)]
    pub const fn sram0_eibit0(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x18usize) as _) }
    }
    #[doc = "desc SRAM0_EIBIT1."]
    #[inline(always)]
    pub const fn sram0_eibit1(self) -> crate::common::Reg<regs::Sram0Eibit1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1cusize) as _) }
    }
    #[doc = "desc SRAM0_ECCERRADDR."]
    #[inline(always)]
    pub const fn sram0_eccerraddr(
        self,
    ) -> crate::common::Reg<regs::Sram0Eccerraddr, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x20usize) as _) }
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
                .field("eccmod", &self.eccmod())
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
        #[doc = "desc SRAM0_1ERR."]
        #[must_use]
        #[inline(always)]
        pub const fn sram0_1err(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "desc SRAM0_1ERR."]
        #[inline(always)]
        pub const fn set_sram0_1err(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "desc SRAM0_2ERR."]
        #[must_use]
        #[inline(always)]
        pub const fn sram0_2err(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "desc SRAM0_2ERR."]
        #[inline(always)]
        pub const fn set_sram0_2err(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
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
                .field("sramh_pyerr", &self.sramh_pyerr())
                .field("sram0_1err", &self.sram0_1err())
                .field("sram0_2err", &self.sram0_2err())
                .field("cache_pyerr", &self.cache_pyerr())
                .finish()
        }
    }
    #[doc = "desc SRAM0_ECCERRADDR."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Sram0Eccerraddr(pub u32);
    impl Sram0Eccerraddr {
        #[doc = "desc ECCERRADDR."]
        #[must_use]
        #[inline(always)]
        pub const fn eccerraddr(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x7fff;
            val as u16
        }
        #[doc = "desc ECCERRADDR."]
        #[inline(always)]
        pub const fn set_eccerraddr(&mut self, val: u16) {
            self.0 = (self.0 & !(0x7fff << 0usize)) | (((val as u32) & 0x7fff) << 0usize);
        }
    }
    impl Default for Sram0Eccerraddr {
        #[inline(always)]
        fn default() -> Sram0Eccerraddr {
            Sram0Eccerraddr(0)
        }
    }
    impl core::fmt::Debug for Sram0Eccerraddr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Sram0Eccerraddr")
                .field("eccerraddr", &self.eccerraddr())
                .finish()
        }
    }
    #[doc = "desc SRAM0_EIBIT1."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Sram0Eibit1(pub u32);
    impl Sram0Eibit1 {
        #[doc = "desc EIBIT."]
        #[must_use]
        #[inline(always)]
        pub const fn eibit(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x7f;
            val as u8
        }
        #[doc = "desc EIBIT."]
        #[inline(always)]
        pub const fn set_eibit(&mut self, val: u8) {
            self.0 = (self.0 & !(0x7f << 0usize)) | (((val as u32) & 0x7f) << 0usize);
        }
    }
    impl Default for Sram0Eibit1 {
        #[inline(always)]
        fn default() -> Sram0Eibit1 {
            Sram0Eibit1(0)
        }
    }
    impl core::fmt::Debug for Sram0Eibit1 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Sram0Eibit1")
                .field("eibit", &self.eibit())
                .finish()
        }
    }
    #[doc = "desc SRAM0_EIEN."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Sram0Eien(pub u32);
    impl Sram0Eien {
        #[doc = "desc EIEN."]
        #[must_use]
        #[inline(always)]
        pub const fn eien(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "desc EIEN."]
        #[inline(always)]
        pub const fn set_eien(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
    }
    impl Default for Sram0Eien {
        #[inline(always)]
        fn default() -> Sram0Eien {
            Sram0Eien(0)
        }
    }
    impl core::fmt::Debug for Sram0Eien {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Sram0Eien")
                .field("eien", &self.eien())
                .finish()
        }
    }
}
