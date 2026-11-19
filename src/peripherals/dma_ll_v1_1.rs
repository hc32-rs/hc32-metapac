#![allow(clippy::missing_safety_doc)]
#![allow(clippy::identity_op)]
#![allow(clippy::unnecessary_cast)]
#![allow(clippy::erasing_op)]

#[doc = "desc DMA."]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dma {
    ptr: *mut u8,
}
unsafe impl Send for Dma {}
unsafe impl Sync for Dma {}
impl Dma {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "desc EN."]
    #[inline(always)]
    pub const fn en(self) -> crate::common::Reg<regs::En, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0usize) as _) }
    }
    #[doc = "desc INTSTAT0."]
    #[inline(always)]
    pub const fn intstat0(self) -> crate::common::Reg<regs::Intstat0, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x04usize) as _) }
    }
    #[doc = "desc INTSTAT1."]
    #[inline(always)]
    pub const fn intstat1(self) -> crate::common::Reg<regs::Intstat1, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x08usize) as _) }
    }
    #[doc = "desc INTMASK0."]
    #[inline(always)]
    pub const fn intmask0(self) -> crate::common::Reg<regs::Intmask0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0cusize) as _) }
    }
    #[doc = "desc INTMASK1."]
    #[inline(always)]
    pub const fn intmask1(self) -> crate::common::Reg<regs::Intmask1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x10usize) as _) }
    }
    #[doc = "desc INTCLR0."]
    #[inline(always)]
    pub const fn intclr0(self) -> crate::common::Reg<regs::Intclr0, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x14usize) as _) }
    }
    #[doc = "desc INTCLR1."]
    #[inline(always)]
    pub const fn intclr1(self) -> crate::common::Reg<regs::Intclr1, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x18usize) as _) }
    }
    #[doc = "desc CHEN."]
    #[inline(always)]
    pub const fn chen(self) -> crate::common::Reg<regs::Chen, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1cusize) as _) }
    }
    #[doc = "desc CHSTAT."]
    #[inline(always)]
    pub const fn chstat(self) -> crate::common::Reg<regs::Chstat, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x24usize) as _) }
    }
    #[doc = "desc BASE1_LLP."]
    #[inline(always)]
    pub const fn base1_llp(self) -> crate::common::Reg<regs::Base1Llp, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x30usize) as _) }
    }
    #[doc = "desc BASE2_LLP."]
    #[inline(always)]
    pub const fn base2_llp(self) -> crate::common::Reg<regs::Base2Llp, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x34usize) as _) }
    }
    #[doc = "desc CHENCLR."]
    #[inline(always)]
    pub const fn chenclr(self) -> crate::common::Reg<regs::Chenclr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x38usize) as _) }
    }
    #[doc = "desc SAR0."]
    #[inline(always)]
    pub const fn sar0(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x40usize) as _) }
    }
    #[doc = "desc DAR0."]
    #[inline(always)]
    pub const fn dar0(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x44usize) as _) }
    }
    #[doc = "desc CH0CTL0."]
    #[inline(always)]
    pub const fn ch0ctl0(self) -> crate::common::Reg<regs::Ch0ctl0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x48usize) as _) }
    }
    #[doc = "desc CH0CTL1."]
    #[inline(always)]
    pub const fn ch0ctl1(self) -> crate::common::Reg<regs::Ch0ctl1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x4cusize) as _) }
    }
    #[doc = "desc SAR1."]
    #[inline(always)]
    pub const fn sar1(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x80usize) as _) }
    }
    #[doc = "desc DAR1."]
    #[inline(always)]
    pub const fn dar1(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x84usize) as _) }
    }
    #[doc = "desc CH1CTL0."]
    #[inline(always)]
    pub const fn ch1ctl0(self) -> crate::common::Reg<regs::Ch1ctl0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x88usize) as _) }
    }
    #[doc = "desc CH1CTL1."]
    #[inline(always)]
    pub const fn ch1ctl1(self) -> crate::common::Reg<regs::Ch1ctl1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x8cusize) as _) }
    }
}
pub mod regs {
    #[doc = "desc BASE1_LLP."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Base1Llp(pub u32);
    impl Base1Llp {
        #[doc = "desc LLP."]
        #[must_use]
        #[inline(always)]
        pub const fn llp(&self) -> u32 {
            let val = (self.0 >> 14usize) & 0x0003_ffff;
            val as u32
        }
        #[doc = "desc LLP."]
        #[inline(always)]
        pub const fn set_llp(&mut self, val: u32) {
            self.0 =
                (self.0 & !(0x0003_ffff << 14usize)) | (((val as u32) & 0x0003_ffff) << 14usize);
        }
    }
    impl Default for Base1Llp {
        #[inline(always)]
        fn default() -> Base1Llp {
            Base1Llp(0)
        }
    }
    impl core::fmt::Debug for Base1Llp {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Base1Llp")
                .field("llp", &self.llp())
                .finish()
        }
    }
    #[doc = "desc BASE2_LLP."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Base2Llp(pub u32);
    impl Base2Llp {
        #[doc = "desc LLP."]
        #[must_use]
        #[inline(always)]
        pub const fn llp(&self) -> u32 {
            let val = (self.0 >> 14usize) & 0x0003_ffff;
            val as u32
        }
        #[doc = "desc LLP."]
        #[inline(always)]
        pub const fn set_llp(&mut self, val: u32) {
            self.0 =
                (self.0 & !(0x0003_ffff << 14usize)) | (((val as u32) & 0x0003_ffff) << 14usize);
        }
    }
    impl Default for Base2Llp {
        #[inline(always)]
        fn default() -> Base2Llp {
            Base2Llp(0)
        }
    }
    impl core::fmt::Debug for Base2Llp {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Base2Llp")
                .field("llp", &self.llp())
                .finish()
        }
    }
    #[doc = "desc CH0CTL0."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ch0ctl0(pub u32);
    impl Ch0ctl0 {
        #[doc = "desc BLKSIZE."]
        #[must_use]
        #[inline(always)]
        pub const fn blksize(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[doc = "desc BLKSIZE."]
        #[inline(always)]
        pub const fn set_blksize(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
        }
        #[doc = "desc CNT."]
        #[must_use]
        #[inline(always)]
        pub const fn cnt(&self) -> u16 {
            let val = (self.0 >> 8usize) & 0x03ff;
            val as u16
        }
        #[doc = "desc CNT."]
        #[inline(always)]
        pub const fn set_cnt(&mut self, val: u16) {
            self.0 = (self.0 & !(0x03ff << 8usize)) | (((val as u32) & 0x03ff) << 8usize);
        }
        #[doc = "desc LLP."]
        #[must_use]
        #[inline(always)]
        pub const fn llp(&self) -> u16 {
            let val = (self.0 >> 18usize) & 0x03ff;
            val as u16
        }
        #[doc = "desc LLP."]
        #[inline(always)]
        pub const fn set_llp(&mut self, val: u16) {
            self.0 = (self.0 & !(0x03ff << 18usize)) | (((val as u32) & 0x03ff) << 18usize);
        }
        #[doc = "desc LLPEN."]
        #[must_use]
        #[inline(always)]
        pub const fn llpen(&self) -> bool {
            let val = (self.0 >> 28usize) & 0x01;
            val != 0
        }
        #[doc = "desc LLPEN."]
        #[inline(always)]
        pub const fn set_llpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
        }
        #[doc = "desc LLPRUN."]
        #[must_use]
        #[inline(always)]
        pub const fn llprun(&self) -> bool {
            let val = (self.0 >> 29usize) & 0x01;
            val != 0
        }
        #[doc = "desc LLPRUN."]
        #[inline(always)]
        pub const fn set_llprun(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
        }
        #[doc = "desc HSIZE."]
        #[must_use]
        #[inline(always)]
        pub const fn hsize(&self) -> u8 {
            let val = (self.0 >> 30usize) & 0x03;
            val as u8
        }
        #[doc = "desc HSIZE."]
        #[inline(always)]
        pub const fn set_hsize(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 30usize)) | (((val as u32) & 0x03) << 30usize);
        }
    }
    impl Default for Ch0ctl0 {
        #[inline(always)]
        fn default() -> Ch0ctl0 {
            Ch0ctl0(0)
        }
    }
    impl core::fmt::Debug for Ch0ctl0 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Ch0ctl0")
                .field("blksize", &self.blksize())
                .field("cnt", &self.cnt())
                .field("llp", &self.llp())
                .field("llpen", &self.llpen())
                .field("llprun", &self.llprun())
                .field("hsize", &self.hsize())
                .finish()
        }
    }
    #[doc = "desc CH0CTL1."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ch0ctl1(pub u32);
    impl Ch0ctl1 {
        #[doc = "desc OFFSET."]
        #[must_use]
        #[inline(always)]
        pub const fn offset(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[doc = "desc OFFSET."]
        #[inline(always)]
        pub const fn set_offset(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
        }
        #[doc = "desc RPTNSCNT."]
        #[must_use]
        #[inline(always)]
        pub const fn rptnscnt(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0xff;
            val as u8
        }
        #[doc = "desc RPTNSCNT."]
        #[inline(always)]
        pub const fn set_rptnscnt(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 16usize)) | (((val as u32) & 0xff) << 16usize);
        }
        #[doc = "desc RPTNSEN."]
        #[must_use]
        #[inline(always)]
        pub const fn rptnsen(&self) -> bool {
            let val = (self.0 >> 24usize) & 0x01;
            val != 0
        }
        #[doc = "desc RPTNSEN."]
        #[inline(always)]
        pub const fn set_rptnsen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
        }
        #[doc = "desc RPTNSSEL."]
        #[must_use]
        #[inline(always)]
        pub const fn rptnssel(&self) -> u8 {
            let val = (self.0 >> 25usize) & 0x03;
            val as u8
        }
        #[doc = "desc RPTNSSEL."]
        #[inline(always)]
        pub const fn set_rptnssel(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 25usize)) | (((val as u32) & 0x03) << 25usize);
        }
        #[doc = "desc LLPSEL."]
        #[must_use]
        #[inline(always)]
        pub const fn llpsel(&self) -> bool {
            let val = (self.0 >> 27usize) & 0x01;
            val != 0
        }
        #[doc = "desc LLPSEL."]
        #[inline(always)]
        pub const fn set_llpsel(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
        }
        #[doc = "desc SINC."]
        #[must_use]
        #[inline(always)]
        pub const fn sinc(&self) -> u8 {
            let val = (self.0 >> 28usize) & 0x03;
            val as u8
        }
        #[doc = "desc SINC."]
        #[inline(always)]
        pub const fn set_sinc(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 28usize)) | (((val as u32) & 0x03) << 28usize);
        }
        #[doc = "desc DINC."]
        #[must_use]
        #[inline(always)]
        pub const fn dinc(&self) -> u8 {
            let val = (self.0 >> 30usize) & 0x03;
            val as u8
        }
        #[doc = "desc DINC."]
        #[inline(always)]
        pub const fn set_dinc(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 30usize)) | (((val as u32) & 0x03) << 30usize);
        }
    }
    impl Default for Ch0ctl1 {
        #[inline(always)]
        fn default() -> Ch0ctl1 {
            Ch0ctl1(0)
        }
    }
    impl core::fmt::Debug for Ch0ctl1 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Ch0ctl1")
                .field("offset", &self.offset())
                .field("rptnscnt", &self.rptnscnt())
                .field("rptnsen", &self.rptnsen())
                .field("rptnssel", &self.rptnssel())
                .field("llpsel", &self.llpsel())
                .field("sinc", &self.sinc())
                .field("dinc", &self.dinc())
                .finish()
        }
    }
    #[doc = "desc CH1CTL0."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ch1ctl0(pub u32);
    impl Ch1ctl0 {
        #[doc = "desc BLKSIZE."]
        #[must_use]
        #[inline(always)]
        pub const fn blksize(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[doc = "desc BLKSIZE."]
        #[inline(always)]
        pub const fn set_blksize(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
        }
        #[doc = "desc CNT."]
        #[must_use]
        #[inline(always)]
        pub const fn cnt(&self) -> u16 {
            let val = (self.0 >> 8usize) & 0x03ff;
            val as u16
        }
        #[doc = "desc CNT."]
        #[inline(always)]
        pub const fn set_cnt(&mut self, val: u16) {
            self.0 = (self.0 & !(0x03ff << 8usize)) | (((val as u32) & 0x03ff) << 8usize);
        }
        #[doc = "desc LLP."]
        #[must_use]
        #[inline(always)]
        pub const fn llp(&self) -> u16 {
            let val = (self.0 >> 18usize) & 0x03ff;
            val as u16
        }
        #[doc = "desc LLP."]
        #[inline(always)]
        pub const fn set_llp(&mut self, val: u16) {
            self.0 = (self.0 & !(0x03ff << 18usize)) | (((val as u32) & 0x03ff) << 18usize);
        }
        #[doc = "desc LLPEN."]
        #[must_use]
        #[inline(always)]
        pub const fn llpen(&self) -> bool {
            let val = (self.0 >> 28usize) & 0x01;
            val != 0
        }
        #[doc = "desc LLPEN."]
        #[inline(always)]
        pub const fn set_llpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
        }
        #[doc = "desc LLPRUN."]
        #[must_use]
        #[inline(always)]
        pub const fn llprun(&self) -> bool {
            let val = (self.0 >> 29usize) & 0x01;
            val != 0
        }
        #[doc = "desc LLPRUN."]
        #[inline(always)]
        pub const fn set_llprun(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
        }
        #[doc = "desc HSIZE."]
        #[must_use]
        #[inline(always)]
        pub const fn hsize(&self) -> u8 {
            let val = (self.0 >> 30usize) & 0x03;
            val as u8
        }
        #[doc = "desc HSIZE."]
        #[inline(always)]
        pub const fn set_hsize(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 30usize)) | (((val as u32) & 0x03) << 30usize);
        }
    }
    impl Default for Ch1ctl0 {
        #[inline(always)]
        fn default() -> Ch1ctl0 {
            Ch1ctl0(0)
        }
    }
    impl core::fmt::Debug for Ch1ctl0 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Ch1ctl0")
                .field("blksize", &self.blksize())
                .field("cnt", &self.cnt())
                .field("llp", &self.llp())
                .field("llpen", &self.llpen())
                .field("llprun", &self.llprun())
                .field("hsize", &self.hsize())
                .finish()
        }
    }
    #[doc = "desc CH1CTL1."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ch1ctl1(pub u32);
    impl Ch1ctl1 {
        #[doc = "desc OFFSET."]
        #[must_use]
        #[inline(always)]
        pub const fn offset(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[doc = "desc OFFSET."]
        #[inline(always)]
        pub const fn set_offset(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
        }
        #[doc = "desc RPTNSCNT."]
        #[must_use]
        #[inline(always)]
        pub const fn rptnscnt(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0xff;
            val as u8
        }
        #[doc = "desc RPTNSCNT."]
        #[inline(always)]
        pub const fn set_rptnscnt(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 16usize)) | (((val as u32) & 0xff) << 16usize);
        }
        #[doc = "desc RPTNSEN."]
        #[must_use]
        #[inline(always)]
        pub const fn rptnsen(&self) -> bool {
            let val = (self.0 >> 24usize) & 0x01;
            val != 0
        }
        #[doc = "desc RPTNSEN."]
        #[inline(always)]
        pub const fn set_rptnsen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
        }
        #[doc = "desc RPTNSSEL."]
        #[must_use]
        #[inline(always)]
        pub const fn rptnssel(&self) -> u8 {
            let val = (self.0 >> 25usize) & 0x03;
            val as u8
        }
        #[doc = "desc RPTNSSEL."]
        #[inline(always)]
        pub const fn set_rptnssel(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 25usize)) | (((val as u32) & 0x03) << 25usize);
        }
        #[doc = "desc LLPSEL."]
        #[must_use]
        #[inline(always)]
        pub const fn llpsel(&self) -> bool {
            let val = (self.0 >> 27usize) & 0x01;
            val != 0
        }
        #[doc = "desc LLPSEL."]
        #[inline(always)]
        pub const fn set_llpsel(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
        }
        #[doc = "desc SINC."]
        #[must_use]
        #[inline(always)]
        pub const fn sinc(&self) -> u8 {
            let val = (self.0 >> 28usize) & 0x03;
            val as u8
        }
        #[doc = "desc SINC."]
        #[inline(always)]
        pub const fn set_sinc(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 28usize)) | (((val as u32) & 0x03) << 28usize);
        }
        #[doc = "desc DINC."]
        #[must_use]
        #[inline(always)]
        pub const fn dinc(&self) -> u8 {
            let val = (self.0 >> 30usize) & 0x03;
            val as u8
        }
        #[doc = "desc DINC."]
        #[inline(always)]
        pub const fn set_dinc(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 30usize)) | (((val as u32) & 0x03) << 30usize);
        }
    }
    impl Default for Ch1ctl1 {
        #[inline(always)]
        fn default() -> Ch1ctl1 {
            Ch1ctl1(0)
        }
    }
    impl core::fmt::Debug for Ch1ctl1 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Ch1ctl1")
                .field("offset", &self.offset())
                .field("rptnscnt", &self.rptnscnt())
                .field("rptnsen", &self.rptnsen())
                .field("rptnssel", &self.rptnssel())
                .field("llpsel", &self.llpsel())
                .field("sinc", &self.sinc())
                .field("dinc", &self.dinc())
                .finish()
        }
    }
    #[doc = "desc CHEN."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Chen(pub u32);
    impl Chen {
        #[doc = "desc CHEN."]
        #[must_use]
        #[inline(always)]
        pub const fn chen(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x03;
            val as u8
        }
        #[doc = "desc CHEN."]
        #[inline(always)]
        pub const fn set_chen(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 0usize)) | (((val as u32) & 0x03) << 0usize);
        }
    }
    impl Default for Chen {
        #[inline(always)]
        fn default() -> Chen {
            Chen(0)
        }
    }
    impl core::fmt::Debug for Chen {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Chen").field("chen", &self.chen()).finish()
        }
    }
    #[doc = "desc CHENCLR."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Chenclr(pub u32);
    impl Chenclr {
        #[doc = "desc CHENCLR."]
        #[must_use]
        #[inline(always)]
        pub const fn chenclr(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x03;
            val as u8
        }
        #[doc = "desc CHENCLR."]
        #[inline(always)]
        pub const fn set_chenclr(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 0usize)) | (((val as u32) & 0x03) << 0usize);
        }
    }
    impl Default for Chenclr {
        #[inline(always)]
        fn default() -> Chenclr {
            Chenclr(0)
        }
    }
    impl core::fmt::Debug for Chenclr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Chenclr")
                .field("chenclr", &self.chenclr())
                .finish()
        }
    }
    #[doc = "desc CHSTAT."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Chstat(pub u32);
    impl Chstat {
        #[doc = "desc DMAACT."]
        #[must_use]
        #[inline(always)]
        pub const fn dmaact(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "desc DMAACT."]
        #[inline(always)]
        pub const fn set_dmaact(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "desc CHACT."]
        #[must_use]
        #[inline(always)]
        pub const fn chact(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x03;
            val as u8
        }
        #[doc = "desc CHACT."]
        #[inline(always)]
        pub const fn set_chact(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 16usize)) | (((val as u32) & 0x03) << 16usize);
        }
    }
    impl Default for Chstat {
        #[inline(always)]
        fn default() -> Chstat {
            Chstat(0)
        }
    }
    impl core::fmt::Debug for Chstat {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Chstat")
                .field("dmaact", &self.dmaact())
                .field("chact", &self.chact())
                .finish()
        }
    }
    #[doc = "desc EN."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct En(pub u32);
    impl En {
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
    }
    impl Default for En {
        #[inline(always)]
        fn default() -> En {
            En(0)
        }
    }
    impl core::fmt::Debug for En {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("En").field("en", &self.en()).finish()
        }
    }
    #[doc = "desc INTCLR0."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Intclr0(pub u32);
    impl Intclr0 {
        #[doc = "desc CLRTRNERR."]
        #[must_use]
        #[inline(always)]
        pub const fn clrtrnerr(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x03;
            val as u8
        }
        #[doc = "desc CLRTRNERR."]
        #[inline(always)]
        pub const fn set_clrtrnerr(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 0usize)) | (((val as u32) & 0x03) << 0usize);
        }
        #[doc = "desc CLRREQERR."]
        #[must_use]
        #[inline(always)]
        pub const fn clrreqerr(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x03;
            val as u8
        }
        #[doc = "desc CLRREQERR."]
        #[inline(always)]
        pub const fn set_clrreqerr(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 16usize)) | (((val as u32) & 0x03) << 16usize);
        }
    }
    impl Default for Intclr0 {
        #[inline(always)]
        fn default() -> Intclr0 {
            Intclr0(0)
        }
    }
    impl core::fmt::Debug for Intclr0 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Intclr0")
                .field("clrtrnerr", &self.clrtrnerr())
                .field("clrreqerr", &self.clrreqerr())
                .finish()
        }
    }
    #[doc = "desc INTCLR1."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Intclr1(pub u32);
    impl Intclr1 {
        #[doc = "desc CLRTC."]
        #[must_use]
        #[inline(always)]
        pub const fn clrtc(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x03;
            val as u8
        }
        #[doc = "desc CLRTC."]
        #[inline(always)]
        pub const fn set_clrtc(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 0usize)) | (((val as u32) & 0x03) << 0usize);
        }
        #[doc = "desc CLRBTC."]
        #[must_use]
        #[inline(always)]
        pub const fn clrbtc(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x03;
            val as u8
        }
        #[doc = "desc CLRBTC."]
        #[inline(always)]
        pub const fn set_clrbtc(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 16usize)) | (((val as u32) & 0x03) << 16usize);
        }
    }
    impl Default for Intclr1 {
        #[inline(always)]
        fn default() -> Intclr1 {
            Intclr1(0)
        }
    }
    impl core::fmt::Debug for Intclr1 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Intclr1")
                .field("clrtc", &self.clrtc())
                .field("clrbtc", &self.clrbtc())
                .finish()
        }
    }
    #[doc = "desc INTMASK0."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Intmask0(pub u32);
    impl Intmask0 {
        #[doc = "desc MSKTRNERR."]
        #[must_use]
        #[inline(always)]
        pub const fn msktrnerr(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x03;
            val as u8
        }
        #[doc = "desc MSKTRNERR."]
        #[inline(always)]
        pub const fn set_msktrnerr(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 0usize)) | (((val as u32) & 0x03) << 0usize);
        }
        #[doc = "desc MSKREQERR."]
        #[must_use]
        #[inline(always)]
        pub const fn mskreqerr(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x03;
            val as u8
        }
        #[doc = "desc MSKREQERR."]
        #[inline(always)]
        pub const fn set_mskreqerr(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 16usize)) | (((val as u32) & 0x03) << 16usize);
        }
    }
    impl Default for Intmask0 {
        #[inline(always)]
        fn default() -> Intmask0 {
            Intmask0(0)
        }
    }
    impl core::fmt::Debug for Intmask0 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Intmask0")
                .field("msktrnerr", &self.msktrnerr())
                .field("mskreqerr", &self.mskreqerr())
                .finish()
        }
    }
    #[doc = "desc INTMASK1."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Intmask1(pub u32);
    impl Intmask1 {
        #[doc = "desc MSKTC."]
        #[must_use]
        #[inline(always)]
        pub const fn msktc(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x03;
            val as u8
        }
        #[doc = "desc MSKTC."]
        #[inline(always)]
        pub const fn set_msktc(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 0usize)) | (((val as u32) & 0x03) << 0usize);
        }
        #[doc = "desc MSKBTC."]
        #[must_use]
        #[inline(always)]
        pub const fn mskbtc(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x03;
            val as u8
        }
        #[doc = "desc MSKBTC."]
        #[inline(always)]
        pub const fn set_mskbtc(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 16usize)) | (((val as u32) & 0x03) << 16usize);
        }
    }
    impl Default for Intmask1 {
        #[inline(always)]
        fn default() -> Intmask1 {
            Intmask1(0)
        }
    }
    impl core::fmt::Debug for Intmask1 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Intmask1")
                .field("msktc", &self.msktc())
                .field("mskbtc", &self.mskbtc())
                .finish()
        }
    }
    #[doc = "desc INTSTAT0."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Intstat0(pub u32);
    impl Intstat0 {
        #[doc = "desc TRNERR."]
        #[must_use]
        #[inline(always)]
        pub const fn trnerr(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x03;
            val as u8
        }
        #[doc = "desc TRNERR."]
        #[inline(always)]
        pub const fn set_trnerr(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 0usize)) | (((val as u32) & 0x03) << 0usize);
        }
        #[doc = "desc REQERR."]
        #[must_use]
        #[inline(always)]
        pub const fn reqerr(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x03;
            val as u8
        }
        #[doc = "desc REQERR."]
        #[inline(always)]
        pub const fn set_reqerr(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 16usize)) | (((val as u32) & 0x03) << 16usize);
        }
    }
    impl Default for Intstat0 {
        #[inline(always)]
        fn default() -> Intstat0 {
            Intstat0(0)
        }
    }
    impl core::fmt::Debug for Intstat0 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Intstat0")
                .field("trnerr", &self.trnerr())
                .field("reqerr", &self.reqerr())
                .finish()
        }
    }
    #[doc = "desc INTSTAT1."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Intstat1(pub u32);
    impl Intstat1 {
        #[doc = "desc TC."]
        #[must_use]
        #[inline(always)]
        pub const fn tc(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x03;
            val as u8
        }
        #[doc = "desc TC."]
        #[inline(always)]
        pub const fn set_tc(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 0usize)) | (((val as u32) & 0x03) << 0usize);
        }
        #[doc = "desc BTC."]
        #[must_use]
        #[inline(always)]
        pub const fn btc(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x03;
            val as u8
        }
        #[doc = "desc BTC."]
        #[inline(always)]
        pub const fn set_btc(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 16usize)) | (((val as u32) & 0x03) << 16usize);
        }
    }
    impl Default for Intstat1 {
        #[inline(always)]
        fn default() -> Intstat1 {
            Intstat1(0)
        }
    }
    impl core::fmt::Debug for Intstat1 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Intstat1")
                .field("tc", &self.tc())
                .field("btc", &self.btc())
                .finish()
        }
    }
}
