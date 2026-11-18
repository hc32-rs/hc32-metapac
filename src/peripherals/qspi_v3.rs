#![allow(clippy::missing_safety_doc)]
#![allow(clippy::identity_op)]
#![allow(clippy::unnecessary_cast)]
#![allow(clippy::erasing_op)]

#[doc = "desc QSPI."]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Qspi {
    ptr: *mut u8,
}
unsafe impl Send for Qspi {}
unsafe impl Sync for Qspi {}
impl Qspi {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "desc CR."]
    #[inline(always)]
    pub const fn cr(self) -> crate::common::Reg<regs::Cr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0usize) as _) }
    }
    #[doc = "desc CSCR."]
    #[inline(always)]
    pub const fn cscr(self) -> crate::common::Reg<regs::Cscr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x04usize) as _) }
    }
    #[doc = "desc FCR."]
    #[inline(always)]
    pub const fn fcr(self) -> crate::common::Reg<regs::Fcr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x08usize) as _) }
    }
    #[doc = "desc SR."]
    #[inline(always)]
    pub const fn sr(self) -> crate::common::Reg<regs::Sr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0cusize) as _) }
    }
    #[doc = "desc DCOM."]
    #[inline(always)]
    pub const fn dcom(self) -> crate::common::Reg<regs::Dcom, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x10usize) as _) }
    }
    #[doc = "desc CCMD."]
    #[inline(always)]
    pub const fn ccmd(self) -> crate::common::Reg<regs::Ccmd, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x14usize) as _) }
    }
    #[doc = "desc XCMD."]
    #[inline(always)]
    pub const fn xcmd(self) -> crate::common::Reg<regs::Xcmd, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x18usize) as _) }
    }
    #[doc = "desc SR2."]
    #[inline(always)]
    pub const fn sr2(self) -> crate::common::Reg<regs::Sr2, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x24usize) as _) }
    }
    #[doc = "desc EXAR."]
    #[inline(always)]
    pub const fn exar(self) -> crate::common::Reg<regs::Exar, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0804usize) as _) }
    }
}
pub mod regs {
    #[doc = "desc CCMD."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ccmd(pub u32);
    impl Ccmd {
        #[doc = "desc RIC."]
        #[must_use]
        #[inline(always)]
        pub const fn ric(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[doc = "desc RIC."]
        #[inline(always)]
        pub const fn set_ric(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
        }
    }
    impl Default for Ccmd {
        #[inline(always)]
        fn default() -> Ccmd {
            Ccmd(0)
        }
    }
    impl core::fmt::Debug for Ccmd {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Ccmd").field("ric", &self.ric()).finish()
        }
    }
    #[doc = "desc CR."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Cr(pub u32);
    impl Cr {
        #[doc = "desc MDSEL."]
        #[must_use]
        #[inline(always)]
        pub const fn mdsel(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x07;
            val as u8
        }
        #[doc = "desc MDSEL."]
        #[inline(always)]
        pub const fn set_mdsel(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 0usize)) | (((val as u32) & 0x07) << 0usize);
        }
        #[doc = "desc PFE."]
        #[must_use]
        #[inline(always)]
        pub const fn pfe(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "desc PFE."]
        #[inline(always)]
        pub const fn set_pfe(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "desc PFSAE."]
        #[must_use]
        #[inline(always)]
        pub const fn pfsae(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "desc PFSAE."]
        #[inline(always)]
        pub const fn set_pfsae(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "desc DCOME."]
        #[must_use]
        #[inline(always)]
        pub const fn dcome(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "desc DCOME."]
        #[inline(always)]
        pub const fn set_dcome(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "desc XIPE."]
        #[must_use]
        #[inline(always)]
        pub const fn xipe(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "desc XIPE."]
        #[inline(always)]
        pub const fn set_xipe(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "desc SPIMD3."]
        #[must_use]
        #[inline(always)]
        pub const fn spimd3(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "desc SPIMD3."]
        #[inline(always)]
        pub const fn set_spimd3(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "desc IPRSL."]
        #[must_use]
        #[inline(always)]
        pub const fn iprsl(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0x03;
            val as u8
        }
        #[doc = "desc IPRSL."]
        #[inline(always)]
        pub const fn set_iprsl(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 8usize)) | (((val as u32) & 0x03) << 8usize);
        }
        #[doc = "desc APRSL."]
        #[must_use]
        #[inline(always)]
        pub const fn aprsl(&self) -> u8 {
            let val = (self.0 >> 10usize) & 0x03;
            val as u8
        }
        #[doc = "desc APRSL."]
        #[inline(always)]
        pub const fn set_aprsl(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 10usize)) | (((val as u32) & 0x03) << 10usize);
        }
        #[doc = "desc DPRSL."]
        #[must_use]
        #[inline(always)]
        pub const fn dprsl(&self) -> u8 {
            let val = (self.0 >> 12usize) & 0x03;
            val as u8
        }
        #[doc = "desc DPRSL."]
        #[inline(always)]
        pub const fn set_dprsl(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 12usize)) | (((val as u32) & 0x03) << 12usize);
        }
        #[doc = "desc DIV."]
        #[must_use]
        #[inline(always)]
        pub const fn div(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x3f;
            val as u8
        }
        #[doc = "desc DIV."]
        #[inline(always)]
        pub const fn set_div(&mut self, val: u8) {
            self.0 = (self.0 & !(0x3f << 16usize)) | (((val as u32) & 0x3f) << 16usize);
        }
    }
    impl Default for Cr {
        #[inline(always)]
        fn default() -> Cr {
            Cr(0)
        }
    }
    impl core::fmt::Debug for Cr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Cr")
                .field("mdsel", &self.mdsel())
                .field("pfe", &self.pfe())
                .field("pfsae", &self.pfsae())
                .field("dcome", &self.dcome())
                .field("xipe", &self.xipe())
                .field("spimd3", &self.spimd3())
                .field("iprsl", &self.iprsl())
                .field("aprsl", &self.aprsl())
                .field("dprsl", &self.dprsl())
                .field("div", &self.div())
                .finish()
        }
    }
    #[doc = "desc CSCR."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Cscr(pub u32);
    impl Cscr {
        #[doc = "desc SSHW."]
        #[must_use]
        #[inline(always)]
        pub const fn sshw(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x0f;
            val as u8
        }
        #[doc = "desc SSHW."]
        #[inline(always)]
        pub const fn set_sshw(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
        }
        #[doc = "desc SSNW."]
        #[must_use]
        #[inline(always)]
        pub const fn ssnw(&self) -> u8 {
            let val = (self.0 >> 4usize) & 0x03;
            val as u8
        }
        #[doc = "desc SSNW."]
        #[inline(always)]
        pub const fn set_ssnw(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 4usize)) | (((val as u32) & 0x03) << 4usize);
        }
    }
    impl Default for Cscr {
        #[inline(always)]
        fn default() -> Cscr {
            Cscr(0)
        }
    }
    impl core::fmt::Debug for Cscr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Cscr")
                .field("sshw", &self.sshw())
                .field("ssnw", &self.ssnw())
                .finish()
        }
    }
    #[doc = "desc DCOM."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Dcom(pub u32);
    impl Dcom {
        #[doc = "desc DCOM."]
        #[must_use]
        #[inline(always)]
        pub const fn dcom(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[doc = "desc DCOM."]
        #[inline(always)]
        pub const fn set_dcom(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
        }
    }
    impl Default for Dcom {
        #[inline(always)]
        fn default() -> Dcom {
            Dcom(0)
        }
    }
    impl core::fmt::Debug for Dcom {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Dcom").field("dcom", &self.dcom()).finish()
        }
    }
    #[doc = "desc EXAR."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Exar(pub u32);
    impl Exar {
        #[doc = "desc EXADR."]
        #[must_use]
        #[inline(always)]
        pub const fn exadr(&self) -> u8 {
            let val = (self.0 >> 26usize) & 0x3f;
            val as u8
        }
        #[doc = "desc EXADR."]
        #[inline(always)]
        pub const fn set_exadr(&mut self, val: u8) {
            self.0 = (self.0 & !(0x3f << 26usize)) | (((val as u32) & 0x3f) << 26usize);
        }
    }
    impl Default for Exar {
        #[inline(always)]
        fn default() -> Exar {
            Exar(0)
        }
    }
    impl core::fmt::Debug for Exar {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Exar")
                .field("exadr", &self.exadr())
                .finish()
        }
    }
    #[doc = "desc FCR."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Fcr(pub u32);
    impl Fcr {
        #[doc = "desc AWSL."]
        #[must_use]
        #[inline(always)]
        pub const fn awsl(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x03;
            val as u8
        }
        #[doc = "desc AWSL."]
        #[inline(always)]
        pub const fn set_awsl(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 0usize)) | (((val as u32) & 0x03) << 0usize);
        }
        #[doc = "desc FOUR_BIC."]
        #[must_use]
        #[inline(always)]
        pub const fn four_bic(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "desc FOUR_BIC."]
        #[inline(always)]
        pub const fn set_four_bic(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "desc SSNHD."]
        #[must_use]
        #[inline(always)]
        pub const fn ssnhd(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "desc SSNHD."]
        #[inline(always)]
        pub const fn set_ssnhd(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "desc SSNLD."]
        #[must_use]
        #[inline(always)]
        pub const fn ssnld(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "desc SSNLD."]
        #[inline(always)]
        pub const fn set_ssnld(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "desc WPOL."]
        #[must_use]
        #[inline(always)]
        pub const fn wpol(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "desc WPOL."]
        #[inline(always)]
        pub const fn set_wpol(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "desc DMCYCN."]
        #[must_use]
        #[inline(always)]
        pub const fn dmcycn(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0x0f;
            val as u8
        }
        #[doc = "desc DMCYCN."]
        #[inline(always)]
        pub const fn set_dmcycn(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 8usize)) | (((val as u32) & 0x0f) << 8usize);
        }
        #[doc = "desc DUTY."]
        #[must_use]
        #[inline(always)]
        pub const fn duty(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "desc DUTY."]
        #[inline(always)]
        pub const fn set_duty(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
    }
    impl Default for Fcr {
        #[inline(always)]
        fn default() -> Fcr {
            Fcr(0)
        }
    }
    impl core::fmt::Debug for Fcr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Fcr")
                .field("awsl", &self.awsl())
                .field("four_bic", &self.four_bic())
                .field("ssnhd", &self.ssnhd())
                .field("ssnld", &self.ssnld())
                .field("wpol", &self.wpol())
                .field("dmcycn", &self.dmcycn())
                .field("duty", &self.duty())
                .finish()
        }
    }
    #[doc = "desc SR."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Sr(pub u32);
    impl Sr {
        #[doc = "desc BUSY."]
        #[must_use]
        #[inline(always)]
        pub const fn busy(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "desc BUSY."]
        #[inline(always)]
        pub const fn set_busy(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "desc XIPF."]
        #[must_use]
        #[inline(always)]
        pub const fn xipf(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "desc XIPF."]
        #[inline(always)]
        pub const fn set_xipf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "desc RAER."]
        #[must_use]
        #[inline(always)]
        pub const fn raer(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "desc RAER."]
        #[inline(always)]
        pub const fn set_raer(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "desc PFNUM."]
        #[must_use]
        #[inline(always)]
        pub const fn pfnum(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0x1f;
            val as u8
        }
        #[doc = "desc PFNUM."]
        #[inline(always)]
        pub const fn set_pfnum(&mut self, val: u8) {
            self.0 = (self.0 & !(0x1f << 8usize)) | (((val as u32) & 0x1f) << 8usize);
        }
        #[doc = "desc PFFUL."]
        #[must_use]
        #[inline(always)]
        pub const fn pfful(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "desc PFFUL."]
        #[inline(always)]
        pub const fn set_pfful(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[doc = "desc PFAN."]
        #[must_use]
        #[inline(always)]
        pub const fn pfan(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "desc PFAN."]
        #[inline(always)]
        pub const fn set_pfan(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
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
                .field("busy", &self.busy())
                .field("xipf", &self.xipf())
                .field("raer", &self.raer())
                .field("pfnum", &self.pfnum())
                .field("pfful", &self.pfful())
                .field("pfan", &self.pfan())
                .finish()
        }
    }
    #[doc = "desc SR2."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Sr2(pub u32);
    impl Sr2 {
        #[doc = "desc RAERCLR."]
        #[must_use]
        #[inline(always)]
        pub const fn raerclr(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "desc RAERCLR."]
        #[inline(always)]
        pub const fn set_raerclr(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
    }
    impl Default for Sr2 {
        #[inline(always)]
        fn default() -> Sr2 {
            Sr2(0)
        }
    }
    impl core::fmt::Debug for Sr2 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Sr2")
                .field("raerclr", &self.raerclr())
                .finish()
        }
    }
    #[doc = "desc XCMD."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Xcmd(pub u32);
    impl Xcmd {
        #[doc = "desc XIPMC."]
        #[must_use]
        #[inline(always)]
        pub const fn xipmc(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[doc = "desc XIPMC."]
        #[inline(always)]
        pub const fn set_xipmc(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
        }
    }
    impl Default for Xcmd {
        #[inline(always)]
        fn default() -> Xcmd {
            Xcmd(0)
        }
    }
    impl core::fmt::Debug for Xcmd {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Xcmd")
                .field("xipmc", &self.xipmc())
                .finish()
        }
    }
}
