#![allow(clippy::missing_safety_doc)]
#![allow(clippy::identity_op)]
#![allow(clippy::unnecessary_cast)]
#![allow(clippy::erasing_op)]

#[doc = "desc CMP_COMMON."]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct CmpCommon {
    ptr: *mut u8,
}
unsafe impl Send for CmpCommon {}
unsafe impl Sync for CmpCommon {}
impl CmpCommon {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "desc DADR."]
    #[inline(always)]
    pub const fn dadr(self) -> crate::common::Reg<regs::Dadr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0cusize) as _) }
    }
    #[doc = "desc DACR."]
    #[inline(always)]
    pub const fn dacr(self) -> crate::common::Reg<regs::Dacr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0eusize) as _) }
    }
}
pub mod regs {
    #[doc = "desc DACR."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Dacr(pub u16);
    impl Dacr {
        #[doc = "desc DAEN."]
        #[must_use]
        #[inline(always)]
        pub const fn daen(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "desc DAEN."]
        #[inline(always)]
        pub const fn set_daen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u16) & 0x01) << 0usize);
        }
        #[doc = "desc DA2POE."]
        #[must_use]
        #[inline(always)]
        pub const fn da2poe(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "desc DA2POE."]
        #[inline(always)]
        pub const fn set_da2poe(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u16) & 0x01) << 3usize);
        }
        #[doc = "desc REFSEL."]
        #[must_use]
        #[inline(always)]
        pub const fn refsel(&self) -> u8 {
            let val = (self.0 >> 6usize) & 0x03;
            val as u8
        }
        #[doc = "desc REFSEL."]
        #[inline(always)]
        pub const fn set_refsel(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 6usize)) | (((val as u16) & 0x03) << 6usize);
        }
    }
    impl Default for Dacr {
        #[inline(always)]
        fn default() -> Dacr {
            Dacr(0)
        }
    }
    impl core::fmt::Debug for Dacr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Dacr")
                .field("daen", &self.daen())
                .field("da2poe", &self.da2poe())
                .field("refsel", &self.refsel())
                .finish()
        }
    }
    #[doc = "desc DADR."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Dadr(pub u16);
    impl Dadr {
        #[doc = "desc DADR."]
        #[must_use]
        #[inline(always)]
        pub const fn dadr(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x0fff;
            val as u16
        }
        #[doc = "desc DADR."]
        #[inline(always)]
        pub const fn set_dadr(&mut self, val: u16) {
            self.0 = (self.0 & !(0x0fff << 0usize)) | (((val as u16) & 0x0fff) << 0usize);
        }
    }
    impl Default for Dadr {
        #[inline(always)]
        fn default() -> Dadr {
            Dadr(0)
        }
    }
    impl core::fmt::Debug for Dadr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Dadr").field("dadr", &self.dadr()).finish()
        }
    }
}
