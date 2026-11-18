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
    #[doc = "desc CR."]
    #[inline(always)]
    pub const fn cr(self) -> crate::common::Reg<regs::Cr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x04usize) as _) }
    }
    #[doc = "desc SR."]
    #[inline(always)]
    pub const fn sr(self) -> crate::common::Reg<regs::Sr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x08usize) as _) }
    }
    #[doc = "desc CFGR."]
    #[inline(always)]
    pub const fn cfgr(self) -> crate::common::Reg<regs::Cfgr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1cusize) as _) }
    }
    #[doc = "desc PR."]
    #[inline(always)]
    pub const fn pr(self) -> crate::common::Reg<regs::Pr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x20usize) as _) }
    }
    #[doc = "desc ICR."]
    #[inline(always)]
    pub const fn icr(self) -> crate::common::Reg<regs::Icr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x24usize) as _) }
    }
    #[doc = "desc DRL."]
    #[inline(always)]
    pub const fn drl(self) -> crate::common::Reg<regs::Drl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x28usize) as _) }
    }
    #[doc = "desc DRR."]
    #[inline(always)]
    pub const fn drr(self) -> crate::common::Reg<regs::Drr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x2cusize) as _) }
    }
}
pub mod regs {
    #[doc = "desc CFGR."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Cfgr(pub u32);
    impl Cfgr {
        #[doc = "desc CHIEN."]
        #[must_use]
        #[inline(always)]
        pub const fn chien(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "desc CHIEN."]
        #[inline(always)]
        pub const fn set_chien(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "desc DATLEN."]
        #[must_use]
        #[inline(always)]
        pub const fn datlen(&self) -> u8 {
            let val = (self.0 >> 1usize) & 0x03;
            val as u8
        }
        #[doc = "desc DATLEN."]
        #[inline(always)]
        pub const fn set_datlen(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 1usize)) | (((val as u32) & 0x03) << 1usize);
        }
        #[doc = "desc CKPOL."]
        #[must_use]
        #[inline(always)]
        pub const fn ckpol(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "desc CKPOL."]
        #[inline(always)]
        pub const fn set_ckpol(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "desc STD."]
        #[must_use]
        #[inline(always)]
        pub const fn std(&self) -> u8 {
            let val = (self.0 >> 4usize) & 0x03;
            val as u8
        }
        #[doc = "desc STD."]
        #[inline(always)]
        pub const fn set_std(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 4usize)) | (((val as u32) & 0x03) << 4usize);
        }
        #[doc = "desc CKSEL."]
        #[must_use]
        #[inline(always)]
        pub const fn cksel(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "desc CKSEL."]
        #[inline(always)]
        pub const fn set_cksel(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "desc PCMSYNC."]
        #[must_use]
        #[inline(always)]
        pub const fn pcmsync(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "desc PCMSYNC."]
        #[inline(always)]
        pub const fn set_pcmsync(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "desc CFG."]
        #[must_use]
        #[inline(always)]
        pub const fn cfg(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0x03;
            val as u8
        }
        #[doc = "desc CFG."]
        #[inline(always)]
        pub const fn set_cfg(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 8usize)) | (((val as u32) & 0x03) << 8usize);
        }
        #[doc = "desc E."]
        #[must_use]
        #[inline(always)]
        pub const fn e(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "desc E."]
        #[inline(always)]
        pub const fn set_e(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
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
                .field("chien", &self.chien())
                .field("datlen", &self.datlen())
                .field("ckpol", &self.ckpol())
                .field("std", &self.std())
                .field("cksel", &self.cksel())
                .field("pcmsync", &self.pcmsync())
                .field("cfg", &self.cfg())
                .field("e", &self.e())
                .finish()
        }
    }
    #[doc = "desc CR."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Cr(pub u32);
    impl Cr {
        #[doc = "desc LDMAEN."]
        #[must_use]
        #[inline(always)]
        pub const fn ldmaen(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "desc LDMAEN."]
        #[inline(always)]
        pub const fn set_ldmaen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "desc RDMAEN."]
        #[must_use]
        #[inline(always)]
        pub const fn rdmaen(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "desc RDMAEN."]
        #[inline(always)]
        pub const fn set_rdmaen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "desc ERRIE."]
        #[must_use]
        #[inline(always)]
        pub const fn errie(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "desc ERRIE."]
        #[inline(always)]
        pub const fn set_errie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "desc RXNEIE."]
        #[must_use]
        #[inline(always)]
        pub const fn rxneie(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "desc RXNEIE."]
        #[inline(always)]
        pub const fn set_rxneie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "desc TXEIE."]
        #[must_use]
        #[inline(always)]
        pub const fn txeie(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "desc TXEIE."]
        #[inline(always)]
        pub const fn set_txeie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
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
                .field("ldmaen", &self.ldmaen())
                .field("rdmaen", &self.rdmaen())
                .field("errie", &self.errie())
                .field("rxneie", &self.rxneie())
                .field("txeie", &self.txeie())
                .finish()
        }
    }
    #[doc = "desc DRL."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Drl(pub u32);
    impl Drl {
        #[doc = "desc DRL."]
        #[must_use]
        #[inline(always)]
        pub const fn drl(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[doc = "desc DRL."]
        #[inline(always)]
        pub const fn set_drl(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
        }
    }
    impl Default for Drl {
        #[inline(always)]
        fn default() -> Drl {
            Drl(0)
        }
    }
    impl core::fmt::Debug for Drl {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Drl").field("drl", &self.drl()).finish()
        }
    }
    #[doc = "desc DRR."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Drr(pub u32);
    impl Drr {
        #[doc = "desc DRR."]
        #[must_use]
        #[inline(always)]
        pub const fn drr(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[doc = "desc DRR."]
        #[inline(always)]
        pub const fn set_drr(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
        }
    }
    impl Default for Drr {
        #[inline(always)]
        fn default() -> Drr {
            Drr(0)
        }
    }
    impl core::fmt::Debug for Drr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Drr").field("drr", &self.drr()).finish()
        }
    }
    #[doc = "desc ICR."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Icr(pub u32);
    impl Icr {
        #[doc = "desc UDF."]
        #[must_use]
        #[inline(always)]
        pub const fn udf(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "desc UDF."]
        #[inline(always)]
        pub const fn set_udf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "desc OVR."]
        #[must_use]
        #[inline(always)]
        pub const fn ovr(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "desc OVR."]
        #[inline(always)]
        pub const fn set_ovr(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "desc FRE."]
        #[must_use]
        #[inline(always)]
        pub const fn fre(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "desc FRE."]
        #[inline(always)]
        pub const fn set_fre(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
    }
    impl Default for Icr {
        #[inline(always)]
        fn default() -> Icr {
            Icr(0)
        }
    }
    impl core::fmt::Debug for Icr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Icr")
                .field("udf", &self.udf())
                .field("ovr", &self.ovr())
                .field("fre", &self.fre())
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
        #[doc = "desc ODD."]
        #[must_use]
        #[inline(always)]
        pub const fn odd(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "desc ODD."]
        #[inline(always)]
        pub const fn set_odd(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "desc MCKOE."]
        #[must_use]
        #[inline(always)]
        pub const fn mckoe(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "desc MCKOE."]
        #[inline(always)]
        pub const fn set_mckoe(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "desc FRACT."]
        #[must_use]
        #[inline(always)]
        pub const fn fract(&self) -> u8 {
            let val = (self.0 >> 10usize) & 0x3f;
            val as u8
        }
        #[doc = "desc FRACT."]
        #[inline(always)]
        pub const fn set_fract(&mut self, val: u8) {
            self.0 = (self.0 & !(0x3f << 10usize)) | (((val as u32) & 0x3f) << 10usize);
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
                .field("odd", &self.odd())
                .field("mckoe", &self.mckoe())
                .field("fract", &self.fract())
                .finish()
        }
    }
    #[doc = "desc SR."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Sr(pub u32);
    impl Sr {
        #[doc = "desc RXNE_L."]
        #[must_use]
        #[inline(always)]
        pub const fn rxne_l(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "desc RXNE_L."]
        #[inline(always)]
        pub const fn set_rxne_l(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "desc TXE_L."]
        #[must_use]
        #[inline(always)]
        pub const fn txe_l(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "desc TXE_L."]
        #[inline(always)]
        pub const fn set_txe_l(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "desc UDR_L."]
        #[must_use]
        #[inline(always)]
        pub const fn udr_l(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "desc UDR_L."]
        #[inline(always)]
        pub const fn set_udr_l(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "desc UDR_R."]
        #[must_use]
        #[inline(always)]
        pub const fn udr_r(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "desc UDR_R."]
        #[inline(always)]
        pub const fn set_udr_r(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "desc OVR_L."]
        #[must_use]
        #[inline(always)]
        pub const fn ovr_l(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "desc OVR_L."]
        #[inline(always)]
        pub const fn set_ovr_l(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "desc BSY."]
        #[must_use]
        #[inline(always)]
        pub const fn bsy(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "desc BSY."]
        #[inline(always)]
        pub const fn set_bsy(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "desc FRE."]
        #[must_use]
        #[inline(always)]
        pub const fn fre(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "desc FRE."]
        #[inline(always)]
        pub const fn set_fre(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "desc OVR_R."]
        #[must_use]
        #[inline(always)]
        pub const fn ovr_r(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[doc = "desc OVR_R."]
        #[inline(always)]
        pub const fn set_ovr_r(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[doc = "desc RXNE_R."]
        #[must_use]
        #[inline(always)]
        pub const fn rxne_r(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "desc RXNE_R."]
        #[inline(always)]
        pub const fn set_rxne_r(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[doc = "desc TXE_R."]
        #[must_use]
        #[inline(always)]
        pub const fn txe_r(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "desc TXE_R."]
        #[inline(always)]
        pub const fn set_txe_r(&mut self, val: bool) {
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
                .field("rxne_l", &self.rxne_l())
                .field("txe_l", &self.txe_l())
                .field("udr_l", &self.udr_l())
                .field("udr_r", &self.udr_r())
                .field("ovr_l", &self.ovr_l())
                .field("bsy", &self.bsy())
                .field("fre", &self.fre())
                .field("ovr_r", &self.ovr_r())
                .field("rxne_r", &self.rxne_r())
                .field("txe_r", &self.txe_r())
                .finish()
        }
    }
}
