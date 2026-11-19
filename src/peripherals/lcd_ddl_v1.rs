#![allow(clippy::missing_safety_doc)]
#![allow(clippy::identity_op)]
#![allow(clippy::unnecessary_cast)]
#![allow(clippy::erasing_op)]

#[doc = "desc LCD."]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Lcd {
    ptr: *mut u8,
}
unsafe impl Send for Lcd {}
unsafe impl Sync for Lcd {}
impl Lcd {
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
    #[doc = "desc CR1."]
    #[inline(always)]
    pub const fn cr1(self) -> crate::common::Reg<regs::Cr1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x04usize) as _) }
    }
    #[doc = "desc INTCLR."]
    #[inline(always)]
    pub const fn intclr(self) -> crate::common::Reg<regs::Intclr, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x08usize) as _) }
    }
    #[doc = "desc POEN0."]
    #[inline(always)]
    pub const fn poen0(self) -> crate::common::Reg<regs::Poen0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0cusize) as _) }
    }
    #[doc = "desc POEN1."]
    #[inline(always)]
    pub const fn poen1(self) -> crate::common::Reg<regs::Poen1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x10usize) as _) }
    }
    #[doc = "desc RAM0."]
    #[inline(always)]
    pub const fn ram0(self) -> crate::common::Reg<regs::Ram0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x40usize) as _) }
    }
    #[doc = "desc RAM1."]
    #[inline(always)]
    pub const fn ram1(self) -> crate::common::Reg<regs::Ram1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x44usize) as _) }
    }
    #[doc = "desc RAM2."]
    #[inline(always)]
    pub const fn ram2(self) -> crate::common::Reg<regs::Ram2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x48usize) as _) }
    }
    #[doc = "desc RAM3."]
    #[inline(always)]
    pub const fn ram3(self) -> crate::common::Reg<regs::Ram3, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x4cusize) as _) }
    }
    #[doc = "desc RAM4."]
    #[inline(always)]
    pub const fn ram4(self) -> crate::common::Reg<regs::Ram4, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x50usize) as _) }
    }
    #[doc = "desc RAM5."]
    #[inline(always)]
    pub const fn ram5(self) -> crate::common::Reg<regs::Ram5, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x54usize) as _) }
    }
    #[doc = "desc RAM6."]
    #[inline(always)]
    pub const fn ram6(self) -> crate::common::Reg<regs::Ram6, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x58usize) as _) }
    }
    #[doc = "desc RAM7."]
    #[inline(always)]
    pub const fn ram7(self) -> crate::common::Reg<regs::Ram7, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x5cusize) as _) }
    }
    #[doc = "desc RAM8."]
    #[inline(always)]
    pub const fn ram8(self) -> crate::common::Reg<regs::Ram8, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x60usize) as _) }
    }
    #[doc = "desc RAM9."]
    #[inline(always)]
    pub const fn ram9(self) -> crate::common::Reg<regs::Ram9, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x64usize) as _) }
    }
    #[doc = "desc RAMA."]
    #[inline(always)]
    pub const fn rama(self) -> crate::common::Reg<regs::Rama, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x68usize) as _) }
    }
    #[doc = "desc RAMB."]
    #[inline(always)]
    pub const fn ramb(self) -> crate::common::Reg<regs::Ramb, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x6cusize) as _) }
    }
    #[doc = "desc RAMC."]
    #[inline(always)]
    pub const fn ramc(self) -> crate::common::Reg<regs::Ramc, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x70usize) as _) }
    }
    #[doc = "desc RAMD."]
    #[inline(always)]
    pub const fn ramd(self) -> crate::common::Reg<regs::Ramd, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x74usize) as _) }
    }
    #[doc = "desc RAME."]
    #[inline(always)]
    pub const fn rame(self) -> crate::common::Reg<regs::Rame, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x78usize) as _) }
    }
    #[doc = "desc RAMF."]
    #[inline(always)]
    pub const fn ramf(self) -> crate::common::Reg<regs::Ramf, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x7cusize) as _) }
    }
}
pub mod regs {
    #[doc = "desc CR0."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Cr0(pub u32);
    impl Cr0 {
        #[doc = "desc EN."]
        #[must_use]
        #[inline(always)]
        pub const fn en(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "desc EN."]
        #[inline(always)]
        pub const fn set_en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "desc LCDCLK."]
        #[must_use]
        #[inline(always)]
        pub const fn lcdclk(&self) -> u8 {
            let val = (self.0 >> 1usize) & 0x03;
            val as u8
        }
        #[doc = "desc LCDCLK."]
        #[inline(always)]
        pub const fn set_lcdclk(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 1usize)) | (((val as u32) & 0x03) << 1usize);
        }
        #[doc = "desc CPCLK."]
        #[must_use]
        #[inline(always)]
        pub const fn cpclk(&self) -> u8 {
            let val = (self.0 >> 3usize) & 0x03;
            val as u8
        }
        #[doc = "desc CPCLK."]
        #[inline(always)]
        pub const fn set_cpclk(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 3usize)) | (((val as u32) & 0x03) << 3usize);
        }
        #[doc = "desc BIAS."]
        #[must_use]
        #[inline(always)]
        pub const fn bias(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "desc BIAS."]
        #[inline(always)]
        pub const fn set_bias(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "desc DUTY."]
        #[must_use]
        #[inline(always)]
        pub const fn duty(&self) -> u8 {
            let val = (self.0 >> 6usize) & 0x07;
            val as u8
        }
        #[doc = "desc DUTY."]
        #[inline(always)]
        pub const fn set_duty(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 6usize)) | (((val as u32) & 0x07) << 6usize);
        }
        #[doc = "desc BSEL."]
        #[must_use]
        #[inline(always)]
        pub const fn bsel(&self) -> u8 {
            let val = (self.0 >> 9usize) & 0x07;
            val as u8
        }
        #[doc = "desc BSEL."]
        #[inline(always)]
        pub const fn set_bsel(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 9usize)) | (((val as u32) & 0x07) << 9usize);
        }
        #[doc = "desc CONTRAST."]
        #[must_use]
        #[inline(always)]
        pub const fn contrast(&self) -> u8 {
            let val = (self.0 >> 12usize) & 0x0f;
            val as u8
        }
        #[doc = "desc CONTRAST."]
        #[inline(always)]
        pub const fn set_contrast(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 12usize)) | (((val as u32) & 0x0f) << 12usize);
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
    impl Default for Cr0 {
        #[inline(always)]
        fn default() -> Cr0 {
            Cr0(0)
        }
    }
    impl core::fmt::Debug for Cr0 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Cr0")
                .field("en", &self.en())
                .field("lcdclk", &self.lcdclk())
                .field("cpclk", &self.cpclk())
                .field("bias", &self.bias())
                .field("duty", &self.duty())
                .field("bsel", &self.bsel())
                .field("contrast", &self.contrast())
                .field("rsv", &self.rsv())
                .finish()
        }
    }
    #[doc = "desc CR1."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Cr1(pub u32);
    impl Cr1 {
        #[doc = "desc BLINKCNT."]
        #[must_use]
        #[inline(always)]
        pub const fn blinkcnt(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x3f;
            val as u8
        }
        #[doc = "desc BLINKCNT."]
        #[inline(always)]
        pub const fn set_blinkcnt(&mut self, val: u8) {
            self.0 = (self.0 & !(0x3f << 0usize)) | (((val as u32) & 0x3f) << 0usize);
        }
        #[doc = "desc BLINKEN."]
        #[must_use]
        #[inline(always)]
        pub const fn blinken(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "desc BLINKEN."]
        #[inline(always)]
        pub const fn set_blinken(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "desc CLKSRC."]
        #[must_use]
        #[inline(always)]
        pub const fn clksrc(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "desc CLKSRC."]
        #[inline(always)]
        pub const fn set_clksrc(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "desc MODE."]
        #[must_use]
        #[inline(always)]
        pub const fn mode(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "desc MODE."]
        #[inline(always)]
        pub const fn set_mode(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "desc IE."]
        #[must_use]
        #[inline(always)]
        pub const fn ie(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "desc IE."]
        #[inline(always)]
        pub const fn set_ie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "desc DMAEN."]
        #[must_use]
        #[inline(always)]
        pub const fn dmaen(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "desc DMAEN."]
        #[inline(always)]
        pub const fn set_dmaen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[doc = "desc INTF."]
        #[must_use]
        #[inline(always)]
        pub const fn intf(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "desc INTF."]
        #[inline(always)]
        pub const fn set_intf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
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
    impl Default for Cr1 {
        #[inline(always)]
        fn default() -> Cr1 {
            Cr1(0)
        }
    }
    impl core::fmt::Debug for Cr1 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Cr1")
                .field("blinkcnt", &self.blinkcnt())
                .field("blinken", &self.blinken())
                .field("clksrc", &self.clksrc())
                .field("mode", &self.mode())
                .field("ie", &self.ie())
                .field("dmaen", &self.dmaen())
                .field("intf", &self.intf())
                .field("rsv", &self.rsv())
                .finish()
        }
    }
    #[doc = "desc INTCLR."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Intclr(pub u32);
    impl Intclr {
        #[doc = "desc INTF."]
        #[must_use]
        #[inline(always)]
        pub const fn intf(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "desc INTF."]
        #[inline(always)]
        pub const fn set_intf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
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
    impl Default for Intclr {
        #[inline(always)]
        fn default() -> Intclr {
            Intclr(0)
        }
    }
    impl core::fmt::Debug for Intclr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Intclr")
                .field("intf", &self.intf())
                .field("rsv", &self.rsv())
                .finish()
        }
    }
    #[doc = "desc POEN0."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Poen0(pub u32);
    impl Poen0 {
        #[doc = "desc S0."]
        #[must_use]
        #[inline(always)]
        pub const fn s0(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "desc S0."]
        #[inline(always)]
        pub const fn set_s0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "desc S1."]
        #[must_use]
        #[inline(always)]
        pub const fn s1(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "desc S1."]
        #[inline(always)]
        pub const fn set_s1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "desc S2."]
        #[must_use]
        #[inline(always)]
        pub const fn s2(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "desc S2."]
        #[inline(always)]
        pub const fn set_s2(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "desc S3."]
        #[must_use]
        #[inline(always)]
        pub const fn s3(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "desc S3."]
        #[inline(always)]
        pub const fn set_s3(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "desc S4."]
        #[must_use]
        #[inline(always)]
        pub const fn s4(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "desc S4."]
        #[inline(always)]
        pub const fn set_s4(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "desc S5."]
        #[must_use]
        #[inline(always)]
        pub const fn s5(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "desc S5."]
        #[inline(always)]
        pub const fn set_s5(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "desc S6."]
        #[must_use]
        #[inline(always)]
        pub const fn s6(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "desc S6."]
        #[inline(always)]
        pub const fn set_s6(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "desc S7."]
        #[must_use]
        #[inline(always)]
        pub const fn s7(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "desc S7."]
        #[inline(always)]
        pub const fn set_s7(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "desc S8."]
        #[must_use]
        #[inline(always)]
        pub const fn s8(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "desc S8."]
        #[inline(always)]
        pub const fn set_s8(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "desc S9."]
        #[must_use]
        #[inline(always)]
        pub const fn s9(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "desc S9."]
        #[inline(always)]
        pub const fn set_s9(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "desc S10."]
        #[must_use]
        #[inline(always)]
        pub const fn s10(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "desc S10."]
        #[inline(always)]
        pub const fn set_s10(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[doc = "desc S11."]
        #[must_use]
        #[inline(always)]
        pub const fn s11(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "desc S11."]
        #[inline(always)]
        pub const fn set_s11(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[doc = "desc S12."]
        #[must_use]
        #[inline(always)]
        pub const fn s12(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "desc S12."]
        #[inline(always)]
        pub const fn set_s12(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "desc S13."]
        #[must_use]
        #[inline(always)]
        pub const fn s13(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[doc = "desc S13."]
        #[inline(always)]
        pub const fn set_s13(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[doc = "desc S14."]
        #[must_use]
        #[inline(always)]
        pub const fn s14(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "desc S14."]
        #[inline(always)]
        pub const fn set_s14(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[doc = "desc S15."]
        #[must_use]
        #[inline(always)]
        pub const fn s15(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "desc S15."]
        #[inline(always)]
        pub const fn set_s15(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
        #[doc = "desc S16."]
        #[must_use]
        #[inline(always)]
        pub const fn s16(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "desc S16."]
        #[inline(always)]
        pub const fn set_s16(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "desc S17."]
        #[must_use]
        #[inline(always)]
        pub const fn s17(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "desc S17."]
        #[inline(always)]
        pub const fn set_s17(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[doc = "desc S18."]
        #[must_use]
        #[inline(always)]
        pub const fn s18(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[doc = "desc S18."]
        #[inline(always)]
        pub const fn set_s18(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
        #[doc = "desc S19."]
        #[must_use]
        #[inline(always)]
        pub const fn s19(&self) -> bool {
            let val = (self.0 >> 19usize) & 0x01;
            val != 0
        }
        #[doc = "desc S19."]
        #[inline(always)]
        pub const fn set_s19(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
        }
        #[doc = "desc S20."]
        #[must_use]
        #[inline(always)]
        pub const fn s20(&self) -> bool {
            let val = (self.0 >> 20usize) & 0x01;
            val != 0
        }
        #[doc = "desc S20."]
        #[inline(always)]
        pub const fn set_s20(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
        }
        #[doc = "desc S21."]
        #[must_use]
        #[inline(always)]
        pub const fn s21(&self) -> bool {
            let val = (self.0 >> 21usize) & 0x01;
            val != 0
        }
        #[doc = "desc S21."]
        #[inline(always)]
        pub const fn set_s21(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
        }
        #[doc = "desc S22."]
        #[must_use]
        #[inline(always)]
        pub const fn s22(&self) -> bool {
            let val = (self.0 >> 22usize) & 0x01;
            val != 0
        }
        #[doc = "desc S22."]
        #[inline(always)]
        pub const fn set_s22(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
        }
        #[doc = "desc S23."]
        #[must_use]
        #[inline(always)]
        pub const fn s23(&self) -> bool {
            let val = (self.0 >> 23usize) & 0x01;
            val != 0
        }
        #[doc = "desc S23."]
        #[inline(always)]
        pub const fn set_s23(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
        }
        #[doc = "desc S24."]
        #[must_use]
        #[inline(always)]
        pub const fn s24(&self) -> bool {
            let val = (self.0 >> 24usize) & 0x01;
            val != 0
        }
        #[doc = "desc S24."]
        #[inline(always)]
        pub const fn set_s24(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
        }
        #[doc = "desc S25."]
        #[must_use]
        #[inline(always)]
        pub const fn s25(&self) -> bool {
            let val = (self.0 >> 25usize) & 0x01;
            val != 0
        }
        #[doc = "desc S25."]
        #[inline(always)]
        pub const fn set_s25(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
        }
        #[doc = "desc S26."]
        #[must_use]
        #[inline(always)]
        pub const fn s26(&self) -> bool {
            let val = (self.0 >> 26usize) & 0x01;
            val != 0
        }
        #[doc = "desc S26."]
        #[inline(always)]
        pub const fn set_s26(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
        }
        #[doc = "desc S27."]
        #[must_use]
        #[inline(always)]
        pub const fn s27(&self) -> bool {
            let val = (self.0 >> 27usize) & 0x01;
            val != 0
        }
        #[doc = "desc S27."]
        #[inline(always)]
        pub const fn set_s27(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
        }
        #[doc = "desc S28."]
        #[must_use]
        #[inline(always)]
        pub const fn s28(&self) -> bool {
            let val = (self.0 >> 28usize) & 0x01;
            val != 0
        }
        #[doc = "desc S28."]
        #[inline(always)]
        pub const fn set_s28(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
        }
        #[doc = "desc S29."]
        #[must_use]
        #[inline(always)]
        pub const fn s29(&self) -> bool {
            let val = (self.0 >> 29usize) & 0x01;
            val != 0
        }
        #[doc = "desc S29."]
        #[inline(always)]
        pub const fn set_s29(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
        }
        #[doc = "desc S30."]
        #[must_use]
        #[inline(always)]
        pub const fn s30(&self) -> bool {
            let val = (self.0 >> 30usize) & 0x01;
            val != 0
        }
        #[doc = "desc S30."]
        #[inline(always)]
        pub const fn set_s30(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
        }
        #[doc = "desc S31."]
        #[must_use]
        #[inline(always)]
        pub const fn s31(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[doc = "desc S31."]
        #[inline(always)]
        pub const fn set_s31(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for Poen0 {
        #[inline(always)]
        fn default() -> Poen0 {
            Poen0(0)
        }
    }
    impl core::fmt::Debug for Poen0 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Poen0")
                .field("s0", &self.s0())
                .field("s1", &self.s1())
                .field("s2", &self.s2())
                .field("s3", &self.s3())
                .field("s4", &self.s4())
                .field("s5", &self.s5())
                .field("s6", &self.s6())
                .field("s7", &self.s7())
                .field("s8", &self.s8())
                .field("s9", &self.s9())
                .field("s10", &self.s10())
                .field("s11", &self.s11())
                .field("s12", &self.s12())
                .field("s13", &self.s13())
                .field("s14", &self.s14())
                .field("s15", &self.s15())
                .field("s16", &self.s16())
                .field("s17", &self.s17())
                .field("s18", &self.s18())
                .field("s19", &self.s19())
                .field("s20", &self.s20())
                .field("s21", &self.s21())
                .field("s22", &self.s22())
                .field("s23", &self.s23())
                .field("s24", &self.s24())
                .field("s25", &self.s25())
                .field("s26", &self.s26())
                .field("s27", &self.s27())
                .field("s28", &self.s28())
                .field("s29", &self.s29())
                .field("s30", &self.s30())
                .field("s31", &self.s31())
                .finish()
        }
    }
    #[doc = "desc POEN1."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Poen1(pub u32);
    impl Poen1 {
        #[doc = "desc S32."]
        #[must_use]
        #[inline(always)]
        pub const fn s32(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "desc S32."]
        #[inline(always)]
        pub const fn set_s32(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "desc S33."]
        #[must_use]
        #[inline(always)]
        pub const fn s33(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "desc S33."]
        #[inline(always)]
        pub const fn set_s33(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "desc S34."]
        #[must_use]
        #[inline(always)]
        pub const fn s34(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "desc S34."]
        #[inline(always)]
        pub const fn set_s34(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "desc S35."]
        #[must_use]
        #[inline(always)]
        pub const fn s35(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "desc S35."]
        #[inline(always)]
        pub const fn set_s35(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "desc S36C7."]
        #[must_use]
        #[inline(always)]
        pub const fn s36c7(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "desc S36C7."]
        #[inline(always)]
        pub const fn set_s36c7(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "desc S37C6."]
        #[must_use]
        #[inline(always)]
        pub const fn s37c6(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "desc S37C6."]
        #[inline(always)]
        pub const fn set_s37c6(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "desc S38C5."]
        #[must_use]
        #[inline(always)]
        pub const fn s38c5(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "desc S38C5."]
        #[inline(always)]
        pub const fn set_s38c5(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "desc S39C4."]
        #[must_use]
        #[inline(always)]
        pub const fn s39c4(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "desc S39C4."]
        #[inline(always)]
        pub const fn set_s39c4(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "desc C0."]
        #[must_use]
        #[inline(always)]
        pub const fn c0(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "desc C0."]
        #[inline(always)]
        pub const fn set_c0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "desc C1."]
        #[must_use]
        #[inline(always)]
        pub const fn c1(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "desc C1."]
        #[inline(always)]
        pub const fn set_c1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "desc C2."]
        #[must_use]
        #[inline(always)]
        pub const fn c2(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "desc C2."]
        #[inline(always)]
        pub const fn set_c2(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[doc = "desc C3."]
        #[must_use]
        #[inline(always)]
        pub const fn c3(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "desc C3."]
        #[inline(always)]
        pub const fn set_c3(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[doc = "desc MUX."]
        #[must_use]
        #[inline(always)]
        pub const fn mux(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "desc MUX."]
        #[inline(always)]
        pub const fn set_mux(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
    }
    impl Default for Poen1 {
        #[inline(always)]
        fn default() -> Poen1 {
            Poen1(0)
        }
    }
    impl core::fmt::Debug for Poen1 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Poen1")
                .field("s32", &self.s32())
                .field("s33", &self.s33())
                .field("s34", &self.s34())
                .field("s35", &self.s35())
                .field("s36c7", &self.s36c7())
                .field("s37c6", &self.s37c6())
                .field("s38c5", &self.s38c5())
                .field("s39c4", &self.s39c4())
                .field("c0", &self.c0())
                .field("c1", &self.c1())
                .field("c2", &self.c2())
                .field("c3", &self.c3())
                .field("mux", &self.mux())
                .finish()
        }
    }
    #[doc = "desc RAM0."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ram0(pub u32);
    impl Ram0 {
        #[doc = "desc D0."]
        #[must_use]
        #[inline(always)]
        pub const fn d0(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "desc D0."]
        #[inline(always)]
        pub const fn set_d0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "desc D1."]
        #[must_use]
        #[inline(always)]
        pub const fn d1(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "desc D1."]
        #[inline(always)]
        pub const fn set_d1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "desc D2."]
        #[must_use]
        #[inline(always)]
        pub const fn d2(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "desc D2."]
        #[inline(always)]
        pub const fn set_d2(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "desc D3."]
        #[must_use]
        #[inline(always)]
        pub const fn d3(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "desc D3."]
        #[inline(always)]
        pub const fn set_d3(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "desc D4."]
        #[must_use]
        #[inline(always)]
        pub const fn d4(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "desc D4."]
        #[inline(always)]
        pub const fn set_d4(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "desc D5."]
        #[must_use]
        #[inline(always)]
        pub const fn d5(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "desc D5."]
        #[inline(always)]
        pub const fn set_d5(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "desc D6."]
        #[must_use]
        #[inline(always)]
        pub const fn d6(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "desc D6."]
        #[inline(always)]
        pub const fn set_d6(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "desc D7."]
        #[must_use]
        #[inline(always)]
        pub const fn d7(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "desc D7."]
        #[inline(always)]
        pub const fn set_d7(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "desc D8."]
        #[must_use]
        #[inline(always)]
        pub const fn d8(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "desc D8."]
        #[inline(always)]
        pub const fn set_d8(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "desc D9."]
        #[must_use]
        #[inline(always)]
        pub const fn d9(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "desc D9."]
        #[inline(always)]
        pub const fn set_d9(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "desc D10."]
        #[must_use]
        #[inline(always)]
        pub const fn d10(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "desc D10."]
        #[inline(always)]
        pub const fn set_d10(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[doc = "desc D11."]
        #[must_use]
        #[inline(always)]
        pub const fn d11(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "desc D11."]
        #[inline(always)]
        pub const fn set_d11(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[doc = "desc D12."]
        #[must_use]
        #[inline(always)]
        pub const fn d12(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "desc D12."]
        #[inline(always)]
        pub const fn set_d12(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "desc D13."]
        #[must_use]
        #[inline(always)]
        pub const fn d13(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[doc = "desc D13."]
        #[inline(always)]
        pub const fn set_d13(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[doc = "desc D14."]
        #[must_use]
        #[inline(always)]
        pub const fn d14(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "desc D14."]
        #[inline(always)]
        pub const fn set_d14(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[doc = "desc D15."]
        #[must_use]
        #[inline(always)]
        pub const fn d15(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "desc D15."]
        #[inline(always)]
        pub const fn set_d15(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
        #[doc = "desc D16."]
        #[must_use]
        #[inline(always)]
        pub const fn d16(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "desc D16."]
        #[inline(always)]
        pub const fn set_d16(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "desc D17."]
        #[must_use]
        #[inline(always)]
        pub const fn d17(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "desc D17."]
        #[inline(always)]
        pub const fn set_d17(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[doc = "desc D18."]
        #[must_use]
        #[inline(always)]
        pub const fn d18(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[doc = "desc D18."]
        #[inline(always)]
        pub const fn set_d18(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
        #[doc = "desc D19."]
        #[must_use]
        #[inline(always)]
        pub const fn d19(&self) -> bool {
            let val = (self.0 >> 19usize) & 0x01;
            val != 0
        }
        #[doc = "desc D19."]
        #[inline(always)]
        pub const fn set_d19(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
        }
        #[doc = "desc D20."]
        #[must_use]
        #[inline(always)]
        pub const fn d20(&self) -> bool {
            let val = (self.0 >> 20usize) & 0x01;
            val != 0
        }
        #[doc = "desc D20."]
        #[inline(always)]
        pub const fn set_d20(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
        }
        #[doc = "desc D21."]
        #[must_use]
        #[inline(always)]
        pub const fn d21(&self) -> bool {
            let val = (self.0 >> 21usize) & 0x01;
            val != 0
        }
        #[doc = "desc D21."]
        #[inline(always)]
        pub const fn set_d21(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
        }
        #[doc = "desc D22."]
        #[must_use]
        #[inline(always)]
        pub const fn d22(&self) -> bool {
            let val = (self.0 >> 22usize) & 0x01;
            val != 0
        }
        #[doc = "desc D22."]
        #[inline(always)]
        pub const fn set_d22(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
        }
        #[doc = "desc D23."]
        #[must_use]
        #[inline(always)]
        pub const fn d23(&self) -> bool {
            let val = (self.0 >> 23usize) & 0x01;
            val != 0
        }
        #[doc = "desc D23."]
        #[inline(always)]
        pub const fn set_d23(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
        }
        #[doc = "desc D24."]
        #[must_use]
        #[inline(always)]
        pub const fn d24(&self) -> bool {
            let val = (self.0 >> 24usize) & 0x01;
            val != 0
        }
        #[doc = "desc D24."]
        #[inline(always)]
        pub const fn set_d24(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
        }
        #[doc = "desc D25."]
        #[must_use]
        #[inline(always)]
        pub const fn d25(&self) -> bool {
            let val = (self.0 >> 25usize) & 0x01;
            val != 0
        }
        #[doc = "desc D25."]
        #[inline(always)]
        pub const fn set_d25(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
        }
        #[doc = "desc D26."]
        #[must_use]
        #[inline(always)]
        pub const fn d26(&self) -> bool {
            let val = (self.0 >> 26usize) & 0x01;
            val != 0
        }
        #[doc = "desc D26."]
        #[inline(always)]
        pub const fn set_d26(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
        }
        #[doc = "desc D27."]
        #[must_use]
        #[inline(always)]
        pub const fn d27(&self) -> bool {
            let val = (self.0 >> 27usize) & 0x01;
            val != 0
        }
        #[doc = "desc D27."]
        #[inline(always)]
        pub const fn set_d27(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
        }
        #[doc = "desc D28."]
        #[must_use]
        #[inline(always)]
        pub const fn d28(&self) -> bool {
            let val = (self.0 >> 28usize) & 0x01;
            val != 0
        }
        #[doc = "desc D28."]
        #[inline(always)]
        pub const fn set_d28(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
        }
        #[doc = "desc D29."]
        #[must_use]
        #[inline(always)]
        pub const fn d29(&self) -> bool {
            let val = (self.0 >> 29usize) & 0x01;
            val != 0
        }
        #[doc = "desc D29."]
        #[inline(always)]
        pub const fn set_d29(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
        }
        #[doc = "desc D30."]
        #[must_use]
        #[inline(always)]
        pub const fn d30(&self) -> bool {
            let val = (self.0 >> 30usize) & 0x01;
            val != 0
        }
        #[doc = "desc D30."]
        #[inline(always)]
        pub const fn set_d30(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
        }
        #[doc = "desc D31."]
        #[must_use]
        #[inline(always)]
        pub const fn d31(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[doc = "desc D31."]
        #[inline(always)]
        pub const fn set_d31(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for Ram0 {
        #[inline(always)]
        fn default() -> Ram0 {
            Ram0(0)
        }
    }
    impl core::fmt::Debug for Ram0 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Ram0")
                .field("d0", &self.d0())
                .field("d1", &self.d1())
                .field("d2", &self.d2())
                .field("d3", &self.d3())
                .field("d4", &self.d4())
                .field("d5", &self.d5())
                .field("d6", &self.d6())
                .field("d7", &self.d7())
                .field("d8", &self.d8())
                .field("d9", &self.d9())
                .field("d10", &self.d10())
                .field("d11", &self.d11())
                .field("d12", &self.d12())
                .field("d13", &self.d13())
                .field("d14", &self.d14())
                .field("d15", &self.d15())
                .field("d16", &self.d16())
                .field("d17", &self.d17())
                .field("d18", &self.d18())
                .field("d19", &self.d19())
                .field("d20", &self.d20())
                .field("d21", &self.d21())
                .field("d22", &self.d22())
                .field("d23", &self.d23())
                .field("d24", &self.d24())
                .field("d25", &self.d25())
                .field("d26", &self.d26())
                .field("d27", &self.d27())
                .field("d28", &self.d28())
                .field("d29", &self.d29())
                .field("d30", &self.d30())
                .field("d31", &self.d31())
                .finish()
        }
    }
    #[doc = "desc RAM1."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ram1(pub u32);
    impl Ram1 {
        #[doc = "desc D0."]
        #[must_use]
        #[inline(always)]
        pub const fn d0(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "desc D0."]
        #[inline(always)]
        pub const fn set_d0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "desc D1."]
        #[must_use]
        #[inline(always)]
        pub const fn d1(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "desc D1."]
        #[inline(always)]
        pub const fn set_d1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "desc D2."]
        #[must_use]
        #[inline(always)]
        pub const fn d2(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "desc D2."]
        #[inline(always)]
        pub const fn set_d2(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "desc D3."]
        #[must_use]
        #[inline(always)]
        pub const fn d3(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "desc D3."]
        #[inline(always)]
        pub const fn set_d3(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "desc D4."]
        #[must_use]
        #[inline(always)]
        pub const fn d4(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "desc D4."]
        #[inline(always)]
        pub const fn set_d4(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "desc D5."]
        #[must_use]
        #[inline(always)]
        pub const fn d5(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "desc D5."]
        #[inline(always)]
        pub const fn set_d5(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "desc D6."]
        #[must_use]
        #[inline(always)]
        pub const fn d6(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "desc D6."]
        #[inline(always)]
        pub const fn set_d6(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "desc D7."]
        #[must_use]
        #[inline(always)]
        pub const fn d7(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "desc D7."]
        #[inline(always)]
        pub const fn set_d7(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "desc D8."]
        #[must_use]
        #[inline(always)]
        pub const fn d8(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "desc D8."]
        #[inline(always)]
        pub const fn set_d8(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "desc D9."]
        #[must_use]
        #[inline(always)]
        pub const fn d9(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "desc D9."]
        #[inline(always)]
        pub const fn set_d9(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "desc D10."]
        #[must_use]
        #[inline(always)]
        pub const fn d10(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "desc D10."]
        #[inline(always)]
        pub const fn set_d10(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[doc = "desc D11."]
        #[must_use]
        #[inline(always)]
        pub const fn d11(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "desc D11."]
        #[inline(always)]
        pub const fn set_d11(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[doc = "desc D12."]
        #[must_use]
        #[inline(always)]
        pub const fn d12(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "desc D12."]
        #[inline(always)]
        pub const fn set_d12(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "desc D13."]
        #[must_use]
        #[inline(always)]
        pub const fn d13(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[doc = "desc D13."]
        #[inline(always)]
        pub const fn set_d13(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[doc = "desc D14."]
        #[must_use]
        #[inline(always)]
        pub const fn d14(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "desc D14."]
        #[inline(always)]
        pub const fn set_d14(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[doc = "desc D15."]
        #[must_use]
        #[inline(always)]
        pub const fn d15(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "desc D15."]
        #[inline(always)]
        pub const fn set_d15(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
        #[doc = "desc D16."]
        #[must_use]
        #[inline(always)]
        pub const fn d16(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "desc D16."]
        #[inline(always)]
        pub const fn set_d16(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "desc D17."]
        #[must_use]
        #[inline(always)]
        pub const fn d17(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "desc D17."]
        #[inline(always)]
        pub const fn set_d17(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[doc = "desc D18."]
        #[must_use]
        #[inline(always)]
        pub const fn d18(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[doc = "desc D18."]
        #[inline(always)]
        pub const fn set_d18(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
        #[doc = "desc D19."]
        #[must_use]
        #[inline(always)]
        pub const fn d19(&self) -> bool {
            let val = (self.0 >> 19usize) & 0x01;
            val != 0
        }
        #[doc = "desc D19."]
        #[inline(always)]
        pub const fn set_d19(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
        }
        #[doc = "desc D20."]
        #[must_use]
        #[inline(always)]
        pub const fn d20(&self) -> bool {
            let val = (self.0 >> 20usize) & 0x01;
            val != 0
        }
        #[doc = "desc D20."]
        #[inline(always)]
        pub const fn set_d20(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
        }
        #[doc = "desc D21."]
        #[must_use]
        #[inline(always)]
        pub const fn d21(&self) -> bool {
            let val = (self.0 >> 21usize) & 0x01;
            val != 0
        }
        #[doc = "desc D21."]
        #[inline(always)]
        pub const fn set_d21(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
        }
        #[doc = "desc D22."]
        #[must_use]
        #[inline(always)]
        pub const fn d22(&self) -> bool {
            let val = (self.0 >> 22usize) & 0x01;
            val != 0
        }
        #[doc = "desc D22."]
        #[inline(always)]
        pub const fn set_d22(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
        }
        #[doc = "desc D23."]
        #[must_use]
        #[inline(always)]
        pub const fn d23(&self) -> bool {
            let val = (self.0 >> 23usize) & 0x01;
            val != 0
        }
        #[doc = "desc D23."]
        #[inline(always)]
        pub const fn set_d23(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
        }
        #[doc = "desc D24."]
        #[must_use]
        #[inline(always)]
        pub const fn d24(&self) -> bool {
            let val = (self.0 >> 24usize) & 0x01;
            val != 0
        }
        #[doc = "desc D24."]
        #[inline(always)]
        pub const fn set_d24(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
        }
        #[doc = "desc D25."]
        #[must_use]
        #[inline(always)]
        pub const fn d25(&self) -> bool {
            let val = (self.0 >> 25usize) & 0x01;
            val != 0
        }
        #[doc = "desc D25."]
        #[inline(always)]
        pub const fn set_d25(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
        }
        #[doc = "desc D26."]
        #[must_use]
        #[inline(always)]
        pub const fn d26(&self) -> bool {
            let val = (self.0 >> 26usize) & 0x01;
            val != 0
        }
        #[doc = "desc D26."]
        #[inline(always)]
        pub const fn set_d26(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
        }
        #[doc = "desc D27."]
        #[must_use]
        #[inline(always)]
        pub const fn d27(&self) -> bool {
            let val = (self.0 >> 27usize) & 0x01;
            val != 0
        }
        #[doc = "desc D27."]
        #[inline(always)]
        pub const fn set_d27(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
        }
        #[doc = "desc D28."]
        #[must_use]
        #[inline(always)]
        pub const fn d28(&self) -> bool {
            let val = (self.0 >> 28usize) & 0x01;
            val != 0
        }
        #[doc = "desc D28."]
        #[inline(always)]
        pub const fn set_d28(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
        }
        #[doc = "desc D29."]
        #[must_use]
        #[inline(always)]
        pub const fn d29(&self) -> bool {
            let val = (self.0 >> 29usize) & 0x01;
            val != 0
        }
        #[doc = "desc D29."]
        #[inline(always)]
        pub const fn set_d29(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
        }
        #[doc = "desc D30."]
        #[must_use]
        #[inline(always)]
        pub const fn d30(&self) -> bool {
            let val = (self.0 >> 30usize) & 0x01;
            val != 0
        }
        #[doc = "desc D30."]
        #[inline(always)]
        pub const fn set_d30(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
        }
        #[doc = "desc D31."]
        #[must_use]
        #[inline(always)]
        pub const fn d31(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[doc = "desc D31."]
        #[inline(always)]
        pub const fn set_d31(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for Ram1 {
        #[inline(always)]
        fn default() -> Ram1 {
            Ram1(0)
        }
    }
    impl core::fmt::Debug for Ram1 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Ram1")
                .field("d0", &self.d0())
                .field("d1", &self.d1())
                .field("d2", &self.d2())
                .field("d3", &self.d3())
                .field("d4", &self.d4())
                .field("d5", &self.d5())
                .field("d6", &self.d6())
                .field("d7", &self.d7())
                .field("d8", &self.d8())
                .field("d9", &self.d9())
                .field("d10", &self.d10())
                .field("d11", &self.d11())
                .field("d12", &self.d12())
                .field("d13", &self.d13())
                .field("d14", &self.d14())
                .field("d15", &self.d15())
                .field("d16", &self.d16())
                .field("d17", &self.d17())
                .field("d18", &self.d18())
                .field("d19", &self.d19())
                .field("d20", &self.d20())
                .field("d21", &self.d21())
                .field("d22", &self.d22())
                .field("d23", &self.d23())
                .field("d24", &self.d24())
                .field("d25", &self.d25())
                .field("d26", &self.d26())
                .field("d27", &self.d27())
                .field("d28", &self.d28())
                .field("d29", &self.d29())
                .field("d30", &self.d30())
                .field("d31", &self.d31())
                .finish()
        }
    }
    #[doc = "desc RAM2."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ram2(pub u32);
    impl Ram2 {
        #[doc = "desc D0."]
        #[must_use]
        #[inline(always)]
        pub const fn d0(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "desc D0."]
        #[inline(always)]
        pub const fn set_d0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "desc D1."]
        #[must_use]
        #[inline(always)]
        pub const fn d1(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "desc D1."]
        #[inline(always)]
        pub const fn set_d1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "desc D2."]
        #[must_use]
        #[inline(always)]
        pub const fn d2(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "desc D2."]
        #[inline(always)]
        pub const fn set_d2(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "desc D3."]
        #[must_use]
        #[inline(always)]
        pub const fn d3(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "desc D3."]
        #[inline(always)]
        pub const fn set_d3(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "desc D4."]
        #[must_use]
        #[inline(always)]
        pub const fn d4(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "desc D4."]
        #[inline(always)]
        pub const fn set_d4(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "desc D5."]
        #[must_use]
        #[inline(always)]
        pub const fn d5(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "desc D5."]
        #[inline(always)]
        pub const fn set_d5(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "desc D6."]
        #[must_use]
        #[inline(always)]
        pub const fn d6(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "desc D6."]
        #[inline(always)]
        pub const fn set_d6(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "desc D7."]
        #[must_use]
        #[inline(always)]
        pub const fn d7(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "desc D7."]
        #[inline(always)]
        pub const fn set_d7(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "desc D8."]
        #[must_use]
        #[inline(always)]
        pub const fn d8(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "desc D8."]
        #[inline(always)]
        pub const fn set_d8(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "desc D9."]
        #[must_use]
        #[inline(always)]
        pub const fn d9(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "desc D9."]
        #[inline(always)]
        pub const fn set_d9(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "desc D10."]
        #[must_use]
        #[inline(always)]
        pub const fn d10(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "desc D10."]
        #[inline(always)]
        pub const fn set_d10(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[doc = "desc D11."]
        #[must_use]
        #[inline(always)]
        pub const fn d11(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "desc D11."]
        #[inline(always)]
        pub const fn set_d11(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[doc = "desc D12."]
        #[must_use]
        #[inline(always)]
        pub const fn d12(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "desc D12."]
        #[inline(always)]
        pub const fn set_d12(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "desc D13."]
        #[must_use]
        #[inline(always)]
        pub const fn d13(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[doc = "desc D13."]
        #[inline(always)]
        pub const fn set_d13(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[doc = "desc D14."]
        #[must_use]
        #[inline(always)]
        pub const fn d14(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "desc D14."]
        #[inline(always)]
        pub const fn set_d14(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[doc = "desc D15."]
        #[must_use]
        #[inline(always)]
        pub const fn d15(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "desc D15."]
        #[inline(always)]
        pub const fn set_d15(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
        #[doc = "desc D16."]
        #[must_use]
        #[inline(always)]
        pub const fn d16(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "desc D16."]
        #[inline(always)]
        pub const fn set_d16(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "desc D17."]
        #[must_use]
        #[inline(always)]
        pub const fn d17(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "desc D17."]
        #[inline(always)]
        pub const fn set_d17(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[doc = "desc D18."]
        #[must_use]
        #[inline(always)]
        pub const fn d18(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[doc = "desc D18."]
        #[inline(always)]
        pub const fn set_d18(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
        #[doc = "desc D19."]
        #[must_use]
        #[inline(always)]
        pub const fn d19(&self) -> bool {
            let val = (self.0 >> 19usize) & 0x01;
            val != 0
        }
        #[doc = "desc D19."]
        #[inline(always)]
        pub const fn set_d19(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
        }
        #[doc = "desc D20."]
        #[must_use]
        #[inline(always)]
        pub const fn d20(&self) -> bool {
            let val = (self.0 >> 20usize) & 0x01;
            val != 0
        }
        #[doc = "desc D20."]
        #[inline(always)]
        pub const fn set_d20(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
        }
        #[doc = "desc D21."]
        #[must_use]
        #[inline(always)]
        pub const fn d21(&self) -> bool {
            let val = (self.0 >> 21usize) & 0x01;
            val != 0
        }
        #[doc = "desc D21."]
        #[inline(always)]
        pub const fn set_d21(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
        }
        #[doc = "desc D22."]
        #[must_use]
        #[inline(always)]
        pub const fn d22(&self) -> bool {
            let val = (self.0 >> 22usize) & 0x01;
            val != 0
        }
        #[doc = "desc D22."]
        #[inline(always)]
        pub const fn set_d22(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
        }
        #[doc = "desc D23."]
        #[must_use]
        #[inline(always)]
        pub const fn d23(&self) -> bool {
            let val = (self.0 >> 23usize) & 0x01;
            val != 0
        }
        #[doc = "desc D23."]
        #[inline(always)]
        pub const fn set_d23(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
        }
        #[doc = "desc D24."]
        #[must_use]
        #[inline(always)]
        pub const fn d24(&self) -> bool {
            let val = (self.0 >> 24usize) & 0x01;
            val != 0
        }
        #[doc = "desc D24."]
        #[inline(always)]
        pub const fn set_d24(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
        }
        #[doc = "desc D25."]
        #[must_use]
        #[inline(always)]
        pub const fn d25(&self) -> bool {
            let val = (self.0 >> 25usize) & 0x01;
            val != 0
        }
        #[doc = "desc D25."]
        #[inline(always)]
        pub const fn set_d25(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
        }
        #[doc = "desc D26."]
        #[must_use]
        #[inline(always)]
        pub const fn d26(&self) -> bool {
            let val = (self.0 >> 26usize) & 0x01;
            val != 0
        }
        #[doc = "desc D26."]
        #[inline(always)]
        pub const fn set_d26(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
        }
        #[doc = "desc D27."]
        #[must_use]
        #[inline(always)]
        pub const fn d27(&self) -> bool {
            let val = (self.0 >> 27usize) & 0x01;
            val != 0
        }
        #[doc = "desc D27."]
        #[inline(always)]
        pub const fn set_d27(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
        }
        #[doc = "desc D28."]
        #[must_use]
        #[inline(always)]
        pub const fn d28(&self) -> bool {
            let val = (self.0 >> 28usize) & 0x01;
            val != 0
        }
        #[doc = "desc D28."]
        #[inline(always)]
        pub const fn set_d28(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
        }
        #[doc = "desc D29."]
        #[must_use]
        #[inline(always)]
        pub const fn d29(&self) -> bool {
            let val = (self.0 >> 29usize) & 0x01;
            val != 0
        }
        #[doc = "desc D29."]
        #[inline(always)]
        pub const fn set_d29(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
        }
        #[doc = "desc D30."]
        #[must_use]
        #[inline(always)]
        pub const fn d30(&self) -> bool {
            let val = (self.0 >> 30usize) & 0x01;
            val != 0
        }
        #[doc = "desc D30."]
        #[inline(always)]
        pub const fn set_d30(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
        }
        #[doc = "desc D31."]
        #[must_use]
        #[inline(always)]
        pub const fn d31(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[doc = "desc D31."]
        #[inline(always)]
        pub const fn set_d31(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for Ram2 {
        #[inline(always)]
        fn default() -> Ram2 {
            Ram2(0)
        }
    }
    impl core::fmt::Debug for Ram2 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Ram2")
                .field("d0", &self.d0())
                .field("d1", &self.d1())
                .field("d2", &self.d2())
                .field("d3", &self.d3())
                .field("d4", &self.d4())
                .field("d5", &self.d5())
                .field("d6", &self.d6())
                .field("d7", &self.d7())
                .field("d8", &self.d8())
                .field("d9", &self.d9())
                .field("d10", &self.d10())
                .field("d11", &self.d11())
                .field("d12", &self.d12())
                .field("d13", &self.d13())
                .field("d14", &self.d14())
                .field("d15", &self.d15())
                .field("d16", &self.d16())
                .field("d17", &self.d17())
                .field("d18", &self.d18())
                .field("d19", &self.d19())
                .field("d20", &self.d20())
                .field("d21", &self.d21())
                .field("d22", &self.d22())
                .field("d23", &self.d23())
                .field("d24", &self.d24())
                .field("d25", &self.d25())
                .field("d26", &self.d26())
                .field("d27", &self.d27())
                .field("d28", &self.d28())
                .field("d29", &self.d29())
                .field("d30", &self.d30())
                .field("d31", &self.d31())
                .finish()
        }
    }
    #[doc = "desc RAM3."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ram3(pub u32);
    impl Ram3 {
        #[doc = "desc D0."]
        #[must_use]
        #[inline(always)]
        pub const fn d0(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "desc D0."]
        #[inline(always)]
        pub const fn set_d0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "desc D1."]
        #[must_use]
        #[inline(always)]
        pub const fn d1(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "desc D1."]
        #[inline(always)]
        pub const fn set_d1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "desc D2."]
        #[must_use]
        #[inline(always)]
        pub const fn d2(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "desc D2."]
        #[inline(always)]
        pub const fn set_d2(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "desc D3."]
        #[must_use]
        #[inline(always)]
        pub const fn d3(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "desc D3."]
        #[inline(always)]
        pub const fn set_d3(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "desc D4."]
        #[must_use]
        #[inline(always)]
        pub const fn d4(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "desc D4."]
        #[inline(always)]
        pub const fn set_d4(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "desc D5."]
        #[must_use]
        #[inline(always)]
        pub const fn d5(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "desc D5."]
        #[inline(always)]
        pub const fn set_d5(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "desc D6."]
        #[must_use]
        #[inline(always)]
        pub const fn d6(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "desc D6."]
        #[inline(always)]
        pub const fn set_d6(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "desc D7."]
        #[must_use]
        #[inline(always)]
        pub const fn d7(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "desc D7."]
        #[inline(always)]
        pub const fn set_d7(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "desc D8."]
        #[must_use]
        #[inline(always)]
        pub const fn d8(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "desc D8."]
        #[inline(always)]
        pub const fn set_d8(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "desc D9."]
        #[must_use]
        #[inline(always)]
        pub const fn d9(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "desc D9."]
        #[inline(always)]
        pub const fn set_d9(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "desc D10."]
        #[must_use]
        #[inline(always)]
        pub const fn d10(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "desc D10."]
        #[inline(always)]
        pub const fn set_d10(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[doc = "desc D11."]
        #[must_use]
        #[inline(always)]
        pub const fn d11(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "desc D11."]
        #[inline(always)]
        pub const fn set_d11(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[doc = "desc D12."]
        #[must_use]
        #[inline(always)]
        pub const fn d12(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "desc D12."]
        #[inline(always)]
        pub const fn set_d12(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "desc D13."]
        #[must_use]
        #[inline(always)]
        pub const fn d13(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[doc = "desc D13."]
        #[inline(always)]
        pub const fn set_d13(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[doc = "desc D14."]
        #[must_use]
        #[inline(always)]
        pub const fn d14(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "desc D14."]
        #[inline(always)]
        pub const fn set_d14(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[doc = "desc D15."]
        #[must_use]
        #[inline(always)]
        pub const fn d15(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "desc D15."]
        #[inline(always)]
        pub const fn set_d15(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
        #[doc = "desc D16."]
        #[must_use]
        #[inline(always)]
        pub const fn d16(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "desc D16."]
        #[inline(always)]
        pub const fn set_d16(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "desc D17."]
        #[must_use]
        #[inline(always)]
        pub const fn d17(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "desc D17."]
        #[inline(always)]
        pub const fn set_d17(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[doc = "desc D18."]
        #[must_use]
        #[inline(always)]
        pub const fn d18(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[doc = "desc D18."]
        #[inline(always)]
        pub const fn set_d18(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
        #[doc = "desc D19."]
        #[must_use]
        #[inline(always)]
        pub const fn d19(&self) -> bool {
            let val = (self.0 >> 19usize) & 0x01;
            val != 0
        }
        #[doc = "desc D19."]
        #[inline(always)]
        pub const fn set_d19(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
        }
        #[doc = "desc D20."]
        #[must_use]
        #[inline(always)]
        pub const fn d20(&self) -> bool {
            let val = (self.0 >> 20usize) & 0x01;
            val != 0
        }
        #[doc = "desc D20."]
        #[inline(always)]
        pub const fn set_d20(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
        }
        #[doc = "desc D21."]
        #[must_use]
        #[inline(always)]
        pub const fn d21(&self) -> bool {
            let val = (self.0 >> 21usize) & 0x01;
            val != 0
        }
        #[doc = "desc D21."]
        #[inline(always)]
        pub const fn set_d21(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
        }
        #[doc = "desc D22."]
        #[must_use]
        #[inline(always)]
        pub const fn d22(&self) -> bool {
            let val = (self.0 >> 22usize) & 0x01;
            val != 0
        }
        #[doc = "desc D22."]
        #[inline(always)]
        pub const fn set_d22(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
        }
        #[doc = "desc D23."]
        #[must_use]
        #[inline(always)]
        pub const fn d23(&self) -> bool {
            let val = (self.0 >> 23usize) & 0x01;
            val != 0
        }
        #[doc = "desc D23."]
        #[inline(always)]
        pub const fn set_d23(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
        }
        #[doc = "desc D24."]
        #[must_use]
        #[inline(always)]
        pub const fn d24(&self) -> bool {
            let val = (self.0 >> 24usize) & 0x01;
            val != 0
        }
        #[doc = "desc D24."]
        #[inline(always)]
        pub const fn set_d24(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
        }
        #[doc = "desc D25."]
        #[must_use]
        #[inline(always)]
        pub const fn d25(&self) -> bool {
            let val = (self.0 >> 25usize) & 0x01;
            val != 0
        }
        #[doc = "desc D25."]
        #[inline(always)]
        pub const fn set_d25(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
        }
        #[doc = "desc D26."]
        #[must_use]
        #[inline(always)]
        pub const fn d26(&self) -> bool {
            let val = (self.0 >> 26usize) & 0x01;
            val != 0
        }
        #[doc = "desc D26."]
        #[inline(always)]
        pub const fn set_d26(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
        }
        #[doc = "desc D27."]
        #[must_use]
        #[inline(always)]
        pub const fn d27(&self) -> bool {
            let val = (self.0 >> 27usize) & 0x01;
            val != 0
        }
        #[doc = "desc D27."]
        #[inline(always)]
        pub const fn set_d27(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
        }
        #[doc = "desc D28."]
        #[must_use]
        #[inline(always)]
        pub const fn d28(&self) -> bool {
            let val = (self.0 >> 28usize) & 0x01;
            val != 0
        }
        #[doc = "desc D28."]
        #[inline(always)]
        pub const fn set_d28(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
        }
        #[doc = "desc D29."]
        #[must_use]
        #[inline(always)]
        pub const fn d29(&self) -> bool {
            let val = (self.0 >> 29usize) & 0x01;
            val != 0
        }
        #[doc = "desc D29."]
        #[inline(always)]
        pub const fn set_d29(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
        }
        #[doc = "desc D30."]
        #[must_use]
        #[inline(always)]
        pub const fn d30(&self) -> bool {
            let val = (self.0 >> 30usize) & 0x01;
            val != 0
        }
        #[doc = "desc D30."]
        #[inline(always)]
        pub const fn set_d30(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
        }
        #[doc = "desc D31."]
        #[must_use]
        #[inline(always)]
        pub const fn d31(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[doc = "desc D31."]
        #[inline(always)]
        pub const fn set_d31(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for Ram3 {
        #[inline(always)]
        fn default() -> Ram3 {
            Ram3(0)
        }
    }
    impl core::fmt::Debug for Ram3 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Ram3")
                .field("d0", &self.d0())
                .field("d1", &self.d1())
                .field("d2", &self.d2())
                .field("d3", &self.d3())
                .field("d4", &self.d4())
                .field("d5", &self.d5())
                .field("d6", &self.d6())
                .field("d7", &self.d7())
                .field("d8", &self.d8())
                .field("d9", &self.d9())
                .field("d10", &self.d10())
                .field("d11", &self.d11())
                .field("d12", &self.d12())
                .field("d13", &self.d13())
                .field("d14", &self.d14())
                .field("d15", &self.d15())
                .field("d16", &self.d16())
                .field("d17", &self.d17())
                .field("d18", &self.d18())
                .field("d19", &self.d19())
                .field("d20", &self.d20())
                .field("d21", &self.d21())
                .field("d22", &self.d22())
                .field("d23", &self.d23())
                .field("d24", &self.d24())
                .field("d25", &self.d25())
                .field("d26", &self.d26())
                .field("d27", &self.d27())
                .field("d28", &self.d28())
                .field("d29", &self.d29())
                .field("d30", &self.d30())
                .field("d31", &self.d31())
                .finish()
        }
    }
    #[doc = "desc RAM4."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ram4(pub u32);
    impl Ram4 {
        #[doc = "desc D0."]
        #[must_use]
        #[inline(always)]
        pub const fn d0(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "desc D0."]
        #[inline(always)]
        pub const fn set_d0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "desc D1."]
        #[must_use]
        #[inline(always)]
        pub const fn d1(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "desc D1."]
        #[inline(always)]
        pub const fn set_d1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "desc D2."]
        #[must_use]
        #[inline(always)]
        pub const fn d2(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "desc D2."]
        #[inline(always)]
        pub const fn set_d2(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "desc D3."]
        #[must_use]
        #[inline(always)]
        pub const fn d3(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "desc D3."]
        #[inline(always)]
        pub const fn set_d3(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "desc D4."]
        #[must_use]
        #[inline(always)]
        pub const fn d4(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "desc D4."]
        #[inline(always)]
        pub const fn set_d4(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "desc D5."]
        #[must_use]
        #[inline(always)]
        pub const fn d5(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "desc D5."]
        #[inline(always)]
        pub const fn set_d5(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "desc D6."]
        #[must_use]
        #[inline(always)]
        pub const fn d6(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "desc D6."]
        #[inline(always)]
        pub const fn set_d6(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "desc D7."]
        #[must_use]
        #[inline(always)]
        pub const fn d7(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "desc D7."]
        #[inline(always)]
        pub const fn set_d7(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "desc D8."]
        #[must_use]
        #[inline(always)]
        pub const fn d8(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "desc D8."]
        #[inline(always)]
        pub const fn set_d8(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "desc D9."]
        #[must_use]
        #[inline(always)]
        pub const fn d9(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "desc D9."]
        #[inline(always)]
        pub const fn set_d9(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "desc D10."]
        #[must_use]
        #[inline(always)]
        pub const fn d10(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "desc D10."]
        #[inline(always)]
        pub const fn set_d10(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[doc = "desc D11."]
        #[must_use]
        #[inline(always)]
        pub const fn d11(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "desc D11."]
        #[inline(always)]
        pub const fn set_d11(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[doc = "desc D12."]
        #[must_use]
        #[inline(always)]
        pub const fn d12(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "desc D12."]
        #[inline(always)]
        pub const fn set_d12(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "desc D13."]
        #[must_use]
        #[inline(always)]
        pub const fn d13(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[doc = "desc D13."]
        #[inline(always)]
        pub const fn set_d13(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[doc = "desc D14."]
        #[must_use]
        #[inline(always)]
        pub const fn d14(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "desc D14."]
        #[inline(always)]
        pub const fn set_d14(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[doc = "desc D15."]
        #[must_use]
        #[inline(always)]
        pub const fn d15(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "desc D15."]
        #[inline(always)]
        pub const fn set_d15(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
        #[doc = "desc D16."]
        #[must_use]
        #[inline(always)]
        pub const fn d16(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "desc D16."]
        #[inline(always)]
        pub const fn set_d16(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "desc D17."]
        #[must_use]
        #[inline(always)]
        pub const fn d17(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "desc D17."]
        #[inline(always)]
        pub const fn set_d17(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[doc = "desc D18."]
        #[must_use]
        #[inline(always)]
        pub const fn d18(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[doc = "desc D18."]
        #[inline(always)]
        pub const fn set_d18(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
        #[doc = "desc D19."]
        #[must_use]
        #[inline(always)]
        pub const fn d19(&self) -> bool {
            let val = (self.0 >> 19usize) & 0x01;
            val != 0
        }
        #[doc = "desc D19."]
        #[inline(always)]
        pub const fn set_d19(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
        }
        #[doc = "desc D20."]
        #[must_use]
        #[inline(always)]
        pub const fn d20(&self) -> bool {
            let val = (self.0 >> 20usize) & 0x01;
            val != 0
        }
        #[doc = "desc D20."]
        #[inline(always)]
        pub const fn set_d20(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
        }
        #[doc = "desc D21."]
        #[must_use]
        #[inline(always)]
        pub const fn d21(&self) -> bool {
            let val = (self.0 >> 21usize) & 0x01;
            val != 0
        }
        #[doc = "desc D21."]
        #[inline(always)]
        pub const fn set_d21(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
        }
        #[doc = "desc D22."]
        #[must_use]
        #[inline(always)]
        pub const fn d22(&self) -> bool {
            let val = (self.0 >> 22usize) & 0x01;
            val != 0
        }
        #[doc = "desc D22."]
        #[inline(always)]
        pub const fn set_d22(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
        }
        #[doc = "desc D23."]
        #[must_use]
        #[inline(always)]
        pub const fn d23(&self) -> bool {
            let val = (self.0 >> 23usize) & 0x01;
            val != 0
        }
        #[doc = "desc D23."]
        #[inline(always)]
        pub const fn set_d23(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
        }
        #[doc = "desc D24."]
        #[must_use]
        #[inline(always)]
        pub const fn d24(&self) -> bool {
            let val = (self.0 >> 24usize) & 0x01;
            val != 0
        }
        #[doc = "desc D24."]
        #[inline(always)]
        pub const fn set_d24(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
        }
        #[doc = "desc D25."]
        #[must_use]
        #[inline(always)]
        pub const fn d25(&self) -> bool {
            let val = (self.0 >> 25usize) & 0x01;
            val != 0
        }
        #[doc = "desc D25."]
        #[inline(always)]
        pub const fn set_d25(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
        }
        #[doc = "desc D26."]
        #[must_use]
        #[inline(always)]
        pub const fn d26(&self) -> bool {
            let val = (self.0 >> 26usize) & 0x01;
            val != 0
        }
        #[doc = "desc D26."]
        #[inline(always)]
        pub const fn set_d26(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
        }
        #[doc = "desc D27."]
        #[must_use]
        #[inline(always)]
        pub const fn d27(&self) -> bool {
            let val = (self.0 >> 27usize) & 0x01;
            val != 0
        }
        #[doc = "desc D27."]
        #[inline(always)]
        pub const fn set_d27(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
        }
        #[doc = "desc D28."]
        #[must_use]
        #[inline(always)]
        pub const fn d28(&self) -> bool {
            let val = (self.0 >> 28usize) & 0x01;
            val != 0
        }
        #[doc = "desc D28."]
        #[inline(always)]
        pub const fn set_d28(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
        }
        #[doc = "desc D29."]
        #[must_use]
        #[inline(always)]
        pub const fn d29(&self) -> bool {
            let val = (self.0 >> 29usize) & 0x01;
            val != 0
        }
        #[doc = "desc D29."]
        #[inline(always)]
        pub const fn set_d29(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
        }
        #[doc = "desc D30."]
        #[must_use]
        #[inline(always)]
        pub const fn d30(&self) -> bool {
            let val = (self.0 >> 30usize) & 0x01;
            val != 0
        }
        #[doc = "desc D30."]
        #[inline(always)]
        pub const fn set_d30(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
        }
        #[doc = "desc D31."]
        #[must_use]
        #[inline(always)]
        pub const fn d31(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[doc = "desc D31."]
        #[inline(always)]
        pub const fn set_d31(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for Ram4 {
        #[inline(always)]
        fn default() -> Ram4 {
            Ram4(0)
        }
    }
    impl core::fmt::Debug for Ram4 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Ram4")
                .field("d0", &self.d0())
                .field("d1", &self.d1())
                .field("d2", &self.d2())
                .field("d3", &self.d3())
                .field("d4", &self.d4())
                .field("d5", &self.d5())
                .field("d6", &self.d6())
                .field("d7", &self.d7())
                .field("d8", &self.d8())
                .field("d9", &self.d9())
                .field("d10", &self.d10())
                .field("d11", &self.d11())
                .field("d12", &self.d12())
                .field("d13", &self.d13())
                .field("d14", &self.d14())
                .field("d15", &self.d15())
                .field("d16", &self.d16())
                .field("d17", &self.d17())
                .field("d18", &self.d18())
                .field("d19", &self.d19())
                .field("d20", &self.d20())
                .field("d21", &self.d21())
                .field("d22", &self.d22())
                .field("d23", &self.d23())
                .field("d24", &self.d24())
                .field("d25", &self.d25())
                .field("d26", &self.d26())
                .field("d27", &self.d27())
                .field("d28", &self.d28())
                .field("d29", &self.d29())
                .field("d30", &self.d30())
                .field("d31", &self.d31())
                .finish()
        }
    }
    #[doc = "desc RAM5."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ram5(pub u32);
    impl Ram5 {
        #[doc = "desc D0."]
        #[must_use]
        #[inline(always)]
        pub const fn d0(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "desc D0."]
        #[inline(always)]
        pub const fn set_d0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "desc D1."]
        #[must_use]
        #[inline(always)]
        pub const fn d1(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "desc D1."]
        #[inline(always)]
        pub const fn set_d1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "desc D2."]
        #[must_use]
        #[inline(always)]
        pub const fn d2(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "desc D2."]
        #[inline(always)]
        pub const fn set_d2(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "desc D3."]
        #[must_use]
        #[inline(always)]
        pub const fn d3(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "desc D3."]
        #[inline(always)]
        pub const fn set_d3(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "desc D4."]
        #[must_use]
        #[inline(always)]
        pub const fn d4(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "desc D4."]
        #[inline(always)]
        pub const fn set_d4(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "desc D5."]
        #[must_use]
        #[inline(always)]
        pub const fn d5(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "desc D5."]
        #[inline(always)]
        pub const fn set_d5(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "desc D6."]
        #[must_use]
        #[inline(always)]
        pub const fn d6(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "desc D6."]
        #[inline(always)]
        pub const fn set_d6(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "desc D7."]
        #[must_use]
        #[inline(always)]
        pub const fn d7(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "desc D7."]
        #[inline(always)]
        pub const fn set_d7(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "desc D8."]
        #[must_use]
        #[inline(always)]
        pub const fn d8(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "desc D8."]
        #[inline(always)]
        pub const fn set_d8(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "desc D9."]
        #[must_use]
        #[inline(always)]
        pub const fn d9(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "desc D9."]
        #[inline(always)]
        pub const fn set_d9(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "desc D10."]
        #[must_use]
        #[inline(always)]
        pub const fn d10(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "desc D10."]
        #[inline(always)]
        pub const fn set_d10(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[doc = "desc D11."]
        #[must_use]
        #[inline(always)]
        pub const fn d11(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "desc D11."]
        #[inline(always)]
        pub const fn set_d11(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[doc = "desc D12."]
        #[must_use]
        #[inline(always)]
        pub const fn d12(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "desc D12."]
        #[inline(always)]
        pub const fn set_d12(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "desc D13."]
        #[must_use]
        #[inline(always)]
        pub const fn d13(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[doc = "desc D13."]
        #[inline(always)]
        pub const fn set_d13(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[doc = "desc D14."]
        #[must_use]
        #[inline(always)]
        pub const fn d14(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "desc D14."]
        #[inline(always)]
        pub const fn set_d14(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[doc = "desc D15."]
        #[must_use]
        #[inline(always)]
        pub const fn d15(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "desc D15."]
        #[inline(always)]
        pub const fn set_d15(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
        #[doc = "desc D16."]
        #[must_use]
        #[inline(always)]
        pub const fn d16(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "desc D16."]
        #[inline(always)]
        pub const fn set_d16(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "desc D17."]
        #[must_use]
        #[inline(always)]
        pub const fn d17(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "desc D17."]
        #[inline(always)]
        pub const fn set_d17(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[doc = "desc D18."]
        #[must_use]
        #[inline(always)]
        pub const fn d18(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[doc = "desc D18."]
        #[inline(always)]
        pub const fn set_d18(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
        #[doc = "desc D19."]
        #[must_use]
        #[inline(always)]
        pub const fn d19(&self) -> bool {
            let val = (self.0 >> 19usize) & 0x01;
            val != 0
        }
        #[doc = "desc D19."]
        #[inline(always)]
        pub const fn set_d19(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
        }
        #[doc = "desc D20."]
        #[must_use]
        #[inline(always)]
        pub const fn d20(&self) -> bool {
            let val = (self.0 >> 20usize) & 0x01;
            val != 0
        }
        #[doc = "desc D20."]
        #[inline(always)]
        pub const fn set_d20(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
        }
        #[doc = "desc D21."]
        #[must_use]
        #[inline(always)]
        pub const fn d21(&self) -> bool {
            let val = (self.0 >> 21usize) & 0x01;
            val != 0
        }
        #[doc = "desc D21."]
        #[inline(always)]
        pub const fn set_d21(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
        }
        #[doc = "desc D22."]
        #[must_use]
        #[inline(always)]
        pub const fn d22(&self) -> bool {
            let val = (self.0 >> 22usize) & 0x01;
            val != 0
        }
        #[doc = "desc D22."]
        #[inline(always)]
        pub const fn set_d22(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
        }
        #[doc = "desc D23."]
        #[must_use]
        #[inline(always)]
        pub const fn d23(&self) -> bool {
            let val = (self.0 >> 23usize) & 0x01;
            val != 0
        }
        #[doc = "desc D23."]
        #[inline(always)]
        pub const fn set_d23(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
        }
        #[doc = "desc D24."]
        #[must_use]
        #[inline(always)]
        pub const fn d24(&self) -> bool {
            let val = (self.0 >> 24usize) & 0x01;
            val != 0
        }
        #[doc = "desc D24."]
        #[inline(always)]
        pub const fn set_d24(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
        }
        #[doc = "desc D25."]
        #[must_use]
        #[inline(always)]
        pub const fn d25(&self) -> bool {
            let val = (self.0 >> 25usize) & 0x01;
            val != 0
        }
        #[doc = "desc D25."]
        #[inline(always)]
        pub const fn set_d25(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
        }
        #[doc = "desc D26."]
        #[must_use]
        #[inline(always)]
        pub const fn d26(&self) -> bool {
            let val = (self.0 >> 26usize) & 0x01;
            val != 0
        }
        #[doc = "desc D26."]
        #[inline(always)]
        pub const fn set_d26(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
        }
        #[doc = "desc D27."]
        #[must_use]
        #[inline(always)]
        pub const fn d27(&self) -> bool {
            let val = (self.0 >> 27usize) & 0x01;
            val != 0
        }
        #[doc = "desc D27."]
        #[inline(always)]
        pub const fn set_d27(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
        }
        #[doc = "desc D28."]
        #[must_use]
        #[inline(always)]
        pub const fn d28(&self) -> bool {
            let val = (self.0 >> 28usize) & 0x01;
            val != 0
        }
        #[doc = "desc D28."]
        #[inline(always)]
        pub const fn set_d28(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
        }
        #[doc = "desc D29."]
        #[must_use]
        #[inline(always)]
        pub const fn d29(&self) -> bool {
            let val = (self.0 >> 29usize) & 0x01;
            val != 0
        }
        #[doc = "desc D29."]
        #[inline(always)]
        pub const fn set_d29(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
        }
        #[doc = "desc D30."]
        #[must_use]
        #[inline(always)]
        pub const fn d30(&self) -> bool {
            let val = (self.0 >> 30usize) & 0x01;
            val != 0
        }
        #[doc = "desc D30."]
        #[inline(always)]
        pub const fn set_d30(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
        }
        #[doc = "desc D31."]
        #[must_use]
        #[inline(always)]
        pub const fn d31(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[doc = "desc D31."]
        #[inline(always)]
        pub const fn set_d31(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for Ram5 {
        #[inline(always)]
        fn default() -> Ram5 {
            Ram5(0)
        }
    }
    impl core::fmt::Debug for Ram5 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Ram5")
                .field("d0", &self.d0())
                .field("d1", &self.d1())
                .field("d2", &self.d2())
                .field("d3", &self.d3())
                .field("d4", &self.d4())
                .field("d5", &self.d5())
                .field("d6", &self.d6())
                .field("d7", &self.d7())
                .field("d8", &self.d8())
                .field("d9", &self.d9())
                .field("d10", &self.d10())
                .field("d11", &self.d11())
                .field("d12", &self.d12())
                .field("d13", &self.d13())
                .field("d14", &self.d14())
                .field("d15", &self.d15())
                .field("d16", &self.d16())
                .field("d17", &self.d17())
                .field("d18", &self.d18())
                .field("d19", &self.d19())
                .field("d20", &self.d20())
                .field("d21", &self.d21())
                .field("d22", &self.d22())
                .field("d23", &self.d23())
                .field("d24", &self.d24())
                .field("d25", &self.d25())
                .field("d26", &self.d26())
                .field("d27", &self.d27())
                .field("d28", &self.d28())
                .field("d29", &self.d29())
                .field("d30", &self.d30())
                .field("d31", &self.d31())
                .finish()
        }
    }
    #[doc = "desc RAM6."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ram6(pub u32);
    impl Ram6 {
        #[doc = "desc D0."]
        #[must_use]
        #[inline(always)]
        pub const fn d0(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "desc D0."]
        #[inline(always)]
        pub const fn set_d0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "desc D1."]
        #[must_use]
        #[inline(always)]
        pub const fn d1(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "desc D1."]
        #[inline(always)]
        pub const fn set_d1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "desc D2."]
        #[must_use]
        #[inline(always)]
        pub const fn d2(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "desc D2."]
        #[inline(always)]
        pub const fn set_d2(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "desc D3."]
        #[must_use]
        #[inline(always)]
        pub const fn d3(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "desc D3."]
        #[inline(always)]
        pub const fn set_d3(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "desc D4."]
        #[must_use]
        #[inline(always)]
        pub const fn d4(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "desc D4."]
        #[inline(always)]
        pub const fn set_d4(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "desc D5."]
        #[must_use]
        #[inline(always)]
        pub const fn d5(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "desc D5."]
        #[inline(always)]
        pub const fn set_d5(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "desc D6."]
        #[must_use]
        #[inline(always)]
        pub const fn d6(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "desc D6."]
        #[inline(always)]
        pub const fn set_d6(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "desc D7."]
        #[must_use]
        #[inline(always)]
        pub const fn d7(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "desc D7."]
        #[inline(always)]
        pub const fn set_d7(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "desc D8."]
        #[must_use]
        #[inline(always)]
        pub const fn d8(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "desc D8."]
        #[inline(always)]
        pub const fn set_d8(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "desc D9."]
        #[must_use]
        #[inline(always)]
        pub const fn d9(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "desc D9."]
        #[inline(always)]
        pub const fn set_d9(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "desc D10."]
        #[must_use]
        #[inline(always)]
        pub const fn d10(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "desc D10."]
        #[inline(always)]
        pub const fn set_d10(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[doc = "desc D11."]
        #[must_use]
        #[inline(always)]
        pub const fn d11(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "desc D11."]
        #[inline(always)]
        pub const fn set_d11(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[doc = "desc D12."]
        #[must_use]
        #[inline(always)]
        pub const fn d12(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "desc D12."]
        #[inline(always)]
        pub const fn set_d12(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "desc D13."]
        #[must_use]
        #[inline(always)]
        pub const fn d13(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[doc = "desc D13."]
        #[inline(always)]
        pub const fn set_d13(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[doc = "desc D14."]
        #[must_use]
        #[inline(always)]
        pub const fn d14(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "desc D14."]
        #[inline(always)]
        pub const fn set_d14(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[doc = "desc D15."]
        #[must_use]
        #[inline(always)]
        pub const fn d15(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "desc D15."]
        #[inline(always)]
        pub const fn set_d15(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
        #[doc = "desc D16."]
        #[must_use]
        #[inline(always)]
        pub const fn d16(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "desc D16."]
        #[inline(always)]
        pub const fn set_d16(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "desc D17."]
        #[must_use]
        #[inline(always)]
        pub const fn d17(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "desc D17."]
        #[inline(always)]
        pub const fn set_d17(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[doc = "desc D18."]
        #[must_use]
        #[inline(always)]
        pub const fn d18(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[doc = "desc D18."]
        #[inline(always)]
        pub const fn set_d18(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
        #[doc = "desc D19."]
        #[must_use]
        #[inline(always)]
        pub const fn d19(&self) -> bool {
            let val = (self.0 >> 19usize) & 0x01;
            val != 0
        }
        #[doc = "desc D19."]
        #[inline(always)]
        pub const fn set_d19(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
        }
        #[doc = "desc D20."]
        #[must_use]
        #[inline(always)]
        pub const fn d20(&self) -> bool {
            let val = (self.0 >> 20usize) & 0x01;
            val != 0
        }
        #[doc = "desc D20."]
        #[inline(always)]
        pub const fn set_d20(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
        }
        #[doc = "desc D21."]
        #[must_use]
        #[inline(always)]
        pub const fn d21(&self) -> bool {
            let val = (self.0 >> 21usize) & 0x01;
            val != 0
        }
        #[doc = "desc D21."]
        #[inline(always)]
        pub const fn set_d21(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
        }
        #[doc = "desc D22."]
        #[must_use]
        #[inline(always)]
        pub const fn d22(&self) -> bool {
            let val = (self.0 >> 22usize) & 0x01;
            val != 0
        }
        #[doc = "desc D22."]
        #[inline(always)]
        pub const fn set_d22(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
        }
        #[doc = "desc D23."]
        #[must_use]
        #[inline(always)]
        pub const fn d23(&self) -> bool {
            let val = (self.0 >> 23usize) & 0x01;
            val != 0
        }
        #[doc = "desc D23."]
        #[inline(always)]
        pub const fn set_d23(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
        }
        #[doc = "desc D24."]
        #[must_use]
        #[inline(always)]
        pub const fn d24(&self) -> bool {
            let val = (self.0 >> 24usize) & 0x01;
            val != 0
        }
        #[doc = "desc D24."]
        #[inline(always)]
        pub const fn set_d24(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
        }
        #[doc = "desc D25."]
        #[must_use]
        #[inline(always)]
        pub const fn d25(&self) -> bool {
            let val = (self.0 >> 25usize) & 0x01;
            val != 0
        }
        #[doc = "desc D25."]
        #[inline(always)]
        pub const fn set_d25(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
        }
        #[doc = "desc D26."]
        #[must_use]
        #[inline(always)]
        pub const fn d26(&self) -> bool {
            let val = (self.0 >> 26usize) & 0x01;
            val != 0
        }
        #[doc = "desc D26."]
        #[inline(always)]
        pub const fn set_d26(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
        }
        #[doc = "desc D27."]
        #[must_use]
        #[inline(always)]
        pub const fn d27(&self) -> bool {
            let val = (self.0 >> 27usize) & 0x01;
            val != 0
        }
        #[doc = "desc D27."]
        #[inline(always)]
        pub const fn set_d27(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
        }
        #[doc = "desc D28."]
        #[must_use]
        #[inline(always)]
        pub const fn d28(&self) -> bool {
            let val = (self.0 >> 28usize) & 0x01;
            val != 0
        }
        #[doc = "desc D28."]
        #[inline(always)]
        pub const fn set_d28(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
        }
        #[doc = "desc D29."]
        #[must_use]
        #[inline(always)]
        pub const fn d29(&self) -> bool {
            let val = (self.0 >> 29usize) & 0x01;
            val != 0
        }
        #[doc = "desc D29."]
        #[inline(always)]
        pub const fn set_d29(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
        }
        #[doc = "desc D30."]
        #[must_use]
        #[inline(always)]
        pub const fn d30(&self) -> bool {
            let val = (self.0 >> 30usize) & 0x01;
            val != 0
        }
        #[doc = "desc D30."]
        #[inline(always)]
        pub const fn set_d30(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
        }
        #[doc = "desc D31."]
        #[must_use]
        #[inline(always)]
        pub const fn d31(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[doc = "desc D31."]
        #[inline(always)]
        pub const fn set_d31(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for Ram6 {
        #[inline(always)]
        fn default() -> Ram6 {
            Ram6(0)
        }
    }
    impl core::fmt::Debug for Ram6 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Ram6")
                .field("d0", &self.d0())
                .field("d1", &self.d1())
                .field("d2", &self.d2())
                .field("d3", &self.d3())
                .field("d4", &self.d4())
                .field("d5", &self.d5())
                .field("d6", &self.d6())
                .field("d7", &self.d7())
                .field("d8", &self.d8())
                .field("d9", &self.d9())
                .field("d10", &self.d10())
                .field("d11", &self.d11())
                .field("d12", &self.d12())
                .field("d13", &self.d13())
                .field("d14", &self.d14())
                .field("d15", &self.d15())
                .field("d16", &self.d16())
                .field("d17", &self.d17())
                .field("d18", &self.d18())
                .field("d19", &self.d19())
                .field("d20", &self.d20())
                .field("d21", &self.d21())
                .field("d22", &self.d22())
                .field("d23", &self.d23())
                .field("d24", &self.d24())
                .field("d25", &self.d25())
                .field("d26", &self.d26())
                .field("d27", &self.d27())
                .field("d28", &self.d28())
                .field("d29", &self.d29())
                .field("d30", &self.d30())
                .field("d31", &self.d31())
                .finish()
        }
    }
    #[doc = "desc RAM7."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ram7(pub u32);
    impl Ram7 {
        #[doc = "desc D0."]
        #[must_use]
        #[inline(always)]
        pub const fn d0(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "desc D0."]
        #[inline(always)]
        pub const fn set_d0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "desc D1."]
        #[must_use]
        #[inline(always)]
        pub const fn d1(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "desc D1."]
        #[inline(always)]
        pub const fn set_d1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "desc D2."]
        #[must_use]
        #[inline(always)]
        pub const fn d2(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "desc D2."]
        #[inline(always)]
        pub const fn set_d2(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "desc D3."]
        #[must_use]
        #[inline(always)]
        pub const fn d3(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "desc D3."]
        #[inline(always)]
        pub const fn set_d3(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "desc D4."]
        #[must_use]
        #[inline(always)]
        pub const fn d4(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "desc D4."]
        #[inline(always)]
        pub const fn set_d4(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "desc D5."]
        #[must_use]
        #[inline(always)]
        pub const fn d5(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "desc D5."]
        #[inline(always)]
        pub const fn set_d5(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "desc D6."]
        #[must_use]
        #[inline(always)]
        pub const fn d6(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "desc D6."]
        #[inline(always)]
        pub const fn set_d6(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "desc D7."]
        #[must_use]
        #[inline(always)]
        pub const fn d7(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "desc D7."]
        #[inline(always)]
        pub const fn set_d7(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "desc D8."]
        #[must_use]
        #[inline(always)]
        pub const fn d8(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "desc D8."]
        #[inline(always)]
        pub const fn set_d8(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "desc D9."]
        #[must_use]
        #[inline(always)]
        pub const fn d9(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "desc D9."]
        #[inline(always)]
        pub const fn set_d9(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "desc D10."]
        #[must_use]
        #[inline(always)]
        pub const fn d10(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "desc D10."]
        #[inline(always)]
        pub const fn set_d10(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[doc = "desc D11."]
        #[must_use]
        #[inline(always)]
        pub const fn d11(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "desc D11."]
        #[inline(always)]
        pub const fn set_d11(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[doc = "desc D12."]
        #[must_use]
        #[inline(always)]
        pub const fn d12(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "desc D12."]
        #[inline(always)]
        pub const fn set_d12(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "desc D13."]
        #[must_use]
        #[inline(always)]
        pub const fn d13(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[doc = "desc D13."]
        #[inline(always)]
        pub const fn set_d13(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[doc = "desc D14."]
        #[must_use]
        #[inline(always)]
        pub const fn d14(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "desc D14."]
        #[inline(always)]
        pub const fn set_d14(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[doc = "desc D15."]
        #[must_use]
        #[inline(always)]
        pub const fn d15(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "desc D15."]
        #[inline(always)]
        pub const fn set_d15(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
        #[doc = "desc D16."]
        #[must_use]
        #[inline(always)]
        pub const fn d16(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "desc D16."]
        #[inline(always)]
        pub const fn set_d16(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "desc D17."]
        #[must_use]
        #[inline(always)]
        pub const fn d17(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "desc D17."]
        #[inline(always)]
        pub const fn set_d17(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[doc = "desc D18."]
        #[must_use]
        #[inline(always)]
        pub const fn d18(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[doc = "desc D18."]
        #[inline(always)]
        pub const fn set_d18(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
        #[doc = "desc D19."]
        #[must_use]
        #[inline(always)]
        pub const fn d19(&self) -> bool {
            let val = (self.0 >> 19usize) & 0x01;
            val != 0
        }
        #[doc = "desc D19."]
        #[inline(always)]
        pub const fn set_d19(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
        }
        #[doc = "desc D20."]
        #[must_use]
        #[inline(always)]
        pub const fn d20(&self) -> bool {
            let val = (self.0 >> 20usize) & 0x01;
            val != 0
        }
        #[doc = "desc D20."]
        #[inline(always)]
        pub const fn set_d20(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
        }
        #[doc = "desc D21."]
        #[must_use]
        #[inline(always)]
        pub const fn d21(&self) -> bool {
            let val = (self.0 >> 21usize) & 0x01;
            val != 0
        }
        #[doc = "desc D21."]
        #[inline(always)]
        pub const fn set_d21(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
        }
        #[doc = "desc D22."]
        #[must_use]
        #[inline(always)]
        pub const fn d22(&self) -> bool {
            let val = (self.0 >> 22usize) & 0x01;
            val != 0
        }
        #[doc = "desc D22."]
        #[inline(always)]
        pub const fn set_d22(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
        }
        #[doc = "desc D23."]
        #[must_use]
        #[inline(always)]
        pub const fn d23(&self) -> bool {
            let val = (self.0 >> 23usize) & 0x01;
            val != 0
        }
        #[doc = "desc D23."]
        #[inline(always)]
        pub const fn set_d23(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
        }
        #[doc = "desc D24."]
        #[must_use]
        #[inline(always)]
        pub const fn d24(&self) -> bool {
            let val = (self.0 >> 24usize) & 0x01;
            val != 0
        }
        #[doc = "desc D24."]
        #[inline(always)]
        pub const fn set_d24(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
        }
        #[doc = "desc D25."]
        #[must_use]
        #[inline(always)]
        pub const fn d25(&self) -> bool {
            let val = (self.0 >> 25usize) & 0x01;
            val != 0
        }
        #[doc = "desc D25."]
        #[inline(always)]
        pub const fn set_d25(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
        }
        #[doc = "desc D26."]
        #[must_use]
        #[inline(always)]
        pub const fn d26(&self) -> bool {
            let val = (self.0 >> 26usize) & 0x01;
            val != 0
        }
        #[doc = "desc D26."]
        #[inline(always)]
        pub const fn set_d26(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
        }
        #[doc = "desc D27."]
        #[must_use]
        #[inline(always)]
        pub const fn d27(&self) -> bool {
            let val = (self.0 >> 27usize) & 0x01;
            val != 0
        }
        #[doc = "desc D27."]
        #[inline(always)]
        pub const fn set_d27(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
        }
        #[doc = "desc D28."]
        #[must_use]
        #[inline(always)]
        pub const fn d28(&self) -> bool {
            let val = (self.0 >> 28usize) & 0x01;
            val != 0
        }
        #[doc = "desc D28."]
        #[inline(always)]
        pub const fn set_d28(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
        }
        #[doc = "desc D29."]
        #[must_use]
        #[inline(always)]
        pub const fn d29(&self) -> bool {
            let val = (self.0 >> 29usize) & 0x01;
            val != 0
        }
        #[doc = "desc D29."]
        #[inline(always)]
        pub const fn set_d29(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
        }
        #[doc = "desc D30."]
        #[must_use]
        #[inline(always)]
        pub const fn d30(&self) -> bool {
            let val = (self.0 >> 30usize) & 0x01;
            val != 0
        }
        #[doc = "desc D30."]
        #[inline(always)]
        pub const fn set_d30(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
        }
        #[doc = "desc D31."]
        #[must_use]
        #[inline(always)]
        pub const fn d31(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[doc = "desc D31."]
        #[inline(always)]
        pub const fn set_d31(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for Ram7 {
        #[inline(always)]
        fn default() -> Ram7 {
            Ram7(0)
        }
    }
    impl core::fmt::Debug for Ram7 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Ram7")
                .field("d0", &self.d0())
                .field("d1", &self.d1())
                .field("d2", &self.d2())
                .field("d3", &self.d3())
                .field("d4", &self.d4())
                .field("d5", &self.d5())
                .field("d6", &self.d6())
                .field("d7", &self.d7())
                .field("d8", &self.d8())
                .field("d9", &self.d9())
                .field("d10", &self.d10())
                .field("d11", &self.d11())
                .field("d12", &self.d12())
                .field("d13", &self.d13())
                .field("d14", &self.d14())
                .field("d15", &self.d15())
                .field("d16", &self.d16())
                .field("d17", &self.d17())
                .field("d18", &self.d18())
                .field("d19", &self.d19())
                .field("d20", &self.d20())
                .field("d21", &self.d21())
                .field("d22", &self.d22())
                .field("d23", &self.d23())
                .field("d24", &self.d24())
                .field("d25", &self.d25())
                .field("d26", &self.d26())
                .field("d27", &self.d27())
                .field("d28", &self.d28())
                .field("d29", &self.d29())
                .field("d30", &self.d30())
                .field("d31", &self.d31())
                .finish()
        }
    }
    #[doc = "desc RAM8."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ram8(pub u32);
    impl Ram8 {
        #[doc = "desc D0."]
        #[must_use]
        #[inline(always)]
        pub const fn d0(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "desc D0."]
        #[inline(always)]
        pub const fn set_d0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "desc D1."]
        #[must_use]
        #[inline(always)]
        pub const fn d1(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "desc D1."]
        #[inline(always)]
        pub const fn set_d1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "desc D2."]
        #[must_use]
        #[inline(always)]
        pub const fn d2(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "desc D2."]
        #[inline(always)]
        pub const fn set_d2(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "desc D3."]
        #[must_use]
        #[inline(always)]
        pub const fn d3(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "desc D3."]
        #[inline(always)]
        pub const fn set_d3(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "desc D4."]
        #[must_use]
        #[inline(always)]
        pub const fn d4(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "desc D4."]
        #[inline(always)]
        pub const fn set_d4(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "desc D5."]
        #[must_use]
        #[inline(always)]
        pub const fn d5(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "desc D5."]
        #[inline(always)]
        pub const fn set_d5(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "desc D6."]
        #[must_use]
        #[inline(always)]
        pub const fn d6(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "desc D6."]
        #[inline(always)]
        pub const fn set_d6(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "desc D7."]
        #[must_use]
        #[inline(always)]
        pub const fn d7(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "desc D7."]
        #[inline(always)]
        pub const fn set_d7(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
    }
    impl Default for Ram8 {
        #[inline(always)]
        fn default() -> Ram8 {
            Ram8(0)
        }
    }
    impl core::fmt::Debug for Ram8 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Ram8")
                .field("d0", &self.d0())
                .field("d1", &self.d1())
                .field("d2", &self.d2())
                .field("d3", &self.d3())
                .field("d4", &self.d4())
                .field("d5", &self.d5())
                .field("d6", &self.d6())
                .field("d7", &self.d7())
                .finish()
        }
    }
    #[doc = "desc RAM9."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ram9(pub u32);
    impl Ram9 {
        #[doc = "desc D0."]
        #[must_use]
        #[inline(always)]
        pub const fn d0(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "desc D0."]
        #[inline(always)]
        pub const fn set_d0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "desc D1."]
        #[must_use]
        #[inline(always)]
        pub const fn d1(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "desc D1."]
        #[inline(always)]
        pub const fn set_d1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "desc D2."]
        #[must_use]
        #[inline(always)]
        pub const fn d2(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "desc D2."]
        #[inline(always)]
        pub const fn set_d2(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "desc D3."]
        #[must_use]
        #[inline(always)]
        pub const fn d3(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "desc D3."]
        #[inline(always)]
        pub const fn set_d3(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "desc D4."]
        #[must_use]
        #[inline(always)]
        pub const fn d4(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "desc D4."]
        #[inline(always)]
        pub const fn set_d4(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "desc D5."]
        #[must_use]
        #[inline(always)]
        pub const fn d5(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "desc D5."]
        #[inline(always)]
        pub const fn set_d5(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "desc D6."]
        #[must_use]
        #[inline(always)]
        pub const fn d6(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "desc D6."]
        #[inline(always)]
        pub const fn set_d6(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "desc D7."]
        #[must_use]
        #[inline(always)]
        pub const fn d7(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "desc D7."]
        #[inline(always)]
        pub const fn set_d7(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
    }
    impl Default for Ram9 {
        #[inline(always)]
        fn default() -> Ram9 {
            Ram9(0)
        }
    }
    impl core::fmt::Debug for Ram9 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Ram9")
                .field("d0", &self.d0())
                .field("d1", &self.d1())
                .field("d2", &self.d2())
                .field("d3", &self.d3())
                .field("d4", &self.d4())
                .field("d5", &self.d5())
                .field("d6", &self.d6())
                .field("d7", &self.d7())
                .finish()
        }
    }
    #[doc = "desc RAMA."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Rama(pub u32);
    impl Rama {
        #[doc = "desc D0."]
        #[must_use]
        #[inline(always)]
        pub const fn d0(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "desc D0."]
        #[inline(always)]
        pub const fn set_d0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "desc D1."]
        #[must_use]
        #[inline(always)]
        pub const fn d1(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "desc D1."]
        #[inline(always)]
        pub const fn set_d1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "desc D2."]
        #[must_use]
        #[inline(always)]
        pub const fn d2(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "desc D2."]
        #[inline(always)]
        pub const fn set_d2(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "desc D3."]
        #[must_use]
        #[inline(always)]
        pub const fn d3(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "desc D3."]
        #[inline(always)]
        pub const fn set_d3(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "desc D4."]
        #[must_use]
        #[inline(always)]
        pub const fn d4(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "desc D4."]
        #[inline(always)]
        pub const fn set_d4(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "desc D5."]
        #[must_use]
        #[inline(always)]
        pub const fn d5(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "desc D5."]
        #[inline(always)]
        pub const fn set_d5(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "desc D6."]
        #[must_use]
        #[inline(always)]
        pub const fn d6(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "desc D6."]
        #[inline(always)]
        pub const fn set_d6(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "desc D7."]
        #[must_use]
        #[inline(always)]
        pub const fn d7(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "desc D7."]
        #[inline(always)]
        pub const fn set_d7(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
    }
    impl Default for Rama {
        #[inline(always)]
        fn default() -> Rama {
            Rama(0)
        }
    }
    impl core::fmt::Debug for Rama {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Rama")
                .field("d0", &self.d0())
                .field("d1", &self.d1())
                .field("d2", &self.d2())
                .field("d3", &self.d3())
                .field("d4", &self.d4())
                .field("d5", &self.d5())
                .field("d6", &self.d6())
                .field("d7", &self.d7())
                .finish()
        }
    }
    #[doc = "desc RAMB."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ramb(pub u32);
    impl Ramb {
        #[doc = "desc D0."]
        #[must_use]
        #[inline(always)]
        pub const fn d0(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "desc D0."]
        #[inline(always)]
        pub const fn set_d0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "desc D1."]
        #[must_use]
        #[inline(always)]
        pub const fn d1(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "desc D1."]
        #[inline(always)]
        pub const fn set_d1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "desc D2."]
        #[must_use]
        #[inline(always)]
        pub const fn d2(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "desc D2."]
        #[inline(always)]
        pub const fn set_d2(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "desc D3."]
        #[must_use]
        #[inline(always)]
        pub const fn d3(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "desc D3."]
        #[inline(always)]
        pub const fn set_d3(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "desc D4."]
        #[must_use]
        #[inline(always)]
        pub const fn d4(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "desc D4."]
        #[inline(always)]
        pub const fn set_d4(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "desc D5."]
        #[must_use]
        #[inline(always)]
        pub const fn d5(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "desc D5."]
        #[inline(always)]
        pub const fn set_d5(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "desc D6."]
        #[must_use]
        #[inline(always)]
        pub const fn d6(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "desc D6."]
        #[inline(always)]
        pub const fn set_d6(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "desc D7."]
        #[must_use]
        #[inline(always)]
        pub const fn d7(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "desc D7."]
        #[inline(always)]
        pub const fn set_d7(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
    }
    impl Default for Ramb {
        #[inline(always)]
        fn default() -> Ramb {
            Ramb(0)
        }
    }
    impl core::fmt::Debug for Ramb {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Ramb")
                .field("d0", &self.d0())
                .field("d1", &self.d1())
                .field("d2", &self.d2())
                .field("d3", &self.d3())
                .field("d4", &self.d4())
                .field("d5", &self.d5())
                .field("d6", &self.d6())
                .field("d7", &self.d7())
                .finish()
        }
    }
    #[doc = "desc RAMC."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ramc(pub u32);
    impl Ramc {
        #[doc = "desc D0."]
        #[must_use]
        #[inline(always)]
        pub const fn d0(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "desc D0."]
        #[inline(always)]
        pub const fn set_d0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "desc D1."]
        #[must_use]
        #[inline(always)]
        pub const fn d1(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "desc D1."]
        #[inline(always)]
        pub const fn set_d1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "desc D2."]
        #[must_use]
        #[inline(always)]
        pub const fn d2(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "desc D2."]
        #[inline(always)]
        pub const fn set_d2(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "desc D3."]
        #[must_use]
        #[inline(always)]
        pub const fn d3(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "desc D3."]
        #[inline(always)]
        pub const fn set_d3(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "desc D4."]
        #[must_use]
        #[inline(always)]
        pub const fn d4(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "desc D4."]
        #[inline(always)]
        pub const fn set_d4(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "desc D5."]
        #[must_use]
        #[inline(always)]
        pub const fn d5(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "desc D5."]
        #[inline(always)]
        pub const fn set_d5(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "desc D6."]
        #[must_use]
        #[inline(always)]
        pub const fn d6(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "desc D6."]
        #[inline(always)]
        pub const fn set_d6(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "desc D7."]
        #[must_use]
        #[inline(always)]
        pub const fn d7(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "desc D7."]
        #[inline(always)]
        pub const fn set_d7(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
    }
    impl Default for Ramc {
        #[inline(always)]
        fn default() -> Ramc {
            Ramc(0)
        }
    }
    impl core::fmt::Debug for Ramc {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Ramc")
                .field("d0", &self.d0())
                .field("d1", &self.d1())
                .field("d2", &self.d2())
                .field("d3", &self.d3())
                .field("d4", &self.d4())
                .field("d5", &self.d5())
                .field("d6", &self.d6())
                .field("d7", &self.d7())
                .finish()
        }
    }
    #[doc = "desc RAMD."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ramd(pub u32);
    impl Ramd {
        #[doc = "desc D0."]
        #[must_use]
        #[inline(always)]
        pub const fn d0(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "desc D0."]
        #[inline(always)]
        pub const fn set_d0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "desc D1."]
        #[must_use]
        #[inline(always)]
        pub const fn d1(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "desc D1."]
        #[inline(always)]
        pub const fn set_d1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "desc D2."]
        #[must_use]
        #[inline(always)]
        pub const fn d2(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "desc D2."]
        #[inline(always)]
        pub const fn set_d2(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "desc D3."]
        #[must_use]
        #[inline(always)]
        pub const fn d3(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "desc D3."]
        #[inline(always)]
        pub const fn set_d3(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "desc D4."]
        #[must_use]
        #[inline(always)]
        pub const fn d4(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "desc D4."]
        #[inline(always)]
        pub const fn set_d4(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "desc D5."]
        #[must_use]
        #[inline(always)]
        pub const fn d5(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "desc D5."]
        #[inline(always)]
        pub const fn set_d5(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "desc D6."]
        #[must_use]
        #[inline(always)]
        pub const fn d6(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "desc D6."]
        #[inline(always)]
        pub const fn set_d6(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "desc D7."]
        #[must_use]
        #[inline(always)]
        pub const fn d7(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "desc D7."]
        #[inline(always)]
        pub const fn set_d7(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
    }
    impl Default for Ramd {
        #[inline(always)]
        fn default() -> Ramd {
            Ramd(0)
        }
    }
    impl core::fmt::Debug for Ramd {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Ramd")
                .field("d0", &self.d0())
                .field("d1", &self.d1())
                .field("d2", &self.d2())
                .field("d3", &self.d3())
                .field("d4", &self.d4())
                .field("d5", &self.d5())
                .field("d6", &self.d6())
                .field("d7", &self.d7())
                .finish()
        }
    }
    #[doc = "desc RAME."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Rame(pub u32);
    impl Rame {
        #[doc = "desc D0."]
        #[must_use]
        #[inline(always)]
        pub const fn d0(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "desc D0."]
        #[inline(always)]
        pub const fn set_d0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "desc D1."]
        #[must_use]
        #[inline(always)]
        pub const fn d1(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "desc D1."]
        #[inline(always)]
        pub const fn set_d1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "desc D2."]
        #[must_use]
        #[inline(always)]
        pub const fn d2(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "desc D2."]
        #[inline(always)]
        pub const fn set_d2(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "desc D3."]
        #[must_use]
        #[inline(always)]
        pub const fn d3(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "desc D3."]
        #[inline(always)]
        pub const fn set_d3(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "desc D4."]
        #[must_use]
        #[inline(always)]
        pub const fn d4(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "desc D4."]
        #[inline(always)]
        pub const fn set_d4(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "desc D5."]
        #[must_use]
        #[inline(always)]
        pub const fn d5(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "desc D5."]
        #[inline(always)]
        pub const fn set_d5(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "desc D6."]
        #[must_use]
        #[inline(always)]
        pub const fn d6(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "desc D6."]
        #[inline(always)]
        pub const fn set_d6(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "desc D7."]
        #[must_use]
        #[inline(always)]
        pub const fn d7(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "desc D7."]
        #[inline(always)]
        pub const fn set_d7(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
    }
    impl Default for Rame {
        #[inline(always)]
        fn default() -> Rame {
            Rame(0)
        }
    }
    impl core::fmt::Debug for Rame {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Rame")
                .field("d0", &self.d0())
                .field("d1", &self.d1())
                .field("d2", &self.d2())
                .field("d3", &self.d3())
                .field("d4", &self.d4())
                .field("d5", &self.d5())
                .field("d6", &self.d6())
                .field("d7", &self.d7())
                .finish()
        }
    }
    #[doc = "desc RAMF."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ramf(pub u32);
    impl Ramf {
        #[doc = "desc D0."]
        #[must_use]
        #[inline(always)]
        pub const fn d0(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "desc D0."]
        #[inline(always)]
        pub const fn set_d0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "desc D1."]
        #[must_use]
        #[inline(always)]
        pub const fn d1(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "desc D1."]
        #[inline(always)]
        pub const fn set_d1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "desc D2."]
        #[must_use]
        #[inline(always)]
        pub const fn d2(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "desc D2."]
        #[inline(always)]
        pub const fn set_d2(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "desc D3."]
        #[must_use]
        #[inline(always)]
        pub const fn d3(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "desc D3."]
        #[inline(always)]
        pub const fn set_d3(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "desc D4."]
        #[must_use]
        #[inline(always)]
        pub const fn d4(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "desc D4."]
        #[inline(always)]
        pub const fn set_d4(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "desc D5."]
        #[must_use]
        #[inline(always)]
        pub const fn d5(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "desc D5."]
        #[inline(always)]
        pub const fn set_d5(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "desc D6."]
        #[must_use]
        #[inline(always)]
        pub const fn d6(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "desc D6."]
        #[inline(always)]
        pub const fn set_d6(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "desc D7."]
        #[must_use]
        #[inline(always)]
        pub const fn d7(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "desc D7."]
        #[inline(always)]
        pub const fn set_d7(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
    }
    impl Default for Ramf {
        #[inline(always)]
        fn default() -> Ramf {
            Ramf(0)
        }
    }
    impl core::fmt::Debug for Ramf {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Ramf")
                .field("d0", &self.d0())
                .field("d1", &self.d1())
                .field("d2", &self.d2())
                .field("d3", &self.d3())
                .field("d4", &self.d4())
                .field("d5", &self.d5())
                .field("d6", &self.d6())
                .field("d7", &self.d7())
                .finish()
        }
    }
}
