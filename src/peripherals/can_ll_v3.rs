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
    #[doc = "desc RBUF."]
    #[inline(always)]
    pub const fn rbuf(self) -> crate::common::Reg<u32, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0usize) as _) }
    }
    #[doc = "desc TBUF."]
    #[inline(always)]
    pub const fn tbuf(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x50usize) as _) }
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
    #[doc = "desc SBT."]
    #[inline(always)]
    pub const fn sbt(self) -> crate::common::Reg<regs::Sbt, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xa8usize) as _) }
    }
    #[doc = "desc FBT."]
    #[inline(always)]
    pub const fn fbt(self) -> crate::common::Reg<regs::Fbt, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xacusize) as _) }
    }
    #[doc = "desc EALCAP."]
    #[inline(always)]
    pub const fn ealcap(self) -> crate::common::Reg<regs::Ealcap, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xb0usize) as _) }
    }
    #[doc = "desc TDC."]
    #[inline(always)]
    pub const fn tdc(self) -> crate::common::Reg<regs::Tdc, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xb1usize) as _) }
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
    #[doc = "desc RAM_CKCR."]
    #[inline(always)]
    pub const fn ram_ckcr(self) -> crate::common::Reg<regs::RamCkcr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0100usize) as _) }
    }
    #[doc = "desc RAM_CKSR."]
    #[inline(always)]
    pub const fn ram_cksr(self) -> crate::common::Reg<regs::RamCksr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0104usize) as _) }
    }
    #[doc = "desc RAM_EIEN."]
    #[inline(always)]
    pub const fn ram_eien(self) -> crate::common::Reg<regs::RamEien, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0108usize) as _) }
    }
    #[doc = "desc RAM_EIBIT0."]
    #[inline(always)]
    pub const fn ram_eibit0(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x010cusize) as _) }
    }
    #[doc = "desc RAM_EIBIT1."]
    #[inline(always)]
    pub const fn ram_eibit1(self) -> crate::common::Reg<regs::RamEibit1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0110usize) as _) }
    }
    #[doc = "desc RAM_ECCERRADDR."]
    #[inline(always)]
    pub const fn ram_eccerraddr(self) -> crate::common::Reg<regs::RamEccerraddr, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0114usize) as _) }
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
    pub struct Acfen(pub u16);
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
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u16) & 0x01) << 0usize);
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
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u16) & 0x01) << 1usize);
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
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u16) & 0x01) << 2usize);
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
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u16) & 0x01) << 3usize);
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
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u16) & 0x01) << 4usize);
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
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u16) & 0x01) << 5usize);
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
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u16) & 0x01) << 6usize);
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
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u16) & 0x01) << 7usize);
        }
        #[doc = "desc AE_9."]
        #[must_use]
        #[inline(always)]
        pub const fn ae_9(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "desc AE_9."]
        #[inline(always)]
        pub const fn set_ae_9(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u16) & 0x01) << 8usize);
        }
        #[doc = "desc AE_10."]
        #[must_use]
        #[inline(always)]
        pub const fn ae_10(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "desc AE_10."]
        #[inline(always)]
        pub const fn set_ae_10(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u16) & 0x01) << 9usize);
        }
        #[doc = "desc AE_11."]
        #[must_use]
        #[inline(always)]
        pub const fn ae_11(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "desc AE_11."]
        #[inline(always)]
        pub const fn set_ae_11(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u16) & 0x01) << 10usize);
        }
        #[doc = "desc AE_12."]
        #[must_use]
        #[inline(always)]
        pub const fn ae_12(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "desc AE_12."]
        #[inline(always)]
        pub const fn set_ae_12(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u16) & 0x01) << 11usize);
        }
        #[doc = "desc AE_13."]
        #[must_use]
        #[inline(always)]
        pub const fn ae_13(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "desc AE_13."]
        #[inline(always)]
        pub const fn set_ae_13(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u16) & 0x01) << 12usize);
        }
        #[doc = "desc AE_14."]
        #[must_use]
        #[inline(always)]
        pub const fn ae_14(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[doc = "desc AE_14."]
        #[inline(always)]
        pub const fn set_ae_14(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u16) & 0x01) << 13usize);
        }
        #[doc = "desc AE_15."]
        #[must_use]
        #[inline(always)]
        pub const fn ae_15(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "desc AE_15."]
        #[inline(always)]
        pub const fn set_ae_15(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u16) & 0x01) << 14usize);
        }
        #[doc = "desc AE_16."]
        #[must_use]
        #[inline(always)]
        pub const fn ae_16(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "desc AE_16."]
        #[inline(always)]
        pub const fn set_ae_16(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u16) & 0x01) << 15usize);
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
                .field("ae_9", &self.ae_9())
                .field("ae_10", &self.ae_10())
                .field("ae_11", &self.ae_11())
                .field("ae_12", &self.ae_12())
                .field("ae_13", &self.ae_13())
                .field("ae_14", &self.ae_14())
                .field("ae_15", &self.ae_15())
                .field("ae_16", &self.ae_16())
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
    #[doc = "desc FBT."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Fbt(pub u32);
    impl Fbt {
        #[doc = "desc F_SEG_1."]
        #[must_use]
        #[inline(always)]
        pub const fn f_seg_1(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x1f;
            val as u8
        }
        #[doc = "desc F_SEG_1."]
        #[inline(always)]
        pub const fn set_f_seg_1(&mut self, val: u8) {
            self.0 = (self.0 & !(0x1f << 0usize)) | (((val as u32) & 0x1f) << 0usize);
        }
        #[doc = "desc F_SEG_2."]
        #[must_use]
        #[inline(always)]
        pub const fn f_seg_2(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0x0f;
            val as u8
        }
        #[doc = "desc F_SEG_2."]
        #[inline(always)]
        pub const fn set_f_seg_2(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 8usize)) | (((val as u32) & 0x0f) << 8usize);
        }
        #[doc = "desc F_SJW."]
        #[must_use]
        #[inline(always)]
        pub const fn f_sjw(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x0f;
            val as u8
        }
        #[doc = "desc F_SJW."]
        #[inline(always)]
        pub const fn set_f_sjw(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 16usize)) | (((val as u32) & 0x0f) << 16usize);
        }
        #[doc = "desc F_PRESC."]
        #[must_use]
        #[inline(always)]
        pub const fn f_presc(&self) -> u8 {
            let val = (self.0 >> 24usize) & 0xff;
            val as u8
        }
        #[doc = "desc F_PRESC."]
        #[inline(always)]
        pub const fn set_f_presc(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 24usize)) | (((val as u32) & 0xff) << 24usize);
        }
    }
    impl Default for Fbt {
        #[inline(always)]
        fn default() -> Fbt {
            Fbt(0)
        }
    }
    impl core::fmt::Debug for Fbt {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Fbt")
                .field("f_seg_1", &self.f_seg_1())
                .field("f_seg_2", &self.f_seg_2())
                .field("f_sjw", &self.f_sjw())
                .field("f_presc", &self.f_presc())
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
    #[doc = "desc RAM_CKCR."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct RamCkcr(pub u32);
    impl RamCkcr {
        #[doc = "desc ECCMOD."]
        #[must_use]
        #[inline(always)]
        pub const fn eccmod(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x03;
            val as u8
        }
        #[doc = "desc ECCMOD."]
        #[inline(always)]
        pub const fn set_eccmod(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 0usize)) | (((val as u32) & 0x03) << 0usize);
        }
    }
    impl Default for RamCkcr {
        #[inline(always)]
        fn default() -> RamCkcr {
            RamCkcr(0)
        }
    }
    impl core::fmt::Debug for RamCkcr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("RamCkcr")
                .field("eccmod", &self.eccmod())
                .finish()
        }
    }
    #[doc = "desc RAM_CKSR."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct RamCksr(pub u32);
    impl RamCksr {
        #[doc = "desc RAM_1ERR."]
        #[must_use]
        #[inline(always)]
        pub const fn ram_1err(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "desc RAM_1ERR."]
        #[inline(always)]
        pub const fn set_ram_1err(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "desc RAM_2ERR."]
        #[must_use]
        #[inline(always)]
        pub const fn ram_2err(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "desc RAM_2ERR."]
        #[inline(always)]
        pub const fn set_ram_2err(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
    }
    impl Default for RamCksr {
        #[inline(always)]
        fn default() -> RamCksr {
            RamCksr(0)
        }
    }
    impl core::fmt::Debug for RamCksr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("RamCksr")
                .field("ram_1err", &self.ram_1err())
                .field("ram_2err", &self.ram_2err())
                .finish()
        }
    }
    #[doc = "desc RAM_ECCERRADDR."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct RamEccerraddr(pub u32);
    impl RamEccerraddr {
        #[doc = "desc ECCERRADDR."]
        #[must_use]
        #[inline(always)]
        pub const fn eccerraddr(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x07ff;
            val as u16
        }
        #[doc = "desc ECCERRADDR."]
        #[inline(always)]
        pub const fn set_eccerraddr(&mut self, val: u16) {
            self.0 = (self.0 & !(0x07ff << 0usize)) | (((val as u32) & 0x07ff) << 0usize);
        }
    }
    impl Default for RamEccerraddr {
        #[inline(always)]
        fn default() -> RamEccerraddr {
            RamEccerraddr(0)
        }
    }
    impl core::fmt::Debug for RamEccerraddr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("RamEccerraddr")
                .field("eccerraddr", &self.eccerraddr())
                .finish()
        }
    }
    #[doc = "desc RAM_EIBIT1."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct RamEibit1(pub u32);
    impl RamEibit1 {
        #[doc = "desc EIBIT."]
        #[must_use]
        #[inline(always)]
        pub const fn eibit(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x7f;
            val as u8
        }
        #[doc = "desc EIBIT."]
        #[inline(always)]
        pub const fn set_eibit(&mut self, val: u8) {
            self.0 = (self.0 & !(0x7f << 0usize)) | (((val as u32) & 0x7f) << 0usize);
        }
    }
    impl Default for RamEibit1 {
        #[inline(always)]
        fn default() -> RamEibit1 {
            RamEibit1(0)
        }
    }
    impl core::fmt::Debug for RamEibit1 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("RamEibit1")
                .field("eibit", &self.eibit())
                .finish()
        }
    }
    #[doc = "desc RAM_EIEN."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct RamEien(pub u32);
    impl RamEien {
        #[doc = "desc EIEN."]
        #[must_use]
        #[inline(always)]
        pub const fn eien(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "desc EIEN."]
        #[inline(always)]
        pub const fn set_eien(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
    }
    impl Default for RamEien {
        #[inline(always)]
        fn default() -> RamEien {
            RamEien(0)
        }
    }
    impl core::fmt::Debug for RamEien {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("RamEien")
                .field("eien", &self.eien())
                .finish()
        }
    }
    #[doc = "desc RCTRL."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Rctrl(pub u8);
    impl Rctrl {
        #[doc = "desc RSTAT."]
        #[must_use]
        #[inline(always)]
        pub const fn rstat(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x03;
            val as u8
        }
        #[doc = "desc RSTAT."]
        #[inline(always)]
        pub const fn set_rstat(&mut self, val: u8) {
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
                .field("rstat", &self.rstat())
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
    #[doc = "desc SBT."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Sbt(pub u32);
    impl Sbt {
        #[doc = "desc S_SEG_1."]
        #[must_use]
        #[inline(always)]
        pub const fn s_seg_1(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[doc = "desc S_SEG_1."]
        #[inline(always)]
        pub const fn set_s_seg_1(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
        }
        #[doc = "desc S_SEG_2."]
        #[must_use]
        #[inline(always)]
        pub const fn s_seg_2(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0x7f;
            val as u8
        }
        #[doc = "desc S_SEG_2."]
        #[inline(always)]
        pub const fn set_s_seg_2(&mut self, val: u8) {
            self.0 = (self.0 & !(0x7f << 8usize)) | (((val as u32) & 0x7f) << 8usize);
        }
        #[doc = "desc S_SJW."]
        #[must_use]
        #[inline(always)]
        pub const fn s_sjw(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x7f;
            val as u8
        }
        #[doc = "desc S_SJW."]
        #[inline(always)]
        pub const fn set_s_sjw(&mut self, val: u8) {
            self.0 = (self.0 & !(0x7f << 16usize)) | (((val as u32) & 0x7f) << 16usize);
        }
        #[doc = "desc S_PRESC."]
        #[must_use]
        #[inline(always)]
        pub const fn s_presc(&self) -> u8 {
            let val = (self.0 >> 24usize) & 0xff;
            val as u8
        }
        #[doc = "desc S_PRESC."]
        #[inline(always)]
        pub const fn set_s_presc(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 24usize)) | (((val as u32) & 0xff) << 24usize);
        }
    }
    impl Default for Sbt {
        #[inline(always)]
        fn default() -> Sbt {
            Sbt(0)
        }
    }
    impl core::fmt::Debug for Sbt {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Sbt")
                .field("s_seg_1", &self.s_seg_1())
                .field("s_seg_2", &self.s_seg_2())
                .field("s_sjw", &self.s_sjw())
                .field("s_presc", &self.s_presc())
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
            let val = (self.0 >> 0usize) & 0x07;
            val as u8
        }
        #[doc = "desc TBPTR."]
        #[inline(always)]
        pub const fn set_tbptr(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 0usize)) | (((val as u8) & 0x07) << 0usize);
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
        #[doc = "desc TTTBM."]
        #[must_use]
        #[inline(always)]
        pub const fn tttbm(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "desc TTTBM."]
        #[inline(always)]
        pub const fn set_tttbm(&mut self, val: bool) {
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
        #[doc = "desc FD_ISO."]
        #[must_use]
        #[inline(always)]
        pub const fn fd_iso(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "desc FD_ISO."]
        #[inline(always)]
        pub const fn set_fd_iso(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u8) & 0x01) << 7usize);
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
                .field("tttbm", &self.tttbm())
                .field("tsmode", &self.tsmode())
                .field("tsnext", &self.tsnext())
                .field("fd_iso", &self.fd_iso())
                .finish()
        }
    }
    #[doc = "desc TDC."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Tdc(pub u8);
    impl Tdc {
        #[doc = "desc SSPOFF."]
        #[must_use]
        #[inline(always)]
        pub const fn sspoff(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x7f;
            val as u8
        }
        #[doc = "desc SSPOFF."]
        #[inline(always)]
        pub const fn set_sspoff(&mut self, val: u8) {
            self.0 = (self.0 & !(0x7f << 0usize)) | (((val as u8) & 0x7f) << 0usize);
        }
        #[doc = "desc TDCEN."]
        #[must_use]
        #[inline(always)]
        pub const fn tdcen(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "desc TDCEN."]
        #[inline(always)]
        pub const fn set_tdcen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u8) & 0x01) << 7usize);
        }
    }
    impl Default for Tdc {
        #[inline(always)]
        fn default() -> Tdc {
            Tdc(0)
        }
    }
    impl core::fmt::Debug for Tdc {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Tdc")
                .field("sspoff", &self.sspoff())
                .field("tdcen", &self.tdcen())
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
            let val = (self.0 >> 0usize) & 0x07;
            val as u8
        }
        #[doc = "desc TTPTR."]
        #[inline(always)]
        pub const fn set_ttptr(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 0usize)) | (((val as u16) & 0x07) << 0usize);
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
