#![allow(clippy::missing_safety_doc)]
#![allow(clippy::identity_op)]
#![allow(clippy::unnecessary_cast)]
#![allow(clippy::erasing_op)]

#[doc = "desc FCM_COM."]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct FcmCommon {
    ptr: *mut u8,
}
unsafe impl Send for FcmCommon {}
unsafe impl Sync for FcmCommon {}
impl FcmCommon {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "desc STR."]
    #[inline(always)]
    pub const fn str(self) -> crate::common::Reg<regs::Str, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0cusize) as _) }
    }
    #[doc = "desc SR."]
    #[inline(always)]
    pub const fn sr(self) -> crate::common::Reg<regs::Sr, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1cusize) as _) }
    }
}
pub mod regs {
    #[doc = "desc SR."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Sr(pub u32);
    impl Sr {
        #[doc = "desc ERRF1."]
        #[must_use]
        #[inline(always)]
        pub const fn errf1(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "desc ERRF1."]
        #[inline(always)]
        pub const fn set_errf1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "desc MENDF1."]
        #[must_use]
        #[inline(always)]
        pub const fn mendf1(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "desc MENDF1."]
        #[inline(always)]
        pub const fn set_mendf1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "desc OVF1."]
        #[must_use]
        #[inline(always)]
        pub const fn ovf1(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "desc OVF1."]
        #[inline(always)]
        pub const fn set_ovf1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "desc ERRF2."]
        #[must_use]
        #[inline(always)]
        pub const fn errf2(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "desc ERRF2."]
        #[inline(always)]
        pub const fn set_errf2(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "desc MENDF2."]
        #[must_use]
        #[inline(always)]
        pub const fn mendf2(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "desc MENDF2."]
        #[inline(always)]
        pub const fn set_mendf2(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "desc OVF2."]
        #[must_use]
        #[inline(always)]
        pub const fn ovf2(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "desc OVF2."]
        #[inline(always)]
        pub const fn set_ovf2(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "desc ERRF3."]
        #[must_use]
        #[inline(always)]
        pub const fn errf3(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "desc ERRF3."]
        #[inline(always)]
        pub const fn set_errf3(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "desc MENDF3."]
        #[must_use]
        #[inline(always)]
        pub const fn mendf3(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "desc MENDF3."]
        #[inline(always)]
        pub const fn set_mendf3(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "desc OVF3."]
        #[must_use]
        #[inline(always)]
        pub const fn ovf3(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "desc OVF3."]
        #[inline(always)]
        pub const fn set_ovf3(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "desc ERRF4."]
        #[must_use]
        #[inline(always)]
        pub const fn errf4(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "desc ERRF4."]
        #[inline(always)]
        pub const fn set_errf4(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "desc MENDF4."]
        #[must_use]
        #[inline(always)]
        pub const fn mendf4(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "desc MENDF4."]
        #[inline(always)]
        pub const fn set_mendf4(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[doc = "desc OVF4."]
        #[must_use]
        #[inline(always)]
        pub const fn ovf4(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "desc OVF4."]
        #[inline(always)]
        pub const fn set_ovf4(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
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
                .field("errf1", &self.errf1())
                .field("mendf1", &self.mendf1())
                .field("ovf1", &self.ovf1())
                .field("errf2", &self.errf2())
                .field("mendf2", &self.mendf2())
                .field("ovf2", &self.ovf2())
                .field("errf3", &self.errf3())
                .field("mendf3", &self.mendf3())
                .field("ovf3", &self.ovf3())
                .field("errf4", &self.errf4())
                .field("mendf4", &self.mendf4())
                .field("ovf4", &self.ovf4())
                .finish()
        }
    }
    #[doc = "desc STR."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Str(pub u32);
    impl Str {
        #[doc = "desc START1."]
        #[must_use]
        #[inline(always)]
        pub const fn start1(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "desc START1."]
        #[inline(always)]
        pub const fn set_start1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "desc START2."]
        #[must_use]
        #[inline(always)]
        pub const fn start2(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "desc START2."]
        #[inline(always)]
        pub const fn set_start2(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "desc START3."]
        #[must_use]
        #[inline(always)]
        pub const fn start3(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "desc START3."]
        #[inline(always)]
        pub const fn set_start3(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "desc START4."]
        #[must_use]
        #[inline(always)]
        pub const fn start4(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "desc START4."]
        #[inline(always)]
        pub const fn set_start4(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
    }
    impl Default for Str {
        #[inline(always)]
        fn default() -> Str {
            Str(0)
        }
    }
    impl core::fmt::Debug for Str {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Str")
                .field("start1", &self.start1())
                .field("start2", &self.start2())
                .field("start3", &self.start3())
                .field("start4", &self.start4())
                .finish()
        }
    }
}
