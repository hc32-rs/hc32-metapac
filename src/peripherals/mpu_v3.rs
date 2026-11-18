#![allow(clippy::missing_safety_doc)]
#![allow(clippy::identity_op)]
#![allow(clippy::unnecessary_cast)]
#![allow(clippy::erasing_op)]

#[doc = "desc MPU."]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Mpu {
    ptr: *mut u8,
}
unsafe impl Send for Mpu {}
unsafe impl Sync for Mpu {}
impl Mpu {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "desc RGD0."]
    #[inline(always)]
    pub const fn rgd0(self) -> crate::common::Reg<regs::Rgd0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0usize) as _) }
    }
    #[doc = "desc RGD1."]
    #[inline(always)]
    pub const fn rgd1(self) -> crate::common::Reg<regs::Rgd1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x04usize) as _) }
    }
    #[doc = "desc RGD2."]
    #[inline(always)]
    pub const fn rgd2(self) -> crate::common::Reg<regs::Rgd2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x08usize) as _) }
    }
    #[doc = "desc RGD3."]
    #[inline(always)]
    pub const fn rgd3(self) -> crate::common::Reg<regs::Rgd3, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0cusize) as _) }
    }
    #[doc = "desc RGD4."]
    #[inline(always)]
    pub const fn rgd4(self) -> crate::common::Reg<regs::Rgd4, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x10usize) as _) }
    }
    #[doc = "desc RGD5."]
    #[inline(always)]
    pub const fn rgd5(self) -> crate::common::Reg<regs::Rgd5, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x14usize) as _) }
    }
    #[doc = "desc RGD6."]
    #[inline(always)]
    pub const fn rgd6(self) -> crate::common::Reg<regs::Rgd6, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x18usize) as _) }
    }
    #[doc = "desc RGD7."]
    #[inline(always)]
    pub const fn rgd7(self) -> crate::common::Reg<regs::Rgd7, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1cusize) as _) }
    }
    #[doc = "desc RGD8."]
    #[inline(always)]
    pub const fn rgd8(self) -> crate::common::Reg<regs::Rgd8, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x20usize) as _) }
    }
    #[doc = "desc RGD9."]
    #[inline(always)]
    pub const fn rgd9(self) -> crate::common::Reg<regs::Rgd9, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x24usize) as _) }
    }
    #[doc = "desc RGD10."]
    #[inline(always)]
    pub const fn rgd10(self) -> crate::common::Reg<regs::Rgd10, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x28usize) as _) }
    }
    #[doc = "desc RGD11."]
    #[inline(always)]
    pub const fn rgd11(self) -> crate::common::Reg<regs::Rgd11, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x2cusize) as _) }
    }
    #[doc = "desc RGD12."]
    #[inline(always)]
    pub const fn rgd12(self) -> crate::common::Reg<regs::Rgd12, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x30usize) as _) }
    }
    #[doc = "desc RGD13."]
    #[inline(always)]
    pub const fn rgd13(self) -> crate::common::Reg<regs::Rgd13, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x34usize) as _) }
    }
    #[doc = "desc RGD14."]
    #[inline(always)]
    pub const fn rgd14(self) -> crate::common::Reg<regs::Rgd14, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x38usize) as _) }
    }
    #[doc = "desc RGD15."]
    #[inline(always)]
    pub const fn rgd15(self) -> crate::common::Reg<regs::Rgd15, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x3cusize) as _) }
    }
    #[doc = "desc SR."]
    #[inline(always)]
    pub const fn sr(self) -> crate::common::Reg<regs::Sr, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x40usize) as _) }
    }
    #[doc = "desc ECLR."]
    #[inline(always)]
    pub const fn eclr(self) -> crate::common::Reg<regs::Eclr, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x44usize) as _) }
    }
    #[doc = "desc WP."]
    #[inline(always)]
    pub const fn wp(self) -> crate::common::Reg<regs::Wp, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x48usize) as _) }
    }
    #[doc = "desc IPPR."]
    #[inline(always)]
    pub const fn ippr(self) -> crate::common::Reg<regs::Ippr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x4cusize) as _) }
    }
    #[doc = "desc MSPPBA."]
    #[inline(always)]
    pub const fn msppba(self) -> crate::common::Reg<regs::Msppba, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x50usize) as _) }
    }
    #[doc = "desc MSPPCTL."]
    #[inline(always)]
    pub const fn msppctl(self) -> crate::common::Reg<regs::Msppctl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x54usize) as _) }
    }
    #[doc = "desc PSPPBA."]
    #[inline(always)]
    pub const fn psppba(self) -> crate::common::Reg<regs::Psppba, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x58usize) as _) }
    }
    #[doc = "desc PSPPCTL."]
    #[inline(always)]
    pub const fn psppctl(self) -> crate::common::Reg<regs::Psppctl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x5cusize) as _) }
    }
    #[doc = "desc S1RGE."]
    #[inline(always)]
    pub const fn s1rge(self) -> crate::common::Reg<regs::S1rge, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x60usize) as _) }
    }
    #[doc = "desc S1RGWP."]
    #[inline(always)]
    pub const fn s1rgwp(self) -> crate::common::Reg<regs::S1rgwp, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x64usize) as _) }
    }
    #[doc = "desc S1RGRP."]
    #[inline(always)]
    pub const fn s1rgrp(self) -> crate::common::Reg<regs::S1rgrp, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x68usize) as _) }
    }
    #[doc = "desc S1CR."]
    #[inline(always)]
    pub const fn s1cr(self) -> crate::common::Reg<regs::S1cr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x6cusize) as _) }
    }
    #[doc = "desc S2RGE."]
    #[inline(always)]
    pub const fn s2rge(self) -> crate::common::Reg<regs::S2rge, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x70usize) as _) }
    }
    #[doc = "desc S2RGWP."]
    #[inline(always)]
    pub const fn s2rgwp(self) -> crate::common::Reg<regs::S2rgwp, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x74usize) as _) }
    }
    #[doc = "desc S2RGRP."]
    #[inline(always)]
    pub const fn s2rgrp(self) -> crate::common::Reg<regs::S2rgrp, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x78usize) as _) }
    }
    #[doc = "desc S2CR."]
    #[inline(always)]
    pub const fn s2cr(self) -> crate::common::Reg<regs::S2cr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x7cusize) as _) }
    }
    #[doc = "desc FRGE."]
    #[inline(always)]
    pub const fn frge(self) -> crate::common::Reg<regs::Frge, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x80usize) as _) }
    }
    #[doc = "desc FRGWP."]
    #[inline(always)]
    pub const fn frgwp(self) -> crate::common::Reg<regs::Frgwp, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x84usize) as _) }
    }
    #[doc = "desc FRGRP."]
    #[inline(always)]
    pub const fn frgrp(self) -> crate::common::Reg<regs::Frgrp, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x88usize) as _) }
    }
    #[doc = "desc FCR."]
    #[inline(always)]
    pub const fn fcr(self) -> crate::common::Reg<regs::Fcr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x8cusize) as _) }
    }
}
pub mod regs {
    #[doc = "desc ECLR."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Eclr(pub u32);
    impl Eclr {
        #[doc = "desc SMPU1ECLR."]
        #[must_use]
        #[inline(always)]
        pub const fn smpu1eclr(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "desc SMPU1ECLR."]
        #[inline(always)]
        pub const fn set_smpu1eclr(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "desc SMPU2ECLR."]
        #[must_use]
        #[inline(always)]
        pub const fn smpu2eclr(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "desc SMPU2ECLR."]
        #[inline(always)]
        pub const fn set_smpu2eclr(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "desc FMPUECLR."]
        #[must_use]
        #[inline(always)]
        pub const fn fmpueclr(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "desc FMPUECLR."]
        #[inline(always)]
        pub const fn set_fmpueclr(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "desc PSPECLR."]
        #[must_use]
        #[inline(always)]
        pub const fn pspeclr(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "desc PSPECLR."]
        #[inline(always)]
        pub const fn set_pspeclr(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "desc MSPECLR."]
        #[must_use]
        #[inline(always)]
        pub const fn mspeclr(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "desc MSPECLR."]
        #[inline(always)]
        pub const fn set_mspeclr(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
    }
    impl Default for Eclr {
        #[inline(always)]
        fn default() -> Eclr {
            Eclr(0)
        }
    }
    impl core::fmt::Debug for Eclr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Eclr")
                .field("smpu1eclr", &self.smpu1eclr())
                .field("smpu2eclr", &self.smpu2eclr())
                .field("fmpueclr", &self.fmpueclr())
                .field("pspeclr", &self.pspeclr())
                .field("mspeclr", &self.mspeclr())
                .finish()
        }
    }
    #[doc = "desc FCR."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Fcr(pub u32);
    impl Fcr {
        #[doc = "desc FMPUBRP."]
        #[must_use]
        #[inline(always)]
        pub const fn fmpubrp(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "desc FMPUBRP."]
        #[inline(always)]
        pub const fn set_fmpubrp(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "desc FMPUBWP."]
        #[must_use]
        #[inline(always)]
        pub const fn fmpubwp(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "desc FMPUBWP."]
        #[inline(always)]
        pub const fn set_fmpubwp(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "desc FMPUACT."]
        #[must_use]
        #[inline(always)]
        pub const fn fmpuact(&self) -> u8 {
            let val = (self.0 >> 2usize) & 0x03;
            val as u8
        }
        #[doc = "desc FMPUACT."]
        #[inline(always)]
        pub const fn set_fmpuact(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 2usize)) | (((val as u32) & 0x03) << 2usize);
        }
        #[doc = "desc FMPUE."]
        #[must_use]
        #[inline(always)]
        pub const fn fmpue(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "desc FMPUE."]
        #[inline(always)]
        pub const fn set_fmpue(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
    }
    impl Default for Fcr {
        #[inline(always)]
        fn default() -> Fcr {
            Fcr(0)
        }
    }
    impl core::fmt::Debug for Fcr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Fcr")
                .field("fmpubrp", &self.fmpubrp())
                .field("fmpubwp", &self.fmpubwp())
                .field("fmpuact", &self.fmpuact())
                .field("fmpue", &self.fmpue())
                .finish()
        }
    }
    #[doc = "desc FRGE."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Frge(pub u32);
    impl Frge {
        #[doc = "desc FRG0E."]
        #[must_use]
        #[inline(always)]
        pub const fn frg0e(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "desc FRG0E."]
        #[inline(always)]
        pub const fn set_frg0e(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "desc FRG1E."]
        #[must_use]
        #[inline(always)]
        pub const fn frg1e(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "desc FRG1E."]
        #[inline(always)]
        pub const fn set_frg1e(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "desc FRG2E."]
        #[must_use]
        #[inline(always)]
        pub const fn frg2e(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "desc FRG2E."]
        #[inline(always)]
        pub const fn set_frg2e(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "desc FRG3E."]
        #[must_use]
        #[inline(always)]
        pub const fn frg3e(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "desc FRG3E."]
        #[inline(always)]
        pub const fn set_frg3e(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "desc FRG4E."]
        #[must_use]
        #[inline(always)]
        pub const fn frg4e(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "desc FRG4E."]
        #[inline(always)]
        pub const fn set_frg4e(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "desc FRG5E."]
        #[must_use]
        #[inline(always)]
        pub const fn frg5e(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "desc FRG5E."]
        #[inline(always)]
        pub const fn set_frg5e(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "desc FRG6E."]
        #[must_use]
        #[inline(always)]
        pub const fn frg6e(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "desc FRG6E."]
        #[inline(always)]
        pub const fn set_frg6e(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "desc FRG7E."]
        #[must_use]
        #[inline(always)]
        pub const fn frg7e(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "desc FRG7E."]
        #[inline(always)]
        pub const fn set_frg7e(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
    }
    impl Default for Frge {
        #[inline(always)]
        fn default() -> Frge {
            Frge(0)
        }
    }
    impl core::fmt::Debug for Frge {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Frge")
                .field("frg0e", &self.frg0e())
                .field("frg1e", &self.frg1e())
                .field("frg2e", &self.frg2e())
                .field("frg3e", &self.frg3e())
                .field("frg4e", &self.frg4e())
                .field("frg5e", &self.frg5e())
                .field("frg6e", &self.frg6e())
                .field("frg7e", &self.frg7e())
                .finish()
        }
    }
    #[doc = "desc FRGRP."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Frgrp(pub u32);
    impl Frgrp {
        #[doc = "desc FRG0RP."]
        #[must_use]
        #[inline(always)]
        pub const fn frg0rp(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "desc FRG0RP."]
        #[inline(always)]
        pub const fn set_frg0rp(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "desc FRG1RP."]
        #[must_use]
        #[inline(always)]
        pub const fn frg1rp(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "desc FRG1RP."]
        #[inline(always)]
        pub const fn set_frg1rp(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "desc FRG2RP."]
        #[must_use]
        #[inline(always)]
        pub const fn frg2rp(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "desc FRG2RP."]
        #[inline(always)]
        pub const fn set_frg2rp(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "desc FRG3RP."]
        #[must_use]
        #[inline(always)]
        pub const fn frg3rp(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "desc FRG3RP."]
        #[inline(always)]
        pub const fn set_frg3rp(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "desc FRG4RP."]
        #[must_use]
        #[inline(always)]
        pub const fn frg4rp(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "desc FRG4RP."]
        #[inline(always)]
        pub const fn set_frg4rp(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "desc FRG5RP."]
        #[must_use]
        #[inline(always)]
        pub const fn frg5rp(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "desc FRG5RP."]
        #[inline(always)]
        pub const fn set_frg5rp(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "desc FRG6RP."]
        #[must_use]
        #[inline(always)]
        pub const fn frg6rp(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "desc FRG6RP."]
        #[inline(always)]
        pub const fn set_frg6rp(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "desc FRG7RP."]
        #[must_use]
        #[inline(always)]
        pub const fn frg7rp(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "desc FRG7RP."]
        #[inline(always)]
        pub const fn set_frg7rp(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
    }
    impl Default for Frgrp {
        #[inline(always)]
        fn default() -> Frgrp {
            Frgrp(0)
        }
    }
    impl core::fmt::Debug for Frgrp {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Frgrp")
                .field("frg0rp", &self.frg0rp())
                .field("frg1rp", &self.frg1rp())
                .field("frg2rp", &self.frg2rp())
                .field("frg3rp", &self.frg3rp())
                .field("frg4rp", &self.frg4rp())
                .field("frg5rp", &self.frg5rp())
                .field("frg6rp", &self.frg6rp())
                .field("frg7rp", &self.frg7rp())
                .finish()
        }
    }
    #[doc = "desc FRGWP."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Frgwp(pub u32);
    impl Frgwp {
        #[doc = "desc FRG0WP."]
        #[must_use]
        #[inline(always)]
        pub const fn frg0wp(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "desc FRG0WP."]
        #[inline(always)]
        pub const fn set_frg0wp(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "desc FRG1WP."]
        #[must_use]
        #[inline(always)]
        pub const fn frg1wp(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "desc FRG1WP."]
        #[inline(always)]
        pub const fn set_frg1wp(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "desc FRG2WP."]
        #[must_use]
        #[inline(always)]
        pub const fn frg2wp(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "desc FRG2WP."]
        #[inline(always)]
        pub const fn set_frg2wp(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "desc FRG3WP."]
        #[must_use]
        #[inline(always)]
        pub const fn frg3wp(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "desc FRG3WP."]
        #[inline(always)]
        pub const fn set_frg3wp(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "desc FRG4WP."]
        #[must_use]
        #[inline(always)]
        pub const fn frg4wp(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "desc FRG4WP."]
        #[inline(always)]
        pub const fn set_frg4wp(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "desc FRG5WP."]
        #[must_use]
        #[inline(always)]
        pub const fn frg5wp(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "desc FRG5WP."]
        #[inline(always)]
        pub const fn set_frg5wp(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "desc FRG6WP."]
        #[must_use]
        #[inline(always)]
        pub const fn frg6wp(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "desc FRG6WP."]
        #[inline(always)]
        pub const fn set_frg6wp(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "desc FRG7WP."]
        #[must_use]
        #[inline(always)]
        pub const fn frg7wp(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "desc FRG7WP."]
        #[inline(always)]
        pub const fn set_frg7wp(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
    }
    impl Default for Frgwp {
        #[inline(always)]
        fn default() -> Frgwp {
            Frgwp(0)
        }
    }
    impl core::fmt::Debug for Frgwp {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Frgwp")
                .field("frg0wp", &self.frg0wp())
                .field("frg1wp", &self.frg1wp())
                .field("frg2wp", &self.frg2wp())
                .field("frg3wp", &self.frg3wp())
                .field("frg4wp", &self.frg4wp())
                .field("frg5wp", &self.frg5wp())
                .field("frg6wp", &self.frg6wp())
                .field("frg7wp", &self.frg7wp())
                .finish()
        }
    }
    #[doc = "desc IPPR."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ippr(pub u32);
    impl Ippr {
        #[doc = "desc AESRDP."]
        #[must_use]
        #[inline(always)]
        pub const fn aesrdp(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "desc AESRDP."]
        #[inline(always)]
        pub const fn set_aesrdp(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "desc AESWRP."]
        #[must_use]
        #[inline(always)]
        pub const fn aeswrp(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "desc AESWRP."]
        #[inline(always)]
        pub const fn set_aeswrp(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "desc HASHRDP."]
        #[must_use]
        #[inline(always)]
        pub const fn hashrdp(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "desc HASHRDP."]
        #[inline(always)]
        pub const fn set_hashrdp(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "desc HASHWRP."]
        #[must_use]
        #[inline(always)]
        pub const fn hashwrp(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "desc HASHWRP."]
        #[inline(always)]
        pub const fn set_hashwrp(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "desc TRNGRDP."]
        #[must_use]
        #[inline(always)]
        pub const fn trngrdp(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "desc TRNGRDP."]
        #[inline(always)]
        pub const fn set_trngrdp(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "desc TRNGWRP."]
        #[must_use]
        #[inline(always)]
        pub const fn trngwrp(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "desc TRNGWRP."]
        #[inline(always)]
        pub const fn set_trngwrp(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "desc CRCRDP."]
        #[must_use]
        #[inline(always)]
        pub const fn crcrdp(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "desc CRCRDP."]
        #[inline(always)]
        pub const fn set_crcrdp(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "desc CRCWRP."]
        #[must_use]
        #[inline(always)]
        pub const fn crcwrp(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "desc CRCWRP."]
        #[inline(always)]
        pub const fn set_crcwrp(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "desc EFMRDP."]
        #[must_use]
        #[inline(always)]
        pub const fn efmrdp(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "desc EFMRDP."]
        #[inline(always)]
        pub const fn set_efmrdp(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "desc EFMWRP."]
        #[must_use]
        #[inline(always)]
        pub const fn efmwrp(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "desc EFMWRP."]
        #[inline(always)]
        pub const fn set_efmwrp(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "desc WDTRDP."]
        #[must_use]
        #[inline(always)]
        pub const fn wdtrdp(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "desc WDTRDP."]
        #[inline(always)]
        pub const fn set_wdtrdp(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "desc WDTWRP."]
        #[must_use]
        #[inline(always)]
        pub const fn wdtwrp(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[doc = "desc WDTWRP."]
        #[inline(always)]
        pub const fn set_wdtwrp(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[doc = "desc SWDTRDP."]
        #[must_use]
        #[inline(always)]
        pub const fn swdtrdp(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "desc SWDTRDP."]
        #[inline(always)]
        pub const fn set_swdtrdp(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[doc = "desc SWDTWRP."]
        #[must_use]
        #[inline(always)]
        pub const fn swdtwrp(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "desc SWDTWRP."]
        #[inline(always)]
        pub const fn set_swdtwrp(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
        #[doc = "desc BKSRAMRDP."]
        #[must_use]
        #[inline(always)]
        pub const fn bksramrdp(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "desc BKSRAMRDP."]
        #[inline(always)]
        pub const fn set_bksramrdp(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "desc BKSRAMWRP."]
        #[must_use]
        #[inline(always)]
        pub const fn bksramwrp(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "desc BKSRAMWRP."]
        #[inline(always)]
        pub const fn set_bksramwrp(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[doc = "desc RTCRDP."]
        #[must_use]
        #[inline(always)]
        pub const fn rtcrdp(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[doc = "desc RTCRDP."]
        #[inline(always)]
        pub const fn set_rtcrdp(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
        #[doc = "desc RTCWRP."]
        #[must_use]
        #[inline(always)]
        pub const fn rtcwrp(&self) -> bool {
            let val = (self.0 >> 19usize) & 0x01;
            val != 0
        }
        #[doc = "desc RTCWRP."]
        #[inline(always)]
        pub const fn set_rtcwrp(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
        }
        #[doc = "desc DMPURDP."]
        #[must_use]
        #[inline(always)]
        pub const fn dmpurdp(&self) -> bool {
            let val = (self.0 >> 20usize) & 0x01;
            val != 0
        }
        #[doc = "desc DMPURDP."]
        #[inline(always)]
        pub const fn set_dmpurdp(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
        }
        #[doc = "desc DMPUWRP."]
        #[must_use]
        #[inline(always)]
        pub const fn dmpuwrp(&self) -> bool {
            let val = (self.0 >> 21usize) & 0x01;
            val != 0
        }
        #[doc = "desc DMPUWRP."]
        #[inline(always)]
        pub const fn set_dmpuwrp(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
        }
        #[doc = "desc SRAMCRDP."]
        #[must_use]
        #[inline(always)]
        pub const fn sramcrdp(&self) -> bool {
            let val = (self.0 >> 22usize) & 0x01;
            val != 0
        }
        #[doc = "desc SRAMCRDP."]
        #[inline(always)]
        pub const fn set_sramcrdp(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
        }
        #[doc = "desc SRAMCWRP."]
        #[must_use]
        #[inline(always)]
        pub const fn sramcwrp(&self) -> bool {
            let val = (self.0 >> 23usize) & 0x01;
            val != 0
        }
        #[doc = "desc SRAMCWRP."]
        #[inline(always)]
        pub const fn set_sramcwrp(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
        }
        #[doc = "desc INTCRDP."]
        #[must_use]
        #[inline(always)]
        pub const fn intcrdp(&self) -> bool {
            let val = (self.0 >> 24usize) & 0x01;
            val != 0
        }
        #[doc = "desc INTCRDP."]
        #[inline(always)]
        pub const fn set_intcrdp(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
        }
        #[doc = "desc INTCWRP."]
        #[must_use]
        #[inline(always)]
        pub const fn intcwrp(&self) -> bool {
            let val = (self.0 >> 25usize) & 0x01;
            val != 0
        }
        #[doc = "desc INTCWRP."]
        #[inline(always)]
        pub const fn set_intcwrp(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
        }
        #[doc = "desc SYSCRDP."]
        #[must_use]
        #[inline(always)]
        pub const fn syscrdp(&self) -> bool {
            let val = (self.0 >> 26usize) & 0x01;
            val != 0
        }
        #[doc = "desc SYSCRDP."]
        #[inline(always)]
        pub const fn set_syscrdp(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
        }
        #[doc = "desc SYSCWRP."]
        #[must_use]
        #[inline(always)]
        pub const fn syscwrp(&self) -> bool {
            let val = (self.0 >> 27usize) & 0x01;
            val != 0
        }
        #[doc = "desc SYSCWRP."]
        #[inline(always)]
        pub const fn set_syscwrp(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
        }
        #[doc = "desc MSTPRDP."]
        #[must_use]
        #[inline(always)]
        pub const fn mstprdp(&self) -> bool {
            let val = (self.0 >> 28usize) & 0x01;
            val != 0
        }
        #[doc = "desc MSTPRDP."]
        #[inline(always)]
        pub const fn set_mstprdp(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
        }
        #[doc = "desc MSPTWRP."]
        #[must_use]
        #[inline(always)]
        pub const fn msptwrp(&self) -> bool {
            let val = (self.0 >> 29usize) & 0x01;
            val != 0
        }
        #[doc = "desc MSPTWRP."]
        #[inline(always)]
        pub const fn set_msptwrp(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
        }
        #[doc = "desc BUSERRE."]
        #[must_use]
        #[inline(always)]
        pub const fn buserre(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[doc = "desc BUSERRE."]
        #[inline(always)]
        pub const fn set_buserre(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for Ippr {
        #[inline(always)]
        fn default() -> Ippr {
            Ippr(0)
        }
    }
    impl core::fmt::Debug for Ippr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Ippr")
                .field("aesrdp", &self.aesrdp())
                .field("aeswrp", &self.aeswrp())
                .field("hashrdp", &self.hashrdp())
                .field("hashwrp", &self.hashwrp())
                .field("trngrdp", &self.trngrdp())
                .field("trngwrp", &self.trngwrp())
                .field("crcrdp", &self.crcrdp())
                .field("crcwrp", &self.crcwrp())
                .field("efmrdp", &self.efmrdp())
                .field("efmwrp", &self.efmwrp())
                .field("wdtrdp", &self.wdtrdp())
                .field("wdtwrp", &self.wdtwrp())
                .field("swdtrdp", &self.swdtrdp())
                .field("swdtwrp", &self.swdtwrp())
                .field("bksramrdp", &self.bksramrdp())
                .field("bksramwrp", &self.bksramwrp())
                .field("rtcrdp", &self.rtcrdp())
                .field("rtcwrp", &self.rtcwrp())
                .field("dmpurdp", &self.dmpurdp())
                .field("dmpuwrp", &self.dmpuwrp())
                .field("sramcrdp", &self.sramcrdp())
                .field("sramcwrp", &self.sramcwrp())
                .field("intcrdp", &self.intcrdp())
                .field("intcwrp", &self.intcwrp())
                .field("syscrdp", &self.syscrdp())
                .field("syscwrp", &self.syscwrp())
                .field("mstprdp", &self.mstprdp())
                .field("msptwrp", &self.msptwrp())
                .field("buserre", &self.buserre())
                .finish()
        }
    }
    #[doc = "desc MSPPBA."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Msppba(pub u32);
    impl Msppba {
        #[doc = "desc MSPPBA."]
        #[must_use]
        #[inline(always)]
        pub const fn msppba(&self) -> u32 {
            let val = (self.0 >> 2usize) & 0x3fff_ffff;
            val as u32
        }
        #[doc = "desc MSPPBA."]
        #[inline(always)]
        pub const fn set_msppba(&mut self, val: u32) {
            self.0 = (self.0 & !(0x3fff_ffff << 2usize)) | (((val as u32) & 0x3fff_ffff) << 2usize);
        }
    }
    impl Default for Msppba {
        #[inline(always)]
        fn default() -> Msppba {
            Msppba(0)
        }
    }
    impl core::fmt::Debug for Msppba {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Msppba")
                .field("msppba", &self.msppba())
                .finish()
        }
    }
    #[doc = "desc MSPPCTL."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Msppctl(pub u32);
    impl Msppctl {
        #[doc = "desc MSPPSIZE."]
        #[must_use]
        #[inline(always)]
        pub const fn msppsize(&self) -> u16 {
            let val = (self.0 >> 2usize) & 0x3fff;
            val as u16
        }
        #[doc = "desc MSPPSIZE."]
        #[inline(always)]
        pub const fn set_msppsize(&mut self, val: u16) {
            self.0 = (self.0 & !(0x3fff << 2usize)) | (((val as u32) & 0x3fff) << 2usize);
        }
        #[doc = "desc MSPPACT."]
        #[must_use]
        #[inline(always)]
        pub const fn msppact(&self) -> bool {
            let val = (self.0 >> 30usize) & 0x01;
            val != 0
        }
        #[doc = "desc MSPPACT."]
        #[inline(always)]
        pub const fn set_msppact(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
        }
        #[doc = "desc MSPPE."]
        #[must_use]
        #[inline(always)]
        pub const fn msppe(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[doc = "desc MSPPE."]
        #[inline(always)]
        pub const fn set_msppe(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for Msppctl {
        #[inline(always)]
        fn default() -> Msppctl {
            Msppctl(0)
        }
    }
    impl core::fmt::Debug for Msppctl {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Msppctl")
                .field("msppsize", &self.msppsize())
                .field("msppact", &self.msppact())
                .field("msppe", &self.msppe())
                .finish()
        }
    }
    #[doc = "desc PSPPBA."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Psppba(pub u32);
    impl Psppba {
        #[doc = "desc PSPPBA."]
        #[must_use]
        #[inline(always)]
        pub const fn psppba(&self) -> u32 {
            let val = (self.0 >> 2usize) & 0x3fff_ffff;
            val as u32
        }
        #[doc = "desc PSPPBA."]
        #[inline(always)]
        pub const fn set_psppba(&mut self, val: u32) {
            self.0 = (self.0 & !(0x3fff_ffff << 2usize)) | (((val as u32) & 0x3fff_ffff) << 2usize);
        }
    }
    impl Default for Psppba {
        #[inline(always)]
        fn default() -> Psppba {
            Psppba(0)
        }
    }
    impl core::fmt::Debug for Psppba {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Psppba")
                .field("psppba", &self.psppba())
                .finish()
        }
    }
    #[doc = "desc PSPPCTL."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Psppctl(pub u32);
    impl Psppctl {
        #[doc = "desc PSPPSIZE."]
        #[must_use]
        #[inline(always)]
        pub const fn psppsize(&self) -> u16 {
            let val = (self.0 >> 2usize) & 0x3fff;
            val as u16
        }
        #[doc = "desc PSPPSIZE."]
        #[inline(always)]
        pub const fn set_psppsize(&mut self, val: u16) {
            self.0 = (self.0 & !(0x3fff << 2usize)) | (((val as u32) & 0x3fff) << 2usize);
        }
        #[doc = "desc PSPPACT."]
        #[must_use]
        #[inline(always)]
        pub const fn psppact(&self) -> bool {
            let val = (self.0 >> 30usize) & 0x01;
            val != 0
        }
        #[doc = "desc PSPPACT."]
        #[inline(always)]
        pub const fn set_psppact(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
        }
        #[doc = "desc PSPPE."]
        #[must_use]
        #[inline(always)]
        pub const fn psppe(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[doc = "desc PSPPE."]
        #[inline(always)]
        pub const fn set_psppe(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for Psppctl {
        #[inline(always)]
        fn default() -> Psppctl {
            Psppctl(0)
        }
    }
    impl core::fmt::Debug for Psppctl {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Psppctl")
                .field("psppsize", &self.psppsize())
                .field("psppact", &self.psppact())
                .field("psppe", &self.psppe())
                .finish()
        }
    }
    #[doc = "desc RGD0."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Rgd0(pub u32);
    impl Rgd0 {
        #[doc = "desc MPURGSIZE."]
        #[must_use]
        #[inline(always)]
        pub const fn mpurgsize(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x1f;
            val as u8
        }
        #[doc = "desc MPURGSIZE."]
        #[inline(always)]
        pub const fn set_mpurgsize(&mut self, val: u8) {
            self.0 = (self.0 & !(0x1f << 0usize)) | (((val as u32) & 0x1f) << 0usize);
        }
        #[doc = "desc MPURGADDR."]
        #[must_use]
        #[inline(always)]
        pub const fn mpurgaddr(&self) -> u32 {
            let val = (self.0 >> 5usize) & 0x07ff_ffff;
            val as u32
        }
        #[doc = "desc MPURGADDR."]
        #[inline(always)]
        pub const fn set_mpurgaddr(&mut self, val: u32) {
            self.0 = (self.0 & !(0x07ff_ffff << 5usize)) | (((val as u32) & 0x07ff_ffff) << 5usize);
        }
    }
    impl Default for Rgd0 {
        #[inline(always)]
        fn default() -> Rgd0 {
            Rgd0(0)
        }
    }
    impl core::fmt::Debug for Rgd0 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Rgd0")
                .field("mpurgsize", &self.mpurgsize())
                .field("mpurgaddr", &self.mpurgaddr())
                .finish()
        }
    }
    #[doc = "desc RGD1."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Rgd1(pub u32);
    impl Rgd1 {
        #[doc = "desc MPURGSIZE."]
        #[must_use]
        #[inline(always)]
        pub const fn mpurgsize(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x1f;
            val as u8
        }
        #[doc = "desc MPURGSIZE."]
        #[inline(always)]
        pub const fn set_mpurgsize(&mut self, val: u8) {
            self.0 = (self.0 & !(0x1f << 0usize)) | (((val as u32) & 0x1f) << 0usize);
        }
        #[doc = "desc MPURGADDR."]
        #[must_use]
        #[inline(always)]
        pub const fn mpurgaddr(&self) -> u32 {
            let val = (self.0 >> 5usize) & 0x07ff_ffff;
            val as u32
        }
        #[doc = "desc MPURGADDR."]
        #[inline(always)]
        pub const fn set_mpurgaddr(&mut self, val: u32) {
            self.0 = (self.0 & !(0x07ff_ffff << 5usize)) | (((val as u32) & 0x07ff_ffff) << 5usize);
        }
    }
    impl Default for Rgd1 {
        #[inline(always)]
        fn default() -> Rgd1 {
            Rgd1(0)
        }
    }
    impl core::fmt::Debug for Rgd1 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Rgd1")
                .field("mpurgsize", &self.mpurgsize())
                .field("mpurgaddr", &self.mpurgaddr())
                .finish()
        }
    }
    #[doc = "desc RGD10."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Rgd10(pub u32);
    impl Rgd10 {
        #[doc = "desc MPURGSIZE."]
        #[must_use]
        #[inline(always)]
        pub const fn mpurgsize(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x1f;
            val as u8
        }
        #[doc = "desc MPURGSIZE."]
        #[inline(always)]
        pub const fn set_mpurgsize(&mut self, val: u8) {
            self.0 = (self.0 & !(0x1f << 0usize)) | (((val as u32) & 0x1f) << 0usize);
        }
        #[doc = "desc MPURGADDR."]
        #[must_use]
        #[inline(always)]
        pub const fn mpurgaddr(&self) -> u32 {
            let val = (self.0 >> 5usize) & 0x07ff_ffff;
            val as u32
        }
        #[doc = "desc MPURGADDR."]
        #[inline(always)]
        pub const fn set_mpurgaddr(&mut self, val: u32) {
            self.0 = (self.0 & !(0x07ff_ffff << 5usize)) | (((val as u32) & 0x07ff_ffff) << 5usize);
        }
    }
    impl Default for Rgd10 {
        #[inline(always)]
        fn default() -> Rgd10 {
            Rgd10(0)
        }
    }
    impl core::fmt::Debug for Rgd10 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Rgd10")
                .field("mpurgsize", &self.mpurgsize())
                .field("mpurgaddr", &self.mpurgaddr())
                .finish()
        }
    }
    #[doc = "desc RGD11."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Rgd11(pub u32);
    impl Rgd11 {
        #[doc = "desc MPURGSIZE."]
        #[must_use]
        #[inline(always)]
        pub const fn mpurgsize(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x1f;
            val as u8
        }
        #[doc = "desc MPURGSIZE."]
        #[inline(always)]
        pub const fn set_mpurgsize(&mut self, val: u8) {
            self.0 = (self.0 & !(0x1f << 0usize)) | (((val as u32) & 0x1f) << 0usize);
        }
        #[doc = "desc MPURGADDR."]
        #[must_use]
        #[inline(always)]
        pub const fn mpurgaddr(&self) -> u32 {
            let val = (self.0 >> 5usize) & 0x07ff_ffff;
            val as u32
        }
        #[doc = "desc MPURGADDR."]
        #[inline(always)]
        pub const fn set_mpurgaddr(&mut self, val: u32) {
            self.0 = (self.0 & !(0x07ff_ffff << 5usize)) | (((val as u32) & 0x07ff_ffff) << 5usize);
        }
    }
    impl Default for Rgd11 {
        #[inline(always)]
        fn default() -> Rgd11 {
            Rgd11(0)
        }
    }
    impl core::fmt::Debug for Rgd11 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Rgd11")
                .field("mpurgsize", &self.mpurgsize())
                .field("mpurgaddr", &self.mpurgaddr())
                .finish()
        }
    }
    #[doc = "desc RGD12."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Rgd12(pub u32);
    impl Rgd12 {
        #[doc = "desc MPURGSIZE."]
        #[must_use]
        #[inline(always)]
        pub const fn mpurgsize(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x1f;
            val as u8
        }
        #[doc = "desc MPURGSIZE."]
        #[inline(always)]
        pub const fn set_mpurgsize(&mut self, val: u8) {
            self.0 = (self.0 & !(0x1f << 0usize)) | (((val as u32) & 0x1f) << 0usize);
        }
        #[doc = "desc MPURGADDR."]
        #[must_use]
        #[inline(always)]
        pub const fn mpurgaddr(&self) -> u32 {
            let val = (self.0 >> 5usize) & 0x07ff_ffff;
            val as u32
        }
        #[doc = "desc MPURGADDR."]
        #[inline(always)]
        pub const fn set_mpurgaddr(&mut self, val: u32) {
            self.0 = (self.0 & !(0x07ff_ffff << 5usize)) | (((val as u32) & 0x07ff_ffff) << 5usize);
        }
    }
    impl Default for Rgd12 {
        #[inline(always)]
        fn default() -> Rgd12 {
            Rgd12(0)
        }
    }
    impl core::fmt::Debug for Rgd12 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Rgd12")
                .field("mpurgsize", &self.mpurgsize())
                .field("mpurgaddr", &self.mpurgaddr())
                .finish()
        }
    }
    #[doc = "desc RGD13."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Rgd13(pub u32);
    impl Rgd13 {
        #[doc = "desc MPURGSIZE."]
        #[must_use]
        #[inline(always)]
        pub const fn mpurgsize(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x1f;
            val as u8
        }
        #[doc = "desc MPURGSIZE."]
        #[inline(always)]
        pub const fn set_mpurgsize(&mut self, val: u8) {
            self.0 = (self.0 & !(0x1f << 0usize)) | (((val as u32) & 0x1f) << 0usize);
        }
        #[doc = "desc MPURGADDR."]
        #[must_use]
        #[inline(always)]
        pub const fn mpurgaddr(&self) -> u32 {
            let val = (self.0 >> 5usize) & 0x07ff_ffff;
            val as u32
        }
        #[doc = "desc MPURGADDR."]
        #[inline(always)]
        pub const fn set_mpurgaddr(&mut self, val: u32) {
            self.0 = (self.0 & !(0x07ff_ffff << 5usize)) | (((val as u32) & 0x07ff_ffff) << 5usize);
        }
    }
    impl Default for Rgd13 {
        #[inline(always)]
        fn default() -> Rgd13 {
            Rgd13(0)
        }
    }
    impl core::fmt::Debug for Rgd13 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Rgd13")
                .field("mpurgsize", &self.mpurgsize())
                .field("mpurgaddr", &self.mpurgaddr())
                .finish()
        }
    }
    #[doc = "desc RGD14."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Rgd14(pub u32);
    impl Rgd14 {
        #[doc = "desc MPURGSIZE."]
        #[must_use]
        #[inline(always)]
        pub const fn mpurgsize(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x1f;
            val as u8
        }
        #[doc = "desc MPURGSIZE."]
        #[inline(always)]
        pub const fn set_mpurgsize(&mut self, val: u8) {
            self.0 = (self.0 & !(0x1f << 0usize)) | (((val as u32) & 0x1f) << 0usize);
        }
        #[doc = "desc MPURGADDR."]
        #[must_use]
        #[inline(always)]
        pub const fn mpurgaddr(&self) -> u32 {
            let val = (self.0 >> 5usize) & 0x07ff_ffff;
            val as u32
        }
        #[doc = "desc MPURGADDR."]
        #[inline(always)]
        pub const fn set_mpurgaddr(&mut self, val: u32) {
            self.0 = (self.0 & !(0x07ff_ffff << 5usize)) | (((val as u32) & 0x07ff_ffff) << 5usize);
        }
    }
    impl Default for Rgd14 {
        #[inline(always)]
        fn default() -> Rgd14 {
            Rgd14(0)
        }
    }
    impl core::fmt::Debug for Rgd14 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Rgd14")
                .field("mpurgsize", &self.mpurgsize())
                .field("mpurgaddr", &self.mpurgaddr())
                .finish()
        }
    }
    #[doc = "desc RGD15."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Rgd15(pub u32);
    impl Rgd15 {
        #[doc = "desc MPURGSIZE."]
        #[must_use]
        #[inline(always)]
        pub const fn mpurgsize(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x1f;
            val as u8
        }
        #[doc = "desc MPURGSIZE."]
        #[inline(always)]
        pub const fn set_mpurgsize(&mut self, val: u8) {
            self.0 = (self.0 & !(0x1f << 0usize)) | (((val as u32) & 0x1f) << 0usize);
        }
        #[doc = "desc MPURGADDR."]
        #[must_use]
        #[inline(always)]
        pub const fn mpurgaddr(&self) -> u32 {
            let val = (self.0 >> 5usize) & 0x07ff_ffff;
            val as u32
        }
        #[doc = "desc MPURGADDR."]
        #[inline(always)]
        pub const fn set_mpurgaddr(&mut self, val: u32) {
            self.0 = (self.0 & !(0x07ff_ffff << 5usize)) | (((val as u32) & 0x07ff_ffff) << 5usize);
        }
    }
    impl Default for Rgd15 {
        #[inline(always)]
        fn default() -> Rgd15 {
            Rgd15(0)
        }
    }
    impl core::fmt::Debug for Rgd15 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Rgd15")
                .field("mpurgsize", &self.mpurgsize())
                .field("mpurgaddr", &self.mpurgaddr())
                .finish()
        }
    }
    #[doc = "desc RGD2."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Rgd2(pub u32);
    impl Rgd2 {
        #[doc = "desc MPURGSIZE."]
        #[must_use]
        #[inline(always)]
        pub const fn mpurgsize(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x1f;
            val as u8
        }
        #[doc = "desc MPURGSIZE."]
        #[inline(always)]
        pub const fn set_mpurgsize(&mut self, val: u8) {
            self.0 = (self.0 & !(0x1f << 0usize)) | (((val as u32) & 0x1f) << 0usize);
        }
        #[doc = "desc MPURGADDR."]
        #[must_use]
        #[inline(always)]
        pub const fn mpurgaddr(&self) -> u32 {
            let val = (self.0 >> 5usize) & 0x07ff_ffff;
            val as u32
        }
        #[doc = "desc MPURGADDR."]
        #[inline(always)]
        pub const fn set_mpurgaddr(&mut self, val: u32) {
            self.0 = (self.0 & !(0x07ff_ffff << 5usize)) | (((val as u32) & 0x07ff_ffff) << 5usize);
        }
    }
    impl Default for Rgd2 {
        #[inline(always)]
        fn default() -> Rgd2 {
            Rgd2(0)
        }
    }
    impl core::fmt::Debug for Rgd2 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Rgd2")
                .field("mpurgsize", &self.mpurgsize())
                .field("mpurgaddr", &self.mpurgaddr())
                .finish()
        }
    }
    #[doc = "desc RGD3."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Rgd3(pub u32);
    impl Rgd3 {
        #[doc = "desc MPURGSIZE."]
        #[must_use]
        #[inline(always)]
        pub const fn mpurgsize(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x1f;
            val as u8
        }
        #[doc = "desc MPURGSIZE."]
        #[inline(always)]
        pub const fn set_mpurgsize(&mut self, val: u8) {
            self.0 = (self.0 & !(0x1f << 0usize)) | (((val as u32) & 0x1f) << 0usize);
        }
        #[doc = "desc MPURGADDR."]
        #[must_use]
        #[inline(always)]
        pub const fn mpurgaddr(&self) -> u32 {
            let val = (self.0 >> 5usize) & 0x07ff_ffff;
            val as u32
        }
        #[doc = "desc MPURGADDR."]
        #[inline(always)]
        pub const fn set_mpurgaddr(&mut self, val: u32) {
            self.0 = (self.0 & !(0x07ff_ffff << 5usize)) | (((val as u32) & 0x07ff_ffff) << 5usize);
        }
    }
    impl Default for Rgd3 {
        #[inline(always)]
        fn default() -> Rgd3 {
            Rgd3(0)
        }
    }
    impl core::fmt::Debug for Rgd3 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Rgd3")
                .field("mpurgsize", &self.mpurgsize())
                .field("mpurgaddr", &self.mpurgaddr())
                .finish()
        }
    }
    #[doc = "desc RGD4."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Rgd4(pub u32);
    impl Rgd4 {
        #[doc = "desc MPURGSIZE."]
        #[must_use]
        #[inline(always)]
        pub const fn mpurgsize(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x1f;
            val as u8
        }
        #[doc = "desc MPURGSIZE."]
        #[inline(always)]
        pub const fn set_mpurgsize(&mut self, val: u8) {
            self.0 = (self.0 & !(0x1f << 0usize)) | (((val as u32) & 0x1f) << 0usize);
        }
        #[doc = "desc MPURGADDR."]
        #[must_use]
        #[inline(always)]
        pub const fn mpurgaddr(&self) -> u32 {
            let val = (self.0 >> 5usize) & 0x07ff_ffff;
            val as u32
        }
        #[doc = "desc MPURGADDR."]
        #[inline(always)]
        pub const fn set_mpurgaddr(&mut self, val: u32) {
            self.0 = (self.0 & !(0x07ff_ffff << 5usize)) | (((val as u32) & 0x07ff_ffff) << 5usize);
        }
    }
    impl Default for Rgd4 {
        #[inline(always)]
        fn default() -> Rgd4 {
            Rgd4(0)
        }
    }
    impl core::fmt::Debug for Rgd4 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Rgd4")
                .field("mpurgsize", &self.mpurgsize())
                .field("mpurgaddr", &self.mpurgaddr())
                .finish()
        }
    }
    #[doc = "desc RGD5."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Rgd5(pub u32);
    impl Rgd5 {
        #[doc = "desc MPURGSIZE."]
        #[must_use]
        #[inline(always)]
        pub const fn mpurgsize(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x1f;
            val as u8
        }
        #[doc = "desc MPURGSIZE."]
        #[inline(always)]
        pub const fn set_mpurgsize(&mut self, val: u8) {
            self.0 = (self.0 & !(0x1f << 0usize)) | (((val as u32) & 0x1f) << 0usize);
        }
        #[doc = "desc MPURGADDR."]
        #[must_use]
        #[inline(always)]
        pub const fn mpurgaddr(&self) -> u32 {
            let val = (self.0 >> 5usize) & 0x07ff_ffff;
            val as u32
        }
        #[doc = "desc MPURGADDR."]
        #[inline(always)]
        pub const fn set_mpurgaddr(&mut self, val: u32) {
            self.0 = (self.0 & !(0x07ff_ffff << 5usize)) | (((val as u32) & 0x07ff_ffff) << 5usize);
        }
    }
    impl Default for Rgd5 {
        #[inline(always)]
        fn default() -> Rgd5 {
            Rgd5(0)
        }
    }
    impl core::fmt::Debug for Rgd5 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Rgd5")
                .field("mpurgsize", &self.mpurgsize())
                .field("mpurgaddr", &self.mpurgaddr())
                .finish()
        }
    }
    #[doc = "desc RGD6."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Rgd6(pub u32);
    impl Rgd6 {
        #[doc = "desc MPURGSIZE."]
        #[must_use]
        #[inline(always)]
        pub const fn mpurgsize(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x1f;
            val as u8
        }
        #[doc = "desc MPURGSIZE."]
        #[inline(always)]
        pub const fn set_mpurgsize(&mut self, val: u8) {
            self.0 = (self.0 & !(0x1f << 0usize)) | (((val as u32) & 0x1f) << 0usize);
        }
        #[doc = "desc MPURGADDR."]
        #[must_use]
        #[inline(always)]
        pub const fn mpurgaddr(&self) -> u32 {
            let val = (self.0 >> 5usize) & 0x07ff_ffff;
            val as u32
        }
        #[doc = "desc MPURGADDR."]
        #[inline(always)]
        pub const fn set_mpurgaddr(&mut self, val: u32) {
            self.0 = (self.0 & !(0x07ff_ffff << 5usize)) | (((val as u32) & 0x07ff_ffff) << 5usize);
        }
    }
    impl Default for Rgd6 {
        #[inline(always)]
        fn default() -> Rgd6 {
            Rgd6(0)
        }
    }
    impl core::fmt::Debug for Rgd6 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Rgd6")
                .field("mpurgsize", &self.mpurgsize())
                .field("mpurgaddr", &self.mpurgaddr())
                .finish()
        }
    }
    #[doc = "desc RGD7."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Rgd7(pub u32);
    impl Rgd7 {
        #[doc = "desc MPURGSIZE."]
        #[must_use]
        #[inline(always)]
        pub const fn mpurgsize(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x1f;
            val as u8
        }
        #[doc = "desc MPURGSIZE."]
        #[inline(always)]
        pub const fn set_mpurgsize(&mut self, val: u8) {
            self.0 = (self.0 & !(0x1f << 0usize)) | (((val as u32) & 0x1f) << 0usize);
        }
        #[doc = "desc MPURGADDR."]
        #[must_use]
        #[inline(always)]
        pub const fn mpurgaddr(&self) -> u32 {
            let val = (self.0 >> 5usize) & 0x07ff_ffff;
            val as u32
        }
        #[doc = "desc MPURGADDR."]
        #[inline(always)]
        pub const fn set_mpurgaddr(&mut self, val: u32) {
            self.0 = (self.0 & !(0x07ff_ffff << 5usize)) | (((val as u32) & 0x07ff_ffff) << 5usize);
        }
    }
    impl Default for Rgd7 {
        #[inline(always)]
        fn default() -> Rgd7 {
            Rgd7(0)
        }
    }
    impl core::fmt::Debug for Rgd7 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Rgd7")
                .field("mpurgsize", &self.mpurgsize())
                .field("mpurgaddr", &self.mpurgaddr())
                .finish()
        }
    }
    #[doc = "desc RGD8."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Rgd8(pub u32);
    impl Rgd8 {
        #[doc = "desc MPURGSIZE."]
        #[must_use]
        #[inline(always)]
        pub const fn mpurgsize(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x1f;
            val as u8
        }
        #[doc = "desc MPURGSIZE."]
        #[inline(always)]
        pub const fn set_mpurgsize(&mut self, val: u8) {
            self.0 = (self.0 & !(0x1f << 0usize)) | (((val as u32) & 0x1f) << 0usize);
        }
        #[doc = "desc MPURGADDR."]
        #[must_use]
        #[inline(always)]
        pub const fn mpurgaddr(&self) -> u32 {
            let val = (self.0 >> 5usize) & 0x07ff_ffff;
            val as u32
        }
        #[doc = "desc MPURGADDR."]
        #[inline(always)]
        pub const fn set_mpurgaddr(&mut self, val: u32) {
            self.0 = (self.0 & !(0x07ff_ffff << 5usize)) | (((val as u32) & 0x07ff_ffff) << 5usize);
        }
    }
    impl Default for Rgd8 {
        #[inline(always)]
        fn default() -> Rgd8 {
            Rgd8(0)
        }
    }
    impl core::fmt::Debug for Rgd8 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Rgd8")
                .field("mpurgsize", &self.mpurgsize())
                .field("mpurgaddr", &self.mpurgaddr())
                .finish()
        }
    }
    #[doc = "desc RGD9."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Rgd9(pub u32);
    impl Rgd9 {
        #[doc = "desc MPURGSIZE."]
        #[must_use]
        #[inline(always)]
        pub const fn mpurgsize(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x1f;
            val as u8
        }
        #[doc = "desc MPURGSIZE."]
        #[inline(always)]
        pub const fn set_mpurgsize(&mut self, val: u8) {
            self.0 = (self.0 & !(0x1f << 0usize)) | (((val as u32) & 0x1f) << 0usize);
        }
        #[doc = "desc MPURGADDR."]
        #[must_use]
        #[inline(always)]
        pub const fn mpurgaddr(&self) -> u32 {
            let val = (self.0 >> 5usize) & 0x07ff_ffff;
            val as u32
        }
        #[doc = "desc MPURGADDR."]
        #[inline(always)]
        pub const fn set_mpurgaddr(&mut self, val: u32) {
            self.0 = (self.0 & !(0x07ff_ffff << 5usize)) | (((val as u32) & 0x07ff_ffff) << 5usize);
        }
    }
    impl Default for Rgd9 {
        #[inline(always)]
        fn default() -> Rgd9 {
            Rgd9(0)
        }
    }
    impl core::fmt::Debug for Rgd9 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Rgd9")
                .field("mpurgsize", &self.mpurgsize())
                .field("mpurgaddr", &self.mpurgaddr())
                .finish()
        }
    }
    #[doc = "desc S1CR."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct S1cr(pub u32);
    impl S1cr {
        #[doc = "desc SMPUBRP."]
        #[must_use]
        #[inline(always)]
        pub const fn smpubrp(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "desc SMPUBRP."]
        #[inline(always)]
        pub const fn set_smpubrp(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "desc SMPUBWP."]
        #[must_use]
        #[inline(always)]
        pub const fn smpubwp(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "desc SMPUBWP."]
        #[inline(always)]
        pub const fn set_smpubwp(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "desc SMPUACT."]
        #[must_use]
        #[inline(always)]
        pub const fn smpuact(&self) -> u8 {
            let val = (self.0 >> 2usize) & 0x03;
            val as u8
        }
        #[doc = "desc SMPUACT."]
        #[inline(always)]
        pub const fn set_smpuact(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 2usize)) | (((val as u32) & 0x03) << 2usize);
        }
        #[doc = "desc SMPUE."]
        #[must_use]
        #[inline(always)]
        pub const fn smpue(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "desc SMPUE."]
        #[inline(always)]
        pub const fn set_smpue(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
    }
    impl Default for S1cr {
        #[inline(always)]
        fn default() -> S1cr {
            S1cr(0)
        }
    }
    impl core::fmt::Debug for S1cr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("S1cr")
                .field("smpubrp", &self.smpubrp())
                .field("smpubwp", &self.smpubwp())
                .field("smpuact", &self.smpuact())
                .field("smpue", &self.smpue())
                .finish()
        }
    }
    #[doc = "desc S1RGE."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct S1rge(pub u32);
    impl S1rge {
        #[doc = "desc RG0E."]
        #[must_use]
        #[inline(always)]
        pub const fn rg0e(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "desc RG0E."]
        #[inline(always)]
        pub const fn set_rg0e(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "desc RG1E."]
        #[must_use]
        #[inline(always)]
        pub const fn rg1e(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "desc RG1E."]
        #[inline(always)]
        pub const fn set_rg1e(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "desc RG2E."]
        #[must_use]
        #[inline(always)]
        pub const fn rg2e(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "desc RG2E."]
        #[inline(always)]
        pub const fn set_rg2e(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "desc RG3E."]
        #[must_use]
        #[inline(always)]
        pub const fn rg3e(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "desc RG3E."]
        #[inline(always)]
        pub const fn set_rg3e(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "desc RG4E."]
        #[must_use]
        #[inline(always)]
        pub const fn rg4e(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "desc RG4E."]
        #[inline(always)]
        pub const fn set_rg4e(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "desc RG5E."]
        #[must_use]
        #[inline(always)]
        pub const fn rg5e(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "desc RG5E."]
        #[inline(always)]
        pub const fn set_rg5e(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "desc RG6E."]
        #[must_use]
        #[inline(always)]
        pub const fn rg6e(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "desc RG6E."]
        #[inline(always)]
        pub const fn set_rg6e(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "desc RG7E."]
        #[must_use]
        #[inline(always)]
        pub const fn rg7e(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "desc RG7E."]
        #[inline(always)]
        pub const fn set_rg7e(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "desc RG8E."]
        #[must_use]
        #[inline(always)]
        pub const fn rg8e(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "desc RG8E."]
        #[inline(always)]
        pub const fn set_rg8e(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "desc RG9E."]
        #[must_use]
        #[inline(always)]
        pub const fn rg9e(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "desc RG9E."]
        #[inline(always)]
        pub const fn set_rg9e(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "desc RG10E."]
        #[must_use]
        #[inline(always)]
        pub const fn rg10e(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "desc RG10E."]
        #[inline(always)]
        pub const fn set_rg10e(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[doc = "desc RG11E."]
        #[must_use]
        #[inline(always)]
        pub const fn rg11e(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "desc RG11E."]
        #[inline(always)]
        pub const fn set_rg11e(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[doc = "desc RG12E."]
        #[must_use]
        #[inline(always)]
        pub const fn rg12e(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "desc RG12E."]
        #[inline(always)]
        pub const fn set_rg12e(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "desc RG13E."]
        #[must_use]
        #[inline(always)]
        pub const fn rg13e(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[doc = "desc RG13E."]
        #[inline(always)]
        pub const fn set_rg13e(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[doc = "desc RG14E."]
        #[must_use]
        #[inline(always)]
        pub const fn rg14e(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "desc RG14E."]
        #[inline(always)]
        pub const fn set_rg14e(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[doc = "desc RG15E."]
        #[must_use]
        #[inline(always)]
        pub const fn rg15e(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "desc RG15E."]
        #[inline(always)]
        pub const fn set_rg15e(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
    }
    impl Default for S1rge {
        #[inline(always)]
        fn default() -> S1rge {
            S1rge(0)
        }
    }
    impl core::fmt::Debug for S1rge {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("S1rge")
                .field("rg0e", &self.rg0e())
                .field("rg1e", &self.rg1e())
                .field("rg2e", &self.rg2e())
                .field("rg3e", &self.rg3e())
                .field("rg4e", &self.rg4e())
                .field("rg5e", &self.rg5e())
                .field("rg6e", &self.rg6e())
                .field("rg7e", &self.rg7e())
                .field("rg8e", &self.rg8e())
                .field("rg9e", &self.rg9e())
                .field("rg10e", &self.rg10e())
                .field("rg11e", &self.rg11e())
                .field("rg12e", &self.rg12e())
                .field("rg13e", &self.rg13e())
                .field("rg14e", &self.rg14e())
                .field("rg15e", &self.rg15e())
                .finish()
        }
    }
    #[doc = "desc S1RGRP."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct S1rgrp(pub u32);
    impl S1rgrp {
        #[doc = "desc RG0RP."]
        #[must_use]
        #[inline(always)]
        pub const fn rg0rp(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "desc RG0RP."]
        #[inline(always)]
        pub const fn set_rg0rp(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "desc RG1RP."]
        #[must_use]
        #[inline(always)]
        pub const fn rg1rp(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "desc RG1RP."]
        #[inline(always)]
        pub const fn set_rg1rp(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "desc RG2RP."]
        #[must_use]
        #[inline(always)]
        pub const fn rg2rp(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "desc RG2RP."]
        #[inline(always)]
        pub const fn set_rg2rp(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "desc RG3RP."]
        #[must_use]
        #[inline(always)]
        pub const fn rg3rp(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "desc RG3RP."]
        #[inline(always)]
        pub const fn set_rg3rp(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "desc RG4RP."]
        #[must_use]
        #[inline(always)]
        pub const fn rg4rp(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "desc RG4RP."]
        #[inline(always)]
        pub const fn set_rg4rp(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "desc RG5RP."]
        #[must_use]
        #[inline(always)]
        pub const fn rg5rp(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "desc RG5RP."]
        #[inline(always)]
        pub const fn set_rg5rp(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "desc RG6RP."]
        #[must_use]
        #[inline(always)]
        pub const fn rg6rp(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "desc RG6RP."]
        #[inline(always)]
        pub const fn set_rg6rp(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "desc RG7RP."]
        #[must_use]
        #[inline(always)]
        pub const fn rg7rp(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "desc RG7RP."]
        #[inline(always)]
        pub const fn set_rg7rp(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "desc RG8RP."]
        #[must_use]
        #[inline(always)]
        pub const fn rg8rp(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "desc RG8RP."]
        #[inline(always)]
        pub const fn set_rg8rp(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "desc RG9RP."]
        #[must_use]
        #[inline(always)]
        pub const fn rg9rp(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "desc RG9RP."]
        #[inline(always)]
        pub const fn set_rg9rp(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "desc RG10RP."]
        #[must_use]
        #[inline(always)]
        pub const fn rg10rp(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "desc RG10RP."]
        #[inline(always)]
        pub const fn set_rg10rp(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[doc = "desc RG11RP."]
        #[must_use]
        #[inline(always)]
        pub const fn rg11rp(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "desc RG11RP."]
        #[inline(always)]
        pub const fn set_rg11rp(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[doc = "desc RG12RP."]
        #[must_use]
        #[inline(always)]
        pub const fn rg12rp(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "desc RG12RP."]
        #[inline(always)]
        pub const fn set_rg12rp(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "desc RG13RP."]
        #[must_use]
        #[inline(always)]
        pub const fn rg13rp(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[doc = "desc RG13RP."]
        #[inline(always)]
        pub const fn set_rg13rp(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[doc = "desc RG14RP."]
        #[must_use]
        #[inline(always)]
        pub const fn rg14rp(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "desc RG14RP."]
        #[inline(always)]
        pub const fn set_rg14rp(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[doc = "desc RG15RP."]
        #[must_use]
        #[inline(always)]
        pub const fn rg15rp(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "desc RG15RP."]
        #[inline(always)]
        pub const fn set_rg15rp(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
    }
    impl Default for S1rgrp {
        #[inline(always)]
        fn default() -> S1rgrp {
            S1rgrp(0)
        }
    }
    impl core::fmt::Debug for S1rgrp {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("S1rgrp")
                .field("rg0rp", &self.rg0rp())
                .field("rg1rp", &self.rg1rp())
                .field("rg2rp", &self.rg2rp())
                .field("rg3rp", &self.rg3rp())
                .field("rg4rp", &self.rg4rp())
                .field("rg5rp", &self.rg5rp())
                .field("rg6rp", &self.rg6rp())
                .field("rg7rp", &self.rg7rp())
                .field("rg8rp", &self.rg8rp())
                .field("rg9rp", &self.rg9rp())
                .field("rg10rp", &self.rg10rp())
                .field("rg11rp", &self.rg11rp())
                .field("rg12rp", &self.rg12rp())
                .field("rg13rp", &self.rg13rp())
                .field("rg14rp", &self.rg14rp())
                .field("rg15rp", &self.rg15rp())
                .finish()
        }
    }
    #[doc = "desc S1RGWP."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct S1rgwp(pub u32);
    impl S1rgwp {
        #[doc = "desc RG0WP."]
        #[must_use]
        #[inline(always)]
        pub const fn rg0wp(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "desc RG0WP."]
        #[inline(always)]
        pub const fn set_rg0wp(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "desc RG1WP."]
        #[must_use]
        #[inline(always)]
        pub const fn rg1wp(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "desc RG1WP."]
        #[inline(always)]
        pub const fn set_rg1wp(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "desc RG2WP."]
        #[must_use]
        #[inline(always)]
        pub const fn rg2wp(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "desc RG2WP."]
        #[inline(always)]
        pub const fn set_rg2wp(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "desc RG3WP."]
        #[must_use]
        #[inline(always)]
        pub const fn rg3wp(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "desc RG3WP."]
        #[inline(always)]
        pub const fn set_rg3wp(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "desc RG4WP."]
        #[must_use]
        #[inline(always)]
        pub const fn rg4wp(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "desc RG4WP."]
        #[inline(always)]
        pub const fn set_rg4wp(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "desc RG5WP."]
        #[must_use]
        #[inline(always)]
        pub const fn rg5wp(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "desc RG5WP."]
        #[inline(always)]
        pub const fn set_rg5wp(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "desc RG6WP."]
        #[must_use]
        #[inline(always)]
        pub const fn rg6wp(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "desc RG6WP."]
        #[inline(always)]
        pub const fn set_rg6wp(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "desc RG7WP."]
        #[must_use]
        #[inline(always)]
        pub const fn rg7wp(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "desc RG7WP."]
        #[inline(always)]
        pub const fn set_rg7wp(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "desc RG8WP."]
        #[must_use]
        #[inline(always)]
        pub const fn rg8wp(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "desc RG8WP."]
        #[inline(always)]
        pub const fn set_rg8wp(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "desc RG9WP."]
        #[must_use]
        #[inline(always)]
        pub const fn rg9wp(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "desc RG9WP."]
        #[inline(always)]
        pub const fn set_rg9wp(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "desc RG10WP."]
        #[must_use]
        #[inline(always)]
        pub const fn rg10wp(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "desc RG10WP."]
        #[inline(always)]
        pub const fn set_rg10wp(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[doc = "desc RG11WP."]
        #[must_use]
        #[inline(always)]
        pub const fn rg11wp(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "desc RG11WP."]
        #[inline(always)]
        pub const fn set_rg11wp(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[doc = "desc RG12WP."]
        #[must_use]
        #[inline(always)]
        pub const fn rg12wp(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "desc RG12WP."]
        #[inline(always)]
        pub const fn set_rg12wp(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "desc RG13WP."]
        #[must_use]
        #[inline(always)]
        pub const fn rg13wp(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[doc = "desc RG13WP."]
        #[inline(always)]
        pub const fn set_rg13wp(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[doc = "desc RG14WP."]
        #[must_use]
        #[inline(always)]
        pub const fn rg14wp(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "desc RG14WP."]
        #[inline(always)]
        pub const fn set_rg14wp(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[doc = "desc RG15WP."]
        #[must_use]
        #[inline(always)]
        pub const fn rg15wp(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "desc RG15WP."]
        #[inline(always)]
        pub const fn set_rg15wp(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
    }
    impl Default for S1rgwp {
        #[inline(always)]
        fn default() -> S1rgwp {
            S1rgwp(0)
        }
    }
    impl core::fmt::Debug for S1rgwp {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("S1rgwp")
                .field("rg0wp", &self.rg0wp())
                .field("rg1wp", &self.rg1wp())
                .field("rg2wp", &self.rg2wp())
                .field("rg3wp", &self.rg3wp())
                .field("rg4wp", &self.rg4wp())
                .field("rg5wp", &self.rg5wp())
                .field("rg6wp", &self.rg6wp())
                .field("rg7wp", &self.rg7wp())
                .field("rg8wp", &self.rg8wp())
                .field("rg9wp", &self.rg9wp())
                .field("rg10wp", &self.rg10wp())
                .field("rg11wp", &self.rg11wp())
                .field("rg12wp", &self.rg12wp())
                .field("rg13wp", &self.rg13wp())
                .field("rg14wp", &self.rg14wp())
                .field("rg15wp", &self.rg15wp())
                .finish()
        }
    }
    #[doc = "desc S2CR."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct S2cr(pub u32);
    impl S2cr {
        #[doc = "desc SMPUBRP."]
        #[must_use]
        #[inline(always)]
        pub const fn smpubrp(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "desc SMPUBRP."]
        #[inline(always)]
        pub const fn set_smpubrp(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "desc SMPUBWP."]
        #[must_use]
        #[inline(always)]
        pub const fn smpubwp(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "desc SMPUBWP."]
        #[inline(always)]
        pub const fn set_smpubwp(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "desc SMPUACT."]
        #[must_use]
        #[inline(always)]
        pub const fn smpuact(&self) -> u8 {
            let val = (self.0 >> 2usize) & 0x03;
            val as u8
        }
        #[doc = "desc SMPUACT."]
        #[inline(always)]
        pub const fn set_smpuact(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 2usize)) | (((val as u32) & 0x03) << 2usize);
        }
        #[doc = "desc SMPUE."]
        #[must_use]
        #[inline(always)]
        pub const fn smpue(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "desc SMPUE."]
        #[inline(always)]
        pub const fn set_smpue(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
    }
    impl Default for S2cr {
        #[inline(always)]
        fn default() -> S2cr {
            S2cr(0)
        }
    }
    impl core::fmt::Debug for S2cr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("S2cr")
                .field("smpubrp", &self.smpubrp())
                .field("smpubwp", &self.smpubwp())
                .field("smpuact", &self.smpuact())
                .field("smpue", &self.smpue())
                .finish()
        }
    }
    #[doc = "desc S2RGE."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct S2rge(pub u32);
    impl S2rge {
        #[doc = "desc RG0E."]
        #[must_use]
        #[inline(always)]
        pub const fn rg0e(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "desc RG0E."]
        #[inline(always)]
        pub const fn set_rg0e(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "desc RG1E."]
        #[must_use]
        #[inline(always)]
        pub const fn rg1e(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "desc RG1E."]
        #[inline(always)]
        pub const fn set_rg1e(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "desc RG2E."]
        #[must_use]
        #[inline(always)]
        pub const fn rg2e(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "desc RG2E."]
        #[inline(always)]
        pub const fn set_rg2e(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "desc RG3E."]
        #[must_use]
        #[inline(always)]
        pub const fn rg3e(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "desc RG3E."]
        #[inline(always)]
        pub const fn set_rg3e(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "desc RG4E."]
        #[must_use]
        #[inline(always)]
        pub const fn rg4e(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "desc RG4E."]
        #[inline(always)]
        pub const fn set_rg4e(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "desc RG5E."]
        #[must_use]
        #[inline(always)]
        pub const fn rg5e(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "desc RG5E."]
        #[inline(always)]
        pub const fn set_rg5e(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "desc RG6E."]
        #[must_use]
        #[inline(always)]
        pub const fn rg6e(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "desc RG6E."]
        #[inline(always)]
        pub const fn set_rg6e(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "desc RG7E."]
        #[must_use]
        #[inline(always)]
        pub const fn rg7e(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "desc RG7E."]
        #[inline(always)]
        pub const fn set_rg7e(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "desc RG8E."]
        #[must_use]
        #[inline(always)]
        pub const fn rg8e(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "desc RG8E."]
        #[inline(always)]
        pub const fn set_rg8e(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "desc RG9E."]
        #[must_use]
        #[inline(always)]
        pub const fn rg9e(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "desc RG9E."]
        #[inline(always)]
        pub const fn set_rg9e(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "desc RG10E."]
        #[must_use]
        #[inline(always)]
        pub const fn rg10e(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "desc RG10E."]
        #[inline(always)]
        pub const fn set_rg10e(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[doc = "desc RG11E."]
        #[must_use]
        #[inline(always)]
        pub const fn rg11e(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "desc RG11E."]
        #[inline(always)]
        pub const fn set_rg11e(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[doc = "desc RG12E."]
        #[must_use]
        #[inline(always)]
        pub const fn rg12e(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "desc RG12E."]
        #[inline(always)]
        pub const fn set_rg12e(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "desc RG13E."]
        #[must_use]
        #[inline(always)]
        pub const fn rg13e(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[doc = "desc RG13E."]
        #[inline(always)]
        pub const fn set_rg13e(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[doc = "desc RG14E."]
        #[must_use]
        #[inline(always)]
        pub const fn rg14e(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "desc RG14E."]
        #[inline(always)]
        pub const fn set_rg14e(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[doc = "desc RG15E."]
        #[must_use]
        #[inline(always)]
        pub const fn rg15e(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "desc RG15E."]
        #[inline(always)]
        pub const fn set_rg15e(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
    }
    impl Default for S2rge {
        #[inline(always)]
        fn default() -> S2rge {
            S2rge(0)
        }
    }
    impl core::fmt::Debug for S2rge {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("S2rge")
                .field("rg0e", &self.rg0e())
                .field("rg1e", &self.rg1e())
                .field("rg2e", &self.rg2e())
                .field("rg3e", &self.rg3e())
                .field("rg4e", &self.rg4e())
                .field("rg5e", &self.rg5e())
                .field("rg6e", &self.rg6e())
                .field("rg7e", &self.rg7e())
                .field("rg8e", &self.rg8e())
                .field("rg9e", &self.rg9e())
                .field("rg10e", &self.rg10e())
                .field("rg11e", &self.rg11e())
                .field("rg12e", &self.rg12e())
                .field("rg13e", &self.rg13e())
                .field("rg14e", &self.rg14e())
                .field("rg15e", &self.rg15e())
                .finish()
        }
    }
    #[doc = "desc S2RGRP."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct S2rgrp(pub u32);
    impl S2rgrp {
        #[doc = "desc RG0RP."]
        #[must_use]
        #[inline(always)]
        pub const fn rg0rp(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "desc RG0RP."]
        #[inline(always)]
        pub const fn set_rg0rp(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "desc RG1RP."]
        #[must_use]
        #[inline(always)]
        pub const fn rg1rp(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "desc RG1RP."]
        #[inline(always)]
        pub const fn set_rg1rp(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "desc RG2RP."]
        #[must_use]
        #[inline(always)]
        pub const fn rg2rp(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "desc RG2RP."]
        #[inline(always)]
        pub const fn set_rg2rp(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "desc RG3RP."]
        #[must_use]
        #[inline(always)]
        pub const fn rg3rp(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "desc RG3RP."]
        #[inline(always)]
        pub const fn set_rg3rp(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "desc RG4RP."]
        #[must_use]
        #[inline(always)]
        pub const fn rg4rp(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "desc RG4RP."]
        #[inline(always)]
        pub const fn set_rg4rp(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "desc RG5RP."]
        #[must_use]
        #[inline(always)]
        pub const fn rg5rp(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "desc RG5RP."]
        #[inline(always)]
        pub const fn set_rg5rp(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "desc RG6RP."]
        #[must_use]
        #[inline(always)]
        pub const fn rg6rp(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "desc RG6RP."]
        #[inline(always)]
        pub const fn set_rg6rp(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "desc RG7RP."]
        #[must_use]
        #[inline(always)]
        pub const fn rg7rp(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "desc RG7RP."]
        #[inline(always)]
        pub const fn set_rg7rp(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "desc RG8RP."]
        #[must_use]
        #[inline(always)]
        pub const fn rg8rp(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "desc RG8RP."]
        #[inline(always)]
        pub const fn set_rg8rp(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "desc RG9RP."]
        #[must_use]
        #[inline(always)]
        pub const fn rg9rp(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "desc RG9RP."]
        #[inline(always)]
        pub const fn set_rg9rp(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "desc RG10RP."]
        #[must_use]
        #[inline(always)]
        pub const fn rg10rp(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "desc RG10RP."]
        #[inline(always)]
        pub const fn set_rg10rp(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[doc = "desc RG11RP."]
        #[must_use]
        #[inline(always)]
        pub const fn rg11rp(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "desc RG11RP."]
        #[inline(always)]
        pub const fn set_rg11rp(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[doc = "desc RG12RP."]
        #[must_use]
        #[inline(always)]
        pub const fn rg12rp(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "desc RG12RP."]
        #[inline(always)]
        pub const fn set_rg12rp(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "desc RG13RP."]
        #[must_use]
        #[inline(always)]
        pub const fn rg13rp(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[doc = "desc RG13RP."]
        #[inline(always)]
        pub const fn set_rg13rp(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[doc = "desc RG14RP."]
        #[must_use]
        #[inline(always)]
        pub const fn rg14rp(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "desc RG14RP."]
        #[inline(always)]
        pub const fn set_rg14rp(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[doc = "desc RG15RP."]
        #[must_use]
        #[inline(always)]
        pub const fn rg15rp(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "desc RG15RP."]
        #[inline(always)]
        pub const fn set_rg15rp(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
    }
    impl Default for S2rgrp {
        #[inline(always)]
        fn default() -> S2rgrp {
            S2rgrp(0)
        }
    }
    impl core::fmt::Debug for S2rgrp {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("S2rgrp")
                .field("rg0rp", &self.rg0rp())
                .field("rg1rp", &self.rg1rp())
                .field("rg2rp", &self.rg2rp())
                .field("rg3rp", &self.rg3rp())
                .field("rg4rp", &self.rg4rp())
                .field("rg5rp", &self.rg5rp())
                .field("rg6rp", &self.rg6rp())
                .field("rg7rp", &self.rg7rp())
                .field("rg8rp", &self.rg8rp())
                .field("rg9rp", &self.rg9rp())
                .field("rg10rp", &self.rg10rp())
                .field("rg11rp", &self.rg11rp())
                .field("rg12rp", &self.rg12rp())
                .field("rg13rp", &self.rg13rp())
                .field("rg14rp", &self.rg14rp())
                .field("rg15rp", &self.rg15rp())
                .finish()
        }
    }
    #[doc = "desc S2RGWP."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct S2rgwp(pub u32);
    impl S2rgwp {
        #[doc = "desc RG0WP."]
        #[must_use]
        #[inline(always)]
        pub const fn rg0wp(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "desc RG0WP."]
        #[inline(always)]
        pub const fn set_rg0wp(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "desc RG1WP."]
        #[must_use]
        #[inline(always)]
        pub const fn rg1wp(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "desc RG1WP."]
        #[inline(always)]
        pub const fn set_rg1wp(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "desc RG2WP."]
        #[must_use]
        #[inline(always)]
        pub const fn rg2wp(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "desc RG2WP."]
        #[inline(always)]
        pub const fn set_rg2wp(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "desc RG3WP."]
        #[must_use]
        #[inline(always)]
        pub const fn rg3wp(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "desc RG3WP."]
        #[inline(always)]
        pub const fn set_rg3wp(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "desc RG4WP."]
        #[must_use]
        #[inline(always)]
        pub const fn rg4wp(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "desc RG4WP."]
        #[inline(always)]
        pub const fn set_rg4wp(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "desc RG5WP."]
        #[must_use]
        #[inline(always)]
        pub const fn rg5wp(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "desc RG5WP."]
        #[inline(always)]
        pub const fn set_rg5wp(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "desc RG6WP."]
        #[must_use]
        #[inline(always)]
        pub const fn rg6wp(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "desc RG6WP."]
        #[inline(always)]
        pub const fn set_rg6wp(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "desc RG7WP."]
        #[must_use]
        #[inline(always)]
        pub const fn rg7wp(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "desc RG7WP."]
        #[inline(always)]
        pub const fn set_rg7wp(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "desc RG8WP."]
        #[must_use]
        #[inline(always)]
        pub const fn rg8wp(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "desc RG8WP."]
        #[inline(always)]
        pub const fn set_rg8wp(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "desc RG9WP."]
        #[must_use]
        #[inline(always)]
        pub const fn rg9wp(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "desc RG9WP."]
        #[inline(always)]
        pub const fn set_rg9wp(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "desc RG10WP."]
        #[must_use]
        #[inline(always)]
        pub const fn rg10wp(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "desc RG10WP."]
        #[inline(always)]
        pub const fn set_rg10wp(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[doc = "desc RG11WP."]
        #[must_use]
        #[inline(always)]
        pub const fn rg11wp(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "desc RG11WP."]
        #[inline(always)]
        pub const fn set_rg11wp(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[doc = "desc RG12WP."]
        #[must_use]
        #[inline(always)]
        pub const fn rg12wp(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "desc RG12WP."]
        #[inline(always)]
        pub const fn set_rg12wp(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "desc RG13WP."]
        #[must_use]
        #[inline(always)]
        pub const fn rg13wp(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[doc = "desc RG13WP."]
        #[inline(always)]
        pub const fn set_rg13wp(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[doc = "desc RG14WP."]
        #[must_use]
        #[inline(always)]
        pub const fn rg14wp(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "desc RG14WP."]
        #[inline(always)]
        pub const fn set_rg14wp(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[doc = "desc RG15WP."]
        #[must_use]
        #[inline(always)]
        pub const fn rg15wp(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "desc RG15WP."]
        #[inline(always)]
        pub const fn set_rg15wp(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
    }
    impl Default for S2rgwp {
        #[inline(always)]
        fn default() -> S2rgwp {
            S2rgwp(0)
        }
    }
    impl core::fmt::Debug for S2rgwp {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("S2rgwp")
                .field("rg0wp", &self.rg0wp())
                .field("rg1wp", &self.rg1wp())
                .field("rg2wp", &self.rg2wp())
                .field("rg3wp", &self.rg3wp())
                .field("rg4wp", &self.rg4wp())
                .field("rg5wp", &self.rg5wp())
                .field("rg6wp", &self.rg6wp())
                .field("rg7wp", &self.rg7wp())
                .field("rg8wp", &self.rg8wp())
                .field("rg9wp", &self.rg9wp())
                .field("rg10wp", &self.rg10wp())
                .field("rg11wp", &self.rg11wp())
                .field("rg12wp", &self.rg12wp())
                .field("rg13wp", &self.rg13wp())
                .field("rg14wp", &self.rg14wp())
                .field("rg15wp", &self.rg15wp())
                .finish()
        }
    }
    #[doc = "desc SR."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Sr(pub u32);
    impl Sr {
        #[doc = "desc SMPU1EAF."]
        #[must_use]
        #[inline(always)]
        pub const fn smpu1eaf(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "desc SMPU1EAF."]
        #[inline(always)]
        pub const fn set_smpu1eaf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "desc SMPU2EAF."]
        #[must_use]
        #[inline(always)]
        pub const fn smpu2eaf(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "desc SMPU2EAF."]
        #[inline(always)]
        pub const fn set_smpu2eaf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "desc FMPUEAF."]
        #[must_use]
        #[inline(always)]
        pub const fn fmpueaf(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "desc FMPUEAF."]
        #[inline(always)]
        pub const fn set_fmpueaf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "desc PSPEF."]
        #[must_use]
        #[inline(always)]
        pub const fn pspef(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "desc PSPEF."]
        #[inline(always)]
        pub const fn set_pspef(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "desc MSPEF."]
        #[must_use]
        #[inline(always)]
        pub const fn mspef(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "desc MSPEF."]
        #[inline(always)]
        pub const fn set_mspef(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
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
                .field("smpu1eaf", &self.smpu1eaf())
                .field("smpu2eaf", &self.smpu2eaf())
                .field("fmpueaf", &self.fmpueaf())
                .field("pspef", &self.pspef())
                .field("mspef", &self.mspef())
                .finish()
        }
    }
    #[doc = "desc WP."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Wp(pub u32);
    impl Wp {
        #[doc = "desc MPUWE."]
        #[must_use]
        #[inline(always)]
        pub const fn mpuwe(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "desc MPUWE."]
        #[inline(always)]
        pub const fn set_mpuwe(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "desc WKEY."]
        #[must_use]
        #[inline(always)]
        pub const fn wkey(&self) -> u16 {
            let val = (self.0 >> 1usize) & 0x7fff;
            val as u16
        }
        #[doc = "desc WKEY."]
        #[inline(always)]
        pub const fn set_wkey(&mut self, val: u16) {
            self.0 = (self.0 & !(0x7fff << 1usize)) | (((val as u32) & 0x7fff) << 1usize);
        }
    }
    impl Default for Wp {
        #[inline(always)]
        fn default() -> Wp {
            Wp(0)
        }
    }
    impl core::fmt::Debug for Wp {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Wp")
                .field("mpuwe", &self.mpuwe())
                .field("wkey", &self.wkey())
                .finish()
        }
    }
}
