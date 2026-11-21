#![allow(clippy::missing_safety_doc)]
#![allow(clippy::identity_op)]
#![allow(clippy::unnecessary_cast)]
#![allow(clippy::erasing_op)]

#[doc = "desc HDIV."]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Hdiv {
    ptr: *mut u8,
}
unsafe impl Send for Hdiv {}
unsafe impl Sync for Hdiv {}
impl Hdiv {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "desc DIVIDEND."]
    #[inline(always)]
    pub const fn dividend(self) -> crate::common::Reg<regs::Dividend, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0usize) as _) }
    }
    #[doc = "desc DIVISOR."]
    #[inline(always)]
    pub const fn divisor(self) -> crate::common::Reg<regs::Divisor, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x04usize) as _) }
    }
    #[doc = "desc QUOTIENT."]
    #[inline(always)]
    pub const fn quotient(self) -> crate::common::Reg<regs::Quotient, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x08usize) as _) }
    }
    #[doc = "desc REMAINDER."]
    #[inline(always)]
    pub const fn remainder(self) -> crate::common::Reg<regs::Remainder, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0cusize) as _) }
    }
    #[doc = "desc SIGN."]
    #[inline(always)]
    pub const fn sign(self) -> crate::common::Reg<regs::Sign, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x10usize) as _) }
    }
    #[doc = "desc STAT."]
    #[inline(always)]
    pub const fn stat(self) -> crate::common::Reg<regs::Stat, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x14usize) as _) }
    }
}
pub mod regs {
    #[doc = "desc DIVIDEND."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Dividend(pub u32);
    impl Dividend {
        #[doc = "desc DIVIDEND."]
        #[must_use]
        #[inline(always)]
        pub const fn dividend(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "desc DIVIDEND."]
        #[inline(always)]
        pub const fn set_dividend(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Dividend {
        #[inline(always)]
        fn default() -> Dividend {
            Dividend(0)
        }
    }
    impl core::fmt::Debug for Dividend {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Dividend")
                .field("dividend", &self.dividend())
                .finish()
        }
    }
    #[doc = "desc DIVISOR."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Divisor(pub u32);
    impl Divisor {
        #[doc = "desc DIVISOR."]
        #[must_use]
        #[inline(always)]
        pub const fn divisor(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[doc = "desc DIVISOR."]
        #[inline(always)]
        pub const fn set_divisor(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
        }
    }
    impl Default for Divisor {
        #[inline(always)]
        fn default() -> Divisor {
            Divisor(0)
        }
    }
    impl core::fmt::Debug for Divisor {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Divisor")
                .field("divisor", &self.divisor())
                .finish()
        }
    }
    #[doc = "desc QUOTIENT."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Quotient(pub u32);
    impl Quotient {
        #[doc = "desc QUOTIENT."]
        #[must_use]
        #[inline(always)]
        pub const fn quotient(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "desc QUOTIENT."]
        #[inline(always)]
        pub const fn set_quotient(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Quotient {
        #[inline(always)]
        fn default() -> Quotient {
            Quotient(0)
        }
    }
    impl core::fmt::Debug for Quotient {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Quotient")
                .field("quotient", &self.quotient())
                .finish()
        }
    }
    #[doc = "desc REMAINDER."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Remainder(pub u32);
    impl Remainder {
        #[doc = "desc REMAINDER."]
        #[must_use]
        #[inline(always)]
        pub const fn remainder(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "desc REMAINDER."]
        #[inline(always)]
        pub const fn set_remainder(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Remainder {
        #[inline(always)]
        fn default() -> Remainder {
            Remainder(0)
        }
    }
    impl core::fmt::Debug for Remainder {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Remainder")
                .field("remainder", &self.remainder())
                .finish()
        }
    }
    #[doc = "desc SIGN."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Sign(pub u32);
    impl Sign {
        #[doc = "desc SIGN."]
        #[must_use]
        #[inline(always)]
        pub const fn sign(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "desc SIGN."]
        #[inline(always)]
        pub const fn set_sign(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
    }
    impl Default for Sign {
        #[inline(always)]
        fn default() -> Sign {
            Sign(0)
        }
    }
    impl core::fmt::Debug for Sign {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Sign").field("sign", &self.sign()).finish()
        }
    }
    #[doc = "desc STAT."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Stat(pub u32);
    impl Stat {
        #[doc = "desc END."]
        #[must_use]
        #[inline(always)]
        pub const fn end(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "desc END."]
        #[inline(always)]
        pub const fn set_end(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "desc ZERO."]
        #[must_use]
        #[inline(always)]
        pub const fn zero(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "desc ZERO."]
        #[inline(always)]
        pub const fn set_zero(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
    }
    impl Default for Stat {
        #[inline(always)]
        fn default() -> Stat {
            Stat(0)
        }
    }
    impl core::fmt::Debug for Stat {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Stat")
                .field("end", &self.end())
                .field("zero", &self.zero())
                .finish()
        }
    }
}
