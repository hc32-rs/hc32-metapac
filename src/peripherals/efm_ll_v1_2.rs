#![allow(clippy::missing_safety_doc)]
#![allow(clippy::identity_op)]
#![allow(clippy::unnecessary_cast)]
#![allow(clippy::erasing_op)]

#[doc = "desc EFM."]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Efm {
    ptr: *mut u8,
}
unsafe impl Send for Efm {}
unsafe impl Sync for Efm {}
impl Efm {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "desc FAPRT."]
    #[inline(always)]
    pub const fn faprt(self) -> crate::common::Reg<regs::Faprt, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0usize) as _) }
    }
    #[doc = "desc FSTP."]
    #[inline(always)]
    pub const fn fstp(self) -> crate::common::Reg<regs::Fstp, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x04usize) as _) }
    }
    #[doc = "desc FRMC."]
    #[inline(always)]
    pub const fn frmc(self) -> crate::common::Reg<regs::Frmc, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x08usize) as _) }
    }
    #[doc = "desc FWMC."]
    #[inline(always)]
    pub const fn fwmc(self) -> crate::common::Reg<regs::Fwmc, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0cusize) as _) }
    }
    #[doc = "desc FSR."]
    #[inline(always)]
    pub const fn fsr(self) -> crate::common::Reg<regs::Fsr, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x10usize) as _) }
    }
    #[doc = "desc FSCLR."]
    #[inline(always)]
    pub const fn fsclr(self) -> crate::common::Reg<regs::Fsclr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x14usize) as _) }
    }
    #[doc = "desc FITE."]
    #[inline(always)]
    pub const fn fite(self) -> crate::common::Reg<regs::Fite, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x18usize) as _) }
    }
    #[doc = "desc FPMTSW."]
    #[inline(always)]
    pub const fn fpmtsw(self) -> crate::common::Reg<regs::Fpmtsw, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x20usize) as _) }
    }
    #[doc = "desc FPMTEW."]
    #[inline(always)]
    pub const fn fpmtew(self) -> crate::common::Reg<regs::Fpmtew, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x24usize) as _) }
    }
    #[doc = "desc UQID0."]
    #[inline(always)]
    pub const fn uqid0(self) -> crate::common::Reg<u32, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x50usize) as _) }
    }
    #[doc = "desc UQID1."]
    #[inline(always)]
    pub const fn uqid1(self) -> crate::common::Reg<u32, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x54usize) as _) }
    }
    #[doc = "desc UQID2."]
    #[inline(always)]
    pub const fn uqid2(self) -> crate::common::Reg<u32, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x58usize) as _) }
    }
    #[doc = "desc HRCCFGR."]
    #[inline(always)]
    pub const fn hrccfgr(self) -> crate::common::Reg<regs::Hrccfgr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0282usize) as _) }
    }
    #[doc = "desc LVDICGCR."]
    #[inline(always)]
    pub const fn lvdicgcr(self) -> crate::common::Reg<regs::Lvdicgcr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0284usize) as _) }
    }
}
pub mod regs {
    #[doc = "desc FAPRT."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Faprt(pub u32);
    impl Faprt {
        #[doc = "desc FAPRT."]
        #[must_use]
        #[inline(always)]
        pub const fn faprt(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[doc = "desc FAPRT."]
        #[inline(always)]
        pub const fn set_faprt(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
        }
    }
    impl Default for Faprt {
        #[inline(always)]
        fn default() -> Faprt {
            Faprt(0)
        }
    }
    impl core::fmt::Debug for Faprt {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Faprt")
                .field("faprt", &self.faprt())
                .finish()
        }
    }
    #[doc = "desc FITE."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Fite(pub u32);
    impl Fite {
        #[doc = "desc PEERRITE."]
        #[must_use]
        #[inline(always)]
        pub const fn peerrite(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "desc PEERRITE."]
        #[inline(always)]
        pub const fn set_peerrite(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "desc OPTENDITE."]
        #[must_use]
        #[inline(always)]
        pub const fn optendite(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "desc OPTENDITE."]
        #[inline(always)]
        pub const fn set_optendite(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "desc COLERRITE."]
        #[must_use]
        #[inline(always)]
        pub const fn colerrite(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "desc COLERRITE."]
        #[inline(always)]
        pub const fn set_colerrite(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
    }
    impl Default for Fite {
        #[inline(always)]
        fn default() -> Fite {
            Fite(0)
        }
    }
    impl core::fmt::Debug for Fite {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Fite")
                .field("peerrite", &self.peerrite())
                .field("optendite", &self.optendite())
                .field("colerrite", &self.colerrite())
                .finish()
        }
    }
    #[doc = "desc FPMTEW."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Fpmtew(pub u32);
    impl Fpmtew {
        #[doc = "desc FPMTEW."]
        #[must_use]
        #[inline(always)]
        pub const fn fpmtew(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0x0003_ffff;
            val as u32
        }
        #[doc = "desc FPMTEW."]
        #[inline(always)]
        pub const fn set_fpmtew(&mut self, val: u32) {
            self.0 = (self.0 & !(0x0003_ffff << 0usize)) | (((val as u32) & 0x0003_ffff) << 0usize);
        }
    }
    impl Default for Fpmtew {
        #[inline(always)]
        fn default() -> Fpmtew {
            Fpmtew(0)
        }
    }
    impl core::fmt::Debug for Fpmtew {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Fpmtew")
                .field("fpmtew", &self.fpmtew())
                .finish()
        }
    }
    #[doc = "desc FPMTSW."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Fpmtsw(pub u32);
    impl Fpmtsw {
        #[doc = "desc FPMTSW."]
        #[must_use]
        #[inline(always)]
        pub const fn fpmtsw(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0x0003_ffff;
            val as u32
        }
        #[doc = "desc FPMTSW."]
        #[inline(always)]
        pub const fn set_fpmtsw(&mut self, val: u32) {
            self.0 = (self.0 & !(0x0003_ffff << 0usize)) | (((val as u32) & 0x0003_ffff) << 0usize);
        }
    }
    impl Default for Fpmtsw {
        #[inline(always)]
        fn default() -> Fpmtsw {
            Fpmtsw(0)
        }
    }
    impl core::fmt::Debug for Fpmtsw {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Fpmtsw")
                .field("fpmtsw", &self.fpmtsw())
                .finish()
        }
    }
    #[doc = "desc FRMC."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Frmc(pub u32);
    impl Frmc {
        #[doc = "desc FLWT."]
        #[must_use]
        #[inline(always)]
        pub const fn flwt(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x03;
            val as u8
        }
        #[doc = "desc FLWT."]
        #[inline(always)]
        pub const fn set_flwt(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 0usize)) | (((val as u32) & 0x03) << 0usize);
        }
        #[doc = "desc PREFETE."]
        #[must_use]
        #[inline(always)]
        pub const fn prefete(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "desc PREFETE."]
        #[inline(always)]
        pub const fn set_prefete(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
    }
    impl Default for Frmc {
        #[inline(always)]
        fn default() -> Frmc {
            Frmc(0)
        }
    }
    impl core::fmt::Debug for Frmc {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Frmc")
                .field("flwt", &self.flwt())
                .field("prefete", &self.prefete())
                .finish()
        }
    }
    #[doc = "desc FSCLR."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Fsclr(pub u32);
    impl Fsclr {
        #[doc = "desc PEWERRCLR."]
        #[must_use]
        #[inline(always)]
        pub const fn pewerrclr(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "desc PEWERRCLR."]
        #[inline(always)]
        pub const fn set_pewerrclr(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "desc PEPRTERRCLR."]
        #[must_use]
        #[inline(always)]
        pub const fn peprterrclr(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "desc PEPRTERRCLR."]
        #[inline(always)]
        pub const fn set_peprterrclr(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "desc PGMISMTCHCLR."]
        #[must_use]
        #[inline(always)]
        pub const fn pgmismtchclr(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "desc PGMISMTCHCLR."]
        #[inline(always)]
        pub const fn set_pgmismtchclr(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "desc OPTENDCLR."]
        #[must_use]
        #[inline(always)]
        pub const fn optendclr(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "desc OPTENDCLR."]
        #[inline(always)]
        pub const fn set_optendclr(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "desc COLERRCLR."]
        #[must_use]
        #[inline(always)]
        pub const fn colerrclr(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "desc COLERRCLR."]
        #[inline(always)]
        pub const fn set_colerrclr(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "desc ERCALCLR."]
        #[must_use]
        #[inline(always)]
        pub const fn ercalclr(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "desc ERCALCLR."]
        #[inline(always)]
        pub const fn set_ercalclr(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
    }
    impl Default for Fsclr {
        #[inline(always)]
        fn default() -> Fsclr {
            Fsclr(0)
        }
    }
    impl core::fmt::Debug for Fsclr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Fsclr")
                .field("pewerrclr", &self.pewerrclr())
                .field("peprterrclr", &self.peprterrclr())
                .field("pgmismtchclr", &self.pgmismtchclr())
                .field("optendclr", &self.optendclr())
                .field("colerrclr", &self.colerrclr())
                .field("ercalclr", &self.ercalclr())
                .finish()
        }
    }
    #[doc = "desc FSR."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Fsr(pub u32);
    impl Fsr {
        #[doc = "desc PEWERR."]
        #[must_use]
        #[inline(always)]
        pub const fn pewerr(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "desc PEWERR."]
        #[inline(always)]
        pub const fn set_pewerr(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "desc PEPRTERR."]
        #[must_use]
        #[inline(always)]
        pub const fn peprterr(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "desc PEPRTERR."]
        #[inline(always)]
        pub const fn set_peprterr(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "desc PGMISMTCH."]
        #[must_use]
        #[inline(always)]
        pub const fn pgmismtch(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "desc PGMISMTCH."]
        #[inline(always)]
        pub const fn set_pgmismtch(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "desc OPTEND."]
        #[must_use]
        #[inline(always)]
        pub const fn optend(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "desc OPTEND."]
        #[inline(always)]
        pub const fn set_optend(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "desc COLERR."]
        #[must_use]
        #[inline(always)]
        pub const fn colerr(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "desc COLERR."]
        #[inline(always)]
        pub const fn set_colerr(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "desc ERCAL."]
        #[must_use]
        #[inline(always)]
        pub const fn ercal(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "desc ERCAL."]
        #[inline(always)]
        pub const fn set_ercal(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "desc RDY."]
        #[must_use]
        #[inline(always)]
        pub const fn rdy(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "desc RDY."]
        #[inline(always)]
        pub const fn set_rdy(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
    }
    impl Default for Fsr {
        #[inline(always)]
        fn default() -> Fsr {
            Fsr(0)
        }
    }
    impl core::fmt::Debug for Fsr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Fsr")
                .field("pewerr", &self.pewerr())
                .field("peprterr", &self.peprterr())
                .field("pgmismtch", &self.pgmismtch())
                .field("optend", &self.optend())
                .field("colerr", &self.colerr())
                .field("ercal", &self.ercal())
                .field("rdy", &self.rdy())
                .finish()
        }
    }
    #[doc = "desc FSTP."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Fstp(pub u32);
    impl Fstp {
        #[doc = "desc FSTP."]
        #[must_use]
        #[inline(always)]
        pub const fn fstp(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "desc FSTP."]
        #[inline(always)]
        pub const fn set_fstp(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
    }
    impl Default for Fstp {
        #[inline(always)]
        fn default() -> Fstp {
            Fstp(0)
        }
    }
    impl core::fmt::Debug for Fstp {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Fstp").field("fstp", &self.fstp()).finish()
        }
    }
    #[doc = "desc FWMC."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Fwmc(pub u32);
    impl Fwmc {
        #[doc = "desc PEMODE."]
        #[must_use]
        #[inline(always)]
        pub const fn pemode(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "desc PEMODE."]
        #[inline(always)]
        pub const fn set_pemode(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "desc PEMOD."]
        #[must_use]
        #[inline(always)]
        pub const fn pemod(&self) -> u8 {
            let val = (self.0 >> 4usize) & 0x07;
            val as u8
        }
        #[doc = "desc PEMOD."]
        #[inline(always)]
        pub const fn set_pemod(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 4usize)) | (((val as u32) & 0x07) << 4usize);
        }
        #[doc = "desc BUSHLDCTL."]
        #[must_use]
        #[inline(always)]
        pub const fn bushldctl(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "desc BUSHLDCTL."]
        #[inline(always)]
        pub const fn set_bushldctl(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "desc ERCALE."]
        #[must_use]
        #[inline(always)]
        pub const fn ercale(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "desc ERCALE."]
        #[inline(always)]
        pub const fn set_ercale(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
    }
    impl Default for Fwmc {
        #[inline(always)]
        fn default() -> Fwmc {
            Fwmc(0)
        }
    }
    impl core::fmt::Debug for Fwmc {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Fwmc")
                .field("pemode", &self.pemode())
                .field("pemod", &self.pemod())
                .field("bushldctl", &self.bushldctl())
                .field("ercale", &self.ercale())
                .finish()
        }
    }
    #[doc = "desc HRCCFGR."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Hrccfgr(pub u8);
    impl Hrccfgr {
        #[doc = "desc HRCFREQS."]
        #[must_use]
        #[inline(always)]
        pub const fn hrcfreqs(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x0f;
            val as u8
        }
        #[doc = "desc HRCFREQS."]
        #[inline(always)]
        pub const fn set_hrcfreqs(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u8) & 0x0f) << 0usize);
        }
    }
    impl Default for Hrccfgr {
        #[inline(always)]
        fn default() -> Hrccfgr {
            Hrccfgr(0)
        }
    }
    impl core::fmt::Debug for Hrccfgr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Hrccfgr")
                .field("hrcfreqs", &self.hrcfreqs())
                .finish()
        }
    }
    #[doc = "desc LVDICGCR."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Lvdicgcr(pub u16);
    impl Lvdicgcr {
        #[doc = "desc DFS."]
        #[must_use]
        #[inline(always)]
        pub const fn dfs(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x03;
            val as u8
        }
        #[doc = "desc DFS."]
        #[inline(always)]
        pub const fn set_dfs(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 0usize)) | (((val as u16) & 0x03) << 0usize);
        }
        #[doc = "desc DFDIS."]
        #[must_use]
        #[inline(always)]
        pub const fn dfdis(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "desc DFDIS."]
        #[inline(always)]
        pub const fn set_dfdis(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u16) & 0x01) << 2usize);
        }
        #[doc = "desc LVDLVL."]
        #[must_use]
        #[inline(always)]
        pub const fn lvdlvl(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0x0f;
            val as u8
        }
        #[doc = "desc LVDLVL."]
        #[inline(always)]
        pub const fn set_lvdlvl(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 8usize)) | (((val as u16) & 0x0f) << 8usize);
        }
        #[doc = "desc NMIS."]
        #[must_use]
        #[inline(always)]
        pub const fn nmis(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "desc NMIS."]
        #[inline(always)]
        pub const fn set_nmis(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u16) & 0x01) << 12usize);
        }
        #[doc = "desc IRS."]
        #[must_use]
        #[inline(always)]
        pub const fn irs(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[doc = "desc IRS."]
        #[inline(always)]
        pub const fn set_irs(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u16) & 0x01) << 13usize);
        }
        #[doc = "desc IRDIS."]
        #[must_use]
        #[inline(always)]
        pub const fn irdis(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "desc IRDIS."]
        #[inline(always)]
        pub const fn set_irdis(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u16) & 0x01) << 14usize);
        }
        #[doc = "desc LVDDIS."]
        #[must_use]
        #[inline(always)]
        pub const fn lvddis(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "desc LVDDIS."]
        #[inline(always)]
        pub const fn set_lvddis(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u16) & 0x01) << 15usize);
        }
    }
    impl Default for Lvdicgcr {
        #[inline(always)]
        fn default() -> Lvdicgcr {
            Lvdicgcr(0)
        }
    }
    impl core::fmt::Debug for Lvdicgcr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Lvdicgcr")
                .field("dfs", &self.dfs())
                .field("dfdis", &self.dfdis())
                .field("lvdlvl", &self.lvdlvl())
                .field("nmis", &self.nmis())
                .field("irs", &self.irs())
                .field("irdis", &self.irdis())
                .field("lvddis", &self.lvddis())
                .finish()
        }
    }
}
