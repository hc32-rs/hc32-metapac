#![allow(clippy::missing_safety_doc)]
#![allow(clippy::identity_op)]
#![allow(clippy::unnecessary_cast)]
#![allow(clippy::erasing_op)]

#[doc = "desc EFM."]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Efm {
    ptr: *mut u8,
}
unsafe impl Send for Efm {}
unsafe impl Sync for Efm {}
impl Efm {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "desc FAPRT."]
    #[inline(always)]
    pub const fn faprt(self) -> crate::common::Reg<regs::Faprt, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0usize) as _) }
    }
    #[doc = "desc KEY1."]
    #[inline(always)]
    pub const fn key1(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x04usize) as _) }
    }
    #[doc = "desc KEY2."]
    #[inline(always)]
    pub const fn key2(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x08usize) as _) }
    }
    #[doc = "desc FSTP."]
    #[inline(always)]
    pub const fn fstp(self) -> crate::common::Reg<regs::Fstp, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x14usize) as _) }
    }
    #[doc = "desc FRMC."]
    #[inline(always)]
    pub const fn frmc(self) -> crate::common::Reg<regs::Frmc, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x18usize) as _) }
    }
    #[doc = "desc FWMC."]
    #[inline(always)]
    pub const fn fwmc(self) -> crate::common::Reg<regs::Fwmc, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1cusize) as _) }
    }
    #[doc = "desc FSR."]
    #[inline(always)]
    pub const fn fsr(self) -> crate::common::Reg<regs::Fsr, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x20usize) as _) }
    }
    #[doc = "desc FSCLR."]
    #[inline(always)]
    pub const fn fsclr(self) -> crate::common::Reg<regs::Fsclr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x24usize) as _) }
    }
    #[doc = "desc FITE."]
    #[inline(always)]
    pub const fn fite(self) -> crate::common::Reg<regs::Fite, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x28usize) as _) }
    }
    #[doc = "desc FSWP."]
    #[inline(always)]
    pub const fn fswp(self) -> crate::common::Reg<regs::Fswp, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x2cusize) as _) }
    }
    #[doc = "desc CHIPID."]
    #[inline(always)]
    pub const fn chipid(self) -> crate::common::Reg<u32, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x40usize) as _) }
    }
    #[doc = "desc UQID0."]
    #[inline(always)]
    pub const fn uqid0(self) -> crate::common::Reg<u32, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x50usize) as _) }
    }
    #[doc = "desc UQID1."]
    #[inline(always)]
    pub const fn uqid1(self) -> crate::common::Reg<u32, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x54usize) as _) }
    }
    #[doc = "desc UQID2."]
    #[inline(always)]
    pub const fn uqid2(self) -> crate::common::Reg<u32, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x58usize) as _) }
    }
    #[doc = "desc MMF_REMPRT."]
    #[inline(always)]
    pub const fn mmf_remprt(self) -> crate::common::Reg<regs::MmfRemprt, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0100usize) as _) }
    }
    #[doc = "desc MMF_REMCR0."]
    #[inline(always)]
    pub const fn mmf_remcr0(self) -> crate::common::Reg<regs::MmfRemcr0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0104usize) as _) }
    }
    #[doc = "desc MMF_REMCR1."]
    #[inline(always)]
    pub const fn mmf_remcr1(self) -> crate::common::Reg<regs::MmfRemcr1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0108usize) as _) }
    }
    #[doc = "desc WLOCK."]
    #[inline(always)]
    pub const fn wlock(self) -> crate::common::Reg<regs::Wlock, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0180usize) as _) }
    }
    #[doc = "desc F0NWPRT."]
    #[inline(always)]
    pub const fn f0nwprt(self) -> crate::common::Reg<regs::F0nwprt, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0190usize) as _) }
    }
    #[doc = "desc F1NWPRT."]
    #[inline(always)]
    pub const fn f1nwprt(self) -> crate::common::Reg<regs::F1nwprt, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x01a0usize) as _) }
    }
}
pub mod regs {
    #[doc = "desc F0NWPRT."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct F0nwprt(pub u32);
    impl F0nwprt {
        #[doc = "desc F0NWPRT0."]
        #[must_use]
        #[inline(always)]
        pub const fn f0nwprt0(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "desc F0NWPRT0."]
        #[inline(always)]
        pub const fn set_f0nwprt0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "desc F0NWPRT1."]
        #[must_use]
        #[inline(always)]
        pub const fn f0nwprt1(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "desc F0NWPRT1."]
        #[inline(always)]
        pub const fn set_f0nwprt1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "desc F0NWPRT2."]
        #[must_use]
        #[inline(always)]
        pub const fn f0nwprt2(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "desc F0NWPRT2."]
        #[inline(always)]
        pub const fn set_f0nwprt2(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "desc F0NWPRT3."]
        #[must_use]
        #[inline(always)]
        pub const fn f0nwprt3(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "desc F0NWPRT3."]
        #[inline(always)]
        pub const fn set_f0nwprt3(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "desc F0NWPRT4."]
        #[must_use]
        #[inline(always)]
        pub const fn f0nwprt4(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "desc F0NWPRT4."]
        #[inline(always)]
        pub const fn set_f0nwprt4(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "desc F0NWPRT5."]
        #[must_use]
        #[inline(always)]
        pub const fn f0nwprt5(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "desc F0NWPRT5."]
        #[inline(always)]
        pub const fn set_f0nwprt5(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "desc F0NWPRT6."]
        #[must_use]
        #[inline(always)]
        pub const fn f0nwprt6(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "desc F0NWPRT6."]
        #[inline(always)]
        pub const fn set_f0nwprt6(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "desc F0NWPRT7."]
        #[must_use]
        #[inline(always)]
        pub const fn f0nwprt7(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "desc F0NWPRT7."]
        #[inline(always)]
        pub const fn set_f0nwprt7(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "desc F0NWPRT8."]
        #[must_use]
        #[inline(always)]
        pub const fn f0nwprt8(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "desc F0NWPRT8."]
        #[inline(always)]
        pub const fn set_f0nwprt8(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "desc F0NWPRT9."]
        #[must_use]
        #[inline(always)]
        pub const fn f0nwprt9(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "desc F0NWPRT9."]
        #[inline(always)]
        pub const fn set_f0nwprt9(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "desc F0NWPRT10."]
        #[must_use]
        #[inline(always)]
        pub const fn f0nwprt10(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "desc F0NWPRT10."]
        #[inline(always)]
        pub const fn set_f0nwprt10(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[doc = "desc F0NWPRT11."]
        #[must_use]
        #[inline(always)]
        pub const fn f0nwprt11(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "desc F0NWPRT11."]
        #[inline(always)]
        pub const fn set_f0nwprt11(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[doc = "desc F0NWPRT12."]
        #[must_use]
        #[inline(always)]
        pub const fn f0nwprt12(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "desc F0NWPRT12."]
        #[inline(always)]
        pub const fn set_f0nwprt12(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "desc F0NWPRT13."]
        #[must_use]
        #[inline(always)]
        pub const fn f0nwprt13(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[doc = "desc F0NWPRT13."]
        #[inline(always)]
        pub const fn set_f0nwprt13(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[doc = "desc F0NWPRT14."]
        #[must_use]
        #[inline(always)]
        pub const fn f0nwprt14(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "desc F0NWPRT14."]
        #[inline(always)]
        pub const fn set_f0nwprt14(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[doc = "desc F0NWPRT15."]
        #[must_use]
        #[inline(always)]
        pub const fn f0nwprt15(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "desc F0NWPRT15."]
        #[inline(always)]
        pub const fn set_f0nwprt15(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
        #[doc = "desc F0NWPRT16."]
        #[must_use]
        #[inline(always)]
        pub const fn f0nwprt16(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "desc F0NWPRT16."]
        #[inline(always)]
        pub const fn set_f0nwprt16(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "desc F0NWPRT17."]
        #[must_use]
        #[inline(always)]
        pub const fn f0nwprt17(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "desc F0NWPRT17."]
        #[inline(always)]
        pub const fn set_f0nwprt17(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[doc = "desc F0NWPRT18."]
        #[must_use]
        #[inline(always)]
        pub const fn f0nwprt18(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[doc = "desc F0NWPRT18."]
        #[inline(always)]
        pub const fn set_f0nwprt18(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
        #[doc = "desc F0NWPRT19."]
        #[must_use]
        #[inline(always)]
        pub const fn f0nwprt19(&self) -> bool {
            let val = (self.0 >> 19usize) & 0x01;
            val != 0
        }
        #[doc = "desc F0NWPRT19."]
        #[inline(always)]
        pub const fn set_f0nwprt19(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
        }
        #[doc = "desc F0NWPRT20."]
        #[must_use]
        #[inline(always)]
        pub const fn f0nwprt20(&self) -> bool {
            let val = (self.0 >> 20usize) & 0x01;
            val != 0
        }
        #[doc = "desc F0NWPRT20."]
        #[inline(always)]
        pub const fn set_f0nwprt20(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
        }
        #[doc = "desc F0NWPRT21."]
        #[must_use]
        #[inline(always)]
        pub const fn f0nwprt21(&self) -> bool {
            let val = (self.0 >> 21usize) & 0x01;
            val != 0
        }
        #[doc = "desc F0NWPRT21."]
        #[inline(always)]
        pub const fn set_f0nwprt21(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
        }
        #[doc = "desc F0NWPRT22."]
        #[must_use]
        #[inline(always)]
        pub const fn f0nwprt22(&self) -> bool {
            let val = (self.0 >> 22usize) & 0x01;
            val != 0
        }
        #[doc = "desc F0NWPRT22."]
        #[inline(always)]
        pub const fn set_f0nwprt22(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
        }
        #[doc = "desc F0NWPRT23."]
        #[must_use]
        #[inline(always)]
        pub const fn f0nwprt23(&self) -> bool {
            let val = (self.0 >> 23usize) & 0x01;
            val != 0
        }
        #[doc = "desc F0NWPRT23."]
        #[inline(always)]
        pub const fn set_f0nwprt23(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
        }
        #[doc = "desc F0NWPRT24."]
        #[must_use]
        #[inline(always)]
        pub const fn f0nwprt24(&self) -> bool {
            let val = (self.0 >> 24usize) & 0x01;
            val != 0
        }
        #[doc = "desc F0NWPRT24."]
        #[inline(always)]
        pub const fn set_f0nwprt24(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
        }
        #[doc = "desc F0NWPRT25."]
        #[must_use]
        #[inline(always)]
        pub const fn f0nwprt25(&self) -> bool {
            let val = (self.0 >> 25usize) & 0x01;
            val != 0
        }
        #[doc = "desc F0NWPRT25."]
        #[inline(always)]
        pub const fn set_f0nwprt25(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
        }
        #[doc = "desc F0NWPRT26."]
        #[must_use]
        #[inline(always)]
        pub const fn f0nwprt26(&self) -> bool {
            let val = (self.0 >> 26usize) & 0x01;
            val != 0
        }
        #[doc = "desc F0NWPRT26."]
        #[inline(always)]
        pub const fn set_f0nwprt26(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
        }
        #[doc = "desc F0NWPRT27."]
        #[must_use]
        #[inline(always)]
        pub const fn f0nwprt27(&self) -> bool {
            let val = (self.0 >> 27usize) & 0x01;
            val != 0
        }
        #[doc = "desc F0NWPRT27."]
        #[inline(always)]
        pub const fn set_f0nwprt27(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
        }
        #[doc = "desc F0NWPRT28."]
        #[must_use]
        #[inline(always)]
        pub const fn f0nwprt28(&self) -> bool {
            let val = (self.0 >> 28usize) & 0x01;
            val != 0
        }
        #[doc = "desc F0NWPRT28."]
        #[inline(always)]
        pub const fn set_f0nwprt28(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
        }
        #[doc = "desc F0NWPRT29."]
        #[must_use]
        #[inline(always)]
        pub const fn f0nwprt29(&self) -> bool {
            let val = (self.0 >> 29usize) & 0x01;
            val != 0
        }
        #[doc = "desc F0NWPRT29."]
        #[inline(always)]
        pub const fn set_f0nwprt29(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
        }
        #[doc = "desc F0NWPRT30."]
        #[must_use]
        #[inline(always)]
        pub const fn f0nwprt30(&self) -> bool {
            let val = (self.0 >> 30usize) & 0x01;
            val != 0
        }
        #[doc = "desc F0NWPRT30."]
        #[inline(always)]
        pub const fn set_f0nwprt30(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
        }
        #[doc = "desc F0NWPRT31."]
        #[must_use]
        #[inline(always)]
        pub const fn f0nwprt31(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[doc = "desc F0NWPRT31."]
        #[inline(always)]
        pub const fn set_f0nwprt31(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for F0nwprt {
        #[inline(always)]
        fn default() -> F0nwprt {
            F0nwprt(0)
        }
    }
    impl core::fmt::Debug for F0nwprt {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("F0nwprt")
                .field("f0nwprt0", &self.f0nwprt0())
                .field("f0nwprt1", &self.f0nwprt1())
                .field("f0nwprt2", &self.f0nwprt2())
                .field("f0nwprt3", &self.f0nwprt3())
                .field("f0nwprt4", &self.f0nwprt4())
                .field("f0nwprt5", &self.f0nwprt5())
                .field("f0nwprt6", &self.f0nwprt6())
                .field("f0nwprt7", &self.f0nwprt7())
                .field("f0nwprt8", &self.f0nwprt8())
                .field("f0nwprt9", &self.f0nwprt9())
                .field("f0nwprt10", &self.f0nwprt10())
                .field("f0nwprt11", &self.f0nwprt11())
                .field("f0nwprt12", &self.f0nwprt12())
                .field("f0nwprt13", &self.f0nwprt13())
                .field("f0nwprt14", &self.f0nwprt14())
                .field("f0nwprt15", &self.f0nwprt15())
                .field("f0nwprt16", &self.f0nwprt16())
                .field("f0nwprt17", &self.f0nwprt17())
                .field("f0nwprt18", &self.f0nwprt18())
                .field("f0nwprt19", &self.f0nwprt19())
                .field("f0nwprt20", &self.f0nwprt20())
                .field("f0nwprt21", &self.f0nwprt21())
                .field("f0nwprt22", &self.f0nwprt22())
                .field("f0nwprt23", &self.f0nwprt23())
                .field("f0nwprt24", &self.f0nwprt24())
                .field("f0nwprt25", &self.f0nwprt25())
                .field("f0nwprt26", &self.f0nwprt26())
                .field("f0nwprt27", &self.f0nwprt27())
                .field("f0nwprt28", &self.f0nwprt28())
                .field("f0nwprt29", &self.f0nwprt29())
                .field("f0nwprt30", &self.f0nwprt30())
                .field("f0nwprt31", &self.f0nwprt31())
                .finish()
        }
    }
    #[doc = "desc F1NWPRT."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct F1nwprt(pub u32);
    impl F1nwprt {
        #[doc = "desc F1NWPRT0."]
        #[must_use]
        #[inline(always)]
        pub const fn f1nwprt0(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "desc F1NWPRT0."]
        #[inline(always)]
        pub const fn set_f1nwprt0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "desc F1NWPRT1."]
        #[must_use]
        #[inline(always)]
        pub const fn f1nwprt1(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "desc F1NWPRT1."]
        #[inline(always)]
        pub const fn set_f1nwprt1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "desc F1NWPRT2."]
        #[must_use]
        #[inline(always)]
        pub const fn f1nwprt2(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "desc F1NWPRT2."]
        #[inline(always)]
        pub const fn set_f1nwprt2(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "desc F1NWPRT3."]
        #[must_use]
        #[inline(always)]
        pub const fn f1nwprt3(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "desc F1NWPRT3."]
        #[inline(always)]
        pub const fn set_f1nwprt3(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "desc F1NWPRT4."]
        #[must_use]
        #[inline(always)]
        pub const fn f1nwprt4(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "desc F1NWPRT4."]
        #[inline(always)]
        pub const fn set_f1nwprt4(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "desc F1NWPRT5."]
        #[must_use]
        #[inline(always)]
        pub const fn f1nwprt5(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "desc F1NWPRT5."]
        #[inline(always)]
        pub const fn set_f1nwprt5(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "desc F1NWPRT6."]
        #[must_use]
        #[inline(always)]
        pub const fn f1nwprt6(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "desc F1NWPRT6."]
        #[inline(always)]
        pub const fn set_f1nwprt6(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "desc F1NWPRT7."]
        #[must_use]
        #[inline(always)]
        pub const fn f1nwprt7(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "desc F1NWPRT7."]
        #[inline(always)]
        pub const fn set_f1nwprt7(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "desc F1NWPRT8."]
        #[must_use]
        #[inline(always)]
        pub const fn f1nwprt8(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "desc F1NWPRT8."]
        #[inline(always)]
        pub const fn set_f1nwprt8(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "desc F1NWPRT9."]
        #[must_use]
        #[inline(always)]
        pub const fn f1nwprt9(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "desc F1NWPRT9."]
        #[inline(always)]
        pub const fn set_f1nwprt9(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "desc F1NWPRT10."]
        #[must_use]
        #[inline(always)]
        pub const fn f1nwprt10(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "desc F1NWPRT10."]
        #[inline(always)]
        pub const fn set_f1nwprt10(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[doc = "desc F1NWPRT11."]
        #[must_use]
        #[inline(always)]
        pub const fn f1nwprt11(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "desc F1NWPRT11."]
        #[inline(always)]
        pub const fn set_f1nwprt11(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[doc = "desc F1NWPRT12."]
        #[must_use]
        #[inline(always)]
        pub const fn f1nwprt12(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "desc F1NWPRT12."]
        #[inline(always)]
        pub const fn set_f1nwprt12(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "desc F1NWPRT13."]
        #[must_use]
        #[inline(always)]
        pub const fn f1nwprt13(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[doc = "desc F1NWPRT13."]
        #[inline(always)]
        pub const fn set_f1nwprt13(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[doc = "desc F1NWPRT14."]
        #[must_use]
        #[inline(always)]
        pub const fn f1nwprt14(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "desc F1NWPRT14."]
        #[inline(always)]
        pub const fn set_f1nwprt14(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[doc = "desc F1NWPRT15."]
        #[must_use]
        #[inline(always)]
        pub const fn f1nwprt15(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "desc F1NWPRT15."]
        #[inline(always)]
        pub const fn set_f1nwprt15(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
        #[doc = "desc F1NWPRT16."]
        #[must_use]
        #[inline(always)]
        pub const fn f1nwprt16(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "desc F1NWPRT16."]
        #[inline(always)]
        pub const fn set_f1nwprt16(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "desc F1NWPRT17."]
        #[must_use]
        #[inline(always)]
        pub const fn f1nwprt17(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "desc F1NWPRT17."]
        #[inline(always)]
        pub const fn set_f1nwprt17(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[doc = "desc F1NWPRT18."]
        #[must_use]
        #[inline(always)]
        pub const fn f1nwprt18(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[doc = "desc F1NWPRT18."]
        #[inline(always)]
        pub const fn set_f1nwprt18(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
        #[doc = "desc F1NWPRT19."]
        #[must_use]
        #[inline(always)]
        pub const fn f1nwprt19(&self) -> bool {
            let val = (self.0 >> 19usize) & 0x01;
            val != 0
        }
        #[doc = "desc F1NWPRT19."]
        #[inline(always)]
        pub const fn set_f1nwprt19(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
        }
        #[doc = "desc F1NWPRT20."]
        #[must_use]
        #[inline(always)]
        pub const fn f1nwprt20(&self) -> bool {
            let val = (self.0 >> 20usize) & 0x01;
            val != 0
        }
        #[doc = "desc F1NWPRT20."]
        #[inline(always)]
        pub const fn set_f1nwprt20(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
        }
        #[doc = "desc F1NWPRT21."]
        #[must_use]
        #[inline(always)]
        pub const fn f1nwprt21(&self) -> bool {
            let val = (self.0 >> 21usize) & 0x01;
            val != 0
        }
        #[doc = "desc F1NWPRT21."]
        #[inline(always)]
        pub const fn set_f1nwprt21(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
        }
        #[doc = "desc F1NWPRT22."]
        #[must_use]
        #[inline(always)]
        pub const fn f1nwprt22(&self) -> bool {
            let val = (self.0 >> 22usize) & 0x01;
            val != 0
        }
        #[doc = "desc F1NWPRT22."]
        #[inline(always)]
        pub const fn set_f1nwprt22(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
        }
        #[doc = "desc F1NWPRT23."]
        #[must_use]
        #[inline(always)]
        pub const fn f1nwprt23(&self) -> bool {
            let val = (self.0 >> 23usize) & 0x01;
            val != 0
        }
        #[doc = "desc F1NWPRT23."]
        #[inline(always)]
        pub const fn set_f1nwprt23(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
        }
        #[doc = "desc F1NWPRT24."]
        #[must_use]
        #[inline(always)]
        pub const fn f1nwprt24(&self) -> bool {
            let val = (self.0 >> 24usize) & 0x01;
            val != 0
        }
        #[doc = "desc F1NWPRT24."]
        #[inline(always)]
        pub const fn set_f1nwprt24(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
        }
        #[doc = "desc F1NWPRT25."]
        #[must_use]
        #[inline(always)]
        pub const fn f1nwprt25(&self) -> bool {
            let val = (self.0 >> 25usize) & 0x01;
            val != 0
        }
        #[doc = "desc F1NWPRT25."]
        #[inline(always)]
        pub const fn set_f1nwprt25(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
        }
        #[doc = "desc F1NWPRT26."]
        #[must_use]
        #[inline(always)]
        pub const fn f1nwprt26(&self) -> bool {
            let val = (self.0 >> 26usize) & 0x01;
            val != 0
        }
        #[doc = "desc F1NWPRT26."]
        #[inline(always)]
        pub const fn set_f1nwprt26(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
        }
        #[doc = "desc F1NWPRT27."]
        #[must_use]
        #[inline(always)]
        pub const fn f1nwprt27(&self) -> bool {
            let val = (self.0 >> 27usize) & 0x01;
            val != 0
        }
        #[doc = "desc F1NWPRT27."]
        #[inline(always)]
        pub const fn set_f1nwprt27(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
        }
        #[doc = "desc F1NWPRT28."]
        #[must_use]
        #[inline(always)]
        pub const fn f1nwprt28(&self) -> bool {
            let val = (self.0 >> 28usize) & 0x01;
            val != 0
        }
        #[doc = "desc F1NWPRT28."]
        #[inline(always)]
        pub const fn set_f1nwprt28(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
        }
        #[doc = "desc F1NWPRT29."]
        #[must_use]
        #[inline(always)]
        pub const fn f1nwprt29(&self) -> bool {
            let val = (self.0 >> 29usize) & 0x01;
            val != 0
        }
        #[doc = "desc F1NWPRT29."]
        #[inline(always)]
        pub const fn set_f1nwprt29(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
        }
        #[doc = "desc F1NWPRT30."]
        #[must_use]
        #[inline(always)]
        pub const fn f1nwprt30(&self) -> bool {
            let val = (self.0 >> 30usize) & 0x01;
            val != 0
        }
        #[doc = "desc F1NWPRT30."]
        #[inline(always)]
        pub const fn set_f1nwprt30(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
        }
        #[doc = "desc F1NWPRT31."]
        #[must_use]
        #[inline(always)]
        pub const fn f1nwprt31(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[doc = "desc F1NWPRT31."]
        #[inline(always)]
        pub const fn set_f1nwprt31(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for F1nwprt {
        #[inline(always)]
        fn default() -> F1nwprt {
            F1nwprt(0)
        }
    }
    impl core::fmt::Debug for F1nwprt {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("F1nwprt")
                .field("f1nwprt0", &self.f1nwprt0())
                .field("f1nwprt1", &self.f1nwprt1())
                .field("f1nwprt2", &self.f1nwprt2())
                .field("f1nwprt3", &self.f1nwprt3())
                .field("f1nwprt4", &self.f1nwprt4())
                .field("f1nwprt5", &self.f1nwprt5())
                .field("f1nwprt6", &self.f1nwprt6())
                .field("f1nwprt7", &self.f1nwprt7())
                .field("f1nwprt8", &self.f1nwprt8())
                .field("f1nwprt9", &self.f1nwprt9())
                .field("f1nwprt10", &self.f1nwprt10())
                .field("f1nwprt11", &self.f1nwprt11())
                .field("f1nwprt12", &self.f1nwprt12())
                .field("f1nwprt13", &self.f1nwprt13())
                .field("f1nwprt14", &self.f1nwprt14())
                .field("f1nwprt15", &self.f1nwprt15())
                .field("f1nwprt16", &self.f1nwprt16())
                .field("f1nwprt17", &self.f1nwprt17())
                .field("f1nwprt18", &self.f1nwprt18())
                .field("f1nwprt19", &self.f1nwprt19())
                .field("f1nwprt20", &self.f1nwprt20())
                .field("f1nwprt21", &self.f1nwprt21())
                .field("f1nwprt22", &self.f1nwprt22())
                .field("f1nwprt23", &self.f1nwprt23())
                .field("f1nwprt24", &self.f1nwprt24())
                .field("f1nwprt25", &self.f1nwprt25())
                .field("f1nwprt26", &self.f1nwprt26())
                .field("f1nwprt27", &self.f1nwprt27())
                .field("f1nwprt28", &self.f1nwprt28())
                .field("f1nwprt29", &self.f1nwprt29())
                .field("f1nwprt30", &self.f1nwprt30())
                .field("f1nwprt31", &self.f1nwprt31())
                .finish()
        }
    }
    #[doc = "desc FAPRT."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Faprt(pub u32);
    impl Faprt {
        #[doc = "desc FAPRT."]
        #[must_use]
        #[inline(always)]
        pub const fn faprt(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[doc = "desc FAPRT."]
        #[inline(always)]
        pub const fn set_faprt(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
        }
    }
    impl Default for Faprt {
        #[inline(always)]
        fn default() -> Faprt {
            Faprt(0)
        }
    }
    impl core::fmt::Debug for Faprt {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Faprt")
                .field("faprt", &self.faprt())
                .finish()
        }
    }
    #[doc = "desc FITE."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Fite(pub u32);
    impl Fite {
        #[doc = "desc PEERRITE."]
        #[must_use]
        #[inline(always)]
        pub const fn peerrite(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "desc PEERRITE."]
        #[inline(always)]
        pub const fn set_peerrite(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "desc OPTENDITE."]
        #[must_use]
        #[inline(always)]
        pub const fn optendite(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "desc OPTENDITE."]
        #[inline(always)]
        pub const fn set_optendite(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "desc COLERRITE."]
        #[must_use]
        #[inline(always)]
        pub const fn colerrite(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "desc COLERRITE."]
        #[inline(always)]
        pub const fn set_colerrite(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
    }
    impl Default for Fite {
        #[inline(always)]
        fn default() -> Fite {
            Fite(0)
        }
    }
    impl core::fmt::Debug for Fite {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Fite")
                .field("peerrite", &self.peerrite())
                .field("optendite", &self.optendite())
                .field("colerrite", &self.colerrite())
                .finish()
        }
    }
    #[doc = "desc FRMC."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Frmc(pub u32);
    impl Frmc {
        #[doc = "desc FLWT."]
        #[must_use]
        #[inline(always)]
        pub const fn flwt(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x0f;
            val as u8
        }
        #[doc = "desc FLWT."]
        #[inline(always)]
        pub const fn set_flwt(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
        }
        #[doc = "desc LVM."]
        #[must_use]
        #[inline(always)]
        pub const fn lvm(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "desc LVM."]
        #[inline(always)]
        pub const fn set_lvm(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "desc ICACHE."]
        #[must_use]
        #[inline(always)]
        pub const fn icache(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "desc ICACHE."]
        #[inline(always)]
        pub const fn set_icache(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "desc DCACHE."]
        #[must_use]
        #[inline(always)]
        pub const fn dcache(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "desc DCACHE."]
        #[inline(always)]
        pub const fn set_dcache(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[doc = "desc PREFETE."]
        #[must_use]
        #[inline(always)]
        pub const fn prefete(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[doc = "desc PREFETE."]
        #[inline(always)]
        pub const fn set_prefete(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
        #[doc = "desc CRST."]
        #[must_use]
        #[inline(always)]
        pub const fn crst(&self) -> bool {
            let val = (self.0 >> 19usize) & 0x01;
            val != 0
        }
        #[doc = "desc CRST."]
        #[inline(always)]
        pub const fn set_crst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
        }
    }
    impl Default for Frmc {
        #[inline(always)]
        fn default() -> Frmc {
            Frmc(0)
        }
    }
    impl core::fmt::Debug for Frmc {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Frmc")
                .field("flwt", &self.flwt())
                .field("lvm", &self.lvm())
                .field("icache", &self.icache())
                .field("dcache", &self.dcache())
                .field("prefete", &self.prefete())
                .field("crst", &self.crst())
                .finish()
        }
    }
    #[doc = "desc FSCLR."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Fsclr(pub u32);
    impl Fsclr {
        #[doc = "desc OTPWERRCLR0."]
        #[must_use]
        #[inline(always)]
        pub const fn otpwerrclr0(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "desc OTPWERRCLR0."]
        #[inline(always)]
        pub const fn set_otpwerrclr0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "desc PRTWERRCLR0."]
        #[must_use]
        #[inline(always)]
        pub const fn prtwerrclr0(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "desc PRTWERRCLR0."]
        #[inline(always)]
        pub const fn set_prtwerrclr0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "desc PGSZERRCLR0."]
        #[must_use]
        #[inline(always)]
        pub const fn pgszerrclr0(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "desc PGSZERRCLR0."]
        #[inline(always)]
        pub const fn set_pgszerrclr0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "desc MISMTCHCLR0."]
        #[must_use]
        #[inline(always)]
        pub const fn mismtchclr0(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "desc MISMTCHCLR0."]
        #[inline(always)]
        pub const fn set_mismtchclr0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "desc OPTENDCLR0."]
        #[must_use]
        #[inline(always)]
        pub const fn optendclr0(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "desc OPTENDCLR0."]
        #[inline(always)]
        pub const fn set_optendclr0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "desc COLERRCLR0."]
        #[must_use]
        #[inline(always)]
        pub const fn colerrclr0(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "desc COLERRCLR0."]
        #[inline(always)]
        pub const fn set_colerrclr0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "desc PRTWERRCLR1."]
        #[must_use]
        #[inline(always)]
        pub const fn prtwerrclr1(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "desc PRTWERRCLR1."]
        #[inline(always)]
        pub const fn set_prtwerrclr1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[doc = "desc PGSZERRCLR1."]
        #[must_use]
        #[inline(always)]
        pub const fn pgszerrclr1(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[doc = "desc PGSZERRCLR1."]
        #[inline(always)]
        pub const fn set_pgszerrclr1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
        #[doc = "desc MISMTCHCLR1."]
        #[must_use]
        #[inline(always)]
        pub const fn mismtchclr1(&self) -> bool {
            let val = (self.0 >> 19usize) & 0x01;
            val != 0
        }
        #[doc = "desc MISMTCHCLR1."]
        #[inline(always)]
        pub const fn set_mismtchclr1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
        }
        #[doc = "desc OPTENDCLR1."]
        #[must_use]
        #[inline(always)]
        pub const fn optendclr1(&self) -> bool {
            let val = (self.0 >> 20usize) & 0x01;
            val != 0
        }
        #[doc = "desc OPTENDCLR1."]
        #[inline(always)]
        pub const fn set_optendclr1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
        }
        #[doc = "desc COLERRCLR1."]
        #[must_use]
        #[inline(always)]
        pub const fn colerrclr1(&self) -> bool {
            let val = (self.0 >> 21usize) & 0x01;
            val != 0
        }
        #[doc = "desc COLERRCLR1."]
        #[inline(always)]
        pub const fn set_colerrclr1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
        }
    }
    impl Default for Fsclr {
        #[inline(always)]
        fn default() -> Fsclr {
            Fsclr(0)
        }
    }
    impl core::fmt::Debug for Fsclr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Fsclr")
                .field("otpwerrclr0", &self.otpwerrclr0())
                .field("prtwerrclr0", &self.prtwerrclr0())
                .field("pgszerrclr0", &self.pgszerrclr0())
                .field("mismtchclr0", &self.mismtchclr0())
                .field("optendclr0", &self.optendclr0())
                .field("colerrclr0", &self.colerrclr0())
                .field("prtwerrclr1", &self.prtwerrclr1())
                .field("pgszerrclr1", &self.pgszerrclr1())
                .field("mismtchclr1", &self.mismtchclr1())
                .field("optendclr1", &self.optendclr1())
                .field("colerrclr1", &self.colerrclr1())
                .finish()
        }
    }
    #[doc = "desc FSR."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Fsr(pub u32);
    impl Fsr {
        #[doc = "desc OTPWERR0."]
        #[must_use]
        #[inline(always)]
        pub const fn otpwerr0(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "desc OTPWERR0."]
        #[inline(always)]
        pub const fn set_otpwerr0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "desc PRTWERR0."]
        #[must_use]
        #[inline(always)]
        pub const fn prtwerr0(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "desc PRTWERR0."]
        #[inline(always)]
        pub const fn set_prtwerr0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "desc PGSZERR0."]
        #[must_use]
        #[inline(always)]
        pub const fn pgszerr0(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "desc PGSZERR0."]
        #[inline(always)]
        pub const fn set_pgszerr0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "desc MISMTCH0."]
        #[must_use]
        #[inline(always)]
        pub const fn mismtch0(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "desc MISMTCH0."]
        #[inline(always)]
        pub const fn set_mismtch0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "desc OPTEND0."]
        #[must_use]
        #[inline(always)]
        pub const fn optend0(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "desc OPTEND0."]
        #[inline(always)]
        pub const fn set_optend0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "desc COLERR0."]
        #[must_use]
        #[inline(always)]
        pub const fn colerr0(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "desc COLERR0."]
        #[inline(always)]
        pub const fn set_colerr0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "desc RDY0."]
        #[must_use]
        #[inline(always)]
        pub const fn rdy0(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "desc RDY0."]
        #[inline(always)]
        pub const fn set_rdy0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "desc PRTWERR1."]
        #[must_use]
        #[inline(always)]
        pub const fn prtwerr1(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "desc PRTWERR1."]
        #[inline(always)]
        pub const fn set_prtwerr1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[doc = "desc PGSZERR1."]
        #[must_use]
        #[inline(always)]
        pub const fn pgszerr1(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[doc = "desc PGSZERR1."]
        #[inline(always)]
        pub const fn set_pgszerr1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
        #[doc = "desc MISMTCH1."]
        #[must_use]
        #[inline(always)]
        pub const fn mismtch1(&self) -> bool {
            let val = (self.0 >> 19usize) & 0x01;
            val != 0
        }
        #[doc = "desc MISMTCH1."]
        #[inline(always)]
        pub const fn set_mismtch1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
        }
        #[doc = "desc OPTEND1."]
        #[must_use]
        #[inline(always)]
        pub const fn optend1(&self) -> bool {
            let val = (self.0 >> 20usize) & 0x01;
            val != 0
        }
        #[doc = "desc OPTEND1."]
        #[inline(always)]
        pub const fn set_optend1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
        }
        #[doc = "desc COLERR1."]
        #[must_use]
        #[inline(always)]
        pub const fn colerr1(&self) -> bool {
            let val = (self.0 >> 21usize) & 0x01;
            val != 0
        }
        #[doc = "desc COLERR1."]
        #[inline(always)]
        pub const fn set_colerr1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
        }
        #[doc = "desc RDY1."]
        #[must_use]
        #[inline(always)]
        pub const fn rdy1(&self) -> bool {
            let val = (self.0 >> 24usize) & 0x01;
            val != 0
        }
        #[doc = "desc RDY1."]
        #[inline(always)]
        pub const fn set_rdy1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
        }
    }
    impl Default for Fsr {
        #[inline(always)]
        fn default() -> Fsr {
            Fsr(0)
        }
    }
    impl core::fmt::Debug for Fsr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Fsr")
                .field("otpwerr0", &self.otpwerr0())
                .field("prtwerr0", &self.prtwerr0())
                .field("pgszerr0", &self.pgszerr0())
                .field("mismtch0", &self.mismtch0())
                .field("optend0", &self.optend0())
                .field("colerr0", &self.colerr0())
                .field("rdy0", &self.rdy0())
                .field("prtwerr1", &self.prtwerr1())
                .field("pgszerr1", &self.pgszerr1())
                .field("mismtch1", &self.mismtch1())
                .field("optend1", &self.optend1())
                .field("colerr1", &self.colerr1())
                .field("rdy1", &self.rdy1())
                .finish()
        }
    }
    #[doc = "desc FSTP."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Fstp(pub u32);
    impl Fstp {
        #[doc = "desc F0STP."]
        #[must_use]
        #[inline(always)]
        pub const fn f0stp(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "desc F0STP."]
        #[inline(always)]
        pub const fn set_f0stp(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "desc F1STP."]
        #[must_use]
        #[inline(always)]
        pub const fn f1stp(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "desc F1STP."]
        #[inline(always)]
        pub const fn set_f1stp(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
    }
    impl Default for Fstp {
        #[inline(always)]
        fn default() -> Fstp {
            Fstp(0)
        }
    }
    impl core::fmt::Debug for Fstp {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Fstp")
                .field("f0stp", &self.f0stp())
                .field("f1stp", &self.f1stp())
                .finish()
        }
    }
    #[doc = "desc FSWP."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Fswp(pub u32);
    impl Fswp {
        #[doc = "desc FSWP."]
        #[must_use]
        #[inline(always)]
        pub const fn fswp(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "desc FSWP."]
        #[inline(always)]
        pub const fn set_fswp(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
    }
    impl Default for Fswp {
        #[inline(always)]
        fn default() -> Fswp {
            Fswp(0)
        }
    }
    impl core::fmt::Debug for Fswp {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Fswp").field("fswp", &self.fswp()).finish()
        }
    }
    #[doc = "desc FWMC."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Fwmc(pub u32);
    impl Fwmc {
        #[doc = "desc PEMOD."]
        #[must_use]
        #[inline(always)]
        pub const fn pemod(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x07;
            val as u8
        }
        #[doc = "desc PEMOD."]
        #[inline(always)]
        pub const fn set_pemod(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 0usize)) | (((val as u32) & 0x07) << 0usize);
        }
        #[doc = "desc BUSHLDCTL."]
        #[must_use]
        #[inline(always)]
        pub const fn bushldctl(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "desc BUSHLDCTL."]
        #[inline(always)]
        pub const fn set_bushldctl(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "desc KEY1LOCK."]
        #[must_use]
        #[inline(always)]
        pub const fn key1lock(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "desc KEY1LOCK."]
        #[inline(always)]
        pub const fn set_key1lock(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "desc KEY2LOCK."]
        #[must_use]
        #[inline(always)]
        pub const fn key2lock(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "desc KEY2LOCK."]
        #[inline(always)]
        pub const fn set_key2lock(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
    }
    impl Default for Fwmc {
        #[inline(always)]
        fn default() -> Fwmc {
            Fwmc(0)
        }
    }
    impl core::fmt::Debug for Fwmc {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Fwmc")
                .field("pemod", &self.pemod())
                .field("bushldctl", &self.bushldctl())
                .field("key1lock", &self.key1lock())
                .field("key2lock", &self.key2lock())
                .finish()
        }
    }
    #[doc = "desc MMF_REMCR0."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct MmfRemcr0(pub u32);
    impl MmfRemcr0 {
        #[doc = "desc RMSIZE."]
        #[must_use]
        #[inline(always)]
        pub const fn rmsize(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x1f;
            val as u8
        }
        #[doc = "desc RMSIZE."]
        #[inline(always)]
        pub const fn set_rmsize(&mut self, val: u8) {
            self.0 = (self.0 & !(0x1f << 0usize)) | (((val as u32) & 0x1f) << 0usize);
        }
        #[doc = "desc RMTADDR."]
        #[must_use]
        #[inline(always)]
        pub const fn rmtaddr(&self) -> u32 {
            let val = (self.0 >> 12usize) & 0x0001_ffff;
            val as u32
        }
        #[doc = "desc RMTADDR."]
        #[inline(always)]
        pub const fn set_rmtaddr(&mut self, val: u32) {
            self.0 =
                (self.0 & !(0x0001_ffff << 12usize)) | (((val as u32) & 0x0001_ffff) << 12usize);
        }
        #[doc = "desc EN."]
        #[must_use]
        #[inline(always)]
        pub const fn en(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[doc = "desc EN."]
        #[inline(always)]
        pub const fn set_en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for MmfRemcr0 {
        #[inline(always)]
        fn default() -> MmfRemcr0 {
            MmfRemcr0(0)
        }
    }
    impl core::fmt::Debug for MmfRemcr0 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("MmfRemcr0")
                .field("rmsize", &self.rmsize())
                .field("rmtaddr", &self.rmtaddr())
                .field("en", &self.en())
                .finish()
        }
    }
    #[doc = "desc MMF_REMCR1."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct MmfRemcr1(pub u32);
    impl MmfRemcr1 {
        #[doc = "desc RMSIZE."]
        #[must_use]
        #[inline(always)]
        pub const fn rmsize(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x1f;
            val as u8
        }
        #[doc = "desc RMSIZE."]
        #[inline(always)]
        pub const fn set_rmsize(&mut self, val: u8) {
            self.0 = (self.0 & !(0x1f << 0usize)) | (((val as u32) & 0x1f) << 0usize);
        }
        #[doc = "desc RMTADDR."]
        #[must_use]
        #[inline(always)]
        pub const fn rmtaddr(&self) -> u32 {
            let val = (self.0 >> 12usize) & 0x0001_ffff;
            val as u32
        }
        #[doc = "desc RMTADDR."]
        #[inline(always)]
        pub const fn set_rmtaddr(&mut self, val: u32) {
            self.0 =
                (self.0 & !(0x0001_ffff << 12usize)) | (((val as u32) & 0x0001_ffff) << 12usize);
        }
        #[doc = "desc EN."]
        #[must_use]
        #[inline(always)]
        pub const fn en(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[doc = "desc EN."]
        #[inline(always)]
        pub const fn set_en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for MmfRemcr1 {
        #[inline(always)]
        fn default() -> MmfRemcr1 {
            MmfRemcr1(0)
        }
    }
    impl core::fmt::Debug for MmfRemcr1 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("MmfRemcr1")
                .field("rmsize", &self.rmsize())
                .field("rmtaddr", &self.rmtaddr())
                .field("en", &self.en())
                .finish()
        }
    }
    #[doc = "desc MMF_REMPRT."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct MmfRemprt(pub u32);
    impl MmfRemprt {
        #[doc = "desc MMF_REMPRT."]
        #[must_use]
        #[inline(always)]
        pub const fn mmf_remprt(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[doc = "desc MMF_REMPRT."]
        #[inline(always)]
        pub const fn set_mmf_remprt(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
        }
    }
    impl Default for MmfRemprt {
        #[inline(always)]
        fn default() -> MmfRemprt {
            MmfRemprt(0)
        }
    }
    impl core::fmt::Debug for MmfRemprt {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("MmfRemprt")
                .field("mmf_remprt", &self.mmf_remprt())
                .finish()
        }
    }
    #[doc = "desc WLOCK."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Wlock(pub u32);
    impl Wlock {
        #[doc = "desc WLOCK0."]
        #[must_use]
        #[inline(always)]
        pub const fn wlock0(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "desc WLOCK0."]
        #[inline(always)]
        pub const fn set_wlock0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "desc WLOCK1."]
        #[must_use]
        #[inline(always)]
        pub const fn wlock1(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "desc WLOCK1."]
        #[inline(always)]
        pub const fn set_wlock1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
    }
    impl Default for Wlock {
        #[inline(always)]
        fn default() -> Wlock {
            Wlock(0)
        }
    }
    impl core::fmt::Debug for Wlock {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Wlock")
                .field("wlock0", &self.wlock0())
                .field("wlock1", &self.wlock1())
                .finish()
        }
    }
}
