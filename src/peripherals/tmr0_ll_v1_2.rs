#![allow(clippy::missing_safety_doc)]
#![allow(clippy::identity_op)]
#![allow(clippy::unnecessary_cast)]
#![allow(clippy::erasing_op)]

#[doc = "desc TMR0."]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Tmr0 {
    ptr: *mut u8,
}
unsafe impl Send for Tmr0 {}
unsafe impl Sync for Tmr0 {}
impl Tmr0 {
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
    #[doc = "desc STFLR."]
    #[inline(always)]
    pub const fn stflr(self) -> crate::common::Reg<regs::Stflr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x14usize) as _) }
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
        #[doc = "desc CMENA."]
        #[must_use]
        #[inline(always)]
        pub const fn cmena(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "desc CMENA."]
        #[inline(always)]
        pub const fn set_cmena(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "desc OVENA."]
        #[must_use]
        #[inline(always)]
        pub const fn ovena(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "desc OVENA."]
        #[inline(always)]
        pub const fn set_ovena(&mut self, val: bool) {
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
        #[doc = "desc SYNSA."]
        #[must_use]
        #[inline(always)]
        pub const fn synsa(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "desc SYNSA."]
        #[inline(always)]
        pub const fn set_synsa(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "desc SYNCLKA."]
        #[must_use]
        #[inline(always)]
        pub const fn synclka(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "desc SYNCLKA."]
        #[inline(always)]
        pub const fn set_synclka(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "desc ASYNCLKA."]
        #[must_use]
        #[inline(always)]
        pub const fn asynclka(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "desc ASYNCLKA."]
        #[inline(always)]
        pub const fn set_asynclka(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[doc = "desc HSTAA."]
        #[must_use]
        #[inline(always)]
        pub const fn hstaa(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "desc HSTAA."]
        #[inline(always)]
        pub const fn set_hstaa(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "desc HSTPA."]
        #[must_use]
        #[inline(always)]
        pub const fn hstpa(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[doc = "desc HSTPA."]
        #[inline(always)]
        pub const fn set_hstpa(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[doc = "desc HCLEA."]
        #[must_use]
        #[inline(always)]
        pub const fn hclea(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "desc HCLEA."]
        #[inline(always)]
        pub const fn set_hclea(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[doc = "desc HICPA."]
        #[must_use]
        #[inline(always)]
        pub const fn hicpa(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "desc HICPA."]
        #[inline(always)]
        pub const fn set_hicpa(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
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
                .field("cmena", &self.cmena())
                .field("ovena", &self.ovena())
                .field("ckdiva", &self.ckdiva())
                .field("synsa", &self.synsa())
                .field("synclka", &self.synclka())
                .field("asynclka", &self.asynclka())
                .field("hstaa", &self.hstaa())
                .field("hstpa", &self.hstpa())
                .field("hclea", &self.hclea())
                .field("hicpa", &self.hicpa())
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
                .finish()
        }
    }
}
