#![allow(clippy::missing_safety_doc)]
#![allow(clippy::identity_op)]
#![allow(clippy::unnecessary_cast)]
#![allow(clippy::erasing_op)]

#[doc = "desc CAN."]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Can {
    ptr: *mut u8,
}
unsafe impl Send for Can {}
unsafe impl Sync for Can {}
impl Can {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "desc RBUF0."]
    #[inline(always)]
    pub const fn rbuf0(self) -> crate::common::Reg<u32, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0usize) as _) }
    }
    #[doc = "desc RBUF1."]
    #[inline(always)]
    pub const fn rbuf1(self) -> crate::common::Reg<u32, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x04usize) as _) }
    }
    #[doc = "desc RBUF2."]
    #[inline(always)]
    pub const fn rbuf2(self) -> crate::common::Reg<u32, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x08usize) as _) }
    }
    #[doc = "desc RBUF3."]
    #[inline(always)]
    pub const fn rbuf3(self) -> crate::common::Reg<u32, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0cusize) as _) }
    }
    #[doc = "desc TBUF0."]
    #[inline(always)]
    pub const fn tbuf0(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x50usize) as _) }
    }
    #[doc = "desc TBUF1."]
    #[inline(always)]
    pub const fn tbuf1(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x54usize) as _) }
    }
    #[doc = "desc TBUF2."]
    #[inline(always)]
    pub const fn tbuf2(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x58usize) as _) }
    }
    #[doc = "desc TBUF3."]
    #[inline(always)]
    pub const fn tbuf3(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x5cusize) as _) }
    }
    #[doc = "desc CFG_STAT."]
    #[inline(always)]
    pub const fn cfg_stat(self) -> crate::common::Reg<regs::CfgStat, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xa0usize) as _) }
    }
    #[doc = "desc TCMD."]
    #[inline(always)]
    pub const fn tcmd(self) -> crate::common::Reg<regs::Tcmd, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xa1usize) as _) }
    }
    #[doc = "desc TCTRL."]
    #[inline(always)]
    pub const fn tctrl(self) -> crate::common::Reg<regs::Tctrl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xa2usize) as _) }
    }
    #[doc = "desc RCTRL."]
    #[inline(always)]
    pub const fn rctrl(self) -> crate::common::Reg<regs::Rctrl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xa3usize) as _) }
    }
    #[doc = "desc RTIE."]
    #[inline(always)]
    pub const fn rtie(self) -> crate::common::Reg<regs::Rtie, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xa4usize) as _) }
    }
    #[doc = "desc RTIF."]
    #[inline(always)]
    pub const fn rtif(self) -> crate::common::Reg<regs::Rtif, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xa5usize) as _) }
    }
    #[doc = "desc ERRINT."]
    #[inline(always)]
    pub const fn errint(self) -> crate::common::Reg<regs::Errint, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xa6usize) as _) }
    }
    #[doc = "desc LIMIT."]
    #[inline(always)]
    pub const fn limit(self) -> crate::common::Reg<regs::Limit, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xa7usize) as _) }
    }
    #[doc = "desc BT."]
    #[inline(always)]
    pub const fn bt(self) -> crate::common::Reg<regs::Bt, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xa8usize) as _) }
    }
    #[doc = "desc EALCAP."]
    #[inline(always)]
    pub const fn ealcap(self) -> crate::common::Reg<regs::Ealcap, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xb0usize) as _) }
    }
    #[doc = "desc RECNT."]
    #[inline(always)]
    pub const fn recnt(self) -> crate::common::Reg<u8, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xb2usize) as _) }
    }
    #[doc = "desc TECNT."]
    #[inline(always)]
    pub const fn tecnt(self) -> crate::common::Reg<u8, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xb3usize) as _) }
    }
    #[doc = "desc ACFCTRL."]
    #[inline(always)]
    pub const fn acfctrl(self) -> crate::common::Reg<regs::Acfctrl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xb4usize) as _) }
    }
    #[doc = "desc ACFEN."]
    #[inline(always)]
    pub const fn acfen(self) -> crate::common::Reg<regs::Acfen, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xb6usize) as _) }
    }
    #[doc = "desc ACF."]
    #[inline(always)]
    pub const fn acf(self) -> crate::common::Reg<regs::Acf, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xb8usize) as _) }
    }
    #[doc = "desc TBSLOT."]
    #[inline(always)]
    pub const fn tbslot(self) -> crate::common::Reg<regs::Tbslot, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xbeusize) as _) }
    }
    #[doc = "desc TTCFG."]
    #[inline(always)]
    pub const fn ttcfg(self) -> crate::common::Reg<regs::Ttcfg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xbfusize) as _) }
    }
    #[doc = "desc REF_MSG."]
    #[inline(always)]
    pub const fn ref_msg(self) -> crate::common::Reg<regs::RefMsg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xc0usize) as _) }
    }
    #[doc = "desc TRG_CFG."]
    #[inline(always)]
    pub const fn trg_cfg(self) -> crate::common::Reg<regs::TrgCfg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xc4usize) as _) }
    }
    #[doc = "desc TT_TRIG."]
    #[inline(always)]
    pub const fn tt_trig(self) -> crate::common::Reg<u16, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xc6usize) as _) }
    }
    #[doc = "desc TT_WTRIG."]
    #[inline(always)]
    pub const fn tt_wtrig(self) -> crate::common::Reg<u16, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xc8usize) as _) }
    }
}
pub mod regs {
    #[doc = "desc ACF."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Acf(pub u32);
    impl Acf {
        #[doc = "desc ACODEORAMASK."]
        #[must_use]
        #[inline(always)]
        pub const fn acodeoramask(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0x1fff_ffff;
            val as u32
        }
        #[doc = "desc ACODEORAMASK."]
        #[inline(always)]
        pub const fn set_acodeoramask(&mut self, val: u32) {
            self.0 = (self.0 & !(0x1fff_ffff << 0usize)) | (((val as u32) & 0x1fff_ffff) << 0usize);
        }
        #[doc = "desc AIDE."]
        #[must_use]
        #[inline(always)]
        pub const fn aide(&self) -> bool {
            let val = (self.0 >> 29usize) & 0x01;
            val != 0
        }
        #[doc = "desc AIDE."]
        #[inline(always)]
        pub const fn set_aide(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
        }
        #[doc = "desc AIDEE."]
        #[must_use]
        #[inline(always)]
        pub const fn aidee(&self) -> bool {
            let val = (self.0 >> 30usize) & 0x01;
            val != 0
        }
        #[doc = "desc AIDEE."]
        #[inline(always)]
        pub const fn set_aidee(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
        }
    }
    impl Default for Acf {
        #[inline(always)]
        fn default() -> Acf {
            Acf(0)
        }
    }
    impl core::fmt::Debug for Acf {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Acf")
                .field("acodeoramask", &self.acodeoramask())
                .field("aide", &self.aide())
                .field("aidee", &self.aidee())
                .finish()
        }
    }
    #[doc = "desc ACFCTRL."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Acfctrl(pub u8);
    impl Acfctrl {
        #[doc = "desc ACFADR."]
        #[must_use]
        #[inline(always)]
        pub const fn acfadr(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x0f;
            val as u8
        }
        #[doc = "desc ACFADR."]
        #[inline(always)]
        pub const fn set_acfadr(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u8) & 0x0f) << 0usize);
        }
        #[doc = "desc SELMASK."]
        #[must_use]
        #[inline(always)]
        pub const fn selmask(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "desc SELMASK."]
        #[inline(always)]
        pub const fn set_selmask(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u8) & 0x01) << 5usize);
        }
    }
    impl Default for Acfctrl {
        #[inline(always)]
        fn default() -> Acfctrl {
            Acfctrl(0)
        }
    }
    impl core::fmt::Debug for Acfctrl {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Acfctrl")
                .field("acfadr", &self.acfadr())
                .field("selmask", &self.selmask())
                .finish()
        }
    }
    #[doc = "desc ACFEN."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Acfen(pub u8);
    impl Acfen {
        #[doc = "desc AE_1."]
        #[must_use]
        #[inline(always)]
        pub const fn ae_1(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "desc AE_1."]
        #[inline(always)]
        pub const fn set_ae_1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u8) & 0x01) << 0usize);
        }
        #[doc = "desc AE_2."]
        #[must_use]
        #[inline(always)]
        pub const fn ae_2(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "desc AE_2."]
        #[inline(always)]
        pub const fn set_ae_2(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u8) & 0x01) << 1usize);
        }
        #[doc = "desc AE_3."]
        #[must_use]
        #[inline(always)]
        pub const fn ae_3(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "desc AE_3."]
        #[inline(always)]
        pub const fn set_ae_3(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u8) & 0x01) << 2usize);
        }
        #[doc = "desc AE_4."]
        #[must_use]
        #[inline(always)]
        pub const fn ae_4(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "desc AE_4."]
        #[inline(always)]
        pub const fn set_ae_4(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u8) & 0x01) << 3usize);
        }
        #[doc = "desc AE_5."]
        #[must_use]
        #[inline(always)]
        pub const fn ae_5(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "desc AE_5."]
        #[inline(always)]
        pub const fn set_ae_5(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u8) & 0x01) << 4usize);
        }
        #[doc = "desc AE_6."]
        #[must_use]
        #[inline(always)]
        pub const fn ae_6(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "desc AE_6."]
        #[inline(always)]
        pub const fn set_ae_6(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u8) & 0x01) << 5usize);
        }
        #[doc = "desc AE_7."]
        #[must_use]
        #[inline(always)]
        pub const fn ae_7(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "desc AE_7."]
        #[inline(always)]
        pub const fn set_ae_7(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u8) & 0x01) << 6usize);
        }
        #[doc = "desc AE_8."]
        #[must_use]
        #[inline(always)]
        pub const fn ae_8(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "desc AE_8."]
        #[inline(always)]
        pub const fn set_ae_8(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u8) & 0x01) << 7usize);
        }
    }
    impl Default for Acfen {
        #[inline(always)]
        fn default() -> Acfen {
            Acfen(0)
        }
    }
    impl core::fmt::Debug for Acfen {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Acfen")
                .field("ae_1", &self.ae_1())
                .field("ae_2", &self.ae_2())
                .field("ae_3", &self.ae_3())
                .field("ae_4", &self.ae_4())
                .field("ae_5", &self.ae_5())
                .field("ae_6", &self.ae_6())
                .field("ae_7", &self.ae_7())
                .field("ae_8", &self.ae_8())
                .finish()
        }
    }
    #[doc = "desc BT."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Bt(pub u32);
    impl Bt {
        #[doc = "desc SEG_1."]
        #[must_use]
        #[inline(always)]
        pub const fn seg_1(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[doc = "desc SEG_1."]
        #[inline(always)]
        pub const fn set_seg_1(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
        }
        #[doc = "desc SEG_2."]
        #[must_use]
        #[inline(always)]
        pub const fn seg_2(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0x7f;
            val as u8
        }
        #[doc = "desc SEG_2."]
        #[inline(always)]
        pub const fn set_seg_2(&mut self, val: u8) {
            self.0 = (self.0 & !(0x7f << 8usize)) | (((val as u32) & 0x7f) << 8usize);
        }
        #[doc = "desc SJW."]
        #[must_use]
        #[inline(always)]
        pub const fn sjw(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x7f;
            val as u8
        }
        #[doc = "desc SJW."]
        #[inline(always)]
        pub const fn set_sjw(&mut self, val: u8) {
            self.0 = (self.0 & !(0x7f << 16usize)) | (((val as u32) & 0x7f) << 16usize);
        }
        #[doc = "desc PRESC."]
        #[must_use]
        #[inline(always)]
        pub const fn presc(&self) -> u8 {
            let val = (self.0 >> 24usize) & 0xff;
            val as u8
        }
        #[doc = "desc PRESC."]
        #[inline(always)]
        pub const fn set_presc(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 24usize)) | (((val as u32) & 0xff) << 24usize);
        }
    }
    impl Default for Bt {
        #[inline(always)]
        fn default() -> Bt {
            Bt(0)
        }
    }
    impl core::fmt::Debug for Bt {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Bt")
                .field("seg_1", &self.seg_1())
                .field("seg_2", &self.seg_2())
                .field("sjw", &self.sjw())
                .field("presc", &self.presc())
                .finish()
        }
    }
    #[doc = "desc CFG_STAT."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct CfgStat(pub u8);
    impl CfgStat {
        #[doc = "desc BUSOFF."]
        #[must_use]
        #[inline(always)]
        pub const fn busoff(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "desc BUSOFF."]
        #[inline(always)]
        pub const fn set_busoff(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u8) & 0x01) << 0usize);
        }
        #[doc = "desc TACTIVE."]
        #[must_use]
        #[inline(always)]
        pub const fn tactive(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "desc TACTIVE."]
        #[inline(always)]
        pub const fn set_tactive(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u8) & 0x01) << 1usize);
        }
        #[doc = "desc RACTIVE."]
        #[must_use]
        #[inline(always)]
        pub const fn ractive(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "desc RACTIVE."]
        #[inline(always)]
        pub const fn set_ractive(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u8) & 0x01) << 2usize);
        }
        #[doc = "desc TSSS."]
        #[must_use]
        #[inline(always)]
        pub const fn tsss(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "desc TSSS."]
        #[inline(always)]
        pub const fn set_tsss(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u8) & 0x01) << 3usize);
        }
        #[doc = "desc TPSS."]
        #[must_use]
        #[inline(always)]
        pub const fn tpss(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "desc TPSS."]
        #[inline(always)]
        pub const fn set_tpss(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u8) & 0x01) << 4usize);
        }
        #[doc = "desc LBMI."]
        #[must_use]
        #[inline(always)]
        pub const fn lbmi(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "desc LBMI."]
        #[inline(always)]
        pub const fn set_lbmi(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u8) & 0x01) << 5usize);
        }
        #[doc = "desc LBME."]
        #[must_use]
        #[inline(always)]
        pub const fn lbme(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "desc LBME."]
        #[inline(always)]
        pub const fn set_lbme(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u8) & 0x01) << 6usize);
        }
        #[doc = "desc RESET."]
        #[must_use]
        #[inline(always)]
        pub const fn reset(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "desc RESET."]
        #[inline(always)]
        pub const fn set_reset(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u8) & 0x01) << 7usize);
        }
    }
    impl Default for CfgStat {
        #[inline(always)]
        fn default() -> CfgStat {
            CfgStat(0)
        }
    }
    impl core::fmt::Debug for CfgStat {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("CfgStat")
                .field("busoff", &self.busoff())
                .field("tactive", &self.tactive())
                .field("ractive", &self.ractive())
                .field("tsss", &self.tsss())
                .field("tpss", &self.tpss())
                .field("lbmi", &self.lbmi())
                .field("lbme", &self.lbme())
                .field("reset", &self.reset())
                .finish()
        }
    }
    #[doc = "desc EALCAP."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ealcap(pub u8);
    impl Ealcap {
        #[doc = "desc ALC."]
        #[must_use]
        #[inline(always)]
        pub const fn alc(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x1f;
            val as u8
        }
        #[doc = "desc ALC."]
        #[inline(always)]
        pub const fn set_alc(&mut self, val: u8) {
            self.0 = (self.0 & !(0x1f << 0usize)) | (((val as u8) & 0x1f) << 0usize);
        }
        #[doc = "desc KOER."]
        #[must_use]
        #[inline(always)]
        pub const fn koer(&self) -> u8 {
            let val = (self.0 >> 5usize) & 0x07;
            val as u8
        }
        #[doc = "desc KOER."]
        #[inline(always)]
        pub const fn set_koer(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 5usize)) | (((val as u8) & 0x07) << 5usize);
        }
    }
    impl Default for Ealcap {
        #[inline(always)]
        fn default() -> Ealcap {
            Ealcap(0)
        }
    }
    impl core::fmt::Debug for Ealcap {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Ealcap")
                .field("alc", &self.alc())
                .field("koer", &self.koer())
                .finish()
        }
    }
    #[doc = "desc ERRINT."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Errint(pub u8);
    impl Errint {
        #[doc = "desc BEIF."]
        #[must_use]
        #[inline(always)]
        pub const fn beif(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "desc BEIF."]
        #[inline(always)]
        pub const fn set_beif(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u8) & 0x01) << 0usize);
        }
        #[doc = "desc BEIE."]
        #[must_use]
        #[inline(always)]
        pub const fn beie(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "desc BEIE."]
        #[inline(always)]
        pub const fn set_beie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u8) & 0x01) << 1usize);
        }
        #[doc = "desc ALIF."]
        #[must_use]
        #[inline(always)]
        pub const fn alif(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "desc ALIF."]
        #[inline(always)]
        pub const fn set_alif(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u8) & 0x01) << 2usize);
        }
        #[doc = "desc ALIE."]
        #[must_use]
        #[inline(always)]
        pub const fn alie(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "desc ALIE."]
        #[inline(always)]
        pub const fn set_alie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u8) & 0x01) << 3usize);
        }
        #[doc = "desc EPIF."]
        #[must_use]
        #[inline(always)]
        pub const fn epif(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "desc EPIF."]
        #[inline(always)]
        pub const fn set_epif(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u8) & 0x01) << 4usize);
        }
        #[doc = "desc EPIE."]
        #[must_use]
        #[inline(always)]
        pub const fn epie(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "desc EPIE."]
        #[inline(always)]
        pub const fn set_epie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u8) & 0x01) << 5usize);
        }
        #[doc = "desc EPASS."]
        #[must_use]
        #[inline(always)]
        pub const fn epass(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "desc EPASS."]
        #[inline(always)]
        pub const fn set_epass(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u8) & 0x01) << 6usize);
        }
        #[doc = "desc EWARN."]
        #[must_use]
        #[inline(always)]
        pub const fn ewarn(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "desc EWARN."]
        #[inline(always)]
        pub const fn set_ewarn(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u8) & 0x01) << 7usize);
        }
    }
    impl Default for Errint {
        #[inline(always)]
        fn default() -> Errint {
            Errint(0)
        }
    }
    impl core::fmt::Debug for Errint {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Errint")
                .field("beif", &self.beif())
                .field("beie", &self.beie())
                .field("alif", &self.alif())
                .field("alie", &self.alie())
                .field("epif", &self.epif())
                .field("epie", &self.epie())
                .field("epass", &self.epass())
                .field("ewarn", &self.ewarn())
                .finish()
        }
    }
    #[doc = "desc LIMIT."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Limit(pub u8);
    impl Limit {
        #[doc = "desc EWL."]
        #[must_use]
        #[inline(always)]
        pub const fn ewl(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x0f;
            val as u8
        }
        #[doc = "desc EWL."]
        #[inline(always)]
        pub const fn set_ewl(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u8) & 0x0f) << 0usize);
        }
        #[doc = "desc AFWL."]
        #[must_use]
        #[inline(always)]
        pub const fn afwl(&self) -> u8 {
            let val = (self.0 >> 4usize) & 0x0f;
            val as u8
        }
        #[doc = "desc AFWL."]
        #[inline(always)]
        pub const fn set_afwl(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 4usize)) | (((val as u8) & 0x0f) << 4usize);
        }
    }
    impl Default for Limit {
        #[inline(always)]
        fn default() -> Limit {
            Limit(0)
        }
    }
    impl core::fmt::Debug for Limit {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Limit")
                .field("ewl", &self.ewl())
                .field("afwl", &self.afwl())
                .finish()
        }
    }
    #[doc = "desc RCTRL."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Rctrl(pub u8);
    impl Rctrl {
        #[doc = "desc RSSTAT."]
        #[must_use]
        #[inline(always)]
        pub const fn rsstat(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x03;
            val as u8
        }
        #[doc = "desc RSSTAT."]
        #[inline(always)]
        pub const fn set_rsstat(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 0usize)) | (((val as u8) & 0x03) << 0usize);
        }
        #[doc = "desc RBALL."]
        #[must_use]
        #[inline(always)]
        pub const fn rball(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "desc RBALL."]
        #[inline(always)]
        pub const fn set_rball(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u8) & 0x01) << 3usize);
        }
        #[doc = "desc RREL."]
        #[must_use]
        #[inline(always)]
        pub const fn rrel(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "desc RREL."]
        #[inline(always)]
        pub const fn set_rrel(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u8) & 0x01) << 4usize);
        }
        #[doc = "desc ROV."]
        #[must_use]
        #[inline(always)]
        pub const fn rov(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "desc ROV."]
        #[inline(always)]
        pub const fn set_rov(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u8) & 0x01) << 5usize);
        }
        #[doc = "desc ROM."]
        #[must_use]
        #[inline(always)]
        pub const fn rom(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "desc ROM."]
        #[inline(always)]
        pub const fn set_rom(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u8) & 0x01) << 6usize);
        }
        #[doc = "desc SACK."]
        #[must_use]
        #[inline(always)]
        pub const fn sack(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "desc SACK."]
        #[inline(always)]
        pub const fn set_sack(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u8) & 0x01) << 7usize);
        }
    }
    impl Default for Rctrl {
        #[inline(always)]
        fn default() -> Rctrl {
            Rctrl(0)
        }
    }
    impl core::fmt::Debug for Rctrl {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Rctrl")
                .field("rsstat", &self.rsstat())
                .field("rball", &self.rball())
                .field("rrel", &self.rrel())
                .field("rov", &self.rov())
                .field("rom", &self.rom())
                .field("sack", &self.sack())
                .finish()
        }
    }
    #[doc = "desc REF_MSG."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct RefMsg(pub u32);
    impl RefMsg {
        #[doc = "desc REF_ID."]
        #[must_use]
        #[inline(always)]
        pub const fn ref_id(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0x1fff_ffff;
            val as u32
        }
        #[doc = "desc REF_ID."]
        #[inline(always)]
        pub const fn set_ref_id(&mut self, val: u32) {
            self.0 = (self.0 & !(0x1fff_ffff << 0usize)) | (((val as u32) & 0x1fff_ffff) << 0usize);
        }
        #[doc = "desc REF_IDE."]
        #[must_use]
        #[inline(always)]
        pub const fn ref_ide(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[doc = "desc REF_IDE."]
        #[inline(always)]
        pub const fn set_ref_ide(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for RefMsg {
        #[inline(always)]
        fn default() -> RefMsg {
            RefMsg(0)
        }
    }
    impl core::fmt::Debug for RefMsg {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("RefMsg")
                .field("ref_id", &self.ref_id())
                .field("ref_ide", &self.ref_ide())
                .finish()
        }
    }
    #[doc = "desc RTIE."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Rtie(pub u8);
    impl Rtie {
        #[doc = "desc TSFF."]
        #[must_use]
        #[inline(always)]
        pub const fn tsff(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "desc TSFF."]
        #[inline(always)]
        pub const fn set_tsff(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u8) & 0x01) << 0usize);
        }
        #[doc = "desc EIE."]
        #[must_use]
        #[inline(always)]
        pub const fn eie(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "desc EIE."]
        #[inline(always)]
        pub const fn set_eie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u8) & 0x01) << 1usize);
        }
        #[doc = "desc TSIE."]
        #[must_use]
        #[inline(always)]
        pub const fn tsie(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "desc TSIE."]
        #[inline(always)]
        pub const fn set_tsie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u8) & 0x01) << 2usize);
        }
        #[doc = "desc TPIE."]
        #[must_use]
        #[inline(always)]
        pub const fn tpie(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "desc TPIE."]
        #[inline(always)]
        pub const fn set_tpie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u8) & 0x01) << 3usize);
        }
        #[doc = "desc RAFIE."]
        #[must_use]
        #[inline(always)]
        pub const fn rafie(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "desc RAFIE."]
        #[inline(always)]
        pub const fn set_rafie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u8) & 0x01) << 4usize);
        }
        #[doc = "desc RFIE."]
        #[must_use]
        #[inline(always)]
        pub const fn rfie(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "desc RFIE."]
        #[inline(always)]
        pub const fn set_rfie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u8) & 0x01) << 5usize);
        }
        #[doc = "desc ROIE."]
        #[must_use]
        #[inline(always)]
        pub const fn roie(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "desc ROIE."]
        #[inline(always)]
        pub const fn set_roie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u8) & 0x01) << 6usize);
        }
        #[doc = "desc RIE."]
        #[must_use]
        #[inline(always)]
        pub const fn rie(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "desc RIE."]
        #[inline(always)]
        pub const fn set_rie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u8) & 0x01) << 7usize);
        }
    }
    impl Default for Rtie {
        #[inline(always)]
        fn default() -> Rtie {
            Rtie(0)
        }
    }
    impl core::fmt::Debug for Rtie {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Rtie")
                .field("tsff", &self.tsff())
                .field("eie", &self.eie())
                .field("tsie", &self.tsie())
                .field("tpie", &self.tpie())
                .field("rafie", &self.rafie())
                .field("rfie", &self.rfie())
                .field("roie", &self.roie())
                .field("rie", &self.rie())
                .finish()
        }
    }
    #[doc = "desc RTIF."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Rtif(pub u8);
    impl Rtif {
        #[doc = "desc AIF."]
        #[must_use]
        #[inline(always)]
        pub const fn aif(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "desc AIF."]
        #[inline(always)]
        pub const fn set_aif(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u8) & 0x01) << 0usize);
        }
        #[doc = "desc EIF."]
        #[must_use]
        #[inline(always)]
        pub const fn eif(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "desc EIF."]
        #[inline(always)]
        pub const fn set_eif(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u8) & 0x01) << 1usize);
        }
        #[doc = "desc TSIF."]
        #[must_use]
        #[inline(always)]
        pub const fn tsif(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "desc TSIF."]
        #[inline(always)]
        pub const fn set_tsif(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u8) & 0x01) << 2usize);
        }
        #[doc = "desc TPIF."]
        #[must_use]
        #[inline(always)]
        pub const fn tpif(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "desc TPIF."]
        #[inline(always)]
        pub const fn set_tpif(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u8) & 0x01) << 3usize);
        }
        #[doc = "desc RAFIF."]
        #[must_use]
        #[inline(always)]
        pub const fn rafif(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "desc RAFIF."]
        #[inline(always)]
        pub const fn set_rafif(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u8) & 0x01) << 4usize);
        }
        #[doc = "desc RFIF."]
        #[must_use]
        #[inline(always)]
        pub const fn rfif(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "desc RFIF."]
        #[inline(always)]
        pub const fn set_rfif(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u8) & 0x01) << 5usize);
        }
        #[doc = "desc ROIF."]
        #[must_use]
        #[inline(always)]
        pub const fn roif(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "desc ROIF."]
        #[inline(always)]
        pub const fn set_roif(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u8) & 0x01) << 6usize);
        }
        #[doc = "desc RIF."]
        #[must_use]
        #[inline(always)]
        pub const fn rif(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "desc RIF."]
        #[inline(always)]
        pub const fn set_rif(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u8) & 0x01) << 7usize);
        }
    }
    impl Default for Rtif {
        #[inline(always)]
        fn default() -> Rtif {
            Rtif(0)
        }
    }
    impl core::fmt::Debug for Rtif {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Rtif")
                .field("aif", &self.aif())
                .field("eif", &self.eif())
                .field("tsif", &self.tsif())
                .field("tpif", &self.tpif())
                .field("rafif", &self.rafif())
                .field("rfif", &self.rfif())
                .field("roif", &self.roif())
                .field("rif", &self.rif())
                .finish()
        }
    }
    #[doc = "desc TBSLOT."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Tbslot(pub u8);
    impl Tbslot {
        #[doc = "desc TBPTR."]
        #[must_use]
        #[inline(always)]
        pub const fn tbptr(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x3f;
            val as u8
        }
        #[doc = "desc TBPTR."]
        #[inline(always)]
        pub const fn set_tbptr(&mut self, val: u8) {
            self.0 = (self.0 & !(0x3f << 0usize)) | (((val as u8) & 0x3f) << 0usize);
        }
        #[doc = "desc TBF."]
        #[must_use]
        #[inline(always)]
        pub const fn tbf(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "desc TBF."]
        #[inline(always)]
        pub const fn set_tbf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u8) & 0x01) << 6usize);
        }
        #[doc = "desc TBE."]
        #[must_use]
        #[inline(always)]
        pub const fn tbe(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "desc TBE."]
        #[inline(always)]
        pub const fn set_tbe(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u8) & 0x01) << 7usize);
        }
    }
    impl Default for Tbslot {
        #[inline(always)]
        fn default() -> Tbslot {
            Tbslot(0)
        }
    }
    impl core::fmt::Debug for Tbslot {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Tbslot")
                .field("tbptr", &self.tbptr())
                .field("tbf", &self.tbf())
                .field("tbe", &self.tbe())
                .finish()
        }
    }
    #[doc = "desc TCMD."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Tcmd(pub u8);
    impl Tcmd {
        #[doc = "desc TSA."]
        #[must_use]
        #[inline(always)]
        pub const fn tsa(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "desc TSA."]
        #[inline(always)]
        pub const fn set_tsa(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u8) & 0x01) << 0usize);
        }
        #[doc = "desc TSALL."]
        #[must_use]
        #[inline(always)]
        pub const fn tsall(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "desc TSALL."]
        #[inline(always)]
        pub const fn set_tsall(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u8) & 0x01) << 1usize);
        }
        #[doc = "desc TSONE."]
        #[must_use]
        #[inline(always)]
        pub const fn tsone(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "desc TSONE."]
        #[inline(always)]
        pub const fn set_tsone(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u8) & 0x01) << 2usize);
        }
        #[doc = "desc TPA."]
        #[must_use]
        #[inline(always)]
        pub const fn tpa(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "desc TPA."]
        #[inline(always)]
        pub const fn set_tpa(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u8) & 0x01) << 3usize);
        }
        #[doc = "desc TPE."]
        #[must_use]
        #[inline(always)]
        pub const fn tpe(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "desc TPE."]
        #[inline(always)]
        pub const fn set_tpe(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u8) & 0x01) << 4usize);
        }
        #[doc = "desc STBY."]
        #[must_use]
        #[inline(always)]
        pub const fn stby(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "desc STBY."]
        #[inline(always)]
        pub const fn set_stby(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u8) & 0x01) << 5usize);
        }
        #[doc = "desc LOM."]
        #[must_use]
        #[inline(always)]
        pub const fn lom(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "desc LOM."]
        #[inline(always)]
        pub const fn set_lom(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u8) & 0x01) << 6usize);
        }
        #[doc = "desc TBSEL."]
        #[must_use]
        #[inline(always)]
        pub const fn tbsel(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "desc TBSEL."]
        #[inline(always)]
        pub const fn set_tbsel(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u8) & 0x01) << 7usize);
        }
    }
    impl Default for Tcmd {
        #[inline(always)]
        fn default() -> Tcmd {
            Tcmd(0)
        }
    }
    impl core::fmt::Debug for Tcmd {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Tcmd")
                .field("tsa", &self.tsa())
                .field("tsall", &self.tsall())
                .field("tsone", &self.tsone())
                .field("tpa", &self.tpa())
                .field("tpe", &self.tpe())
                .field("stby", &self.stby())
                .field("lom", &self.lom())
                .field("tbsel", &self.tbsel())
                .finish()
        }
    }
    #[doc = "desc TCTRL."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Tctrl(pub u8);
    impl Tctrl {
        #[doc = "desc TSSTAT."]
        #[must_use]
        #[inline(always)]
        pub const fn tsstat(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x03;
            val as u8
        }
        #[doc = "desc TSSTAT."]
        #[inline(always)]
        pub const fn set_tsstat(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 0usize)) | (((val as u8) & 0x03) << 0usize);
        }
        #[doc = "desc TTBM."]
        #[must_use]
        #[inline(always)]
        pub const fn ttbm(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "desc TTBM."]
        #[inline(always)]
        pub const fn set_ttbm(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u8) & 0x01) << 4usize);
        }
        #[doc = "desc TSMODE."]
        #[must_use]
        #[inline(always)]
        pub const fn tsmode(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "desc TSMODE."]
        #[inline(always)]
        pub const fn set_tsmode(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u8) & 0x01) << 5usize);
        }
        #[doc = "desc TSNEXT."]
        #[must_use]
        #[inline(always)]
        pub const fn tsnext(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "desc TSNEXT."]
        #[inline(always)]
        pub const fn set_tsnext(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u8) & 0x01) << 6usize);
        }
    }
    impl Default for Tctrl {
        #[inline(always)]
        fn default() -> Tctrl {
            Tctrl(0)
        }
    }
    impl core::fmt::Debug for Tctrl {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Tctrl")
                .field("tsstat", &self.tsstat())
                .field("ttbm", &self.ttbm())
                .field("tsmode", &self.tsmode())
                .field("tsnext", &self.tsnext())
                .finish()
        }
    }
    #[doc = "desc TRG_CFG."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct TrgCfg(pub u16);
    impl TrgCfg {
        #[doc = "desc TTPTR."]
        #[must_use]
        #[inline(always)]
        pub const fn ttptr(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x3f;
            val as u8
        }
        #[doc = "desc TTPTR."]
        #[inline(always)]
        pub const fn set_ttptr(&mut self, val: u8) {
            self.0 = (self.0 & !(0x3f << 0usize)) | (((val as u16) & 0x3f) << 0usize);
        }
        #[doc = "desc TTYPE."]
        #[must_use]
        #[inline(always)]
        pub const fn ttype(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0x07;
            val as u8
        }
        #[doc = "desc TTYPE."]
        #[inline(always)]
        pub const fn set_ttype(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 8usize)) | (((val as u16) & 0x07) << 8usize);
        }
        #[doc = "desc TEW."]
        #[must_use]
        #[inline(always)]
        pub const fn tew(&self) -> u8 {
            let val = (self.0 >> 12usize) & 0x0f;
            val as u8
        }
        #[doc = "desc TEW."]
        #[inline(always)]
        pub const fn set_tew(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 12usize)) | (((val as u16) & 0x0f) << 12usize);
        }
    }
    impl Default for TrgCfg {
        #[inline(always)]
        fn default() -> TrgCfg {
            TrgCfg(0)
        }
    }
    impl core::fmt::Debug for TrgCfg {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("TrgCfg")
                .field("ttptr", &self.ttptr())
                .field("ttype", &self.ttype())
                .field("tew", &self.tew())
                .finish()
        }
    }
    #[doc = "desc TTCFG."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ttcfg(pub u8);
    impl Ttcfg {
        #[doc = "desc TTEN."]
        #[must_use]
        #[inline(always)]
        pub const fn tten(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "desc TTEN."]
        #[inline(always)]
        pub const fn set_tten(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u8) & 0x01) << 0usize);
        }
        #[doc = "desc T_PRESC."]
        #[must_use]
        #[inline(always)]
        pub const fn t_presc(&self) -> u8 {
            let val = (self.0 >> 1usize) & 0x03;
            val as u8
        }
        #[doc = "desc T_PRESC."]
        #[inline(always)]
        pub const fn set_t_presc(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 1usize)) | (((val as u8) & 0x03) << 1usize);
        }
        #[doc = "desc TTIF."]
        #[must_use]
        #[inline(always)]
        pub const fn ttif(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "desc TTIF."]
        #[inline(always)]
        pub const fn set_ttif(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u8) & 0x01) << 3usize);
        }
        #[doc = "desc TTIE."]
        #[must_use]
        #[inline(always)]
        pub const fn ttie(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "desc TTIE."]
        #[inline(always)]
        pub const fn set_ttie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u8) & 0x01) << 4usize);
        }
        #[doc = "desc TEIF."]
        #[must_use]
        #[inline(always)]
        pub const fn teif(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "desc TEIF."]
        #[inline(always)]
        pub const fn set_teif(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u8) & 0x01) << 5usize);
        }
        #[doc = "desc WTIF."]
        #[must_use]
        #[inline(always)]
        pub const fn wtif(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "desc WTIF."]
        #[inline(always)]
        pub const fn set_wtif(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u8) & 0x01) << 6usize);
        }
        #[doc = "desc WTIE."]
        #[must_use]
        #[inline(always)]
        pub const fn wtie(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "desc WTIE."]
        #[inline(always)]
        pub const fn set_wtie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u8) & 0x01) << 7usize);
        }
    }
    impl Default for Ttcfg {
        #[inline(always)]
        fn default() -> Ttcfg {
            Ttcfg(0)
        }
    }
    impl core::fmt::Debug for Ttcfg {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Ttcfg")
                .field("tten", &self.tten())
                .field("t_presc", &self.t_presc())
                .field("ttif", &self.ttif())
                .field("ttie", &self.ttie())
                .field("teif", &self.teif())
                .field("wtif", &self.wtif())
                .field("wtie", &self.wtie())
                .finish()
        }
    }
}
