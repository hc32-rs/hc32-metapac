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
    #[doc = "desc XTALDIVR."]
    #[inline(always)]
    pub const fn xtaldivr(self) -> crate::common::Reg<regs::Xtaldivr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x18usize) as _) }
    }
    #[doc = "desc XTALDIVCR."]
    #[inline(always)]
    pub const fn xtaldivcr(self) -> crate::common::Reg<regs::Xtaldivcr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1cusize) as _) }
    }
    #[doc = "desc XTALCFGR."]
    #[inline(always)]
    pub const fn xtalcfgr(self) -> crate::common::Reg<regs::Xtalcfgr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x4c50usize) as _) }
    }
    #[doc = "desc XTAL32CR."]
    #[inline(always)]
    pub const fn xtal32cr(self) -> crate::common::Reg<regs::Xtal32cr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x4c54usize) as _) }
    }
    #[doc = "desc XTAL32CFGR."]
    #[inline(always)]
    pub const fn xtal32cfgr(self) -> crate::common::Reg<regs::Xtal32cfgr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x4c58usize) as _) }
    }
    #[doc = "desc XTAL32NFR."]
    #[inline(always)]
    pub const fn xtal32nfr(self) -> crate::common::Reg<regs::Xtal32nfr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x4c68usize) as _) }
    }
    #[doc = "desc LRCCR."]
    #[inline(always)]
    pub const fn lrccr(self) -> crate::common::Reg<regs::Lrccr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x4c6cusize) as _) }
    }
    #[doc = "desc LRCTRM."]
    #[inline(always)]
    pub const fn lrctrm(self) -> crate::common::Reg<u8, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x4c74usize) as _) }
    }
    #[doc = "desc PERICKSEL."]
    #[inline(always)]
    pub const fn pericksel(self) -> crate::common::Reg<regs::Pericksel, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xc010usize) as _) }
    }
    #[doc = "desc CANCKCFGR."]
    #[inline(always)]
    pub const fn canckcfgr(self) -> crate::common::Reg<regs::Canckcfgr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xc018usize) as _) }
    }
    #[doc = "desc SCFGR."]
    #[inline(always)]
    pub const fn scfgr(self) -> crate::common::Reg<regs::Scfgr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xc020usize) as _) }
    }
    #[doc = "desc CKSWR."]
    #[inline(always)]
    pub const fn ckswr(self) -> crate::common::Reg<regs::Ckswr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xc026usize) as _) }
    }
    #[doc = "desc PLLHCR."]
    #[inline(always)]
    pub const fn pllhcr(self) -> crate::common::Reg<regs::Pllhcr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xc02ausize) as _) }
    }
    #[doc = "desc XTALCR."]
    #[inline(always)]
    pub const fn xtalcr(self) -> crate::common::Reg<regs::Xtalcr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xc032usize) as _) }
    }
    #[doc = "desc HRCCR."]
    #[inline(always)]
    pub const fn hrccr(self) -> crate::common::Reg<regs::Hrccr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xc036usize) as _) }
    }
    #[doc = "desc MRCCR."]
    #[inline(always)]
    pub const fn mrccr(self) -> crate::common::Reg<regs::Mrccr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xc038usize) as _) }
    }
    #[doc = "desc OSCSTBSR."]
    #[inline(always)]
    pub const fn oscstbsr(self) -> crate::common::Reg<regs::Oscstbsr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xc03cusize) as _) }
    }
    #[doc = "desc MCO1CFGR."]
    #[inline(always)]
    pub const fn mco1cfgr(self) -> crate::common::Reg<regs::Mco1cfgr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xc03dusize) as _) }
    }
    #[doc = "desc MCO2CFGR."]
    #[inline(always)]
    pub const fn mco2cfgr(self) -> crate::common::Reg<regs::Mco2cfgr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xc03eusize) as _) }
    }
    #[doc = "desc TPIUCKCFGR."]
    #[inline(always)]
    pub const fn tpiuckcfgr(self) -> crate::common::Reg<regs::Tpiuckcfgr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xc03fusize) as _) }
    }
    #[doc = "desc XTALSTDCR."]
    #[inline(always)]
    pub const fn xtalstdcr(self) -> crate::common::Reg<regs::Xtalstdcr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xc040usize) as _) }
    }
    #[doc = "desc XTALSTDSR."]
    #[inline(always)]
    pub const fn xtalstdsr(self) -> crate::common::Reg<regs::Xtalstdsr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xc041usize) as _) }
    }
    #[doc = "desc MRCTRM."]
    #[inline(always)]
    pub const fn mrctrm(self) -> crate::common::Reg<u8, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xc061usize) as _) }
    }
    #[doc = "desc HRCTRM."]
    #[inline(always)]
    pub const fn hrctrm(self) -> crate::common::Reg<u8, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xc062usize) as _) }
    }
    #[doc = "desc XTALSTBCR."]
    #[inline(always)]
    pub const fn xtalstbcr(self) -> crate::common::Reg<regs::Xtalstbcr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xc0a2usize) as _) }
    }
    #[doc = "desc PLLHCFGR."]
    #[inline(always)]
    pub const fn pllhcfgr(self) -> crate::common::Reg<regs::Pllhcfgr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xc100usize) as _) }
    }
}
pub mod regs {
    #[doc = "desc CANCKCFGR."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Canckcfgr(pub u8);
    impl Canckcfgr {
        #[doc = "desc MCAN1CKS."]
        #[must_use]
        #[inline(always)]
        pub const fn mcan1cks(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x0f;
            val as u8
        }
        #[doc = "desc MCAN1CKS."]
        #[inline(always)]
        pub const fn set_mcan1cks(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u8) & 0x0f) << 0usize);
        }
        #[doc = "desc MCAN2CKS."]
        #[must_use]
        #[inline(always)]
        pub const fn mcan2cks(&self) -> u8 {
            let val = (self.0 >> 4usize) & 0x0f;
            val as u8
        }
        #[doc = "desc MCAN2CKS."]
        #[inline(always)]
        pub const fn set_mcan2cks(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 4usize)) | (((val as u8) & 0x0f) << 4usize);
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
                .field("mcan1cks", &self.mcan1cks())
                .field("mcan2cks", &self.mcan2cks())
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
        #[doc = "desc PLLHSTBF."]
        #[must_use]
        #[inline(always)]
        pub const fn pllhstbf(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "desc PLLHSTBF."]
        #[inline(always)]
        pub const fn set_pllhstbf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u8) & 0x01) << 5usize);
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
                .field("pllhstbf", &self.pllhstbf())
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
    #[doc = "desc PLLHCFGR."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Pllhcfgr(pub u32);
    impl Pllhcfgr {
        #[doc = "desc PLLHM."]
        #[must_use]
        #[inline(always)]
        pub const fn pllhm(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x1f;
            val as u8
        }
        #[doc = "desc PLLHM."]
        #[inline(always)]
        pub const fn set_pllhm(&mut self, val: u8) {
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
        #[doc = "desc PLLHN."]
        #[must_use]
        #[inline(always)]
        pub const fn pllhn(&self) -> u16 {
            let val = (self.0 >> 8usize) & 0x01ff;
            val as u16
        }
        #[doc = "desc PLLHN."]
        #[inline(always)]
        pub const fn set_pllhn(&mut self, val: u16) {
            self.0 = (self.0 & !(0x01ff << 8usize)) | (((val as u32) & 0x01ff) << 8usize);
        }
        #[doc = "desc PLLHR."]
        #[must_use]
        #[inline(always)]
        pub const fn pllhr(&self) -> u8 {
            let val = (self.0 >> 20usize) & 0x0f;
            val as u8
        }
        #[doc = "desc PLLHR."]
        #[inline(always)]
        pub const fn set_pllhr(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 20usize)) | (((val as u32) & 0x0f) << 20usize);
        }
        #[doc = "desc PLLHQ."]
        #[must_use]
        #[inline(always)]
        pub const fn pllhq(&self) -> u8 {
            let val = (self.0 >> 24usize) & 0x0f;
            val as u8
        }
        #[doc = "desc PLLHQ."]
        #[inline(always)]
        pub const fn set_pllhq(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 24usize)) | (((val as u32) & 0x0f) << 24usize);
        }
        #[doc = "desc PLLHP."]
        #[must_use]
        #[inline(always)]
        pub const fn pllhp(&self) -> u8 {
            let val = (self.0 >> 28usize) & 0x0f;
            val as u8
        }
        #[doc = "desc PLLHP."]
        #[inline(always)]
        pub const fn set_pllhp(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 28usize)) | (((val as u32) & 0x0f) << 28usize);
        }
    }
    impl Default for Pllhcfgr {
        #[inline(always)]
        fn default() -> Pllhcfgr {
            Pllhcfgr(0)
        }
    }
    impl core::fmt::Debug for Pllhcfgr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Pllhcfgr")
                .field("pllhm", &self.pllhm())
                .field("pllsrc", &self.pllsrc())
                .field("pllhn", &self.pllhn())
                .field("pllhr", &self.pllhr())
                .field("pllhq", &self.pllhq())
                .field("pllhp", &self.pllhp())
                .finish()
        }
    }
    #[doc = "desc PLLHCR."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Pllhcr(pub u8);
    impl Pllhcr {
        #[doc = "desc PLLHOFF."]
        #[must_use]
        #[inline(always)]
        pub const fn pllhoff(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "desc PLLHOFF."]
        #[inline(always)]
        pub const fn set_pllhoff(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u8) & 0x01) << 0usize);
        }
    }
    impl Default for Pllhcr {
        #[inline(always)]
        fn default() -> Pllhcr {
            Pllhcr(0)
        }
    }
    impl core::fmt::Debug for Pllhcr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Pllhcr")
                .field("pllhoff", &self.pllhoff())
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
        #[doc = "desc XTALMS."]
        #[must_use]
        #[inline(always)]
        pub const fn xtalms(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "desc XTALMS."]
        #[inline(always)]
        pub const fn set_xtalms(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u8) & 0x01) << 3usize);
        }
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
                .field("xtalms", &self.xtalms())
                .field("xtaldrv", &self.xtaldrv())
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
    #[doc = "desc XTALDIVCR."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Xtaldivcr(pub u32);
    impl Xtaldivcr {
        #[doc = "desc FRADIVEN."]
        #[must_use]
        #[inline(always)]
        pub const fn fradiven(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "desc FRADIVEN."]
        #[inline(always)]
        pub const fn set_fradiven(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
    }
    impl Default for Xtaldivcr {
        #[inline(always)]
        fn default() -> Xtaldivcr {
            Xtaldivcr(0)
        }
    }
    impl core::fmt::Debug for Xtaldivcr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Xtaldivcr")
                .field("fradiven", &self.fradiven())
                .finish()
        }
    }
    #[doc = "desc XTALDIVR."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Xtaldivr(pub u32);
    impl Xtaldivr {
        #[doc = "desc DENOM."]
        #[must_use]
        #[inline(always)]
        pub const fn denom(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x07ff;
            val as u16
        }
        #[doc = "desc DENOM."]
        #[inline(always)]
        pub const fn set_denom(&mut self, val: u16) {
            self.0 = (self.0 & !(0x07ff << 0usize)) | (((val as u32) & 0x07ff) << 0usize);
        }
        #[doc = "desc NUMER."]
        #[must_use]
        #[inline(always)]
        pub const fn numer(&self) -> u32 {
            let val = (self.0 >> 12usize) & 0x0001_ffff;
            val as u32
        }
        #[doc = "desc NUMER."]
        #[inline(always)]
        pub const fn set_numer(&mut self, val: u32) {
            self.0 =
                (self.0 & !(0x0001_ffff << 12usize)) | (((val as u32) & 0x0001_ffff) << 12usize);
        }
    }
    impl Default for Xtaldivr {
        #[inline(always)]
        fn default() -> Xtaldivr {
            Xtaldivr(0)
        }
    }
    impl core::fmt::Debug for Xtaldivr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Xtaldivr")
                .field("denom", &self.denom())
                .field("numer", &self.numer())
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
