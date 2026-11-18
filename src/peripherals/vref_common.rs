#![allow(clippy::missing_safety_doc)]
#![allow(clippy::identity_op)]
#![allow(clippy::unnecessary_cast)]
#![allow(clippy::erasing_op)]

#[doc = "desc VREF."]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Vref {
    ptr: *mut u8,
}
unsafe impl Send for Vref {}
unsafe impl Sync for Vref {}
impl Vref {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "desc BGRCR."]
    #[inline(always)]
    pub const fn bgrcr(self) -> crate::common::Reg<regs::Bgrcr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0usize) as _) }
    }
    #[doc = "desc CSCR."]
    #[inline(always)]
    pub const fn cscr(self) -> crate::common::Reg<regs::Cscr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x04usize) as _) }
    }
    #[doc = "desc CR."]
    #[inline(always)]
    pub const fn cr(self) -> crate::common::Reg<regs::Cr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x08usize) as _) }
    }
}
pub mod regs {
    #[doc = "desc BGRCR."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Bgrcr(pub u32);
    impl Bgrcr {
        #[doc = "desc BGREN."]
        #[must_use]
        #[inline(always)]
        pub const fn bgren(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "desc BGREN."]
        #[inline(always)]
        pub const fn set_bgren(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
    }
    impl Default for Bgrcr {
        #[inline(always)]
        fn default() -> Bgrcr {
            Bgrcr(0)
        }
    }
    impl core::fmt::Debug for Bgrcr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Bgrcr")
                .field("bgren", &self.bgren())
                .finish()
        }
    }
    #[doc = "desc CR."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Cr(pub u32);
    impl Cr {
        #[doc = "desc AVCCMONEN."]
        #[must_use]
        #[inline(always)]
        pub const fn avccmonen(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "desc AVCCMONEN."]
        #[inline(always)]
        pub const fn set_avccmonen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "desc VREFINTEN."]
        #[must_use]
        #[inline(always)]
        pub const fn vrefinten(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "desc VREFINTEN."]
        #[inline(always)]
        pub const fn set_vrefinten(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "desc IVREFEN."]
        #[must_use]
        #[inline(always)]
        pub const fn ivrefen(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "desc IVREFEN."]
        #[inline(always)]
        pub const fn set_ivrefen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "desc HIZ."]
        #[must_use]
        #[inline(always)]
        pub const fn hiz(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "desc HIZ."]
        #[inline(always)]
        pub const fn set_hiz(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "desc AVCCMONS."]
        #[must_use]
        #[inline(always)]
        pub const fn avccmons(&self) -> u8 {
            let val = (self.0 >> 4usize) & 0x03;
            val as u8
        }
        #[doc = "desc AVCCMONS."]
        #[inline(always)]
        pub const fn set_avccmons(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 4usize)) | (((val as u32) & 0x03) << 4usize);
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
                .field("avccmonen", &self.avccmonen())
                .field("vrefinten", &self.vrefinten())
                .field("ivrefen", &self.ivrefen())
                .field("hiz", &self.hiz())
                .field("avccmons", &self.avccmons())
                .finish()
        }
    }
    #[doc = "desc CSCR."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Cscr(pub u32);
    impl Cscr {
        #[doc = "desc CSEN."]
        #[must_use]
        #[inline(always)]
        pub const fn csen(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "desc CSEN."]
        #[inline(always)]
        pub const fn set_csen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
    }
    impl Default for Cscr {
        #[inline(always)]
        fn default() -> Cscr {
            Cscr(0)
        }
    }
    impl core::fmt::Debug for Cscr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Cscr").field("csen", &self.csen()).finish()
        }
    }
}
