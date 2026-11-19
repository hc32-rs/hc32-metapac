#![allow(clippy::missing_safety_doc)]
#![allow(clippy::identity_op)]
#![allow(clippy::unnecessary_cast)]
#![allow(clippy::erasing_op)]

#[doc = "desc TIM2_MODE0."]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Tim {
    ptr: *mut u8,
}
unsafe impl Send for Tim {}
unsafe impl Sync for Tim {}
impl Tim {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "desc ARR."]
    #[inline(always)]
    pub const fn arr(self) -> crate::common::Reg<regs::Arr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0usize) as _) }
    }
    #[doc = "desc CNT."]
    #[inline(always)]
    pub const fn cnt(self) -> crate::common::Reg<regs::Cnt, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x04usize) as _) }
    }
    #[doc = "desc CNT32."]
    #[inline(always)]
    pub const fn cnt32(self) -> crate::common::Reg<regs::Cnt32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x08usize) as _) }
    }
    #[doc = "desc M0CR."]
    #[inline(always)]
    pub const fn m0cr(self) -> crate::common::Reg<regs::M0cr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0cusize) as _) }
    }
    #[doc = "desc IFR."]
    #[inline(always)]
    pub const fn ifr(self) -> crate::common::Reg<regs::Ifr, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x10usize) as _) }
    }
    #[doc = "desc ICLR."]
    #[inline(always)]
    pub const fn iclr(self) -> crate::common::Reg<regs::Iclr, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x14usize) as _) }
    }
    #[doc = "desc DTR."]
    #[inline(always)]
    pub const fn dtr(self) -> crate::common::Reg<regs::Dtr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x30usize) as _) }
    }
}
pub mod regs {
    #[doc = "desc ARR."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Arr(pub u32);
    impl Arr {
        #[doc = "desc ARR."]
        #[must_use]
        #[inline(always)]
        pub const fn arr(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[doc = "desc ARR."]
        #[inline(always)]
        pub const fn set_arr(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
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
    impl Default for Arr {
        #[inline(always)]
        fn default() -> Arr {
            Arr(0)
        }
    }
    impl core::fmt::Debug for Arr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Arr")
                .field("arr", &self.arr())
                .field("rsv", &self.rsv())
                .finish()
        }
    }
    #[doc = "desc CNT."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Cnt(pub u32);
    impl Cnt {
        #[doc = "desc CNT."]
        #[must_use]
        #[inline(always)]
        pub const fn cnt(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[doc = "desc CNT."]
        #[inline(always)]
        pub const fn set_cnt(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
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
    impl Default for Cnt {
        #[inline(always)]
        fn default() -> Cnt {
            Cnt(0)
        }
    }
    impl core::fmt::Debug for Cnt {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Cnt")
                .field("cnt", &self.cnt())
                .field("rsv", &self.rsv())
                .finish()
        }
    }
    #[doc = "desc CNT32."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Cnt32(pub u32);
    impl Cnt32 {
        #[doc = "desc CNT32."]
        #[must_use]
        #[inline(always)]
        pub const fn cnt32(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "desc CNT32."]
        #[inline(always)]
        pub const fn set_cnt32(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Cnt32 {
        #[inline(always)]
        fn default() -> Cnt32 {
            Cnt32(0)
        }
    }
    impl core::fmt::Debug for Cnt32 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Cnt32")
                .field("cnt32", &self.cnt32())
                .finish()
        }
    }
    #[doc = "desc DTR."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Dtr(pub u32);
    impl Dtr {
        #[doc = "desc MOE."]
        #[must_use]
        #[inline(always)]
        pub const fn moe(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "desc MOE."]
        #[inline(always)]
        pub const fn set_moe(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
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
    impl Default for Dtr {
        #[inline(always)]
        fn default() -> Dtr {
            Dtr(0)
        }
    }
    impl core::fmt::Debug for Dtr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Dtr")
                .field("moe", &self.moe())
                .field("rsv", &self.rsv())
                .finish()
        }
    }
    #[doc = "desc ICLR."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Iclr(pub u32);
    impl Iclr {
        #[doc = "desc UIF."]
        #[must_use]
        #[inline(always)]
        pub const fn uif(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "desc UIF."]
        #[inline(always)]
        pub const fn set_uif(&mut self, val: bool) {
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
    impl Default for Iclr {
        #[inline(always)]
        fn default() -> Iclr {
            Iclr(0)
        }
    }
    impl core::fmt::Debug for Iclr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Iclr")
                .field("uif", &self.uif())
                .field("rsv", &self.rsv())
                .finish()
        }
    }
    #[doc = "desc IFR."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ifr(pub u32);
    impl Ifr {
        #[doc = "desc UIF."]
        #[must_use]
        #[inline(always)]
        pub const fn uif(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "desc UIF."]
        #[inline(always)]
        pub const fn set_uif(&mut self, val: bool) {
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
    impl Default for Ifr {
        #[inline(always)]
        fn default() -> Ifr {
            Ifr(0)
        }
    }
    impl core::fmt::Debug for Ifr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Ifr")
                .field("uif", &self.uif())
                .field("rsv", &self.rsv())
                .finish()
        }
    }
    #[doc = "desc M0CR."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct M0cr(pub u32);
    impl M0cr {
        #[doc = "desc CTEN."]
        #[must_use]
        #[inline(always)]
        pub const fn cten(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "desc CTEN."]
        #[inline(always)]
        pub const fn set_cten(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "desc MD."]
        #[must_use]
        #[inline(always)]
        pub const fn md(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "desc MD."]
        #[inline(always)]
        pub const fn set_md(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "desc CT."]
        #[must_use]
        #[inline(always)]
        pub const fn ct(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "desc CT."]
        #[inline(always)]
        pub const fn set_ct(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "desc TOGEN."]
        #[must_use]
        #[inline(always)]
        pub const fn togen(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "desc TOGEN."]
        #[inline(always)]
        pub const fn set_togen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "desc PRS."]
        #[must_use]
        #[inline(always)]
        pub const fn prs(&self) -> u8 {
            let val = (self.0 >> 4usize) & 0x07;
            val as u8
        }
        #[doc = "desc PRS."]
        #[inline(always)]
        pub const fn set_prs(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 4usize)) | (((val as u32) & 0x07) << 4usize);
        }
        #[doc = "desc GATE."]
        #[must_use]
        #[inline(always)]
        pub const fn gate(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "desc GATE."]
        #[inline(always)]
        pub const fn set_gate(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "desc GATEP."]
        #[must_use]
        #[inline(always)]
        pub const fn gatep(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "desc GATEP."]
        #[inline(always)]
        pub const fn set_gatep(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "desc UIE."]
        #[must_use]
        #[inline(always)]
        pub const fn uie(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "desc UIE."]
        #[inline(always)]
        pub const fn set_uie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[doc = "desc MODE."]
        #[must_use]
        #[inline(always)]
        pub const fn mode(&self) -> u8 {
            let val = (self.0 >> 12usize) & 0x03;
            val as u8
        }
        #[doc = "desc MODE."]
        #[inline(always)]
        pub const fn set_mode(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 12usize)) | (((val as u32) & 0x03) << 12usize);
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
    impl Default for M0cr {
        #[inline(always)]
        fn default() -> M0cr {
            M0cr(0)
        }
    }
    impl core::fmt::Debug for M0cr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("M0cr")
                .field("cten", &self.cten())
                .field("md", &self.md())
                .field("ct", &self.ct())
                .field("togen", &self.togen())
                .field("prs", &self.prs())
                .field("gate", &self.gate())
                .field("gatep", &self.gatep())
                .field("uie", &self.uie())
                .field("mode", &self.mode())
                .field("rsv", &self.rsv())
                .finish()
        }
    }
}
