#![allow(clippy::missing_safety_doc)]
#![allow(clippy::identity_op)]
#![allow(clippy::unnecessary_cast)]
#![allow(clippy::erasing_op)]

#[doc = "desc CSU."]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Csu {
    ptr: *mut u8,
}
unsafe impl Send for Csu {}
unsafe impl Sync for Csu {}
impl Csu {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "desc CR."]
    #[inline(always)]
    pub const fn cr(self) -> crate::common::Reg<regs::Cr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0usize) as _) }
    }
    #[doc = "desc LVR."]
    #[inline(always)]
    pub const fn lvr(self) -> crate::common::Reg<regs::Lvr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x04usize) as _) }
    }
    #[doc = "desc UVR."]
    #[inline(always)]
    pub const fn uvr(self) -> crate::common::Reg<regs::Uvr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x08usize) as _) }
    }
    #[doc = "desc CNTR."]
    #[inline(always)]
    pub const fn cntr(self) -> crate::common::Reg<regs::Cntr, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0cusize) as _) }
    }
    #[doc = "desc CFGR."]
    #[inline(always)]
    pub const fn cfgr(self) -> crate::common::Reg<regs::Cfgr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x10usize) as _) }
    }
    #[doc = "desc SR."]
    #[inline(always)]
    pub const fn sr(self) -> crate::common::Reg<regs::Sr, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x14usize) as _) }
    }
    #[doc = "desc CLR."]
    #[inline(always)]
    pub const fn clr(self) -> crate::common::Reg<regs::Clr, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x18usize) as _) }
    }
}
pub mod regs {
    #[doc = "desc CFGR."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Cfgr(pub u32);
    impl Cfgr {
        #[doc = "desc RDIVS."]
        #[must_use]
        #[inline(always)]
        pub const fn rdivs(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x03;
            val as u8
        }
        #[doc = "desc RDIVS."]
        #[inline(always)]
        pub const fn set_rdivs(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 0usize)) | (((val as u32) & 0x03) << 0usize);
        }
        #[doc = "desc SDIVS."]
        #[must_use]
        #[inline(always)]
        pub const fn sdivs(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0x03;
            val as u8
        }
        #[doc = "desc SDIVS."]
        #[inline(always)]
        pub const fn set_sdivs(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 8usize)) | (((val as u32) & 0x03) << 8usize);
        }
        #[doc = "desc STPRE."]
        #[must_use]
        #[inline(always)]
        pub const fn stpre(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "desc STPRE."]
        #[inline(always)]
        pub const fn set_stpre(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "desc FOVRE."]
        #[must_use]
        #[inline(always)]
        pub const fn fovre(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "desc FOVRE."]
        #[inline(always)]
        pub const fn set_fovre(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[doc = "desc STPPET."]
        #[must_use]
        #[inline(always)]
        pub const fn stppet(&self) -> bool {
            let val = (self.0 >> 20usize) & 0x01;
            val != 0
        }
        #[doc = "desc STPPET."]
        #[inline(always)]
        pub const fn set_stppet(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
        }
        #[doc = "desc FOVPET."]
        #[must_use]
        #[inline(always)]
        pub const fn fovpet(&self) -> bool {
            let val = (self.0 >> 21usize) & 0x01;
            val != 0
        }
        #[doc = "desc FOVPET."]
        #[inline(always)]
        pub const fn set_fovpet(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
        }
        #[doc = "desc FUDPET."]
        #[must_use]
        #[inline(always)]
        pub const fn fudpet(&self) -> bool {
            let val = (self.0 >> 22usize) & 0x01;
            val != 0
        }
        #[doc = "desc FUDPET."]
        #[inline(always)]
        pub const fn set_fudpet(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
        }
    }
    impl Default for Cfgr {
        #[inline(always)]
        fn default() -> Cfgr {
            Cfgr(0)
        }
    }
    impl core::fmt::Debug for Cfgr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Cfgr")
                .field("rdivs", &self.rdivs())
                .field("sdivs", &self.sdivs())
                .field("stpre", &self.stpre())
                .field("fovre", &self.fovre())
                .field("stppet", &self.stppet())
                .field("fovpet", &self.fovpet())
                .field("fudpet", &self.fudpet())
                .finish()
        }
    }
    #[doc = "desc CLR."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Clr(pub u32);
    impl Clr {
        #[doc = "desc STPERR."]
        #[must_use]
        #[inline(always)]
        pub const fn stperr(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "desc STPERR."]
        #[inline(always)]
        pub const fn set_stperr(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "desc FOVERR."]
        #[must_use]
        #[inline(always)]
        pub const fn foverr(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "desc FOVERR."]
        #[inline(always)]
        pub const fn set_foverr(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "desc FUDERR."]
        #[must_use]
        #[inline(always)]
        pub const fn fuderr(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "desc FUDERR."]
        #[inline(always)]
        pub const fn set_fuderr(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
    }
    impl Default for Clr {
        #[inline(always)]
        fn default() -> Clr {
            Clr(0)
        }
    }
    impl core::fmt::Debug for Clr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Clr")
                .field("stperr", &self.stperr())
                .field("foverr", &self.foverr())
                .field("fuderr", &self.fuderr())
                .finish()
        }
    }
    #[doc = "desc CNTR."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Cntr(pub u32);
    impl Cntr {
        #[doc = "desc CNTR."]
        #[must_use]
        #[inline(always)]
        pub const fn cntr(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[doc = "desc CNTR."]
        #[inline(always)]
        pub const fn set_cntr(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
        }
    }
    impl Default for Cntr {
        #[inline(always)]
        fn default() -> Cntr {
            Cntr(0)
        }
    }
    impl core::fmt::Debug for Cntr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Cntr").field("cntr", &self.cntr()).finish()
        }
    }
    #[doc = "desc CR."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Cr(pub u32);
    impl Cr {
        #[doc = "desc CSE."]
        #[must_use]
        #[inline(always)]
        pub const fn cse(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "desc CSE."]
        #[inline(always)]
        pub const fn set_cse(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
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
            f.debug_struct("Cr").field("cse", &self.cse()).finish()
        }
    }
    #[doc = "desc LVR."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Lvr(pub u32);
    impl Lvr {
        #[doc = "desc LVR."]
        #[must_use]
        #[inline(always)]
        pub const fn lvr(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[doc = "desc LVR."]
        #[inline(always)]
        pub const fn set_lvr(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
        }
    }
    impl Default for Lvr {
        #[inline(always)]
        fn default() -> Lvr {
            Lvr(0)
        }
    }
    impl core::fmt::Debug for Lvr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Lvr").field("lvr", &self.lvr()).finish()
        }
    }
    #[doc = "desc SR."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Sr(pub u32);
    impl Sr {
        #[doc = "desc STPERR."]
        #[must_use]
        #[inline(always)]
        pub const fn stperr(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "desc STPERR."]
        #[inline(always)]
        pub const fn set_stperr(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "desc FOVERR."]
        #[must_use]
        #[inline(always)]
        pub const fn foverr(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "desc FOVERR."]
        #[inline(always)]
        pub const fn set_foverr(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "desc FUDERR."]
        #[must_use]
        #[inline(always)]
        pub const fn fuderr(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "desc FUDERR."]
        #[inline(always)]
        pub const fn set_fuderr(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "desc CSEF."]
        #[must_use]
        #[inline(always)]
        pub const fn csef(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "desc CSEF."]
        #[inline(always)]
        pub const fn set_csef(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
    }
    impl Default for Sr {
        #[inline(always)]
        fn default() -> Sr {
            Sr(0)
        }
    }
    impl core::fmt::Debug for Sr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Sr")
                .field("stperr", &self.stperr())
                .field("foverr", &self.foverr())
                .field("fuderr", &self.fuderr())
                .field("csef", &self.csef())
                .finish()
        }
    }
    #[doc = "desc UVR."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Uvr(pub u32);
    impl Uvr {
        #[doc = "desc UVR."]
        #[must_use]
        #[inline(always)]
        pub const fn uvr(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x1fff;
            val as u16
        }
        #[doc = "desc UVR."]
        #[inline(always)]
        pub const fn set_uvr(&mut self, val: u16) {
            self.0 = (self.0 & !(0x1fff << 0usize)) | (((val as u32) & 0x1fff) << 0usize);
        }
    }
    impl Default for Uvr {
        #[inline(always)]
        fn default() -> Uvr {
            Uvr(0)
        }
    }
    impl core::fmt::Debug for Uvr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Uvr").field("uvr", &self.uvr()).finish()
        }
    }
}
