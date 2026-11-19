#![allow(clippy::missing_safety_doc)]
#![allow(clippy::identity_op)]
#![allow(clippy::unnecessary_cast)]
#![allow(clippy::erasing_op)]

#[doc = "desc DAC."]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dac {
    ptr: *mut u8,
}
unsafe impl Send for Dac {}
unsafe impl Sync for Dac {}
impl Dac {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "desc CR0."]
    #[inline(always)]
    pub const fn cr0(self) -> crate::common::Reg<regs::Cr0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0usize) as _) }
    }
    #[doc = "desc SWTRIGR."]
    #[inline(always)]
    pub const fn swtrigr(self) -> crate::common::Reg<regs::Swtrigr, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x04usize) as _) }
    }
    #[doc = "desc DHR12R0."]
    #[inline(always)]
    pub const fn dhr12r0(self) -> crate::common::Reg<regs::Dhr12r0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x08usize) as _) }
    }
    #[doc = "desc DHR12L0."]
    #[inline(always)]
    pub const fn dhr12l0(self) -> crate::common::Reg<regs::Dhr12l0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0cusize) as _) }
    }
    #[doc = "desc DHR8R0."]
    #[inline(always)]
    pub const fn dhr8r0(self) -> crate::common::Reg<regs::Dhr8r0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x10usize) as _) }
    }
    #[doc = "desc DOR0."]
    #[inline(always)]
    pub const fn dor0(self) -> crate::common::Reg<regs::Dor0, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x2cusize) as _) }
    }
    #[doc = "desc SR."]
    #[inline(always)]
    pub const fn sr(self) -> crate::common::Reg<regs::Sr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x34usize) as _) }
    }
    #[doc = "desc ETRS."]
    #[inline(always)]
    pub const fn etrs(self) -> crate::common::Reg<regs::Etrs, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x38usize) as _) }
    }
}
pub mod regs {
    #[doc = "desc CR0."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Cr0(pub u32);
    impl Cr0 {
        #[doc = "desc EN0."]
        #[must_use]
        #[inline(always)]
        pub const fn en0(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "desc EN0."]
        #[inline(always)]
        pub const fn set_en0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "desc BOFF0."]
        #[must_use]
        #[inline(always)]
        pub const fn boff0(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "desc BOFF0."]
        #[inline(always)]
        pub const fn set_boff0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "desc TEN0."]
        #[must_use]
        #[inline(always)]
        pub const fn ten0(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "desc TEN0."]
        #[inline(always)]
        pub const fn set_ten0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "desc TSEL0."]
        #[must_use]
        #[inline(always)]
        pub const fn tsel0(&self) -> u8 {
            let val = (self.0 >> 3usize) & 0x07;
            val as u8
        }
        #[doc = "desc TSEL0."]
        #[inline(always)]
        pub const fn set_tsel0(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 3usize)) | (((val as u32) & 0x07) << 3usize);
        }
        #[doc = "desc WAVE0."]
        #[must_use]
        #[inline(always)]
        pub const fn wave0(&self) -> u8 {
            let val = (self.0 >> 6usize) & 0x03;
            val as u8
        }
        #[doc = "desc WAVE0."]
        #[inline(always)]
        pub const fn set_wave0(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 6usize)) | (((val as u32) & 0x03) << 6usize);
        }
        #[doc = "desc MAMP0."]
        #[must_use]
        #[inline(always)]
        pub const fn mamp0(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0x0f;
            val as u8
        }
        #[doc = "desc MAMP0."]
        #[inline(always)]
        pub const fn set_mamp0(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 8usize)) | (((val as u32) & 0x0f) << 8usize);
        }
        #[doc = "desc DMAEN0."]
        #[must_use]
        #[inline(always)]
        pub const fn dmaen0(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "desc DMAEN0."]
        #[inline(always)]
        pub const fn set_dmaen0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "desc DMAUDRIE0."]
        #[must_use]
        #[inline(always)]
        pub const fn dmaudrie0(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[doc = "desc DMAUDRIE0."]
        #[inline(always)]
        pub const fn set_dmaudrie0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[doc = "desc SREF0."]
        #[must_use]
        #[inline(always)]
        pub const fn sref0(&self) -> u8 {
            let val = (self.0 >> 14usize) & 0x03;
            val as u8
        }
        #[doc = "desc SREF0."]
        #[inline(always)]
        pub const fn set_sref0(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 14usize)) | (((val as u32) & 0x03) << 14usize);
        }
    }
    impl Default for Cr0 {
        #[inline(always)]
        fn default() -> Cr0 {
            Cr0(0)
        }
    }
    impl core::fmt::Debug for Cr0 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Cr0")
                .field("en0", &self.en0())
                .field("boff0", &self.boff0())
                .field("ten0", &self.ten0())
                .field("tsel0", &self.tsel0())
                .field("wave0", &self.wave0())
                .field("mamp0", &self.mamp0())
                .field("dmaen0", &self.dmaen0())
                .field("dmaudrie0", &self.dmaudrie0())
                .field("sref0", &self.sref0())
                .finish()
        }
    }
    #[doc = "desc DHR12L0."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Dhr12l0(pub u32);
    impl Dhr12l0 {
        #[doc = "desc DHR0."]
        #[must_use]
        #[inline(always)]
        pub const fn dhr0(&self) -> u16 {
            let val = (self.0 >> 4usize) & 0x0fff;
            val as u16
        }
        #[doc = "desc DHR0."]
        #[inline(always)]
        pub const fn set_dhr0(&mut self, val: u16) {
            self.0 = (self.0 & !(0x0fff << 4usize)) | (((val as u32) & 0x0fff) << 4usize);
        }
        #[doc = "desc RSV."]
        #[must_use]
        #[inline(always)]
        pub const fn rsv(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[doc = "desc RSV."]
        #[inline(always)]
        pub const fn set_rsv(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for Dhr12l0 {
        #[inline(always)]
        fn default() -> Dhr12l0 {
            Dhr12l0(0)
        }
    }
    impl core::fmt::Debug for Dhr12l0 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Dhr12l0")
                .field("dhr0", &self.dhr0())
                .field("rsv", &self.rsv())
                .finish()
        }
    }
    #[doc = "desc DHR12R0."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Dhr12r0(pub u32);
    impl Dhr12r0 {
        #[doc = "desc DHR0."]
        #[must_use]
        #[inline(always)]
        pub const fn dhr0(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x0fff;
            val as u16
        }
        #[doc = "desc DHR0."]
        #[inline(always)]
        pub const fn set_dhr0(&mut self, val: u16) {
            self.0 = (self.0 & !(0x0fff << 0usize)) | (((val as u32) & 0x0fff) << 0usize);
        }
        #[doc = "desc RSV."]
        #[must_use]
        #[inline(always)]
        pub const fn rsv(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[doc = "desc RSV."]
        #[inline(always)]
        pub const fn set_rsv(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for Dhr12r0 {
        #[inline(always)]
        fn default() -> Dhr12r0 {
            Dhr12r0(0)
        }
    }
    impl core::fmt::Debug for Dhr12r0 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Dhr12r0")
                .field("dhr0", &self.dhr0())
                .field("rsv", &self.rsv())
                .finish()
        }
    }
    #[doc = "desc DHR8R0."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Dhr8r0(pub u32);
    impl Dhr8r0 {
        #[doc = "desc DHR0."]
        #[must_use]
        #[inline(always)]
        pub const fn dhr0(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[doc = "desc DHR0."]
        #[inline(always)]
        pub const fn set_dhr0(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
        }
        #[doc = "desc RSV."]
        #[must_use]
        #[inline(always)]
        pub const fn rsv(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[doc = "desc RSV."]
        #[inline(always)]
        pub const fn set_rsv(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for Dhr8r0 {
        #[inline(always)]
        fn default() -> Dhr8r0 {
            Dhr8r0(0)
        }
    }
    impl core::fmt::Debug for Dhr8r0 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Dhr8r0")
                .field("dhr0", &self.dhr0())
                .field("rsv", &self.rsv())
                .finish()
        }
    }
    #[doc = "desc DOR0."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Dor0(pub u32);
    impl Dor0 {
        #[doc = "desc DOR0."]
        #[must_use]
        #[inline(always)]
        pub const fn dor0(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x0fff;
            val as u16
        }
        #[doc = "desc DOR0."]
        #[inline(always)]
        pub const fn set_dor0(&mut self, val: u16) {
            self.0 = (self.0 & !(0x0fff << 0usize)) | (((val as u32) & 0x0fff) << 0usize);
        }
        #[doc = "desc RSV."]
        #[must_use]
        #[inline(always)]
        pub const fn rsv(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[doc = "desc RSV."]
        #[inline(always)]
        pub const fn set_rsv(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for Dor0 {
        #[inline(always)]
        fn default() -> Dor0 {
            Dor0(0)
        }
    }
    impl core::fmt::Debug for Dor0 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Dor0")
                .field("dor0", &self.dor0())
                .field("rsv", &self.rsv())
                .finish()
        }
    }
    #[doc = "desc ETRS."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Etrs(pub u32);
    impl Etrs {
        #[doc = "desc PTRIGSEL."]
        #[must_use]
        #[inline(always)]
        pub const fn ptrigsel(&self) -> u8 {
            let val = (self.0 >> 4usize) & 0x07;
            val as u8
        }
        #[doc = "desc PTRIGSEL."]
        #[inline(always)]
        pub const fn set_ptrigsel(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 4usize)) | (((val as u32) & 0x07) << 4usize);
        }
        #[doc = "desc RSV."]
        #[must_use]
        #[inline(always)]
        pub const fn rsv(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[doc = "desc RSV."]
        #[inline(always)]
        pub const fn set_rsv(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for Etrs {
        #[inline(always)]
        fn default() -> Etrs {
            Etrs(0)
        }
    }
    impl core::fmt::Debug for Etrs {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Etrs")
                .field("ptrigsel", &self.ptrigsel())
                .field("rsv", &self.rsv())
                .finish()
        }
    }
    #[doc = "desc SR."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Sr(pub u32);
    impl Sr {
        #[doc = "desc DMAUDR0."]
        #[must_use]
        #[inline(always)]
        pub const fn dmaudr0(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[doc = "desc DMAUDR0."]
        #[inline(always)]
        pub const fn set_dmaudr0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[doc = "desc RSV."]
        #[must_use]
        #[inline(always)]
        pub const fn rsv(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[doc = "desc RSV."]
        #[inline(always)]
        pub const fn set_rsv(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
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
                .field("dmaudr0", &self.dmaudr0())
                .field("rsv", &self.rsv())
                .finish()
        }
    }
    #[doc = "desc SWTRIGR."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Swtrigr(pub u32);
    impl Swtrigr {
        #[doc = "desc SWTRIG0."]
        #[must_use]
        #[inline(always)]
        pub const fn swtrig0(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "desc SWTRIG0."]
        #[inline(always)]
        pub const fn set_swtrig0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "desc RSV."]
        #[must_use]
        #[inline(always)]
        pub const fn rsv(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[doc = "desc RSV."]
        #[inline(always)]
        pub const fn set_rsv(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for Swtrigr {
        #[inline(always)]
        fn default() -> Swtrigr {
            Swtrigr(0)
        }
    }
    impl core::fmt::Debug for Swtrigr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Swtrigr")
                .field("swtrig0", &self.swtrig0())
                .field("rsv", &self.rsv())
                .finish()
        }
    }
}
