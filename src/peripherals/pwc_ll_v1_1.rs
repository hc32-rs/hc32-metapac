#![allow(clippy::missing_safety_doc)]
#![allow(clippy::identity_op)]
#![allow(clippy::unnecessary_cast)]
#![allow(clippy::erasing_op)]

#[doc = "desc PWC."]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pwc {
    ptr: *mut u8,
}
unsafe impl Send for Pwc {}
unsafe impl Sync for Pwc {}
impl Pwc {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "desc STPMCR."]
    #[inline(always)]
    pub const fn stpmcr(self) -> crate::common::Reg<regs::Stpmcr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0usize) as _) }
    }
    #[doc = "desc PWRC."]
    #[inline(always)]
    pub const fn pwrc(self) -> crate::common::Reg<regs::Pwrc, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x04usize) as _) }
    }
    #[doc = "desc RAMCR."]
    #[inline(always)]
    pub const fn ramcr(self) -> crate::common::Reg<regs::Ramcr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0cusize) as _) }
    }
    #[doc = "desc LVDCSR."]
    #[inline(always)]
    pub const fn lvdcsr(self) -> crate::common::Reg<regs::Lvdcsr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x10usize) as _) }
    }
    #[doc = "desc FPRC."]
    #[inline(always)]
    pub const fn fprc(self) -> crate::common::Reg<regs::Fprc, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x20usize) as _) }
    }
    #[doc = "desc DBGC."]
    #[inline(always)]
    pub const fn dbgc(self) -> crate::common::Reg<regs::Dbgc, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x40usize) as _) }
    }
}
pub mod regs {
    #[doc = "desc DBGC."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Dbgc(pub u8);
    impl Dbgc {
        #[doc = "desc DBGEN."]
        #[must_use]
        #[inline(always)]
        pub const fn dbgen(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "desc DBGEN."]
        #[inline(always)]
        pub const fn set_dbgen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u8) & 0x01) << 0usize);
        }
    }
    impl Default for Dbgc {
        #[inline(always)]
        fn default() -> Dbgc {
            Dbgc(0)
        }
    }
    impl core::fmt::Debug for Dbgc {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Dbgc")
                .field("dbgen", &self.dbgen())
                .finish()
        }
    }
    #[doc = "desc FPRC."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Fprc(pub u16);
    impl Fprc {
        #[doc = "desc CKRWE."]
        #[must_use]
        #[inline(always)]
        pub const fn ckrwe(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "desc CKRWE."]
        #[inline(always)]
        pub const fn set_ckrwe(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u16) & 0x01) << 0usize);
        }
        #[doc = "desc PWRWE."]
        #[must_use]
        #[inline(always)]
        pub const fn pwrwe(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "desc PWRWE."]
        #[inline(always)]
        pub const fn set_pwrwe(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u16) & 0x01) << 1usize);
        }
        #[doc = "desc FPRCB2."]
        #[must_use]
        #[inline(always)]
        pub const fn fprcb2(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "desc FPRCB2."]
        #[inline(always)]
        pub const fn set_fprcb2(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u16) & 0x01) << 2usize);
        }
        #[doc = "desc LVRWE."]
        #[must_use]
        #[inline(always)]
        pub const fn lvrwe(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "desc LVRWE."]
        #[inline(always)]
        pub const fn set_lvrwe(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u16) & 0x01) << 3usize);
        }
        #[doc = "desc FPRCB4."]
        #[must_use]
        #[inline(always)]
        pub const fn fprcb4(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "desc FPRCB4."]
        #[inline(always)]
        pub const fn set_fprcb4(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u16) & 0x01) << 4usize);
        }
        #[doc = "desc FPRCB5."]
        #[must_use]
        #[inline(always)]
        pub const fn fprcb5(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "desc FPRCB5."]
        #[inline(always)]
        pub const fn set_fprcb5(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u16) & 0x01) << 5usize);
        }
        #[doc = "desc FPRCB6."]
        #[must_use]
        #[inline(always)]
        pub const fn fprcb6(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "desc FPRCB6."]
        #[inline(always)]
        pub const fn set_fprcb6(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u16) & 0x01) << 6usize);
        }
        #[doc = "desc FPRCB7."]
        #[must_use]
        #[inline(always)]
        pub const fn fprcb7(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "desc FPRCB7."]
        #[inline(always)]
        pub const fn set_fprcb7(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u16) & 0x01) << 7usize);
        }
        #[doc = "desc FPRCWE."]
        #[must_use]
        #[inline(always)]
        pub const fn fprcwe(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0xff;
            val as u8
        }
        #[doc = "desc FPRCWE."]
        #[inline(always)]
        pub const fn set_fprcwe(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 8usize)) | (((val as u16) & 0xff) << 8usize);
        }
    }
    impl Default for Fprc {
        #[inline(always)]
        fn default() -> Fprc {
            Fprc(0)
        }
    }
    impl core::fmt::Debug for Fprc {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Fprc")
                .field("ckrwe", &self.ckrwe())
                .field("pwrwe", &self.pwrwe())
                .field("fprcb2", &self.fprcb2())
                .field("lvrwe", &self.lvrwe())
                .field("fprcb4", &self.fprcb4())
                .field("fprcb5", &self.fprcb5())
                .field("fprcb6", &self.fprcb6())
                .field("fprcb7", &self.fprcb7())
                .field("fprcwe", &self.fprcwe())
                .finish()
        }
    }
    #[doc = "desc LVDCSR."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Lvdcsr(pub u8);
    impl Lvdcsr {
        #[doc = "desc EXVCCINEN."]
        #[must_use]
        #[inline(always)]
        pub const fn exvccinen(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "desc EXVCCINEN."]
        #[inline(always)]
        pub const fn set_exvccinen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u8) & 0x01) << 0usize);
        }
        #[doc = "desc LVIF."]
        #[must_use]
        #[inline(always)]
        pub const fn lvif(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "desc LVIF."]
        #[inline(always)]
        pub const fn set_lvif(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u8) & 0x01) << 3usize);
        }
        #[doc = "desc DETF."]
        #[must_use]
        #[inline(always)]
        pub const fn detf(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "desc DETF."]
        #[inline(always)]
        pub const fn set_detf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u8) & 0x01) << 4usize);
        }
        #[doc = "desc CMPOE."]
        #[must_use]
        #[inline(always)]
        pub const fn cmpoe(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "desc CMPOE."]
        #[inline(always)]
        pub const fn set_cmpoe(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u8) & 0x01) << 7usize);
        }
    }
    impl Default for Lvdcsr {
        #[inline(always)]
        fn default() -> Lvdcsr {
            Lvdcsr(0)
        }
    }
    impl core::fmt::Debug for Lvdcsr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Lvdcsr")
                .field("exvccinen", &self.exvccinen())
                .field("lvif", &self.lvif())
                .field("detf", &self.detf())
                .field("cmpoe", &self.cmpoe())
                .finish()
        }
    }
    #[doc = "desc PWRC."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Pwrc(pub u8);
    impl Pwrc {
        #[doc = "desc PWDRV."]
        #[must_use]
        #[inline(always)]
        pub const fn pwdrv(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x07;
            val as u8
        }
        #[doc = "desc PWDRV."]
        #[inline(always)]
        pub const fn set_pwdrv(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 0usize)) | (((val as u8) & 0x07) << 0usize);
        }
        #[doc = "desc PWMONSEL."]
        #[must_use]
        #[inline(always)]
        pub const fn pwmonsel(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "desc PWMONSEL."]
        #[inline(always)]
        pub const fn set_pwmonsel(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u8) & 0x01) << 4usize);
        }
        #[doc = "desc PWMONE."]
        #[must_use]
        #[inline(always)]
        pub const fn pwmone(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "desc PWMONE."]
        #[inline(always)]
        pub const fn set_pwmone(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u8) & 0x01) << 5usize);
        }
        #[doc = "desc LPDADJ."]
        #[must_use]
        #[inline(always)]
        pub const fn lpdadj(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "desc LPDADJ."]
        #[inline(always)]
        pub const fn set_lpdadj(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u8) & 0x01) << 6usize);
        }
    }
    impl Default for Pwrc {
        #[inline(always)]
        fn default() -> Pwrc {
            Pwrc(0)
        }
    }
    impl core::fmt::Debug for Pwrc {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Pwrc")
                .field("pwdrv", &self.pwdrv())
                .field("pwmonsel", &self.pwmonsel())
                .field("pwmone", &self.pwmone())
                .field("lpdadj", &self.lpdadj())
                .finish()
        }
    }
    #[doc = "desc RAMCR."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ramcr(pub u8);
    impl Ramcr {
        #[doc = "desc RPRTA."]
        #[must_use]
        #[inline(always)]
        pub const fn rprta(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x03;
            val as u8
        }
        #[doc = "desc RPRTA."]
        #[inline(always)]
        pub const fn set_rprta(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 0usize)) | (((val as u8) & 0x03) << 0usize);
        }
        #[doc = "desc RPERDIS."]
        #[must_use]
        #[inline(always)]
        pub const fn rperdis(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "desc RPERDIS."]
        #[inline(always)]
        pub const fn set_rperdis(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u8) & 0x01) << 4usize);
        }
        #[doc = "desc RPEF."]
        #[must_use]
        #[inline(always)]
        pub const fn rpef(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "desc RPEF."]
        #[inline(always)]
        pub const fn set_rpef(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u8) & 0x01) << 5usize);
        }
    }
    impl Default for Ramcr {
        #[inline(always)]
        fn default() -> Ramcr {
            Ramcr(0)
        }
    }
    impl core::fmt::Debug for Ramcr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Ramcr")
                .field("rprta", &self.rprta())
                .field("rperdis", &self.rperdis())
                .field("rpef", &self.rpef())
                .finish()
        }
    }
    #[doc = "desc STPMCR."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Stpmcr(pub u8);
    impl Stpmcr {
        #[doc = "desc CKSHRC."]
        #[must_use]
        #[inline(always)]
        pub const fn ckshrc(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "desc CKSHRC."]
        #[inline(always)]
        pub const fn set_ckshrc(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u8) & 0x01) << 1usize);
        }
        #[doc = "desc HAPORDIS."]
        #[must_use]
        #[inline(always)]
        pub const fn hapordis(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "desc HAPORDIS."]
        #[inline(always)]
        pub const fn set_hapordis(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u8) & 0x01) << 3usize);
        }
        #[doc = "desc STOP."]
        #[must_use]
        #[inline(always)]
        pub const fn stop(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "desc STOP."]
        #[inline(always)]
        pub const fn set_stop(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u8) & 0x01) << 7usize);
        }
    }
    impl Default for Stpmcr {
        #[inline(always)]
        fn default() -> Stpmcr {
            Stpmcr(0)
        }
    }
    impl core::fmt::Debug for Stpmcr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Stpmcr")
                .field("ckshrc", &self.ckshrc())
                .field("hapordis", &self.hapordis())
                .field("stop", &self.stop())
                .finish()
        }
    }
}
