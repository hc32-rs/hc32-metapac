#![allow(clippy::missing_safety_doc)]
#![allow(clippy::identity_op)]
#![allow(clippy::unnecessary_cast)]
#![allow(clippy::erasing_op)]

#[doc = "desc CMU."]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cmu {
    ptr: *mut u8,
}
unsafe impl Send for Cmu {}
unsafe impl Sync for Cmu {}
impl Cmu {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "desc PERICKSEL."]
    #[inline(always)]
    pub const fn pericksel(self) -> crate::common::Reg<regs::Pericksel, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0usize) as _) }
    }
    #[doc = "desc XTALSTDSR."]
    #[inline(always)]
    pub const fn xtalstdsr(self) -> crate::common::Reg<regs::Xtalstdsr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x04usize) as _) }
    }
    #[doc = "desc SCKDIVR."]
    #[inline(always)]
    pub const fn sckdivr(self) -> crate::common::Reg<regs::Sckdivr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x08usize) as _) }
    }
    #[doc = "desc CKSWR."]
    #[inline(always)]
    pub const fn ckswr(self) -> crate::common::Reg<regs::Ckswr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0cusize) as _) }
    }
    #[doc = "desc XTALCR."]
    #[inline(always)]
    pub const fn xtalcr(self) -> crate::common::Reg<regs::Xtalcr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x10usize) as _) }
    }
    #[doc = "desc XTALCFGR."]
    #[inline(always)]
    pub const fn xtalcfgr(self) -> crate::common::Reg<regs::Xtalcfgr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x14usize) as _) }
    }
    #[doc = "desc XTALSTBCR."]
    #[inline(always)]
    pub const fn xtalstbcr(self) -> crate::common::Reg<regs::Xtalstbcr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x15usize) as _) }
    }
    #[doc = "desc HRCCR."]
    #[inline(always)]
    pub const fn hrccr(self) -> crate::common::Reg<regs::Hrccr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x18usize) as _) }
    }
    #[doc = "desc OSCSTBSR."]
    #[inline(always)]
    pub const fn oscstbsr(self) -> crate::common::Reg<regs::Oscstbsr, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x20usize) as _) }
    }
    #[doc = "desc MCO1CFGR."]
    #[inline(always)]
    pub const fn mco1cfgr(self) -> crate::common::Reg<regs::Mco1cfgr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x24usize) as _) }
    }
    #[doc = "desc XTALSTDCR."]
    #[inline(always)]
    pub const fn xtalstdcr(self) -> crate::common::Reg<regs::Xtalstdcr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x28usize) as _) }
    }
    #[doc = "desc FCG."]
    #[inline(always)]
    pub const fn fcg(self) -> crate::common::Reg<regs::Fcg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x30usize) as _) }
    }
    #[doc = "desc XTAL32CR."]
    #[inline(always)]
    pub const fn xtal32cr(self) -> crate::common::Reg<regs::Xtal32cr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x34usize) as _) }
    }
    #[doc = "desc XTAL32CFGR."]
    #[inline(always)]
    pub const fn xtal32cfgr(self) -> crate::common::Reg<regs::Xtal32cfgr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x38usize) as _) }
    }
    #[doc = "desc XTAL32NFR."]
    #[inline(always)]
    pub const fn xtal32nfr(self) -> crate::common::Reg<regs::Xtal32nfr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x39usize) as _) }
    }
    #[doc = "desc LRCCR."]
    #[inline(always)]
    pub const fn lrccr(self) -> crate::common::Reg<regs::Lrccr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x3cusize) as _) }
    }
    #[doc = "desc CANCKCFGR."]
    #[inline(always)]
    pub const fn canckcfgr(self) -> crate::common::Reg<regs::Canckcfgr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x40usize) as _) }
    }
}
pub mod regs {
    #[doc = "desc CANCKCFGR."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Canckcfgr(pub u8);
    impl Canckcfgr {
        #[doc = "desc MCANCKS."]
        #[must_use]
        #[inline(always)]
        pub const fn mcancks(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "desc MCANCKS."]
        #[inline(always)]
        pub const fn set_mcancks(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u8) & 0x01) << 0usize);
        }
    }
    impl Default for Canckcfgr {
        #[inline(always)]
        fn default() -> Canckcfgr {
            Canckcfgr(0)
        }
    }
    impl core::fmt::Debug for Canckcfgr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Canckcfgr")
                .field("mcancks", &self.mcancks())
                .finish()
        }
    }
    #[doc = "desc CKSWR."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ckswr(pub u8);
    impl Ckswr {
        #[doc = "desc CKSW."]
        #[must_use]
        #[inline(always)]
        pub const fn cksw(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x03;
            val as u8
        }
        #[doc = "desc CKSW."]
        #[inline(always)]
        pub const fn set_cksw(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 0usize)) | (((val as u8) & 0x03) << 0usize);
        }
    }
    impl Default for Ckswr {
        #[inline(always)]
        fn default() -> Ckswr {
            Ckswr(0)
        }
    }
    impl core::fmt::Debug for Ckswr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Ckswr").field("cksw", &self.cksw()).finish()
        }
    }
    #[doc = "desc FCG."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Fcg(pub u32);
    impl Fcg {
        #[doc = "desc ADC."]
        #[must_use]
        #[inline(always)]
        pub const fn adc(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "desc ADC."]
        #[inline(always)]
        pub const fn set_adc(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "desc CTC."]
        #[must_use]
        #[inline(always)]
        pub const fn ctc(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "desc CTC."]
        #[inline(always)]
        pub const fn set_ctc(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "desc CMP."]
        #[must_use]
        #[inline(always)]
        pub const fn cmp(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "desc CMP."]
        #[inline(always)]
        pub const fn set_cmp(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "desc AOS."]
        #[must_use]
        #[inline(always)]
        pub const fn aos(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "desc AOS."]
        #[inline(always)]
        pub const fn set_aos(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "desc DMA."]
        #[must_use]
        #[inline(always)]
        pub const fn dma(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "desc DMA."]
        #[inline(always)]
        pub const fn set_dma(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "desc CRC."]
        #[must_use]
        #[inline(always)]
        pub const fn crc(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "desc CRC."]
        #[inline(always)]
        pub const fn set_crc(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "desc TMR0."]
        #[must_use]
        #[inline(always)]
        pub const fn tmr0(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "desc TMR0."]
        #[inline(always)]
        pub const fn set_tmr0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "desc FCM."]
        #[must_use]
        #[inline(always)]
        pub const fn fcm(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "desc FCM."]
        #[inline(always)]
        pub const fn set_fcm(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "desc MCAN."]
        #[must_use]
        #[inline(always)]
        pub const fn mcan(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[doc = "desc MCAN."]
        #[inline(always)]
        pub const fn set_mcan(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[doc = "desc TMR2_1."]
        #[must_use]
        #[inline(always)]
        pub const fn tmr2_1(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "desc TMR2_1."]
        #[inline(always)]
        pub const fn set_tmr2_1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "desc TMR2_2."]
        #[must_use]
        #[inline(always)]
        pub const fn tmr2_2(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "desc TMR2_2."]
        #[inline(always)]
        pub const fn set_tmr2_2(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[doc = "desc TMR4."]
        #[must_use]
        #[inline(always)]
        pub const fn tmr4(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[doc = "desc TMR4."]
        #[inline(always)]
        pub const fn set_tmr4(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
        #[doc = "desc TMRA."]
        #[must_use]
        #[inline(always)]
        pub const fn tmra(&self) -> bool {
            let val = (self.0 >> 19usize) & 0x01;
            val != 0
        }
        #[doc = "desc TMRA."]
        #[inline(always)]
        pub const fn set_tmra(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
        }
        #[doc = "desc EMB."]
        #[must_use]
        #[inline(always)]
        pub const fn emb(&self) -> bool {
            let val = (self.0 >> 20usize) & 0x01;
            val != 0
        }
        #[doc = "desc EMB."]
        #[inline(always)]
        pub const fn set_emb(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
        }
        #[doc = "desc RTC."]
        #[must_use]
        #[inline(always)]
        pub const fn rtc(&self) -> bool {
            let val = (self.0 >> 23usize) & 0x01;
            val != 0
        }
        #[doc = "desc RTC."]
        #[inline(always)]
        pub const fn set_rtc(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
        }
        #[doc = "desc USART1."]
        #[must_use]
        #[inline(always)]
        pub const fn usart1(&self) -> bool {
            let val = (self.0 >> 24usize) & 0x01;
            val != 0
        }
        #[doc = "desc USART1."]
        #[inline(always)]
        pub const fn set_usart1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
        }
        #[doc = "desc USART2."]
        #[must_use]
        #[inline(always)]
        pub const fn usart2(&self) -> bool {
            let val = (self.0 >> 25usize) & 0x01;
            val != 0
        }
        #[doc = "desc USART2."]
        #[inline(always)]
        pub const fn set_usart2(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
        }
        #[doc = "desc USART3."]
        #[must_use]
        #[inline(always)]
        pub const fn usart3(&self) -> bool {
            let val = (self.0 >> 26usize) & 0x01;
            val != 0
        }
        #[doc = "desc USART3."]
        #[inline(always)]
        pub const fn set_usart3(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
        }
        #[doc = "desc USART4."]
        #[must_use]
        #[inline(always)]
        pub const fn usart4(&self) -> bool {
            let val = (self.0 >> 27usize) & 0x01;
            val != 0
        }
        #[doc = "desc USART4."]
        #[inline(always)]
        pub const fn set_usart4(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
        }
        #[doc = "desc I2C."]
        #[must_use]
        #[inline(always)]
        pub const fn i2c(&self) -> bool {
            let val = (self.0 >> 28usize) & 0x01;
            val != 0
        }
        #[doc = "desc I2C."]
        #[inline(always)]
        pub const fn set_i2c(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
        }
        #[doc = "desc SPI1."]
        #[must_use]
        #[inline(always)]
        pub const fn spi1(&self) -> bool {
            let val = (self.0 >> 29usize) & 0x01;
            val != 0
        }
        #[doc = "desc SPI1."]
        #[inline(always)]
        pub const fn set_spi1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
        }
        #[doc = "desc SPI2."]
        #[must_use]
        #[inline(always)]
        pub const fn spi2(&self) -> bool {
            let val = (self.0 >> 30usize) & 0x01;
            val != 0
        }
        #[doc = "desc SPI2."]
        #[inline(always)]
        pub const fn set_spi2(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
        }
    }
    impl Default for Fcg {
        #[inline(always)]
        fn default() -> Fcg {
            Fcg(0)
        }
    }
    impl core::fmt::Debug for Fcg {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Fcg")
                .field("adc", &self.adc())
                .field("ctc", &self.ctc())
                .field("cmp", &self.cmp())
                .field("aos", &self.aos())
                .field("dma", &self.dma())
                .field("crc", &self.crc())
                .field("tmr0", &self.tmr0())
                .field("fcm", &self.fcm())
                .field("mcan", &self.mcan())
                .field("tmr2_1", &self.tmr2_1())
                .field("tmr2_2", &self.tmr2_2())
                .field("tmr4", &self.tmr4())
                .field("tmra", &self.tmra())
                .field("emb", &self.emb())
                .field("rtc", &self.rtc())
                .field("usart1", &self.usart1())
                .field("usart2", &self.usart2())
                .field("usart3", &self.usart3())
                .field("usart4", &self.usart4())
                .field("i2c", &self.i2c())
                .field("spi1", &self.spi1())
                .field("spi2", &self.spi2())
                .finish()
        }
    }
    #[doc = "desc HRCCR."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Hrccr(pub u8);
    impl Hrccr {
        #[doc = "desc HRCSTP."]
        #[must_use]
        #[inline(always)]
        pub const fn hrcstp(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "desc HRCSTP."]
        #[inline(always)]
        pub const fn set_hrcstp(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u8) & 0x01) << 0usize);
        }
    }
    impl Default for Hrccr {
        #[inline(always)]
        fn default() -> Hrccr {
            Hrccr(0)
        }
    }
    impl core::fmt::Debug for Hrccr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Hrccr")
                .field("hrcstp", &self.hrcstp())
                .finish()
        }
    }
    #[doc = "desc LRCCR."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Lrccr(pub u8);
    impl Lrccr {
        #[doc = "desc LRCSTP."]
        #[must_use]
        #[inline(always)]
        pub const fn lrcstp(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "desc LRCSTP."]
        #[inline(always)]
        pub const fn set_lrcstp(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u8) & 0x01) << 0usize);
        }
    }
    impl Default for Lrccr {
        #[inline(always)]
        fn default() -> Lrccr {
            Lrccr(0)
        }
    }
    impl core::fmt::Debug for Lrccr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Lrccr")
                .field("lrcstp", &self.lrcstp())
                .finish()
        }
    }
    #[doc = "desc MCO1CFGR."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Mco1cfgr(pub u8);
    impl Mco1cfgr {
        #[doc = "desc MCOSEL."]
        #[must_use]
        #[inline(always)]
        pub const fn mcosel(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x0f;
            val as u8
        }
        #[doc = "desc MCOSEL."]
        #[inline(always)]
        pub const fn set_mcosel(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u8) & 0x0f) << 0usize);
        }
        #[doc = "desc MCODIV."]
        #[must_use]
        #[inline(always)]
        pub const fn mcodiv(&self) -> u8 {
            let val = (self.0 >> 4usize) & 0x07;
            val as u8
        }
        #[doc = "desc MCODIV."]
        #[inline(always)]
        pub const fn set_mcodiv(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 4usize)) | (((val as u8) & 0x07) << 4usize);
        }
        #[doc = "desc MCOEN."]
        #[must_use]
        #[inline(always)]
        pub const fn mcoen(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "desc MCOEN."]
        #[inline(always)]
        pub const fn set_mcoen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u8) & 0x01) << 7usize);
        }
    }
    impl Default for Mco1cfgr {
        #[inline(always)]
        fn default() -> Mco1cfgr {
            Mco1cfgr(0)
        }
    }
    impl core::fmt::Debug for Mco1cfgr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Mco1cfgr")
                .field("mcosel", &self.mcosel())
                .field("mcodiv", &self.mcodiv())
                .field("mcoen", &self.mcoen())
                .finish()
        }
    }
    #[doc = "desc OSCSTBSR."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Oscstbsr(pub u8);
    impl Oscstbsr {
        #[doc = "desc HRCSTBF."]
        #[must_use]
        #[inline(always)]
        pub const fn hrcstbf(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "desc HRCSTBF."]
        #[inline(always)]
        pub const fn set_hrcstbf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u8) & 0x01) << 0usize);
        }
        #[doc = "desc XTALSTBF."]
        #[must_use]
        #[inline(always)]
        pub const fn xtalstbf(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "desc XTALSTBF."]
        #[inline(always)]
        pub const fn set_xtalstbf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u8) & 0x01) << 3usize);
        }
        #[doc = "desc XTAL32STBF."]
        #[must_use]
        #[inline(always)]
        pub const fn xtal32stbf(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "desc XTAL32STBF."]
        #[inline(always)]
        pub const fn set_xtal32stbf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u8) & 0x01) << 4usize);
        }
    }
    impl Default for Oscstbsr {
        #[inline(always)]
        fn default() -> Oscstbsr {
            Oscstbsr(0)
        }
    }
    impl core::fmt::Debug for Oscstbsr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Oscstbsr")
                .field("hrcstbf", &self.hrcstbf())
                .field("xtalstbf", &self.xtalstbf())
                .field("xtal32stbf", &self.xtal32stbf())
                .finish()
        }
    }
    #[doc = "desc PERICKSEL."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Pericksel(pub u8);
    impl Pericksel {
        #[doc = "desc PERICKSEL."]
        #[must_use]
        #[inline(always)]
        pub const fn pericksel(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x07;
            val as u8
        }
        #[doc = "desc PERICKSEL."]
        #[inline(always)]
        pub const fn set_pericksel(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 0usize)) | (((val as u8) & 0x07) << 0usize);
        }
    }
    impl Default for Pericksel {
        #[inline(always)]
        fn default() -> Pericksel {
            Pericksel(0)
        }
    }
    impl core::fmt::Debug for Pericksel {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Pericksel")
                .field("pericksel", &self.pericksel())
                .finish()
        }
    }
    #[doc = "desc SCKDIVR."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Sckdivr(pub u8);
    impl Sckdivr {
        #[doc = "desc SCKDIV."]
        #[must_use]
        #[inline(always)]
        pub const fn sckdiv(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x07;
            val as u8
        }
        #[doc = "desc SCKDIV."]
        #[inline(always)]
        pub const fn set_sckdiv(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 0usize)) | (((val as u8) & 0x07) << 0usize);
        }
    }
    impl Default for Sckdivr {
        #[inline(always)]
        fn default() -> Sckdivr {
            Sckdivr(0)
        }
    }
    impl core::fmt::Debug for Sckdivr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Sckdivr")
                .field("sckdiv", &self.sckdiv())
                .finish()
        }
    }
    #[doc = "desc XTAL32CFGR."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Xtal32cfgr(pub u8);
    impl Xtal32cfgr {
        #[doc = "desc XTAL32DRV."]
        #[must_use]
        #[inline(always)]
        pub const fn xtal32drv(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "desc XTAL32DRV."]
        #[inline(always)]
        pub const fn set_xtal32drv(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u8) & 0x01) << 0usize);
        }
    }
    impl Default for Xtal32cfgr {
        #[inline(always)]
        fn default() -> Xtal32cfgr {
            Xtal32cfgr(0)
        }
    }
    impl core::fmt::Debug for Xtal32cfgr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Xtal32cfgr")
                .field("xtal32drv", &self.xtal32drv())
                .finish()
        }
    }
    #[doc = "desc XTAL32CR."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Xtal32cr(pub u8);
    impl Xtal32cr {
        #[doc = "desc XTAL32STP."]
        #[must_use]
        #[inline(always)]
        pub const fn xtal32stp(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "desc XTAL32STP."]
        #[inline(always)]
        pub const fn set_xtal32stp(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u8) & 0x01) << 0usize);
        }
    }
    impl Default for Xtal32cr {
        #[inline(always)]
        fn default() -> Xtal32cr {
            Xtal32cr(0)
        }
    }
    impl core::fmt::Debug for Xtal32cr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Xtal32cr")
                .field("xtal32stp", &self.xtal32stp())
                .finish()
        }
    }
    #[doc = "desc XTAL32NFR."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Xtal32nfr(pub u8);
    impl Xtal32nfr {
        #[doc = "desc XTAL32NF."]
        #[must_use]
        #[inline(always)]
        pub const fn xtal32nf(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x03;
            val as u8
        }
        #[doc = "desc XTAL32NF."]
        #[inline(always)]
        pub const fn set_xtal32nf(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 0usize)) | (((val as u8) & 0x03) << 0usize);
        }
    }
    impl Default for Xtal32nfr {
        #[inline(always)]
        fn default() -> Xtal32nfr {
            Xtal32nfr(0)
        }
    }
    impl core::fmt::Debug for Xtal32nfr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Xtal32nfr")
                .field("xtal32nf", &self.xtal32nf())
                .finish()
        }
    }
    #[doc = "desc XTALCFGR."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Xtalcfgr(pub u8);
    impl Xtalcfgr {
        #[doc = "desc XTALDRV."]
        #[must_use]
        #[inline(always)]
        pub const fn xtaldrv(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x0f;
            val as u8
        }
        #[doc = "desc XTALDRV."]
        #[inline(always)]
        pub const fn set_xtaldrv(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u8) & 0x0f) << 0usize);
        }
        #[doc = "desc RFCUT."]
        #[must_use]
        #[inline(always)]
        pub const fn rfcut(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "desc RFCUT."]
        #[inline(always)]
        pub const fn set_rfcut(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u8) & 0x01) << 4usize);
        }
        #[doc = "desc EXTALS."]
        #[must_use]
        #[inline(always)]
        pub const fn extals(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "desc EXTALS."]
        #[inline(always)]
        pub const fn set_extals(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u8) & 0x01) << 5usize);
        }
        #[doc = "desc XTALMS."]
        #[must_use]
        #[inline(always)]
        pub const fn xtalms(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "desc XTALMS."]
        #[inline(always)]
        pub const fn set_xtalms(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u8) & 0x01) << 6usize);
        }
        #[doc = "desc SUPDRV."]
        #[must_use]
        #[inline(always)]
        pub const fn supdrv(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "desc SUPDRV."]
        #[inline(always)]
        pub const fn set_supdrv(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u8) & 0x01) << 7usize);
        }
    }
    impl Default for Xtalcfgr {
        #[inline(always)]
        fn default() -> Xtalcfgr {
            Xtalcfgr(0)
        }
    }
    impl core::fmt::Debug for Xtalcfgr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Xtalcfgr")
                .field("xtaldrv", &self.xtaldrv())
                .field("rfcut", &self.rfcut())
                .field("extals", &self.extals())
                .field("xtalms", &self.xtalms())
                .field("supdrv", &self.supdrv())
                .finish()
        }
    }
    #[doc = "desc XTALCR."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Xtalcr(pub u8);
    impl Xtalcr {
        #[doc = "desc XTALSTP."]
        #[must_use]
        #[inline(always)]
        pub const fn xtalstp(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "desc XTALSTP."]
        #[inline(always)]
        pub const fn set_xtalstp(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u8) & 0x01) << 0usize);
        }
    }
    impl Default for Xtalcr {
        #[inline(always)]
        fn default() -> Xtalcr {
            Xtalcr(0)
        }
    }
    impl core::fmt::Debug for Xtalcr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Xtalcr")
                .field("xtalstp", &self.xtalstp())
                .finish()
        }
    }
    #[doc = "desc XTALSTBCR."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Xtalstbcr(pub u8);
    impl Xtalstbcr {
        #[doc = "desc XTALSTB."]
        #[must_use]
        #[inline(always)]
        pub const fn xtalstb(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x07;
            val as u8
        }
        #[doc = "desc XTALSTB."]
        #[inline(always)]
        pub const fn set_xtalstb(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 0usize)) | (((val as u8) & 0x07) << 0usize);
        }
    }
    impl Default for Xtalstbcr {
        #[inline(always)]
        fn default() -> Xtalstbcr {
            Xtalstbcr(0)
        }
    }
    impl core::fmt::Debug for Xtalstbcr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Xtalstbcr")
                .field("xtalstb", &self.xtalstb())
                .finish()
        }
    }
    #[doc = "desc XTALSTDCR."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Xtalstdcr(pub u8);
    impl Xtalstdcr {
        #[doc = "desc XTALSTDIE."]
        #[must_use]
        #[inline(always)]
        pub const fn xtalstdie(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "desc XTALSTDIE."]
        #[inline(always)]
        pub const fn set_xtalstdie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u8) & 0x01) << 0usize);
        }
        #[doc = "desc XTALSTDRE."]
        #[must_use]
        #[inline(always)]
        pub const fn xtalstdre(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "desc XTALSTDRE."]
        #[inline(always)]
        pub const fn set_xtalstdre(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u8) & 0x01) << 1usize);
        }
        #[doc = "desc XTALSTDRIS."]
        #[must_use]
        #[inline(always)]
        pub const fn xtalstdris(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "desc XTALSTDRIS."]
        #[inline(always)]
        pub const fn set_xtalstdris(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u8) & 0x01) << 2usize);
        }
        #[doc = "desc XTALSTDE."]
        #[must_use]
        #[inline(always)]
        pub const fn xtalstde(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "desc XTALSTDE."]
        #[inline(always)]
        pub const fn set_xtalstde(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u8) & 0x01) << 7usize);
        }
    }
    impl Default for Xtalstdcr {
        #[inline(always)]
        fn default() -> Xtalstdcr {
            Xtalstdcr(0)
        }
    }
    impl core::fmt::Debug for Xtalstdcr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Xtalstdcr")
                .field("xtalstdie", &self.xtalstdie())
                .field("xtalstdre", &self.xtalstdre())
                .field("xtalstdris", &self.xtalstdris())
                .field("xtalstde", &self.xtalstde())
                .finish()
        }
    }
    #[doc = "desc XTALSTDSR."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Xtalstdsr(pub u8);
    impl Xtalstdsr {
        #[doc = "desc XTALSTDF."]
        #[must_use]
        #[inline(always)]
        pub const fn xtalstdf(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "desc XTALSTDF."]
        #[inline(always)]
        pub const fn set_xtalstdf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u8) & 0x01) << 0usize);
        }
    }
    impl Default for Xtalstdsr {
        #[inline(always)]
        fn default() -> Xtalstdsr {
            Xtalstdsr(0)
        }
    }
    impl core::fmt::Debug for Xtalstdsr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Xtalstdsr")
                .field("xtalstdf", &self.xtalstdf())
                .finish()
        }
    }
}
