#![allow(clippy::missing_safety_doc)]
#![allow(clippy::identity_op)]
#![allow(clippy::unnecessary_cast)]
#![allow(clippy::erasing_op)]

#[doc = "desc PERIC."]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Peric {
    ptr: *mut u8,
}
unsafe impl Send for Peric {}
unsafe impl Sync for Peric {}
impl Peric {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "desc USBFS_SYCTLREG."]
    #[inline(always)]
    pub const fn usbfs_syctlreg(
        self,
    ) -> crate::common::Reg<regs::UsbfsSyctlreg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0usize) as _) }
    }
    #[doc = "desc SDIOC_SYCTLREG."]
    #[inline(always)]
    pub const fn sdioc_syctlreg(
        self,
    ) -> crate::common::Reg<regs::SdiocSyctlreg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x04usize) as _) }
    }
}
pub mod regs {
    #[doc = "desc SDIOC_SYCTLREG."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct SdiocSyctlreg(pub u32);
    impl SdiocSyctlreg {
        #[doc = "desc SELMMC1."]
        #[must_use]
        #[inline(always)]
        pub const fn selmmc1(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "desc SELMMC1."]
        #[inline(always)]
        pub const fn set_selmmc1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "desc SELMMC2."]
        #[must_use]
        #[inline(always)]
        pub const fn selmmc2(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "desc SELMMC2."]
        #[inline(always)]
        pub const fn set_selmmc2(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
    }
    impl Default for SdiocSyctlreg {
        #[inline(always)]
        fn default() -> SdiocSyctlreg {
            SdiocSyctlreg(0)
        }
    }
    impl core::fmt::Debug for SdiocSyctlreg {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("SdiocSyctlreg")
                .field("selmmc1", &self.selmmc1())
                .field("selmmc2", &self.selmmc2())
                .finish()
        }
    }
    #[doc = "desc USBFS_SYCTLREG."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct UsbfsSyctlreg(pub u32);
    impl UsbfsSyctlreg {
        #[doc = "desc DFB."]
        #[must_use]
        #[inline(always)]
        pub const fn dfb(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "desc DFB."]
        #[inline(always)]
        pub const fn set_dfb(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "desc SOFEN."]
        #[must_use]
        #[inline(always)]
        pub const fn sofen(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "desc SOFEN."]
        #[inline(always)]
        pub const fn set_sofen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
    }
    impl Default for UsbfsSyctlreg {
        #[inline(always)]
        fn default() -> UsbfsSyctlreg {
            UsbfsSyctlreg(0)
        }
    }
    impl core::fmt::Debug for UsbfsSyctlreg {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("UsbfsSyctlreg")
                .field("dfb", &self.dfb())
                .field("sofen", &self.sofen())
                .finish()
        }
    }
}
