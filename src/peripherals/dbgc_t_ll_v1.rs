#![allow(clippy::missing_safety_doc)]
#![allow(clippy::identity_op)]
#![allow(clippy::unnecessary_cast)]
#![allow(clippy::erasing_op)]

#[doc = "desc DBGC_T."]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct DbgcT {
    ptr: *mut u8,
}
unsafe impl Send for DbgcT {}
unsafe impl Sync for DbgcT {}
impl DbgcT {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "desc AUTHID0."]
    #[inline(always)]
    pub const fn authid0(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0usize) as _) }
    }
    #[doc = "desc AUTHID1."]
    #[inline(always)]
    pub const fn authid1(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x04usize) as _) }
    }
    #[doc = "desc AUTHID2."]
    #[inline(always)]
    pub const fn authid2(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x08usize) as _) }
    }
    #[doc = "desc RESV0."]
    #[inline(always)]
    pub const fn resv0(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0cusize) as _) }
    }
    #[doc = "desc MCUSTAT."]
    #[inline(always)]
    pub const fn mcustat(self) -> crate::common::Reg<regs::Mcustat, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x10usize) as _) }
    }
    #[doc = "desc MCUCTL."]
    #[inline(always)]
    pub const fn mcuctl(self) -> crate::common::Reg<regs::Mcuctl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x14usize) as _) }
    }
    #[doc = "desc FMCCTL."]
    #[inline(always)]
    pub const fn fmcctl(self) -> crate::common::Reg<regs::Fmcctl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x18usize) as _) }
    }
}
pub mod regs {
    #[doc = "desc FMCCTL."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Fmcctl(pub u32);
    impl Fmcctl {
        #[doc = "desc ERASEREQ."]
        #[must_use]
        #[inline(always)]
        pub const fn erasereq(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "desc ERASEREQ."]
        #[inline(always)]
        pub const fn set_erasereq(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "desc ERASEACK."]
        #[must_use]
        #[inline(always)]
        pub const fn eraseack(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "desc ERASEACK."]
        #[inline(always)]
        pub const fn set_eraseack(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "desc ERASEERR."]
        #[must_use]
        #[inline(always)]
        pub const fn eraseerr(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "desc ERASEERR."]
        #[inline(always)]
        pub const fn set_eraseerr(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
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
                .field("erasereq", &self.erasereq())
                .field("eraseack", &self.eraseack())
                .field("eraseerr", &self.eraseerr())
                .finish()
        }
    }
    #[doc = "desc MCUCTL."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Mcuctl(pub u32);
    impl Mcuctl {
        #[doc = "desc EDBGRQ."]
        #[must_use]
        #[inline(always)]
        pub const fn edbgrq(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "desc EDBGRQ."]
        #[inline(always)]
        pub const fn set_edbgrq(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "desc RESTART."]
        #[must_use]
        #[inline(always)]
        pub const fn restart(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "desc RESTART."]
        #[inline(always)]
        pub const fn set_restart(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "desc DIRQ."]
        #[must_use]
        #[inline(always)]
        pub const fn dirq(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "desc DIRQ."]
        #[inline(always)]
        pub const fn set_dirq(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
    }
    impl Default for Mcuctl {
        #[inline(always)]
        fn default() -> Mcuctl {
            Mcuctl(0)
        }
    }
    impl core::fmt::Debug for Mcuctl {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Mcuctl")
                .field("edbgrq", &self.edbgrq())
                .field("restart", &self.restart())
                .field("dirq", &self.dirq())
                .finish()
        }
    }
    #[doc = "desc MCUSTAT."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Mcustat(pub u32);
    impl Mcustat {
        #[doc = "desc AUTH."]
        #[must_use]
        #[inline(always)]
        pub const fn auth(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "desc AUTH."]
        #[inline(always)]
        pub const fn set_auth(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "desc REMVLOCK."]
        #[must_use]
        #[inline(always)]
        pub const fn remvlock(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "desc REMVLOCK."]
        #[inline(always)]
        pub const fn set_remvlock(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "desc SAFTYLOCK1."]
        #[must_use]
        #[inline(always)]
        pub const fn saftylock1(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "desc SAFTYLOCK1."]
        #[inline(always)]
        pub const fn set_saftylock1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "desc SAFTYLOCK2."]
        #[must_use]
        #[inline(always)]
        pub const fn saftylock2(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "desc SAFTYLOCK2."]
        #[inline(always)]
        pub const fn set_saftylock2(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "desc MCUSTAT0."]
        #[must_use]
        #[inline(always)]
        pub const fn mcustat0(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "desc MCUSTAT0."]
        #[inline(always)]
        pub const fn set_mcustat0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "desc MCUSTAT1."]
        #[must_use]
        #[inline(always)]
        pub const fn mcustat1(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "desc MCUSTAT1."]
        #[inline(always)]
        pub const fn set_mcustat1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
    }
    impl Default for Mcustat {
        #[inline(always)]
        fn default() -> Mcustat {
            Mcustat(0)
        }
    }
    impl core::fmt::Debug for Mcustat {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Mcustat")
                .field("auth", &self.auth())
                .field("remvlock", &self.remvlock())
                .field("saftylock1", &self.saftylock1())
                .field("saftylock2", &self.saftylock2())
                .field("mcustat0", &self.mcustat0())
                .field("mcustat1", &self.mcustat1())
                .finish()
        }
    }
}
