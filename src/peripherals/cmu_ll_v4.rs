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
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x10usize) as _) }
    }
    #[doc = "desc I2SCKSEL."]
    #[inline(always)]
    pub const fn i2scksel(self) -> crate::common::Reg<regs::I2scksel, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x12usize) as _) }
    }
    #[doc = "desc SCFGR."]
    #[inline(always)]
    pub const fn scfgr(self) -> crate::common::Reg<regs::Scfgr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x20usize) as _) }
    }
    #[doc = "desc USBCKCFGR."]
    #[inline(always)]
    pub const fn usbckcfgr(self) -> crate::common::Reg<regs::Usbckcfgr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x24usize) as _) }
    }
    #[doc = "desc CKSWR."]
    #[inline(always)]
    pub const fn ckswr(self) -> crate::common::Reg<regs::Ckswr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x26usize) as _) }
    }
    #[doc = "desc PLLCR."]
    #[inline(always)]
    pub const fn pllcr(self) -> crate::common::Reg<regs::Pllcr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x2ausize) as _) }
    }
    #[doc = "desc UPLLCR."]
    #[inline(always)]
    pub const fn upllcr(self) -> crate::common::Reg<regs::Upllcr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x2eusize) as _) }
    }
    #[doc = "desc XTALCR."]
    #[inline(always)]
    pub const fn xtalcr(self) -> crate::common::Reg<regs::Xtalcr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x32usize) as _) }
    }
    #[doc = "desc HRCCR."]
    #[inline(always)]
    pub const fn hrccr(self) -> crate::common::Reg<regs::Hrccr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x36usize) as _) }
    }
    #[doc = "desc MRCCR."]
    #[inline(always)]
    pub const fn mrccr(self) -> crate::common::Reg<regs::Mrccr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x38usize) as _) }
    }
    #[doc = "desc OSCSTBSR."]
    #[inline(always)]
    pub const fn oscstbsr(self) -> crate::common::Reg<regs::Oscstbsr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x3cusize) as _) }
    }
    #[doc = "desc MCO1CFGR."]
    #[inline(always)]
    pub const fn mco1cfgr(self) -> crate::common::Reg<regs::Mco1cfgr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x3dusize) as _) }
    }
    #[doc = "desc MCO2CFGR."]
    #[inline(always)]
    pub const fn mco2cfgr(self) -> crate::common::Reg<regs::Mco2cfgr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x3eusize) as _) }
    }
    #[doc = "desc TPIUCKCFGR."]
    #[inline(always)]
    pub const fn tpiuckcfgr(self) -> crate::common::Reg<regs::Tpiuckcfgr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x3fusize) as _) }
    }
    #[doc = "desc XTALSTDCR."]
    #[inline(always)]
    pub const fn xtalstdcr(self) -> crate::common::Reg<regs::Xtalstdcr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x40usize) as _) }
    }
    #[doc = "desc XTALSTDSR."]
    #[inline(always)]
    pub const fn xtalstdsr(self) -> crate::common::Reg<regs::Xtalstdsr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x41usize) as _) }
    }
    #[doc = "desc MRCTRM."]
    #[inline(always)]
    pub const fn mrctrm(self) -> crate::common::Reg<u8, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x61usize) as _) }
    }
    #[doc = "desc HRCTRM."]
    #[inline(always)]
    pub const fn hrctrm(self) -> crate::common::Reg<u8, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x62usize) as _) }
    }
    #[doc = "desc XTALSTBCR."]
    #[inline(always)]
    pub const fn xtalstbcr(self) -> crate::common::Reg<regs::Xtalstbcr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xa2usize) as _) }
    }
    #[doc = "desc PLLCFGR."]
    #[inline(always)]
    pub const fn pllcfgr(self) -> crate::common::Reg<regs::Pllcfgr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0100usize) as _) }
    }
    #[doc = "desc UPLLCFGR."]
    #[inline(always)]
    pub const fn upllcfgr(self) -> crate::common::Reg<regs::Upllcfgr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0104usize) as _) }
    }
    #[doc = "desc XTALCFGR."]
    #[inline(always)]
    pub const fn xtalcfgr(self) -> crate::common::Reg<regs::Xtalcfgr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0410usize) as _) }
    }
    #[doc = "desc XTAL32CR."]
    #[inline(always)]
    pub const fn xtal32cr(self) -> crate::common::Reg<regs::Xtal32cr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0420usize) as _) }
    }
    #[doc = "desc XTAL32CFGR."]
    #[inline(always)]
    pub const fn xtal32cfgr(self) -> crate::common::Reg<regs::Xtal32cfgr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0421usize) as _) }
    }
    #[doc = "desc XTAL32NFR."]
    #[inline(always)]
    pub const fn xtal32nfr(self) -> crate::common::Reg<regs::Xtal32nfr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0425usize) as _) }
    }
    #[doc = "desc LRCCR."]
    #[inline(always)]
    pub const fn lrccr(self) -> crate::common::Reg<regs::Lrccr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0427usize) as _) }
    }
    #[doc = "desc LRCTRM."]
    #[inline(always)]
    pub const fn lrctrm(self) -> crate::common::Reg<u8, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0429usize) as _) }
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
            let val = (self.0 >> 0usize) & 0x07;
            val as u8
        }
        #[doc = "desc CKSW."]
        #[inline(always)]
        pub const fn set_cksw(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 0usize)) | (((val as u8) & 0x07) << 0usize);
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
    #[doc = "desc I2SCKSEL."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct I2scksel(pub u16);
    impl I2scksel {
        #[doc = "desc I2S1CKSEL."]
        #[must_use]
        #[inline(always)]
        pub const fn i2s1cksel(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x0f;
            val as u8
        }
        #[doc = "desc I2S1CKSEL."]
        #[inline(always)]
        pub const fn set_i2s1cksel(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u16) & 0x0f) << 0usize);
        }
        #[doc = "desc I2S2CKSEL."]
        #[must_use]
        #[inline(always)]
        pub const fn i2s2cksel(&self) -> u8 {
            let val = (self.0 >> 4usize) & 0x0f;
            val as u8
        }
        #[doc = "desc I2S2CKSEL."]
        #[inline(always)]
        pub const fn set_i2s2cksel(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 4usize)) | (((val as u16) & 0x0f) << 4usize);
        }
        #[doc = "desc I2S3CKSEL."]
        #[must_use]
        #[inline(always)]
        pub const fn i2s3cksel(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0x0f;
            val as u8
        }
        #[doc = "desc I2S3CKSEL."]
        #[inline(always)]
        pub const fn set_i2s3cksel(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 8usize)) | (((val as u16) & 0x0f) << 8usize);
        }
        #[doc = "desc I2S4CKSEL."]
        #[must_use]
        #[inline(always)]
        pub const fn i2s4cksel(&self) -> u8 {
            let val = (self.0 >> 12usize) & 0x0f;
            val as u8
        }
        #[doc = "desc I2S4CKSEL."]
        #[inline(always)]
        pub const fn set_i2s4cksel(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 12usize)) | (((val as u16) & 0x0f) << 12usize);
        }
    }
    impl Default for I2scksel {
        #[inline(always)]
        fn default() -> I2scksel {
            I2scksel(0)
        }
    }
    impl core::fmt::Debug for I2scksel {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("I2scksel")
                .field("i2s1cksel", &self.i2s1cksel())
                .field("i2s2cksel", &self.i2s2cksel())
                .field("i2s3cksel", &self.i2s3cksel())
                .field("i2s4cksel", &self.i2s4cksel())
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
    #[doc = "desc MCO2CFGR."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Mco2cfgr(pub u8);
    impl Mco2cfgr {
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
    impl Default for Mco2cfgr {
        #[inline(always)]
        fn default() -> Mco2cfgr {
            Mco2cfgr(0)
        }
    }
    impl core::fmt::Debug for Mco2cfgr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Mco2cfgr")
                .field("mcosel", &self.mcosel())
                .field("mcodiv", &self.mcodiv())
                .field("mcoen", &self.mcoen())
                .finish()
        }
    }
    #[doc = "desc MRCCR."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Mrccr(pub u8);
    impl Mrccr {
        #[doc = "desc MRCSTP."]
        #[must_use]
        #[inline(always)]
        pub const fn mrcstp(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "desc MRCSTP."]
        #[inline(always)]
        pub const fn set_mrcstp(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u8) & 0x01) << 0usize);
        }
    }
    impl Default for Mrccr {
        #[inline(always)]
        fn default() -> Mrccr {
            Mrccr(0)
        }
    }
    impl core::fmt::Debug for Mrccr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Mrccr")
                .field("mrcstp", &self.mrcstp())
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
        #[doc = "desc MPLLSTBF."]
        #[must_use]
        #[inline(always)]
        pub const fn mpllstbf(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "desc MPLLSTBF."]
        #[inline(always)]
        pub const fn set_mpllstbf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u8) & 0x01) << 5usize);
        }
        #[doc = "desc UPLLSTBF."]
        #[must_use]
        #[inline(always)]
        pub const fn upllstbf(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "desc UPLLSTBF."]
        #[inline(always)]
        pub const fn set_upllstbf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u8) & 0x01) << 6usize);
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
                .field("mpllstbf", &self.mpllstbf())
                .field("upllstbf", &self.upllstbf())
                .finish()
        }
    }
    #[doc = "desc PERICKSEL."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Pericksel(pub u16);
    impl Pericksel {
        #[doc = "desc PERICKSEL."]
        #[must_use]
        #[inline(always)]
        pub const fn pericksel(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x0f;
            val as u8
        }
        #[doc = "desc PERICKSEL."]
        #[inline(always)]
        pub const fn set_pericksel(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u16) & 0x0f) << 0usize);
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
    #[doc = "desc PLLCFGR."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Pllcfgr(pub u32);
    impl Pllcfgr {
        #[doc = "desc MPLLM."]
        #[must_use]
        #[inline(always)]
        pub const fn mpllm(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x1f;
            val as u8
        }
        #[doc = "desc MPLLM."]
        #[inline(always)]
        pub const fn set_mpllm(&mut self, val: u8) {
            self.0 = (self.0 & !(0x1f << 0usize)) | (((val as u32) & 0x1f) << 0usize);
        }
        #[doc = "desc PLLSRC."]
        #[must_use]
        #[inline(always)]
        pub const fn pllsrc(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "desc PLLSRC."]
        #[inline(always)]
        pub const fn set_pllsrc(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "desc MPLLN."]
        #[must_use]
        #[inline(always)]
        pub const fn mplln(&self) -> u16 {
            let val = (self.0 >> 8usize) & 0x01ff;
            val as u16
        }
        #[doc = "desc MPLLN."]
        #[inline(always)]
        pub const fn set_mplln(&mut self, val: u16) {
            self.0 = (self.0 & !(0x01ff << 8usize)) | (((val as u32) & 0x01ff) << 8usize);
        }
        #[doc = "desc MPLLR."]
        #[must_use]
        #[inline(always)]
        pub const fn mpllr(&self) -> u8 {
            let val = (self.0 >> 20usize) & 0x0f;
            val as u8
        }
        #[doc = "desc MPLLR."]
        #[inline(always)]
        pub const fn set_mpllr(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 20usize)) | (((val as u32) & 0x0f) << 20usize);
        }
        #[doc = "desc MPLLQ."]
        #[must_use]
        #[inline(always)]
        pub const fn mpllq(&self) -> u8 {
            let val = (self.0 >> 24usize) & 0x0f;
            val as u8
        }
        #[doc = "desc MPLLQ."]
        #[inline(always)]
        pub const fn set_mpllq(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 24usize)) | (((val as u32) & 0x0f) << 24usize);
        }
        #[doc = "desc MPLLP."]
        #[must_use]
        #[inline(always)]
        pub const fn mpllp(&self) -> u8 {
            let val = (self.0 >> 28usize) & 0x0f;
            val as u8
        }
        #[doc = "desc MPLLP."]
        #[inline(always)]
        pub const fn set_mpllp(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 28usize)) | (((val as u32) & 0x0f) << 28usize);
        }
    }
    impl Default for Pllcfgr {
        #[inline(always)]
        fn default() -> Pllcfgr {
            Pllcfgr(0)
        }
    }
    impl core::fmt::Debug for Pllcfgr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Pllcfgr")
                .field("mpllm", &self.mpllm())
                .field("pllsrc", &self.pllsrc())
                .field("mplln", &self.mplln())
                .field("mpllr", &self.mpllr())
                .field("mpllq", &self.mpllq())
                .field("mpllp", &self.mpllp())
                .finish()
        }
    }
    #[doc = "desc PLLCR."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Pllcr(pub u8);
    impl Pllcr {
        #[doc = "desc MPLLOFF."]
        #[must_use]
        #[inline(always)]
        pub const fn mplloff(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "desc MPLLOFF."]
        #[inline(always)]
        pub const fn set_mplloff(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u8) & 0x01) << 0usize);
        }
    }
    impl Default for Pllcr {
        #[inline(always)]
        fn default() -> Pllcr {
            Pllcr(0)
        }
    }
    impl core::fmt::Debug for Pllcr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Pllcr")
                .field("mplloff", &self.mplloff())
                .finish()
        }
    }
    #[doc = "desc SCFGR."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Scfgr(pub u32);
    impl Scfgr {
        #[doc = "desc PCLK0S."]
        #[must_use]
        #[inline(always)]
        pub const fn pclk0s(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x07;
            val as u8
        }
        #[doc = "desc PCLK0S."]
        #[inline(always)]
        pub const fn set_pclk0s(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 0usize)) | (((val as u32) & 0x07) << 0usize);
        }
        #[doc = "desc PCLK1S."]
        #[must_use]
        #[inline(always)]
        pub const fn pclk1s(&self) -> u8 {
            let val = (self.0 >> 4usize) & 0x07;
            val as u8
        }
        #[doc = "desc PCLK1S."]
        #[inline(always)]
        pub const fn set_pclk1s(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 4usize)) | (((val as u32) & 0x07) << 4usize);
        }
        #[doc = "desc PCLK2S."]
        #[must_use]
        #[inline(always)]
        pub const fn pclk2s(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0x07;
            val as u8
        }
        #[doc = "desc PCLK2S."]
        #[inline(always)]
        pub const fn set_pclk2s(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 8usize)) | (((val as u32) & 0x07) << 8usize);
        }
        #[doc = "desc PCLK3S."]
        #[must_use]
        #[inline(always)]
        pub const fn pclk3s(&self) -> u8 {
            let val = (self.0 >> 12usize) & 0x07;
            val as u8
        }
        #[doc = "desc PCLK3S."]
        #[inline(always)]
        pub const fn set_pclk3s(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 12usize)) | (((val as u32) & 0x07) << 12usize);
        }
        #[doc = "desc PCLK4S."]
        #[must_use]
        #[inline(always)]
        pub const fn pclk4s(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x07;
            val as u8
        }
        #[doc = "desc PCLK4S."]
        #[inline(always)]
        pub const fn set_pclk4s(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 16usize)) | (((val as u32) & 0x07) << 16usize);
        }
        #[doc = "desc EXCKS."]
        #[must_use]
        #[inline(always)]
        pub const fn excks(&self) -> u8 {
            let val = (self.0 >> 20usize) & 0x07;
            val as u8
        }
        #[doc = "desc EXCKS."]
        #[inline(always)]
        pub const fn set_excks(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 20usize)) | (((val as u32) & 0x07) << 20usize);
        }
        #[doc = "desc HCLKS."]
        #[must_use]
        #[inline(always)]
        pub const fn hclks(&self) -> u8 {
            let val = (self.0 >> 24usize) & 0x07;
            val as u8
        }
        #[doc = "desc HCLKS."]
        #[inline(always)]
        pub const fn set_hclks(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 24usize)) | (((val as u32) & 0x07) << 24usize);
        }
    }
    impl Default for Scfgr {
        #[inline(always)]
        fn default() -> Scfgr {
            Scfgr(0)
        }
    }
    impl core::fmt::Debug for Scfgr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Scfgr")
                .field("pclk0s", &self.pclk0s())
                .field("pclk1s", &self.pclk1s())
                .field("pclk2s", &self.pclk2s())
                .field("pclk3s", &self.pclk3s())
                .field("pclk4s", &self.pclk4s())
                .field("excks", &self.excks())
                .field("hclks", &self.hclks())
                .finish()
        }
    }
    #[doc = "desc TPIUCKCFGR."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Tpiuckcfgr(pub u8);
    impl Tpiuckcfgr {
        #[doc = "desc TPIUCKS."]
        #[must_use]
        #[inline(always)]
        pub const fn tpiucks(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x03;
            val as u8
        }
        #[doc = "desc TPIUCKS."]
        #[inline(always)]
        pub const fn set_tpiucks(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 0usize)) | (((val as u8) & 0x03) << 0usize);
        }
        #[doc = "desc TPIUCKOE."]
        #[must_use]
        #[inline(always)]
        pub const fn tpiuckoe(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "desc TPIUCKOE."]
        #[inline(always)]
        pub const fn set_tpiuckoe(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u8) & 0x01) << 7usize);
        }
    }
    impl Default for Tpiuckcfgr {
        #[inline(always)]
        fn default() -> Tpiuckcfgr {
            Tpiuckcfgr(0)
        }
    }
    impl core::fmt::Debug for Tpiuckcfgr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Tpiuckcfgr")
                .field("tpiucks", &self.tpiucks())
                .field("tpiuckoe", &self.tpiuckoe())
                .finish()
        }
    }
    #[doc = "desc UPLLCFGR."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Upllcfgr(pub u32);
    impl Upllcfgr {
        #[doc = "desc UPLLM."]
        #[must_use]
        #[inline(always)]
        pub const fn upllm(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x1f;
            val as u8
        }
        #[doc = "desc UPLLM."]
        #[inline(always)]
        pub const fn set_upllm(&mut self, val: u8) {
            self.0 = (self.0 & !(0x1f << 0usize)) | (((val as u32) & 0x1f) << 0usize);
        }
        #[doc = "desc UPLLN."]
        #[must_use]
        #[inline(always)]
        pub const fn uplln(&self) -> u16 {
            let val = (self.0 >> 8usize) & 0x01ff;
            val as u16
        }
        #[doc = "desc UPLLN."]
        #[inline(always)]
        pub const fn set_uplln(&mut self, val: u16) {
            self.0 = (self.0 & !(0x01ff << 8usize)) | (((val as u32) & 0x01ff) << 8usize);
        }
        #[doc = "desc UPLLR."]
        #[must_use]
        #[inline(always)]
        pub const fn upllr(&self) -> u8 {
            let val = (self.0 >> 20usize) & 0x0f;
            val as u8
        }
        #[doc = "desc UPLLR."]
        #[inline(always)]
        pub const fn set_upllr(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 20usize)) | (((val as u32) & 0x0f) << 20usize);
        }
        #[doc = "desc UPLLQ."]
        #[must_use]
        #[inline(always)]
        pub const fn upllq(&self) -> u8 {
            let val = (self.0 >> 24usize) & 0x0f;
            val as u8
        }
        #[doc = "desc UPLLQ."]
        #[inline(always)]
        pub const fn set_upllq(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 24usize)) | (((val as u32) & 0x0f) << 24usize);
        }
        #[doc = "desc UPLLP."]
        #[must_use]
        #[inline(always)]
        pub const fn upllp(&self) -> u8 {
            let val = (self.0 >> 28usize) & 0x0f;
            val as u8
        }
        #[doc = "desc UPLLP."]
        #[inline(always)]
        pub const fn set_upllp(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 28usize)) | (((val as u32) & 0x0f) << 28usize);
        }
    }
    impl Default for Upllcfgr {
        #[inline(always)]
        fn default() -> Upllcfgr {
            Upllcfgr(0)
        }
    }
    impl core::fmt::Debug for Upllcfgr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Upllcfgr")
                .field("upllm", &self.upllm())
                .field("uplln", &self.uplln())
                .field("upllr", &self.upllr())
                .field("upllq", &self.upllq())
                .field("upllp", &self.upllp())
                .finish()
        }
    }
    #[doc = "desc UPLLCR."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Upllcr(pub u8);
    impl Upllcr {
        #[doc = "desc UPLLOFF."]
        #[must_use]
        #[inline(always)]
        pub const fn uplloff(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "desc UPLLOFF."]
        #[inline(always)]
        pub const fn set_uplloff(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u8) & 0x01) << 0usize);
        }
    }
    impl Default for Upllcr {
        #[inline(always)]
        fn default() -> Upllcr {
            Upllcr(0)
        }
    }
    impl core::fmt::Debug for Upllcr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Upllcr")
                .field("uplloff", &self.uplloff())
                .finish()
        }
    }
    #[doc = "desc USBCKCFGR."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Usbckcfgr(pub u8);
    impl Usbckcfgr {
        #[doc = "desc USBCKS."]
        #[must_use]
        #[inline(always)]
        pub const fn usbcks(&self) -> u8 {
            let val = (self.0 >> 4usize) & 0x0f;
            val as u8
        }
        #[doc = "desc USBCKS."]
        #[inline(always)]
        pub const fn set_usbcks(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 4usize)) | (((val as u8) & 0x0f) << 4usize);
        }
    }
    impl Default for Usbckcfgr {
        #[inline(always)]
        fn default() -> Usbckcfgr {
            Usbckcfgr(0)
        }
    }
    impl core::fmt::Debug for Usbckcfgr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Usbckcfgr")
                .field("usbcks", &self.usbcks())
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
        pub const fn xtal32drv(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x07;
            val as u8
        }
        #[doc = "desc XTAL32DRV."]
        #[inline(always)]
        pub const fn set_xtal32drv(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 0usize)) | (((val as u8) & 0x07) << 0usize);
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
            let val = (self.0 >> 0usize) & 0x0f;
            val as u8
        }
        #[doc = "desc XTALSTB."]
        #[inline(always)]
        pub const fn set_xtalstb(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u8) & 0x0f) << 0usize);
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
