#![allow(clippy::missing_safety_doc)]
#![allow(clippy::identity_op)]
#![allow(clippy::unnecessary_cast)]
#![allow(clippy::erasing_op)]

#[doc = "desc I2S."]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct I2s {
    ptr: *mut u8,
}
unsafe impl Send for I2s {}
unsafe impl Sync for I2s {}
impl I2s {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "desc CTRL."]
    #[inline(always)]
    pub const fn ctrl(self) -> crate::common::Reg<regs::Ctrl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0usize) as _) }
    }
    #[doc = "desc SR."]
    #[inline(always)]
    pub const fn sr(self) -> crate::common::Reg<regs::Sr, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x04usize) as _) }
    }
    #[doc = "desc ER."]
    #[inline(always)]
    pub const fn er(self) -> crate::common::Reg<regs::Er, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x08usize) as _) }
    }
    #[doc = "desc CFGR."]
    #[inline(always)]
    pub const fn cfgr(self) -> crate::common::Reg<regs::Cfgr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0cusize) as _) }
    }
    #[doc = "desc TXBUF."]
    #[inline(always)]
    pub const fn txbuf(self) -> crate::common::Reg<u32, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x10usize) as _) }
    }
    #[doc = "desc RXBUF."]
    #[inline(always)]
    pub const fn rxbuf(self) -> crate::common::Reg<u32, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x14usize) as _) }
    }
    #[doc = "desc PR."]
    #[inline(always)]
    pub const fn pr(self) -> crate::common::Reg<regs::Pr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x18usize) as _) }
    }
}
pub mod regs {
    #[doc = "desc CFGR."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Cfgr(pub u32);
    impl Cfgr {
        #[doc = "desc I2SSTD."]
        #[must_use]
        #[inline(always)]
        pub const fn i2sstd(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x03;
            val as u8
        }
        #[doc = "desc I2SSTD."]
        #[inline(always)]
        pub const fn set_i2sstd(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 0usize)) | (((val as u32) & 0x03) << 0usize);
        }
        #[doc = "desc DATLEN."]
        #[must_use]
        #[inline(always)]
        pub const fn datlen(&self) -> u8 {
            let val = (self.0 >> 2usize) & 0x03;
            val as u8
        }
        #[doc = "desc DATLEN."]
        #[inline(always)]
        pub const fn set_datlen(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 2usize)) | (((val as u32) & 0x03) << 2usize);
        }
        #[doc = "desc CHLEN."]
        #[must_use]
        #[inline(always)]
        pub const fn chlen(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "desc CHLEN."]
        #[inline(always)]
        pub const fn set_chlen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "desc PCMSYNC."]
        #[must_use]
        #[inline(always)]
        pub const fn pcmsync(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "desc PCMSYNC."]
        #[inline(always)]
        pub const fn set_pcmsync(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
    }
    impl Default for Cfgr {
        #[inline(always)]
        fn default() -> Cfgr {
            Cfgr(0)
        }
    }
    impl core::fmt::Debug for Cfgr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Cfgr")
                .field("i2sstd", &self.i2sstd())
                .field("datlen", &self.datlen())
                .field("chlen", &self.chlen())
                .field("pcmsync", &self.pcmsync())
                .finish()
        }
    }
    #[doc = "desc CTRL."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ctrl(pub u32);
    impl Ctrl {
        #[doc = "desc TXE."]
        #[must_use]
        #[inline(always)]
        pub const fn txe(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "desc TXE."]
        #[inline(always)]
        pub const fn set_txe(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "desc TXIE."]
        #[must_use]
        #[inline(always)]
        pub const fn txie(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "desc TXIE."]
        #[inline(always)]
        pub const fn set_txie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "desc RXE."]
        #[must_use]
        #[inline(always)]
        pub const fn rxe(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "desc RXE."]
        #[inline(always)]
        pub const fn set_rxe(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "desc RXIE."]
        #[must_use]
        #[inline(always)]
        pub const fn rxie(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "desc RXIE."]
        #[inline(always)]
        pub const fn set_rxie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "desc EIE."]
        #[must_use]
        #[inline(always)]
        pub const fn eie(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "desc EIE."]
        #[inline(always)]
        pub const fn set_eie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "desc WMS."]
        #[must_use]
        #[inline(always)]
        pub const fn wms(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "desc WMS."]
        #[inline(always)]
        pub const fn set_wms(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "desc ODD."]
        #[must_use]
        #[inline(always)]
        pub const fn odd(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "desc ODD."]
        #[inline(always)]
        pub const fn set_odd(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "desc MCKOE."]
        #[must_use]
        #[inline(always)]
        pub const fn mckoe(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "desc MCKOE."]
        #[inline(always)]
        pub const fn set_mckoe(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "desc TXBIRQWL."]
        #[must_use]
        #[inline(always)]
        pub const fn txbirqwl(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0x07;
            val as u8
        }
        #[doc = "desc TXBIRQWL."]
        #[inline(always)]
        pub const fn set_txbirqwl(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 8usize)) | (((val as u32) & 0x07) << 8usize);
        }
        #[doc = "desc RXBIRQWL."]
        #[must_use]
        #[inline(always)]
        pub const fn rxbirqwl(&self) -> u8 {
            let val = (self.0 >> 12usize) & 0x07;
            val as u8
        }
        #[doc = "desc RXBIRQWL."]
        #[inline(always)]
        pub const fn set_rxbirqwl(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 12usize)) | (((val as u32) & 0x07) << 12usize);
        }
        #[doc = "desc FIFOR."]
        #[must_use]
        #[inline(always)]
        pub const fn fifor(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "desc FIFOR."]
        #[inline(always)]
        pub const fn set_fifor(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "desc I2SPLLSEL."]
        #[must_use]
        #[inline(always)]
        pub const fn i2spllsel(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[doc = "desc I2SPLLSEL."]
        #[inline(always)]
        pub const fn set_i2spllsel(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
        #[doc = "desc SDOE."]
        #[must_use]
        #[inline(always)]
        pub const fn sdoe(&self) -> bool {
            let val = (self.0 >> 19usize) & 0x01;
            val != 0
        }
        #[doc = "desc SDOE."]
        #[inline(always)]
        pub const fn set_sdoe(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
        }
        #[doc = "desc LRCKOE."]
        #[must_use]
        #[inline(always)]
        pub const fn lrckoe(&self) -> bool {
            let val = (self.0 >> 20usize) & 0x01;
            val != 0
        }
        #[doc = "desc LRCKOE."]
        #[inline(always)]
        pub const fn set_lrckoe(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
        }
        #[doc = "desc CKOE."]
        #[must_use]
        #[inline(always)]
        pub const fn ckoe(&self) -> bool {
            let val = (self.0 >> 21usize) & 0x01;
            val != 0
        }
        #[doc = "desc CKOE."]
        #[inline(always)]
        pub const fn set_ckoe(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
        }
        #[doc = "desc DUPLEX."]
        #[must_use]
        #[inline(always)]
        pub const fn duplex(&self) -> bool {
            let val = (self.0 >> 22usize) & 0x01;
            val != 0
        }
        #[doc = "desc DUPLEX."]
        #[inline(always)]
        pub const fn set_duplex(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
        }
        #[doc = "desc CLKSEL."]
        #[must_use]
        #[inline(always)]
        pub const fn clksel(&self) -> bool {
            let val = (self.0 >> 23usize) & 0x01;
            val != 0
        }
        #[doc = "desc CLKSEL."]
        #[inline(always)]
        pub const fn set_clksel(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
        }
        #[doc = "desc SRST."]
        #[must_use]
        #[inline(always)]
        pub const fn srst(&self) -> bool {
            let val = (self.0 >> 27usize) & 0x01;
            val != 0
        }
        #[doc = "desc SRST."]
        #[inline(always)]
        pub const fn set_srst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
        }
    }
    impl Default for Ctrl {
        #[inline(always)]
        fn default() -> Ctrl {
            Ctrl(0)
        }
    }
    impl core::fmt::Debug for Ctrl {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Ctrl")
                .field("txe", &self.txe())
                .field("txie", &self.txie())
                .field("rxe", &self.rxe())
                .field("rxie", &self.rxie())
                .field("eie", &self.eie())
                .field("wms", &self.wms())
                .field("odd", &self.odd())
                .field("mckoe", &self.mckoe())
                .field("txbirqwl", &self.txbirqwl())
                .field("rxbirqwl", &self.rxbirqwl())
                .field("fifor", &self.fifor())
                .field("i2spllsel", &self.i2spllsel())
                .field("sdoe", &self.sdoe())
                .field("lrckoe", &self.lrckoe())
                .field("ckoe", &self.ckoe())
                .field("duplex", &self.duplex())
                .field("clksel", &self.clksel())
                .field("srst", &self.srst())
                .finish()
        }
    }
    #[doc = "desc ER."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Er(pub u32);
    impl Er {
        #[doc = "desc TXERR."]
        #[must_use]
        #[inline(always)]
        pub const fn txerr(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "desc TXERR."]
        #[inline(always)]
        pub const fn set_txerr(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "desc RXERR."]
        #[must_use]
        #[inline(always)]
        pub const fn rxerr(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "desc RXERR."]
        #[inline(always)]
        pub const fn set_rxerr(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
    }
    impl Default for Er {
        #[inline(always)]
        fn default() -> Er {
            Er(0)
        }
    }
    impl core::fmt::Debug for Er {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Er")
                .field("txerr", &self.txerr())
                .field("rxerr", &self.rxerr())
                .finish()
        }
    }
    #[doc = "desc PR."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Pr(pub u32);
    impl Pr {
        #[doc = "desc I2SDIV."]
        #[must_use]
        #[inline(always)]
        pub const fn i2sdiv(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[doc = "desc I2SDIV."]
        #[inline(always)]
        pub const fn set_i2sdiv(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
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
                .field("i2sdiv", &self.i2sdiv())
                .finish()
        }
    }
    #[doc = "desc SR."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Sr(pub u32);
    impl Sr {
        #[doc = "desc TXBA."]
        #[must_use]
        #[inline(always)]
        pub const fn txba(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "desc TXBA."]
        #[inline(always)]
        pub const fn set_txba(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "desc RXBA."]
        #[must_use]
        #[inline(always)]
        pub const fn rxba(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "desc RXBA."]
        #[inline(always)]
        pub const fn set_rxba(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "desc TXBE."]
        #[must_use]
        #[inline(always)]
        pub const fn txbe(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "desc TXBE."]
        #[inline(always)]
        pub const fn set_txbe(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "desc TXBF."]
        #[must_use]
        #[inline(always)]
        pub const fn txbf(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "desc TXBF."]
        #[inline(always)]
        pub const fn set_txbf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "desc RXBE."]
        #[must_use]
        #[inline(always)]
        pub const fn rxbe(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "desc RXBE."]
        #[inline(always)]
        pub const fn set_rxbe(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "desc RXBF."]
        #[must_use]
        #[inline(always)]
        pub const fn rxbf(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "desc RXBF."]
        #[inline(always)]
        pub const fn set_rxbf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "desc RXID."]
        #[must_use]
        #[inline(always)]
        pub const fn rxid(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "desc RXID."]
        #[inline(always)]
        pub const fn set_rxid(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
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
                .field("txba", &self.txba())
                .field("rxba", &self.rxba())
                .field("txbe", &self.txbe())
                .field("txbf", &self.txbf())
                .field("rxbe", &self.rxbe())
                .field("rxbf", &self.rxbf())
                .field("rxid", &self.rxid())
                .finish()
        }
    }
}
