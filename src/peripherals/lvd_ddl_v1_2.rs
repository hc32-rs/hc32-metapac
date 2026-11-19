#![allow(clippy::missing_safety_doc)]
#![allow(clippy::identity_op)]
#![allow(clippy::unnecessary_cast)]
#![allow(clippy::erasing_op)]

#[doc = "desc LVD."]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Lvd {
    ptr: *mut u8,
}
unsafe impl Send for Lvd {}
unsafe impl Sync for Lvd {}
impl Lvd {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "desc CR."]
    #[inline(always)]
    pub const fn cr(self) -> crate::common::Reg<regs::Cr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0usize) as _) }
    }
    #[doc = "desc SR."]
    #[inline(always)]
    pub const fn sr(self) -> crate::common::Reg<regs::Sr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x04usize) as _) }
    }
}
pub mod regs {
    #[doc = "desc CR."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Cr(pub u32);
    impl Cr {
        #[doc = "desc LVDEN."]
        #[must_use]
        #[inline(always)]
        pub const fn lvden(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "desc LVDEN."]
        #[inline(always)]
        pub const fn set_lvden(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "desc ACT."]
        #[must_use]
        #[inline(always)]
        pub const fn act(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "desc ACT."]
        #[inline(always)]
        pub const fn set_act(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "desc SOURCE."]
        #[must_use]
        #[inline(always)]
        pub const fn source(&self) -> u8 {
            let val = (self.0 >> 2usize) & 0x03;
            val as u8
        }
        #[doc = "desc SOURCE."]
        #[inline(always)]
        pub const fn set_source(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 2usize)) | (((val as u32) & 0x03) << 2usize);
        }
        #[doc = "desc VTDS."]
        #[must_use]
        #[inline(always)]
        pub const fn vtds(&self) -> u8 {
            let val = (self.0 >> 4usize) & 0x0f;
            val as u8
        }
        #[doc = "desc VTDS."]
        #[inline(always)]
        pub const fn set_vtds(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 4usize)) | (((val as u32) & 0x0f) << 4usize);
        }
        #[doc = "desc FLTEN."]
        #[must_use]
        #[inline(always)]
        pub const fn flten(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "desc FLTEN."]
        #[inline(always)]
        pub const fn set_flten(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "desc DEBOUNCETIME."]
        #[must_use]
        #[inline(always)]
        pub const fn debouncetime(&self) -> u8 {
            let val = (self.0 >> 9usize) & 0x07;
            val as u8
        }
        #[doc = "desc DEBOUNCETIME."]
        #[inline(always)]
        pub const fn set_debouncetime(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 9usize)) | (((val as u32) & 0x07) << 9usize);
        }
        #[doc = "desc FTEN."]
        #[must_use]
        #[inline(always)]
        pub const fn ften(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "desc FTEN."]
        #[inline(always)]
        pub const fn set_ften(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "desc RTEN."]
        #[must_use]
        #[inline(always)]
        pub const fn rten(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[doc = "desc RTEN."]
        #[inline(always)]
        pub const fn set_rten(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[doc = "desc HTEN."]
        #[must_use]
        #[inline(always)]
        pub const fn hten(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "desc HTEN."]
        #[inline(always)]
        pub const fn set_hten(&mut self, val: bool) {
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
    impl Default for Cr {
        #[inline(always)]
        fn default() -> Cr {
            Cr(0)
        }
    }
    impl core::fmt::Debug for Cr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Cr")
                .field("lvden", &self.lvden())
                .field("act", &self.act())
                .field("source", &self.source())
                .field("vtds", &self.vtds())
                .field("flten", &self.flten())
                .field("debouncetime", &self.debouncetime())
                .field("ften", &self.ften())
                .field("rten", &self.rten())
                .field("hten", &self.hten())
                .field("ie", &self.ie())
                .field("rsv", &self.rsv())
                .finish()
        }
    }
    #[doc = "desc SR."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Sr(pub u32);
    impl Sr {
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
        #[doc = "desc FILTER."]
        #[must_use]
        #[inline(always)]
        pub const fn filter(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "desc FILTER."]
        #[inline(always)]
        pub const fn set_filter(&mut self, val: bool) {
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
    impl Default for Sr {
        #[inline(always)]
        fn default() -> Sr {
            Sr(0)
        }
    }
    impl core::fmt::Debug for Sr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Sr")
                .field("intf", &self.intf())
                .field("filter", &self.filter())
                .field("rsv", &self.rsv())
                .finish()
        }
    }
}
