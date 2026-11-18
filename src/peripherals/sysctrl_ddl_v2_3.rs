#![allow(clippy::missing_safety_doc)]
#![allow(clippy::identity_op)]
#![allow(clippy::unnecessary_cast)]
#![allow(clippy::erasing_op)]

#[doc = "desc SYSCTRL."]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sysctrl {
    ptr: *mut u8,
}
unsafe impl Send for Sysctrl {}
unsafe impl Sync for Sysctrl {}
impl Sysctrl {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "desc SYSCTRL0."]
    #[inline(always)]
    pub const fn sysctrl0(self) -> crate::common::Reg<regs::Sysctrl0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0usize) as _) }
    }
    #[doc = "desc SYSCTRL1."]
    #[inline(always)]
    pub const fn sysctrl1(self) -> crate::common::Reg<regs::Sysctrl1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x04usize) as _) }
    }
    #[doc = "desc SYSCTRL2."]
    #[inline(always)]
    pub const fn sysctrl2(self) -> crate::common::Reg<regs::Sysctrl2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x08usize) as _) }
    }
    #[doc = "desc RC48M_CR."]
    #[inline(always)]
    pub const fn rc48m_cr(self) -> crate::common::Reg<regs::Rc48mCr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0cusize) as _) }
    }
    #[doc = "desc RCL_CR."]
    #[inline(always)]
    pub const fn rcl_cr(self) -> crate::common::Reg<regs::RclCr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x14usize) as _) }
    }
    #[doc = "desc XTL_CR."]
    #[inline(always)]
    pub const fn xtl_cr(self) -> crate::common::Reg<regs::XtlCr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x18usize) as _) }
    }
    #[doc = "desc RESET_FLAG."]
    #[inline(always)]
    pub const fn reset_flag(self) -> crate::common::Reg<regs::ResetFlag, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1cusize) as _) }
    }
    #[doc = "desc PERI_CLKEN0."]
    #[inline(always)]
    pub const fn peri_clken0(self) -> crate::common::Reg<regs::PeriClken0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x20usize) as _) }
    }
    #[doc = "desc PERI_CLKEN1."]
    #[inline(always)]
    pub const fn peri_clken1(self) -> crate::common::Reg<regs::PeriClken1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x24usize) as _) }
    }
    #[doc = "desc PERI_RESET0."]
    #[inline(always)]
    pub const fn peri_reset0(self) -> crate::common::Reg<regs::PeriReset0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x28usize) as _) }
    }
    #[doc = "desc PERI_RESET1."]
    #[inline(always)]
    pub const fn peri_reset1(self) -> crate::common::Reg<regs::PeriReset1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x2cusize) as _) }
    }
    #[doc = "desc DEBUG_ACTIVE."]
    #[inline(always)]
    pub const fn debug_active(self) -> crate::common::Reg<regs::DebugActive, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x38usize) as _) }
    }
}
pub mod regs {
    #[doc = "desc DEBUG_ACTIVE."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct DebugActive(pub u32);
    impl DebugActive {
        #[doc = "desc IWDT_DEBUG."]
        #[must_use]
        #[inline(always)]
        pub const fn iwdt_debug(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "desc IWDT_DEBUG."]
        #[inline(always)]
        pub const fn set_iwdt_debug(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "desc RTC_DEBUG."]
        #[must_use]
        #[inline(always)]
        pub const fn rtc_debug(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "desc RTC_DEBUG."]
        #[inline(always)]
        pub const fn set_rtc_debug(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "desc ATIM3_DEBUG."]
        #[must_use]
        #[inline(always)]
        pub const fn atim3_debug(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "desc ATIM3_DEBUG."]
        #[inline(always)]
        pub const fn set_atim3_debug(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[doc = "desc CTIM0_DEBUG."]
        #[must_use]
        #[inline(always)]
        pub const fn ctim0_debug(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "desc CTIM0_DEBUG."]
        #[inline(always)]
        pub const fn set_ctim0_debug(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "desc CTIM1_DEBUG."]
        #[must_use]
        #[inline(always)]
        pub const fn ctim1_debug(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "desc CTIM1_DEBUG."]
        #[inline(always)]
        pub const fn set_ctim1_debug(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[doc = "desc RSV."]
        #[must_use]
        #[inline(always)]
        pub const fn rsv(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[doc = "desc RSV."]
        #[inline(always)]
        pub const fn set_rsv(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for DebugActive {
        #[inline(always)]
        fn default() -> DebugActive {
            DebugActive(0)
        }
    }
    impl core::fmt::Debug for DebugActive {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("DebugActive")
                .field("iwdt_debug", &self.iwdt_debug())
                .field("rtc_debug", &self.rtc_debug())
                .field("atim3_debug", &self.atim3_debug())
                .field("ctim0_debug", &self.ctim0_debug())
                .field("ctim1_debug", &self.ctim1_debug())
                .field("rsv", &self.rsv())
                .finish()
        }
    }
    #[doc = "desc PERI_CLKEN0."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct PeriClken0(pub u32);
    impl PeriClken0 {
        #[doc = "desc LPUART0_EN."]
        #[must_use]
        #[inline(always)]
        pub const fn lpuart0_en(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "desc LPUART0_EN."]
        #[inline(always)]
        pub const fn set_lpuart0_en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "desc LPUART1_EN."]
        #[must_use]
        #[inline(always)]
        pub const fn lpuart1_en(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "desc LPUART1_EN."]
        #[inline(always)]
        pub const fn set_lpuart1_en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "desc SPI_EN."]
        #[must_use]
        #[inline(always)]
        pub const fn spi_en(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "desc SPI_EN."]
        #[inline(always)]
        pub const fn set_spi_en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "desc ATIM3_EN."]
        #[must_use]
        #[inline(always)]
        pub const fn atim3_en(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "desc ATIM3_EN."]
        #[inline(always)]
        pub const fn set_atim3_en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[doc = "desc IWDT_EN."]
        #[must_use]
        #[inline(always)]
        pub const fn iwdt_en(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "desc IWDT_EN."]
        #[inline(always)]
        pub const fn set_iwdt_en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
        #[doc = "desc ADC_EN."]
        #[must_use]
        #[inline(always)]
        pub const fn adc_en(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "desc ADC_EN."]
        #[inline(always)]
        pub const fn set_adc_en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "desc VC_EN."]
        #[must_use]
        #[inline(always)]
        pub const fn vc_en(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "desc VC_EN."]
        #[inline(always)]
        pub const fn set_vc_en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[doc = "desc TRNG_EN."]
        #[must_use]
        #[inline(always)]
        pub const fn trng_en(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[doc = "desc TRNG_EN."]
        #[inline(always)]
        pub const fn set_trng_en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
        #[doc = "desc RTC_EN."]
        #[must_use]
        #[inline(always)]
        pub const fn rtc_en(&self) -> bool {
            let val = (self.0 >> 20usize) & 0x01;
            val != 0
        }
        #[doc = "desc RTC_EN."]
        #[inline(always)]
        pub const fn set_rtc_en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
        }
        #[doc = "desc GPIO_EN."]
        #[must_use]
        #[inline(always)]
        pub const fn gpio_en(&self) -> bool {
            let val = (self.0 >> 28usize) & 0x01;
            val != 0
        }
        #[doc = "desc GPIO_EN."]
        #[inline(always)]
        pub const fn set_gpio_en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
        }
        #[doc = "desc HSI2C_EN."]
        #[must_use]
        #[inline(always)]
        pub const fn hsi2c_en(&self) -> bool {
            let val = (self.0 >> 30usize) & 0x01;
            val != 0
        }
        #[doc = "desc HSI2C_EN."]
        #[inline(always)]
        pub const fn set_hsi2c_en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
        }
        #[doc = "desc FLASH_EN."]
        #[must_use]
        #[inline(always)]
        pub const fn flash_en(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[doc = "desc FLASH_EN."]
        #[inline(always)]
        pub const fn set_flash_en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for PeriClken0 {
        #[inline(always)]
        fn default() -> PeriClken0 {
            PeriClken0(0)
        }
    }
    impl core::fmt::Debug for PeriClken0 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("PeriClken0")
                .field("lpuart0_en", &self.lpuart0_en())
                .field("lpuart1_en", &self.lpuart1_en())
                .field("spi_en", &self.spi_en())
                .field("atim3_en", &self.atim3_en())
                .field("iwdt_en", &self.iwdt_en())
                .field("adc_en", &self.adc_en())
                .field("vc_en", &self.vc_en())
                .field("trng_en", &self.trng_en())
                .field("rtc_en", &self.rtc_en())
                .field("gpio_en", &self.gpio_en())
                .field("hsi2c_en", &self.hsi2c_en())
                .field("flash_en", &self.flash_en())
                .finish()
        }
    }
    #[doc = "desc PERI_CLKEN1."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct PeriClken1(pub u32);
    impl PeriClken1 {
        #[doc = "desc CTIM0_EN."]
        #[must_use]
        #[inline(always)]
        pub const fn ctim0_en(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "desc CTIM0_EN."]
        #[inline(always)]
        pub const fn set_ctim0_en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "desc CTIM1_EN."]
        #[must_use]
        #[inline(always)]
        pub const fn ctim1_en(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "desc CTIM1_EN."]
        #[inline(always)]
        pub const fn set_ctim1_en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "desc CTRIM_EN."]
        #[must_use]
        #[inline(always)]
        pub const fn ctrim_en(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "desc CTRIM_EN."]
        #[inline(always)]
        pub const fn set_ctrim_en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "desc RSV."]
        #[must_use]
        #[inline(always)]
        pub const fn rsv(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[doc = "desc RSV."]
        #[inline(always)]
        pub const fn set_rsv(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for PeriClken1 {
        #[inline(always)]
        fn default() -> PeriClken1 {
            PeriClken1(0)
        }
    }
    impl core::fmt::Debug for PeriClken1 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("PeriClken1")
                .field("ctim0_en", &self.ctim0_en())
                .field("ctim1_en", &self.ctim1_en())
                .field("ctrim_en", &self.ctrim_en())
                .field("rsv", &self.rsv())
                .finish()
        }
    }
    #[doc = "desc PERI_RESET0."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct PeriReset0(pub u32);
    impl PeriReset0 {
        #[doc = "desc LPUART0_RST."]
        #[must_use]
        #[inline(always)]
        pub const fn lpuart0_rst(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "desc LPUART0_RST."]
        #[inline(always)]
        pub const fn set_lpuart0_rst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "desc LPUART1_RST."]
        #[must_use]
        #[inline(always)]
        pub const fn lpuart1_rst(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "desc LPUART1_RST."]
        #[inline(always)]
        pub const fn set_lpuart1_rst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "desc SPI_RST."]
        #[must_use]
        #[inline(always)]
        pub const fn spi_rst(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "desc SPI_RST."]
        #[inline(always)]
        pub const fn set_spi_rst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "desc ATIM3_RST."]
        #[must_use]
        #[inline(always)]
        pub const fn atim3_rst(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "desc ATIM3_RST."]
        #[inline(always)]
        pub const fn set_atim3_rst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[doc = "desc ADC_RST."]
        #[must_use]
        #[inline(always)]
        pub const fn adc_rst(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "desc ADC_RST."]
        #[inline(always)]
        pub const fn set_adc_rst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "desc VC_RST."]
        #[must_use]
        #[inline(always)]
        pub const fn vc_rst(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "desc VC_RST."]
        #[inline(always)]
        pub const fn set_vc_rst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[doc = "desc TRNG_RST."]
        #[must_use]
        #[inline(always)]
        pub const fn trng_rst(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[doc = "desc TRNG_RST."]
        #[inline(always)]
        pub const fn set_trng_rst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
        #[doc = "desc RTC_RST."]
        #[must_use]
        #[inline(always)]
        pub const fn rtc_rst(&self) -> bool {
            let val = (self.0 >> 20usize) & 0x01;
            val != 0
        }
        #[doc = "desc RTC_RST."]
        #[inline(always)]
        pub const fn set_rtc_rst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
        }
        #[doc = "desc GPIO_RST."]
        #[must_use]
        #[inline(always)]
        pub const fn gpio_rst(&self) -> bool {
            let val = (self.0 >> 28usize) & 0x01;
            val != 0
        }
        #[doc = "desc GPIO_RST."]
        #[inline(always)]
        pub const fn set_gpio_rst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
        }
        #[doc = "desc HSI2C_RST."]
        #[must_use]
        #[inline(always)]
        pub const fn hsi2c_rst(&self) -> bool {
            let val = (self.0 >> 30usize) & 0x01;
            val != 0
        }
        #[doc = "desc HSI2C_RST."]
        #[inline(always)]
        pub const fn set_hsi2c_rst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
        }
        #[doc = "desc RSV."]
        #[must_use]
        #[inline(always)]
        pub const fn rsv(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[doc = "desc RSV."]
        #[inline(always)]
        pub const fn set_rsv(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for PeriReset0 {
        #[inline(always)]
        fn default() -> PeriReset0 {
            PeriReset0(0)
        }
    }
    impl core::fmt::Debug for PeriReset0 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("PeriReset0")
                .field("lpuart0_rst", &self.lpuart0_rst())
                .field("lpuart1_rst", &self.lpuart1_rst())
                .field("spi_rst", &self.spi_rst())
                .field("atim3_rst", &self.atim3_rst())
                .field("adc_rst", &self.adc_rst())
                .field("vc_rst", &self.vc_rst())
                .field("trng_rst", &self.trng_rst())
                .field("rtc_rst", &self.rtc_rst())
                .field("gpio_rst", &self.gpio_rst())
                .field("hsi2c_rst", &self.hsi2c_rst())
                .field("rsv", &self.rsv())
                .finish()
        }
    }
    #[doc = "desc PERI_RESET1."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct PeriReset1(pub u32);
    impl PeriReset1 {
        #[doc = "desc CTIM0_RST."]
        #[must_use]
        #[inline(always)]
        pub const fn ctim0_rst(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "desc CTIM0_RST."]
        #[inline(always)]
        pub const fn set_ctim0_rst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "desc CTIM1_RST."]
        #[must_use]
        #[inline(always)]
        pub const fn ctim1_rst(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "desc CTIM1_RST."]
        #[inline(always)]
        pub const fn set_ctim1_rst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "desc CTRIM_RST."]
        #[must_use]
        #[inline(always)]
        pub const fn ctrim_rst(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "desc CTRIM_RST."]
        #[inline(always)]
        pub const fn set_ctrim_rst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "desc RSV."]
        #[must_use]
        #[inline(always)]
        pub const fn rsv(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[doc = "desc RSV."]
        #[inline(always)]
        pub const fn set_rsv(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for PeriReset1 {
        #[inline(always)]
        fn default() -> PeriReset1 {
            PeriReset1(0)
        }
    }
    impl core::fmt::Debug for PeriReset1 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("PeriReset1")
                .field("ctim0_rst", &self.ctim0_rst())
                .field("ctim1_rst", &self.ctim1_rst())
                .field("ctrim_rst", &self.ctrim_rst())
                .field("rsv", &self.rsv())
                .finish()
        }
    }
    #[doc = "desc RC48M_CR."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Rc48mCr(pub u32);
    impl Rc48mCr {
        #[doc = "desc TRIMF."]
        #[must_use]
        #[inline(always)]
        pub const fn trimf(&self) -> u8 {
            let val = (self.0 >> 2usize) & 0x3f;
            val as u8
        }
        #[doc = "desc TRIMF."]
        #[inline(always)]
        pub const fn set_trimf(&mut self, val: u8) {
            self.0 = (self.0 & !(0x3f << 2usize)) | (((val as u32) & 0x3f) << 2usize);
        }
        #[doc = "desc TRIMC."]
        #[must_use]
        #[inline(always)]
        pub const fn trimc(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0xff;
            val as u8
        }
        #[doc = "desc TRIMC."]
        #[inline(always)]
        pub const fn set_trimc(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
        }
        #[doc = "desc FSEL."]
        #[must_use]
        #[inline(always)]
        pub const fn fsel(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x03;
            val as u8
        }
        #[doc = "desc FSEL."]
        #[inline(always)]
        pub const fn set_fsel(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 16usize)) | (((val as u32) & 0x03) << 16usize);
        }
        #[doc = "desc STABLE."]
        #[must_use]
        #[inline(always)]
        pub const fn stable(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[doc = "desc STABLE."]
        #[inline(always)]
        pub const fn set_stable(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for Rc48mCr {
        #[inline(always)]
        fn default() -> Rc48mCr {
            Rc48mCr(0)
        }
    }
    impl core::fmt::Debug for Rc48mCr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Rc48mCr")
                .field("trimf", &self.trimf())
                .field("trimc", &self.trimc())
                .field("fsel", &self.fsel())
                .field("stable", &self.stable())
                .finish()
        }
    }
    #[doc = "desc RCL_CR."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct RclCr(pub u32);
    impl RclCr {
        #[doc = "desc TRIM."]
        #[must_use]
        #[inline(always)]
        pub const fn trim(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x03ff;
            val as u16
        }
        #[doc = "desc TRIM."]
        #[inline(always)]
        pub const fn set_trim(&mut self, val: u16) {
            self.0 = (self.0 & !(0x03ff << 0usize)) | (((val as u32) & 0x03ff) << 0usize);
        }
        #[doc = "desc STABLE."]
        #[must_use]
        #[inline(always)]
        pub const fn stable(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "desc STABLE."]
        #[inline(always)]
        pub const fn set_stable(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "desc MSEL."]
        #[must_use]
        #[inline(always)]
        pub const fn msel(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "desc MSEL."]
        #[inline(always)]
        pub const fn set_msel(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "desc RSV."]
        #[must_use]
        #[inline(always)]
        pub const fn rsv(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[doc = "desc RSV."]
        #[inline(always)]
        pub const fn set_rsv(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for RclCr {
        #[inline(always)]
        fn default() -> RclCr {
            RclCr(0)
        }
    }
    impl core::fmt::Debug for RclCr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("RclCr")
                .field("trim", &self.trim())
                .field("stable", &self.stable())
                .field("msel", &self.msel())
                .field("rsv", &self.rsv())
                .finish()
        }
    }
    #[doc = "desc RESET_FLAG."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct ResetFlag(pub u32);
    impl ResetFlag {
        #[doc = "desc POR5V."]
        #[must_use]
        #[inline(always)]
        pub const fn por5v(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "desc POR5V."]
        #[inline(always)]
        pub const fn set_por5v(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "desc POR15V."]
        #[must_use]
        #[inline(always)]
        pub const fn por15v(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "desc POR15V."]
        #[inline(always)]
        pub const fn set_por15v(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "desc LVD_FLAG."]
        #[must_use]
        #[inline(always)]
        pub const fn lvd_flag(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "desc LVD_FLAG."]
        #[inline(always)]
        pub const fn set_lvd_flag(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "desc IWDT_FLAG."]
        #[must_use]
        #[inline(always)]
        pub const fn iwdt_flag(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "desc IWDT_FLAG."]
        #[inline(always)]
        pub const fn set_iwdt_flag(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "desc LOCKUP."]
        #[must_use]
        #[inline(always)]
        pub const fn lockup(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "desc LOCKUP."]
        #[inline(always)]
        pub const fn set_lockup(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "desc SYSREQ."]
        #[must_use]
        #[inline(always)]
        pub const fn sysreq(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "desc SYSREQ."]
        #[inline(always)]
        pub const fn set_sysreq(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "desc RSTB."]
        #[must_use]
        #[inline(always)]
        pub const fn rstb(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "desc RSTB."]
        #[inline(always)]
        pub const fn set_rstb(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "desc RSV."]
        #[must_use]
        #[inline(always)]
        pub const fn rsv(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[doc = "desc RSV."]
        #[inline(always)]
        pub const fn set_rsv(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for ResetFlag {
        #[inline(always)]
        fn default() -> ResetFlag {
            ResetFlag(0)
        }
    }
    impl core::fmt::Debug for ResetFlag {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("ResetFlag")
                .field("por5v", &self.por5v())
                .field("por15v", &self.por15v())
                .field("lvd_flag", &self.lvd_flag())
                .field("iwdt_flag", &self.iwdt_flag())
                .field("lockup", &self.lockup())
                .field("sysreq", &self.sysreq())
                .field("rstb", &self.rstb())
                .field("rsv", &self.rsv())
                .finish()
        }
    }
    #[doc = "desc SYSCTRL0."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Sysctrl0(pub u32);
    impl Sysctrl0 {
        #[doc = "desc RC48M_EN."]
        #[must_use]
        #[inline(always)]
        pub const fn rc48m_en(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "desc RC48M_EN."]
        #[inline(always)]
        pub const fn set_rc48m_en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "desc RCL_EN."]
        #[must_use]
        #[inline(always)]
        pub const fn rcl_en(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "desc RCL_EN."]
        #[inline(always)]
        pub const fn set_rcl_en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "desc XTL_EN."]
        #[must_use]
        #[inline(always)]
        pub const fn xtl_en(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "desc XTL_EN."]
        #[inline(always)]
        pub const fn set_xtl_en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "desc CLKSW."]
        #[must_use]
        #[inline(always)]
        pub const fn clksw(&self) -> u8 {
            let val = (self.0 >> 5usize) & 0x07;
            val as u8
        }
        #[doc = "desc CLKSW."]
        #[inline(always)]
        pub const fn set_clksw(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 5usize)) | (((val as u32) & 0x07) << 5usize);
        }
        #[doc = "desc HCLK_PRS."]
        #[must_use]
        #[inline(always)]
        pub const fn hclk_prs(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0x07;
            val as u8
        }
        #[doc = "desc HCLK_PRS."]
        #[inline(always)]
        pub const fn set_hclk_prs(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 8usize)) | (((val as u32) & 0x07) << 8usize);
        }
        #[doc = "desc WAKEUP_BYRC48M."]
        #[must_use]
        #[inline(always)]
        pub const fn wakeup_byrc48m(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "desc WAKEUP_BYRC48M."]
        #[inline(always)]
        pub const fn set_wakeup_byrc48m(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
        #[doc = "desc RSV."]
        #[must_use]
        #[inline(always)]
        pub const fn rsv(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[doc = "desc RSV."]
        #[inline(always)]
        pub const fn set_rsv(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for Sysctrl0 {
        #[inline(always)]
        fn default() -> Sysctrl0 {
            Sysctrl0(0)
        }
    }
    impl core::fmt::Debug for Sysctrl0 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Sysctrl0")
                .field("rc48m_en", &self.rc48m_en())
                .field("rcl_en", &self.rcl_en())
                .field("xtl_en", &self.xtl_en())
                .field("clksw", &self.clksw())
                .field("hclk_prs", &self.hclk_prs())
                .field("wakeup_byrc48m", &self.wakeup_byrc48m())
                .field("rsv", &self.rsv())
                .finish()
        }
    }
    #[doc = "desc SYSCTRL1."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Sysctrl1(pub u32);
    impl Sysctrl1 {
        #[doc = "desc RESET_USE_IO."]
        #[must_use]
        #[inline(always)]
        pub const fn reset_use_io(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "desc RESET_USE_IO."]
        #[inline(always)]
        pub const fn set_reset_use_io(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "desc EXTH_EN."]
        #[must_use]
        #[inline(always)]
        pub const fn exth_en(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "desc EXTH_EN."]
        #[inline(always)]
        pub const fn set_exth_en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "desc EXTL_EN."]
        #[must_use]
        #[inline(always)]
        pub const fn extl_en(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "desc EXTL_EN."]
        #[inline(always)]
        pub const fn set_extl_en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "desc XTL_ALWAYS_ON."]
        #[must_use]
        #[inline(always)]
        pub const fn xtl_always_on(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "desc XTL_ALWAYS_ON."]
        #[inline(always)]
        pub const fn set_xtl_always_on(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "desc CLOCK_FAULT_EN."]
        #[must_use]
        #[inline(always)]
        pub const fn clock_fault_en(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "desc CLOCK_FAULT_EN."]
        #[inline(always)]
        pub const fn set_clock_fault_en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "desc LOCKUP_EN."]
        #[must_use]
        #[inline(always)]
        pub const fn lockup_en(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "desc LOCKUP_EN."]
        #[inline(always)]
        pub const fn set_lockup_en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "desc SPI_PCTRL."]
        #[must_use]
        #[inline(always)]
        pub const fn spi_pctrl(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "desc SPI_PCTRL."]
        #[inline(always)]
        pub const fn set_spi_pctrl(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "desc SWD_USE_IO."]
        #[must_use]
        #[inline(always)]
        pub const fn swd_use_io(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "desc SWD_USE_IO."]
        #[inline(always)]
        pub const fn set_swd_use_io(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "desc CTIMER0_FUN_SEL."]
        #[must_use]
        #[inline(always)]
        pub const fn ctimer0_fun_sel(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "desc CTIMER0_FUN_SEL."]
        #[inline(always)]
        pub const fn set_ctimer0_fun_sel(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "desc CTIMER1_FUN_SEL."]
        #[must_use]
        #[inline(always)]
        pub const fn ctimer1_fun_sel(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "desc CTIMER1_FUN_SEL."]
        #[inline(always)]
        pub const fn set_ctimer1_fun_sel(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[doc = "desc RSV."]
        #[must_use]
        #[inline(always)]
        pub const fn rsv(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[doc = "desc RSV."]
        #[inline(always)]
        pub const fn set_rsv(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for Sysctrl1 {
        #[inline(always)]
        fn default() -> Sysctrl1 {
            Sysctrl1(0)
        }
    }
    impl core::fmt::Debug for Sysctrl1 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Sysctrl1")
                .field("reset_use_io", &self.reset_use_io())
                .field("exth_en", &self.exth_en())
                .field("extl_en", &self.extl_en())
                .field("xtl_always_on", &self.xtl_always_on())
                .field("clock_fault_en", &self.clock_fault_en())
                .field("lockup_en", &self.lockup_en())
                .field("spi_pctrl", &self.spi_pctrl())
                .field("swd_use_io", &self.swd_use_io())
                .field("ctimer0_fun_sel", &self.ctimer0_fun_sel())
                .field("ctimer1_fun_sel", &self.ctimer1_fun_sel())
                .field("rsv", &self.rsv())
                .finish()
        }
    }
    #[doc = "desc SYSCTRL2."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Sysctrl2(pub u32);
    impl Sysctrl2 {
        #[doc = "desc SYSCTRL2."]
        #[must_use]
        #[inline(always)]
        pub const fn sysctrl2(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[doc = "desc SYSCTRL2."]
        #[inline(always)]
        pub const fn set_sysctrl2(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
        }
        #[doc = "desc RSV."]
        #[must_use]
        #[inline(always)]
        pub const fn rsv(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[doc = "desc RSV."]
        #[inline(always)]
        pub const fn set_rsv(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for Sysctrl2 {
        #[inline(always)]
        fn default() -> Sysctrl2 {
            Sysctrl2(0)
        }
    }
    impl core::fmt::Debug for Sysctrl2 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Sysctrl2")
                .field("sysctrl2", &self.sysctrl2())
                .field("rsv", &self.rsv())
                .finish()
        }
    }
    #[doc = "desc XTL_CR."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct XtlCr(pub u32);
    impl XtlCr {
        #[doc = "desc DRIVER."]
        #[must_use]
        #[inline(always)]
        pub const fn driver(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x03;
            val as u8
        }
        #[doc = "desc DRIVER."]
        #[inline(always)]
        pub const fn set_driver(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 0usize)) | (((val as u32) & 0x03) << 0usize);
        }
        #[doc = "desc AMP_SEL."]
        #[must_use]
        #[inline(always)]
        pub const fn amp_sel(&self) -> u8 {
            let val = (self.0 >> 2usize) & 0x03;
            val as u8
        }
        #[doc = "desc AMP_SEL."]
        #[inline(always)]
        pub const fn set_amp_sel(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 2usize)) | (((val as u32) & 0x03) << 2usize);
        }
        #[doc = "desc STARTUP."]
        #[must_use]
        #[inline(always)]
        pub const fn startup(&self) -> u8 {
            let val = (self.0 >> 4usize) & 0x03;
            val as u8
        }
        #[doc = "desc STARTUP."]
        #[inline(always)]
        pub const fn set_startup(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 4usize)) | (((val as u32) & 0x03) << 4usize);
        }
        #[doc = "desc STABLE."]
        #[must_use]
        #[inline(always)]
        pub const fn stable(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "desc STABLE."]
        #[inline(always)]
        pub const fn set_stable(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "desc DETIE."]
        #[must_use]
        #[inline(always)]
        pub const fn detie(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "desc DETIE."]
        #[inline(always)]
        pub const fn set_detie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "desc DETTIME."]
        #[must_use]
        #[inline(always)]
        pub const fn dettime(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "desc DETTIME."]
        #[inline(always)]
        pub const fn set_dettime(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "desc DETEN."]
        #[must_use]
        #[inline(always)]
        pub const fn deten(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "desc DETEN."]
        #[inline(always)]
        pub const fn set_deten(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "desc FAULT_STATE."]
        #[must_use]
        #[inline(always)]
        pub const fn fault_state(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "desc FAULT_STATE."]
        #[inline(always)]
        pub const fn set_fault_state(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[doc = "desc RSV."]
        #[must_use]
        #[inline(always)]
        pub const fn rsv(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[doc = "desc RSV."]
        #[inline(always)]
        pub const fn set_rsv(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for XtlCr {
        #[inline(always)]
        fn default() -> XtlCr {
            XtlCr(0)
        }
    }
    impl core::fmt::Debug for XtlCr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("XtlCr")
                .field("driver", &self.driver())
                .field("amp_sel", &self.amp_sel())
                .field("startup", &self.startup())
                .field("stable", &self.stable())
                .field("detie", &self.detie())
                .field("dettime", &self.dettime())
                .field("deten", &self.deten())
                .field("fault_state", &self.fault_state())
                .field("rsv", &self.rsv())
                .finish()
        }
    }
}
