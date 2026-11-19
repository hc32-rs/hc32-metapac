#![allow(clippy::missing_safety_doc)]
#![allow(clippy::identity_op)]
#![allow(clippy::unnecessary_cast)]
#![allow(clippy::erasing_op)]

#[doc = "desc KEYSCAN."]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Keyscan {
    ptr: *mut u8,
}
unsafe impl Send for Keyscan {}
unsafe impl Sync for Keyscan {}
impl Keyscan {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "desc SCR."]
    #[inline(always)]
    pub const fn scr(self) -> crate::common::Reg<regs::Scr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0usize) as _) }
    }
    #[doc = "desc SER."]
    #[inline(always)]
    pub const fn ser(self) -> crate::common::Reg<regs::Ser, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x04usize) as _) }
    }
    #[doc = "desc SSR."]
    #[inline(always)]
    pub const fn ssr(self) -> crate::common::Reg<regs::Ssr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x08usize) as _) }
    }
}
pub mod regs {
    #[doc = "desc SCR."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Scr(pub u32);
    impl Scr {
        #[doc = "desc KEYINSEL."]
        #[must_use]
        #[inline(always)]
        pub const fn keyinsel(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[doc = "desc KEYINSEL."]
        #[inline(always)]
        pub const fn set_keyinsel(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
        }
        #[doc = "desc KEYOUTSEL."]
        #[must_use]
        #[inline(always)]
        pub const fn keyoutsel(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x07;
            val as u8
        }
        #[doc = "desc KEYOUTSEL."]
        #[inline(always)]
        pub const fn set_keyoutsel(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 16usize)) | (((val as u32) & 0x07) << 16usize);
        }
        #[doc = "desc CKSEL."]
        #[must_use]
        #[inline(always)]
        pub const fn cksel(&self) -> u8 {
            let val = (self.0 >> 20usize) & 0x03;
            val as u8
        }
        #[doc = "desc CKSEL."]
        #[inline(always)]
        pub const fn set_cksel(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 20usize)) | (((val as u32) & 0x03) << 20usize);
        }
        #[doc = "desc T_LLEVEL."]
        #[must_use]
        #[inline(always)]
        pub const fn t_llevel(&self) -> u8 {
            let val = (self.0 >> 24usize) & 0x1f;
            val as u8
        }
        #[doc = "desc T_LLEVEL."]
        #[inline(always)]
        pub const fn set_t_llevel(&mut self, val: u8) {
            self.0 = (self.0 & !(0x1f << 24usize)) | (((val as u32) & 0x1f) << 24usize);
        }
        #[doc = "desc T_HIZ."]
        #[must_use]
        #[inline(always)]
        pub const fn t_hiz(&self) -> u8 {
            let val = (self.0 >> 29usize) & 0x07;
            val as u8
        }
        #[doc = "desc T_HIZ."]
        #[inline(always)]
        pub const fn set_t_hiz(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 29usize)) | (((val as u32) & 0x07) << 29usize);
        }
    }
    impl Default for Scr {
        #[inline(always)]
        fn default() -> Scr {
            Scr(0)
        }
    }
    impl core::fmt::Debug for Scr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Scr")
                .field("keyinsel", &self.keyinsel())
                .field("keyoutsel", &self.keyoutsel())
                .field("cksel", &self.cksel())
                .field("t_llevel", &self.t_llevel())
                .field("t_hiz", &self.t_hiz())
                .finish()
        }
    }
    #[doc = "desc SER."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ser(pub u32);
    impl Ser {
        #[doc = "desc SEN."]
        #[must_use]
        #[inline(always)]
        pub const fn sen(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "desc SEN."]
        #[inline(always)]
        pub const fn set_sen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
    }
    impl Default for Ser {
        #[inline(always)]
        fn default() -> Ser {
            Ser(0)
        }
    }
    impl core::fmt::Debug for Ser {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Ser").field("sen", &self.sen()).finish()
        }
    }
    #[doc = "desc SSR."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ssr(pub u32);
    impl Ssr {
        #[doc = "desc INDEX."]
        #[must_use]
        #[inline(always)]
        pub const fn index(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x07;
            val as u8
        }
        #[doc = "desc INDEX."]
        #[inline(always)]
        pub const fn set_index(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 0usize)) | (((val as u32) & 0x07) << 0usize);
        }
    }
    impl Default for Ssr {
        #[inline(always)]
        fn default() -> Ssr {
            Ssr(0)
        }
    }
    impl core::fmt::Debug for Ssr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Ssr").field("index", &self.index()).finish()
        }
    }
}
