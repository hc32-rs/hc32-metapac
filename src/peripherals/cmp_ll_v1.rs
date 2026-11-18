#![allow(clippy::missing_safety_doc)]
#![allow(clippy::identity_op)]
#![allow(clippy::unnecessary_cast)]
#![allow(clippy::erasing_op)]

#[doc = "desc CMP."]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cmp {
    ptr: *mut u8,
}
unsafe impl Send for Cmp {}
unsafe impl Sync for Cmp {}
impl Cmp {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "desc CTRL."]
    #[inline(always)]
    pub const fn ctrl(self) -> crate::common::Reg<regs::Ctrl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0usize) as _) }
    }
    #[doc = "desc VLTSEL."]
    #[inline(always)]
    pub const fn vltsel(self) -> crate::common::Reg<regs::Vltsel, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x02usize) as _) }
    }
    #[doc = "desc OUTMON."]
    #[inline(always)]
    pub const fn outmon(self) -> crate::common::Reg<regs::Outmon, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x04usize) as _) }
    }
    #[doc = "desc CVSSTB."]
    #[inline(always)]
    pub const fn cvsstb(self) -> crate::common::Reg<regs::Cvsstb, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x06usize) as _) }
    }
    #[doc = "desc CVSPRD."]
    #[inline(always)]
    pub const fn cvsprd(self) -> crate::common::Reg<regs::Cvsprd, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x08usize) as _) }
    }
}
pub mod regs {
    #[doc = "desc CTRL."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ctrl(pub u16);
    impl Ctrl {
        #[doc = "desc FLTSL."]
        #[must_use]
        #[inline(always)]
        pub const fn fltsl(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x07;
            val as u8
        }
        #[doc = "desc FLTSL."]
        #[inline(always)]
        pub const fn set_fltsl(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 0usize)) | (((val as u16) & 0x07) << 0usize);
        }
        #[doc = "desc EDGSL."]
        #[must_use]
        #[inline(always)]
        pub const fn edgsl(&self) -> u8 {
            let val = (self.0 >> 5usize) & 0x03;
            val as u8
        }
        #[doc = "desc EDGSL."]
        #[inline(always)]
        pub const fn set_edgsl(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 5usize)) | (((val as u16) & 0x03) << 5usize);
        }
        #[doc = "desc IEN."]
        #[must_use]
        #[inline(always)]
        pub const fn ien(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "desc IEN."]
        #[inline(always)]
        pub const fn set_ien(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u16) & 0x01) << 7usize);
        }
        #[doc = "desc CVSEN."]
        #[must_use]
        #[inline(always)]
        pub const fn cvsen(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "desc CVSEN."]
        #[inline(always)]
        pub const fn set_cvsen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u16) & 0x01) << 8usize);
        }
        #[doc = "desc OUTEN."]
        #[must_use]
        #[inline(always)]
        pub const fn outen(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "desc OUTEN."]
        #[inline(always)]
        pub const fn set_outen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u16) & 0x01) << 12usize);
        }
        #[doc = "desc INV."]
        #[must_use]
        #[inline(always)]
        pub const fn inv(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[doc = "desc INV."]
        #[inline(always)]
        pub const fn set_inv(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u16) & 0x01) << 13usize);
        }
        #[doc = "desc CMPOE."]
        #[must_use]
        #[inline(always)]
        pub const fn cmpoe(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "desc CMPOE."]
        #[inline(always)]
        pub const fn set_cmpoe(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u16) & 0x01) << 14usize);
        }
        #[doc = "desc CMPON."]
        #[must_use]
        #[inline(always)]
        pub const fn cmpon(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "desc CMPON."]
        #[inline(always)]
        pub const fn set_cmpon(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u16) & 0x01) << 15usize);
        }
    }
    impl Default for Ctrl {
        #[inline(always)]
        fn default() -> Ctrl {
            Ctrl(0)
        }
    }
    impl core::fmt::Debug for Ctrl {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Ctrl")
                .field("fltsl", &self.fltsl())
                .field("edgsl", &self.edgsl())
                .field("ien", &self.ien())
                .field("cvsen", &self.cvsen())
                .field("outen", &self.outen())
                .field("inv", &self.inv())
                .field("cmpoe", &self.cmpoe())
                .field("cmpon", &self.cmpon())
                .finish()
        }
    }
    #[doc = "desc CVSPRD."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Cvsprd(pub u16);
    impl Cvsprd {
        #[doc = "desc PRD."]
        #[must_use]
        #[inline(always)]
        pub const fn prd(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[doc = "desc PRD."]
        #[inline(always)]
        pub const fn set_prd(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u16) & 0xff) << 0usize);
        }
    }
    impl Default for Cvsprd {
        #[inline(always)]
        fn default() -> Cvsprd {
            Cvsprd(0)
        }
    }
    impl core::fmt::Debug for Cvsprd {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Cvsprd").field("prd", &self.prd()).finish()
        }
    }
    #[doc = "desc CVSSTB."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Cvsstb(pub u16);
    impl Cvsstb {
        #[doc = "desc STB."]
        #[must_use]
        #[inline(always)]
        pub const fn stb(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x0f;
            val as u8
        }
        #[doc = "desc STB."]
        #[inline(always)]
        pub const fn set_stb(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u16) & 0x0f) << 0usize);
        }
    }
    impl Default for Cvsstb {
        #[inline(always)]
        fn default() -> Cvsstb {
            Cvsstb(0)
        }
    }
    impl core::fmt::Debug for Cvsstb {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Cvsstb").field("stb", &self.stb()).finish()
        }
    }
    #[doc = "desc OUTMON."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Outmon(pub u16);
    impl Outmon {
        #[doc = "desc OMON."]
        #[must_use]
        #[inline(always)]
        pub const fn omon(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "desc OMON."]
        #[inline(always)]
        pub const fn set_omon(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u16) & 0x01) << 0usize);
        }
        #[doc = "desc CVST."]
        #[must_use]
        #[inline(always)]
        pub const fn cvst(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0x0f;
            val as u8
        }
        #[doc = "desc CVST."]
        #[inline(always)]
        pub const fn set_cvst(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 8usize)) | (((val as u16) & 0x0f) << 8usize);
        }
    }
    impl Default for Outmon {
        #[inline(always)]
        fn default() -> Outmon {
            Outmon(0)
        }
    }
    impl core::fmt::Debug for Outmon {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Outmon")
                .field("omon", &self.omon())
                .field("cvst", &self.cvst())
                .finish()
        }
    }
    #[doc = "desc VLTSEL."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Vltsel(pub u16);
    impl Vltsel {
        #[doc = "desc RVSL."]
        #[must_use]
        #[inline(always)]
        pub const fn rvsl(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x0f;
            val as u8
        }
        #[doc = "desc RVSL."]
        #[inline(always)]
        pub const fn set_rvsl(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u16) & 0x0f) << 0usize);
        }
        #[doc = "desc CVSL."]
        #[must_use]
        #[inline(always)]
        pub const fn cvsl(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0x0f;
            val as u8
        }
        #[doc = "desc CVSL."]
        #[inline(always)]
        pub const fn set_cvsl(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 8usize)) | (((val as u16) & 0x0f) << 8usize);
        }
        #[doc = "desc C4SL."]
        #[must_use]
        #[inline(always)]
        pub const fn c4sl(&self) -> u8 {
            let val = (self.0 >> 12usize) & 0x07;
            val as u8
        }
        #[doc = "desc C4SL."]
        #[inline(always)]
        pub const fn set_c4sl(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 12usize)) | (((val as u16) & 0x07) << 12usize);
        }
    }
    impl Default for Vltsel {
        #[inline(always)]
        fn default() -> Vltsel {
            Vltsel(0)
        }
    }
    impl core::fmt::Debug for Vltsel {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Vltsel")
                .field("rvsl", &self.rvsl())
                .field("cvsl", &self.cvsl())
                .field("c4sl", &self.c4sl())
                .finish()
        }
    }
}
