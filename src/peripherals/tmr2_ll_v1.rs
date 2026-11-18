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
    #[doc = "desc CMPAR."]
    #[inline(always)]
    pub const fn cmpar(self) -> crate::common::Reg<regs::Cmpar, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x08usize) as _) }
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
                .finish()
        }
    }
}
