#![allow(clippy::missing_safety_doc)]
#![allow(clippy::identity_op)]
#![allow(clippy::unnecessary_cast)]
#![allow(clippy::erasing_op)]

#[doc = "desc DBGC."]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dbgc {
    ptr: *mut u8,
}
unsafe impl Send for Dbgc {}
unsafe impl Sync for Dbgc {}
impl Dbgc {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "desc FMCCTL."]
    #[inline(always)]
    pub const fn fmcctl(self) -> crate::common::Reg<regs::Fmcctl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x18usize) as _) }
    }
    #[doc = "desc FCRCCR0."]
    #[inline(always)]
    pub const fn fcrccr0(self) -> crate::common::Reg<regs::Fcrccr0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x30usize) as _) }
    }
    #[doc = "desc FCRCCR1."]
    #[inline(always)]
    pub const fn fcrccr1(self) -> crate::common::Reg<regs::Fcrccr1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x34usize) as _) }
    }
    #[doc = "desc FCRCDOR."]
    #[inline(always)]
    pub const fn fcrcdor(self) -> crate::common::Reg<u32, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x38usize) as _) }
    }
    #[doc = "desc FCRCREF."]
    #[inline(always)]
    pub const fn fcrcref(self) -> crate::common::Reg<u32, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x3cusize) as _) }
    }
}
pub mod regs {
    #[doc = "desc FCRCCR0."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Fcrccr0(pub u32);
    impl Fcrccr0 {
        #[doc = "desc FCRCREQ."]
        #[must_use]
        #[inline(always)]
        pub const fn fcrcreq(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "desc FCRCREQ."]
        #[inline(always)]
        pub const fn set_fcrcreq(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "desc FCRCACK."]
        #[must_use]
        #[inline(always)]
        pub const fn fcrcack(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "desc FCRCACK."]
        #[inline(always)]
        pub const fn set_fcrcack(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "desc FCRCERR."]
        #[must_use]
        #[inline(always)]
        pub const fn fcrcerr(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "desc FCRCERR."]
        #[inline(always)]
        pub const fn set_fcrcerr(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "desc FCRCDORCLR."]
        #[must_use]
        #[inline(always)]
        pub const fn fcrcdorclr(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "desc FCRCDORCLR."]
        #[inline(always)]
        pub const fn set_fcrcdorclr(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
    }
    impl Default for Fcrccr0 {
        #[inline(always)]
        fn default() -> Fcrccr0 {
            Fcrccr0(0)
        }
    }
    impl core::fmt::Debug for Fcrccr0 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Fcrccr0")
                .field("fcrcreq", &self.fcrcreq())
                .field("fcrcack", &self.fcrcack())
                .field("fcrcerr", &self.fcrcerr())
                .field("fcrcdorclr", &self.fcrcdorclr())
                .finish()
        }
    }
    #[doc = "desc FCRCCR1."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Fcrccr1(pub u32);
    impl Fcrccr1 {
        #[doc = "desc FCRCSR."]
        #[must_use]
        #[inline(always)]
        pub const fn fcrcsr(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[doc = "desc FCRCSR."]
        #[inline(always)]
        pub const fn set_fcrcsr(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
        }
        #[doc = "desc FCRCER."]
        #[must_use]
        #[inline(always)]
        pub const fn fcrcer(&self) -> u16 {
            let val = (self.0 >> 16usize) & 0xffff;
            val as u16
        }
        #[doc = "desc FCRCER."]
        #[inline(always)]
        pub const fn set_fcrcer(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 16usize)) | (((val as u32) & 0xffff) << 16usize);
        }
    }
    impl Default for Fcrccr1 {
        #[inline(always)]
        fn default() -> Fcrccr1 {
            Fcrccr1(0)
        }
    }
    impl core::fmt::Debug for Fcrccr1 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Fcrccr1")
                .field("fcrcsr", &self.fcrcsr())
                .field("fcrcer", &self.fcrcer())
                .finish()
        }
    }
    #[doc = "desc FMCCTL."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Fmcctl(pub u32);
    impl Fmcctl {
        #[doc = "desc PCROPERSREQ."]
        #[must_use]
        #[inline(always)]
        pub const fn pcropersreq(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "desc PCROPERSREQ."]
        #[inline(always)]
        pub const fn set_pcropersreq(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "desc PCROPERSACK."]
        #[must_use]
        #[inline(always)]
        pub const fn pcropersack(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "desc PCROPERSACK."]
        #[inline(always)]
        pub const fn set_pcropersack(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[doc = "desc PCROPERSERR."]
        #[must_use]
        #[inline(always)]
        pub const fn pcroperserr(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[doc = "desc PCROPERSERR."]
        #[inline(always)]
        pub const fn set_pcroperserr(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
    }
    impl Default for Fmcctl {
        #[inline(always)]
        fn default() -> Fmcctl {
            Fmcctl(0)
        }
    }
    impl core::fmt::Debug for Fmcctl {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Fmcctl")
                .field("pcropersreq", &self.pcropersreq())
                .field("pcropersack", &self.pcropersack())
                .field("pcroperserr", &self.pcroperserr())
                .finish()
        }
    }
}
