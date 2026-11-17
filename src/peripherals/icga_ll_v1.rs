#![allow(clippy::missing_safety_doc)]
#![allow(clippy::identity_op)]
#![allow(clippy::unnecessary_cast)]
#![allow(clippy::erasing_op)]

#[doc = "desc ICGA."]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Icga {
    ptr: *mut u8,
}
unsafe impl Send for Icga {}
unsafe impl Sync for Icga {}
impl Icga {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "desc ICGA0A."]
    #[inline(always)]
    pub const fn icga0a(self) -> crate::common::Reg<regs::Icga0a, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0usize) as _) }
    }
    #[doc = "desc ICGA0B."]
    #[inline(always)]
    pub const fn icga0b(self) -> crate::common::Reg<regs::Icga0b, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x04usize) as _) }
    }
    #[doc = "desc ICGA0C."]
    #[inline(always)]
    pub const fn icga0c(self) -> crate::common::Reg<regs::Icga0c, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x08usize) as _) }
    }
    #[doc = "desc ICGA1A."]
    #[inline(always)]
    pub const fn icga1a(self) -> crate::common::Reg<regs::Icga1a, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x10usize) as _) }
    }
    #[doc = "desc ICGA1B."]
    #[inline(always)]
    pub const fn icga1b(self) -> crate::common::Reg<regs::Icga1b, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x14usize) as _) }
    }
    #[doc = "desc ICGA1C."]
    #[inline(always)]
    pub const fn icga1c(self) -> crate::common::Reg<regs::Icga1c, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x18usize) as _) }
    }
    #[doc = "desc ICGA2A."]
    #[inline(always)]
    pub const fn icga2a(self) -> crate::common::Reg<regs::Icga2a, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x20usize) as _) }
    }
    #[doc = "desc ICGA2B."]
    #[inline(always)]
    pub const fn icga2b(self) -> crate::common::Reg<regs::Icga2b, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x24usize) as _) }
    }
    #[doc = "desc ICGA2C."]
    #[inline(always)]
    pub const fn icga2c(self) -> crate::common::Reg<regs::Icga2c, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x28usize) as _) }
    }
    #[doc = "desc ICGA3A."]
    #[inline(always)]
    pub const fn icga3a(self) -> crate::common::Reg<regs::Icga3a, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x30usize) as _) }
    }
    #[doc = "desc ICGA3B."]
    #[inline(always)]
    pub const fn icga3b(self) -> crate::common::Reg<regs::Icga3b, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x34usize) as _) }
    }
    #[doc = "desc ICGA3C."]
    #[inline(always)]
    pub const fn icga3c(self) -> crate::common::Reg<regs::Icga3c, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x38usize) as _) }
    }
    #[doc = "desc ICGA4A."]
    #[inline(always)]
    pub const fn icga4a(self) -> crate::common::Reg<regs::Icga4a, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x40usize) as _) }
    }
    #[doc = "desc ICGA4B."]
    #[inline(always)]
    pub const fn icga4b(self) -> crate::common::Reg<regs::Icga4b, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x44usize) as _) }
    }
    #[doc = "desc ICGA4C."]
    #[inline(always)]
    pub const fn icga4c(self) -> crate::common::Reg<regs::Icga4c, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x48usize) as _) }
    }
}
pub mod regs {
    #[doc = "desc ICGA0A."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Icga0a(pub u32);
    impl Icga0a {
        #[doc = "desc SWDTAUTS."]
        #[must_use]
        #[inline(always)]
        pub const fn swdtauts(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "desc SWDTAUTS."]
        #[inline(always)]
        pub const fn set_swdtauts(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "desc SWDTITS."]
        #[must_use]
        #[inline(always)]
        pub const fn swdtits(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "desc SWDTITS."]
        #[inline(always)]
        pub const fn set_swdtits(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "desc SWDTPERI."]
        #[must_use]
        #[inline(always)]
        pub const fn swdtperi(&self) -> u8 {
            let val = (self.0 >> 2usize) & 0x03;
            val as u8
        }
        #[doc = "desc SWDTPERI."]
        #[inline(always)]
        pub const fn set_swdtperi(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 2usize)) | (((val as u32) & 0x03) << 2usize);
        }
        #[doc = "desc SWDTCKS."]
        #[must_use]
        #[inline(always)]
        pub const fn swdtcks(&self) -> u8 {
            let val = (self.0 >> 4usize) & 0x0f;
            val as u8
        }
        #[doc = "desc SWDTCKS."]
        #[inline(always)]
        pub const fn set_swdtcks(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 4usize)) | (((val as u32) & 0x0f) << 4usize);
        }
        #[doc = "desc SWDTWDPT."]
        #[must_use]
        #[inline(always)]
        pub const fn swdtwdpt(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0x0f;
            val as u8
        }
        #[doc = "desc SWDTWDPT."]
        #[inline(always)]
        pub const fn set_swdtwdpt(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 8usize)) | (((val as u32) & 0x0f) << 8usize);
        }
        #[doc = "desc SWDTSLPOFF."]
        #[must_use]
        #[inline(always)]
        pub const fn swdtslpoff(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "desc SWDTSLPOFF."]
        #[inline(always)]
        pub const fn set_swdtslpoff(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
    }
    impl Default for Icga0a {
        #[inline(always)]
        fn default() -> Icga0a {
            Icga0a(0)
        }
    }
    impl core::fmt::Debug for Icga0a {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Icga0a")
                .field("swdtauts", &self.swdtauts())
                .field("swdtits", &self.swdtits())
                .field("swdtperi", &self.swdtperi())
                .field("swdtcks", &self.swdtcks())
                .field("swdtwdpt", &self.swdtwdpt())
                .field("swdtslpoff", &self.swdtslpoff())
                .finish()
        }
    }
    #[doc = "desc ICGA0B."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Icga0b(pub u32);
    impl Icga0b {
        #[doc = "desc SWDTAUTS."]
        #[must_use]
        #[inline(always)]
        pub const fn swdtauts(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "desc SWDTAUTS."]
        #[inline(always)]
        pub const fn set_swdtauts(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "desc SWDTITS."]
        #[must_use]
        #[inline(always)]
        pub const fn swdtits(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "desc SWDTITS."]
        #[inline(always)]
        pub const fn set_swdtits(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "desc SWDTPERI."]
        #[must_use]
        #[inline(always)]
        pub const fn swdtperi(&self) -> u8 {
            let val = (self.0 >> 2usize) & 0x03;
            val as u8
        }
        #[doc = "desc SWDTPERI."]
        #[inline(always)]
        pub const fn set_swdtperi(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 2usize)) | (((val as u32) & 0x03) << 2usize);
        }
        #[doc = "desc SWDTCKS."]
        #[must_use]
        #[inline(always)]
        pub const fn swdtcks(&self) -> u8 {
            let val = (self.0 >> 4usize) & 0x0f;
            val as u8
        }
        #[doc = "desc SWDTCKS."]
        #[inline(always)]
        pub const fn set_swdtcks(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 4usize)) | (((val as u32) & 0x0f) << 4usize);
        }
        #[doc = "desc SWDTWDPT."]
        #[must_use]
        #[inline(always)]
        pub const fn swdtwdpt(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0x0f;
            val as u8
        }
        #[doc = "desc SWDTWDPT."]
        #[inline(always)]
        pub const fn set_swdtwdpt(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 8usize)) | (((val as u32) & 0x0f) << 8usize);
        }
        #[doc = "desc SWDTSLPOFF."]
        #[must_use]
        #[inline(always)]
        pub const fn swdtslpoff(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "desc SWDTSLPOFF."]
        #[inline(always)]
        pub const fn set_swdtslpoff(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
    }
    impl Default for Icga0b {
        #[inline(always)]
        fn default() -> Icga0b {
            Icga0b(0)
        }
    }
    impl core::fmt::Debug for Icga0b {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Icga0b")
                .field("swdtauts", &self.swdtauts())
                .field("swdtits", &self.swdtits())
                .field("swdtperi", &self.swdtperi())
                .field("swdtcks", &self.swdtcks())
                .field("swdtwdpt", &self.swdtwdpt())
                .field("swdtslpoff", &self.swdtslpoff())
                .finish()
        }
    }
    #[doc = "desc ICGA0C."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Icga0c(pub u32);
    impl Icga0c {
        #[doc = "desc SWDTAUTS."]
        #[must_use]
        #[inline(always)]
        pub const fn swdtauts(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "desc SWDTAUTS."]
        #[inline(always)]
        pub const fn set_swdtauts(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "desc SWDTITS."]
        #[must_use]
        #[inline(always)]
        pub const fn swdtits(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "desc SWDTITS."]
        #[inline(always)]
        pub const fn set_swdtits(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "desc SWDTPERI."]
        #[must_use]
        #[inline(always)]
        pub const fn swdtperi(&self) -> u8 {
            let val = (self.0 >> 2usize) & 0x03;
            val as u8
        }
        #[doc = "desc SWDTPERI."]
        #[inline(always)]
        pub const fn set_swdtperi(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 2usize)) | (((val as u32) & 0x03) << 2usize);
        }
        #[doc = "desc SWDTCKS."]
        #[must_use]
        #[inline(always)]
        pub const fn swdtcks(&self) -> u8 {
            let val = (self.0 >> 4usize) & 0x0f;
            val as u8
        }
        #[doc = "desc SWDTCKS."]
        #[inline(always)]
        pub const fn set_swdtcks(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 4usize)) | (((val as u32) & 0x0f) << 4usize);
        }
        #[doc = "desc SWDTWDPT."]
        #[must_use]
        #[inline(always)]
        pub const fn swdtwdpt(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0x0f;
            val as u8
        }
        #[doc = "desc SWDTWDPT."]
        #[inline(always)]
        pub const fn set_swdtwdpt(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 8usize)) | (((val as u32) & 0x0f) << 8usize);
        }
        #[doc = "desc SWDTSLPOFF."]
        #[must_use]
        #[inline(always)]
        pub const fn swdtslpoff(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "desc SWDTSLPOFF."]
        #[inline(always)]
        pub const fn set_swdtslpoff(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
    }
    impl Default for Icga0c {
        #[inline(always)]
        fn default() -> Icga0c {
            Icga0c(0)
        }
    }
    impl core::fmt::Debug for Icga0c {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Icga0c")
                .field("swdtauts", &self.swdtauts())
                .field("swdtits", &self.swdtits())
                .field("swdtperi", &self.swdtperi())
                .field("swdtcks", &self.swdtcks())
                .field("swdtwdpt", &self.swdtwdpt())
                .field("swdtslpoff", &self.swdtslpoff())
                .finish()
        }
    }
    #[doc = "desc ICGA1A."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Icga1a(pub u32);
    impl Icga1a {
        #[doc = "desc LVD1DFS."]
        #[must_use]
        #[inline(always)]
        pub const fn lvd1dfs(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x03;
            val as u8
        }
        #[doc = "desc LVD1DFS."]
        #[inline(always)]
        pub const fn set_lvd1dfs(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 0usize)) | (((val as u32) & 0x03) << 0usize);
        }
        #[doc = "desc LVD1DFDIS."]
        #[must_use]
        #[inline(always)]
        pub const fn lvd1dfdis(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "desc LVD1DFDIS."]
        #[inline(always)]
        pub const fn set_lvd1dfdis(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "desc LVD1LVL."]
        #[must_use]
        #[inline(always)]
        pub const fn lvd1lvl(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0x03;
            val as u8
        }
        #[doc = "desc LVD1LVL."]
        #[inline(always)]
        pub const fn set_lvd1lvl(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 8usize)) | (((val as u32) & 0x03) << 8usize);
        }
        #[doc = "desc LVD1NMIS."]
        #[must_use]
        #[inline(always)]
        pub const fn lvd1nmis(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "desc LVD1NMIS."]
        #[inline(always)]
        pub const fn set_lvd1nmis(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "desc LVD1IRS."]
        #[must_use]
        #[inline(always)]
        pub const fn lvd1irs(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[doc = "desc LVD1IRS."]
        #[inline(always)]
        pub const fn set_lvd1irs(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[doc = "desc LVD1IRDIS."]
        #[must_use]
        #[inline(always)]
        pub const fn lvd1irdis(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "desc LVD1IRDIS."]
        #[inline(always)]
        pub const fn set_lvd1irdis(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[doc = "desc LVD1DIS."]
        #[must_use]
        #[inline(always)]
        pub const fn lvd1dis(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "desc LVD1DIS."]
        #[inline(always)]
        pub const fn set_lvd1dis(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
        #[doc = "desc HRCFREQS."]
        #[must_use]
        #[inline(always)]
        pub const fn hrcfreqs(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x0f;
            val as u8
        }
        #[doc = "desc HRCFREQS."]
        #[inline(always)]
        pub const fn set_hrcfreqs(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 16usize)) | (((val as u32) & 0x0f) << 16usize);
        }
        #[doc = "desc LKUPDIS."]
        #[must_use]
        #[inline(always)]
        pub const fn lkupdis(&self) -> bool {
            let val = (self.0 >> 23usize) & 0x01;
            val != 0
        }
        #[doc = "desc LKUPDIS."]
        #[inline(always)]
        pub const fn set_lkupdis(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
        }
        #[doc = "desc NRSTEN."]
        #[must_use]
        #[inline(always)]
        pub const fn nrsten(&self) -> bool {
            let val = (self.0 >> 24usize) & 0x01;
            val != 0
        }
        #[doc = "desc NRSTEN."]
        #[inline(always)]
        pub const fn set_nrsten(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
        }
        #[doc = "desc ROE."]
        #[must_use]
        #[inline(always)]
        pub const fn roe(&self) -> bool {
            let val = (self.0 >> 25usize) & 0x01;
            val != 0
        }
        #[doc = "desc ROE."]
        #[inline(always)]
        pub const fn set_roe(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
        }
        #[doc = "desc NMIFCLK."]
        #[must_use]
        #[inline(always)]
        pub const fn nmifclk(&self) -> u8 {
            let val = (self.0 >> 26usize) & 0x03;
            val as u8
        }
        #[doc = "desc NMIFCLK."]
        #[inline(always)]
        pub const fn set_nmifclk(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 26usize)) | (((val as u32) & 0x03) << 26usize);
        }
        #[doc = "desc NMIFEN."]
        #[must_use]
        #[inline(always)]
        pub const fn nmifen(&self) -> bool {
            let val = (self.0 >> 28usize) & 0x01;
            val != 0
        }
        #[doc = "desc NMIFEN."]
        #[inline(always)]
        pub const fn set_nmifen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
        }
        #[doc = "desc NMITRG."]
        #[must_use]
        #[inline(always)]
        pub const fn nmitrg(&self) -> bool {
            let val = (self.0 >> 29usize) & 0x01;
            val != 0
        }
        #[doc = "desc NMITRG."]
        #[inline(always)]
        pub const fn set_nmitrg(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
        }
        #[doc = "desc NMIEN."]
        #[must_use]
        #[inline(always)]
        pub const fn nmien(&self) -> bool {
            let val = (self.0 >> 30usize) & 0x01;
            val != 0
        }
        #[doc = "desc NMIEN."]
        #[inline(always)]
        pub const fn set_nmien(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
        }
        #[doc = "desc NMIICGEN."]
        #[must_use]
        #[inline(always)]
        pub const fn nmiicgen(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[doc = "desc NMIICGEN."]
        #[inline(always)]
        pub const fn set_nmiicgen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for Icga1a {
        #[inline(always)]
        fn default() -> Icga1a {
            Icga1a(0)
        }
    }
    impl core::fmt::Debug for Icga1a {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Icga1a")
                .field("lvd1dfs", &self.lvd1dfs())
                .field("lvd1dfdis", &self.lvd1dfdis())
                .field("lvd1lvl", &self.lvd1lvl())
                .field("lvd1nmis", &self.lvd1nmis())
                .field("lvd1irs", &self.lvd1irs())
                .field("lvd1irdis", &self.lvd1irdis())
                .field("lvd1dis", &self.lvd1dis())
                .field("hrcfreqs", &self.hrcfreqs())
                .field("lkupdis", &self.lkupdis())
                .field("nrsten", &self.nrsten())
                .field("roe", &self.roe())
                .field("nmifclk", &self.nmifclk())
                .field("nmifen", &self.nmifen())
                .field("nmitrg", &self.nmitrg())
                .field("nmien", &self.nmien())
                .field("nmiicgen", &self.nmiicgen())
                .finish()
        }
    }
    #[doc = "desc ICGA1B."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Icga1b(pub u32);
    impl Icga1b {
        #[doc = "desc LVD1DFS."]
        #[must_use]
        #[inline(always)]
        pub const fn lvd1dfs(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x03;
            val as u8
        }
        #[doc = "desc LVD1DFS."]
        #[inline(always)]
        pub const fn set_lvd1dfs(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 0usize)) | (((val as u32) & 0x03) << 0usize);
        }
        #[doc = "desc LVD1DFDIS."]
        #[must_use]
        #[inline(always)]
        pub const fn lvd1dfdis(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "desc LVD1DFDIS."]
        #[inline(always)]
        pub const fn set_lvd1dfdis(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "desc LVD1LVL."]
        #[must_use]
        #[inline(always)]
        pub const fn lvd1lvl(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0x03;
            val as u8
        }
        #[doc = "desc LVD1LVL."]
        #[inline(always)]
        pub const fn set_lvd1lvl(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 8usize)) | (((val as u32) & 0x03) << 8usize);
        }
        #[doc = "desc LVD1NMIS."]
        #[must_use]
        #[inline(always)]
        pub const fn lvd1nmis(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "desc LVD1NMIS."]
        #[inline(always)]
        pub const fn set_lvd1nmis(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "desc LVD1IRS."]
        #[must_use]
        #[inline(always)]
        pub const fn lvd1irs(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[doc = "desc LVD1IRS."]
        #[inline(always)]
        pub const fn set_lvd1irs(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[doc = "desc LVD1IRDIS."]
        #[must_use]
        #[inline(always)]
        pub const fn lvd1irdis(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "desc LVD1IRDIS."]
        #[inline(always)]
        pub const fn set_lvd1irdis(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[doc = "desc LVD1DIS."]
        #[must_use]
        #[inline(always)]
        pub const fn lvd1dis(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "desc LVD1DIS."]
        #[inline(always)]
        pub const fn set_lvd1dis(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
        #[doc = "desc HRCFREQS."]
        #[must_use]
        #[inline(always)]
        pub const fn hrcfreqs(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x0f;
            val as u8
        }
        #[doc = "desc HRCFREQS."]
        #[inline(always)]
        pub const fn set_hrcfreqs(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 16usize)) | (((val as u32) & 0x0f) << 16usize);
        }
        #[doc = "desc LKUPDIS."]
        #[must_use]
        #[inline(always)]
        pub const fn lkupdis(&self) -> bool {
            let val = (self.0 >> 23usize) & 0x01;
            val != 0
        }
        #[doc = "desc LKUPDIS."]
        #[inline(always)]
        pub const fn set_lkupdis(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
        }
        #[doc = "desc NRSTEN."]
        #[must_use]
        #[inline(always)]
        pub const fn nrsten(&self) -> bool {
            let val = (self.0 >> 24usize) & 0x01;
            val != 0
        }
        #[doc = "desc NRSTEN."]
        #[inline(always)]
        pub const fn set_nrsten(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
        }
        #[doc = "desc ROE."]
        #[must_use]
        #[inline(always)]
        pub const fn roe(&self) -> bool {
            let val = (self.0 >> 25usize) & 0x01;
            val != 0
        }
        #[doc = "desc ROE."]
        #[inline(always)]
        pub const fn set_roe(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
        }
        #[doc = "desc NMIFCLK."]
        #[must_use]
        #[inline(always)]
        pub const fn nmifclk(&self) -> u8 {
            let val = (self.0 >> 26usize) & 0x03;
            val as u8
        }
        #[doc = "desc NMIFCLK."]
        #[inline(always)]
        pub const fn set_nmifclk(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 26usize)) | (((val as u32) & 0x03) << 26usize);
        }
        #[doc = "desc NMIFEN."]
        #[must_use]
        #[inline(always)]
        pub const fn nmifen(&self) -> bool {
            let val = (self.0 >> 28usize) & 0x01;
            val != 0
        }
        #[doc = "desc NMIFEN."]
        #[inline(always)]
        pub const fn set_nmifen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
        }
        #[doc = "desc NMITRG."]
        #[must_use]
        #[inline(always)]
        pub const fn nmitrg(&self) -> bool {
            let val = (self.0 >> 29usize) & 0x01;
            val != 0
        }
        #[doc = "desc NMITRG."]
        #[inline(always)]
        pub const fn set_nmitrg(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
        }
        #[doc = "desc NMIEN."]
        #[must_use]
        #[inline(always)]
        pub const fn nmien(&self) -> bool {
            let val = (self.0 >> 30usize) & 0x01;
            val != 0
        }
        #[doc = "desc NMIEN."]
        #[inline(always)]
        pub const fn set_nmien(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
        }
        #[doc = "desc NMIICGEN."]
        #[must_use]
        #[inline(always)]
        pub const fn nmiicgen(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[doc = "desc NMIICGEN."]
        #[inline(always)]
        pub const fn set_nmiicgen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for Icga1b {
        #[inline(always)]
        fn default() -> Icga1b {
            Icga1b(0)
        }
    }
    impl core::fmt::Debug for Icga1b {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Icga1b")
                .field("lvd1dfs", &self.lvd1dfs())
                .field("lvd1dfdis", &self.lvd1dfdis())
                .field("lvd1lvl", &self.lvd1lvl())
                .field("lvd1nmis", &self.lvd1nmis())
                .field("lvd1irs", &self.lvd1irs())
                .field("lvd1irdis", &self.lvd1irdis())
                .field("lvd1dis", &self.lvd1dis())
                .field("hrcfreqs", &self.hrcfreqs())
                .field("lkupdis", &self.lkupdis())
                .field("nrsten", &self.nrsten())
                .field("roe", &self.roe())
                .field("nmifclk", &self.nmifclk())
                .field("nmifen", &self.nmifen())
                .field("nmitrg", &self.nmitrg())
                .field("nmien", &self.nmien())
                .field("nmiicgen", &self.nmiicgen())
                .finish()
        }
    }
    #[doc = "desc ICGA1C."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Icga1c(pub u32);
    impl Icga1c {
        #[doc = "desc LVD1DFS."]
        #[must_use]
        #[inline(always)]
        pub const fn lvd1dfs(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x03;
            val as u8
        }
        #[doc = "desc LVD1DFS."]
        #[inline(always)]
        pub const fn set_lvd1dfs(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 0usize)) | (((val as u32) & 0x03) << 0usize);
        }
        #[doc = "desc LVD1DFDIS."]
        #[must_use]
        #[inline(always)]
        pub const fn lvd1dfdis(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "desc LVD1DFDIS."]
        #[inline(always)]
        pub const fn set_lvd1dfdis(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "desc LVD1LVL."]
        #[must_use]
        #[inline(always)]
        pub const fn lvd1lvl(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0x03;
            val as u8
        }
        #[doc = "desc LVD1LVL."]
        #[inline(always)]
        pub const fn set_lvd1lvl(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 8usize)) | (((val as u32) & 0x03) << 8usize);
        }
        #[doc = "desc LVD1NMIS."]
        #[must_use]
        #[inline(always)]
        pub const fn lvd1nmis(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "desc LVD1NMIS."]
        #[inline(always)]
        pub const fn set_lvd1nmis(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "desc LVD1IRS."]
        #[must_use]
        #[inline(always)]
        pub const fn lvd1irs(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[doc = "desc LVD1IRS."]
        #[inline(always)]
        pub const fn set_lvd1irs(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[doc = "desc LVD1IRDIS."]
        #[must_use]
        #[inline(always)]
        pub const fn lvd1irdis(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "desc LVD1IRDIS."]
        #[inline(always)]
        pub const fn set_lvd1irdis(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[doc = "desc LVD1DIS."]
        #[must_use]
        #[inline(always)]
        pub const fn lvd1dis(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "desc LVD1DIS."]
        #[inline(always)]
        pub const fn set_lvd1dis(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
        #[doc = "desc HRCFREQS."]
        #[must_use]
        #[inline(always)]
        pub const fn hrcfreqs(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x0f;
            val as u8
        }
        #[doc = "desc HRCFREQS."]
        #[inline(always)]
        pub const fn set_hrcfreqs(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 16usize)) | (((val as u32) & 0x0f) << 16usize);
        }
        #[doc = "desc LKUPDIS."]
        #[must_use]
        #[inline(always)]
        pub const fn lkupdis(&self) -> bool {
            let val = (self.0 >> 23usize) & 0x01;
            val != 0
        }
        #[doc = "desc LKUPDIS."]
        #[inline(always)]
        pub const fn set_lkupdis(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
        }
        #[doc = "desc NRSTEN."]
        #[must_use]
        #[inline(always)]
        pub const fn nrsten(&self) -> bool {
            let val = (self.0 >> 24usize) & 0x01;
            val != 0
        }
        #[doc = "desc NRSTEN."]
        #[inline(always)]
        pub const fn set_nrsten(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
        }
        #[doc = "desc ROE."]
        #[must_use]
        #[inline(always)]
        pub const fn roe(&self) -> bool {
            let val = (self.0 >> 25usize) & 0x01;
            val != 0
        }
        #[doc = "desc ROE."]
        #[inline(always)]
        pub const fn set_roe(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
        }
        #[doc = "desc NMIFCLK."]
        #[must_use]
        #[inline(always)]
        pub const fn nmifclk(&self) -> u8 {
            let val = (self.0 >> 26usize) & 0x03;
            val as u8
        }
        #[doc = "desc NMIFCLK."]
        #[inline(always)]
        pub const fn set_nmifclk(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 26usize)) | (((val as u32) & 0x03) << 26usize);
        }
        #[doc = "desc NMIFEN."]
        #[must_use]
        #[inline(always)]
        pub const fn nmifen(&self) -> bool {
            let val = (self.0 >> 28usize) & 0x01;
            val != 0
        }
        #[doc = "desc NMIFEN."]
        #[inline(always)]
        pub const fn set_nmifen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
        }
        #[doc = "desc NMITRG."]
        #[must_use]
        #[inline(always)]
        pub const fn nmitrg(&self) -> bool {
            let val = (self.0 >> 29usize) & 0x01;
            val != 0
        }
        #[doc = "desc NMITRG."]
        #[inline(always)]
        pub const fn set_nmitrg(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
        }
        #[doc = "desc NMIEN."]
        #[must_use]
        #[inline(always)]
        pub const fn nmien(&self) -> bool {
            let val = (self.0 >> 30usize) & 0x01;
            val != 0
        }
        #[doc = "desc NMIEN."]
        #[inline(always)]
        pub const fn set_nmien(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
        }
        #[doc = "desc NMIICGEN."]
        #[must_use]
        #[inline(always)]
        pub const fn nmiicgen(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[doc = "desc NMIICGEN."]
        #[inline(always)]
        pub const fn set_nmiicgen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for Icga1c {
        #[inline(always)]
        fn default() -> Icga1c {
            Icga1c(0)
        }
    }
    impl core::fmt::Debug for Icga1c {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Icga1c")
                .field("lvd1dfs", &self.lvd1dfs())
                .field("lvd1dfdis", &self.lvd1dfdis())
                .field("lvd1lvl", &self.lvd1lvl())
                .field("lvd1nmis", &self.lvd1nmis())
                .field("lvd1irs", &self.lvd1irs())
                .field("lvd1irdis", &self.lvd1irdis())
                .field("lvd1dis", &self.lvd1dis())
                .field("hrcfreqs", &self.hrcfreqs())
                .field("lkupdis", &self.lkupdis())
                .field("nrsten", &self.nrsten())
                .field("roe", &self.roe())
                .field("nmifclk", &self.nmifclk())
                .field("nmifen", &self.nmifen())
                .field("nmitrg", &self.nmitrg())
                .field("nmien", &self.nmien())
                .field("nmiicgen", &self.nmiicgen())
                .finish()
        }
    }
    #[doc = "desc ICGA2A."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Icga2a(pub u32);
    impl Icga2a {
        #[doc = "desc ECCOAD."]
        #[must_use]
        #[inline(always)]
        pub const fn eccoad(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "desc ECCOAD."]
        #[inline(always)]
        pub const fn set_eccoad(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "desc ECCMOD."]
        #[must_use]
        #[inline(always)]
        pub const fn eccmod(&self) -> u8 {
            let val = (self.0 >> 1usize) & 0x03;
            val as u8
        }
        #[doc = "desc ECCMOD."]
        #[inline(always)]
        pub const fn set_eccmod(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 1usize)) | (((val as u32) & 0x03) << 1usize);
        }
        #[doc = "desc DBUSPSZ."]
        #[must_use]
        #[inline(always)]
        pub const fn dbuspsz(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "desc DBUSPSZ."]
        #[inline(always)]
        pub const fn set_dbuspsz(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "desc DBUSPRT."]
        #[must_use]
        #[inline(always)]
        pub const fn dbusprt(&self) -> u16 {
            let val = (self.0 >> 20usize) & 0x0fff;
            val as u16
        }
        #[doc = "desc DBUSPRT."]
        #[inline(always)]
        pub const fn set_dbusprt(&mut self, val: u16) {
            self.0 = (self.0 & !(0x0fff << 20usize)) | (((val as u32) & 0x0fff) << 20usize);
        }
    }
    impl Default for Icga2a {
        #[inline(always)]
        fn default() -> Icga2a {
            Icga2a(0)
        }
    }
    impl core::fmt::Debug for Icga2a {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Icga2a")
                .field("eccoad", &self.eccoad())
                .field("eccmod", &self.eccmod())
                .field("dbuspsz", &self.dbuspsz())
                .field("dbusprt", &self.dbusprt())
                .finish()
        }
    }
    #[doc = "desc ICGA2B."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Icga2b(pub u32);
    impl Icga2b {
        #[doc = "desc ECCOAD."]
        #[must_use]
        #[inline(always)]
        pub const fn eccoad(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "desc ECCOAD."]
        #[inline(always)]
        pub const fn set_eccoad(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "desc ECCMOD."]
        #[must_use]
        #[inline(always)]
        pub const fn eccmod(&self) -> u8 {
            let val = (self.0 >> 1usize) & 0x03;
            val as u8
        }
        #[doc = "desc ECCMOD."]
        #[inline(always)]
        pub const fn set_eccmod(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 1usize)) | (((val as u32) & 0x03) << 1usize);
        }
        #[doc = "desc DBUSPSZ."]
        #[must_use]
        #[inline(always)]
        pub const fn dbuspsz(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "desc DBUSPSZ."]
        #[inline(always)]
        pub const fn set_dbuspsz(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "desc DBUSPRT."]
        #[must_use]
        #[inline(always)]
        pub const fn dbusprt(&self) -> u16 {
            let val = (self.0 >> 20usize) & 0x0fff;
            val as u16
        }
        #[doc = "desc DBUSPRT."]
        #[inline(always)]
        pub const fn set_dbusprt(&mut self, val: u16) {
            self.0 = (self.0 & !(0x0fff << 20usize)) | (((val as u32) & 0x0fff) << 20usize);
        }
    }
    impl Default for Icga2b {
        #[inline(always)]
        fn default() -> Icga2b {
            Icga2b(0)
        }
    }
    impl core::fmt::Debug for Icga2b {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Icga2b")
                .field("eccoad", &self.eccoad())
                .field("eccmod", &self.eccmod())
                .field("dbuspsz", &self.dbuspsz())
                .field("dbusprt", &self.dbusprt())
                .finish()
        }
    }
    #[doc = "desc ICGA2C."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Icga2c(pub u32);
    impl Icga2c {
        #[doc = "desc ECCOAD."]
        #[must_use]
        #[inline(always)]
        pub const fn eccoad(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "desc ECCOAD."]
        #[inline(always)]
        pub const fn set_eccoad(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "desc ECCMOD."]
        #[must_use]
        #[inline(always)]
        pub const fn eccmod(&self) -> u8 {
            let val = (self.0 >> 1usize) & 0x03;
            val as u8
        }
        #[doc = "desc ECCMOD."]
        #[inline(always)]
        pub const fn set_eccmod(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 1usize)) | (((val as u32) & 0x03) << 1usize);
        }
        #[doc = "desc DBUSPSZ."]
        #[must_use]
        #[inline(always)]
        pub const fn dbuspsz(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "desc DBUSPSZ."]
        #[inline(always)]
        pub const fn set_dbuspsz(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "desc DBUSPRT."]
        #[must_use]
        #[inline(always)]
        pub const fn dbusprt(&self) -> u16 {
            let val = (self.0 >> 20usize) & 0x0fff;
            val as u16
        }
        #[doc = "desc DBUSPRT."]
        #[inline(always)]
        pub const fn set_dbusprt(&mut self, val: u16) {
            self.0 = (self.0 & !(0x0fff << 20usize)) | (((val as u32) & 0x0fff) << 20usize);
        }
    }
    impl Default for Icga2c {
        #[inline(always)]
        fn default() -> Icga2c {
            Icga2c(0)
        }
    }
    impl core::fmt::Debug for Icga2c {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Icga2c")
                .field("eccoad", &self.eccoad())
                .field("eccmod", &self.eccmod())
                .field("dbuspsz", &self.dbuspsz())
                .field("dbusprt", &self.dbusprt())
                .finish()
        }
    }
    #[doc = "desc ICGA3A."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Icga3a(pub u32);
    impl Icga3a {
        #[doc = "desc FNWPRT0."]
        #[must_use]
        #[inline(always)]
        pub const fn fnwprt0(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "desc FNWPRT0."]
        #[inline(always)]
        pub const fn set_fnwprt0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "desc FNWPRT1."]
        #[must_use]
        #[inline(always)]
        pub const fn fnwprt1(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "desc FNWPRT1."]
        #[inline(always)]
        pub const fn set_fnwprt1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "desc FNWPRT2."]
        #[must_use]
        #[inline(always)]
        pub const fn fnwprt2(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "desc FNWPRT2."]
        #[inline(always)]
        pub const fn set_fnwprt2(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "desc FNWPRT3."]
        #[must_use]
        #[inline(always)]
        pub const fn fnwprt3(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "desc FNWPRT3."]
        #[inline(always)]
        pub const fn set_fnwprt3(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "desc FNWPRT4."]
        #[must_use]
        #[inline(always)]
        pub const fn fnwprt4(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "desc FNWPRT4."]
        #[inline(always)]
        pub const fn set_fnwprt4(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "desc FNWPRT5."]
        #[must_use]
        #[inline(always)]
        pub const fn fnwprt5(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "desc FNWPRT5."]
        #[inline(always)]
        pub const fn set_fnwprt5(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "desc FNWPRT6."]
        #[must_use]
        #[inline(always)]
        pub const fn fnwprt6(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "desc FNWPRT6."]
        #[inline(always)]
        pub const fn set_fnwprt6(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "desc FNWPRT7."]
        #[must_use]
        #[inline(always)]
        pub const fn fnwprt7(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "desc FNWPRT7."]
        #[inline(always)]
        pub const fn set_fnwprt7(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "desc FNWPRT8."]
        #[must_use]
        #[inline(always)]
        pub const fn fnwprt8(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "desc FNWPRT8."]
        #[inline(always)]
        pub const fn set_fnwprt8(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "desc FNWPRT9."]
        #[must_use]
        #[inline(always)]
        pub const fn fnwprt9(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "desc FNWPRT9."]
        #[inline(always)]
        pub const fn set_fnwprt9(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "desc FNWPRT10."]
        #[must_use]
        #[inline(always)]
        pub const fn fnwprt10(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "desc FNWPRT10."]
        #[inline(always)]
        pub const fn set_fnwprt10(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[doc = "desc FNWPRT11."]
        #[must_use]
        #[inline(always)]
        pub const fn fnwprt11(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "desc FNWPRT11."]
        #[inline(always)]
        pub const fn set_fnwprt11(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[doc = "desc FNWPRT12."]
        #[must_use]
        #[inline(always)]
        pub const fn fnwprt12(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "desc FNWPRT12."]
        #[inline(always)]
        pub const fn set_fnwprt12(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "desc FNWPRT13."]
        #[must_use]
        #[inline(always)]
        pub const fn fnwprt13(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[doc = "desc FNWPRT13."]
        #[inline(always)]
        pub const fn set_fnwprt13(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[doc = "desc FNWPRT14."]
        #[must_use]
        #[inline(always)]
        pub const fn fnwprt14(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "desc FNWPRT14."]
        #[inline(always)]
        pub const fn set_fnwprt14(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[doc = "desc FNWPRT15."]
        #[must_use]
        #[inline(always)]
        pub const fn fnwprt15(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "desc FNWPRT15."]
        #[inline(always)]
        pub const fn set_fnwprt15(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
        #[doc = "desc FNWPRT16."]
        #[must_use]
        #[inline(always)]
        pub const fn fnwprt16(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "desc FNWPRT16."]
        #[inline(always)]
        pub const fn set_fnwprt16(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "desc FNWPRT17."]
        #[must_use]
        #[inline(always)]
        pub const fn fnwprt17(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "desc FNWPRT17."]
        #[inline(always)]
        pub const fn set_fnwprt17(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[doc = "desc FNWPRT18."]
        #[must_use]
        #[inline(always)]
        pub const fn fnwprt18(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[doc = "desc FNWPRT18."]
        #[inline(always)]
        pub const fn set_fnwprt18(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
        #[doc = "desc FNWPRT19."]
        #[must_use]
        #[inline(always)]
        pub const fn fnwprt19(&self) -> bool {
            let val = (self.0 >> 19usize) & 0x01;
            val != 0
        }
        #[doc = "desc FNWPRT19."]
        #[inline(always)]
        pub const fn set_fnwprt19(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
        }
        #[doc = "desc FNWPRT20."]
        #[must_use]
        #[inline(always)]
        pub const fn fnwprt20(&self) -> bool {
            let val = (self.0 >> 20usize) & 0x01;
            val != 0
        }
        #[doc = "desc FNWPRT20."]
        #[inline(always)]
        pub const fn set_fnwprt20(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
        }
        #[doc = "desc FNWPRT21."]
        #[must_use]
        #[inline(always)]
        pub const fn fnwprt21(&self) -> bool {
            let val = (self.0 >> 21usize) & 0x01;
            val != 0
        }
        #[doc = "desc FNWPRT21."]
        #[inline(always)]
        pub const fn set_fnwprt21(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
        }
        #[doc = "desc FNWPRT22."]
        #[must_use]
        #[inline(always)]
        pub const fn fnwprt22(&self) -> bool {
            let val = (self.0 >> 22usize) & 0x01;
            val != 0
        }
        #[doc = "desc FNWPRT22."]
        #[inline(always)]
        pub const fn set_fnwprt22(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
        }
        #[doc = "desc FNWPRT23."]
        #[must_use]
        #[inline(always)]
        pub const fn fnwprt23(&self) -> bool {
            let val = (self.0 >> 23usize) & 0x01;
            val != 0
        }
        #[doc = "desc FNWPRT23."]
        #[inline(always)]
        pub const fn set_fnwprt23(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
        }
        #[doc = "desc FNWPRT24."]
        #[must_use]
        #[inline(always)]
        pub const fn fnwprt24(&self) -> bool {
            let val = (self.0 >> 24usize) & 0x01;
            val != 0
        }
        #[doc = "desc FNWPRT24."]
        #[inline(always)]
        pub const fn set_fnwprt24(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
        }
        #[doc = "desc FNWPRT25."]
        #[must_use]
        #[inline(always)]
        pub const fn fnwprt25(&self) -> bool {
            let val = (self.0 >> 25usize) & 0x01;
            val != 0
        }
        #[doc = "desc FNWPRT25."]
        #[inline(always)]
        pub const fn set_fnwprt25(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
        }
        #[doc = "desc FNWPRT26."]
        #[must_use]
        #[inline(always)]
        pub const fn fnwprt26(&self) -> bool {
            let val = (self.0 >> 26usize) & 0x01;
            val != 0
        }
        #[doc = "desc FNWPRT26."]
        #[inline(always)]
        pub const fn set_fnwprt26(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
        }
        #[doc = "desc FNWPRT27."]
        #[must_use]
        #[inline(always)]
        pub const fn fnwprt27(&self) -> bool {
            let val = (self.0 >> 27usize) & 0x01;
            val != 0
        }
        #[doc = "desc FNWPRT27."]
        #[inline(always)]
        pub const fn set_fnwprt27(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
        }
        #[doc = "desc FNWPRT28."]
        #[must_use]
        #[inline(always)]
        pub const fn fnwprt28(&self) -> bool {
            let val = (self.0 >> 28usize) & 0x01;
            val != 0
        }
        #[doc = "desc FNWPRT28."]
        #[inline(always)]
        pub const fn set_fnwprt28(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
        }
        #[doc = "desc FNWPRT29."]
        #[must_use]
        #[inline(always)]
        pub const fn fnwprt29(&self) -> bool {
            let val = (self.0 >> 29usize) & 0x01;
            val != 0
        }
        #[doc = "desc FNWPRT29."]
        #[inline(always)]
        pub const fn set_fnwprt29(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
        }
        #[doc = "desc FNWPRT30."]
        #[must_use]
        #[inline(always)]
        pub const fn fnwprt30(&self) -> bool {
            let val = (self.0 >> 30usize) & 0x01;
            val != 0
        }
        #[doc = "desc FNWPRT30."]
        #[inline(always)]
        pub const fn set_fnwprt30(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
        }
        #[doc = "desc FNWPRT31."]
        #[must_use]
        #[inline(always)]
        pub const fn fnwprt31(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[doc = "desc FNWPRT31."]
        #[inline(always)]
        pub const fn set_fnwprt31(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for Icga3a {
        #[inline(always)]
        fn default() -> Icga3a {
            Icga3a(0)
        }
    }
    impl core::fmt::Debug for Icga3a {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Icga3a")
                .field("fnwprt0", &self.fnwprt0())
                .field("fnwprt1", &self.fnwprt1())
                .field("fnwprt2", &self.fnwprt2())
                .field("fnwprt3", &self.fnwprt3())
                .field("fnwprt4", &self.fnwprt4())
                .field("fnwprt5", &self.fnwprt5())
                .field("fnwprt6", &self.fnwprt6())
                .field("fnwprt7", &self.fnwprt7())
                .field("fnwprt8", &self.fnwprt8())
                .field("fnwprt9", &self.fnwprt9())
                .field("fnwprt10", &self.fnwprt10())
                .field("fnwprt11", &self.fnwprt11())
                .field("fnwprt12", &self.fnwprt12())
                .field("fnwprt13", &self.fnwprt13())
                .field("fnwprt14", &self.fnwprt14())
                .field("fnwprt15", &self.fnwprt15())
                .field("fnwprt16", &self.fnwprt16())
                .field("fnwprt17", &self.fnwprt17())
                .field("fnwprt18", &self.fnwprt18())
                .field("fnwprt19", &self.fnwprt19())
                .field("fnwprt20", &self.fnwprt20())
                .field("fnwprt21", &self.fnwprt21())
                .field("fnwprt22", &self.fnwprt22())
                .field("fnwprt23", &self.fnwprt23())
                .field("fnwprt24", &self.fnwprt24())
                .field("fnwprt25", &self.fnwprt25())
                .field("fnwprt26", &self.fnwprt26())
                .field("fnwprt27", &self.fnwprt27())
                .field("fnwprt28", &self.fnwprt28())
                .field("fnwprt29", &self.fnwprt29())
                .field("fnwprt30", &self.fnwprt30())
                .field("fnwprt31", &self.fnwprt31())
                .finish()
        }
    }
    #[doc = "desc ICGA3B."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Icga3b(pub u32);
    impl Icga3b {
        #[doc = "desc FNWPRT0."]
        #[must_use]
        #[inline(always)]
        pub const fn fnwprt0(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "desc FNWPRT0."]
        #[inline(always)]
        pub const fn set_fnwprt0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "desc FNWPRT1."]
        #[must_use]
        #[inline(always)]
        pub const fn fnwprt1(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "desc FNWPRT1."]
        #[inline(always)]
        pub const fn set_fnwprt1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "desc FNWPRT2."]
        #[must_use]
        #[inline(always)]
        pub const fn fnwprt2(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "desc FNWPRT2."]
        #[inline(always)]
        pub const fn set_fnwprt2(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "desc FNWPRT3."]
        #[must_use]
        #[inline(always)]
        pub const fn fnwprt3(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "desc FNWPRT3."]
        #[inline(always)]
        pub const fn set_fnwprt3(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "desc FNWPRT4."]
        #[must_use]
        #[inline(always)]
        pub const fn fnwprt4(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "desc FNWPRT4."]
        #[inline(always)]
        pub const fn set_fnwprt4(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "desc FNWPRT5."]
        #[must_use]
        #[inline(always)]
        pub const fn fnwprt5(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "desc FNWPRT5."]
        #[inline(always)]
        pub const fn set_fnwprt5(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "desc FNWPRT6."]
        #[must_use]
        #[inline(always)]
        pub const fn fnwprt6(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "desc FNWPRT6."]
        #[inline(always)]
        pub const fn set_fnwprt6(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "desc FNWPRT7."]
        #[must_use]
        #[inline(always)]
        pub const fn fnwprt7(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "desc FNWPRT7."]
        #[inline(always)]
        pub const fn set_fnwprt7(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "desc FNWPRT8."]
        #[must_use]
        #[inline(always)]
        pub const fn fnwprt8(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "desc FNWPRT8."]
        #[inline(always)]
        pub const fn set_fnwprt8(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "desc FNWPRT9."]
        #[must_use]
        #[inline(always)]
        pub const fn fnwprt9(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "desc FNWPRT9."]
        #[inline(always)]
        pub const fn set_fnwprt9(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "desc FNWPRT10."]
        #[must_use]
        #[inline(always)]
        pub const fn fnwprt10(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "desc FNWPRT10."]
        #[inline(always)]
        pub const fn set_fnwprt10(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[doc = "desc FNWPRT11."]
        #[must_use]
        #[inline(always)]
        pub const fn fnwprt11(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "desc FNWPRT11."]
        #[inline(always)]
        pub const fn set_fnwprt11(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[doc = "desc FNWPRT12."]
        #[must_use]
        #[inline(always)]
        pub const fn fnwprt12(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "desc FNWPRT12."]
        #[inline(always)]
        pub const fn set_fnwprt12(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "desc FNWPRT13."]
        #[must_use]
        #[inline(always)]
        pub const fn fnwprt13(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[doc = "desc FNWPRT13."]
        #[inline(always)]
        pub const fn set_fnwprt13(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[doc = "desc FNWPRT14."]
        #[must_use]
        #[inline(always)]
        pub const fn fnwprt14(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "desc FNWPRT14."]
        #[inline(always)]
        pub const fn set_fnwprt14(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[doc = "desc FNWPRT15."]
        #[must_use]
        #[inline(always)]
        pub const fn fnwprt15(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "desc FNWPRT15."]
        #[inline(always)]
        pub const fn set_fnwprt15(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
        #[doc = "desc FNWPRT16."]
        #[must_use]
        #[inline(always)]
        pub const fn fnwprt16(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "desc FNWPRT16."]
        #[inline(always)]
        pub const fn set_fnwprt16(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "desc FNWPRT17."]
        #[must_use]
        #[inline(always)]
        pub const fn fnwprt17(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "desc FNWPRT17."]
        #[inline(always)]
        pub const fn set_fnwprt17(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[doc = "desc FNWPRT18."]
        #[must_use]
        #[inline(always)]
        pub const fn fnwprt18(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[doc = "desc FNWPRT18."]
        #[inline(always)]
        pub const fn set_fnwprt18(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
        #[doc = "desc FNWPRT19."]
        #[must_use]
        #[inline(always)]
        pub const fn fnwprt19(&self) -> bool {
            let val = (self.0 >> 19usize) & 0x01;
            val != 0
        }
        #[doc = "desc FNWPRT19."]
        #[inline(always)]
        pub const fn set_fnwprt19(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
        }
        #[doc = "desc FNWPRT20."]
        #[must_use]
        #[inline(always)]
        pub const fn fnwprt20(&self) -> bool {
            let val = (self.0 >> 20usize) & 0x01;
            val != 0
        }
        #[doc = "desc FNWPRT20."]
        #[inline(always)]
        pub const fn set_fnwprt20(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
        }
        #[doc = "desc FNWPRT21."]
        #[must_use]
        #[inline(always)]
        pub const fn fnwprt21(&self) -> bool {
            let val = (self.0 >> 21usize) & 0x01;
            val != 0
        }
        #[doc = "desc FNWPRT21."]
        #[inline(always)]
        pub const fn set_fnwprt21(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
        }
        #[doc = "desc FNWPRT22."]
        #[must_use]
        #[inline(always)]
        pub const fn fnwprt22(&self) -> bool {
            let val = (self.0 >> 22usize) & 0x01;
            val != 0
        }
        #[doc = "desc FNWPRT22."]
        #[inline(always)]
        pub const fn set_fnwprt22(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
        }
        #[doc = "desc FNWPRT23."]
        #[must_use]
        #[inline(always)]
        pub const fn fnwprt23(&self) -> bool {
            let val = (self.0 >> 23usize) & 0x01;
            val != 0
        }
        #[doc = "desc FNWPRT23."]
        #[inline(always)]
        pub const fn set_fnwprt23(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
        }
        #[doc = "desc FNWPRT24."]
        #[must_use]
        #[inline(always)]
        pub const fn fnwprt24(&self) -> bool {
            let val = (self.0 >> 24usize) & 0x01;
            val != 0
        }
        #[doc = "desc FNWPRT24."]
        #[inline(always)]
        pub const fn set_fnwprt24(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
        }
        #[doc = "desc FNWPRT25."]
        #[must_use]
        #[inline(always)]
        pub const fn fnwprt25(&self) -> bool {
            let val = (self.0 >> 25usize) & 0x01;
            val != 0
        }
        #[doc = "desc FNWPRT25."]
        #[inline(always)]
        pub const fn set_fnwprt25(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
        }
        #[doc = "desc FNWPRT26."]
        #[must_use]
        #[inline(always)]
        pub const fn fnwprt26(&self) -> bool {
            let val = (self.0 >> 26usize) & 0x01;
            val != 0
        }
        #[doc = "desc FNWPRT26."]
        #[inline(always)]
        pub const fn set_fnwprt26(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
        }
        #[doc = "desc FNWPRT27."]
        #[must_use]
        #[inline(always)]
        pub const fn fnwprt27(&self) -> bool {
            let val = (self.0 >> 27usize) & 0x01;
            val != 0
        }
        #[doc = "desc FNWPRT27."]
        #[inline(always)]
        pub const fn set_fnwprt27(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
        }
        #[doc = "desc FNWPRT28."]
        #[must_use]
        #[inline(always)]
        pub const fn fnwprt28(&self) -> bool {
            let val = (self.0 >> 28usize) & 0x01;
            val != 0
        }
        #[doc = "desc FNWPRT28."]
        #[inline(always)]
        pub const fn set_fnwprt28(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
        }
        #[doc = "desc FNWPRT29."]
        #[must_use]
        #[inline(always)]
        pub const fn fnwprt29(&self) -> bool {
            let val = (self.0 >> 29usize) & 0x01;
            val != 0
        }
        #[doc = "desc FNWPRT29."]
        #[inline(always)]
        pub const fn set_fnwprt29(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
        }
        #[doc = "desc FNWPRT30."]
        #[must_use]
        #[inline(always)]
        pub const fn fnwprt30(&self) -> bool {
            let val = (self.0 >> 30usize) & 0x01;
            val != 0
        }
        #[doc = "desc FNWPRT30."]
        #[inline(always)]
        pub const fn set_fnwprt30(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
        }
        #[doc = "desc FNWPRT31."]
        #[must_use]
        #[inline(always)]
        pub const fn fnwprt31(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[doc = "desc FNWPRT31."]
        #[inline(always)]
        pub const fn set_fnwprt31(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for Icga3b {
        #[inline(always)]
        fn default() -> Icga3b {
            Icga3b(0)
        }
    }
    impl core::fmt::Debug for Icga3b {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Icga3b")
                .field("fnwprt0", &self.fnwprt0())
                .field("fnwprt1", &self.fnwprt1())
                .field("fnwprt2", &self.fnwprt2())
                .field("fnwprt3", &self.fnwprt3())
                .field("fnwprt4", &self.fnwprt4())
                .field("fnwprt5", &self.fnwprt5())
                .field("fnwprt6", &self.fnwprt6())
                .field("fnwprt7", &self.fnwprt7())
                .field("fnwprt8", &self.fnwprt8())
                .field("fnwprt9", &self.fnwprt9())
                .field("fnwprt10", &self.fnwprt10())
                .field("fnwprt11", &self.fnwprt11())
                .field("fnwprt12", &self.fnwprt12())
                .field("fnwprt13", &self.fnwprt13())
                .field("fnwprt14", &self.fnwprt14())
                .field("fnwprt15", &self.fnwprt15())
                .field("fnwprt16", &self.fnwprt16())
                .field("fnwprt17", &self.fnwprt17())
                .field("fnwprt18", &self.fnwprt18())
                .field("fnwprt19", &self.fnwprt19())
                .field("fnwprt20", &self.fnwprt20())
                .field("fnwprt21", &self.fnwprt21())
                .field("fnwprt22", &self.fnwprt22())
                .field("fnwprt23", &self.fnwprt23())
                .field("fnwprt24", &self.fnwprt24())
                .field("fnwprt25", &self.fnwprt25())
                .field("fnwprt26", &self.fnwprt26())
                .field("fnwprt27", &self.fnwprt27())
                .field("fnwprt28", &self.fnwprt28())
                .field("fnwprt29", &self.fnwprt29())
                .field("fnwprt30", &self.fnwprt30())
                .field("fnwprt31", &self.fnwprt31())
                .finish()
        }
    }
    #[doc = "desc ICGA3C."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Icga3c(pub u32);
    impl Icga3c {
        #[doc = "desc FNWPRT0."]
        #[must_use]
        #[inline(always)]
        pub const fn fnwprt0(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "desc FNWPRT0."]
        #[inline(always)]
        pub const fn set_fnwprt0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "desc FNWPRT1."]
        #[must_use]
        #[inline(always)]
        pub const fn fnwprt1(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "desc FNWPRT1."]
        #[inline(always)]
        pub const fn set_fnwprt1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "desc FNWPRT2."]
        #[must_use]
        #[inline(always)]
        pub const fn fnwprt2(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "desc FNWPRT2."]
        #[inline(always)]
        pub const fn set_fnwprt2(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "desc FNWPRT3."]
        #[must_use]
        #[inline(always)]
        pub const fn fnwprt3(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "desc FNWPRT3."]
        #[inline(always)]
        pub const fn set_fnwprt3(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "desc FNWPRT4."]
        #[must_use]
        #[inline(always)]
        pub const fn fnwprt4(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "desc FNWPRT4."]
        #[inline(always)]
        pub const fn set_fnwprt4(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "desc FNWPRT5."]
        #[must_use]
        #[inline(always)]
        pub const fn fnwprt5(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "desc FNWPRT5."]
        #[inline(always)]
        pub const fn set_fnwprt5(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "desc FNWPRT6."]
        #[must_use]
        #[inline(always)]
        pub const fn fnwprt6(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "desc FNWPRT6."]
        #[inline(always)]
        pub const fn set_fnwprt6(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "desc FNWPRT7."]
        #[must_use]
        #[inline(always)]
        pub const fn fnwprt7(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "desc FNWPRT7."]
        #[inline(always)]
        pub const fn set_fnwprt7(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "desc FNWPRT8."]
        #[must_use]
        #[inline(always)]
        pub const fn fnwprt8(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "desc FNWPRT8."]
        #[inline(always)]
        pub const fn set_fnwprt8(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "desc FNWPRT9."]
        #[must_use]
        #[inline(always)]
        pub const fn fnwprt9(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "desc FNWPRT9."]
        #[inline(always)]
        pub const fn set_fnwprt9(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "desc FNWPRT10."]
        #[must_use]
        #[inline(always)]
        pub const fn fnwprt10(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "desc FNWPRT10."]
        #[inline(always)]
        pub const fn set_fnwprt10(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[doc = "desc FNWPRT11."]
        #[must_use]
        #[inline(always)]
        pub const fn fnwprt11(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "desc FNWPRT11."]
        #[inline(always)]
        pub const fn set_fnwprt11(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[doc = "desc FNWPRT12."]
        #[must_use]
        #[inline(always)]
        pub const fn fnwprt12(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "desc FNWPRT12."]
        #[inline(always)]
        pub const fn set_fnwprt12(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "desc FNWPRT13."]
        #[must_use]
        #[inline(always)]
        pub const fn fnwprt13(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[doc = "desc FNWPRT13."]
        #[inline(always)]
        pub const fn set_fnwprt13(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[doc = "desc FNWPRT14."]
        #[must_use]
        #[inline(always)]
        pub const fn fnwprt14(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "desc FNWPRT14."]
        #[inline(always)]
        pub const fn set_fnwprt14(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[doc = "desc FNWPRT15."]
        #[must_use]
        #[inline(always)]
        pub const fn fnwprt15(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "desc FNWPRT15."]
        #[inline(always)]
        pub const fn set_fnwprt15(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
        #[doc = "desc FNWPRT16."]
        #[must_use]
        #[inline(always)]
        pub const fn fnwprt16(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "desc FNWPRT16."]
        #[inline(always)]
        pub const fn set_fnwprt16(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "desc FNWPRT17."]
        #[must_use]
        #[inline(always)]
        pub const fn fnwprt17(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "desc FNWPRT17."]
        #[inline(always)]
        pub const fn set_fnwprt17(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[doc = "desc FNWPRT18."]
        #[must_use]
        #[inline(always)]
        pub const fn fnwprt18(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[doc = "desc FNWPRT18."]
        #[inline(always)]
        pub const fn set_fnwprt18(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
        #[doc = "desc FNWPRT19."]
        #[must_use]
        #[inline(always)]
        pub const fn fnwprt19(&self) -> bool {
            let val = (self.0 >> 19usize) & 0x01;
            val != 0
        }
        #[doc = "desc FNWPRT19."]
        #[inline(always)]
        pub const fn set_fnwprt19(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
        }
        #[doc = "desc FNWPRT20."]
        #[must_use]
        #[inline(always)]
        pub const fn fnwprt20(&self) -> bool {
            let val = (self.0 >> 20usize) & 0x01;
            val != 0
        }
        #[doc = "desc FNWPRT20."]
        #[inline(always)]
        pub const fn set_fnwprt20(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
        }
        #[doc = "desc FNWPRT21."]
        #[must_use]
        #[inline(always)]
        pub const fn fnwprt21(&self) -> bool {
            let val = (self.0 >> 21usize) & 0x01;
            val != 0
        }
        #[doc = "desc FNWPRT21."]
        #[inline(always)]
        pub const fn set_fnwprt21(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
        }
        #[doc = "desc FNWPRT22."]
        #[must_use]
        #[inline(always)]
        pub const fn fnwprt22(&self) -> bool {
            let val = (self.0 >> 22usize) & 0x01;
            val != 0
        }
        #[doc = "desc FNWPRT22."]
        #[inline(always)]
        pub const fn set_fnwprt22(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
        }
        #[doc = "desc FNWPRT23."]
        #[must_use]
        #[inline(always)]
        pub const fn fnwprt23(&self) -> bool {
            let val = (self.0 >> 23usize) & 0x01;
            val != 0
        }
        #[doc = "desc FNWPRT23."]
        #[inline(always)]
        pub const fn set_fnwprt23(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
        }
        #[doc = "desc FNWPRT24."]
        #[must_use]
        #[inline(always)]
        pub const fn fnwprt24(&self) -> bool {
            let val = (self.0 >> 24usize) & 0x01;
            val != 0
        }
        #[doc = "desc FNWPRT24."]
        #[inline(always)]
        pub const fn set_fnwprt24(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
        }
        #[doc = "desc FNWPRT25."]
        #[must_use]
        #[inline(always)]
        pub const fn fnwprt25(&self) -> bool {
            let val = (self.0 >> 25usize) & 0x01;
            val != 0
        }
        #[doc = "desc FNWPRT25."]
        #[inline(always)]
        pub const fn set_fnwprt25(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
        }
        #[doc = "desc FNWPRT26."]
        #[must_use]
        #[inline(always)]
        pub const fn fnwprt26(&self) -> bool {
            let val = (self.0 >> 26usize) & 0x01;
            val != 0
        }
        #[doc = "desc FNWPRT26."]
        #[inline(always)]
        pub const fn set_fnwprt26(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
        }
        #[doc = "desc FNWPRT27."]
        #[must_use]
        #[inline(always)]
        pub const fn fnwprt27(&self) -> bool {
            let val = (self.0 >> 27usize) & 0x01;
            val != 0
        }
        #[doc = "desc FNWPRT27."]
        #[inline(always)]
        pub const fn set_fnwprt27(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
        }
        #[doc = "desc FNWPRT28."]
        #[must_use]
        #[inline(always)]
        pub const fn fnwprt28(&self) -> bool {
            let val = (self.0 >> 28usize) & 0x01;
            val != 0
        }
        #[doc = "desc FNWPRT28."]
        #[inline(always)]
        pub const fn set_fnwprt28(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
        }
        #[doc = "desc FNWPRT29."]
        #[must_use]
        #[inline(always)]
        pub const fn fnwprt29(&self) -> bool {
            let val = (self.0 >> 29usize) & 0x01;
            val != 0
        }
        #[doc = "desc FNWPRT29."]
        #[inline(always)]
        pub const fn set_fnwprt29(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
        }
        #[doc = "desc FNWPRT30."]
        #[must_use]
        #[inline(always)]
        pub const fn fnwprt30(&self) -> bool {
            let val = (self.0 >> 30usize) & 0x01;
            val != 0
        }
        #[doc = "desc FNWPRT30."]
        #[inline(always)]
        pub const fn set_fnwprt30(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
        }
        #[doc = "desc FNWPRT31."]
        #[must_use]
        #[inline(always)]
        pub const fn fnwprt31(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[doc = "desc FNWPRT31."]
        #[inline(always)]
        pub const fn set_fnwprt31(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for Icga3c {
        #[inline(always)]
        fn default() -> Icga3c {
            Icga3c(0)
        }
    }
    impl core::fmt::Debug for Icga3c {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Icga3c")
                .field("fnwprt0", &self.fnwprt0())
                .field("fnwprt1", &self.fnwprt1())
                .field("fnwprt2", &self.fnwprt2())
                .field("fnwprt3", &self.fnwprt3())
                .field("fnwprt4", &self.fnwprt4())
                .field("fnwprt5", &self.fnwprt5())
                .field("fnwprt6", &self.fnwprt6())
                .field("fnwprt7", &self.fnwprt7())
                .field("fnwprt8", &self.fnwprt8())
                .field("fnwprt9", &self.fnwprt9())
                .field("fnwprt10", &self.fnwprt10())
                .field("fnwprt11", &self.fnwprt11())
                .field("fnwprt12", &self.fnwprt12())
                .field("fnwprt13", &self.fnwprt13())
                .field("fnwprt14", &self.fnwprt14())
                .field("fnwprt15", &self.fnwprt15())
                .field("fnwprt16", &self.fnwprt16())
                .field("fnwprt17", &self.fnwprt17())
                .field("fnwprt18", &self.fnwprt18())
                .field("fnwprt19", &self.fnwprt19())
                .field("fnwprt20", &self.fnwprt20())
                .field("fnwprt21", &self.fnwprt21())
                .field("fnwprt22", &self.fnwprt22())
                .field("fnwprt23", &self.fnwprt23())
                .field("fnwprt24", &self.fnwprt24())
                .field("fnwprt25", &self.fnwprt25())
                .field("fnwprt26", &self.fnwprt26())
                .field("fnwprt27", &self.fnwprt27())
                .field("fnwprt28", &self.fnwprt28())
                .field("fnwprt29", &self.fnwprt29())
                .field("fnwprt30", &self.fnwprt30())
                .field("fnwprt31", &self.fnwprt31())
                .finish()
        }
    }
    #[doc = "desc ICGA4A."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Icga4a(pub u32);
    impl Icga4a {
        #[doc = "desc FNWPRT32."]
        #[must_use]
        #[inline(always)]
        pub const fn fnwprt32(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "desc FNWPRT32."]
        #[inline(always)]
        pub const fn set_fnwprt32(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "desc FNWPRT33."]
        #[must_use]
        #[inline(always)]
        pub const fn fnwprt33(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "desc FNWPRT33."]
        #[inline(always)]
        pub const fn set_fnwprt33(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "desc FNWPRT34."]
        #[must_use]
        #[inline(always)]
        pub const fn fnwprt34(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "desc FNWPRT34."]
        #[inline(always)]
        pub const fn set_fnwprt34(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "desc FNWPRT35."]
        #[must_use]
        #[inline(always)]
        pub const fn fnwprt35(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "desc FNWPRT35."]
        #[inline(always)]
        pub const fn set_fnwprt35(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "desc FNWPRT36."]
        #[must_use]
        #[inline(always)]
        pub const fn fnwprt36(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "desc FNWPRT36."]
        #[inline(always)]
        pub const fn set_fnwprt36(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "desc FNWPRT37."]
        #[must_use]
        #[inline(always)]
        pub const fn fnwprt37(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "desc FNWPRT37."]
        #[inline(always)]
        pub const fn set_fnwprt37(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "desc FNWPRT38."]
        #[must_use]
        #[inline(always)]
        pub const fn fnwprt38(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "desc FNWPRT38."]
        #[inline(always)]
        pub const fn set_fnwprt38(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "desc FNWPRT39."]
        #[must_use]
        #[inline(always)]
        pub const fn fnwprt39(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "desc FNWPRT39."]
        #[inline(always)]
        pub const fn set_fnwprt39(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "desc FNWPRT40."]
        #[must_use]
        #[inline(always)]
        pub const fn fnwprt40(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "desc FNWPRT40."]
        #[inline(always)]
        pub const fn set_fnwprt40(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "desc FNWPRT41."]
        #[must_use]
        #[inline(always)]
        pub const fn fnwprt41(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "desc FNWPRT41."]
        #[inline(always)]
        pub const fn set_fnwprt41(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "desc FNWPRT42."]
        #[must_use]
        #[inline(always)]
        pub const fn fnwprt42(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "desc FNWPRT42."]
        #[inline(always)]
        pub const fn set_fnwprt42(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[doc = "desc FNWPRT43."]
        #[must_use]
        #[inline(always)]
        pub const fn fnwprt43(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "desc FNWPRT43."]
        #[inline(always)]
        pub const fn set_fnwprt43(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[doc = "desc FNWPRT44."]
        #[must_use]
        #[inline(always)]
        pub const fn fnwprt44(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "desc FNWPRT44."]
        #[inline(always)]
        pub const fn set_fnwprt44(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "desc FNWPRT45."]
        #[must_use]
        #[inline(always)]
        pub const fn fnwprt45(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[doc = "desc FNWPRT45."]
        #[inline(always)]
        pub const fn set_fnwprt45(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[doc = "desc FNWPRT46."]
        #[must_use]
        #[inline(always)]
        pub const fn fnwprt46(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "desc FNWPRT46."]
        #[inline(always)]
        pub const fn set_fnwprt46(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[doc = "desc FNWPRT47."]
        #[must_use]
        #[inline(always)]
        pub const fn fnwprt47(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "desc FNWPRT47."]
        #[inline(always)]
        pub const fn set_fnwprt47(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
        #[doc = "desc FNWPRT48."]
        #[must_use]
        #[inline(always)]
        pub const fn fnwprt48(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "desc FNWPRT48."]
        #[inline(always)]
        pub const fn set_fnwprt48(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "desc FNWPRT49."]
        #[must_use]
        #[inline(always)]
        pub const fn fnwprt49(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "desc FNWPRT49."]
        #[inline(always)]
        pub const fn set_fnwprt49(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[doc = "desc FNWPRT50."]
        #[must_use]
        #[inline(always)]
        pub const fn fnwprt50(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[doc = "desc FNWPRT50."]
        #[inline(always)]
        pub const fn set_fnwprt50(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
        #[doc = "desc FNWPRT51."]
        #[must_use]
        #[inline(always)]
        pub const fn fnwprt51(&self) -> bool {
            let val = (self.0 >> 19usize) & 0x01;
            val != 0
        }
        #[doc = "desc FNWPRT51."]
        #[inline(always)]
        pub const fn set_fnwprt51(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
        }
        #[doc = "desc FNWPRT52."]
        #[must_use]
        #[inline(always)]
        pub const fn fnwprt52(&self) -> bool {
            let val = (self.0 >> 20usize) & 0x01;
            val != 0
        }
        #[doc = "desc FNWPRT52."]
        #[inline(always)]
        pub const fn set_fnwprt52(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
        }
        #[doc = "desc FNWPRT53."]
        #[must_use]
        #[inline(always)]
        pub const fn fnwprt53(&self) -> bool {
            let val = (self.0 >> 21usize) & 0x01;
            val != 0
        }
        #[doc = "desc FNWPRT53."]
        #[inline(always)]
        pub const fn set_fnwprt53(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
        }
        #[doc = "desc FNWPRT54."]
        #[must_use]
        #[inline(always)]
        pub const fn fnwprt54(&self) -> bool {
            let val = (self.0 >> 22usize) & 0x01;
            val != 0
        }
        #[doc = "desc FNWPRT54."]
        #[inline(always)]
        pub const fn set_fnwprt54(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
        }
        #[doc = "desc FNWPRT55."]
        #[must_use]
        #[inline(always)]
        pub const fn fnwprt55(&self) -> bool {
            let val = (self.0 >> 23usize) & 0x01;
            val != 0
        }
        #[doc = "desc FNWPRT55."]
        #[inline(always)]
        pub const fn set_fnwprt55(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
        }
        #[doc = "desc FNWPRT56."]
        #[must_use]
        #[inline(always)]
        pub const fn fnwprt56(&self) -> bool {
            let val = (self.0 >> 24usize) & 0x01;
            val != 0
        }
        #[doc = "desc FNWPRT56."]
        #[inline(always)]
        pub const fn set_fnwprt56(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
        }
        #[doc = "desc FNWPRT57."]
        #[must_use]
        #[inline(always)]
        pub const fn fnwprt57(&self) -> bool {
            let val = (self.0 >> 25usize) & 0x01;
            val != 0
        }
        #[doc = "desc FNWPRT57."]
        #[inline(always)]
        pub const fn set_fnwprt57(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
        }
        #[doc = "desc FNWPRT58."]
        #[must_use]
        #[inline(always)]
        pub const fn fnwprt58(&self) -> bool {
            let val = (self.0 >> 26usize) & 0x01;
            val != 0
        }
        #[doc = "desc FNWPRT58."]
        #[inline(always)]
        pub const fn set_fnwprt58(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
        }
        #[doc = "desc FNWPRT59."]
        #[must_use]
        #[inline(always)]
        pub const fn fnwprt59(&self) -> bool {
            let val = (self.0 >> 27usize) & 0x01;
            val != 0
        }
        #[doc = "desc FNWPRT59."]
        #[inline(always)]
        pub const fn set_fnwprt59(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
        }
        #[doc = "desc FNWPRT60."]
        #[must_use]
        #[inline(always)]
        pub const fn fnwprt60(&self) -> bool {
            let val = (self.0 >> 28usize) & 0x01;
            val != 0
        }
        #[doc = "desc FNWPRT60."]
        #[inline(always)]
        pub const fn set_fnwprt60(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
        }
        #[doc = "desc FNWPRT61."]
        #[must_use]
        #[inline(always)]
        pub const fn fnwprt61(&self) -> bool {
            let val = (self.0 >> 29usize) & 0x01;
            val != 0
        }
        #[doc = "desc FNWPRT61."]
        #[inline(always)]
        pub const fn set_fnwprt61(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
        }
        #[doc = "desc FNWPRT62."]
        #[must_use]
        #[inline(always)]
        pub const fn fnwprt62(&self) -> bool {
            let val = (self.0 >> 30usize) & 0x01;
            val != 0
        }
        #[doc = "desc FNWPRT62."]
        #[inline(always)]
        pub const fn set_fnwprt62(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
        }
        #[doc = "desc FNWPRT63."]
        #[must_use]
        #[inline(always)]
        pub const fn fnwprt63(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[doc = "desc FNWPRT63."]
        #[inline(always)]
        pub const fn set_fnwprt63(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for Icga4a {
        #[inline(always)]
        fn default() -> Icga4a {
            Icga4a(0)
        }
    }
    impl core::fmt::Debug for Icga4a {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Icga4a")
                .field("fnwprt32", &self.fnwprt32())
                .field("fnwprt33", &self.fnwprt33())
                .field("fnwprt34", &self.fnwprt34())
                .field("fnwprt35", &self.fnwprt35())
                .field("fnwprt36", &self.fnwprt36())
                .field("fnwprt37", &self.fnwprt37())
                .field("fnwprt38", &self.fnwprt38())
                .field("fnwprt39", &self.fnwprt39())
                .field("fnwprt40", &self.fnwprt40())
                .field("fnwprt41", &self.fnwprt41())
                .field("fnwprt42", &self.fnwprt42())
                .field("fnwprt43", &self.fnwprt43())
                .field("fnwprt44", &self.fnwprt44())
                .field("fnwprt45", &self.fnwprt45())
                .field("fnwprt46", &self.fnwprt46())
                .field("fnwprt47", &self.fnwprt47())
                .field("fnwprt48", &self.fnwprt48())
                .field("fnwprt49", &self.fnwprt49())
                .field("fnwprt50", &self.fnwprt50())
                .field("fnwprt51", &self.fnwprt51())
                .field("fnwprt52", &self.fnwprt52())
                .field("fnwprt53", &self.fnwprt53())
                .field("fnwprt54", &self.fnwprt54())
                .field("fnwprt55", &self.fnwprt55())
                .field("fnwprt56", &self.fnwprt56())
                .field("fnwprt57", &self.fnwprt57())
                .field("fnwprt58", &self.fnwprt58())
                .field("fnwprt59", &self.fnwprt59())
                .field("fnwprt60", &self.fnwprt60())
                .field("fnwprt61", &self.fnwprt61())
                .field("fnwprt62", &self.fnwprt62())
                .field("fnwprt63", &self.fnwprt63())
                .finish()
        }
    }
    #[doc = "desc ICGA4B."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Icga4b(pub u32);
    impl Icga4b {
        #[doc = "desc FNWPRT32."]
        #[must_use]
        #[inline(always)]
        pub const fn fnwprt32(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "desc FNWPRT32."]
        #[inline(always)]
        pub const fn set_fnwprt32(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "desc FNWPRT33."]
        #[must_use]
        #[inline(always)]
        pub const fn fnwprt33(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "desc FNWPRT33."]
        #[inline(always)]
        pub const fn set_fnwprt33(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "desc FNWPRT34."]
        #[must_use]
        #[inline(always)]
        pub const fn fnwprt34(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "desc FNWPRT34."]
        #[inline(always)]
        pub const fn set_fnwprt34(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "desc FNWPRT35."]
        #[must_use]
        #[inline(always)]
        pub const fn fnwprt35(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "desc FNWPRT35."]
        #[inline(always)]
        pub const fn set_fnwprt35(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "desc FNWPRT36."]
        #[must_use]
        #[inline(always)]
        pub const fn fnwprt36(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "desc FNWPRT36."]
        #[inline(always)]
        pub const fn set_fnwprt36(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "desc FNWPRT37."]
        #[must_use]
        #[inline(always)]
        pub const fn fnwprt37(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "desc FNWPRT37."]
        #[inline(always)]
        pub const fn set_fnwprt37(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "desc FNWPRT38."]
        #[must_use]
        #[inline(always)]
        pub const fn fnwprt38(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "desc FNWPRT38."]
        #[inline(always)]
        pub const fn set_fnwprt38(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "desc FNWPRT39."]
        #[must_use]
        #[inline(always)]
        pub const fn fnwprt39(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "desc FNWPRT39."]
        #[inline(always)]
        pub const fn set_fnwprt39(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "desc FNWPRT40."]
        #[must_use]
        #[inline(always)]
        pub const fn fnwprt40(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "desc FNWPRT40."]
        #[inline(always)]
        pub const fn set_fnwprt40(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "desc FNWPRT41."]
        #[must_use]
        #[inline(always)]
        pub const fn fnwprt41(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "desc FNWPRT41."]
        #[inline(always)]
        pub const fn set_fnwprt41(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "desc FNWPRT42."]
        #[must_use]
        #[inline(always)]
        pub const fn fnwprt42(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "desc FNWPRT42."]
        #[inline(always)]
        pub const fn set_fnwprt42(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[doc = "desc FNWPRT43."]
        #[must_use]
        #[inline(always)]
        pub const fn fnwprt43(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "desc FNWPRT43."]
        #[inline(always)]
        pub const fn set_fnwprt43(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[doc = "desc FNWPRT44."]
        #[must_use]
        #[inline(always)]
        pub const fn fnwprt44(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "desc FNWPRT44."]
        #[inline(always)]
        pub const fn set_fnwprt44(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "desc FNWPRT45."]
        #[must_use]
        #[inline(always)]
        pub const fn fnwprt45(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[doc = "desc FNWPRT45."]
        #[inline(always)]
        pub const fn set_fnwprt45(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[doc = "desc FNWPRT46."]
        #[must_use]
        #[inline(always)]
        pub const fn fnwprt46(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "desc FNWPRT46."]
        #[inline(always)]
        pub const fn set_fnwprt46(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[doc = "desc FNWPRT47."]
        #[must_use]
        #[inline(always)]
        pub const fn fnwprt47(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "desc FNWPRT47."]
        #[inline(always)]
        pub const fn set_fnwprt47(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
        #[doc = "desc FNWPRT48."]
        #[must_use]
        #[inline(always)]
        pub const fn fnwprt48(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "desc FNWPRT48."]
        #[inline(always)]
        pub const fn set_fnwprt48(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "desc FNWPRT49."]
        #[must_use]
        #[inline(always)]
        pub const fn fnwprt49(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "desc FNWPRT49."]
        #[inline(always)]
        pub const fn set_fnwprt49(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[doc = "desc FNWPRT50."]
        #[must_use]
        #[inline(always)]
        pub const fn fnwprt50(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[doc = "desc FNWPRT50."]
        #[inline(always)]
        pub const fn set_fnwprt50(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
        #[doc = "desc FNWPRT51."]
        #[must_use]
        #[inline(always)]
        pub const fn fnwprt51(&self) -> bool {
            let val = (self.0 >> 19usize) & 0x01;
            val != 0
        }
        #[doc = "desc FNWPRT51."]
        #[inline(always)]
        pub const fn set_fnwprt51(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
        }
        #[doc = "desc FNWPRT52."]
        #[must_use]
        #[inline(always)]
        pub const fn fnwprt52(&self) -> bool {
            let val = (self.0 >> 20usize) & 0x01;
            val != 0
        }
        #[doc = "desc FNWPRT52."]
        #[inline(always)]
        pub const fn set_fnwprt52(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
        }
        #[doc = "desc FNWPRT53."]
        #[must_use]
        #[inline(always)]
        pub const fn fnwprt53(&self) -> bool {
            let val = (self.0 >> 21usize) & 0x01;
            val != 0
        }
        #[doc = "desc FNWPRT53."]
        #[inline(always)]
        pub const fn set_fnwprt53(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
        }
        #[doc = "desc FNWPRT54."]
        #[must_use]
        #[inline(always)]
        pub const fn fnwprt54(&self) -> bool {
            let val = (self.0 >> 22usize) & 0x01;
            val != 0
        }
        #[doc = "desc FNWPRT54."]
        #[inline(always)]
        pub const fn set_fnwprt54(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
        }
        #[doc = "desc FNWPRT55."]
        #[must_use]
        #[inline(always)]
        pub const fn fnwprt55(&self) -> bool {
            let val = (self.0 >> 23usize) & 0x01;
            val != 0
        }
        #[doc = "desc FNWPRT55."]
        #[inline(always)]
        pub const fn set_fnwprt55(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
        }
        #[doc = "desc FNWPRT56."]
        #[must_use]
        #[inline(always)]
        pub const fn fnwprt56(&self) -> bool {
            let val = (self.0 >> 24usize) & 0x01;
            val != 0
        }
        #[doc = "desc FNWPRT56."]
        #[inline(always)]
        pub const fn set_fnwprt56(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
        }
        #[doc = "desc FNWPRT57."]
        #[must_use]
        #[inline(always)]
        pub const fn fnwprt57(&self) -> bool {
            let val = (self.0 >> 25usize) & 0x01;
            val != 0
        }
        #[doc = "desc FNWPRT57."]
        #[inline(always)]
        pub const fn set_fnwprt57(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
        }
        #[doc = "desc FNWPRT58."]
        #[must_use]
        #[inline(always)]
        pub const fn fnwprt58(&self) -> bool {
            let val = (self.0 >> 26usize) & 0x01;
            val != 0
        }
        #[doc = "desc FNWPRT58."]
        #[inline(always)]
        pub const fn set_fnwprt58(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
        }
        #[doc = "desc FNWPRT59."]
        #[must_use]
        #[inline(always)]
        pub const fn fnwprt59(&self) -> bool {
            let val = (self.0 >> 27usize) & 0x01;
            val != 0
        }
        #[doc = "desc FNWPRT59."]
        #[inline(always)]
        pub const fn set_fnwprt59(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
        }
        #[doc = "desc FNWPRT60."]
        #[must_use]
        #[inline(always)]
        pub const fn fnwprt60(&self) -> bool {
            let val = (self.0 >> 28usize) & 0x01;
            val != 0
        }
        #[doc = "desc FNWPRT60."]
        #[inline(always)]
        pub const fn set_fnwprt60(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
        }
        #[doc = "desc FNWPRT61."]
        #[must_use]
        #[inline(always)]
        pub const fn fnwprt61(&self) -> bool {
            let val = (self.0 >> 29usize) & 0x01;
            val != 0
        }
        #[doc = "desc FNWPRT61."]
        #[inline(always)]
        pub const fn set_fnwprt61(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
        }
        #[doc = "desc FNWPRT62."]
        #[must_use]
        #[inline(always)]
        pub const fn fnwprt62(&self) -> bool {
            let val = (self.0 >> 30usize) & 0x01;
            val != 0
        }
        #[doc = "desc FNWPRT62."]
        #[inline(always)]
        pub const fn set_fnwprt62(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
        }
        #[doc = "desc FNWPRT63."]
        #[must_use]
        #[inline(always)]
        pub const fn fnwprt63(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[doc = "desc FNWPRT63."]
        #[inline(always)]
        pub const fn set_fnwprt63(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for Icga4b {
        #[inline(always)]
        fn default() -> Icga4b {
            Icga4b(0)
        }
    }
    impl core::fmt::Debug for Icga4b {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Icga4b")
                .field("fnwprt32", &self.fnwprt32())
                .field("fnwprt33", &self.fnwprt33())
                .field("fnwprt34", &self.fnwprt34())
                .field("fnwprt35", &self.fnwprt35())
                .field("fnwprt36", &self.fnwprt36())
                .field("fnwprt37", &self.fnwprt37())
                .field("fnwprt38", &self.fnwprt38())
                .field("fnwprt39", &self.fnwprt39())
                .field("fnwprt40", &self.fnwprt40())
                .field("fnwprt41", &self.fnwprt41())
                .field("fnwprt42", &self.fnwprt42())
                .field("fnwprt43", &self.fnwprt43())
                .field("fnwprt44", &self.fnwprt44())
                .field("fnwprt45", &self.fnwprt45())
                .field("fnwprt46", &self.fnwprt46())
                .field("fnwprt47", &self.fnwprt47())
                .field("fnwprt48", &self.fnwprt48())
                .field("fnwprt49", &self.fnwprt49())
                .field("fnwprt50", &self.fnwprt50())
                .field("fnwprt51", &self.fnwprt51())
                .field("fnwprt52", &self.fnwprt52())
                .field("fnwprt53", &self.fnwprt53())
                .field("fnwprt54", &self.fnwprt54())
                .field("fnwprt55", &self.fnwprt55())
                .field("fnwprt56", &self.fnwprt56())
                .field("fnwprt57", &self.fnwprt57())
                .field("fnwprt58", &self.fnwprt58())
                .field("fnwprt59", &self.fnwprt59())
                .field("fnwprt60", &self.fnwprt60())
                .field("fnwprt61", &self.fnwprt61())
                .field("fnwprt62", &self.fnwprt62())
                .field("fnwprt63", &self.fnwprt63())
                .finish()
        }
    }
    #[doc = "desc ICGA4C."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Icga4c(pub u32);
    impl Icga4c {
        #[doc = "desc FNWPRT32."]
        #[must_use]
        #[inline(always)]
        pub const fn fnwprt32(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "desc FNWPRT32."]
        #[inline(always)]
        pub const fn set_fnwprt32(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "desc FNWPRT33."]
        #[must_use]
        #[inline(always)]
        pub const fn fnwprt33(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "desc FNWPRT33."]
        #[inline(always)]
        pub const fn set_fnwprt33(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "desc FNWPRT34."]
        #[must_use]
        #[inline(always)]
        pub const fn fnwprt34(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "desc FNWPRT34."]
        #[inline(always)]
        pub const fn set_fnwprt34(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "desc FNWPRT35."]
        #[must_use]
        #[inline(always)]
        pub const fn fnwprt35(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "desc FNWPRT35."]
        #[inline(always)]
        pub const fn set_fnwprt35(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "desc FNWPRT36."]
        #[must_use]
        #[inline(always)]
        pub const fn fnwprt36(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "desc FNWPRT36."]
        #[inline(always)]
        pub const fn set_fnwprt36(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "desc FNWPRT37."]
        #[must_use]
        #[inline(always)]
        pub const fn fnwprt37(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "desc FNWPRT37."]
        #[inline(always)]
        pub const fn set_fnwprt37(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "desc FNWPRT38."]
        #[must_use]
        #[inline(always)]
        pub const fn fnwprt38(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "desc FNWPRT38."]
        #[inline(always)]
        pub const fn set_fnwprt38(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "desc FNWPRT39."]
        #[must_use]
        #[inline(always)]
        pub const fn fnwprt39(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "desc FNWPRT39."]
        #[inline(always)]
        pub const fn set_fnwprt39(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "desc FNWPRT40."]
        #[must_use]
        #[inline(always)]
        pub const fn fnwprt40(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "desc FNWPRT40."]
        #[inline(always)]
        pub const fn set_fnwprt40(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "desc FNWPRT41."]
        #[must_use]
        #[inline(always)]
        pub const fn fnwprt41(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "desc FNWPRT41."]
        #[inline(always)]
        pub const fn set_fnwprt41(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "desc FNWPRT42."]
        #[must_use]
        #[inline(always)]
        pub const fn fnwprt42(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "desc FNWPRT42."]
        #[inline(always)]
        pub const fn set_fnwprt42(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[doc = "desc FNWPRT43."]
        #[must_use]
        #[inline(always)]
        pub const fn fnwprt43(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "desc FNWPRT43."]
        #[inline(always)]
        pub const fn set_fnwprt43(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[doc = "desc FNWPRT44."]
        #[must_use]
        #[inline(always)]
        pub const fn fnwprt44(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "desc FNWPRT44."]
        #[inline(always)]
        pub const fn set_fnwprt44(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "desc FNWPRT45."]
        #[must_use]
        #[inline(always)]
        pub const fn fnwprt45(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[doc = "desc FNWPRT45."]
        #[inline(always)]
        pub const fn set_fnwprt45(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[doc = "desc FNWPRT46."]
        #[must_use]
        #[inline(always)]
        pub const fn fnwprt46(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "desc FNWPRT46."]
        #[inline(always)]
        pub const fn set_fnwprt46(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[doc = "desc FNWPRT47."]
        #[must_use]
        #[inline(always)]
        pub const fn fnwprt47(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "desc FNWPRT47."]
        #[inline(always)]
        pub const fn set_fnwprt47(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
        #[doc = "desc FNWPRT48."]
        #[must_use]
        #[inline(always)]
        pub const fn fnwprt48(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "desc FNWPRT48."]
        #[inline(always)]
        pub const fn set_fnwprt48(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "desc FNWPRT49."]
        #[must_use]
        #[inline(always)]
        pub const fn fnwprt49(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "desc FNWPRT49."]
        #[inline(always)]
        pub const fn set_fnwprt49(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[doc = "desc FNWPRT50."]
        #[must_use]
        #[inline(always)]
        pub const fn fnwprt50(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[doc = "desc FNWPRT50."]
        #[inline(always)]
        pub const fn set_fnwprt50(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
        #[doc = "desc FNWPRT51."]
        #[must_use]
        #[inline(always)]
        pub const fn fnwprt51(&self) -> bool {
            let val = (self.0 >> 19usize) & 0x01;
            val != 0
        }
        #[doc = "desc FNWPRT51."]
        #[inline(always)]
        pub const fn set_fnwprt51(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
        }
        #[doc = "desc FNWPRT52."]
        #[must_use]
        #[inline(always)]
        pub const fn fnwprt52(&self) -> bool {
            let val = (self.0 >> 20usize) & 0x01;
            val != 0
        }
        #[doc = "desc FNWPRT52."]
        #[inline(always)]
        pub const fn set_fnwprt52(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
        }
        #[doc = "desc FNWPRT53."]
        #[must_use]
        #[inline(always)]
        pub const fn fnwprt53(&self) -> bool {
            let val = (self.0 >> 21usize) & 0x01;
            val != 0
        }
        #[doc = "desc FNWPRT53."]
        #[inline(always)]
        pub const fn set_fnwprt53(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
        }
        #[doc = "desc FNWPRT54."]
        #[must_use]
        #[inline(always)]
        pub const fn fnwprt54(&self) -> bool {
            let val = (self.0 >> 22usize) & 0x01;
            val != 0
        }
        #[doc = "desc FNWPRT54."]
        #[inline(always)]
        pub const fn set_fnwprt54(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
        }
        #[doc = "desc FNWPRT55."]
        #[must_use]
        #[inline(always)]
        pub const fn fnwprt55(&self) -> bool {
            let val = (self.0 >> 23usize) & 0x01;
            val != 0
        }
        #[doc = "desc FNWPRT55."]
        #[inline(always)]
        pub const fn set_fnwprt55(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
        }
        #[doc = "desc FNWPRT56."]
        #[must_use]
        #[inline(always)]
        pub const fn fnwprt56(&self) -> bool {
            let val = (self.0 >> 24usize) & 0x01;
            val != 0
        }
        #[doc = "desc FNWPRT56."]
        #[inline(always)]
        pub const fn set_fnwprt56(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
        }
        #[doc = "desc FNWPRT57."]
        #[must_use]
        #[inline(always)]
        pub const fn fnwprt57(&self) -> bool {
            let val = (self.0 >> 25usize) & 0x01;
            val != 0
        }
        #[doc = "desc FNWPRT57."]
        #[inline(always)]
        pub const fn set_fnwprt57(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
        }
        #[doc = "desc FNWPRT58."]
        #[must_use]
        #[inline(always)]
        pub const fn fnwprt58(&self) -> bool {
            let val = (self.0 >> 26usize) & 0x01;
            val != 0
        }
        #[doc = "desc FNWPRT58."]
        #[inline(always)]
        pub const fn set_fnwprt58(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
        }
        #[doc = "desc FNWPRT59."]
        #[must_use]
        #[inline(always)]
        pub const fn fnwprt59(&self) -> bool {
            let val = (self.0 >> 27usize) & 0x01;
            val != 0
        }
        #[doc = "desc FNWPRT59."]
        #[inline(always)]
        pub const fn set_fnwprt59(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
        }
        #[doc = "desc FNWPRT60."]
        #[must_use]
        #[inline(always)]
        pub const fn fnwprt60(&self) -> bool {
            let val = (self.0 >> 28usize) & 0x01;
            val != 0
        }
        #[doc = "desc FNWPRT60."]
        #[inline(always)]
        pub const fn set_fnwprt60(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
        }
        #[doc = "desc FNWPRT61."]
        #[must_use]
        #[inline(always)]
        pub const fn fnwprt61(&self) -> bool {
            let val = (self.0 >> 29usize) & 0x01;
            val != 0
        }
        #[doc = "desc FNWPRT61."]
        #[inline(always)]
        pub const fn set_fnwprt61(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
        }
        #[doc = "desc FNWPRT62."]
        #[must_use]
        #[inline(always)]
        pub const fn fnwprt62(&self) -> bool {
            let val = (self.0 >> 30usize) & 0x01;
            val != 0
        }
        #[doc = "desc FNWPRT62."]
        #[inline(always)]
        pub const fn set_fnwprt62(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
        }
        #[doc = "desc FNWPRT63."]
        #[must_use]
        #[inline(always)]
        pub const fn fnwprt63(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[doc = "desc FNWPRT63."]
        #[inline(always)]
        pub const fn set_fnwprt63(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for Icga4c {
        #[inline(always)]
        fn default() -> Icga4c {
            Icga4c(0)
        }
    }
    impl core::fmt::Debug for Icga4c {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Icga4c")
                .field("fnwprt32", &self.fnwprt32())
                .field("fnwprt33", &self.fnwprt33())
                .field("fnwprt34", &self.fnwprt34())
                .field("fnwprt35", &self.fnwprt35())
                .field("fnwprt36", &self.fnwprt36())
                .field("fnwprt37", &self.fnwprt37())
                .field("fnwprt38", &self.fnwprt38())
                .field("fnwprt39", &self.fnwprt39())
                .field("fnwprt40", &self.fnwprt40())
                .field("fnwprt41", &self.fnwprt41())
                .field("fnwprt42", &self.fnwprt42())
                .field("fnwprt43", &self.fnwprt43())
                .field("fnwprt44", &self.fnwprt44())
                .field("fnwprt45", &self.fnwprt45())
                .field("fnwprt46", &self.fnwprt46())
                .field("fnwprt47", &self.fnwprt47())
                .field("fnwprt48", &self.fnwprt48())
                .field("fnwprt49", &self.fnwprt49())
                .field("fnwprt50", &self.fnwprt50())
                .field("fnwprt51", &self.fnwprt51())
                .field("fnwprt52", &self.fnwprt52())
                .field("fnwprt53", &self.fnwprt53())
                .field("fnwprt54", &self.fnwprt54())
                .field("fnwprt55", &self.fnwprt55())
                .field("fnwprt56", &self.fnwprt56())
                .field("fnwprt57", &self.fnwprt57())
                .field("fnwprt58", &self.fnwprt58())
                .field("fnwprt59", &self.fnwprt59())
                .field("fnwprt60", &self.fnwprt60())
                .field("fnwprt61", &self.fnwprt61())
                .field("fnwprt62", &self.fnwprt62())
                .field("fnwprt63", &self.fnwprt63())
                .finish()
        }
    }
}
