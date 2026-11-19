#![allow(clippy::missing_safety_doc)]
#![allow(clippy::identity_op)]
#![allow(clippy::unnecessary_cast)]
#![allow(clippy::erasing_op)]

#[doc = "desc TMR2."]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Tmr2 {
    ptr: *mut u8,
}
unsafe impl Send for Tmr2 {}
unsafe impl Sync for Tmr2 {}
impl Tmr2 {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "desc CNTAR."]
    #[inline(always)]
    pub const fn cntar(self) -> crate::common::Reg<regs::Cntar, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0usize) as _) }
    }
    #[doc = "desc CNTBR."]
    #[inline(always)]
    pub const fn cntbr(self) -> crate::common::Reg<regs::Cntbr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x04usize) as _) }
    }
    #[doc = "desc CMPAR."]
    #[inline(always)]
    pub const fn cmpar(self) -> crate::common::Reg<regs::Cmpar, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x08usize) as _) }
    }
    #[doc = "desc CMPBR."]
    #[inline(always)]
    pub const fn cmpbr(self) -> crate::common::Reg<regs::Cmpbr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0cusize) as _) }
    }
    #[doc = "desc BCONR."]
    #[inline(always)]
    pub const fn bconr(self) -> crate::common::Reg<regs::Bconr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x10usize) as _) }
    }
    #[doc = "desc ICONR."]
    #[inline(always)]
    pub const fn iconr(self) -> crate::common::Reg<regs::Iconr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x14usize) as _) }
    }
    #[doc = "desc PCONR."]
    #[inline(always)]
    pub const fn pconr(self) -> crate::common::Reg<regs::Pconr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x18usize) as _) }
    }
    #[doc = "desc HCONR."]
    #[inline(always)]
    pub const fn hconr(self) -> crate::common::Reg<regs::Hconr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1cusize) as _) }
    }
    #[doc = "desc STFLR."]
    #[inline(always)]
    pub const fn stflr(self) -> crate::common::Reg<regs::Stflr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x20usize) as _) }
    }
}
pub mod regs {
    #[doc = "desc BCONR."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Bconr(pub u32);
    impl Bconr {
        #[doc = "desc CSTA."]
        #[must_use]
        #[inline(always)]
        pub const fn csta(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "desc CSTA."]
        #[inline(always)]
        pub const fn set_csta(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "desc CAPMDA."]
        #[must_use]
        #[inline(always)]
        pub const fn capmda(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "desc CAPMDA."]
        #[inline(always)]
        pub const fn set_capmda(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "desc SYNSA."]
        #[must_use]
        #[inline(always)]
        pub const fn synsa(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "desc SYNSA."]
        #[inline(always)]
        pub const fn set_synsa(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "desc CKDIVA."]
        #[must_use]
        #[inline(always)]
        pub const fn ckdiva(&self) -> u8 {
            let val = (self.0 >> 4usize) & 0x0f;
            val as u8
        }
        #[doc = "desc CKDIVA."]
        #[inline(always)]
        pub const fn set_ckdiva(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 4usize)) | (((val as u32) & 0x0f) << 4usize);
        }
        #[doc = "desc SYNCLKA."]
        #[must_use]
        #[inline(always)]
        pub const fn synclka(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0x03;
            val as u8
        }
        #[doc = "desc SYNCLKA."]
        #[inline(always)]
        pub const fn set_synclka(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 8usize)) | (((val as u32) & 0x03) << 8usize);
        }
        #[doc = "desc ASYNCLKA."]
        #[must_use]
        #[inline(always)]
        pub const fn asynclka(&self) -> u8 {
            let val = (self.0 >> 10usize) & 0x03;
            val as u8
        }
        #[doc = "desc ASYNCLKA."]
        #[inline(always)]
        pub const fn set_asynclka(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 10usize)) | (((val as u32) & 0x03) << 10usize);
        }
        #[doc = "desc SYNCLKAT."]
        #[must_use]
        #[inline(always)]
        pub const fn synclkat(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "desc SYNCLKAT."]
        #[inline(always)]
        pub const fn set_synclkat(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "desc CMPASTP."]
        #[must_use]
        #[inline(always)]
        pub const fn cmpastp(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "desc CMPASTP."]
        #[inline(always)]
        pub const fn set_cmpastp(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
        #[doc = "desc CSTB."]
        #[must_use]
        #[inline(always)]
        pub const fn cstb(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "desc CSTB."]
        #[inline(always)]
        pub const fn set_cstb(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "desc CAPMDB."]
        #[must_use]
        #[inline(always)]
        pub const fn capmdb(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "desc CAPMDB."]
        #[inline(always)]
        pub const fn set_capmdb(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[doc = "desc SYNSB."]
        #[must_use]
        #[inline(always)]
        pub const fn synsb(&self) -> bool {
            let val = (self.0 >> 19usize) & 0x01;
            val != 0
        }
        #[doc = "desc SYNSB."]
        #[inline(always)]
        pub const fn set_synsb(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
        }
        #[doc = "desc CKDIVB."]
        #[must_use]
        #[inline(always)]
        pub const fn ckdivb(&self) -> u8 {
            let val = (self.0 >> 20usize) & 0x0f;
            val as u8
        }
        #[doc = "desc CKDIVB."]
        #[inline(always)]
        pub const fn set_ckdivb(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 20usize)) | (((val as u32) & 0x0f) << 20usize);
        }
        #[doc = "desc SYNCLKB."]
        #[must_use]
        #[inline(always)]
        pub const fn synclkb(&self) -> u8 {
            let val = (self.0 >> 24usize) & 0x03;
            val as u8
        }
        #[doc = "desc SYNCLKB."]
        #[inline(always)]
        pub const fn set_synclkb(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 24usize)) | (((val as u32) & 0x03) << 24usize);
        }
        #[doc = "desc ASYNCLKB."]
        #[must_use]
        #[inline(always)]
        pub const fn asynclkb(&self) -> u8 {
            let val = (self.0 >> 26usize) & 0x03;
            val as u8
        }
        #[doc = "desc ASYNCLKB."]
        #[inline(always)]
        pub const fn set_asynclkb(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 26usize)) | (((val as u32) & 0x03) << 26usize);
        }
        #[doc = "desc SYNCLKBT."]
        #[must_use]
        #[inline(always)]
        pub const fn synclkbt(&self) -> bool {
            let val = (self.0 >> 28usize) & 0x01;
            val != 0
        }
        #[doc = "desc SYNCLKBT."]
        #[inline(always)]
        pub const fn set_synclkbt(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
        }
        #[doc = "desc CMPBSTP."]
        #[must_use]
        #[inline(always)]
        pub const fn cmpbstp(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[doc = "desc CMPBSTP."]
        #[inline(always)]
        pub const fn set_cmpbstp(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for Bconr {
        #[inline(always)]
        fn default() -> Bconr {
            Bconr(0)
        }
    }
    impl core::fmt::Debug for Bconr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Bconr")
                .field("csta", &self.csta())
                .field("capmda", &self.capmda())
                .field("synsa", &self.synsa())
                .field("ckdiva", &self.ckdiva())
                .field("synclka", &self.synclka())
                .field("asynclka", &self.asynclka())
                .field("synclkat", &self.synclkat())
                .field("cmpastp", &self.cmpastp())
                .field("cstb", &self.cstb())
                .field("capmdb", &self.capmdb())
                .field("synsb", &self.synsb())
                .field("ckdivb", &self.ckdivb())
                .field("synclkb", &self.synclkb())
                .field("asynclkb", &self.asynclkb())
                .field("synclkbt", &self.synclkbt())
                .field("cmpbstp", &self.cmpbstp())
                .finish()
        }
    }
    #[doc = "desc CMPAR."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Cmpar(pub u32);
    impl Cmpar {
        #[doc = "desc CMPA."]
        #[must_use]
        #[inline(always)]
        pub const fn cmpa(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[doc = "desc CMPA."]
        #[inline(always)]
        pub const fn set_cmpa(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
        }
    }
    impl Default for Cmpar {
        #[inline(always)]
        fn default() -> Cmpar {
            Cmpar(0)
        }
    }
    impl core::fmt::Debug for Cmpar {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Cmpar").field("cmpa", &self.cmpa()).finish()
        }
    }
    #[doc = "desc CMPBR."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Cmpbr(pub u32);
    impl Cmpbr {
        #[doc = "desc CMPB."]
        #[must_use]
        #[inline(always)]
        pub const fn cmpb(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[doc = "desc CMPB."]
        #[inline(always)]
        pub const fn set_cmpb(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
        }
    }
    impl Default for Cmpbr {
        #[inline(always)]
        fn default() -> Cmpbr {
            Cmpbr(0)
        }
    }
    impl core::fmt::Debug for Cmpbr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Cmpbr").field("cmpb", &self.cmpb()).finish()
        }
    }
    #[doc = "desc CNTAR."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Cntar(pub u32);
    impl Cntar {
        #[doc = "desc CNTA."]
        #[must_use]
        #[inline(always)]
        pub const fn cnta(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[doc = "desc CNTA."]
        #[inline(always)]
        pub const fn set_cnta(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
        }
    }
    impl Default for Cntar {
        #[inline(always)]
        fn default() -> Cntar {
            Cntar(0)
        }
    }
    impl core::fmt::Debug for Cntar {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Cntar").field("cnta", &self.cnta()).finish()
        }
    }
    #[doc = "desc CNTBR."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Cntbr(pub u32);
    impl Cntbr {
        #[doc = "desc CNTB."]
        #[must_use]
        #[inline(always)]
        pub const fn cntb(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[doc = "desc CNTB."]
        #[inline(always)]
        pub const fn set_cntb(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
        }
    }
    impl Default for Cntbr {
        #[inline(always)]
        fn default() -> Cntbr {
            Cntbr(0)
        }
    }
    impl core::fmt::Debug for Cntbr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Cntbr").field("cntb", &self.cntb()).finish()
        }
    }
    #[doc = "desc HCONR."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Hconr(pub u32);
    impl Hconr {
        #[doc = "desc HSTAA0."]
        #[must_use]
        #[inline(always)]
        pub const fn hstaa0(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "desc HSTAA0."]
        #[inline(always)]
        pub const fn set_hstaa0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "desc HSTAA1."]
        #[must_use]
        #[inline(always)]
        pub const fn hstaa1(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "desc HSTAA1."]
        #[inline(always)]
        pub const fn set_hstaa1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "desc HSTAA2."]
        #[must_use]
        #[inline(always)]
        pub const fn hstaa2(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "desc HSTAA2."]
        #[inline(always)]
        pub const fn set_hstaa2(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "desc HSTPA0."]
        #[must_use]
        #[inline(always)]
        pub const fn hstpa0(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "desc HSTPA0."]
        #[inline(always)]
        pub const fn set_hstpa0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "desc HSTPA1."]
        #[must_use]
        #[inline(always)]
        pub const fn hstpa1(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "desc HSTPA1."]
        #[inline(always)]
        pub const fn set_hstpa1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "desc HSTPA2."]
        #[must_use]
        #[inline(always)]
        pub const fn hstpa2(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "desc HSTPA2."]
        #[inline(always)]
        pub const fn set_hstpa2(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "desc HCLEA0."]
        #[must_use]
        #[inline(always)]
        pub const fn hclea0(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "desc HCLEA0."]
        #[inline(always)]
        pub const fn set_hclea0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "desc HCLEA1."]
        #[must_use]
        #[inline(always)]
        pub const fn hclea1(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "desc HCLEA1."]
        #[inline(always)]
        pub const fn set_hclea1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "desc HCLEA2."]
        #[must_use]
        #[inline(always)]
        pub const fn hclea2(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "desc HCLEA2."]
        #[inline(always)]
        pub const fn set_hclea2(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[doc = "desc HICPA0."]
        #[must_use]
        #[inline(always)]
        pub const fn hicpa0(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "desc HICPA0."]
        #[inline(always)]
        pub const fn set_hicpa0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "desc HICPA1."]
        #[must_use]
        #[inline(always)]
        pub const fn hicpa1(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[doc = "desc HICPA1."]
        #[inline(always)]
        pub const fn set_hicpa1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[doc = "desc HICPA2."]
        #[must_use]
        #[inline(always)]
        pub const fn hicpa2(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "desc HICPA2."]
        #[inline(always)]
        pub const fn set_hicpa2(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[doc = "desc HSTAB0."]
        #[must_use]
        #[inline(always)]
        pub const fn hstab0(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "desc HSTAB0."]
        #[inline(always)]
        pub const fn set_hstab0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "desc HSTAB1."]
        #[must_use]
        #[inline(always)]
        pub const fn hstab1(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "desc HSTAB1."]
        #[inline(always)]
        pub const fn set_hstab1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[doc = "desc HSTAB2."]
        #[must_use]
        #[inline(always)]
        pub const fn hstab2(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[doc = "desc HSTAB2."]
        #[inline(always)]
        pub const fn set_hstab2(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
        #[doc = "desc HSTPB0."]
        #[must_use]
        #[inline(always)]
        pub const fn hstpb0(&self) -> bool {
            let val = (self.0 >> 20usize) & 0x01;
            val != 0
        }
        #[doc = "desc HSTPB0."]
        #[inline(always)]
        pub const fn set_hstpb0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
        }
        #[doc = "desc HSTPB1."]
        #[must_use]
        #[inline(always)]
        pub const fn hstpb1(&self) -> bool {
            let val = (self.0 >> 21usize) & 0x01;
            val != 0
        }
        #[doc = "desc HSTPB1."]
        #[inline(always)]
        pub const fn set_hstpb1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
        }
        #[doc = "desc HSTPB2."]
        #[must_use]
        #[inline(always)]
        pub const fn hstpb2(&self) -> bool {
            let val = (self.0 >> 22usize) & 0x01;
            val != 0
        }
        #[doc = "desc HSTPB2."]
        #[inline(always)]
        pub const fn set_hstpb2(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
        }
        #[doc = "desc HCLEB0."]
        #[must_use]
        #[inline(always)]
        pub const fn hcleb0(&self) -> bool {
            let val = (self.0 >> 24usize) & 0x01;
            val != 0
        }
        #[doc = "desc HCLEB0."]
        #[inline(always)]
        pub const fn set_hcleb0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
        }
        #[doc = "desc HCLEB1."]
        #[must_use]
        #[inline(always)]
        pub const fn hcleb1(&self) -> bool {
            let val = (self.0 >> 25usize) & 0x01;
            val != 0
        }
        #[doc = "desc HCLEB1."]
        #[inline(always)]
        pub const fn set_hcleb1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
        }
        #[doc = "desc HCLEB2."]
        #[must_use]
        #[inline(always)]
        pub const fn hcleb2(&self) -> bool {
            let val = (self.0 >> 26usize) & 0x01;
            val != 0
        }
        #[doc = "desc HCLEB2."]
        #[inline(always)]
        pub const fn set_hcleb2(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
        }
        #[doc = "desc HICPB0."]
        #[must_use]
        #[inline(always)]
        pub const fn hicpb0(&self) -> bool {
            let val = (self.0 >> 28usize) & 0x01;
            val != 0
        }
        #[doc = "desc HICPB0."]
        #[inline(always)]
        pub const fn set_hicpb0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
        }
        #[doc = "desc HICPB1."]
        #[must_use]
        #[inline(always)]
        pub const fn hicpb1(&self) -> bool {
            let val = (self.0 >> 29usize) & 0x01;
            val != 0
        }
        #[doc = "desc HICPB1."]
        #[inline(always)]
        pub const fn set_hicpb1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
        }
        #[doc = "desc HICPB2."]
        #[must_use]
        #[inline(always)]
        pub const fn hicpb2(&self) -> bool {
            let val = (self.0 >> 30usize) & 0x01;
            val != 0
        }
        #[doc = "desc HICPB2."]
        #[inline(always)]
        pub const fn set_hicpb2(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
        }
    }
    impl Default for Hconr {
        #[inline(always)]
        fn default() -> Hconr {
            Hconr(0)
        }
    }
    impl core::fmt::Debug for Hconr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Hconr")
                .field("hstaa0", &self.hstaa0())
                .field("hstaa1", &self.hstaa1())
                .field("hstaa2", &self.hstaa2())
                .field("hstpa0", &self.hstpa0())
                .field("hstpa1", &self.hstpa1())
                .field("hstpa2", &self.hstpa2())
                .field("hclea0", &self.hclea0())
                .field("hclea1", &self.hclea1())
                .field("hclea2", &self.hclea2())
                .field("hicpa0", &self.hicpa0())
                .field("hicpa1", &self.hicpa1())
                .field("hicpa2", &self.hicpa2())
                .field("hstab0", &self.hstab0())
                .field("hstab1", &self.hstab1())
                .field("hstab2", &self.hstab2())
                .field("hstpb0", &self.hstpb0())
                .field("hstpb1", &self.hstpb1())
                .field("hstpb2", &self.hstpb2())
                .field("hcleb0", &self.hcleb0())
                .field("hcleb1", &self.hcleb1())
                .field("hcleb2", &self.hcleb2())
                .field("hicpb0", &self.hicpb0())
                .field("hicpb1", &self.hicpb1())
                .field("hicpb2", &self.hicpb2())
                .finish()
        }
    }
    #[doc = "desc ICONR."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Iconr(pub u32);
    impl Iconr {
        #[doc = "desc CMENA."]
        #[must_use]
        #[inline(always)]
        pub const fn cmena(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "desc CMENA."]
        #[inline(always)]
        pub const fn set_cmena(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "desc OVENA."]
        #[must_use]
        #[inline(always)]
        pub const fn ovena(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "desc OVENA."]
        #[inline(always)]
        pub const fn set_ovena(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "desc CMENB."]
        #[must_use]
        #[inline(always)]
        pub const fn cmenb(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "desc CMENB."]
        #[inline(always)]
        pub const fn set_cmenb(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "desc OVENB."]
        #[must_use]
        #[inline(always)]
        pub const fn ovenb(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "desc OVENB."]
        #[inline(always)]
        pub const fn set_ovenb(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
    }
    impl Default for Iconr {
        #[inline(always)]
        fn default() -> Iconr {
            Iconr(0)
        }
    }
    impl core::fmt::Debug for Iconr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Iconr")
                .field("cmena", &self.cmena())
                .field("ovena", &self.ovena())
                .field("cmenb", &self.cmenb())
                .field("ovenb", &self.ovenb())
                .finish()
        }
    }
    #[doc = "desc PCONR."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Pconr(pub u32);
    impl Pconr {
        #[doc = "desc STACA."]
        #[must_use]
        #[inline(always)]
        pub const fn staca(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x03;
            val as u8
        }
        #[doc = "desc STACA."]
        #[inline(always)]
        pub const fn set_staca(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 0usize)) | (((val as u32) & 0x03) << 0usize);
        }
        #[doc = "desc STPCA."]
        #[must_use]
        #[inline(always)]
        pub const fn stpca(&self) -> u8 {
            let val = (self.0 >> 2usize) & 0x03;
            val as u8
        }
        #[doc = "desc STPCA."]
        #[inline(always)]
        pub const fn set_stpca(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 2usize)) | (((val as u32) & 0x03) << 2usize);
        }
        #[doc = "desc CMPCA."]
        #[must_use]
        #[inline(always)]
        pub const fn cmpca(&self) -> u8 {
            let val = (self.0 >> 4usize) & 0x03;
            val as u8
        }
        #[doc = "desc CMPCA."]
        #[inline(always)]
        pub const fn set_cmpca(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 4usize)) | (((val as u32) & 0x03) << 4usize);
        }
        #[doc = "desc OUTENA."]
        #[must_use]
        #[inline(always)]
        pub const fn outena(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "desc OUTENA."]
        #[inline(always)]
        pub const fn set_outena(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "desc NOFIENA."]
        #[must_use]
        #[inline(always)]
        pub const fn nofiena(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "desc NOFIENA."]
        #[inline(always)]
        pub const fn set_nofiena(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "desc NOFICKA."]
        #[must_use]
        #[inline(always)]
        pub const fn noficka(&self) -> u8 {
            let val = (self.0 >> 13usize) & 0x03;
            val as u8
        }
        #[doc = "desc NOFICKA."]
        #[inline(always)]
        pub const fn set_noficka(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 13usize)) | (((val as u32) & 0x03) << 13usize);
        }
        #[doc = "desc STACB."]
        #[must_use]
        #[inline(always)]
        pub const fn stacb(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x03;
            val as u8
        }
        #[doc = "desc STACB."]
        #[inline(always)]
        pub const fn set_stacb(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 16usize)) | (((val as u32) & 0x03) << 16usize);
        }
        #[doc = "desc STPCB."]
        #[must_use]
        #[inline(always)]
        pub const fn stpcb(&self) -> u8 {
            let val = (self.0 >> 18usize) & 0x03;
            val as u8
        }
        #[doc = "desc STPCB."]
        #[inline(always)]
        pub const fn set_stpcb(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 18usize)) | (((val as u32) & 0x03) << 18usize);
        }
        #[doc = "desc CMPCB."]
        #[must_use]
        #[inline(always)]
        pub const fn cmpcb(&self) -> u8 {
            let val = (self.0 >> 20usize) & 0x03;
            val as u8
        }
        #[doc = "desc CMPCB."]
        #[inline(always)]
        pub const fn set_cmpcb(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 20usize)) | (((val as u32) & 0x03) << 20usize);
        }
        #[doc = "desc OUTENB."]
        #[must_use]
        #[inline(always)]
        pub const fn outenb(&self) -> bool {
            let val = (self.0 >> 24usize) & 0x01;
            val != 0
        }
        #[doc = "desc OUTENB."]
        #[inline(always)]
        pub const fn set_outenb(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
        }
        #[doc = "desc NOFIENB."]
        #[must_use]
        #[inline(always)]
        pub const fn nofienb(&self) -> bool {
            let val = (self.0 >> 28usize) & 0x01;
            val != 0
        }
        #[doc = "desc NOFIENB."]
        #[inline(always)]
        pub const fn set_nofienb(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
        }
        #[doc = "desc NOFICKB."]
        #[must_use]
        #[inline(always)]
        pub const fn nofickb(&self) -> u8 {
            let val = (self.0 >> 29usize) & 0x03;
            val as u8
        }
        #[doc = "desc NOFICKB."]
        #[inline(always)]
        pub const fn set_nofickb(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 29usize)) | (((val as u32) & 0x03) << 29usize);
        }
    }
    impl Default for Pconr {
        #[inline(always)]
        fn default() -> Pconr {
            Pconr(0)
        }
    }
    impl core::fmt::Debug for Pconr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Pconr")
                .field("staca", &self.staca())
                .field("stpca", &self.stpca())
                .field("cmpca", &self.cmpca())
                .field("outena", &self.outena())
                .field("nofiena", &self.nofiena())
                .field("noficka", &self.noficka())
                .field("stacb", &self.stacb())
                .field("stpcb", &self.stpcb())
                .field("cmpcb", &self.cmpcb())
                .field("outenb", &self.outenb())
                .field("nofienb", &self.nofienb())
                .field("nofickb", &self.nofickb())
                .finish()
        }
    }
    #[doc = "desc STFLR."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Stflr(pub u32);
    impl Stflr {
        #[doc = "desc CMFA."]
        #[must_use]
        #[inline(always)]
        pub const fn cmfa(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "desc CMFA."]
        #[inline(always)]
        pub const fn set_cmfa(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "desc OVFA."]
        #[must_use]
        #[inline(always)]
        pub const fn ovfa(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "desc OVFA."]
        #[inline(always)]
        pub const fn set_ovfa(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "desc ICPA."]
        #[must_use]
        #[inline(always)]
        pub const fn icpa(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "desc ICPA."]
        #[inline(always)]
        pub const fn set_icpa(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "desc SYDA."]
        #[must_use]
        #[inline(always)]
        pub const fn syda(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "desc SYDA."]
        #[inline(always)]
        pub const fn set_syda(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "desc CMFB."]
        #[must_use]
        #[inline(always)]
        pub const fn cmfb(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "desc CMFB."]
        #[inline(always)]
        pub const fn set_cmfb(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "desc OVFB."]
        #[must_use]
        #[inline(always)]
        pub const fn ovfb(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "desc OVFB."]
        #[inline(always)]
        pub const fn set_ovfb(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[doc = "desc ICPB."]
        #[must_use]
        #[inline(always)]
        pub const fn icpb(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[doc = "desc ICPB."]
        #[inline(always)]
        pub const fn set_icpb(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
        #[doc = "desc SYDB."]
        #[must_use]
        #[inline(always)]
        pub const fn sydb(&self) -> bool {
            let val = (self.0 >> 24usize) & 0x01;
            val != 0
        }
        #[doc = "desc SYDB."]
        #[inline(always)]
        pub const fn set_sydb(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
        }
    }
    impl Default for Stflr {
        #[inline(always)]
        fn default() -> Stflr {
            Stflr(0)
        }
    }
    impl core::fmt::Debug for Stflr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Stflr")
                .field("cmfa", &self.cmfa())
                .field("ovfa", &self.ovfa())
                .field("icpa", &self.icpa())
                .field("syda", &self.syda())
                .field("cmfb", &self.cmfb())
                .field("ovfb", &self.ovfb())
                .field("icpb", &self.icpb())
                .field("sydb", &self.sydb())
                .finish()
        }
    }
}
