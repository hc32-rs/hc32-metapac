#![allow(clippy::missing_safety_doc)]
#![allow(clippy::identity_op)]
#![allow(clippy::unnecessary_cast)]
#![allow(clippy::erasing_op)]

#[doc = "desc EMB."]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Emb {
    ptr: *mut u8,
}
unsafe impl Send for Emb {}
unsafe impl Sync for Emb {}
impl Emb {
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
    #[doc = "desc PWMLV."]
    #[inline(always)]
    pub const fn pwmlv(self) -> crate::common::Reg<regs::Pwmlv, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x04usize) as _) }
    }
    #[doc = "desc SOE."]
    #[inline(always)]
    pub const fn soe(self) -> crate::common::Reg<regs::Soe, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x08usize) as _) }
    }
    #[doc = "desc STAT."]
    #[inline(always)]
    pub const fn stat(self) -> crate::common::Reg<regs::Stat, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0cusize) as _) }
    }
    #[doc = "desc STATCLR."]
    #[inline(always)]
    pub const fn statclr(self) -> crate::common::Reg<regs::Statclr, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x10usize) as _) }
    }
    #[doc = "desc INTEN."]
    #[inline(always)]
    pub const fn inten(self) -> crate::common::Reg<regs::Inten, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x14usize) as _) }
    }
}
pub mod regs {
    #[doc = "desc CTL."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ctl(pub u32);
    impl Ctl {
        #[doc = "desc PORTINEN."]
        #[must_use]
        #[inline(always)]
        pub const fn portinen(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "desc PORTINEN."]
        #[inline(always)]
        pub const fn set_portinen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "desc CMPEN1."]
        #[must_use]
        #[inline(always)]
        pub const fn cmpen1(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "desc CMPEN1."]
        #[inline(always)]
        pub const fn set_cmpen1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "desc CMPEN2."]
        #[must_use]
        #[inline(always)]
        pub const fn cmpen2(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "desc CMPEN2."]
        #[inline(always)]
        pub const fn set_cmpen2(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "desc CMPEN3."]
        #[must_use]
        #[inline(always)]
        pub const fn cmpen3(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "desc CMPEN3."]
        #[inline(always)]
        pub const fn set_cmpen3(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "desc OSCSTPEN."]
        #[must_use]
        #[inline(always)]
        pub const fn oscstpen(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "desc OSCSTPEN."]
        #[inline(always)]
        pub const fn set_oscstpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "desc PWMSEN0."]
        #[must_use]
        #[inline(always)]
        pub const fn pwmsen0(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "desc PWMSEN0."]
        #[inline(always)]
        pub const fn set_pwmsen0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "desc PWMSEN1."]
        #[must_use]
        #[inline(always)]
        pub const fn pwmsen1(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "desc PWMSEN1."]
        #[inline(always)]
        pub const fn set_pwmsen1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "desc PWMSEN2."]
        #[must_use]
        #[inline(always)]
        pub const fn pwmsen2(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "desc PWMSEN2."]
        #[inline(always)]
        pub const fn set_pwmsen2(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "desc NFSEL."]
        #[must_use]
        #[inline(always)]
        pub const fn nfsel(&self) -> u8 {
            let val = (self.0 >> 28usize) & 0x03;
            val as u8
        }
        #[doc = "desc NFSEL."]
        #[inline(always)]
        pub const fn set_nfsel(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 28usize)) | (((val as u32) & 0x03) << 28usize);
        }
        #[doc = "desc NFEN."]
        #[must_use]
        #[inline(always)]
        pub const fn nfen(&self) -> bool {
            let val = (self.0 >> 30usize) & 0x01;
            val != 0
        }
        #[doc = "desc NFEN."]
        #[inline(always)]
        pub const fn set_nfen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
        }
        #[doc = "desc INVSEL."]
        #[must_use]
        #[inline(always)]
        pub const fn invsel(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[doc = "desc INVSEL."]
        #[inline(always)]
        pub const fn set_invsel(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
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
                .field("portinen", &self.portinen())
                .field("cmpen1", &self.cmpen1())
                .field("cmpen2", &self.cmpen2())
                .field("cmpen3", &self.cmpen3())
                .field("oscstpen", &self.oscstpen())
                .field("pwmsen0", &self.pwmsen0())
                .field("pwmsen1", &self.pwmsen1())
                .field("pwmsen2", &self.pwmsen2())
                .field("nfsel", &self.nfsel())
                .field("nfen", &self.nfen())
                .field("invsel", &self.invsel())
                .finish()
        }
    }
    #[doc = "desc INTEN."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Inten(pub u32);
    impl Inten {
        #[doc = "desc PORTININTEN."]
        #[must_use]
        #[inline(always)]
        pub const fn portininten(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "desc PORTININTEN."]
        #[inline(always)]
        pub const fn set_portininten(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "desc PWMSINTEN."]
        #[must_use]
        #[inline(always)]
        pub const fn pwmsinten(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "desc PWMSINTEN."]
        #[inline(always)]
        pub const fn set_pwmsinten(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "desc CMPINTEN."]
        #[must_use]
        #[inline(always)]
        pub const fn cmpinten(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "desc CMPINTEN."]
        #[inline(always)]
        pub const fn set_cmpinten(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "desc OSINTEN."]
        #[must_use]
        #[inline(always)]
        pub const fn osinten(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "desc OSINTEN."]
        #[inline(always)]
        pub const fn set_osinten(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
    }
    impl Default for Inten {
        #[inline(always)]
        fn default() -> Inten {
            Inten(0)
        }
    }
    impl core::fmt::Debug for Inten {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Inten")
                .field("portininten", &self.portininten())
                .field("pwmsinten", &self.pwmsinten())
                .field("cmpinten", &self.cmpinten())
                .field("osinten", &self.osinten())
                .finish()
        }
    }
    #[doc = "desc PWMLV."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Pwmlv(pub u32);
    impl Pwmlv {
        #[doc = "desc PWMLV0."]
        #[must_use]
        #[inline(always)]
        pub const fn pwmlv0(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "desc PWMLV0."]
        #[inline(always)]
        pub const fn set_pwmlv0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "desc PWMLV1."]
        #[must_use]
        #[inline(always)]
        pub const fn pwmlv1(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "desc PWMLV1."]
        #[inline(always)]
        pub const fn set_pwmlv1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "desc PWMLV2."]
        #[must_use]
        #[inline(always)]
        pub const fn pwmlv2(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "desc PWMLV2."]
        #[inline(always)]
        pub const fn set_pwmlv2(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
    }
    impl Default for Pwmlv {
        #[inline(always)]
        fn default() -> Pwmlv {
            Pwmlv(0)
        }
    }
    impl core::fmt::Debug for Pwmlv {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Pwmlv")
                .field("pwmlv0", &self.pwmlv0())
                .field("pwmlv1", &self.pwmlv1())
                .field("pwmlv2", &self.pwmlv2())
                .finish()
        }
    }
    #[doc = "desc SOE."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Soe(pub u32);
    impl Soe {
        #[doc = "desc SOE."]
        #[must_use]
        #[inline(always)]
        pub const fn soe(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "desc SOE."]
        #[inline(always)]
        pub const fn set_soe(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
    }
    impl Default for Soe {
        #[inline(always)]
        fn default() -> Soe {
            Soe(0)
        }
    }
    impl core::fmt::Debug for Soe {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Soe").field("soe", &self.soe()).finish()
        }
    }
    #[doc = "desc STAT."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Stat(pub u32);
    impl Stat {
        #[doc = "desc PORTINF."]
        #[must_use]
        #[inline(always)]
        pub const fn portinf(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "desc PORTINF."]
        #[inline(always)]
        pub const fn set_portinf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "desc PWMSF."]
        #[must_use]
        #[inline(always)]
        pub const fn pwmsf(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "desc PWMSF."]
        #[inline(always)]
        pub const fn set_pwmsf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "desc CMPF."]
        #[must_use]
        #[inline(always)]
        pub const fn cmpf(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "desc CMPF."]
        #[inline(always)]
        pub const fn set_cmpf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "desc OSF."]
        #[must_use]
        #[inline(always)]
        pub const fn osf(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "desc OSF."]
        #[inline(always)]
        pub const fn set_osf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "desc PORTINST."]
        #[must_use]
        #[inline(always)]
        pub const fn portinst(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "desc PORTINST."]
        #[inline(always)]
        pub const fn set_portinst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "desc PWMST."]
        #[must_use]
        #[inline(always)]
        pub const fn pwmst(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "desc PWMST."]
        #[inline(always)]
        pub const fn set_pwmst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
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
                .field("portinf", &self.portinf())
                .field("pwmsf", &self.pwmsf())
                .field("cmpf", &self.cmpf())
                .field("osf", &self.osf())
                .field("portinst", &self.portinst())
                .field("pwmst", &self.pwmst())
                .finish()
        }
    }
    #[doc = "desc STATCLR."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Statclr(pub u32);
    impl Statclr {
        #[doc = "desc PORTINFCLR."]
        #[must_use]
        #[inline(always)]
        pub const fn portinfclr(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "desc PORTINFCLR."]
        #[inline(always)]
        pub const fn set_portinfclr(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "desc PWMSFCLR."]
        #[must_use]
        #[inline(always)]
        pub const fn pwmsfclr(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "desc PWMSFCLR."]
        #[inline(always)]
        pub const fn set_pwmsfclr(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "desc CMPFCLR."]
        #[must_use]
        #[inline(always)]
        pub const fn cmpfclr(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "desc CMPFCLR."]
        #[inline(always)]
        pub const fn set_cmpfclr(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "desc OSFCLR."]
        #[must_use]
        #[inline(always)]
        pub const fn osfclr(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "desc OSFCLR."]
        #[inline(always)]
        pub const fn set_osfclr(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
    }
    impl Default for Statclr {
        #[inline(always)]
        fn default() -> Statclr {
            Statclr(0)
        }
    }
    impl core::fmt::Debug for Statclr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Statclr")
                .field("portinfclr", &self.portinfclr())
                .field("pwmsfclr", &self.pwmsfclr())
                .field("cmpfclr", &self.cmpfclr())
                .field("osfclr", &self.osfclr())
                .finish()
        }
    }
}
