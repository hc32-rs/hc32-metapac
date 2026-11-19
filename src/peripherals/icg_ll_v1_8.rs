#![allow(clippy::missing_safety_doc)]
#![allow(clippy::identity_op)]
#![allow(clippy::unnecessary_cast)]
#![allow(clippy::erasing_op)]

#[doc = "desc ICG."]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Icg {
    ptr: *mut u8,
}
unsafe impl Send for Icg {}
unsafe impl Sync for Icg {}
impl Icg {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "desc ICG0."]
    #[inline(always)]
    pub const fn icg0(self) -> crate::common::Reg<regs::Icg0, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0usize) as _) }
    }
    #[doc = "desc ICG1."]
    #[inline(always)]
    pub const fn icg1(self) -> crate::common::Reg<regs::Icg1, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x04usize) as _) }
    }
    #[doc = "desc ICG2."]
    #[inline(always)]
    pub const fn icg2(self) -> crate::common::Reg<u32, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x08usize) as _) }
    }
    #[doc = "desc ICG3."]
    #[inline(always)]
    pub const fn icg3(self) -> crate::common::Reg<u32, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0cusize) as _) }
    }
    #[doc = "desc ICG4."]
    #[inline(always)]
    pub const fn icg4(self) -> crate::common::Reg<u32, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x10usize) as _) }
    }
    #[doc = "desc ICG5."]
    #[inline(always)]
    pub const fn icg5(self) -> crate::common::Reg<u32, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x14usize) as _) }
    }
    #[doc = "desc ICG6."]
    #[inline(always)]
    pub const fn icg6(self) -> crate::common::Reg<u32, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x18usize) as _) }
    }
}
pub mod regs {
    #[doc = "desc ICG0."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Icg0(pub u32);
    impl Icg0 {
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
    impl Default for Icg0 {
        #[inline(always)]
        fn default() -> Icg0 {
            Icg0(0)
        }
    }
    impl core::fmt::Debug for Icg0 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Icg0")
                .field("swdtauts", &self.swdtauts())
                .field("swdtits", &self.swdtits())
                .field("swdtperi", &self.swdtperi())
                .field("swdtcks", &self.swdtcks())
                .field("swdtwdpt", &self.swdtwdpt())
                .field("swdtslpoff", &self.swdtslpoff())
                .field("hrcfreqs", &self.hrcfreqs())
                .field("nmifclk", &self.nmifclk())
                .field("nmifen", &self.nmifen())
                .field("nmitrg", &self.nmitrg())
                .field("nmien", &self.nmien())
                .field("nmiicgen", &self.nmiicgen())
                .finish()
        }
    }
    #[doc = "desc ICG1."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Icg1(pub u32);
    impl Icg1 {
        #[doc = "desc DFS."]
        #[must_use]
        #[inline(always)]
        pub const fn dfs(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x03;
            val as u8
        }
        #[doc = "desc DFS."]
        #[inline(always)]
        pub const fn set_dfs(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 0usize)) | (((val as u32) & 0x03) << 0usize);
        }
        #[doc = "desc DFDIS."]
        #[must_use]
        #[inline(always)]
        pub const fn dfdis(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "desc DFDIS."]
        #[inline(always)]
        pub const fn set_dfdis(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "desc LVDLVL."]
        #[must_use]
        #[inline(always)]
        pub const fn lvdlvl(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0x0f;
            val as u8
        }
        #[doc = "desc LVDLVL."]
        #[inline(always)]
        pub const fn set_lvdlvl(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 8usize)) | (((val as u32) & 0x0f) << 8usize);
        }
        #[doc = "desc NMIS."]
        #[must_use]
        #[inline(always)]
        pub const fn nmis(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "desc NMIS."]
        #[inline(always)]
        pub const fn set_nmis(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "desc IRS."]
        #[must_use]
        #[inline(always)]
        pub const fn irs(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[doc = "desc IRS."]
        #[inline(always)]
        pub const fn set_irs(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[doc = "desc IRDIS."]
        #[must_use]
        #[inline(always)]
        pub const fn irdis(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "desc IRDIS."]
        #[inline(always)]
        pub const fn set_irdis(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[doc = "desc LVDDIS."]
        #[must_use]
        #[inline(always)]
        pub const fn lvddis(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "desc LVDDIS."]
        #[inline(always)]
        pub const fn set_lvddis(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
        #[doc = "desc LKUPDIS."]
        #[must_use]
        #[inline(always)]
        pub const fn lkupdis(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "desc LKUPDIS."]
        #[inline(always)]
        pub const fn set_lkupdis(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
    }
    impl Default for Icg1 {
        #[inline(always)]
        fn default() -> Icg1 {
            Icg1(0)
        }
    }
    impl core::fmt::Debug for Icg1 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Icg1")
                .field("dfs", &self.dfs())
                .field("dfdis", &self.dfdis())
                .field("lvdlvl", &self.lvdlvl())
                .field("nmis", &self.nmis())
                .field("irs", &self.irs())
                .field("irdis", &self.irdis())
                .field("lvddis", &self.lvddis())
                .field("lkupdis", &self.lkupdis())
                .finish()
        }
    }
}
