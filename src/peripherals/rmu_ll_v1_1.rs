#![allow(clippy::missing_safety_doc)]
#![allow(clippy::identity_op)]
#![allow(clippy::unnecessary_cast)]
#![allow(clippy::erasing_op)]

#[doc = "desc RMU."]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Rmu {
    ptr: *mut u8,
}
unsafe impl Send for Rmu {}
unsafe impl Sync for Rmu {}
impl Rmu {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "desc RSTF0."]
    #[inline(always)]
    pub const fn rstf0(self) -> crate::common::Reg<regs::Rstf0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0usize) as _) }
    }
}
pub mod regs {
    #[doc = "desc RSTF0."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Rstf0(pub u16);
    impl Rstf0 {
        #[doc = "desc PORF."]
        #[must_use]
        #[inline(always)]
        pub const fn porf(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "desc PORF."]
        #[inline(always)]
        pub const fn set_porf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u16) & 0x01) << 0usize);
        }
        #[doc = "desc PINRF."]
        #[must_use]
        #[inline(always)]
        pub const fn pinrf(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "desc PINRF."]
        #[inline(always)]
        pub const fn set_pinrf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u16) & 0x01) << 1usize);
        }
        #[doc = "desc LVD1RF."]
        #[must_use]
        #[inline(always)]
        pub const fn lvd1rf(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "desc LVD1RF."]
        #[inline(always)]
        pub const fn set_lvd1rf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u16) & 0x01) << 2usize);
        }
        #[doc = "desc LVD2RF."]
        #[must_use]
        #[inline(always)]
        pub const fn lvd2rf(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "desc LVD2RF."]
        #[inline(always)]
        pub const fn set_lvd2rf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u16) & 0x01) << 3usize);
        }
        #[doc = "desc WDRF."]
        #[must_use]
        #[inline(always)]
        pub const fn wdrf(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "desc WDRF."]
        #[inline(always)]
        pub const fn set_wdrf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u16) & 0x01) << 5usize);
        }
        #[doc = "desc SWRF."]
        #[must_use]
        #[inline(always)]
        pub const fn swrf(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "desc SWRF."]
        #[inline(always)]
        pub const fn set_swrf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u16) & 0x01) << 8usize);
        }
        #[doc = "desc CKFERF."]
        #[must_use]
        #[inline(always)]
        pub const fn ckferf(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "desc CKFERF."]
        #[inline(always)]
        pub const fn set_ckferf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u16) & 0x01) << 9usize);
        }
        #[doc = "desc RAECRF."]
        #[must_use]
        #[inline(always)]
        pub const fn raecrf(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "desc RAECRF."]
        #[inline(always)]
        pub const fn set_raecrf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u16) & 0x01) << 10usize);
        }
        #[doc = "desc FLECRF."]
        #[must_use]
        #[inline(always)]
        pub const fn flecrf(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "desc FLECRF."]
        #[inline(always)]
        pub const fn set_flecrf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u16) & 0x01) << 11usize);
        }
        #[doc = "desc CPULKUPRF."]
        #[must_use]
        #[inline(always)]
        pub const fn cpulkuprf(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "desc CPULKUPRF."]
        #[inline(always)]
        pub const fn set_cpulkuprf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u16) & 0x01) << 12usize);
        }
        #[doc = "desc XTALERF."]
        #[must_use]
        #[inline(always)]
        pub const fn xtalerf(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[doc = "desc XTALERF."]
        #[inline(always)]
        pub const fn set_xtalerf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u16) & 0x01) << 13usize);
        }
        #[doc = "desc MULTIRF."]
        #[must_use]
        #[inline(always)]
        pub const fn multirf(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "desc MULTIRF."]
        #[inline(always)]
        pub const fn set_multirf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u16) & 0x01) << 14usize);
        }
        #[doc = "desc CLRF."]
        #[must_use]
        #[inline(always)]
        pub const fn clrf(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "desc CLRF."]
        #[inline(always)]
        pub const fn set_clrf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u16) & 0x01) << 15usize);
        }
    }
    impl Default for Rstf0 {
        #[inline(always)]
        fn default() -> Rstf0 {
            Rstf0(0)
        }
    }
    impl core::fmt::Debug for Rstf0 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Rstf0")
                .field("porf", &self.porf())
                .field("pinrf", &self.pinrf())
                .field("lvd1rf", &self.lvd1rf())
                .field("lvd2rf", &self.lvd2rf())
                .field("wdrf", &self.wdrf())
                .field("swrf", &self.swrf())
                .field("ckferf", &self.ckferf())
                .field("raecrf", &self.raecrf())
                .field("flecrf", &self.flecrf())
                .field("cpulkuprf", &self.cpulkuprf())
                .field("xtalerf", &self.xtalerf())
                .field("multirf", &self.multirf())
                .field("clrf", &self.clrf())
                .finish()
        }
    }
}
