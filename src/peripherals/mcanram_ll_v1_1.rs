#![allow(clippy::missing_safety_doc)]
#![allow(clippy::identity_op)]
#![allow(clippy::unnecessary_cast)]
#![allow(clippy::erasing_op)]

#[doc = "desc MCANRAM."]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Mcanram {
    ptr: *mut u8,
}
unsafe impl Send for Mcanram {}
unsafe impl Sync for Mcanram {}
impl Mcanram {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "desc CKCR."]
    #[inline(always)]
    pub const fn ckcr(self) -> crate::common::Reg<regs::Ckcr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0usize) as _) }
    }
    #[doc = "desc CKSR."]
    #[inline(always)]
    pub const fn cksr(self) -> crate::common::Reg<regs::Cksr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x04usize) as _) }
    }
    #[doc = "desc EIEN."]
    #[inline(always)]
    pub const fn eien(self) -> crate::common::Reg<regs::Eien, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x08usize) as _) }
    }
    #[doc = "desc EIBIT0."]
    #[inline(always)]
    pub const fn eibit0(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0cusize) as _) }
    }
    #[doc = "desc EIBIT1."]
    #[inline(always)]
    pub const fn eibit1(self) -> crate::common::Reg<regs::Eibit1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x10usize) as _) }
    }
    #[doc = "desc ECCERRADDR."]
    #[inline(always)]
    pub const fn eccerraddr(self) -> crate::common::Reg<regs::Eccerraddr, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x14usize) as _) }
    }
}
pub mod regs {
    #[doc = "desc CKCR."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ckcr(pub u32);
    impl Ckcr {
        #[doc = "desc ECCMOD."]
        #[must_use]
        #[inline(always)]
        pub const fn eccmod(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x03;
            val as u8
        }
        #[doc = "desc ECCMOD."]
        #[inline(always)]
        pub const fn set_eccmod(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 0usize)) | (((val as u32) & 0x03) << 0usize);
        }
    }
    impl Default for Ckcr {
        #[inline(always)]
        fn default() -> Ckcr {
            Ckcr(0)
        }
    }
    impl core::fmt::Debug for Ckcr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Ckcr")
                .field("eccmod", &self.eccmod())
                .finish()
        }
    }
    #[doc = "desc CKSR."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Cksr(pub u32);
    impl Cksr {
        #[doc = "desc RAM_1ERR."]
        #[must_use]
        #[inline(always)]
        pub const fn ram_1err(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "desc RAM_1ERR."]
        #[inline(always)]
        pub const fn set_ram_1err(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "desc RAM_2ERR."]
        #[must_use]
        #[inline(always)]
        pub const fn ram_2err(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "desc RAM_2ERR."]
        #[inline(always)]
        pub const fn set_ram_2err(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
    }
    impl Default for Cksr {
        #[inline(always)]
        fn default() -> Cksr {
            Cksr(0)
        }
    }
    impl core::fmt::Debug for Cksr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Cksr")
                .field("ram_1err", &self.ram_1err())
                .field("ram_2err", &self.ram_2err())
                .finish()
        }
    }
    #[doc = "desc ECCERRADDR."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Eccerraddr(pub u32);
    impl Eccerraddr {
        #[doc = "desc ECCERRADDR."]
        #[must_use]
        #[inline(always)]
        pub const fn eccerraddr(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x01ff;
            val as u16
        }
        #[doc = "desc ECCERRADDR."]
        #[inline(always)]
        pub const fn set_eccerraddr(&mut self, val: u16) {
            self.0 = (self.0 & !(0x01ff << 0usize)) | (((val as u32) & 0x01ff) << 0usize);
        }
    }
    impl Default for Eccerraddr {
        #[inline(always)]
        fn default() -> Eccerraddr {
            Eccerraddr(0)
        }
    }
    impl core::fmt::Debug for Eccerraddr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Eccerraddr")
                .field("eccerraddr", &self.eccerraddr())
                .finish()
        }
    }
    #[doc = "desc EIBIT1."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Eibit1(pub u32);
    impl Eibit1 {
        #[doc = "desc EIBIT."]
        #[must_use]
        #[inline(always)]
        pub const fn eibit(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x7f;
            val as u8
        }
        #[doc = "desc EIBIT."]
        #[inline(always)]
        pub const fn set_eibit(&mut self, val: u8) {
            self.0 = (self.0 & !(0x7f << 0usize)) | (((val as u32) & 0x7f) << 0usize);
        }
    }
    impl Default for Eibit1 {
        #[inline(always)]
        fn default() -> Eibit1 {
            Eibit1(0)
        }
    }
    impl core::fmt::Debug for Eibit1 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Eibit1")
                .field("eibit", &self.eibit())
                .finish()
        }
    }
    #[doc = "desc EIEN."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Eien(pub u32);
    impl Eien {
        #[doc = "desc EIEN."]
        #[must_use]
        #[inline(always)]
        pub const fn eien(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "desc EIEN."]
        #[inline(always)]
        pub const fn set_eien(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
    }
    impl Default for Eien {
        #[inline(always)]
        fn default() -> Eien {
            Eien(0)
        }
    }
    impl core::fmt::Debug for Eien {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Eien").field("eien", &self.eien()).finish()
        }
    }
}
