#![allow(clippy::missing_safety_doc)]
#![allow(clippy::identity_op)]
#![allow(clippy::unnecessary_cast)]
#![allow(clippy::erasing_op)]

#[doc = "desc GPIO."]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Gpio {
    ptr: *mut u8,
}
unsafe impl Send for Gpio {}
unsafe impl Sync for Gpio {}
impl Gpio {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "desc PIN00_SEL."]
    #[inline(always)]
    pub const fn pin00_sel(self) -> crate::common::Reg<regs::Pin00Sel, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0usize) as _) }
    }
    #[doc = "desc IN."]
    #[inline(always)]
    pub const fn in_(self) -> crate::common::Reg<regs::In, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0104usize) as _) }
    }
    #[doc = "desc HIE."]
    #[inline(always)]
    pub const fn hie(self) -> crate::common::Reg<regs::Hie, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0130usize) as _) }
    }
    #[doc = "desc LIE."]
    #[inline(always)]
    pub const fn lie(self) -> crate::common::Reg<regs::Lie, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0134usize) as _) }
    }
    #[doc = "desc RIE."]
    #[inline(always)]
    pub const fn rie(self) -> crate::common::Reg<regs::Rie, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0138usize) as _) }
    }
    #[doc = "desc FIE."]
    #[inline(always)]
    pub const fn fie(self) -> crate::common::Reg<regs::Fie, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x013cusize) as _) }
    }
    #[doc = "desc STAT."]
    #[inline(always)]
    pub const fn stat(self) -> crate::common::Reg<regs::Stat, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0200usize) as _) }
    }
    #[doc = "desc ICLR."]
    #[inline(always)]
    pub const fn iclr(self) -> crate::common::Reg<regs::Iclr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0210usize) as _) }
    }
}
pub mod regs {
    #[doc = "desc FIE."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Fie(pub u32);
    impl Fie {
        #[doc = "desc PIN00."]
        #[must_use]
        #[inline(always)]
        pub const fn pin00(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "desc PIN00."]
        #[inline(always)]
        pub const fn set_pin00(&mut self, val: bool) {
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
    impl Default for Fie {
        #[inline(always)]
        fn default() -> Fie {
            Fie(0)
        }
    }
    impl core::fmt::Debug for Fie {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Fie")
                .field("pin00", &self.pin00())
                .field("rsv", &self.rsv())
                .finish()
        }
    }
    #[doc = "desc HIE."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Hie(pub u32);
    impl Hie {
        #[doc = "desc PIN00."]
        #[must_use]
        #[inline(always)]
        pub const fn pin00(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "desc PIN00."]
        #[inline(always)]
        pub const fn set_pin00(&mut self, val: bool) {
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
    impl Default for Hie {
        #[inline(always)]
        fn default() -> Hie {
            Hie(0)
        }
    }
    impl core::fmt::Debug for Hie {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Hie")
                .field("pin00", &self.pin00())
                .field("rsv", &self.rsv())
                .finish()
        }
    }
    #[doc = "desc ICLR."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Iclr(pub u32);
    impl Iclr {
        #[doc = "desc PIN00."]
        #[must_use]
        #[inline(always)]
        pub const fn pin00(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "desc PIN00."]
        #[inline(always)]
        pub const fn set_pin00(&mut self, val: bool) {
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
                .field("pin00", &self.pin00())
                .field("rsv", &self.rsv())
                .finish()
        }
    }
    #[doc = "desc IN."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct In(pub u32);
    impl In {
        #[doc = "desc PIN00."]
        #[must_use]
        #[inline(always)]
        pub const fn pin00(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "desc PIN00."]
        #[inline(always)]
        pub const fn set_pin00(&mut self, val: bool) {
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
    impl Default for In {
        #[inline(always)]
        fn default() -> In {
            In(0)
        }
    }
    impl core::fmt::Debug for In {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("In")
                .field("pin00", &self.pin00())
                .field("rsv", &self.rsv())
                .finish()
        }
    }
    #[doc = "desc LIE."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Lie(pub u32);
    impl Lie {
        #[doc = "desc PIN00."]
        #[must_use]
        #[inline(always)]
        pub const fn pin00(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "desc PIN00."]
        #[inline(always)]
        pub const fn set_pin00(&mut self, val: bool) {
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
    impl Default for Lie {
        #[inline(always)]
        fn default() -> Lie {
            Lie(0)
        }
    }
    impl core::fmt::Debug for Lie {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Lie")
                .field("pin00", &self.pin00())
                .field("rsv", &self.rsv())
                .finish()
        }
    }
    #[doc = "desc PIN00_SEL."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Pin00Sel(pub u32);
    impl Pin00Sel {
        #[doc = "desc SEL."]
        #[must_use]
        #[inline(always)]
        pub const fn sel(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x07;
            val as u8
        }
        #[doc = "desc SEL."]
        #[inline(always)]
        pub const fn set_sel(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 0usize)) | (((val as u32) & 0x07) << 0usize);
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
    impl Default for Pin00Sel {
        #[inline(always)]
        fn default() -> Pin00Sel {
            Pin00Sel(0)
        }
    }
    impl core::fmt::Debug for Pin00Sel {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Pin00Sel")
                .field("sel", &self.sel())
                .field("rsv", &self.rsv())
                .finish()
        }
    }
    #[doc = "desc RIE."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Rie(pub u32);
    impl Rie {
        #[doc = "desc PIN00."]
        #[must_use]
        #[inline(always)]
        pub const fn pin00(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "desc PIN00."]
        #[inline(always)]
        pub const fn set_pin00(&mut self, val: bool) {
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
    impl Default for Rie {
        #[inline(always)]
        fn default() -> Rie {
            Rie(0)
        }
    }
    impl core::fmt::Debug for Rie {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Rie")
                .field("pin00", &self.pin00())
                .field("rsv", &self.rsv())
                .finish()
        }
    }
    #[doc = "desc STAT."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Stat(pub u32);
    impl Stat {
        #[doc = "desc PIN00."]
        #[must_use]
        #[inline(always)]
        pub const fn pin00(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "desc PIN00."]
        #[inline(always)]
        pub const fn set_pin00(&mut self, val: bool) {
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
    impl Default for Stat {
        #[inline(always)]
        fn default() -> Stat {
            Stat(0)
        }
    }
    impl core::fmt::Debug for Stat {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Stat")
                .field("pin00", &self.pin00())
                .field("rsv", &self.rsv())
                .finish()
        }
    }
}
