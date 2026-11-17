#![allow(clippy::missing_safety_doc)]
#![allow(clippy::identity_op)]
#![allow(clippy::unnecessary_cast)]
#![allow(clippy::erasing_op)]

#[doc = "desc DMC."]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dmc {
    ptr: *mut u8,
}
unsafe impl Send for Dmc {}
unsafe impl Sync for Dmc {}
impl Dmc {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "desc STSR."]
    #[inline(always)]
    pub const fn stsr(self) -> crate::common::Reg<regs::Stsr, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0usize) as _) }
    }
    #[doc = "desc STCR."]
    #[inline(always)]
    pub const fn stcr(self) -> crate::common::Reg<regs::Stcr, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x04usize) as _) }
    }
    #[doc = "desc CMDR."]
    #[inline(always)]
    pub const fn cmdr(self) -> crate::common::Reg<regs::Cmdr, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x08usize) as _) }
    }
    #[doc = "desc CPCR."]
    #[inline(always)]
    pub const fn cpcr(self) -> crate::common::Reg<regs::Cpcr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0cusize) as _) }
    }
    #[doc = "desc RFTR."]
    #[inline(always)]
    pub const fn rftr(self) -> crate::common::Reg<regs::Rftr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x10usize) as _) }
    }
    #[doc = "desc TMCR_T_CASL."]
    #[inline(always)]
    pub const fn tmcr_t_casl(self) -> crate::common::Reg<regs::TmcrTCasl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x14usize) as _) }
    }
    #[doc = "desc TMCR_T_DQSS."]
    #[inline(always)]
    pub const fn tmcr_t_dqss(self) -> crate::common::Reg<regs::TmcrTDqss, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x18usize) as _) }
    }
    #[doc = "desc TMCR_T_MRD."]
    #[inline(always)]
    pub const fn tmcr_t_mrd(self) -> crate::common::Reg<regs::TmcrTMrd, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1cusize) as _) }
    }
    #[doc = "desc TMCR_T_RAS."]
    #[inline(always)]
    pub const fn tmcr_t_ras(self) -> crate::common::Reg<regs::TmcrTRas, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x20usize) as _) }
    }
    #[doc = "desc TMCR_T_RC."]
    #[inline(always)]
    pub const fn tmcr_t_rc(self) -> crate::common::Reg<regs::TmcrTRc, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x24usize) as _) }
    }
    #[doc = "desc TMCR_T_RCD."]
    #[inline(always)]
    pub const fn tmcr_t_rcd(self) -> crate::common::Reg<regs::TmcrTRcd, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x28usize) as _) }
    }
    #[doc = "desc TMCR_T_RFC."]
    #[inline(always)]
    pub const fn tmcr_t_rfc(self) -> crate::common::Reg<regs::TmcrTRfc, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x2cusize) as _) }
    }
    #[doc = "desc TMCR_T_RP."]
    #[inline(always)]
    pub const fn tmcr_t_rp(self) -> crate::common::Reg<regs::TmcrTRp, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x30usize) as _) }
    }
    #[doc = "desc TMCR_T_RRD."]
    #[inline(always)]
    pub const fn tmcr_t_rrd(self) -> crate::common::Reg<regs::TmcrTRrd, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x34usize) as _) }
    }
    #[doc = "desc TMCR_T_WR."]
    #[inline(always)]
    pub const fn tmcr_t_wr(self) -> crate::common::Reg<regs::TmcrTWr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x38usize) as _) }
    }
    #[doc = "desc TMCR_T_WTR."]
    #[inline(always)]
    pub const fn tmcr_t_wtr(self) -> crate::common::Reg<regs::TmcrTWtr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x3cusize) as _) }
    }
    #[doc = "desc TMCR_T_XP."]
    #[inline(always)]
    pub const fn tmcr_t_xp(self) -> crate::common::Reg<regs::TmcrTXp, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x40usize) as _) }
    }
    #[doc = "desc TMCR_T_XSR."]
    #[inline(always)]
    pub const fn tmcr_t_xsr(self) -> crate::common::Reg<regs::TmcrTXsr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x44usize) as _) }
    }
    #[doc = "desc TMCR_T_ESR."]
    #[inline(always)]
    pub const fn tmcr_t_esr(self) -> crate::common::Reg<regs::TmcrTEsr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x48usize) as _) }
    }
    #[doc = "desc CSCR0."]
    #[inline(always)]
    pub const fn cscr0(self) -> crate::common::Reg<regs::Cscr0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0200usize) as _) }
    }
    #[doc = "desc CSCR1."]
    #[inline(always)]
    pub const fn cscr1(self) -> crate::common::Reg<regs::Cscr1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0204usize) as _) }
    }
    #[doc = "desc CSCR2."]
    #[inline(always)]
    pub const fn cscr2(self) -> crate::common::Reg<regs::Cscr2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0208usize) as _) }
    }
    #[doc = "desc CSCR3."]
    #[inline(always)]
    pub const fn cscr3(self) -> crate::common::Reg<regs::Cscr3, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x020cusize) as _) }
    }
    #[doc = "desc BACR."]
    #[inline(always)]
    pub const fn bacr(self) -> crate::common::Reg<regs::Bacr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0300usize) as _) }
    }
}
pub mod regs {
    #[doc = "desc BACR."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Bacr(pub u32);
    impl Bacr {
        #[doc = "desc DMCMW."]
        #[must_use]
        #[inline(always)]
        pub const fn dmcmw(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x03;
            val as u8
        }
        #[doc = "desc DMCMW."]
        #[inline(always)]
        pub const fn set_dmcmw(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 0usize)) | (((val as u32) & 0x03) << 0usize);
        }
        #[doc = "desc CKSEL."]
        #[must_use]
        #[inline(always)]
        pub const fn cksel(&self) -> u8 {
            let val = (self.0 >> 14usize) & 0x03;
            val as u8
        }
        #[doc = "desc CKSEL."]
        #[inline(always)]
        pub const fn set_cksel(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 14usize)) | (((val as u32) & 0x03) << 14usize);
        }
    }
    impl Default for Bacr {
        #[inline(always)]
        fn default() -> Bacr {
            Bacr(0)
        }
    }
    impl core::fmt::Debug for Bacr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Bacr")
                .field("dmcmw", &self.dmcmw())
                .field("cksel", &self.cksel())
                .finish()
        }
    }
    #[doc = "desc CMDR."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Cmdr(pub u32);
    impl Cmdr {
        #[doc = "desc CMDADD."]
        #[must_use]
        #[inline(always)]
        pub const fn cmdadd(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x3fff;
            val as u16
        }
        #[doc = "desc CMDADD."]
        #[inline(always)]
        pub const fn set_cmdadd(&mut self, val: u16) {
            self.0 = (self.0 & !(0x3fff << 0usize)) | (((val as u32) & 0x3fff) << 0usize);
        }
        #[doc = "desc CMDBA."]
        #[must_use]
        #[inline(always)]
        pub const fn cmdba(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x03;
            val as u8
        }
        #[doc = "desc CMDBA."]
        #[inline(always)]
        pub const fn set_cmdba(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 16usize)) | (((val as u32) & 0x03) << 16usize);
        }
        #[doc = "desc CMD."]
        #[must_use]
        #[inline(always)]
        pub const fn cmd(&self) -> u8 {
            let val = (self.0 >> 18usize) & 0x03;
            val as u8
        }
        #[doc = "desc CMD."]
        #[inline(always)]
        pub const fn set_cmd(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 18usize)) | (((val as u32) & 0x03) << 18usize);
        }
        #[doc = "desc CMDCHIP."]
        #[must_use]
        #[inline(always)]
        pub const fn cmdchip(&self) -> u8 {
            let val = (self.0 >> 20usize) & 0x03;
            val as u8
        }
        #[doc = "desc CMDCHIP."]
        #[inline(always)]
        pub const fn set_cmdchip(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 20usize)) | (((val as u32) & 0x03) << 20usize);
        }
    }
    impl Default for Cmdr {
        #[inline(always)]
        fn default() -> Cmdr {
            Cmdr(0)
        }
    }
    impl core::fmt::Debug for Cmdr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Cmdr")
                .field("cmdadd", &self.cmdadd())
                .field("cmdba", &self.cmdba())
                .field("cmd", &self.cmd())
                .field("cmdchip", &self.cmdchip())
                .finish()
        }
    }
    #[doc = "desc CPCR."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Cpcr(pub u32);
    impl Cpcr {
        #[doc = "desc COLBS."]
        #[must_use]
        #[inline(always)]
        pub const fn colbs(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x07;
            val as u8
        }
        #[doc = "desc COLBS."]
        #[inline(always)]
        pub const fn set_colbs(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 0usize)) | (((val as u32) & 0x07) << 0usize);
        }
        #[doc = "desc ROWBS."]
        #[must_use]
        #[inline(always)]
        pub const fn rowbs(&self) -> u8 {
            let val = (self.0 >> 4usize) & 0x07;
            val as u8
        }
        #[doc = "desc ROWBS."]
        #[inline(always)]
        pub const fn set_rowbs(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 4usize)) | (((val as u32) & 0x07) << 4usize);
        }
        #[doc = "desc APBS."]
        #[must_use]
        #[inline(always)]
        pub const fn apbs(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "desc APBS."]
        #[inline(always)]
        pub const fn set_apbs(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "desc CKEDIS."]
        #[must_use]
        #[inline(always)]
        pub const fn ckedis(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "desc CKEDIS."]
        #[inline(always)]
        pub const fn set_ckedis(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "desc CKSTOP."]
        #[must_use]
        #[inline(always)]
        pub const fn ckstop(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "desc CKSTOP."]
        #[inline(always)]
        pub const fn set_ckstop(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "desc CKEDISPRD."]
        #[must_use]
        #[inline(always)]
        pub const fn ckedisprd(&self) -> u8 {
            let val = (self.0 >> 10usize) & 0x3f;
            val as u8
        }
        #[doc = "desc CKEDISPRD."]
        #[inline(always)]
        pub const fn set_ckedisprd(&mut self, val: u8) {
            self.0 = (self.0 & !(0x3f << 10usize)) | (((val as u32) & 0x3f) << 10usize);
        }
        #[doc = "desc BURST."]
        #[must_use]
        #[inline(always)]
        pub const fn burst(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x07;
            val as u8
        }
        #[doc = "desc BURST."]
        #[inline(always)]
        pub const fn set_burst(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 16usize)) | (((val as u32) & 0x07) << 16usize);
        }
        #[doc = "desc ACTCP."]
        #[must_use]
        #[inline(always)]
        pub const fn actcp(&self) -> u8 {
            let val = (self.0 >> 24usize) & 0x03;
            val as u8
        }
        #[doc = "desc ACTCP."]
        #[inline(always)]
        pub const fn set_actcp(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 24usize)) | (((val as u32) & 0x03) << 24usize);
        }
    }
    impl Default for Cpcr {
        #[inline(always)]
        fn default() -> Cpcr {
            Cpcr(0)
        }
    }
    impl core::fmt::Debug for Cpcr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Cpcr")
                .field("colbs", &self.colbs())
                .field("rowbs", &self.rowbs())
                .field("apbs", &self.apbs())
                .field("ckedis", &self.ckedis())
                .field("ckstop", &self.ckstop())
                .field("ckedisprd", &self.ckedisprd())
                .field("burst", &self.burst())
                .field("actcp", &self.actcp())
                .finish()
        }
    }
    #[doc = "desc CSCR0."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Cscr0(pub u32);
    impl Cscr0 {
        #[doc = "desc ADDMSK."]
        #[must_use]
        #[inline(always)]
        pub const fn addmsk(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[doc = "desc ADDMSK."]
        #[inline(always)]
        pub const fn set_addmsk(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
        }
        #[doc = "desc ADDMAT."]
        #[must_use]
        #[inline(always)]
        pub const fn addmat(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0xff;
            val as u8
        }
        #[doc = "desc ADDMAT."]
        #[inline(always)]
        pub const fn set_addmat(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
        }
        #[doc = "desc BRC."]
        #[must_use]
        #[inline(always)]
        pub const fn brc(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "desc BRC."]
        #[inline(always)]
        pub const fn set_brc(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
    }
    impl Default for Cscr0 {
        #[inline(always)]
        fn default() -> Cscr0 {
            Cscr0(0)
        }
    }
    impl core::fmt::Debug for Cscr0 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Cscr0")
                .field("addmsk", &self.addmsk())
                .field("addmat", &self.addmat())
                .field("brc", &self.brc())
                .finish()
        }
    }
    #[doc = "desc CSCR1."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Cscr1(pub u32);
    impl Cscr1 {
        #[doc = "desc ADDMSK."]
        #[must_use]
        #[inline(always)]
        pub const fn addmsk(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[doc = "desc ADDMSK."]
        #[inline(always)]
        pub const fn set_addmsk(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
        }
        #[doc = "desc ADDMAT."]
        #[must_use]
        #[inline(always)]
        pub const fn addmat(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0xff;
            val as u8
        }
        #[doc = "desc ADDMAT."]
        #[inline(always)]
        pub const fn set_addmat(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
        }
        #[doc = "desc BRC."]
        #[must_use]
        #[inline(always)]
        pub const fn brc(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "desc BRC."]
        #[inline(always)]
        pub const fn set_brc(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
    }
    impl Default for Cscr1 {
        #[inline(always)]
        fn default() -> Cscr1 {
            Cscr1(0)
        }
    }
    impl core::fmt::Debug for Cscr1 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Cscr1")
                .field("addmsk", &self.addmsk())
                .field("addmat", &self.addmat())
                .field("brc", &self.brc())
                .finish()
        }
    }
    #[doc = "desc CSCR2."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Cscr2(pub u32);
    impl Cscr2 {
        #[doc = "desc ADDMSK."]
        #[must_use]
        #[inline(always)]
        pub const fn addmsk(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[doc = "desc ADDMSK."]
        #[inline(always)]
        pub const fn set_addmsk(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
        }
        #[doc = "desc ADDMAT."]
        #[must_use]
        #[inline(always)]
        pub const fn addmat(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0xff;
            val as u8
        }
        #[doc = "desc ADDMAT."]
        #[inline(always)]
        pub const fn set_addmat(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
        }
        #[doc = "desc BRC."]
        #[must_use]
        #[inline(always)]
        pub const fn brc(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "desc BRC."]
        #[inline(always)]
        pub const fn set_brc(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
    }
    impl Default for Cscr2 {
        #[inline(always)]
        fn default() -> Cscr2 {
            Cscr2(0)
        }
    }
    impl core::fmt::Debug for Cscr2 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Cscr2")
                .field("addmsk", &self.addmsk())
                .field("addmat", &self.addmat())
                .field("brc", &self.brc())
                .finish()
        }
    }
    #[doc = "desc CSCR3."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Cscr3(pub u32);
    impl Cscr3 {
        #[doc = "desc ADDMSK."]
        #[must_use]
        #[inline(always)]
        pub const fn addmsk(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[doc = "desc ADDMSK."]
        #[inline(always)]
        pub const fn set_addmsk(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
        }
        #[doc = "desc ADDMAT."]
        #[must_use]
        #[inline(always)]
        pub const fn addmat(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0xff;
            val as u8
        }
        #[doc = "desc ADDMAT."]
        #[inline(always)]
        pub const fn set_addmat(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
        }
        #[doc = "desc BRC."]
        #[must_use]
        #[inline(always)]
        pub const fn brc(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "desc BRC."]
        #[inline(always)]
        pub const fn set_brc(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
    }
    impl Default for Cscr3 {
        #[inline(always)]
        fn default() -> Cscr3 {
            Cscr3(0)
        }
    }
    impl core::fmt::Debug for Cscr3 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Cscr3")
                .field("addmsk", &self.addmsk())
                .field("addmat", &self.addmat())
                .field("brc", &self.brc())
                .finish()
        }
    }
    #[doc = "desc RFTR."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Rftr(pub u32);
    impl Rftr {
        #[doc = "desc REFPRD."]
        #[must_use]
        #[inline(always)]
        pub const fn refprd(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x7fff;
            val as u16
        }
        #[doc = "desc REFPRD."]
        #[inline(always)]
        pub const fn set_refprd(&mut self, val: u16) {
            self.0 = (self.0 & !(0x7fff << 0usize)) | (((val as u32) & 0x7fff) << 0usize);
        }
    }
    impl Default for Rftr {
        #[inline(always)]
        fn default() -> Rftr {
            Rftr(0)
        }
    }
    impl core::fmt::Debug for Rftr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Rftr")
                .field("refprd", &self.refprd())
                .finish()
        }
    }
    #[doc = "desc STCR."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Stcr(pub u32);
    impl Stcr {
        #[doc = "desc STCTL."]
        #[must_use]
        #[inline(always)]
        pub const fn stctl(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x07;
            val as u8
        }
        #[doc = "desc STCTL."]
        #[inline(always)]
        pub const fn set_stctl(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 0usize)) | (((val as u32) & 0x07) << 0usize);
        }
    }
    impl Default for Stcr {
        #[inline(always)]
        fn default() -> Stcr {
            Stcr(0)
        }
    }
    impl core::fmt::Debug for Stcr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Stcr")
                .field("stctl", &self.stctl())
                .finish()
        }
    }
    #[doc = "desc STSR."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Stsr(pub u32);
    impl Stsr {
        #[doc = "desc STATUS."]
        #[must_use]
        #[inline(always)]
        pub const fn status(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x03;
            val as u8
        }
        #[doc = "desc STATUS."]
        #[inline(always)]
        pub const fn set_status(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 0usize)) | (((val as u32) & 0x03) << 0usize);
        }
        #[doc = "desc MEMW."]
        #[must_use]
        #[inline(always)]
        pub const fn memw(&self) -> u8 {
            let val = (self.0 >> 2usize) & 0x03;
            val as u8
        }
        #[doc = "desc MEMW."]
        #[inline(always)]
        pub const fn set_memw(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 2usize)) | (((val as u32) & 0x03) << 2usize);
        }
    }
    impl Default for Stsr {
        #[inline(always)]
        fn default() -> Stsr {
            Stsr(0)
        }
    }
    impl core::fmt::Debug for Stsr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Stsr")
                .field("status", &self.status())
                .field("memw", &self.memw())
                .finish()
        }
    }
    #[doc = "desc TMCR_T_CASL."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct TmcrTCasl(pub u32);
    impl TmcrTCasl {
        #[doc = "desc T_CASL."]
        #[must_use]
        #[inline(always)]
        pub const fn t_casl(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x07;
            val as u8
        }
        #[doc = "desc T_CASL."]
        #[inline(always)]
        pub const fn set_t_casl(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 0usize)) | (((val as u32) & 0x07) << 0usize);
        }
    }
    impl Default for TmcrTCasl {
        #[inline(always)]
        fn default() -> TmcrTCasl {
            TmcrTCasl(0)
        }
    }
    impl core::fmt::Debug for TmcrTCasl {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("TmcrTCasl")
                .field("t_casl", &self.t_casl())
                .finish()
        }
    }
    #[doc = "desc TMCR_T_DQSS."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct TmcrTDqss(pub u32);
    impl TmcrTDqss {
        #[doc = "desc T_DQSS."]
        #[must_use]
        #[inline(always)]
        pub const fn t_dqss(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x03;
            val as u8
        }
        #[doc = "desc T_DQSS."]
        #[inline(always)]
        pub const fn set_t_dqss(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 0usize)) | (((val as u32) & 0x03) << 0usize);
        }
    }
    impl Default for TmcrTDqss {
        #[inline(always)]
        fn default() -> TmcrTDqss {
            TmcrTDqss(0)
        }
    }
    impl core::fmt::Debug for TmcrTDqss {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("TmcrTDqss")
                .field("t_dqss", &self.t_dqss())
                .finish()
        }
    }
    #[doc = "desc TMCR_T_ESR."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct TmcrTEsr(pub u32);
    impl TmcrTEsr {
        #[doc = "desc T_ESR."]
        #[must_use]
        #[inline(always)]
        pub const fn t_esr(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[doc = "desc T_ESR."]
        #[inline(always)]
        pub const fn set_t_esr(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
        }
    }
    impl Default for TmcrTEsr {
        #[inline(always)]
        fn default() -> TmcrTEsr {
            TmcrTEsr(0)
        }
    }
    impl core::fmt::Debug for TmcrTEsr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("TmcrTEsr")
                .field("t_esr", &self.t_esr())
                .finish()
        }
    }
    #[doc = "desc TMCR_T_MRD."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct TmcrTMrd(pub u32);
    impl TmcrTMrd {
        #[doc = "desc T_MRD."]
        #[must_use]
        #[inline(always)]
        pub const fn t_mrd(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x7f;
            val as u8
        }
        #[doc = "desc T_MRD."]
        #[inline(always)]
        pub const fn set_t_mrd(&mut self, val: u8) {
            self.0 = (self.0 & !(0x7f << 0usize)) | (((val as u32) & 0x7f) << 0usize);
        }
    }
    impl Default for TmcrTMrd {
        #[inline(always)]
        fn default() -> TmcrTMrd {
            TmcrTMrd(0)
        }
    }
    impl core::fmt::Debug for TmcrTMrd {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("TmcrTMrd")
                .field("t_mrd", &self.t_mrd())
                .finish()
        }
    }
    #[doc = "desc TMCR_T_RAS."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct TmcrTRas(pub u32);
    impl TmcrTRas {
        #[doc = "desc T_RAS."]
        #[must_use]
        #[inline(always)]
        pub const fn t_ras(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x0f;
            val as u8
        }
        #[doc = "desc T_RAS."]
        #[inline(always)]
        pub const fn set_t_ras(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
        }
    }
    impl Default for TmcrTRas {
        #[inline(always)]
        fn default() -> TmcrTRas {
            TmcrTRas(0)
        }
    }
    impl core::fmt::Debug for TmcrTRas {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("TmcrTRas")
                .field("t_ras", &self.t_ras())
                .finish()
        }
    }
    #[doc = "desc TMCR_T_RC."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct TmcrTRc(pub u32);
    impl TmcrTRc {
        #[doc = "desc T_RC."]
        #[must_use]
        #[inline(always)]
        pub const fn t_rc(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x0f;
            val as u8
        }
        #[doc = "desc T_RC."]
        #[inline(always)]
        pub const fn set_t_rc(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
        }
    }
    impl Default for TmcrTRc {
        #[inline(always)]
        fn default() -> TmcrTRc {
            TmcrTRc(0)
        }
    }
    impl core::fmt::Debug for TmcrTRc {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("TmcrTRc")
                .field("t_rc", &self.t_rc())
                .finish()
        }
    }
    #[doc = "desc TMCR_T_RCD."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct TmcrTRcd(pub u32);
    impl TmcrTRcd {
        #[doc = "desc T_RCD_B."]
        #[must_use]
        #[inline(always)]
        pub const fn t_rcd_b(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x07;
            val as u8
        }
        #[doc = "desc T_RCD_B."]
        #[inline(always)]
        pub const fn set_t_rcd_b(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 0usize)) | (((val as u32) & 0x07) << 0usize);
        }
        #[doc = "desc T_RCD_P."]
        #[must_use]
        #[inline(always)]
        pub const fn t_rcd_p(&self) -> u8 {
            let val = (self.0 >> 4usize) & 0x07;
            val as u8
        }
        #[doc = "desc T_RCD_P."]
        #[inline(always)]
        pub const fn set_t_rcd_p(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 4usize)) | (((val as u32) & 0x07) << 4usize);
        }
    }
    impl Default for TmcrTRcd {
        #[inline(always)]
        fn default() -> TmcrTRcd {
            TmcrTRcd(0)
        }
    }
    impl core::fmt::Debug for TmcrTRcd {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("TmcrTRcd")
                .field("t_rcd_b", &self.t_rcd_b())
                .field("t_rcd_p", &self.t_rcd_p())
                .finish()
        }
    }
    #[doc = "desc TMCR_T_RFC."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct TmcrTRfc(pub u32);
    impl TmcrTRfc {
        #[doc = "desc T_RFC_B."]
        #[must_use]
        #[inline(always)]
        pub const fn t_rfc_b(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x1f;
            val as u8
        }
        #[doc = "desc T_RFC_B."]
        #[inline(always)]
        pub const fn set_t_rfc_b(&mut self, val: u8) {
            self.0 = (self.0 & !(0x1f << 0usize)) | (((val as u32) & 0x1f) << 0usize);
        }
        #[doc = "desc T_RFC_P."]
        #[must_use]
        #[inline(always)]
        pub const fn t_rfc_p(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0x1f;
            val as u8
        }
        #[doc = "desc T_RFC_P."]
        #[inline(always)]
        pub const fn set_t_rfc_p(&mut self, val: u8) {
            self.0 = (self.0 & !(0x1f << 8usize)) | (((val as u32) & 0x1f) << 8usize);
        }
    }
    impl Default for TmcrTRfc {
        #[inline(always)]
        fn default() -> TmcrTRfc {
            TmcrTRfc(0)
        }
    }
    impl core::fmt::Debug for TmcrTRfc {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("TmcrTRfc")
                .field("t_rfc_b", &self.t_rfc_b())
                .field("t_rfc_p", &self.t_rfc_p())
                .finish()
        }
    }
    #[doc = "desc TMCR_T_RP."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct TmcrTRp(pub u32);
    impl TmcrTRp {
        #[doc = "desc T_RP_B."]
        #[must_use]
        #[inline(always)]
        pub const fn t_rp_b(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x07;
            val as u8
        }
        #[doc = "desc T_RP_B."]
        #[inline(always)]
        pub const fn set_t_rp_b(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 0usize)) | (((val as u32) & 0x07) << 0usize);
        }
        #[doc = "desc T_RP_P."]
        #[must_use]
        #[inline(always)]
        pub const fn t_rp_p(&self) -> u8 {
            let val = (self.0 >> 4usize) & 0x07;
            val as u8
        }
        #[doc = "desc T_RP_P."]
        #[inline(always)]
        pub const fn set_t_rp_p(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 4usize)) | (((val as u32) & 0x07) << 4usize);
        }
    }
    impl Default for TmcrTRp {
        #[inline(always)]
        fn default() -> TmcrTRp {
            TmcrTRp(0)
        }
    }
    impl core::fmt::Debug for TmcrTRp {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("TmcrTRp")
                .field("t_rp_b", &self.t_rp_b())
                .field("t_rp_p", &self.t_rp_p())
                .finish()
        }
    }
    #[doc = "desc TMCR_T_RRD."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct TmcrTRrd(pub u32);
    impl TmcrTRrd {
        #[doc = "desc T_RRD."]
        #[must_use]
        #[inline(always)]
        pub const fn t_rrd(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x0f;
            val as u8
        }
        #[doc = "desc T_RRD."]
        #[inline(always)]
        pub const fn set_t_rrd(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
        }
    }
    impl Default for TmcrTRrd {
        #[inline(always)]
        fn default() -> TmcrTRrd {
            TmcrTRrd(0)
        }
    }
    impl core::fmt::Debug for TmcrTRrd {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("TmcrTRrd")
                .field("t_rrd", &self.t_rrd())
                .finish()
        }
    }
    #[doc = "desc TMCR_T_WR."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct TmcrTWr(pub u32);
    impl TmcrTWr {
        #[doc = "desc T_WR."]
        #[must_use]
        #[inline(always)]
        pub const fn t_wr(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x07;
            val as u8
        }
        #[doc = "desc T_WR."]
        #[inline(always)]
        pub const fn set_t_wr(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 0usize)) | (((val as u32) & 0x07) << 0usize);
        }
    }
    impl Default for TmcrTWr {
        #[inline(always)]
        fn default() -> TmcrTWr {
            TmcrTWr(0)
        }
    }
    impl core::fmt::Debug for TmcrTWr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("TmcrTWr")
                .field("t_wr", &self.t_wr())
                .finish()
        }
    }
    #[doc = "desc TMCR_T_WTR."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct TmcrTWtr(pub u32);
    impl TmcrTWtr {
        #[doc = "desc T_WTR."]
        #[must_use]
        #[inline(always)]
        pub const fn t_wtr(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x07;
            val as u8
        }
        #[doc = "desc T_WTR."]
        #[inline(always)]
        pub const fn set_t_wtr(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 0usize)) | (((val as u32) & 0x07) << 0usize);
        }
    }
    impl Default for TmcrTWtr {
        #[inline(always)]
        fn default() -> TmcrTWtr {
            TmcrTWtr(0)
        }
    }
    impl core::fmt::Debug for TmcrTWtr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("TmcrTWtr")
                .field("t_wtr", &self.t_wtr())
                .finish()
        }
    }
    #[doc = "desc TMCR_T_XP."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct TmcrTXp(pub u32);
    impl TmcrTXp {
        #[doc = "desc T_XP."]
        #[must_use]
        #[inline(always)]
        pub const fn t_xp(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[doc = "desc T_XP."]
        #[inline(always)]
        pub const fn set_t_xp(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
        }
    }
    impl Default for TmcrTXp {
        #[inline(always)]
        fn default() -> TmcrTXp {
            TmcrTXp(0)
        }
    }
    impl core::fmt::Debug for TmcrTXp {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("TmcrTXp")
                .field("t_xp", &self.t_xp())
                .finish()
        }
    }
    #[doc = "desc TMCR_T_XSR."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct TmcrTXsr(pub u32);
    impl TmcrTXsr {
        #[doc = "desc T_XSR."]
        #[must_use]
        #[inline(always)]
        pub const fn t_xsr(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[doc = "desc T_XSR."]
        #[inline(always)]
        pub const fn set_t_xsr(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
        }
    }
    impl Default for TmcrTXsr {
        #[inline(always)]
        fn default() -> TmcrTXsr {
            TmcrTXsr(0)
        }
    }
    impl core::fmt::Debug for TmcrTXsr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("TmcrTXsr")
                .field("t_xsr", &self.t_xsr())
                .finish()
        }
    }
}
