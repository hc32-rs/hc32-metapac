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
    #[doc = "desc LRCCR."]
    #[inline(always)]
    pub const fn lrccr(self) -> crate::common::Reg<regs::Lrccr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x3cusize) as _) }
    }
}
pub mod regs {
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
        #[doc = "desc TMRB1."]
        #[must_use]
        #[inline(always)]
        pub const fn tmrb1(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "desc TMRB1."]
        #[inline(always)]
        pub const fn set_tmrb1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "desc TMRB2."]
        #[must_use]
        #[inline(always)]
        pub const fn tmrb2(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "desc TMRB2."]
        #[inline(always)]
        pub const fn set_tmrb2(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "desc TMRB3."]
        #[must_use]
        #[inline(always)]
        pub const fn tmrb3(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "desc TMRB3."]
        #[inline(always)]
        pub const fn set_tmrb3(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[doc = "desc TMRB4."]
        #[must_use]
        #[inline(always)]
        pub const fn tmrb4(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "desc TMRB4."]
        #[inline(always)]
        pub const fn set_tmrb4(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[doc = "desc TMR0."]
        #[must_use]
        #[inline(always)]
        pub const fn tmr0(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "desc TMR0."]
        #[inline(always)]
        pub const fn set_tmr0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "desc TMR2."]
        #[must_use]
        #[inline(always)]
        pub const fn tmr2(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "desc TMR2."]
        #[inline(always)]
        pub const fn set_tmr2(&mut self, val: bool) {
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
        #[doc = "desc UART1."]
        #[must_use]
        #[inline(always)]
        pub const fn uart1(&self) -> bool {
            let val = (self.0 >> 24usize) & 0x01;
            val != 0
        }
        #[doc = "desc UART1."]
        #[inline(always)]
        pub const fn set_uart1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
        }
        #[doc = "desc UART2."]
        #[must_use]
        #[inline(always)]
        pub const fn uart2(&self) -> bool {
            let val = (self.0 >> 25usize) & 0x01;
            val != 0
        }
        #[doc = "desc UART2."]
        #[inline(always)]
        pub const fn set_uart2(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
        }
        #[doc = "desc UART3."]
        #[must_use]
        #[inline(always)]
        pub const fn uart3(&self) -> bool {
            let val = (self.0 >> 26usize) & 0x01;
            val != 0
        }
        #[doc = "desc UART3."]
        #[inline(always)]
        pub const fn set_uart3(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
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
        #[doc = "desc SPI."]
        #[must_use]
        #[inline(always)]
        pub const fn spi(&self) -> bool {
            let val = (self.0 >> 29usize) & 0x01;
            val != 0
        }
        #[doc = "desc SPI."]
        #[inline(always)]
        pub const fn set_spi(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
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
                .field("tmrb1", &self.tmrb1())
                .field("tmrb2", &self.tmrb2())
                .field("tmrb3", &self.tmrb3())
                .field("tmrb4", &self.tmrb4())
                .field("tmr0", &self.tmr0())
                .field("tmr2", &self.tmr2())
                .field("tmr4", &self.tmr4())
                .field("tmra", &self.tmra())
                .field("emb", &self.emb())
                .field("uart1", &self.uart1())
                .field("uart2", &self.uart2())
                .field("uart3", &self.uart3())
                .field("i2c", &self.i2c())
                .field("spi", &self.spi())
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
    #[doc = "desc XTALCFGR."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Xtalcfgr(pub u8);
    impl Xtalcfgr {
        #[doc = "desc XTALDRV."]
        #[must_use]
        #[inline(always)]
        pub const fn xtaldrv(&self) -> u8 {
            let val = (self.0 >> 4usize) & 0x03;
            val as u8
        }
        #[doc = "desc XTALDRV."]
        #[inline(always)]
        pub const fn set_xtaldrv(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 4usize)) | (((val as u8) & 0x03) << 4usize);
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
