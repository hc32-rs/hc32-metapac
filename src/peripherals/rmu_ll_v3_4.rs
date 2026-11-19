#![allow(clippy::missing_safety_doc)]
#![allow(clippy::identity_op)]
#![allow(clippy::unnecessary_cast)]
#![allow(clippy::erasing_op)]

#[doc = "desc RMU."]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Rmu {
    ptr: *mut u8,
}
unsafe impl Send for Rmu {}
unsafe impl Sync for Rmu {}
impl Rmu {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "desc FRST0."]
    #[inline(always)]
    pub const fn frst0(self) -> crate::common::Reg<regs::Frst0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0usize) as _) }
    }
    #[doc = "desc FRST1."]
    #[inline(always)]
    pub const fn frst1(self) -> crate::common::Reg<regs::Frst1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x04usize) as _) }
    }
    #[doc = "desc FRST2."]
    #[inline(always)]
    pub const fn frst2(self) -> crate::common::Reg<regs::Frst2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x08usize) as _) }
    }
    #[doc = "desc FRST3."]
    #[inline(always)]
    pub const fn frst3(self) -> crate::common::Reg<regs::Frst3, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0cusize) as _) }
    }
    #[doc = "desc PRSTCR0."]
    #[inline(always)]
    pub const fn prstcr0(self) -> crate::common::Reg<regs::Prstcr0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x10usize) as _) }
    }
    #[doc = "desc RSTF0."]
    #[inline(always)]
    pub const fn rstf0(self) -> crate::common::Reg<regs::Rstf0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x14usize) as _) }
    }
}
pub mod regs {
    #[doc = "desc FRST0."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Frst0(pub u32);
    impl Frst0 {
        #[doc = "desc PLA."]
        #[must_use]
        #[inline(always)]
        pub const fn pla(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "desc PLA."]
        #[inline(always)]
        pub const fn set_pla(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[doc = "desc KEY."]
        #[must_use]
        #[inline(always)]
        pub const fn key(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[doc = "desc KEY."]
        #[inline(always)]
        pub const fn set_key(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[doc = "desc DMA1."]
        #[must_use]
        #[inline(always)]
        pub const fn dma1(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "desc DMA1."]
        #[inline(always)]
        pub const fn set_dma1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[doc = "desc DMA2."]
        #[must_use]
        #[inline(always)]
        pub const fn dma2(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "desc DMA2."]
        #[inline(always)]
        pub const fn set_dma2(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
        #[doc = "desc FCM."]
        #[must_use]
        #[inline(always)]
        pub const fn fcm(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "desc FCM."]
        #[inline(always)]
        pub const fn set_fcm(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "desc AOS."]
        #[must_use]
        #[inline(always)]
        pub const fn aos(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "desc AOS."]
        #[inline(always)]
        pub const fn set_aos(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[doc = "desc CTC."]
        #[must_use]
        #[inline(always)]
        pub const fn ctc(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[doc = "desc CTC."]
        #[inline(always)]
        pub const fn set_ctc(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
        #[doc = "desc MAU."]
        #[must_use]
        #[inline(always)]
        pub const fn mau(&self) -> bool {
            let val = (self.0 >> 19usize) & 0x01;
            val != 0
        }
        #[doc = "desc MAU."]
        #[inline(always)]
        pub const fn set_mau(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
        }
        #[doc = "desc AES."]
        #[must_use]
        #[inline(always)]
        pub const fn aes(&self) -> bool {
            let val = (self.0 >> 20usize) & 0x01;
            val != 0
        }
        #[doc = "desc AES."]
        #[inline(always)]
        pub const fn set_aes(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
        }
        #[doc = "desc HASH."]
        #[must_use]
        #[inline(always)]
        pub const fn hash(&self) -> bool {
            let val = (self.0 >> 21usize) & 0x01;
            val != 0
        }
        #[doc = "desc HASH."]
        #[inline(always)]
        pub const fn set_hash(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
        }
        #[doc = "desc TRNG."]
        #[must_use]
        #[inline(always)]
        pub const fn trng(&self) -> bool {
            let val = (self.0 >> 22usize) & 0x01;
            val != 0
        }
        #[doc = "desc TRNG."]
        #[inline(always)]
        pub const fn set_trng(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
        }
        #[doc = "desc CRC."]
        #[must_use]
        #[inline(always)]
        pub const fn crc(&self) -> bool {
            let val = (self.0 >> 23usize) & 0x01;
            val != 0
        }
        #[doc = "desc CRC."]
        #[inline(always)]
        pub const fn set_crc(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
        }
        #[doc = "desc DCU1."]
        #[must_use]
        #[inline(always)]
        pub const fn dcu1(&self) -> bool {
            let val = (self.0 >> 24usize) & 0x01;
            val != 0
        }
        #[doc = "desc DCU1."]
        #[inline(always)]
        pub const fn set_dcu1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
        }
        #[doc = "desc DCU2."]
        #[must_use]
        #[inline(always)]
        pub const fn dcu2(&self) -> bool {
            let val = (self.0 >> 25usize) & 0x01;
            val != 0
        }
        #[doc = "desc DCU2."]
        #[inline(always)]
        pub const fn set_dcu2(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
        }
        #[doc = "desc DCU3."]
        #[must_use]
        #[inline(always)]
        pub const fn dcu3(&self) -> bool {
            let val = (self.0 >> 26usize) & 0x01;
            val != 0
        }
        #[doc = "desc DCU3."]
        #[inline(always)]
        pub const fn set_dcu3(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
        }
    }
    impl Default for Frst0 {
        #[inline(always)]
        fn default() -> Frst0 {
            Frst0(0)
        }
    }
    impl core::fmt::Debug for Frst0 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Frst0")
                .field("pla", &self.pla())
                .field("key", &self.key())
                .field("dma1", &self.dma1())
                .field("dma2", &self.dma2())
                .field("fcm", &self.fcm())
                .field("aos", &self.aos())
                .field("ctc", &self.ctc())
                .field("mau", &self.mau())
                .field("aes", &self.aes())
                .field("hash", &self.hash())
                .field("trng", &self.trng())
                .field("crc", &self.crc())
                .field("dcu1", &self.dcu1())
                .field("dcu2", &self.dcu2())
                .field("dcu3", &self.dcu3())
                .finish()
        }
    }
    #[doc = "desc FRST1."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Frst1(pub u32);
    impl Frst1 {
        #[doc = "desc QSPI."]
        #[must_use]
        #[inline(always)]
        pub const fn qspi(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "desc QSPI."]
        #[inline(always)]
        pub const fn set_qspi(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "desc MDIO."]
        #[must_use]
        #[inline(always)]
        pub const fn mdio(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "desc MDIO."]
        #[inline(always)]
        pub const fn set_mdio(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "desc SPI1."]
        #[must_use]
        #[inline(always)]
        pub const fn spi1(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "desc SPI1."]
        #[inline(always)]
        pub const fn set_spi1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "desc SPI2."]
        #[must_use]
        #[inline(always)]
        pub const fn spi2(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "desc SPI2."]
        #[inline(always)]
        pub const fn set_spi2(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[doc = "desc SPI3."]
        #[must_use]
        #[inline(always)]
        pub const fn spi3(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[doc = "desc SPI3."]
        #[inline(always)]
        pub const fn set_spi3(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
        #[doc = "desc SPI4."]
        #[must_use]
        #[inline(always)]
        pub const fn spi4(&self) -> bool {
            let val = (self.0 >> 19usize) & 0x01;
            val != 0
        }
        #[doc = "desc SPI4."]
        #[inline(always)]
        pub const fn set_spi4(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
        }
        #[doc = "desc USBFS."]
        #[must_use]
        #[inline(always)]
        pub const fn usbfs(&self) -> bool {
            let val = (self.0 >> 22usize) & 0x01;
            val != 0
        }
        #[doc = "desc USBFS."]
        #[inline(always)]
        pub const fn set_usbfs(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
        }
        #[doc = "desc FMAC1."]
        #[must_use]
        #[inline(always)]
        pub const fn fmac1(&self) -> bool {
            let val = (self.0 >> 24usize) & 0x01;
            val != 0
        }
        #[doc = "desc FMAC1."]
        #[inline(always)]
        pub const fn set_fmac1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
        }
        #[doc = "desc FMAC2."]
        #[must_use]
        #[inline(always)]
        pub const fn fmac2(&self) -> bool {
            let val = (self.0 >> 25usize) & 0x01;
            val != 0
        }
        #[doc = "desc FMAC2."]
        #[inline(always)]
        pub const fn set_fmac2(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
        }
        #[doc = "desc FMAC3."]
        #[must_use]
        #[inline(always)]
        pub const fn fmac3(&self) -> bool {
            let val = (self.0 >> 26usize) & 0x01;
            val != 0
        }
        #[doc = "desc FMAC3."]
        #[inline(always)]
        pub const fn set_fmac3(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
        }
        #[doc = "desc FMAC4."]
        #[must_use]
        #[inline(always)]
        pub const fn fmac4(&self) -> bool {
            let val = (self.0 >> 27usize) & 0x01;
            val != 0
        }
        #[doc = "desc FMAC4."]
        #[inline(always)]
        pub const fn set_fmac4(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
        }
    }
    impl Default for Frst1 {
        #[inline(always)]
        fn default() -> Frst1 {
            Frst1(0)
        }
    }
    impl core::fmt::Debug for Frst1 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Frst1")
                .field("qspi", &self.qspi())
                .field("mdio", &self.mdio())
                .field("spi1", &self.spi1())
                .field("spi2", &self.spi2())
                .field("spi3", &self.spi3())
                .field("spi4", &self.spi4())
                .field("usbfs", &self.usbfs())
                .field("fmac1", &self.fmac1())
                .field("fmac2", &self.fmac2())
                .field("fmac3", &self.fmac3())
                .field("fmac4", &self.fmac4())
                .finish()
        }
    }
    #[doc = "desc FRST2."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Frst2(pub u32);
    impl Frst2 {
        #[doc = "desc TMR6."]
        #[must_use]
        #[inline(always)]
        pub const fn tmr6(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "desc TMR6."]
        #[inline(always)]
        pub const fn set_tmr6(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "desc TMR4."]
        #[must_use]
        #[inline(always)]
        pub const fn tmr4(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "desc TMR4."]
        #[inline(always)]
        pub const fn set_tmr4(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[doc = "desc TMR0."]
        #[must_use]
        #[inline(always)]
        pub const fn tmr0(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "desc TMR0."]
        #[inline(always)]
        pub const fn set_tmr0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "desc EMB."]
        #[must_use]
        #[inline(always)]
        pub const fn emb(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "desc EMB."]
        #[inline(always)]
        pub const fn set_emb(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
        #[doc = "desc TMR2."]
        #[must_use]
        #[inline(always)]
        pub const fn tmr2(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "desc TMR2."]
        #[inline(always)]
        pub const fn set_tmr2(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "desc TMRA."]
        #[must_use]
        #[inline(always)]
        pub const fn tmra(&self) -> bool {
            let val = (self.0 >> 20usize) & 0x01;
            val != 0
        }
        #[doc = "desc TMRA."]
        #[inline(always)]
        pub const fn set_tmra(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
        }
    }
    impl Default for Frst2 {
        #[inline(always)]
        fn default() -> Frst2 {
            Frst2(0)
        }
    }
    impl core::fmt::Debug for Frst2 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Frst2")
                .field("tmr6", &self.tmr6())
                .field("tmr4", &self.tmr4())
                .field("tmr0", &self.tmr0())
                .field("emb", &self.emb())
                .field("tmr2", &self.tmr2())
                .field("tmra", &self.tmra())
                .finish()
        }
    }
    #[doc = "desc FRST3."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Frst3(pub u32);
    impl Frst3 {
        #[doc = "desc ADC1."]
        #[must_use]
        #[inline(always)]
        pub const fn adc1(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "desc ADC1."]
        #[inline(always)]
        pub const fn set_adc1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "desc ADC2."]
        #[must_use]
        #[inline(always)]
        pub const fn adc2(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "desc ADC2."]
        #[inline(always)]
        pub const fn set_adc2(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "desc ADC3."]
        #[must_use]
        #[inline(always)]
        pub const fn adc3(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "desc ADC3."]
        #[inline(always)]
        pub const fn set_adc3(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "desc DAC1."]
        #[must_use]
        #[inline(always)]
        pub const fn dac1(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "desc DAC1."]
        #[inline(always)]
        pub const fn set_dac1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "desc DAC2."]
        #[must_use]
        #[inline(always)]
        pub const fn dac2(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "desc DAC2."]
        #[inline(always)]
        pub const fn set_dac2(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "desc DAC3."]
        #[must_use]
        #[inline(always)]
        pub const fn dac3(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "desc DAC3."]
        #[inline(always)]
        pub const fn set_dac3(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "desc DAC4."]
        #[must_use]
        #[inline(always)]
        pub const fn dac4(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "desc DAC4."]
        #[inline(always)]
        pub const fn set_dac4(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "desc CMP12."]
        #[must_use]
        #[inline(always)]
        pub const fn cmp12(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "desc CMP12."]
        #[inline(always)]
        pub const fn set_cmp12(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "desc CMP34."]
        #[must_use]
        #[inline(always)]
        pub const fn cmp34(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "desc CMP34."]
        #[inline(always)]
        pub const fn set_cmp34(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "desc OTS."]
        #[must_use]
        #[inline(always)]
        pub const fn ots(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "desc OTS."]
        #[inline(always)]
        pub const fn set_ots(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "desc VREF."]
        #[must_use]
        #[inline(always)]
        pub const fn vref(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[doc = "desc VREF."]
        #[inline(always)]
        pub const fn set_vref(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[doc = "desc SMC."]
        #[must_use]
        #[inline(always)]
        pub const fn smc(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "desc SMC."]
        #[inline(always)]
        pub const fn set_smc(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "desc USART1."]
        #[must_use]
        #[inline(always)]
        pub const fn usart1(&self) -> bool {
            let val = (self.0 >> 20usize) & 0x01;
            val != 0
        }
        #[doc = "desc USART1."]
        #[inline(always)]
        pub const fn set_usart1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
        }
        #[doc = "desc USART2."]
        #[must_use]
        #[inline(always)]
        pub const fn usart2(&self) -> bool {
            let val = (self.0 >> 21usize) & 0x01;
            val != 0
        }
        #[doc = "desc USART2."]
        #[inline(always)]
        pub const fn set_usart2(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
        }
        #[doc = "desc USART3."]
        #[must_use]
        #[inline(always)]
        pub const fn usart3(&self) -> bool {
            let val = (self.0 >> 22usize) & 0x01;
            val != 0
        }
        #[doc = "desc USART3."]
        #[inline(always)]
        pub const fn set_usart3(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
        }
        #[doc = "desc USART4."]
        #[must_use]
        #[inline(always)]
        pub const fn usart4(&self) -> bool {
            let val = (self.0 >> 23usize) & 0x01;
            val != 0
        }
        #[doc = "desc USART4."]
        #[inline(always)]
        pub const fn set_usart4(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
        }
        #[doc = "desc USART5."]
        #[must_use]
        #[inline(always)]
        pub const fn usart5(&self) -> bool {
            let val = (self.0 >> 24usize) & 0x01;
            val != 0
        }
        #[doc = "desc USART5."]
        #[inline(always)]
        pub const fn set_usart5(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
        }
        #[doc = "desc USART6."]
        #[must_use]
        #[inline(always)]
        pub const fn usart6(&self) -> bool {
            let val = (self.0 >> 25usize) & 0x01;
            val != 0
        }
        #[doc = "desc USART6."]
        #[inline(always)]
        pub const fn set_usart6(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
        }
    }
    impl Default for Frst3 {
        #[inline(always)]
        fn default() -> Frst3 {
            Frst3(0)
        }
    }
    impl core::fmt::Debug for Frst3 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Frst3")
                .field("adc1", &self.adc1())
                .field("adc2", &self.adc2())
                .field("adc3", &self.adc3())
                .field("dac1", &self.dac1())
                .field("dac2", &self.dac2())
                .field("dac3", &self.dac3())
                .field("dac4", &self.dac4())
                .field("cmp12", &self.cmp12())
                .field("cmp34", &self.cmp34())
                .field("ots", &self.ots())
                .field("vref", &self.vref())
                .field("smc", &self.smc())
                .field("usart1", &self.usart1())
                .field("usart2", &self.usart2())
                .field("usart3", &self.usart3())
                .field("usart4", &self.usart4())
                .field("usart5", &self.usart5())
                .field("usart6", &self.usart6())
                .finish()
        }
    }
    #[doc = "desc PRSTCR0."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Prstcr0(pub u8);
    impl Prstcr0 {
        #[doc = "desc LKUPREN."]
        #[must_use]
        #[inline(always)]
        pub const fn lkupren(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "desc LKUPREN."]
        #[inline(always)]
        pub const fn set_lkupren(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u8) & 0x01) << 5usize);
        }
    }
    impl Default for Prstcr0 {
        #[inline(always)]
        fn default() -> Prstcr0 {
            Prstcr0(0)
        }
    }
    impl core::fmt::Debug for Prstcr0 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Prstcr0")
                .field("lkupren", &self.lkupren())
                .finish()
        }
    }
    #[doc = "desc RSTF0."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Rstf0(pub u32);
    impl Rstf0 {
        #[doc = "desc PORF."]
        #[must_use]
        #[inline(always)]
        pub const fn porf(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "desc PORF."]
        #[inline(always)]
        pub const fn set_porf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "desc PINRF."]
        #[must_use]
        #[inline(always)]
        pub const fn pinrf(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "desc PINRF."]
        #[inline(always)]
        pub const fn set_pinrf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "desc BORF."]
        #[must_use]
        #[inline(always)]
        pub const fn borf(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "desc BORF."]
        #[inline(always)]
        pub const fn set_borf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "desc PVD1RF."]
        #[must_use]
        #[inline(always)]
        pub const fn pvd1rf(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "desc PVD1RF."]
        #[inline(always)]
        pub const fn set_pvd1rf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "desc PVD2RF."]
        #[must_use]
        #[inline(always)]
        pub const fn pvd2rf(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "desc PVD2RF."]
        #[inline(always)]
        pub const fn set_pvd2rf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "desc WDRF."]
        #[must_use]
        #[inline(always)]
        pub const fn wdrf(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "desc WDRF."]
        #[inline(always)]
        pub const fn set_wdrf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "desc SWDRF."]
        #[must_use]
        #[inline(always)]
        pub const fn swdrf(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "desc SWDRF."]
        #[inline(always)]
        pub const fn set_swdrf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "desc PDRF."]
        #[must_use]
        #[inline(always)]
        pub const fn pdrf(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "desc PDRF."]
        #[inline(always)]
        pub const fn set_pdrf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "desc SWRF."]
        #[must_use]
        #[inline(always)]
        pub const fn swrf(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "desc SWRF."]
        #[inline(always)]
        pub const fn set_swrf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "desc MPUERF."]
        #[must_use]
        #[inline(always)]
        pub const fn mpuerf(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "desc MPUERF."]
        #[inline(always)]
        pub const fn set_mpuerf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "desc RAPERF."]
        #[must_use]
        #[inline(always)]
        pub const fn raperf(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "desc RAPERF."]
        #[inline(always)]
        pub const fn set_raperf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[doc = "desc RAECRF."]
        #[must_use]
        #[inline(always)]
        pub const fn raecrf(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "desc RAECRF."]
        #[inline(always)]
        pub const fn set_raecrf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[doc = "desc CKFERF."]
        #[must_use]
        #[inline(always)]
        pub const fn ckferf(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "desc CKFERF."]
        #[inline(always)]
        pub const fn set_ckferf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "desc XTALERF."]
        #[must_use]
        #[inline(always)]
        pub const fn xtalerf(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[doc = "desc XTALERF."]
        #[inline(always)]
        pub const fn set_xtalerf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[doc = "desc LKUPRF."]
        #[must_use]
        #[inline(always)]
        pub const fn lkuprf(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "desc LKUPRF."]
        #[inline(always)]
        pub const fn set_lkuprf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[doc = "desc MULTIRF."]
        #[must_use]
        #[inline(always)]
        pub const fn multirf(&self) -> bool {
            let val = (self.0 >> 30usize) & 0x01;
            val != 0
        }
        #[doc = "desc MULTIRF."]
        #[inline(always)]
        pub const fn set_multirf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
        }
        #[doc = "desc CLRF."]
        #[must_use]
        #[inline(always)]
        pub const fn clrf(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[doc = "desc CLRF."]
        #[inline(always)]
        pub const fn set_clrf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for Rstf0 {
        #[inline(always)]
        fn default() -> Rstf0 {
            Rstf0(0)
        }
    }
    impl core::fmt::Debug for Rstf0 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Rstf0")
                .field("porf", &self.porf())
                .field("pinrf", &self.pinrf())
                .field("borf", &self.borf())
                .field("pvd1rf", &self.pvd1rf())
                .field("pvd2rf", &self.pvd2rf())
                .field("wdrf", &self.wdrf())
                .field("swdrf", &self.swdrf())
                .field("pdrf", &self.pdrf())
                .field("swrf", &self.swrf())
                .field("mpuerf", &self.mpuerf())
                .field("raperf", &self.raperf())
                .field("raecrf", &self.raecrf())
                .field("ckferf", &self.ckferf())
                .field("xtalerf", &self.xtalerf())
                .field("lkuprf", &self.lkuprf())
                .field("multirf", &self.multirf())
                .field("clrf", &self.clrf())
                .finish()
        }
    }
}
