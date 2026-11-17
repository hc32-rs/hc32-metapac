#![allow(clippy::missing_safety_doc)]
#![allow(clippy::identity_op)]
#![allow(clippy::unnecessary_cast)]
#![allow(clippy::erasing_op)]

#[doc = "desc MAU."]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Mau {
    ptr: *mut u8,
}
unsafe impl Send for Mau {}
unsafe impl Sync for Mau {}
impl Mau {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "desc CSR."]
    #[inline(always)]
    pub const fn csr(self) -> crate::common::Reg<regs::Csr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0usize) as _) }
    }
    #[doc = "desc DTR0."]
    #[inline(always)]
    pub const fn dtr0(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x04usize) as _) }
    }
    #[doc = "desc RTR0."]
    #[inline(always)]
    pub const fn rtr0(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0cusize) as _) }
    }
    #[doc = "desc DTR1."]
    #[inline(always)]
    pub const fn dtr1(self) -> crate::common::Reg<regs::Dtr1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x10usize) as _) }
    }
    #[doc = "desc RTR1."]
    #[inline(always)]
    pub const fn rtr1(self) -> crate::common::Reg<regs::Rtr1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x14usize) as _) }
    }
}
pub mod regs {
    #[doc = "desc CSR."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Csr(pub u32);
    impl Csr {
        #[doc = "desc START."]
        #[must_use]
        #[inline(always)]
        pub const fn start(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "desc START."]
        #[inline(always)]
        pub const fn set_start(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "desc INTEN."]
        #[must_use]
        #[inline(always)]
        pub const fn inten(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "desc INTEN."]
        #[inline(always)]
        pub const fn set_inten(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "desc BUSY."]
        #[must_use]
        #[inline(always)]
        pub const fn busy(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "desc BUSY."]
        #[inline(always)]
        pub const fn set_busy(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "desc SHIFT."]
        #[must_use]
        #[inline(always)]
        pub const fn shift(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0x1f;
            val as u8
        }
        #[doc = "desc SHIFT."]
        #[inline(always)]
        pub const fn set_shift(&mut self, val: u8) {
            self.0 = (self.0 & !(0x1f << 8usize)) | (((val as u32) & 0x1f) << 8usize);
        }
    }
    impl Default for Csr {
        #[inline(always)]
        fn default() -> Csr {
            Csr(0)
        }
    }
    impl core::fmt::Debug for Csr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Csr")
                .field("start", &self.start())
                .field("inten", &self.inten())
                .field("busy", &self.busy())
                .field("shift", &self.shift())
                .finish()
        }
    }
    #[doc = "desc DTR1."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Dtr1(pub u32);
    impl Dtr1 {
        #[doc = "desc SIN_DIN."]
        #[must_use]
        #[inline(always)]
        pub const fn sin_din(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x0fff;
            val as u16
        }
        #[doc = "desc SIN_DIN."]
        #[inline(always)]
        pub const fn set_sin_din(&mut self, val: u16) {
            self.0 = (self.0 & !(0x0fff << 0usize)) | (((val as u32) & 0x0fff) << 0usize);
        }
    }
    impl Default for Dtr1 {
        #[inline(always)]
        fn default() -> Dtr1 {
            Dtr1(0)
        }
    }
    impl core::fmt::Debug for Dtr1 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Dtr1")
                .field("sin_din", &self.sin_din())
                .finish()
        }
    }
    #[doc = "desc RTR1."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Rtr1(pub u32);
    impl Rtr1 {
        #[doc = "desc SIN_DOUT."]
        #[must_use]
        #[inline(always)]
        pub const fn sin_dout(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[doc = "desc SIN_DOUT."]
        #[inline(always)]
        pub const fn set_sin_dout(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
        }
    }
    impl Default for Rtr1 {
        #[inline(always)]
        fn default() -> Rtr1 {
            Rtr1(0)
        }
    }
    impl core::fmt::Debug for Rtr1 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Rtr1")
                .field("sin_dout", &self.sin_dout())
                .finish()
        }
    }
}
