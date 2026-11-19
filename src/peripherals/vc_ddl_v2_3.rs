#![allow(clippy::missing_safety_doc)]
#![allow(clippy::identity_op)]
#![allow(clippy::unnecessary_cast)]
#![allow(clippy::erasing_op)]

#[doc = "desc VC."]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Vc {
    ptr: *mut u8,
}
unsafe impl Send for Vc {}
unsafe impl Sync for Vc {}
impl Vc {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "desc VC0_CR0."]
    #[inline(always)]
    pub const fn vc0_cr0(self) -> crate::common::Reg<regs::Vc0Cr0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0usize) as _) }
    }
    #[doc = "desc VC0_CR1."]
    #[inline(always)]
    pub const fn vc0_cr1(self) -> crate::common::Reg<regs::Vc0Cr1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x04usize) as _) }
    }
    #[doc = "desc VC0_CR2."]
    #[inline(always)]
    pub const fn vc0_cr2(self) -> crate::common::Reg<regs::Vc0Cr2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x08usize) as _) }
    }
    #[doc = "desc VC0_SR."]
    #[inline(always)]
    pub const fn vc0_sr(self) -> crate::common::Reg<regs::Vc0Sr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0cusize) as _) }
    }
    #[doc = "desc VC1_CR0."]
    #[inline(always)]
    pub const fn vc1_cr0(self) -> crate::common::Reg<regs::Vc1Cr0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x10usize) as _) }
    }
    #[doc = "desc VC1_CR1."]
    #[inline(always)]
    pub const fn vc1_cr1(self) -> crate::common::Reg<regs::Vc1Cr1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x14usize) as _) }
    }
    #[doc = "desc VC1_CR2."]
    #[inline(always)]
    pub const fn vc1_cr2(self) -> crate::common::Reg<regs::Vc1Cr2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x18usize) as _) }
    }
    #[doc = "desc VC1_SR."]
    #[inline(always)]
    pub const fn vc1_sr(self) -> crate::common::Reg<regs::Vc1Sr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1cusize) as _) }
    }
    #[doc = "desc VC_TRIM."]
    #[inline(always)]
    pub const fn vc_trim(self) -> crate::common::Reg<regs::VcTrim, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x20usize) as _) }
    }
}
pub mod regs {
    #[doc = "desc VC0_CR0."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Vc0Cr0(pub u32);
    impl Vc0Cr0 {
        #[doc = "desc DIV."]
        #[must_use]
        #[inline(always)]
        pub const fn div(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x3f;
            val as u8
        }
        #[doc = "desc DIV."]
        #[inline(always)]
        pub const fn set_div(&mut self, val: u8) {
            self.0 = (self.0 & !(0x3f << 0usize)) | (((val as u32) & 0x3f) << 0usize);
        }
        #[doc = "desc DIV_EN."]
        #[must_use]
        #[inline(always)]
        pub const fn div_en(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "desc DIV_EN."]
        #[inline(always)]
        pub const fn set_div_en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "desc BIAS."]
        #[must_use]
        #[inline(always)]
        pub const fn bias(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0x03;
            val as u8
        }
        #[doc = "desc BIAS."]
        #[inline(always)]
        pub const fn set_bias(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 8usize)) | (((val as u32) & 0x03) << 8usize);
        }
        #[doc = "desc HYS."]
        #[must_use]
        #[inline(always)]
        pub const fn hys(&self) -> u8 {
            let val = (self.0 >> 10usize) & 0x03;
            val as u8
        }
        #[doc = "desc HYS."]
        #[inline(always)]
        pub const fn set_hys(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 10usize)) | (((val as u32) & 0x03) << 10usize);
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
    impl Default for Vc0Cr0 {
        #[inline(always)]
        fn default() -> Vc0Cr0 {
            Vc0Cr0(0)
        }
    }
    impl core::fmt::Debug for Vc0Cr0 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Vc0Cr0")
                .field("div", &self.div())
                .field("div_en", &self.div_en())
                .field("bias", &self.bias())
                .field("hys", &self.hys())
                .field("rsv", &self.rsv())
                .finish()
        }
    }
    #[doc = "desc VC0_CR1."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Vc0Cr1(pub u32);
    impl Vc0Cr1 {
        #[doc = "desc PSEL."]
        #[must_use]
        #[inline(always)]
        pub const fn psel(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x0f;
            val as u8
        }
        #[doc = "desc PSEL."]
        #[inline(always)]
        pub const fn set_psel(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
        }
        #[doc = "desc NSEL."]
        #[must_use]
        #[inline(always)]
        pub const fn nsel(&self) -> u8 {
            let val = (self.0 >> 4usize) & 0x0f;
            val as u8
        }
        #[doc = "desc NSEL."]
        #[inline(always)]
        pub const fn set_nsel(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 4usize)) | (((val as u32) & 0x0f) << 4usize);
        }
        #[doc = "desc FLTTIME."]
        #[must_use]
        #[inline(always)]
        pub const fn flttime(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0x0f;
            val as u8
        }
        #[doc = "desc FLTTIME."]
        #[inline(always)]
        pub const fn set_flttime(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 8usize)) | (((val as u32) & 0x0f) << 8usize);
        }
        #[doc = "desc FALLING."]
        #[must_use]
        #[inline(always)]
        pub const fn falling(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "desc FALLING."]
        #[inline(always)]
        pub const fn set_falling(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "desc RISING."]
        #[must_use]
        #[inline(always)]
        pub const fn rising(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[doc = "desc RISING."]
        #[inline(always)]
        pub const fn set_rising(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[doc = "desc LEVEL."]
        #[must_use]
        #[inline(always)]
        pub const fn level(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "desc LEVEL."]
        #[inline(always)]
        pub const fn set_level(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[doc = "desc IE."]
        #[must_use]
        #[inline(always)]
        pub const fn ie(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "desc IE."]
        #[inline(always)]
        pub const fn set_ie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
        #[doc = "desc EN."]
        #[must_use]
        #[inline(always)]
        pub const fn en(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "desc EN."]
        #[inline(always)]
        pub const fn set_en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "desc FLTCLK."]
        #[must_use]
        #[inline(always)]
        pub const fn fltclk(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "desc FLTCLK."]
        #[inline(always)]
        pub const fn set_fltclk(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[doc = "desc FLT_MODE."]
        #[must_use]
        #[inline(always)]
        pub const fn flt_mode(&self) -> u8 {
            let val = (self.0 >> 18usize) & 0x03;
            val as u8
        }
        #[doc = "desc FLT_MODE."]
        #[inline(always)]
        pub const fn set_flt_mode(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 18usize)) | (((val as u32) & 0x03) << 18usize);
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
    impl Default for Vc0Cr1 {
        #[inline(always)]
        fn default() -> Vc0Cr1 {
            Vc0Cr1(0)
        }
    }
    impl core::fmt::Debug for Vc0Cr1 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Vc0Cr1")
                .field("psel", &self.psel())
                .field("nsel", &self.nsel())
                .field("flttime", &self.flttime())
                .field("falling", &self.falling())
                .field("rising", &self.rising())
                .field("level", &self.level())
                .field("ie", &self.ie())
                .field("en", &self.en())
                .field("fltclk", &self.fltclk())
                .field("flt_mode", &self.flt_mode())
                .field("rsv", &self.rsv())
                .finish()
        }
    }
    #[doc = "desc VC0_CR2."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Vc0Cr2(pub u32);
    impl Vc0Cr2 {
        #[doc = "desc POL."]
        #[must_use]
        #[inline(always)]
        pub const fn pol(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "desc POL."]
        #[inline(always)]
        pub const fn set_pol(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "desc ATIM3CLR."]
        #[must_use]
        #[inline(always)]
        pub const fn atim3clr(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "desc ATIM3CLR."]
        #[inline(always)]
        pub const fn set_atim3clr(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "desc ATIMBK."]
        #[must_use]
        #[inline(always)]
        pub const fn atimbk(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "desc ATIMBK."]
        #[inline(always)]
        pub const fn set_atimbk(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "desc WINDOW."]
        #[must_use]
        #[inline(always)]
        pub const fn window(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "desc WINDOW."]
        #[inline(always)]
        pub const fn set_window(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "desc BLANK."]
        #[must_use]
        #[inline(always)]
        pub const fn blank(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0x07;
            val as u8
        }
        #[doc = "desc BLANK."]
        #[inline(always)]
        pub const fn set_blank(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 8usize)) | (((val as u32) & 0x07) << 8usize);
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
    impl Default for Vc0Cr2 {
        #[inline(always)]
        fn default() -> Vc0Cr2 {
            Vc0Cr2(0)
        }
    }
    impl core::fmt::Debug for Vc0Cr2 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Vc0Cr2")
                .field("pol", &self.pol())
                .field("atim3clr", &self.atim3clr())
                .field("atimbk", &self.atimbk())
                .field("window", &self.window())
                .field("blank", &self.blank())
                .field("rsv", &self.rsv())
                .finish()
        }
    }
    #[doc = "desc VC0_SR."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Vc0Sr(pub u32);
    impl Vc0Sr {
        #[doc = "desc INTF."]
        #[must_use]
        #[inline(always)]
        pub const fn intf(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "desc INTF."]
        #[inline(always)]
        pub const fn set_intf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "desc FLTV."]
        #[must_use]
        #[inline(always)]
        pub const fn fltv(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "desc FLTV."]
        #[inline(always)]
        pub const fn set_fltv(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
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
    impl Default for Vc0Sr {
        #[inline(always)]
        fn default() -> Vc0Sr {
            Vc0Sr(0)
        }
    }
    impl core::fmt::Debug for Vc0Sr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Vc0Sr")
                .field("intf", &self.intf())
                .field("fltv", &self.fltv())
                .field("rsv", &self.rsv())
                .finish()
        }
    }
    #[doc = "desc VC1_CR0."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Vc1Cr0(pub u32);
    impl Vc1Cr0 {
        #[doc = "desc DIV."]
        #[must_use]
        #[inline(always)]
        pub const fn div(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x3f;
            val as u8
        }
        #[doc = "desc DIV."]
        #[inline(always)]
        pub const fn set_div(&mut self, val: u8) {
            self.0 = (self.0 & !(0x3f << 0usize)) | (((val as u32) & 0x3f) << 0usize);
        }
        #[doc = "desc DIV_EN."]
        #[must_use]
        #[inline(always)]
        pub const fn div_en(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "desc DIV_EN."]
        #[inline(always)]
        pub const fn set_div_en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "desc BIAS."]
        #[must_use]
        #[inline(always)]
        pub const fn bias(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0x03;
            val as u8
        }
        #[doc = "desc BIAS."]
        #[inline(always)]
        pub const fn set_bias(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 8usize)) | (((val as u32) & 0x03) << 8usize);
        }
        #[doc = "desc HYS."]
        #[must_use]
        #[inline(always)]
        pub const fn hys(&self) -> u8 {
            let val = (self.0 >> 10usize) & 0x03;
            val as u8
        }
        #[doc = "desc HYS."]
        #[inline(always)]
        pub const fn set_hys(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 10usize)) | (((val as u32) & 0x03) << 10usize);
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
    impl Default for Vc1Cr0 {
        #[inline(always)]
        fn default() -> Vc1Cr0 {
            Vc1Cr0(0)
        }
    }
    impl core::fmt::Debug for Vc1Cr0 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Vc1Cr0")
                .field("div", &self.div())
                .field("div_en", &self.div_en())
                .field("bias", &self.bias())
                .field("hys", &self.hys())
                .field("rsv", &self.rsv())
                .finish()
        }
    }
    #[doc = "desc VC1_CR1."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Vc1Cr1(pub u32);
    impl Vc1Cr1 {
        #[doc = "desc PSEL."]
        #[must_use]
        #[inline(always)]
        pub const fn psel(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x0f;
            val as u8
        }
        #[doc = "desc PSEL."]
        #[inline(always)]
        pub const fn set_psel(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
        }
        #[doc = "desc NSEL."]
        #[must_use]
        #[inline(always)]
        pub const fn nsel(&self) -> u8 {
            let val = (self.0 >> 4usize) & 0x0f;
            val as u8
        }
        #[doc = "desc NSEL."]
        #[inline(always)]
        pub const fn set_nsel(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 4usize)) | (((val as u32) & 0x0f) << 4usize);
        }
        #[doc = "desc FLTTIME."]
        #[must_use]
        #[inline(always)]
        pub const fn flttime(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0x0f;
            val as u8
        }
        #[doc = "desc FLTTIME."]
        #[inline(always)]
        pub const fn set_flttime(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 8usize)) | (((val as u32) & 0x0f) << 8usize);
        }
        #[doc = "desc FALLING."]
        #[must_use]
        #[inline(always)]
        pub const fn falling(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "desc FALLING."]
        #[inline(always)]
        pub const fn set_falling(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "desc RISING."]
        #[must_use]
        #[inline(always)]
        pub const fn rising(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[doc = "desc RISING."]
        #[inline(always)]
        pub const fn set_rising(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[doc = "desc LEVEL."]
        #[must_use]
        #[inline(always)]
        pub const fn level(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "desc LEVEL."]
        #[inline(always)]
        pub const fn set_level(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[doc = "desc IE."]
        #[must_use]
        #[inline(always)]
        pub const fn ie(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "desc IE."]
        #[inline(always)]
        pub const fn set_ie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
        #[doc = "desc EN."]
        #[must_use]
        #[inline(always)]
        pub const fn en(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "desc EN."]
        #[inline(always)]
        pub const fn set_en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "desc FLTCLK."]
        #[must_use]
        #[inline(always)]
        pub const fn fltclk(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "desc FLTCLK."]
        #[inline(always)]
        pub const fn set_fltclk(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[doc = "desc FLT_MODE."]
        #[must_use]
        #[inline(always)]
        pub const fn flt_mode(&self) -> u8 {
            let val = (self.0 >> 18usize) & 0x03;
            val as u8
        }
        #[doc = "desc FLT_MODE."]
        #[inline(always)]
        pub const fn set_flt_mode(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 18usize)) | (((val as u32) & 0x03) << 18usize);
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
    impl Default for Vc1Cr1 {
        #[inline(always)]
        fn default() -> Vc1Cr1 {
            Vc1Cr1(0)
        }
    }
    impl core::fmt::Debug for Vc1Cr1 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Vc1Cr1")
                .field("psel", &self.psel())
                .field("nsel", &self.nsel())
                .field("flttime", &self.flttime())
                .field("falling", &self.falling())
                .field("rising", &self.rising())
                .field("level", &self.level())
                .field("ie", &self.ie())
                .field("en", &self.en())
                .field("fltclk", &self.fltclk())
                .field("flt_mode", &self.flt_mode())
                .field("rsv", &self.rsv())
                .finish()
        }
    }
    #[doc = "desc VC1_CR2."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Vc1Cr2(pub u32);
    impl Vc1Cr2 {
        #[doc = "desc POL."]
        #[must_use]
        #[inline(always)]
        pub const fn pol(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "desc POL."]
        #[inline(always)]
        pub const fn set_pol(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "desc ATIM3CLR."]
        #[must_use]
        #[inline(always)]
        pub const fn atim3clr(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "desc ATIM3CLR."]
        #[inline(always)]
        pub const fn set_atim3clr(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "desc ATIMBK."]
        #[must_use]
        #[inline(always)]
        pub const fn atimbk(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "desc ATIMBK."]
        #[inline(always)]
        pub const fn set_atimbk(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "desc WINDOW."]
        #[must_use]
        #[inline(always)]
        pub const fn window(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "desc WINDOW."]
        #[inline(always)]
        pub const fn set_window(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "desc BLANK."]
        #[must_use]
        #[inline(always)]
        pub const fn blank(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0x07;
            val as u8
        }
        #[doc = "desc BLANK."]
        #[inline(always)]
        pub const fn set_blank(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 8usize)) | (((val as u32) & 0x07) << 8usize);
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
    impl Default for Vc1Cr2 {
        #[inline(always)]
        fn default() -> Vc1Cr2 {
            Vc1Cr2(0)
        }
    }
    impl core::fmt::Debug for Vc1Cr2 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Vc1Cr2")
                .field("pol", &self.pol())
                .field("atim3clr", &self.atim3clr())
                .field("atimbk", &self.atimbk())
                .field("window", &self.window())
                .field("blank", &self.blank())
                .field("rsv", &self.rsv())
                .finish()
        }
    }
    #[doc = "desc VC1_SR."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Vc1Sr(pub u32);
    impl Vc1Sr {
        #[doc = "desc INTF."]
        #[must_use]
        #[inline(always)]
        pub const fn intf(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "desc INTF."]
        #[inline(always)]
        pub const fn set_intf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "desc FLTV."]
        #[must_use]
        #[inline(always)]
        pub const fn fltv(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "desc FLTV."]
        #[inline(always)]
        pub const fn set_fltv(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
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
    impl Default for Vc1Sr {
        #[inline(always)]
        fn default() -> Vc1Sr {
            Vc1Sr(0)
        }
    }
    impl core::fmt::Debug for Vc1Sr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Vc1Sr")
                .field("intf", &self.intf())
                .field("fltv", &self.fltv())
                .field("rsv", &self.rsv())
                .finish()
        }
    }
    #[doc = "desc VC_TRIM."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct VcTrim(pub u32);
    impl VcTrim {
        #[doc = "desc TRIM0."]
        #[must_use]
        #[inline(always)]
        pub const fn trim0(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x0f;
            val as u8
        }
        #[doc = "desc TRIM0."]
        #[inline(always)]
        pub const fn set_trim0(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
        }
        #[doc = "desc TRIM1."]
        #[must_use]
        #[inline(always)]
        pub const fn trim1(&self) -> u8 {
            let val = (self.0 >> 4usize) & 0x0f;
            val as u8
        }
        #[doc = "desc TRIM1."]
        #[inline(always)]
        pub const fn set_trim1(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 4usize)) | (((val as u32) & 0x0f) << 4usize);
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
    impl Default for VcTrim {
        #[inline(always)]
        fn default() -> VcTrim {
            VcTrim(0)
        }
    }
    impl core::fmt::Debug for VcTrim {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("VcTrim")
                .field("trim0", &self.trim0())
                .field("trim1", &self.trim1())
                .field("rsv", &self.rsv())
                .finish()
        }
    }
}
