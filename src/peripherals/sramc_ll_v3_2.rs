#![allow(clippy::missing_safety_doc)]
#![allow(clippy::identity_op)]
#![allow(clippy::unnecessary_cast)]
#![allow(clippy::erasing_op)]

#[doc = "desc SRAMC."]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sramc {
    ptr: *mut u8,
}
unsafe impl Send for Sramc {}
unsafe impl Sync for Sramc {}
impl Sramc {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "desc WTCR."]
    #[inline(always)]
    pub const fn wtcr(self) -> crate::common::Reg<regs::Wtcr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0usize) as _) }
    }
    #[doc = "desc WTPR."]
    #[inline(always)]
    pub const fn wtpr(self) -> crate::common::Reg<regs::Wtpr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x04usize) as _) }
    }
    #[doc = "desc CKCR."]
    #[inline(always)]
    pub const fn ckcr(self) -> crate::common::Reg<regs::Ckcr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x08usize) as _) }
    }
    #[doc = "desc CKPR."]
    #[inline(always)]
    pub const fn ckpr(self) -> crate::common::Reg<regs::Ckpr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0cusize) as _) }
    }
    #[doc = "desc CKSR."]
    #[inline(always)]
    pub const fn cksr(self) -> crate::common::Reg<regs::Cksr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x10usize) as _) }
    }
    #[doc = "desc SRAM1_EIEN."]
    #[inline(always)]
    pub const fn sram1_eien(self) -> crate::common::Reg<regs::Sram1Eien, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x14usize) as _) }
    }
    #[doc = "desc SRAM1_EIBIT0."]
    #[inline(always)]
    pub const fn sram1_eibit0(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x18usize) as _) }
    }
    #[doc = "desc SRAM1_EIBIT1."]
    #[inline(always)]
    pub const fn sram1_eibit1(self) -> crate::common::Reg<regs::Sram1Eibit1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1cusize) as _) }
    }
    #[doc = "desc SRAM1_ECCERRADDR."]
    #[inline(always)]
    pub const fn sram1_eccerraddr(
        self,
    ) -> crate::common::Reg<regs::Sram1Eccerraddr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x20usize) as _) }
    }
    #[doc = "desc SRAMH_EIEN."]
    #[inline(always)]
    pub const fn sramh_eien(self) -> crate::common::Reg<regs::SramhEien, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x24usize) as _) }
    }
    #[doc = "desc SRAMH_EIBIT0."]
    #[inline(always)]
    pub const fn sramh_eibit0(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x28usize) as _) }
    }
    #[doc = "desc SRAMH_EIBIT1."]
    #[inline(always)]
    pub const fn sramh_eibit1(self) -> crate::common::Reg<regs::SramhEibit1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x2cusize) as _) }
    }
    #[doc = "desc SRAMH_ECCERRADDR."]
    #[inline(always)]
    pub const fn sramh_eccerraddr(
        self,
    ) -> crate::common::Reg<regs::SramhEccerraddr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x30usize) as _) }
    }
}
pub mod regs {
    #[doc = "desc CKCR."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ckcr(pub u32);
    impl Ckcr {
        #[doc = "desc CACHE_ECCOAD."]
        #[must_use]
        #[inline(always)]
        pub const fn cache_eccoad(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "desc CACHE_ECCOAD."]
        #[inline(always)]
        pub const fn set_cache_eccoad(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "desc SRAM1_ECCOAD."]
        #[must_use]
        #[inline(always)]
        pub const fn sram1_eccoad(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "desc SRAM1_ECCOAD."]
        #[inline(always)]
        pub const fn set_sram1_eccoad(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "desc SRAMH_ECCOAD."]
        #[must_use]
        #[inline(always)]
        pub const fn sramh_eccoad(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "desc SRAMH_ECCOAD."]
        #[inline(always)]
        pub const fn set_sramh_eccoad(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "desc CACHE_ECCMOD."]
        #[must_use]
        #[inline(always)]
        pub const fn cache_eccmod(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0x03;
            val as u8
        }
        #[doc = "desc CACHE_ECCMOD."]
        #[inline(always)]
        pub const fn set_cache_eccmod(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 8usize)) | (((val as u32) & 0x03) << 8usize);
        }
        #[doc = "desc SRAM1_ECCMOD."]
        #[must_use]
        #[inline(always)]
        pub const fn sram1_eccmod(&self) -> u8 {
            let val = (self.0 >> 10usize) & 0x03;
            val as u8
        }
        #[doc = "desc SRAM1_ECCMOD."]
        #[inline(always)]
        pub const fn set_sram1_eccmod(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 10usize)) | (((val as u32) & 0x03) << 10usize);
        }
        #[doc = "desc SRAMH_ECCMOD."]
        #[must_use]
        #[inline(always)]
        pub const fn sramh_eccmod(&self) -> u8 {
            let val = (self.0 >> 12usize) & 0x03;
            val as u8
        }
        #[doc = "desc SRAMH_ECCMOD."]
        #[inline(always)]
        pub const fn set_sramh_eccmod(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 12usize)) | (((val as u32) & 0x03) << 12usize);
        }
    }
    impl Default for Ckcr {
        #[inline(always)]
        fn default() -> Ckcr {
            Ckcr(0)
        }
    }
    impl core::fmt::Debug for Ckcr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Ckcr")
                .field("cache_eccoad", &self.cache_eccoad())
                .field("sram1_eccoad", &self.sram1_eccoad())
                .field("sramh_eccoad", &self.sramh_eccoad())
                .field("cache_eccmod", &self.cache_eccmod())
                .field("sram1_eccmod", &self.sram1_eccmod())
                .field("sramh_eccmod", &self.sramh_eccmod())
                .finish()
        }
    }
    #[doc = "desc CKPR."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ckpr(pub u32);
    impl Ckpr {
        #[doc = "desc CKPRC."]
        #[must_use]
        #[inline(always)]
        pub const fn ckprc(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "desc CKPRC."]
        #[inline(always)]
        pub const fn set_ckprc(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "desc CKPRKW."]
        #[must_use]
        #[inline(always)]
        pub const fn ckprkw(&self) -> u8 {
            let val = (self.0 >> 1usize) & 0x7f;
            val as u8
        }
        #[doc = "desc CKPRKW."]
        #[inline(always)]
        pub const fn set_ckprkw(&mut self, val: u8) {
            self.0 = (self.0 & !(0x7f << 1usize)) | (((val as u32) & 0x7f) << 1usize);
        }
    }
    impl Default for Ckpr {
        #[inline(always)]
        fn default() -> Ckpr {
            Ckpr(0)
        }
    }
    impl core::fmt::Debug for Ckpr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Ckpr")
                .field("ckprc", &self.ckprc())
                .field("ckprkw", &self.ckprkw())
                .finish()
        }
    }
    #[doc = "desc CKSR."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Cksr(pub u32);
    impl Cksr {
        #[doc = "desc CACHE_1ERR."]
        #[must_use]
        #[inline(always)]
        pub const fn cache_1err(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "desc CACHE_1ERR."]
        #[inline(always)]
        pub const fn set_cache_1err(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "desc CACHE_2ERR."]
        #[must_use]
        #[inline(always)]
        pub const fn cache_2err(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "desc CACHE_2ERR."]
        #[inline(always)]
        pub const fn set_cache_2err(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "desc SRAM1_1ERR."]
        #[must_use]
        #[inline(always)]
        pub const fn sram1_1err(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "desc SRAM1_1ERR."]
        #[inline(always)]
        pub const fn set_sram1_1err(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "desc SRAM1_2ERR."]
        #[must_use]
        #[inline(always)]
        pub const fn sram1_2err(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "desc SRAM1_2ERR."]
        #[inline(always)]
        pub const fn set_sram1_2err(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "desc SRAMH_1ERR."]
        #[must_use]
        #[inline(always)]
        pub const fn sramh_1err(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "desc SRAMH_1ERR."]
        #[inline(always)]
        pub const fn set_sramh_1err(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "desc SRAMH_2ERR."]
        #[must_use]
        #[inline(always)]
        pub const fn sramh_2err(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "desc SRAMH_2ERR."]
        #[inline(always)]
        pub const fn set_sramh_2err(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
    }
    impl Default for Cksr {
        #[inline(always)]
        fn default() -> Cksr {
            Cksr(0)
        }
    }
    impl core::fmt::Debug for Cksr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Cksr")
                .field("cache_1err", &self.cache_1err())
                .field("cache_2err", &self.cache_2err())
                .field("sram1_1err", &self.sram1_1err())
                .field("sram1_2err", &self.sram1_2err())
                .field("sramh_1err", &self.sramh_1err())
                .field("sramh_2err", &self.sramh_2err())
                .finish()
        }
    }
    #[doc = "desc SRAM1_ECCERRADDR."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Sram1Eccerraddr(pub u32);
    impl Sram1Eccerraddr {
        #[doc = "desc ECCERRADDR."]
        #[must_use]
        #[inline(always)]
        pub const fn eccerraddr(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0x0001_ffff;
            val as u32
        }
        #[doc = "desc ECCERRADDR."]
        #[inline(always)]
        pub const fn set_eccerraddr(&mut self, val: u32) {
            self.0 = (self.0 & !(0x0001_ffff << 0usize)) | (((val as u32) & 0x0001_ffff) << 0usize);
        }
    }
    impl Default for Sram1Eccerraddr {
        #[inline(always)]
        fn default() -> Sram1Eccerraddr {
            Sram1Eccerraddr(0)
        }
    }
    impl core::fmt::Debug for Sram1Eccerraddr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Sram1Eccerraddr")
                .field("eccerraddr", &self.eccerraddr())
                .finish()
        }
    }
    #[doc = "desc SRAM1_EIBIT1."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Sram1Eibit1(pub u32);
    impl Sram1Eibit1 {
        #[doc = "desc EIBIT."]
        #[must_use]
        #[inline(always)]
        pub const fn eibit(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x7f;
            val as u8
        }
        #[doc = "desc EIBIT."]
        #[inline(always)]
        pub const fn set_eibit(&mut self, val: u8) {
            self.0 = (self.0 & !(0x7f << 0usize)) | (((val as u32) & 0x7f) << 0usize);
        }
    }
    impl Default for Sram1Eibit1 {
        #[inline(always)]
        fn default() -> Sram1Eibit1 {
            Sram1Eibit1(0)
        }
    }
    impl core::fmt::Debug for Sram1Eibit1 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Sram1Eibit1")
                .field("eibit", &self.eibit())
                .finish()
        }
    }
    #[doc = "desc SRAM1_EIEN."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Sram1Eien(pub u32);
    impl Sram1Eien {
        #[doc = "desc EIEN."]
        #[must_use]
        #[inline(always)]
        pub const fn eien(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "desc EIEN."]
        #[inline(always)]
        pub const fn set_eien(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
    }
    impl Default for Sram1Eien {
        #[inline(always)]
        fn default() -> Sram1Eien {
            Sram1Eien(0)
        }
    }
    impl core::fmt::Debug for Sram1Eien {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Sram1Eien")
                .field("eien", &self.eien())
                .finish()
        }
    }
    #[doc = "desc SRAMH_ECCERRADDR."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct SramhEccerraddr(pub u32);
    impl SramhEccerraddr {
        #[doc = "desc ECCERRADDR."]
        #[must_use]
        #[inline(always)]
        pub const fn eccerraddr(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0x0001_ffff;
            val as u32
        }
        #[doc = "desc ECCERRADDR."]
        #[inline(always)]
        pub const fn set_eccerraddr(&mut self, val: u32) {
            self.0 = (self.0 & !(0x0001_ffff << 0usize)) | (((val as u32) & 0x0001_ffff) << 0usize);
        }
    }
    impl Default for SramhEccerraddr {
        #[inline(always)]
        fn default() -> SramhEccerraddr {
            SramhEccerraddr(0)
        }
    }
    impl core::fmt::Debug for SramhEccerraddr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("SramhEccerraddr")
                .field("eccerraddr", &self.eccerraddr())
                .finish()
        }
    }
    #[doc = "desc SRAMH_EIBIT1."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct SramhEibit1(pub u32);
    impl SramhEibit1 {
        #[doc = "desc EIBIT."]
        #[must_use]
        #[inline(always)]
        pub const fn eibit(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x7f;
            val as u8
        }
        #[doc = "desc EIBIT."]
        #[inline(always)]
        pub const fn set_eibit(&mut self, val: u8) {
            self.0 = (self.0 & !(0x7f << 0usize)) | (((val as u32) & 0x7f) << 0usize);
        }
    }
    impl Default for SramhEibit1 {
        #[inline(always)]
        fn default() -> SramhEibit1 {
            SramhEibit1(0)
        }
    }
    impl core::fmt::Debug for SramhEibit1 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("SramhEibit1")
                .field("eibit", &self.eibit())
                .finish()
        }
    }
    #[doc = "desc SRAMH_EIEN."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct SramhEien(pub u32);
    impl SramhEien {
        #[doc = "desc EIEN."]
        #[must_use]
        #[inline(always)]
        pub const fn eien(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "desc EIEN."]
        #[inline(always)]
        pub const fn set_eien(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
    }
    impl Default for SramhEien {
        #[inline(always)]
        fn default() -> SramhEien {
            SramhEien(0)
        }
    }
    impl core::fmt::Debug for SramhEien {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("SramhEien")
                .field("eien", &self.eien())
                .finish()
        }
    }
    #[doc = "desc WTCR."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Wtcr(pub u32);
    impl Wtcr {
        #[doc = "desc SRAM1RWT."]
        #[must_use]
        #[inline(always)]
        pub const fn sram1rwt(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x07;
            val as u8
        }
        #[doc = "desc SRAM1RWT."]
        #[inline(always)]
        pub const fn set_sram1rwt(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 0usize)) | (((val as u32) & 0x07) << 0usize);
        }
        #[doc = "desc SRAM1WWT."]
        #[must_use]
        #[inline(always)]
        pub const fn sram1wwt(&self) -> u8 {
            let val = (self.0 >> 4usize) & 0x07;
            val as u8
        }
        #[doc = "desc SRAM1WWT."]
        #[inline(always)]
        pub const fn set_sram1wwt(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 4usize)) | (((val as u32) & 0x07) << 4usize);
        }
        #[doc = "desc SRAMHRWT."]
        #[must_use]
        #[inline(always)]
        pub const fn sramhrwt(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0x07;
            val as u8
        }
        #[doc = "desc SRAMHRWT."]
        #[inline(always)]
        pub const fn set_sramhrwt(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 8usize)) | (((val as u32) & 0x07) << 8usize);
        }
        #[doc = "desc SRAMHWWT."]
        #[must_use]
        #[inline(always)]
        pub const fn sramhwwt(&self) -> u8 {
            let val = (self.0 >> 12usize) & 0x07;
            val as u8
        }
        #[doc = "desc SRAMHWWT."]
        #[inline(always)]
        pub const fn set_sramhwwt(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 12usize)) | (((val as u32) & 0x07) << 12usize);
        }
    }
    impl Default for Wtcr {
        #[inline(always)]
        fn default() -> Wtcr {
            Wtcr(0)
        }
    }
    impl core::fmt::Debug for Wtcr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Wtcr")
                .field("sram1rwt", &self.sram1rwt())
                .field("sram1wwt", &self.sram1wwt())
                .field("sramhrwt", &self.sramhrwt())
                .field("sramhwwt", &self.sramhwwt())
                .finish()
        }
    }
    #[doc = "desc WTPR."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Wtpr(pub u32);
    impl Wtpr {
        #[doc = "desc WTPRC."]
        #[must_use]
        #[inline(always)]
        pub const fn wtprc(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "desc WTPRC."]
        #[inline(always)]
        pub const fn set_wtprc(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "desc WTPRKW."]
        #[must_use]
        #[inline(always)]
        pub const fn wtprkw(&self) -> u8 {
            let val = (self.0 >> 1usize) & 0x7f;
            val as u8
        }
        #[doc = "desc WTPRKW."]
        #[inline(always)]
        pub const fn set_wtprkw(&mut self, val: u8) {
            self.0 = (self.0 & !(0x7f << 1usize)) | (((val as u32) & 0x7f) << 1usize);
        }
    }
    impl Default for Wtpr {
        #[inline(always)]
        fn default() -> Wtpr {
            Wtpr(0)
        }
    }
    impl core::fmt::Debug for Wtpr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Wtpr")
                .field("wtprc", &self.wtprc())
                .field("wtprkw", &self.wtprkw())
                .finish()
        }
    }
}
