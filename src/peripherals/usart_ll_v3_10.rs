#![allow(clippy::missing_safety_doc)]
#![allow(clippy::identity_op)]
#![allow(clippy::unnecessary_cast)]
#![allow(clippy::erasing_op)]

#[doc = "desc USART."]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Usart {
    ptr: *mut u8,
}
unsafe impl Send for Usart {}
unsafe impl Sync for Usart {}
impl Usart {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "desc SR."]
    #[inline(always)]
    pub const fn sr(self) -> crate::common::Reg<regs::Sr, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0usize) as _) }
    }
    #[doc = "desc TDR."]
    #[inline(always)]
    pub const fn tdr(self) -> crate::common::Reg<regs::Tdr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x04usize) as _) }
    }
    #[doc = "desc RDR."]
    #[inline(always)]
    pub const fn rdr(self) -> crate::common::Reg<regs::Rdr, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x06usize) as _) }
    }
    #[doc = "desc BRR."]
    #[inline(always)]
    pub const fn brr(self) -> crate::common::Reg<regs::Brr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x08usize) as _) }
    }
    #[doc = "desc CR1."]
    #[inline(always)]
    pub const fn cr1(self) -> crate::common::Reg<regs::Cr1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0cusize) as _) }
    }
    #[doc = "desc CR2."]
    #[inline(always)]
    pub const fn cr2(self) -> crate::common::Reg<regs::Cr2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x10usize) as _) }
    }
    #[doc = "desc CR3."]
    #[inline(always)]
    pub const fn cr3(self) -> crate::common::Reg<regs::Cr3, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x14usize) as _) }
    }
    #[doc = "desc PR."]
    #[inline(always)]
    pub const fn pr(self) -> crate::common::Reg<regs::Pr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x18usize) as _) }
    }
}
pub mod regs {
    #[doc = "desc BRR."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Brr(pub u32);
    impl Brr {
        #[doc = "desc DIV_FRACTION."]
        #[must_use]
        #[inline(always)]
        pub const fn div_fraction(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x7f;
            val as u8
        }
        #[doc = "desc DIV_FRACTION."]
        #[inline(always)]
        pub const fn set_div_fraction(&mut self, val: u8) {
            self.0 = (self.0 & !(0x7f << 0usize)) | (((val as u32) & 0x7f) << 0usize);
        }
        #[doc = "desc DIV_INTEGER."]
        #[must_use]
        #[inline(always)]
        pub const fn div_integer(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0xff;
            val as u8
        }
        #[doc = "desc DIV_INTEGER."]
        #[inline(always)]
        pub const fn set_div_integer(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
        }
    }
    impl Default for Brr {
        #[inline(always)]
        fn default() -> Brr {
            Brr(0)
        }
    }
    impl core::fmt::Debug for Brr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Brr")
                .field("div_fraction", &self.div_fraction())
                .field("div_integer", &self.div_integer())
                .finish()
        }
    }
    #[doc = "desc CR1."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Cr1(pub u32);
    impl Cr1 {
        #[doc = "desc RTOE."]
        #[must_use]
        #[inline(always)]
        pub const fn rtoe(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "desc RTOE."]
        #[inline(always)]
        pub const fn set_rtoe(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "desc RTOIE."]
        #[must_use]
        #[inline(always)]
        pub const fn rtoie(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "desc RTOIE."]
        #[inline(always)]
        pub const fn set_rtoie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "desc RE."]
        #[must_use]
        #[inline(always)]
        pub const fn re(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "desc RE."]
        #[inline(always)]
        pub const fn set_re(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "desc TE."]
        #[must_use]
        #[inline(always)]
        pub const fn te(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "desc TE."]
        #[inline(always)]
        pub const fn set_te(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "desc SLME."]
        #[must_use]
        #[inline(always)]
        pub const fn slme(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "desc SLME."]
        #[inline(always)]
        pub const fn set_slme(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "desc RIE."]
        #[must_use]
        #[inline(always)]
        pub const fn rie(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "desc RIE."]
        #[inline(always)]
        pub const fn set_rie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "desc TCIE."]
        #[must_use]
        #[inline(always)]
        pub const fn tcie(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "desc TCIE."]
        #[inline(always)]
        pub const fn set_tcie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "desc TXEIE."]
        #[must_use]
        #[inline(always)]
        pub const fn txeie(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "desc TXEIE."]
        #[inline(always)]
        pub const fn set_txeie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "desc TENDIE."]
        #[must_use]
        #[inline(always)]
        pub const fn tendie(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "desc TENDIE."]
        #[inline(always)]
        pub const fn set_tendie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "desc PS."]
        #[must_use]
        #[inline(always)]
        pub const fn ps(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "desc PS."]
        #[inline(always)]
        pub const fn set_ps(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "desc PCE."]
        #[must_use]
        #[inline(always)]
        pub const fn pce(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "desc PCE."]
        #[inline(always)]
        pub const fn set_pce(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[doc = "desc M."]
        #[must_use]
        #[inline(always)]
        pub const fn m(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "desc M."]
        #[inline(always)]
        pub const fn set_m(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "desc OVER8."]
        #[must_use]
        #[inline(always)]
        pub const fn over8(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "desc OVER8."]
        #[inline(always)]
        pub const fn set_over8(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
        #[doc = "desc CPE."]
        #[must_use]
        #[inline(always)]
        pub const fn cpe(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "desc CPE."]
        #[inline(always)]
        pub const fn set_cpe(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "desc CFE."]
        #[must_use]
        #[inline(always)]
        pub const fn cfe(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "desc CFE."]
        #[inline(always)]
        pub const fn set_cfe(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[doc = "desc CORE."]
        #[must_use]
        #[inline(always)]
        pub const fn core(&self) -> bool {
            let val = (self.0 >> 19usize) & 0x01;
            val != 0
        }
        #[doc = "desc CORE."]
        #[inline(always)]
        pub const fn set_core(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
        }
        #[doc = "desc CRTOF."]
        #[must_use]
        #[inline(always)]
        pub const fn crtof(&self) -> bool {
            let val = (self.0 >> 20usize) & 0x01;
            val != 0
        }
        #[doc = "desc CRTOF."]
        #[inline(always)]
        pub const fn set_crtof(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
        }
        #[doc = "desc MS."]
        #[must_use]
        #[inline(always)]
        pub const fn ms(&self) -> bool {
            let val = (self.0 >> 24usize) & 0x01;
            val != 0
        }
        #[doc = "desc MS."]
        #[inline(always)]
        pub const fn set_ms(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
        }
        #[doc = "desc CTEND."]
        #[must_use]
        #[inline(always)]
        pub const fn ctend(&self) -> bool {
            let val = (self.0 >> 25usize) & 0x01;
            val != 0
        }
        #[doc = "desc CTEND."]
        #[inline(always)]
        pub const fn set_ctend(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
        }
        #[doc = "desc ML."]
        #[must_use]
        #[inline(always)]
        pub const fn ml(&self) -> bool {
            let val = (self.0 >> 28usize) & 0x01;
            val != 0
        }
        #[doc = "desc ML."]
        #[inline(always)]
        pub const fn set_ml(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
        }
        #[doc = "desc FBME."]
        #[must_use]
        #[inline(always)]
        pub const fn fbme(&self) -> bool {
            let val = (self.0 >> 29usize) & 0x01;
            val != 0
        }
        #[doc = "desc FBME."]
        #[inline(always)]
        pub const fn set_fbme(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
        }
        #[doc = "desc NFE."]
        #[must_use]
        #[inline(always)]
        pub const fn nfe(&self) -> bool {
            let val = (self.0 >> 30usize) & 0x01;
            val != 0
        }
        #[doc = "desc NFE."]
        #[inline(always)]
        pub const fn set_nfe(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
        }
        #[doc = "desc SBS."]
        #[must_use]
        #[inline(always)]
        pub const fn sbs(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[doc = "desc SBS."]
        #[inline(always)]
        pub const fn set_sbs(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for Cr1 {
        #[inline(always)]
        fn default() -> Cr1 {
            Cr1(0)
        }
    }
    impl core::fmt::Debug for Cr1 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Cr1")
                .field("rtoe", &self.rtoe())
                .field("rtoie", &self.rtoie())
                .field("re", &self.re())
                .field("te", &self.te())
                .field("slme", &self.slme())
                .field("rie", &self.rie())
                .field("tcie", &self.tcie())
                .field("txeie", &self.txeie())
                .field("tendie", &self.tendie())
                .field("ps", &self.ps())
                .field("pce", &self.pce())
                .field("m", &self.m())
                .field("over8", &self.over8())
                .field("cpe", &self.cpe())
                .field("cfe", &self.cfe())
                .field("core", &self.core())
                .field("crtof", &self.crtof())
                .field("ms", &self.ms())
                .field("ctend", &self.ctend())
                .field("ml", &self.ml())
                .field("fbme", &self.fbme())
                .field("nfe", &self.nfe())
                .field("sbs", &self.sbs())
                .finish()
        }
    }
    #[doc = "desc CR2."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Cr2(pub u32);
    impl Cr2 {
        #[doc = "desc MPE."]
        #[must_use]
        #[inline(always)]
        pub const fn mpe(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "desc MPE."]
        #[inline(always)]
        pub const fn set_mpe(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "desc CLKC."]
        #[must_use]
        #[inline(always)]
        pub const fn clkc(&self) -> u8 {
            let val = (self.0 >> 11usize) & 0x03;
            val as u8
        }
        #[doc = "desc CLKC."]
        #[inline(always)]
        pub const fn set_clkc(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 11usize)) | (((val as u32) & 0x03) << 11usize);
        }
        #[doc = "desc STOP."]
        #[must_use]
        #[inline(always)]
        pub const fn stop(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[doc = "desc STOP."]
        #[inline(always)]
        pub const fn set_stop(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
    }
    impl Default for Cr2 {
        #[inline(always)]
        fn default() -> Cr2 {
            Cr2(0)
        }
    }
    impl core::fmt::Debug for Cr2 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Cr2")
                .field("mpe", &self.mpe())
                .field("clkc", &self.clkc())
                .field("stop", &self.stop())
                .finish()
        }
    }
    #[doc = "desc CR3."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Cr3(pub u32);
    impl Cr3 {
        #[doc = "desc HDSEL."]
        #[must_use]
        #[inline(always)]
        pub const fn hdsel(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "desc HDSEL."]
        #[inline(always)]
        pub const fn set_hdsel(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "desc SCEN."]
        #[must_use]
        #[inline(always)]
        pub const fn scen(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "desc SCEN."]
        #[inline(always)]
        pub const fn set_scen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "desc RTSE."]
        #[must_use]
        #[inline(always)]
        pub const fn rtse(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "desc RTSE."]
        #[inline(always)]
        pub const fn set_rtse(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "desc CTSE."]
        #[must_use]
        #[inline(always)]
        pub const fn ctse(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "desc CTSE."]
        #[inline(always)]
        pub const fn set_ctse(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "desc DED."]
        #[must_use]
        #[inline(always)]
        pub const fn ded(&self) -> u8 {
            let val = (self.0 >> 10usize) & 0x0f;
            val as u8
        }
        #[doc = "desc DED."]
        #[inline(always)]
        pub const fn set_ded(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 10usize)) | (((val as u32) & 0x0f) << 10usize);
        }
        #[doc = "desc TDS."]
        #[must_use]
        #[inline(always)]
        pub const fn tds(&self) -> u8 {
            let val = (self.0 >> 14usize) & 0x03;
            val as u8
        }
        #[doc = "desc TDS."]
        #[inline(always)]
        pub const fn set_tds(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 14usize)) | (((val as u32) & 0x03) << 14usize);
        }
        #[doc = "desc BCN."]
        #[must_use]
        #[inline(always)]
        pub const fn bcn(&self) -> u8 {
            let val = (self.0 >> 21usize) & 0x07;
            val as u8
        }
        #[doc = "desc BCN."]
        #[inline(always)]
        pub const fn set_bcn(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 21usize)) | (((val as u32) & 0x07) << 21usize);
        }
        #[doc = "desc DEA."]
        #[must_use]
        #[inline(always)]
        pub const fn dea(&self) -> u8 {
            let val = (self.0 >> 25usize) & 0x0f;
            val as u8
        }
        #[doc = "desc DEA."]
        #[inline(always)]
        pub const fn set_dea(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 25usize)) | (((val as u32) & 0x0f) << 25usize);
        }
        #[doc = "desc DEM."]
        #[must_use]
        #[inline(always)]
        pub const fn dem(&self) -> bool {
            let val = (self.0 >> 30usize) & 0x01;
            val != 0
        }
        #[doc = "desc DEM."]
        #[inline(always)]
        pub const fn set_dem(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
        }
        #[doc = "desc DEP."]
        #[must_use]
        #[inline(always)]
        pub const fn dep(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[doc = "desc DEP."]
        #[inline(always)]
        pub const fn set_dep(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for Cr3 {
        #[inline(always)]
        fn default() -> Cr3 {
            Cr3(0)
        }
    }
    impl core::fmt::Debug for Cr3 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Cr3")
                .field("hdsel", &self.hdsel())
                .field("scen", &self.scen())
                .field("rtse", &self.rtse())
                .field("ctse", &self.ctse())
                .field("ded", &self.ded())
                .field("tds", &self.tds())
                .field("bcn", &self.bcn())
                .field("dea", &self.dea())
                .field("dem", &self.dem())
                .field("dep", &self.dep())
                .finish()
        }
    }
    #[doc = "desc PR."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Pr(pub u32);
    impl Pr {
        #[doc = "desc PSC."]
        #[must_use]
        #[inline(always)]
        pub const fn psc(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x03;
            val as u8
        }
        #[doc = "desc PSC."]
        #[inline(always)]
        pub const fn set_psc(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 0usize)) | (((val as u32) & 0x03) << 0usize);
        }
        #[doc = "desc ULBREN."]
        #[must_use]
        #[inline(always)]
        pub const fn ulbren(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "desc ULBREN."]
        #[inline(always)]
        pub const fn set_ulbren(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "desc RTOB."]
        #[must_use]
        #[inline(always)]
        pub const fn rtob(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0x0f;
            val as u8
        }
        #[doc = "desc RTOB."]
        #[inline(always)]
        pub const fn set_rtob(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 8usize)) | (((val as u32) & 0x0f) << 8usize);
        }
    }
    impl Default for Pr {
        #[inline(always)]
        fn default() -> Pr {
            Pr(0)
        }
    }
    impl core::fmt::Debug for Pr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Pr")
                .field("psc", &self.psc())
                .field("ulbren", &self.ulbren())
                .field("rtob", &self.rtob())
                .finish()
        }
    }
    #[doc = "desc RDR."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Rdr(pub u16);
    impl Rdr {
        #[doc = "desc RDR."]
        #[must_use]
        #[inline(always)]
        pub const fn rdr(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x01ff;
            val as u16
        }
        #[doc = "desc RDR."]
        #[inline(always)]
        pub const fn set_rdr(&mut self, val: u16) {
            self.0 = (self.0 & !(0x01ff << 0usize)) | (((val as u16) & 0x01ff) << 0usize);
        }
    }
    impl Default for Rdr {
        #[inline(always)]
        fn default() -> Rdr {
            Rdr(0)
        }
    }
    impl core::fmt::Debug for Rdr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Rdr").field("rdr", &self.rdr()).finish()
        }
    }
    #[doc = "desc SR."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Sr(pub u32);
    impl Sr {
        #[doc = "desc PE."]
        #[must_use]
        #[inline(always)]
        pub const fn pe(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "desc PE."]
        #[inline(always)]
        pub const fn set_pe(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "desc FE."]
        #[must_use]
        #[inline(always)]
        pub const fn fe(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "desc FE."]
        #[inline(always)]
        pub const fn set_fe(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "desc ORE."]
        #[must_use]
        #[inline(always)]
        pub const fn ore(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "desc ORE."]
        #[inline(always)]
        pub const fn set_ore(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "desc RXNE."]
        #[must_use]
        #[inline(always)]
        pub const fn rxne(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "desc RXNE."]
        #[inline(always)]
        pub const fn set_rxne(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "desc TC."]
        #[must_use]
        #[inline(always)]
        pub const fn tc(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "desc TC."]
        #[inline(always)]
        pub const fn set_tc(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "desc TXE."]
        #[must_use]
        #[inline(always)]
        pub const fn txe(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "desc TXE."]
        #[inline(always)]
        pub const fn set_txe(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "desc RTOF."]
        #[must_use]
        #[inline(always)]
        pub const fn rtof(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "desc RTOF."]
        #[inline(always)]
        pub const fn set_rtof(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "desc TEND."]
        #[must_use]
        #[inline(always)]
        pub const fn tend(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "desc TEND."]
        #[inline(always)]
        pub const fn set_tend(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[doc = "desc MPB."]
        #[must_use]
        #[inline(always)]
        pub const fn mpb(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "desc MPB."]
        #[inline(always)]
        pub const fn set_mpb(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
    }
    impl Default for Sr {
        #[inline(always)]
        fn default() -> Sr {
            Sr(0)
        }
    }
    impl core::fmt::Debug for Sr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Sr")
                .field("pe", &self.pe())
                .field("fe", &self.fe())
                .field("ore", &self.ore())
                .field("rxne", &self.rxne())
                .field("tc", &self.tc())
                .field("txe", &self.txe())
                .field("rtof", &self.rtof())
                .field("tend", &self.tend())
                .field("mpb", &self.mpb())
                .finish()
        }
    }
    #[doc = "desc TDR."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Tdr(pub u16);
    impl Tdr {
        #[doc = "desc TDR."]
        #[must_use]
        #[inline(always)]
        pub const fn tdr(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x01ff;
            val as u16
        }
        #[doc = "desc TDR."]
        #[inline(always)]
        pub const fn set_tdr(&mut self, val: u16) {
            self.0 = (self.0 & !(0x01ff << 0usize)) | (((val as u16) & 0x01ff) << 0usize);
        }
        #[doc = "desc MPID."]
        #[must_use]
        #[inline(always)]
        pub const fn mpid(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "desc MPID."]
        #[inline(always)]
        pub const fn set_mpid(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u16) & 0x01) << 9usize);
        }
    }
    impl Default for Tdr {
        #[inline(always)]
        fn default() -> Tdr {
            Tdr(0)
        }
    }
    impl core::fmt::Debug for Tdr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Tdr")
                .field("tdr", &self.tdr())
                .field("mpid", &self.mpid())
                .finish()
        }
    }
}
