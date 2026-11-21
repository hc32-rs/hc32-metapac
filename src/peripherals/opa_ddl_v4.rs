#![allow(clippy::missing_safety_doc)]
#![allow(clippy::identity_op)]
#![allow(clippy::unnecessary_cast)]
#![allow(clippy::erasing_op)]

#[doc = "desc OPA."]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Opa {
    ptr: *mut u8,
}
unsafe impl Send for Opa {}
unsafe impl Sync for Opa {}
impl Opa {
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
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x30usize) as _) }
    }
    #[doc = "desc CR1."]
    #[inline(always)]
    pub const fn cr1(self) -> crate::common::Reg<regs::Cr1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x34usize) as _) }
    }
    #[doc = "desc CR2."]
    #[inline(always)]
    pub const fn cr2(self) -> crate::common::Reg<regs::Cr2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x38usize) as _) }
    }
    #[doc = "desc CR."]
    #[inline(always)]
    pub const fn cr(self) -> crate::common::Reg<regs::Cr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x3cusize) as _) }
    }
}
pub mod regs {
    #[doc = "desc CR."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Cr(pub u32);
    impl Cr {
        #[doc = "desc ADCTR_EN."]
        #[must_use]
        #[inline(always)]
        pub const fn adctr_en(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "desc ADCTR_EN."]
        #[inline(always)]
        pub const fn set_adctr_en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "desc TRIGGER."]
        #[must_use]
        #[inline(always)]
        pub const fn trigger(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "desc TRIGGER."]
        #[inline(always)]
        pub const fn set_trigger(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "desc AZ_PULSE."]
        #[must_use]
        #[inline(always)]
        pub const fn az_pulse(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "desc AZ_PULSE."]
        #[inline(always)]
        pub const fn set_az_pulse(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "desc CLK_SW_SET."]
        #[must_use]
        #[inline(always)]
        pub const fn clk_sw_set(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "desc CLK_SW_SET."]
        #[inline(always)]
        pub const fn set_clk_sw_set(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "desc CLK_SEL."]
        #[must_use]
        #[inline(always)]
        pub const fn clk_sel(&self) -> u8 {
            let val = (self.0 >> 4usize) & 0x0f;
            val as u8
        }
        #[doc = "desc CLK_SEL."]
        #[inline(always)]
        pub const fn set_clk_sel(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 4usize)) | (((val as u32) & 0x0f) << 4usize);
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
                .field("adctr_en", &self.adctr_en())
                .field("trigger", &self.trigger())
                .field("az_pulse", &self.az_pulse())
                .field("clk_sw_set", &self.clk_sw_set())
                .field("clk_sel", &self.clk_sel())
                .finish()
        }
    }
    #[doc = "desc CR0."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Cr0(pub u32);
    impl Cr0 {
        #[doc = "desc OP0OEN1."]
        #[must_use]
        #[inline(always)]
        pub const fn op0oen1(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "desc OP0OEN1."]
        #[inline(always)]
        pub const fn set_op0oen1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "desc OP0OEN2."]
        #[must_use]
        #[inline(always)]
        pub const fn op0oen2(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "desc OP0OEN2."]
        #[inline(always)]
        pub const fn set_op0oen2(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "desc OP0OEN3."]
        #[must_use]
        #[inline(always)]
        pub const fn op0oen3(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "desc OP0OEN3."]
        #[inline(always)]
        pub const fn set_op0oen3(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "desc OP0OEN4."]
        #[must_use]
        #[inline(always)]
        pub const fn op0oen4(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "desc OP0OEN4."]
        #[inline(always)]
        pub const fn set_op0oen4(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "desc OP1OEN1."]
        #[must_use]
        #[inline(always)]
        pub const fn op1oen1(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "desc OP1OEN1."]
        #[inline(always)]
        pub const fn set_op1oen1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "desc OP1OEN2."]
        #[must_use]
        #[inline(always)]
        pub const fn op1oen2(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "desc OP1OEN2."]
        #[inline(always)]
        pub const fn set_op1oen2(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "desc OP1OEN3."]
        #[must_use]
        #[inline(always)]
        pub const fn op1oen3(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "desc OP1OEN3."]
        #[inline(always)]
        pub const fn set_op1oen3(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "desc OP1OEN4."]
        #[must_use]
        #[inline(always)]
        pub const fn op1oen4(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "desc OP1OEN4."]
        #[inline(always)]
        pub const fn set_op1oen4(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "desc OP2OEN1."]
        #[must_use]
        #[inline(always)]
        pub const fn op2oen1(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "desc OP2OEN1."]
        #[inline(always)]
        pub const fn set_op2oen1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "desc OP2OEN2."]
        #[must_use]
        #[inline(always)]
        pub const fn op2oen2(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "desc OP2OEN2."]
        #[inline(always)]
        pub const fn set_op2oen2(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "desc OP2OEN3."]
        #[must_use]
        #[inline(always)]
        pub const fn op2oen3(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "desc OP2OEN3."]
        #[inline(always)]
        pub const fn set_op2oen3(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[doc = "desc OP2OEN4."]
        #[must_use]
        #[inline(always)]
        pub const fn op2oen4(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "desc OP2OEN4."]
        #[inline(always)]
        pub const fn set_op2oen4(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
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
                .field("op0oen1", &self.op0oen1())
                .field("op0oen2", &self.op0oen2())
                .field("op0oen3", &self.op0oen3())
                .field("op0oen4", &self.op0oen4())
                .field("op1oen1", &self.op1oen1())
                .field("op1oen2", &self.op1oen2())
                .field("op1oen3", &self.op1oen3())
                .field("op1oen4", &self.op1oen4())
                .field("op2oen1", &self.op2oen1())
                .field("op2oen2", &self.op2oen2())
                .field("op2oen3", &self.op2oen3())
                .field("op2oen4", &self.op2oen4())
                .finish()
        }
    }
    #[doc = "desc CR1."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Cr1(pub u32);
    impl Cr1 {
        #[doc = "desc AZEN0."]
        #[must_use]
        #[inline(always)]
        pub const fn azen0(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "desc AZEN0."]
        #[inline(always)]
        pub const fn set_azen0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "desc AZEN1."]
        #[must_use]
        #[inline(always)]
        pub const fn azen1(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "desc AZEN1."]
        #[inline(always)]
        pub const fn set_azen1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "desc AZEN2."]
        #[must_use]
        #[inline(always)]
        pub const fn azen2(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "desc AZEN2."]
        #[inline(always)]
        pub const fn set_azen2(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "desc AZEN3."]
        #[must_use]
        #[inline(always)]
        pub const fn azen3(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "desc AZEN3."]
        #[inline(always)]
        pub const fn set_azen3(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "desc AZEN4."]
        #[must_use]
        #[inline(always)]
        pub const fn azen4(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "desc AZEN4."]
        #[inline(always)]
        pub const fn set_azen4(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "desc EN0."]
        #[must_use]
        #[inline(always)]
        pub const fn en0(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "desc EN0."]
        #[inline(always)]
        pub const fn set_en0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "desc EN1."]
        #[must_use]
        #[inline(always)]
        pub const fn en1(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "desc EN1."]
        #[inline(always)]
        pub const fn set_en1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "desc EN2."]
        #[must_use]
        #[inline(always)]
        pub const fn en2(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "desc EN2."]
        #[inline(always)]
        pub const fn set_en2(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "desc EN3."]
        #[must_use]
        #[inline(always)]
        pub const fn en3(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "desc EN3."]
        #[inline(always)]
        pub const fn set_en3(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "desc EN4."]
        #[must_use]
        #[inline(always)]
        pub const fn en4(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "desc EN4."]
        #[inline(always)]
        pub const fn set_en4(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "desc OP3BUFEN."]
        #[must_use]
        #[inline(always)]
        pub const fn op3bufen(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "desc OP3BUFEN."]
        #[inline(always)]
        pub const fn set_op3bufen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[doc = "desc OP4BUFEN."]
        #[must_use]
        #[inline(always)]
        pub const fn op4bufen(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "desc OP4BUFEN."]
        #[inline(always)]
        pub const fn set_op4bufen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
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
                .field("azen0", &self.azen0())
                .field("azen1", &self.azen1())
                .field("azen2", &self.azen2())
                .field("azen3", &self.azen3())
                .field("azen4", &self.azen4())
                .field("en0", &self.en0())
                .field("en1", &self.en1())
                .field("en2", &self.en2())
                .field("en3", &self.en3())
                .field("en4", &self.en4())
                .field("op3bufen", &self.op3bufen())
                .field("op4bufen", &self.op4bufen())
                .finish()
        }
    }
    #[doc = "desc CR2."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Cr2(pub u32);
    impl Cr2 {
        #[doc = "desc BIASSEL0."]
        #[must_use]
        #[inline(always)]
        pub const fn biassel0(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x07;
            val as u8
        }
        #[doc = "desc BIASSEL0."]
        #[inline(always)]
        pub const fn set_biassel0(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 0usize)) | (((val as u32) & 0x07) << 0usize);
        }
        #[doc = "desc BIASSEL1."]
        #[must_use]
        #[inline(always)]
        pub const fn biassel1(&self) -> u8 {
            let val = (self.0 >> 3usize) & 0x07;
            val as u8
        }
        #[doc = "desc BIASSEL1."]
        #[inline(always)]
        pub const fn set_biassel1(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 3usize)) | (((val as u32) & 0x07) << 3usize);
        }
        #[doc = "desc BIASSEL2."]
        #[must_use]
        #[inline(always)]
        pub const fn biassel2(&self) -> u8 {
            let val = (self.0 >> 6usize) & 0x07;
            val as u8
        }
        #[doc = "desc BIASSEL2."]
        #[inline(always)]
        pub const fn set_biassel2(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 6usize)) | (((val as u32) & 0x07) << 6usize);
        }
        #[doc = "desc BIASSEL3."]
        #[must_use]
        #[inline(always)]
        pub const fn biassel3(&self) -> u8 {
            let val = (self.0 >> 9usize) & 0x07;
            val as u8
        }
        #[doc = "desc BIASSEL3."]
        #[inline(always)]
        pub const fn set_biassel3(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 9usize)) | (((val as u32) & 0x07) << 9usize);
        }
        #[doc = "desc BIASSEL4."]
        #[must_use]
        #[inline(always)]
        pub const fn biassel4(&self) -> u8 {
            let val = (self.0 >> 12usize) & 0x07;
            val as u8
        }
        #[doc = "desc BIASSEL4."]
        #[inline(always)]
        pub const fn set_biassel4(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 12usize)) | (((val as u32) & 0x07) << 12usize);
        }
    }
    impl Default for Cr2 {
        #[inline(always)]
        fn default() -> Cr2 {
            Cr2(0)
        }
    }
    impl core::fmt::Debug for Cr2 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Cr2")
                .field("biassel0", &self.biassel0())
                .field("biassel1", &self.biassel1())
                .field("biassel2", &self.biassel2())
                .field("biassel3", &self.biassel3())
                .field("biassel4", &self.biassel4())
                .finish()
        }
    }
}
