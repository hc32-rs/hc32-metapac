#![allow(clippy::missing_safety_doc)]
#![allow(clippy::identity_op)]
#![allow(clippy::unnecessary_cast)]
#![allow(clippy::erasing_op)]

#[doc = "desc AOS."]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Aos {
    ptr: *mut u8,
}
unsafe impl Send for Aos {}
unsafe impl Sync for Aos {}
impl Aos {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "desc INTC_STRGCR."]
    #[inline(always)]
    pub const fn intc_strgcr(self) -> crate::common::Reg<regs::IntcStrgcr, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0usize) as _) }
    }
    #[doc = "desc EVPRT_CR."]
    #[inline(always)]
    pub const fn evprt_cr(self) -> crate::common::Reg<regs::EvprtCr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x04usize) as _) }
    }
    #[doc = "desc EVPRT_SR."]
    #[inline(always)]
    pub const fn evprt_sr(self) -> crate::common::Reg<regs::EvprtSr, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x08usize) as _) }
    }
    #[doc = "desc TMR0_HTSSR."]
    #[inline(always)]
    pub const fn tmr0_htssr(self) -> crate::common::Reg<regs::Tmr0Htssr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x20usize) as _) }
    }
    #[doc = "desc TMR2_HTSSR."]
    #[inline(always)]
    pub const fn tmr2_htssr(self) -> crate::common::Reg<regs::Tmr2Htssr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x20usize) as _) }
    }
    #[doc = "desc TMRA_HTSSR."]
    #[inline(always)]
    pub const fn tmra_htssr(self) -> crate::common::Reg<regs::TmraHtssr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x40usize) as _) }
    }
    #[doc = "desc TMRB_HTSSR."]
    #[inline(always)]
    pub const fn tmrb_htssr(self) -> crate::common::Reg<regs::TmrbHtssr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x50usize) as _) }
    }
    #[doc = "desc ADC_ITRGSELR0."]
    #[inline(always)]
    pub const fn adc_itrgselr0(self) -> crate::common::Reg<regs::AdcItrgselr0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x60usize) as _) }
    }
    #[doc = "desc ADC_ITRGSELR1."]
    #[inline(always)]
    pub const fn adc_itrgselr1(self) -> crate::common::Reg<regs::AdcItrgselr1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x64usize) as _) }
    }
    #[doc = "desc DMA_TRGSEL0."]
    #[inline(always)]
    pub const fn dma_trgsel0(self) -> crate::common::Reg<regs::DmaTrgsel0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x80usize) as _) }
    }
    #[doc = "desc DMA_TRGSEL1."]
    #[inline(always)]
    pub const fn dma_trgsel1(self) -> crate::common::Reg<regs::DmaTrgsel1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x84usize) as _) }
    }
}
pub mod regs {
    #[doc = "desc ADC_ITRGSELR0."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct AdcItrgselr0(pub u32);
    impl AdcItrgselr0 {
        #[doc = "desc TRGSEL."]
        #[must_use]
        #[inline(always)]
        pub const fn trgsel(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x7f;
            val as u8
        }
        #[doc = "desc TRGSEL."]
        #[inline(always)]
        pub const fn set_trgsel(&mut self, val: u8) {
            self.0 = (self.0 & !(0x7f << 0usize)) | (((val as u32) & 0x7f) << 0usize);
        }
    }
    impl Default for AdcItrgselr0 {
        #[inline(always)]
        fn default() -> AdcItrgselr0 {
            AdcItrgselr0(0)
        }
    }
    impl core::fmt::Debug for AdcItrgselr0 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("AdcItrgselr0")
                .field("trgsel", &self.trgsel())
                .finish()
        }
    }
    #[doc = "desc ADC_ITRGSELR1."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct AdcItrgselr1(pub u32);
    impl AdcItrgselr1 {
        #[doc = "desc TRGSEL."]
        #[must_use]
        #[inline(always)]
        pub const fn trgsel(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x7f;
            val as u8
        }
        #[doc = "desc TRGSEL."]
        #[inline(always)]
        pub const fn set_trgsel(&mut self, val: u8) {
            self.0 = (self.0 & !(0x7f << 0usize)) | (((val as u32) & 0x7f) << 0usize);
        }
    }
    impl Default for AdcItrgselr1 {
        #[inline(always)]
        fn default() -> AdcItrgselr1 {
            AdcItrgselr1(0)
        }
    }
    impl core::fmt::Debug for AdcItrgselr1 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("AdcItrgselr1")
                .field("trgsel", &self.trgsel())
                .finish()
        }
    }
    #[doc = "desc DMA_TRGSEL0."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct DmaTrgsel0(pub u32);
    impl DmaTrgsel0 {
        #[doc = "desc TRGSEL."]
        #[must_use]
        #[inline(always)]
        pub const fn trgsel(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x7f;
            val as u8
        }
        #[doc = "desc TRGSEL."]
        #[inline(always)]
        pub const fn set_trgsel(&mut self, val: u8) {
            self.0 = (self.0 & !(0x7f << 0usize)) | (((val as u32) & 0x7f) << 0usize);
        }
    }
    impl Default for DmaTrgsel0 {
        #[inline(always)]
        fn default() -> DmaTrgsel0 {
            DmaTrgsel0(0)
        }
    }
    impl core::fmt::Debug for DmaTrgsel0 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("DmaTrgsel0")
                .field("trgsel", &self.trgsel())
                .finish()
        }
    }
    #[doc = "desc DMA_TRGSEL1."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct DmaTrgsel1(pub u32);
    impl DmaTrgsel1 {
        #[doc = "desc TRGSEL."]
        #[must_use]
        #[inline(always)]
        pub const fn trgsel(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x7f;
            val as u8
        }
        #[doc = "desc TRGSEL."]
        #[inline(always)]
        pub const fn set_trgsel(&mut self, val: u8) {
            self.0 = (self.0 & !(0x7f << 0usize)) | (((val as u32) & 0x7f) << 0usize);
        }
    }
    impl Default for DmaTrgsel1 {
        #[inline(always)]
        fn default() -> DmaTrgsel1 {
            DmaTrgsel1(0)
        }
    }
    impl core::fmt::Debug for DmaTrgsel1 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("DmaTrgsel1")
                .field("trgsel", &self.trgsel())
                .finish()
        }
    }
    #[doc = "desc EVPRT_CR."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct EvprtCr(pub u32);
    impl EvprtCr {
        #[doc = "desc EVP1FEE."]
        #[must_use]
        #[inline(always)]
        pub const fn evp1fee(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "desc EVP1FEE."]
        #[inline(always)]
        pub const fn set_evp1fee(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "desc EVP2FEE."]
        #[must_use]
        #[inline(always)]
        pub const fn evp2fee(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "desc EVP2FEE."]
        #[inline(always)]
        pub const fn set_evp2fee(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "desc EVP3FEE."]
        #[must_use]
        #[inline(always)]
        pub const fn evp3fee(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "desc EVP3FEE."]
        #[inline(always)]
        pub const fn set_evp3fee(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "desc EVP4FEE."]
        #[must_use]
        #[inline(always)]
        pub const fn evp4fee(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "desc EVP4FEE."]
        #[inline(always)]
        pub const fn set_evp4fee(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "desc EVP1REE."]
        #[must_use]
        #[inline(always)]
        pub const fn evp1ree(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "desc EVP1REE."]
        #[inline(always)]
        pub const fn set_evp1ree(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "desc EVP2REE."]
        #[must_use]
        #[inline(always)]
        pub const fn evp2ree(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "desc EVP2REE."]
        #[inline(always)]
        pub const fn set_evp2ree(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "desc EVP3REE."]
        #[must_use]
        #[inline(always)]
        pub const fn evp3ree(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "desc EVP3REE."]
        #[inline(always)]
        pub const fn set_evp3ree(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "desc EVP4REE."]
        #[must_use]
        #[inline(always)]
        pub const fn evp4ree(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "desc EVP4REE."]
        #[inline(always)]
        pub const fn set_evp4ree(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "desc EVPDIVS."]
        #[must_use]
        #[inline(always)]
        pub const fn evpdivs(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0x03;
            val as u8
        }
        #[doc = "desc EVPDIVS."]
        #[inline(always)]
        pub const fn set_evpdivs(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 8usize)) | (((val as u32) & 0x03) << 8usize);
        }
        #[doc = "desc EVPNFE."]
        #[must_use]
        #[inline(always)]
        pub const fn evpnfe(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "desc EVPNFE."]
        #[inline(always)]
        pub const fn set_evpnfe(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
    }
    impl Default for EvprtCr {
        #[inline(always)]
        fn default() -> EvprtCr {
            EvprtCr(0)
        }
    }
    impl core::fmt::Debug for EvprtCr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("EvprtCr")
                .field("evp1fee", &self.evp1fee())
                .field("evp2fee", &self.evp2fee())
                .field("evp3fee", &self.evp3fee())
                .field("evp4fee", &self.evp4fee())
                .field("evp1ree", &self.evp1ree())
                .field("evp2ree", &self.evp2ree())
                .field("evp3ree", &self.evp3ree())
                .field("evp4ree", &self.evp4ree())
                .field("evpdivs", &self.evpdivs())
                .field("evpnfe", &self.evpnfe())
                .finish()
        }
    }
    #[doc = "desc EVPRT_SR."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct EvprtSr(pub u32);
    impl EvprtSr {
        #[doc = "desc PTSTS0."]
        #[must_use]
        #[inline(always)]
        pub const fn ptsts0(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x0f;
            val as u8
        }
        #[doc = "desc PTSTS0."]
        #[inline(always)]
        pub const fn set_ptsts0(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
        }
        #[doc = "desc PTSTS1."]
        #[must_use]
        #[inline(always)]
        pub const fn ptsts1(&self) -> u8 {
            let val = (self.0 >> 4usize) & 0x0f;
            val as u8
        }
        #[doc = "desc PTSTS1."]
        #[inline(always)]
        pub const fn set_ptsts1(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 4usize)) | (((val as u32) & 0x0f) << 4usize);
        }
        #[doc = "desc PTSTS2."]
        #[must_use]
        #[inline(always)]
        pub const fn ptsts2(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0x0f;
            val as u8
        }
        #[doc = "desc PTSTS2."]
        #[inline(always)]
        pub const fn set_ptsts2(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 8usize)) | (((val as u32) & 0x0f) << 8usize);
        }
        #[doc = "desc PTSTS3."]
        #[must_use]
        #[inline(always)]
        pub const fn ptsts3(&self) -> u8 {
            let val = (self.0 >> 12usize) & 0x0f;
            val as u8
        }
        #[doc = "desc PTSTS3."]
        #[inline(always)]
        pub const fn set_ptsts3(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 12usize)) | (((val as u32) & 0x0f) << 12usize);
        }
    }
    impl Default for EvprtSr {
        #[inline(always)]
        fn default() -> EvprtSr {
            EvprtSr(0)
        }
    }
    impl core::fmt::Debug for EvprtSr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("EvprtSr")
                .field("ptsts0", &self.ptsts0())
                .field("ptsts1", &self.ptsts1())
                .field("ptsts2", &self.ptsts2())
                .field("ptsts3", &self.ptsts3())
                .finish()
        }
    }
    #[doc = "desc INTC_STRGCR."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct IntcStrgcr(pub u32);
    impl IntcStrgcr {
        #[doc = "desc STRG."]
        #[must_use]
        #[inline(always)]
        pub const fn strg(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "desc STRG."]
        #[inline(always)]
        pub const fn set_strg(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
    }
    impl Default for IntcStrgcr {
        #[inline(always)]
        fn default() -> IntcStrgcr {
            IntcStrgcr(0)
        }
    }
    impl core::fmt::Debug for IntcStrgcr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("IntcStrgcr")
                .field("strg", &self.strg())
                .finish()
        }
    }
    #[doc = "desc TMR0_HTSSR."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Tmr0Htssr(pub u32);
    impl Tmr0Htssr {
        #[doc = "desc TRGSEL."]
        #[must_use]
        #[inline(always)]
        pub const fn trgsel(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x7f;
            val as u8
        }
        #[doc = "desc TRGSEL."]
        #[inline(always)]
        pub const fn set_trgsel(&mut self, val: u8) {
            self.0 = (self.0 & !(0x7f << 0usize)) | (((val as u32) & 0x7f) << 0usize);
        }
    }
    impl Default for Tmr0Htssr {
        #[inline(always)]
        fn default() -> Tmr0Htssr {
            Tmr0Htssr(0)
        }
    }
    impl core::fmt::Debug for Tmr0Htssr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Tmr0Htssr")
                .field("trgsel", &self.trgsel())
                .finish()
        }
    }
    #[doc = "desc TMR2_HTSSR."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Tmr2Htssr(pub u32);
    impl Tmr2Htssr {
        #[doc = "desc TRGSEL."]
        #[must_use]
        #[inline(always)]
        pub const fn trgsel(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x7f;
            val as u8
        }
        #[doc = "desc TRGSEL."]
        #[inline(always)]
        pub const fn set_trgsel(&mut self, val: u8) {
            self.0 = (self.0 & !(0x7f << 0usize)) | (((val as u32) & 0x7f) << 0usize);
        }
    }
    impl Default for Tmr2Htssr {
        #[inline(always)]
        fn default() -> Tmr2Htssr {
            Tmr2Htssr(0)
        }
    }
    impl core::fmt::Debug for Tmr2Htssr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Tmr2Htssr")
                .field("trgsel", &self.trgsel())
                .finish()
        }
    }
    #[doc = "desc TMRA_HTSSR."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct TmraHtssr(pub u32);
    impl TmraHtssr {
        #[doc = "desc TRGSEL."]
        #[must_use]
        #[inline(always)]
        pub const fn trgsel(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x7f;
            val as u8
        }
        #[doc = "desc TRGSEL."]
        #[inline(always)]
        pub const fn set_trgsel(&mut self, val: u8) {
            self.0 = (self.0 & !(0x7f << 0usize)) | (((val as u32) & 0x7f) << 0usize);
        }
    }
    impl Default for TmraHtssr {
        #[inline(always)]
        fn default() -> TmraHtssr {
            TmraHtssr(0)
        }
    }
    impl core::fmt::Debug for TmraHtssr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("TmraHtssr")
                .field("trgsel", &self.trgsel())
                .finish()
        }
    }
    #[doc = "desc TMRB_HTSSR."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct TmrbHtssr(pub u32);
    impl TmrbHtssr {
        #[doc = "desc TRGSEL."]
        #[must_use]
        #[inline(always)]
        pub const fn trgsel(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x7f;
            val as u8
        }
        #[doc = "desc TRGSEL."]
        #[inline(always)]
        pub const fn set_trgsel(&mut self, val: u8) {
            self.0 = (self.0 & !(0x7f << 0usize)) | (((val as u32) & 0x7f) << 0usize);
        }
    }
    impl Default for TmrbHtssr {
        #[inline(always)]
        fn default() -> TmrbHtssr {
            TmrbHtssr(0)
        }
    }
    impl core::fmt::Debug for TmrbHtssr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("TmrbHtssr")
                .field("trgsel", &self.trgsel())
                .finish()
        }
    }
}
