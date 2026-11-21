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
    #[doc = "desc CTL1."]
    #[inline(always)]
    pub const fn ctl1(self) -> crate::common::Reg<regs::Ctl1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0usize) as _) }
    }
    #[doc = "desc CTL2."]
    #[inline(always)]
    pub const fn ctl2(self) -> crate::common::Reg<regs::Ctl2, crate::common::RW> {
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
    #[doc = "desc RLSSEL."]
    #[inline(always)]
    pub const fn rlssel(self) -> crate::common::Reg<regs::Rlssel, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x18usize) as _) }
    }
}
pub mod regs {
    #[doc = "desc CTL1."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ctl1(pub u32);
    impl Ctl1 {
        #[doc = "desc CMPEN1."]
        #[must_use]
        #[inline(always)]
        pub const fn cmpen1(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "desc CMPEN1."]
        #[inline(always)]
        pub const fn set_cmpen1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "desc CMPEN2."]
        #[must_use]
        #[inline(always)]
        pub const fn cmpen2(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "desc CMPEN2."]
        #[inline(always)]
        pub const fn set_cmpen2(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "desc CMPEN3."]
        #[must_use]
        #[inline(always)]
        pub const fn cmpen3(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "desc CMPEN3."]
        #[inline(always)]
        pub const fn set_cmpen3(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "desc CMPEN4."]
        #[must_use]
        #[inline(always)]
        pub const fn cmpen4(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "desc CMPEN4."]
        #[inline(always)]
        pub const fn set_cmpen4(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "desc SYSEN."]
        #[must_use]
        #[inline(always)]
        pub const fn sysen(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "desc SYSEN."]
        #[inline(always)]
        pub const fn set_sysen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "desc PWMSEN0."]
        #[must_use]
        #[inline(always)]
        pub const fn pwmsen0(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "desc PWMSEN0."]
        #[inline(always)]
        pub const fn set_pwmsen0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "desc PWMSEN1."]
        #[must_use]
        #[inline(always)]
        pub const fn pwmsen1(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "desc PWMSEN1."]
        #[inline(always)]
        pub const fn set_pwmsen1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "desc PWMSEN2."]
        #[must_use]
        #[inline(always)]
        pub const fn pwmsen2(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "desc PWMSEN2."]
        #[inline(always)]
        pub const fn set_pwmsen2(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "desc PWMSEN3."]
        #[must_use]
        #[inline(always)]
        pub const fn pwmsen3(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "desc PWMSEN3."]
        #[inline(always)]
        pub const fn set_pwmsen3(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "desc PORTINEN1."]
        #[must_use]
        #[inline(always)]
        pub const fn portinen1(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "desc PORTINEN1."]
        #[inline(always)]
        pub const fn set_portinen1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "desc PORTINEN2."]
        #[must_use]
        #[inline(always)]
        pub const fn portinen2(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "desc PORTINEN2."]
        #[inline(always)]
        pub const fn set_portinen2(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[doc = "desc PORTINEN3."]
        #[must_use]
        #[inline(always)]
        pub const fn portinen3(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[doc = "desc PORTINEN3."]
        #[inline(always)]
        pub const fn set_portinen3(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
        #[doc = "desc PORTINEN4."]
        #[must_use]
        #[inline(always)]
        pub const fn portinen4(&self) -> bool {
            let val = (self.0 >> 19usize) & 0x01;
            val != 0
        }
        #[doc = "desc PORTINEN4."]
        #[inline(always)]
        pub const fn set_portinen4(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
        }
        #[doc = "desc INVSEL1."]
        #[must_use]
        #[inline(always)]
        pub const fn invsel1(&self) -> bool {
            let val = (self.0 >> 22usize) & 0x01;
            val != 0
        }
        #[doc = "desc INVSEL1."]
        #[inline(always)]
        pub const fn set_invsel1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
        }
        #[doc = "desc INVSEL2."]
        #[must_use]
        #[inline(always)]
        pub const fn invsel2(&self) -> bool {
            let val = (self.0 >> 23usize) & 0x01;
            val != 0
        }
        #[doc = "desc INVSEL2."]
        #[inline(always)]
        pub const fn set_invsel2(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
        }
        #[doc = "desc INVSEL3."]
        #[must_use]
        #[inline(always)]
        pub const fn invsel3(&self) -> bool {
            let val = (self.0 >> 24usize) & 0x01;
            val != 0
        }
        #[doc = "desc INVSEL3."]
        #[inline(always)]
        pub const fn set_invsel3(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
        }
        #[doc = "desc INVSEL4."]
        #[must_use]
        #[inline(always)]
        pub const fn invsel4(&self) -> bool {
            let val = (self.0 >> 25usize) & 0x01;
            val != 0
        }
        #[doc = "desc INVSEL4."]
        #[inline(always)]
        pub const fn set_invsel4(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
        }
        #[doc = "desc OSCSTPEN."]
        #[must_use]
        #[inline(always)]
        pub const fn oscstpen(&self) -> bool {
            let val = (self.0 >> 27usize) & 0x01;
            val != 0
        }
        #[doc = "desc OSCSTPEN."]
        #[inline(always)]
        pub const fn set_oscstpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
        }
        #[doc = "desc SRAMECCERREN."]
        #[must_use]
        #[inline(always)]
        pub const fn srameccerren(&self) -> bool {
            let val = (self.0 >> 28usize) & 0x01;
            val != 0
        }
        #[doc = "desc SRAMECCERREN."]
        #[inline(always)]
        pub const fn set_srameccerren(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
        }
        #[doc = "desc SRAMPYERREN."]
        #[must_use]
        #[inline(always)]
        pub const fn srampyerren(&self) -> bool {
            let val = (self.0 >> 29usize) & 0x01;
            val != 0
        }
        #[doc = "desc SRAMPYERREN."]
        #[inline(always)]
        pub const fn set_srampyerren(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
        }
        #[doc = "desc LOCKUPEN."]
        #[must_use]
        #[inline(always)]
        pub const fn lockupen(&self) -> bool {
            let val = (self.0 >> 30usize) & 0x01;
            val != 0
        }
        #[doc = "desc LOCKUPEN."]
        #[inline(always)]
        pub const fn set_lockupen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
        }
        #[doc = "desc PVDEN."]
        #[must_use]
        #[inline(always)]
        pub const fn pvden(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[doc = "desc PVDEN."]
        #[inline(always)]
        pub const fn set_pvden(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for Ctl1 {
        #[inline(always)]
        fn default() -> Ctl1 {
            Ctl1(0)
        }
    }
    impl core::fmt::Debug for Ctl1 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Ctl1")
                .field("cmpen1", &self.cmpen1())
                .field("cmpen2", &self.cmpen2())
                .field("cmpen3", &self.cmpen3())
                .field("cmpen4", &self.cmpen4())
                .field("sysen", &self.sysen())
                .field("pwmsen0", &self.pwmsen0())
                .field("pwmsen1", &self.pwmsen1())
                .field("pwmsen2", &self.pwmsen2())
                .field("pwmsen3", &self.pwmsen3())
                .field("portinen1", &self.portinen1())
                .field("portinen2", &self.portinen2())
                .field("portinen3", &self.portinen3())
                .field("portinen4", &self.portinen4())
                .field("invsel1", &self.invsel1())
                .field("invsel2", &self.invsel2())
                .field("invsel3", &self.invsel3())
                .field("invsel4", &self.invsel4())
                .field("oscstpen", &self.oscstpen())
                .field("srameccerren", &self.srameccerren())
                .field("srampyerren", &self.srampyerren())
                .field("lockupen", &self.lockupen())
                .field("pvden", &self.pvden())
                .finish()
        }
    }
    #[doc = "desc CTL2."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ctl2(pub u32);
    impl Ctl2 {
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
        #[doc = "desc PWMLV3."]
        #[must_use]
        #[inline(always)]
        pub const fn pwmlv3(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "desc PWMLV3."]
        #[inline(always)]
        pub const fn set_pwmlv3(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "desc NFSEL1."]
        #[must_use]
        #[inline(always)]
        pub const fn nfsel1(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x03;
            val as u8
        }
        #[doc = "desc NFSEL1."]
        #[inline(always)]
        pub const fn set_nfsel1(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 16usize)) | (((val as u32) & 0x03) << 16usize);
        }
        #[doc = "desc NFEN1."]
        #[must_use]
        #[inline(always)]
        pub const fn nfen1(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[doc = "desc NFEN1."]
        #[inline(always)]
        pub const fn set_nfen1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
        #[doc = "desc NFSEL2."]
        #[must_use]
        #[inline(always)]
        pub const fn nfsel2(&self) -> u8 {
            let val = (self.0 >> 19usize) & 0x03;
            val as u8
        }
        #[doc = "desc NFSEL2."]
        #[inline(always)]
        pub const fn set_nfsel2(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 19usize)) | (((val as u32) & 0x03) << 19usize);
        }
        #[doc = "desc NFEN2."]
        #[must_use]
        #[inline(always)]
        pub const fn nfen2(&self) -> bool {
            let val = (self.0 >> 21usize) & 0x01;
            val != 0
        }
        #[doc = "desc NFEN2."]
        #[inline(always)]
        pub const fn set_nfen2(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
        }
        #[doc = "desc NFSEL3."]
        #[must_use]
        #[inline(always)]
        pub const fn nfsel3(&self) -> u8 {
            let val = (self.0 >> 22usize) & 0x03;
            val as u8
        }
        #[doc = "desc NFSEL3."]
        #[inline(always)]
        pub const fn set_nfsel3(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 22usize)) | (((val as u32) & 0x03) << 22usize);
        }
        #[doc = "desc NFEN3."]
        #[must_use]
        #[inline(always)]
        pub const fn nfen3(&self) -> bool {
            let val = (self.0 >> 24usize) & 0x01;
            val != 0
        }
        #[doc = "desc NFEN3."]
        #[inline(always)]
        pub const fn set_nfen3(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
        }
        #[doc = "desc NFSEL4."]
        #[must_use]
        #[inline(always)]
        pub const fn nfsel4(&self) -> u8 {
            let val = (self.0 >> 25usize) & 0x03;
            val as u8
        }
        #[doc = "desc NFSEL4."]
        #[inline(always)]
        pub const fn set_nfsel4(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 25usize)) | (((val as u32) & 0x03) << 25usize);
        }
        #[doc = "desc NFEN4."]
        #[must_use]
        #[inline(always)]
        pub const fn nfen4(&self) -> bool {
            let val = (self.0 >> 27usize) & 0x01;
            val != 0
        }
        #[doc = "desc NFEN4."]
        #[inline(always)]
        pub const fn set_nfen4(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
        }
    }
    impl Default for Ctl2 {
        #[inline(always)]
        fn default() -> Ctl2 {
            Ctl2(0)
        }
    }
    impl core::fmt::Debug for Ctl2 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Ctl2")
                .field("pwmlv0", &self.pwmlv0())
                .field("pwmlv1", &self.pwmlv1())
                .field("pwmlv2", &self.pwmlv2())
                .field("pwmlv3", &self.pwmlv3())
                .field("nfsel1", &self.nfsel1())
                .field("nfen1", &self.nfen1())
                .field("nfsel2", &self.nfsel2())
                .field("nfen2", &self.nfen2())
                .field("nfsel3", &self.nfsel3())
                .field("nfen3", &self.nfen3())
                .field("nfsel4", &self.nfsel4())
                .field("nfen4", &self.nfen4())
                .finish()
        }
    }
    #[doc = "desc INTEN."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Inten(pub u32);
    impl Inten {
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
        #[doc = "desc SYSINTEN."]
        #[must_use]
        #[inline(always)]
        pub const fn sysinten(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "desc SYSINTEN."]
        #[inline(always)]
        pub const fn set_sysinten(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "desc PORTININTEN1."]
        #[must_use]
        #[inline(always)]
        pub const fn portininten1(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "desc PORTININTEN1."]
        #[inline(always)]
        pub const fn set_portininten1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "desc PORTININTEN2."]
        #[must_use]
        #[inline(always)]
        pub const fn portininten2(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "desc PORTININTEN2."]
        #[inline(always)]
        pub const fn set_portininten2(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "desc PORTININTEN3."]
        #[must_use]
        #[inline(always)]
        pub const fn portininten3(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "desc PORTININTEN3."]
        #[inline(always)]
        pub const fn set_portininten3(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[doc = "desc PORTININTEN4."]
        #[must_use]
        #[inline(always)]
        pub const fn portininten4(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "desc PORTININTEN4."]
        #[inline(always)]
        pub const fn set_portininten4(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
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
                .field("pwmsinten", &self.pwmsinten())
                .field("cmpinten", &self.cmpinten())
                .field("sysinten", &self.sysinten())
                .field("portininten1", &self.portininten1())
                .field("portininten2", &self.portininten2())
                .field("portininten3", &self.portininten3())
                .field("portininten4", &self.portininten4())
                .finish()
        }
    }
    #[doc = "desc RLSSEL."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Rlssel(pub u32);
    impl Rlssel {
        #[doc = "desc PWMRSEL."]
        #[must_use]
        #[inline(always)]
        pub const fn pwmrsel(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "desc PWMRSEL."]
        #[inline(always)]
        pub const fn set_pwmrsel(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "desc CMPRSEL."]
        #[must_use]
        #[inline(always)]
        pub const fn cmprsel(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "desc CMPRSEL."]
        #[inline(always)]
        pub const fn set_cmprsel(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "desc SYSRSEL."]
        #[must_use]
        #[inline(always)]
        pub const fn sysrsel(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "desc SYSRSEL."]
        #[inline(always)]
        pub const fn set_sysrsel(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "desc PORTINRSEL1."]
        #[must_use]
        #[inline(always)]
        pub const fn portinrsel1(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "desc PORTINRSEL1."]
        #[inline(always)]
        pub const fn set_portinrsel1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "desc PORTINRSEL2."]
        #[must_use]
        #[inline(always)]
        pub const fn portinrsel2(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "desc PORTINRSEL2."]
        #[inline(always)]
        pub const fn set_portinrsel2(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "desc PORTINRSEL3."]
        #[must_use]
        #[inline(always)]
        pub const fn portinrsel3(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "desc PORTINRSEL3."]
        #[inline(always)]
        pub const fn set_portinrsel3(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[doc = "desc PORTINRSEL4."]
        #[must_use]
        #[inline(always)]
        pub const fn portinrsel4(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "desc PORTINRSEL4."]
        #[inline(always)]
        pub const fn set_portinrsel4(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
    }
    impl Default for Rlssel {
        #[inline(always)]
        fn default() -> Rlssel {
            Rlssel(0)
        }
    }
    impl core::fmt::Debug for Rlssel {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Rlssel")
                .field("pwmrsel", &self.pwmrsel())
                .field("cmprsel", &self.cmprsel())
                .field("sysrsel", &self.sysrsel())
                .field("portinrsel1", &self.portinrsel1())
                .field("portinrsel2", &self.portinrsel2())
                .field("portinrsel3", &self.portinrsel3())
                .field("portinrsel4", &self.portinrsel4())
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
        #[doc = "desc SYSF."]
        #[must_use]
        #[inline(always)]
        pub const fn sysf(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "desc SYSF."]
        #[inline(always)]
        pub const fn set_sysf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
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
        #[doc = "desc CMPST."]
        #[must_use]
        #[inline(always)]
        pub const fn cmpst(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "desc CMPST."]
        #[inline(always)]
        pub const fn set_cmpst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "desc SYSST."]
        #[must_use]
        #[inline(always)]
        pub const fn sysst(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "desc SYSST."]
        #[inline(always)]
        pub const fn set_sysst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "desc PORTINF1."]
        #[must_use]
        #[inline(always)]
        pub const fn portinf1(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "desc PORTINF1."]
        #[inline(always)]
        pub const fn set_portinf1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "desc PORTINF2."]
        #[must_use]
        #[inline(always)]
        pub const fn portinf2(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "desc PORTINF2."]
        #[inline(always)]
        pub const fn set_portinf2(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "desc PORTINF3."]
        #[must_use]
        #[inline(always)]
        pub const fn portinf3(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "desc PORTINF3."]
        #[inline(always)]
        pub const fn set_portinf3(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[doc = "desc PORTINF4."]
        #[must_use]
        #[inline(always)]
        pub const fn portinf4(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "desc PORTINF4."]
        #[inline(always)]
        pub const fn set_portinf4(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[doc = "desc PORTINST1."]
        #[must_use]
        #[inline(always)]
        pub const fn portinst1(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "desc PORTINST1."]
        #[inline(always)]
        pub const fn set_portinst1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[doc = "desc PORTINST2."]
        #[must_use]
        #[inline(always)]
        pub const fn portinst2(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "desc PORTINST2."]
        #[inline(always)]
        pub const fn set_portinst2(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
        #[doc = "desc PORTINST3."]
        #[must_use]
        #[inline(always)]
        pub const fn portinst3(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "desc PORTINST3."]
        #[inline(always)]
        pub const fn set_portinst3(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "desc PORTINST4."]
        #[must_use]
        #[inline(always)]
        pub const fn portinst4(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "desc PORTINST4."]
        #[inline(always)]
        pub const fn set_portinst4(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
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
                .field("pwmsf", &self.pwmsf())
                .field("cmpf", &self.cmpf())
                .field("sysf", &self.sysf())
                .field("pwmst", &self.pwmst())
                .field("cmpst", &self.cmpst())
                .field("sysst", &self.sysst())
                .field("portinf1", &self.portinf1())
                .field("portinf2", &self.portinf2())
                .field("portinf3", &self.portinf3())
                .field("portinf4", &self.portinf4())
                .field("portinst1", &self.portinst1())
                .field("portinst2", &self.portinst2())
                .field("portinst3", &self.portinst3())
                .field("portinst4", &self.portinst4())
                .finish()
        }
    }
    #[doc = "desc STATCLR."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Statclr(pub u32);
    impl Statclr {
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
        #[doc = "desc SYSFCLR."]
        #[must_use]
        #[inline(always)]
        pub const fn sysfclr(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "desc SYSFCLR."]
        #[inline(always)]
        pub const fn set_sysfclr(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "desc PORTINFCLR1."]
        #[must_use]
        #[inline(always)]
        pub const fn portinfclr1(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "desc PORTINFCLR1."]
        #[inline(always)]
        pub const fn set_portinfclr1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "desc PORTINFCLR2."]
        #[must_use]
        #[inline(always)]
        pub const fn portinfclr2(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "desc PORTINFCLR2."]
        #[inline(always)]
        pub const fn set_portinfclr2(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "desc PORTINFCLR3."]
        #[must_use]
        #[inline(always)]
        pub const fn portinfclr3(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "desc PORTINFCLR3."]
        #[inline(always)]
        pub const fn set_portinfclr3(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[doc = "desc PORTINFCLR4."]
        #[must_use]
        #[inline(always)]
        pub const fn portinfclr4(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "desc PORTINFCLR4."]
        #[inline(always)]
        pub const fn set_portinfclr4(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
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
                .field("pwmsfclr", &self.pwmsfclr())
                .field("cmpfclr", &self.cmpfclr())
                .field("sysfclr", &self.sysfclr())
                .field("portinfclr1", &self.portinfclr1())
                .field("portinfclr2", &self.portinfclr2())
                .field("portinfclr3", &self.portinfclr3())
                .field("portinfclr4", &self.portinfclr4())
                .finish()
        }
    }
}
