#![allow(clippy::missing_safety_doc)]
#![allow(clippy::identity_op)]
#![allow(clippy::unnecessary_cast)]
#![allow(clippy::erasing_op)]

#[doc = "desc EAU."]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Eau {
    ptr: *mut u8,
}
unsafe impl Send for Eau {}
unsafe impl Sync for Eau {}
impl Eau {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "desc CSR."]
    #[inline(always)]
    pub const fn csr(self) -> crate::common::Reg<regs::Csr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0usize) as _) }
    }
    #[doc = "desc DIVIDEND."]
    #[inline(always)]
    pub const fn dividend(self) -> crate::common::Reg<regs::Dividend, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x04usize) as _) }
    }
    #[doc = "desc DIVISOR."]
    #[inline(always)]
    pub const fn divisor(self) -> crate::common::Reg<regs::Divisor, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x08usize) as _) }
    }
    #[doc = "desc QUOTIENT."]
    #[inline(always)]
    pub const fn quotient(self) -> crate::common::Reg<regs::Quotient, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0cusize) as _) }
    }
    #[doc = "desc REMAINDER."]
    #[inline(always)]
    pub const fn remainder(self) -> crate::common::Reg<regs::Remainder, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x10usize) as _) }
    }
}
pub mod regs {
    #[doc = "desc CSR."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Csr(pub u32);
    impl Csr {
        #[doc = "desc MODE."]
        #[must_use]
        #[inline(always)]
        pub const fn mode(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x03;
            val as u8
        }
        #[doc = "desc MODE."]
        #[inline(always)]
        pub const fn set_mode(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 0usize)) | (((val as u32) & 0x03) << 0usize);
        }
        #[doc = "desc OVR."]
        #[must_use]
        #[inline(always)]
        pub const fn ovr(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "desc OVR."]
        #[inline(always)]
        pub const fn set_ovr(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "desc ZERO."]
        #[must_use]
        #[inline(always)]
        pub const fn zero(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "desc ZERO."]
        #[inline(always)]
        pub const fn set_zero(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "desc BUSY."]
        #[must_use]
        #[inline(always)]
        pub const fn busy(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "desc BUSY."]
        #[inline(always)]
        pub const fn set_busy(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
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
    impl Default for Csr {
        #[inline(always)]
        fn default() -> Csr {
            Csr(0)
        }
    }
    impl core::fmt::Debug for Csr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Csr")
                .field("mode", &self.mode())
                .field("ovr", &self.ovr())
                .field("zero", &self.zero())
                .field("busy", &self.busy())
                .field("rsv", &self.rsv())
                .finish()
        }
    }
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
        pub const fn divisor(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "desc DIVISOR."]
        #[inline(always)]
        pub const fn set_divisor(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
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
}
