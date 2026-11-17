#![allow(clippy::missing_safety_doc)]
#![allow(clippy::identity_op)]
#![allow(clippy::unnecessary_cast)]
#![allow(clippy::erasing_op)]

#[doc = "desc OTS."]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ots {
    ptr: *mut u8,
}
unsafe impl Send for Ots {}
unsafe impl Sync for Ots {}
impl Ots {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "desc CTL."]
    #[inline(always)]
    pub const fn ctl(self) -> crate::common::Reg<regs::Ctl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0usize) as _) }
    }
    #[doc = "desc DR1."]
    #[inline(always)]
    pub const fn dr1(self) -> crate::common::Reg<u16, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x02usize) as _) }
    }
    #[doc = "desc DR2."]
    #[inline(always)]
    pub const fn dr2(self) -> crate::common::Reg<u16, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x04usize) as _) }
    }
    #[doc = "desc ECR."]
    #[inline(always)]
    pub const fn ecr(self) -> crate::common::Reg<u16, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x06usize) as _) }
    }
}
pub mod regs {
    #[doc = "desc CTL."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ctl(pub u16);
    impl Ctl {
        #[doc = "desc OTSST."]
        #[must_use]
        #[inline(always)]
        pub const fn otsst(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "desc OTSST."]
        #[inline(always)]
        pub const fn set_otsst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u16) & 0x01) << 0usize);
        }
        #[doc = "desc OTSCK."]
        #[must_use]
        #[inline(always)]
        pub const fn otsck(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "desc OTSCK."]
        #[inline(always)]
        pub const fn set_otsck(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u16) & 0x01) << 1usize);
        }
        #[doc = "desc OTSIE."]
        #[must_use]
        #[inline(always)]
        pub const fn otsie(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "desc OTSIE."]
        #[inline(always)]
        pub const fn set_otsie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u16) & 0x01) << 2usize);
        }
        #[doc = "desc TSSTP."]
        #[must_use]
        #[inline(always)]
        pub const fn tsstp(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "desc TSSTP."]
        #[inline(always)]
        pub const fn set_tsstp(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u16) & 0x01) << 3usize);
        }
    }
    impl Default for Ctl {
        #[inline(always)]
        fn default() -> Ctl {
            Ctl(0)
        }
    }
    impl core::fmt::Debug for Ctl {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Ctl")
                .field("otsst", &self.otsst())
                .field("otsck", &self.otsck())
                .field("otsie", &self.otsie())
                .field("tsstp", &self.tsstp())
                .finish()
        }
    }
}
