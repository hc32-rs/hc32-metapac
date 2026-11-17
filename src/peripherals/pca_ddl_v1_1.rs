#![allow(clippy::missing_safety_doc)]
#![allow(clippy::identity_op)]
#![allow(clippy::unnecessary_cast)]
#![allow(clippy::erasing_op)]

#[doc = "desc PCA."]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pca {
    ptr: *mut u8,
}
unsafe impl Send for Pca {}
unsafe impl Sync for Pca {}
impl Pca {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "desc CCON."]
    #[inline(always)]
    pub const fn ccon(self) -> crate::common::Reg<regs::Ccon, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0usize) as _) }
    }
    #[doc = "desc CMOD."]
    #[inline(always)]
    pub const fn cmod(self) -> crate::common::Reg<regs::Cmod, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x04usize) as _) }
    }
    #[doc = "desc CNT."]
    #[inline(always)]
    pub const fn cnt(self) -> crate::common::Reg<regs::Cnt, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x08usize) as _) }
    }
    #[doc = "desc ICLR."]
    #[inline(always)]
    pub const fn iclr(self) -> crate::common::Reg<regs::Iclr, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0cusize) as _) }
    }
    #[doc = "desc CCAPM0."]
    #[inline(always)]
    pub const fn ccapm0(self) -> crate::common::Reg<regs::Ccapm0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x10usize) as _) }
    }
    #[doc = "desc CCAPM1."]
    #[inline(always)]
    pub const fn ccapm1(self) -> crate::common::Reg<regs::Ccapm1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x14usize) as _) }
    }
    #[doc = "desc CCAPM2."]
    #[inline(always)]
    pub const fn ccapm2(self) -> crate::common::Reg<regs::Ccapm2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x18usize) as _) }
    }
    #[doc = "desc CCAPM3."]
    #[inline(always)]
    pub const fn ccapm3(self) -> crate::common::Reg<regs::Ccapm3, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1cusize) as _) }
    }
    #[doc = "desc CCAPM4."]
    #[inline(always)]
    pub const fn ccapm4(self) -> crate::common::Reg<regs::Ccapm4, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x20usize) as _) }
    }
    #[doc = "desc CCAP0H."]
    #[inline(always)]
    pub const fn ccap0h(self) -> crate::common::Reg<regs::Ccap0h, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x24usize) as _) }
    }
    #[doc = "desc CCAP0L."]
    #[inline(always)]
    pub const fn ccap0l(self) -> crate::common::Reg<regs::Ccap0l, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x28usize) as _) }
    }
    #[doc = "desc CCAP1H."]
    #[inline(always)]
    pub const fn ccap1h(self) -> crate::common::Reg<regs::Ccap1h, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x2cusize) as _) }
    }
    #[doc = "desc CCAP1L."]
    #[inline(always)]
    pub const fn ccap1l(self) -> crate::common::Reg<regs::Ccap1l, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x30usize) as _) }
    }
    #[doc = "desc CCAP2H."]
    #[inline(always)]
    pub const fn ccap2h(self) -> crate::common::Reg<regs::Ccap2h, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x34usize) as _) }
    }
    #[doc = "desc CCAP2L."]
    #[inline(always)]
    pub const fn ccap2l(self) -> crate::common::Reg<regs::Ccap2l, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x38usize) as _) }
    }
    #[doc = "desc CCAP3H."]
    #[inline(always)]
    pub const fn ccap3h(self) -> crate::common::Reg<regs::Ccap3h, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x3cusize) as _) }
    }
    #[doc = "desc CCAP3L."]
    #[inline(always)]
    pub const fn ccap3l(self) -> crate::common::Reg<regs::Ccap3l, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x40usize) as _) }
    }
    #[doc = "desc CCAP4H."]
    #[inline(always)]
    pub const fn ccap4h(self) -> crate::common::Reg<regs::Ccap4h, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x44usize) as _) }
    }
    #[doc = "desc CCAP4L."]
    #[inline(always)]
    pub const fn ccap4l(self) -> crate::common::Reg<regs::Ccap4l, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x48usize) as _) }
    }
    #[doc = "desc CCAPO."]
    #[inline(always)]
    pub const fn ccapo(self) -> crate::common::Reg<regs::Ccapo, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x4cusize) as _) }
    }
    #[doc = "desc CCAP0."]
    #[inline(always)]
    pub const fn ccap0(self) -> crate::common::Reg<regs::Ccap0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x50usize) as _) }
    }
    #[doc = "desc CCAP1."]
    #[inline(always)]
    pub const fn ccap1(self) -> crate::common::Reg<regs::Ccap1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x54usize) as _) }
    }
    #[doc = "desc CCAP2."]
    #[inline(always)]
    pub const fn ccap2(self) -> crate::common::Reg<regs::Ccap2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x58usize) as _) }
    }
    #[doc = "desc CCAP3."]
    #[inline(always)]
    pub const fn ccap3(self) -> crate::common::Reg<regs::Ccap3, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x5cusize) as _) }
    }
    #[doc = "desc CCAP4."]
    #[inline(always)]
    pub const fn ccap4(self) -> crate::common::Reg<regs::Ccap4, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x60usize) as _) }
    }
}
pub mod regs {
    #[doc = "desc CCAP0."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ccap0(pub u32);
    impl Ccap0 {
        #[doc = "desc CCAP0."]
        #[must_use]
        #[inline(always)]
        pub const fn ccap0(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[doc = "desc CCAP0."]
        #[inline(always)]
        pub const fn set_ccap0(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
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
    impl Default for Ccap0 {
        #[inline(always)]
        fn default() -> Ccap0 {
            Ccap0(0)
        }
    }
    impl core::fmt::Debug for Ccap0 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Ccap0")
                .field("ccap0", &self.ccap0())
                .field("rsv", &self.rsv())
                .finish()
        }
    }
    #[doc = "desc CCAP0H."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ccap0h(pub u32);
    impl Ccap0h {
        #[doc = "desc CCAP0."]
        #[must_use]
        #[inline(always)]
        pub const fn ccap0(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[doc = "desc CCAP0."]
        #[inline(always)]
        pub const fn set_ccap0(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
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
    impl Default for Ccap0h {
        #[inline(always)]
        fn default() -> Ccap0h {
            Ccap0h(0)
        }
    }
    impl core::fmt::Debug for Ccap0h {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Ccap0h")
                .field("ccap0", &self.ccap0())
                .field("rsv", &self.rsv())
                .finish()
        }
    }
    #[doc = "desc CCAP0L."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ccap0l(pub u32);
    impl Ccap0l {
        #[doc = "desc CCAP0."]
        #[must_use]
        #[inline(always)]
        pub const fn ccap0(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[doc = "desc CCAP0."]
        #[inline(always)]
        pub const fn set_ccap0(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
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
    impl Default for Ccap0l {
        #[inline(always)]
        fn default() -> Ccap0l {
            Ccap0l(0)
        }
    }
    impl core::fmt::Debug for Ccap0l {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Ccap0l")
                .field("ccap0", &self.ccap0())
                .field("rsv", &self.rsv())
                .finish()
        }
    }
    #[doc = "desc CCAP1."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ccap1(pub u32);
    impl Ccap1 {
        #[doc = "desc CCAP1."]
        #[must_use]
        #[inline(always)]
        pub const fn ccap1(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[doc = "desc CCAP1."]
        #[inline(always)]
        pub const fn set_ccap1(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
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
    impl Default for Ccap1 {
        #[inline(always)]
        fn default() -> Ccap1 {
            Ccap1(0)
        }
    }
    impl core::fmt::Debug for Ccap1 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Ccap1")
                .field("ccap1", &self.ccap1())
                .field("rsv", &self.rsv())
                .finish()
        }
    }
    #[doc = "desc CCAP1H."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ccap1h(pub u32);
    impl Ccap1h {
        #[doc = "desc CCAP1."]
        #[must_use]
        #[inline(always)]
        pub const fn ccap1(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[doc = "desc CCAP1."]
        #[inline(always)]
        pub const fn set_ccap1(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
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
    impl Default for Ccap1h {
        #[inline(always)]
        fn default() -> Ccap1h {
            Ccap1h(0)
        }
    }
    impl core::fmt::Debug for Ccap1h {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Ccap1h")
                .field("ccap1", &self.ccap1())
                .field("rsv", &self.rsv())
                .finish()
        }
    }
    #[doc = "desc CCAP1L."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ccap1l(pub u32);
    impl Ccap1l {
        #[doc = "desc CCAP1."]
        #[must_use]
        #[inline(always)]
        pub const fn ccap1(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[doc = "desc CCAP1."]
        #[inline(always)]
        pub const fn set_ccap1(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
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
    impl Default for Ccap1l {
        #[inline(always)]
        fn default() -> Ccap1l {
            Ccap1l(0)
        }
    }
    impl core::fmt::Debug for Ccap1l {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Ccap1l")
                .field("ccap1", &self.ccap1())
                .field("rsv", &self.rsv())
                .finish()
        }
    }
    #[doc = "desc CCAP2."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ccap2(pub u32);
    impl Ccap2 {
        #[doc = "desc CCAP2."]
        #[must_use]
        #[inline(always)]
        pub const fn ccap2(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[doc = "desc CCAP2."]
        #[inline(always)]
        pub const fn set_ccap2(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
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
    impl Default for Ccap2 {
        #[inline(always)]
        fn default() -> Ccap2 {
            Ccap2(0)
        }
    }
    impl core::fmt::Debug for Ccap2 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Ccap2")
                .field("ccap2", &self.ccap2())
                .field("rsv", &self.rsv())
                .finish()
        }
    }
    #[doc = "desc CCAP2H."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ccap2h(pub u32);
    impl Ccap2h {
        #[doc = "desc CCAP2."]
        #[must_use]
        #[inline(always)]
        pub const fn ccap2(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[doc = "desc CCAP2."]
        #[inline(always)]
        pub const fn set_ccap2(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
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
    impl Default for Ccap2h {
        #[inline(always)]
        fn default() -> Ccap2h {
            Ccap2h(0)
        }
    }
    impl core::fmt::Debug for Ccap2h {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Ccap2h")
                .field("ccap2", &self.ccap2())
                .field("rsv", &self.rsv())
                .finish()
        }
    }
    #[doc = "desc CCAP2L."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ccap2l(pub u32);
    impl Ccap2l {
        #[doc = "desc CCAP2."]
        #[must_use]
        #[inline(always)]
        pub const fn ccap2(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[doc = "desc CCAP2."]
        #[inline(always)]
        pub const fn set_ccap2(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
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
    impl Default for Ccap2l {
        #[inline(always)]
        fn default() -> Ccap2l {
            Ccap2l(0)
        }
    }
    impl core::fmt::Debug for Ccap2l {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Ccap2l")
                .field("ccap2", &self.ccap2())
                .field("rsv", &self.rsv())
                .finish()
        }
    }
    #[doc = "desc CCAP3."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ccap3(pub u32);
    impl Ccap3 {
        #[doc = "desc CCAP3."]
        #[must_use]
        #[inline(always)]
        pub const fn ccap3(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[doc = "desc CCAP3."]
        #[inline(always)]
        pub const fn set_ccap3(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
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
    impl Default for Ccap3 {
        #[inline(always)]
        fn default() -> Ccap3 {
            Ccap3(0)
        }
    }
    impl core::fmt::Debug for Ccap3 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Ccap3")
                .field("ccap3", &self.ccap3())
                .field("rsv", &self.rsv())
                .finish()
        }
    }
    #[doc = "desc CCAP3H."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ccap3h(pub u32);
    impl Ccap3h {
        #[doc = "desc CCAP3."]
        #[must_use]
        #[inline(always)]
        pub const fn ccap3(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[doc = "desc CCAP3."]
        #[inline(always)]
        pub const fn set_ccap3(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
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
    impl Default for Ccap3h {
        #[inline(always)]
        fn default() -> Ccap3h {
            Ccap3h(0)
        }
    }
    impl core::fmt::Debug for Ccap3h {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Ccap3h")
                .field("ccap3", &self.ccap3())
                .field("rsv", &self.rsv())
                .finish()
        }
    }
    #[doc = "desc CCAP3L."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ccap3l(pub u32);
    impl Ccap3l {
        #[doc = "desc CCAP3."]
        #[must_use]
        #[inline(always)]
        pub const fn ccap3(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[doc = "desc CCAP3."]
        #[inline(always)]
        pub const fn set_ccap3(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
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
    impl Default for Ccap3l {
        #[inline(always)]
        fn default() -> Ccap3l {
            Ccap3l(0)
        }
    }
    impl core::fmt::Debug for Ccap3l {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Ccap3l")
                .field("ccap3", &self.ccap3())
                .field("rsv", &self.rsv())
                .finish()
        }
    }
    #[doc = "desc CCAP4."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ccap4(pub u32);
    impl Ccap4 {
        #[doc = "desc CCAP4."]
        #[must_use]
        #[inline(always)]
        pub const fn ccap4(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[doc = "desc CCAP4."]
        #[inline(always)]
        pub const fn set_ccap4(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
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
    impl Default for Ccap4 {
        #[inline(always)]
        fn default() -> Ccap4 {
            Ccap4(0)
        }
    }
    impl core::fmt::Debug for Ccap4 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Ccap4")
                .field("ccap4", &self.ccap4())
                .field("rsv", &self.rsv())
                .finish()
        }
    }
    #[doc = "desc CCAP4H."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ccap4h(pub u32);
    impl Ccap4h {
        #[doc = "desc CCAP4."]
        #[must_use]
        #[inline(always)]
        pub const fn ccap4(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[doc = "desc CCAP4."]
        #[inline(always)]
        pub const fn set_ccap4(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
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
    impl Default for Ccap4h {
        #[inline(always)]
        fn default() -> Ccap4h {
            Ccap4h(0)
        }
    }
    impl core::fmt::Debug for Ccap4h {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Ccap4h")
                .field("ccap4", &self.ccap4())
                .field("rsv", &self.rsv())
                .finish()
        }
    }
    #[doc = "desc CCAP4L."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ccap4l(pub u32);
    impl Ccap4l {
        #[doc = "desc CCAP4."]
        #[must_use]
        #[inline(always)]
        pub const fn ccap4(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[doc = "desc CCAP4."]
        #[inline(always)]
        pub const fn set_ccap4(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
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
    impl Default for Ccap4l {
        #[inline(always)]
        fn default() -> Ccap4l {
            Ccap4l(0)
        }
    }
    impl core::fmt::Debug for Ccap4l {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Ccap4l")
                .field("ccap4", &self.ccap4())
                .field("rsv", &self.rsv())
                .finish()
        }
    }
    #[doc = "desc CCAPM0."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ccapm0(pub u32);
    impl Ccapm0 {
        #[doc = "desc CCIE."]
        #[must_use]
        #[inline(always)]
        pub const fn ccie(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "desc CCIE."]
        #[inline(always)]
        pub const fn set_ccie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "desc PWM."]
        #[must_use]
        #[inline(always)]
        pub const fn pwm(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "desc PWM."]
        #[inline(always)]
        pub const fn set_pwm(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "desc TOG."]
        #[must_use]
        #[inline(always)]
        pub const fn tog(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "desc TOG."]
        #[inline(always)]
        pub const fn set_tog(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "desc MAT."]
        #[must_use]
        #[inline(always)]
        pub const fn mat(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "desc MAT."]
        #[inline(always)]
        pub const fn set_mat(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "desc CAPN."]
        #[must_use]
        #[inline(always)]
        pub const fn capn(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "desc CAPN."]
        #[inline(always)]
        pub const fn set_capn(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "desc CAPP."]
        #[must_use]
        #[inline(always)]
        pub const fn capp(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "desc CAPP."]
        #[inline(always)]
        pub const fn set_capp(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "desc ECOM."]
        #[must_use]
        #[inline(always)]
        pub const fn ecom(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "desc ECOM."]
        #[inline(always)]
        pub const fn set_ecom(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
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
    impl Default for Ccapm0 {
        #[inline(always)]
        fn default() -> Ccapm0 {
            Ccapm0(0)
        }
    }
    impl core::fmt::Debug for Ccapm0 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Ccapm0")
                .field("ccie", &self.ccie())
                .field("pwm", &self.pwm())
                .field("tog", &self.tog())
                .field("mat", &self.mat())
                .field("capn", &self.capn())
                .field("capp", &self.capp())
                .field("ecom", &self.ecom())
                .field("rsv", &self.rsv())
                .finish()
        }
    }
    #[doc = "desc CCAPM1."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ccapm1(pub u32);
    impl Ccapm1 {
        #[doc = "desc CCIE."]
        #[must_use]
        #[inline(always)]
        pub const fn ccie(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "desc CCIE."]
        #[inline(always)]
        pub const fn set_ccie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "desc PWM."]
        #[must_use]
        #[inline(always)]
        pub const fn pwm(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "desc PWM."]
        #[inline(always)]
        pub const fn set_pwm(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "desc TOG."]
        #[must_use]
        #[inline(always)]
        pub const fn tog(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "desc TOG."]
        #[inline(always)]
        pub const fn set_tog(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "desc MAT."]
        #[must_use]
        #[inline(always)]
        pub const fn mat(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "desc MAT."]
        #[inline(always)]
        pub const fn set_mat(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "desc CAPN."]
        #[must_use]
        #[inline(always)]
        pub const fn capn(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "desc CAPN."]
        #[inline(always)]
        pub const fn set_capn(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "desc CAPP."]
        #[must_use]
        #[inline(always)]
        pub const fn capp(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "desc CAPP."]
        #[inline(always)]
        pub const fn set_capp(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "desc ECOM."]
        #[must_use]
        #[inline(always)]
        pub const fn ecom(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "desc ECOM."]
        #[inline(always)]
        pub const fn set_ecom(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
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
    impl Default for Ccapm1 {
        #[inline(always)]
        fn default() -> Ccapm1 {
            Ccapm1(0)
        }
    }
    impl core::fmt::Debug for Ccapm1 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Ccapm1")
                .field("ccie", &self.ccie())
                .field("pwm", &self.pwm())
                .field("tog", &self.tog())
                .field("mat", &self.mat())
                .field("capn", &self.capn())
                .field("capp", &self.capp())
                .field("ecom", &self.ecom())
                .field("rsv", &self.rsv())
                .finish()
        }
    }
    #[doc = "desc CCAPM2."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ccapm2(pub u32);
    impl Ccapm2 {
        #[doc = "desc CCIE."]
        #[must_use]
        #[inline(always)]
        pub const fn ccie(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "desc CCIE."]
        #[inline(always)]
        pub const fn set_ccie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "desc PWM."]
        #[must_use]
        #[inline(always)]
        pub const fn pwm(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "desc PWM."]
        #[inline(always)]
        pub const fn set_pwm(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "desc TOG."]
        #[must_use]
        #[inline(always)]
        pub const fn tog(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "desc TOG."]
        #[inline(always)]
        pub const fn set_tog(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "desc MAT."]
        #[must_use]
        #[inline(always)]
        pub const fn mat(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "desc MAT."]
        #[inline(always)]
        pub const fn set_mat(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "desc CAPN."]
        #[must_use]
        #[inline(always)]
        pub const fn capn(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "desc CAPN."]
        #[inline(always)]
        pub const fn set_capn(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "desc CAPP."]
        #[must_use]
        #[inline(always)]
        pub const fn capp(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "desc CAPP."]
        #[inline(always)]
        pub const fn set_capp(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "desc ECOM."]
        #[must_use]
        #[inline(always)]
        pub const fn ecom(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "desc ECOM."]
        #[inline(always)]
        pub const fn set_ecom(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
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
    impl Default for Ccapm2 {
        #[inline(always)]
        fn default() -> Ccapm2 {
            Ccapm2(0)
        }
    }
    impl core::fmt::Debug for Ccapm2 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Ccapm2")
                .field("ccie", &self.ccie())
                .field("pwm", &self.pwm())
                .field("tog", &self.tog())
                .field("mat", &self.mat())
                .field("capn", &self.capn())
                .field("capp", &self.capp())
                .field("ecom", &self.ecom())
                .field("rsv", &self.rsv())
                .finish()
        }
    }
    #[doc = "desc CCAPM3."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ccapm3(pub u32);
    impl Ccapm3 {
        #[doc = "desc CCIE."]
        #[must_use]
        #[inline(always)]
        pub const fn ccie(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "desc CCIE."]
        #[inline(always)]
        pub const fn set_ccie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "desc PWM."]
        #[must_use]
        #[inline(always)]
        pub const fn pwm(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "desc PWM."]
        #[inline(always)]
        pub const fn set_pwm(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "desc TOG."]
        #[must_use]
        #[inline(always)]
        pub const fn tog(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "desc TOG."]
        #[inline(always)]
        pub const fn set_tog(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "desc MAT."]
        #[must_use]
        #[inline(always)]
        pub const fn mat(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "desc MAT."]
        #[inline(always)]
        pub const fn set_mat(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "desc CAPN."]
        #[must_use]
        #[inline(always)]
        pub const fn capn(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "desc CAPN."]
        #[inline(always)]
        pub const fn set_capn(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "desc CAPP."]
        #[must_use]
        #[inline(always)]
        pub const fn capp(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "desc CAPP."]
        #[inline(always)]
        pub const fn set_capp(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "desc ECOM."]
        #[must_use]
        #[inline(always)]
        pub const fn ecom(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "desc ECOM."]
        #[inline(always)]
        pub const fn set_ecom(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
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
    impl Default for Ccapm3 {
        #[inline(always)]
        fn default() -> Ccapm3 {
            Ccapm3(0)
        }
    }
    impl core::fmt::Debug for Ccapm3 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Ccapm3")
                .field("ccie", &self.ccie())
                .field("pwm", &self.pwm())
                .field("tog", &self.tog())
                .field("mat", &self.mat())
                .field("capn", &self.capn())
                .field("capp", &self.capp())
                .field("ecom", &self.ecom())
                .field("rsv", &self.rsv())
                .finish()
        }
    }
    #[doc = "desc CCAPM4."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ccapm4(pub u32);
    impl Ccapm4 {
        #[doc = "desc CCIE."]
        #[must_use]
        #[inline(always)]
        pub const fn ccie(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "desc CCIE."]
        #[inline(always)]
        pub const fn set_ccie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "desc PWM."]
        #[must_use]
        #[inline(always)]
        pub const fn pwm(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "desc PWM."]
        #[inline(always)]
        pub const fn set_pwm(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "desc TOG."]
        #[must_use]
        #[inline(always)]
        pub const fn tog(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "desc TOG."]
        #[inline(always)]
        pub const fn set_tog(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "desc MAT."]
        #[must_use]
        #[inline(always)]
        pub const fn mat(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "desc MAT."]
        #[inline(always)]
        pub const fn set_mat(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "desc CAPN."]
        #[must_use]
        #[inline(always)]
        pub const fn capn(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "desc CAPN."]
        #[inline(always)]
        pub const fn set_capn(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "desc CAPP."]
        #[must_use]
        #[inline(always)]
        pub const fn capp(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "desc CAPP."]
        #[inline(always)]
        pub const fn set_capp(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "desc ECOM."]
        #[must_use]
        #[inline(always)]
        pub const fn ecom(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "desc ECOM."]
        #[inline(always)]
        pub const fn set_ecom(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
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
    impl Default for Ccapm4 {
        #[inline(always)]
        fn default() -> Ccapm4 {
            Ccapm4(0)
        }
    }
    impl core::fmt::Debug for Ccapm4 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Ccapm4")
                .field("ccie", &self.ccie())
                .field("pwm", &self.pwm())
                .field("tog", &self.tog())
                .field("mat", &self.mat())
                .field("capn", &self.capn())
                .field("capp", &self.capp())
                .field("ecom", &self.ecom())
                .field("rsv", &self.rsv())
                .finish()
        }
    }
    #[doc = "desc CCAPO."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ccapo(pub u32);
    impl Ccapo {
        #[doc = "desc CCAPO0."]
        #[must_use]
        #[inline(always)]
        pub const fn ccapo0(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "desc CCAPO0."]
        #[inline(always)]
        pub const fn set_ccapo0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "desc CCAPO1."]
        #[must_use]
        #[inline(always)]
        pub const fn ccapo1(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "desc CCAPO1."]
        #[inline(always)]
        pub const fn set_ccapo1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "desc CCAPO2."]
        #[must_use]
        #[inline(always)]
        pub const fn ccapo2(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "desc CCAPO2."]
        #[inline(always)]
        pub const fn set_ccapo2(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "desc CCAPO3."]
        #[must_use]
        #[inline(always)]
        pub const fn ccapo3(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "desc CCAPO3."]
        #[inline(always)]
        pub const fn set_ccapo3(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "desc CCAPO4."]
        #[must_use]
        #[inline(always)]
        pub const fn ccapo4(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "desc CCAPO4."]
        #[inline(always)]
        pub const fn set_ccapo4(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
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
    impl Default for Ccapo {
        #[inline(always)]
        fn default() -> Ccapo {
            Ccapo(0)
        }
    }
    impl core::fmt::Debug for Ccapo {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Ccapo")
                .field("ccapo0", &self.ccapo0())
                .field("ccapo1", &self.ccapo1())
                .field("ccapo2", &self.ccapo2())
                .field("ccapo3", &self.ccapo3())
                .field("ccapo4", &self.ccapo4())
                .field("rsv", &self.rsv())
                .finish()
        }
    }
    #[doc = "desc CCON."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ccon(pub u32);
    impl Ccon {
        #[doc = "desc CCF0."]
        #[must_use]
        #[inline(always)]
        pub const fn ccf0(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "desc CCF0."]
        #[inline(always)]
        pub const fn set_ccf0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "desc CCF1."]
        #[must_use]
        #[inline(always)]
        pub const fn ccf1(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "desc CCF1."]
        #[inline(always)]
        pub const fn set_ccf1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "desc CCF2."]
        #[must_use]
        #[inline(always)]
        pub const fn ccf2(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "desc CCF2."]
        #[inline(always)]
        pub const fn set_ccf2(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "desc CCF3."]
        #[must_use]
        #[inline(always)]
        pub const fn ccf3(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "desc CCF3."]
        #[inline(always)]
        pub const fn set_ccf3(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "desc CCF4."]
        #[must_use]
        #[inline(always)]
        pub const fn ccf4(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "desc CCF4."]
        #[inline(always)]
        pub const fn set_ccf4(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "desc CR."]
        #[must_use]
        #[inline(always)]
        pub const fn cr(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "desc CR."]
        #[inline(always)]
        pub const fn set_cr(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "desc CF."]
        #[must_use]
        #[inline(always)]
        pub const fn cf(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "desc CF."]
        #[inline(always)]
        pub const fn set_cf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
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
    impl Default for Ccon {
        #[inline(always)]
        fn default() -> Ccon {
            Ccon(0)
        }
    }
    impl core::fmt::Debug for Ccon {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Ccon")
                .field("ccf0", &self.ccf0())
                .field("ccf1", &self.ccf1())
                .field("ccf2", &self.ccf2())
                .field("ccf3", &self.ccf3())
                .field("ccf4", &self.ccf4())
                .field("cr", &self.cr())
                .field("cf", &self.cf())
                .field("rsv", &self.rsv())
                .finish()
        }
    }
    #[doc = "desc CMOD."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Cmod(pub u32);
    impl Cmod {
        #[doc = "desc CFIE."]
        #[must_use]
        #[inline(always)]
        pub const fn cfie(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "desc CFIE."]
        #[inline(always)]
        pub const fn set_cfie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "desc CPS."]
        #[must_use]
        #[inline(always)]
        pub const fn cps(&self) -> u8 {
            let val = (self.0 >> 1usize) & 0x07;
            val as u8
        }
        #[doc = "desc CPS."]
        #[inline(always)]
        pub const fn set_cps(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 1usize)) | (((val as u32) & 0x07) << 1usize);
        }
        #[doc = "desc WDTE."]
        #[must_use]
        #[inline(always)]
        pub const fn wdte(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "desc WDTE."]
        #[inline(always)]
        pub const fn set_wdte(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "desc CIDL."]
        #[must_use]
        #[inline(always)]
        pub const fn cidl(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "desc CIDL."]
        #[inline(always)]
        pub const fn set_cidl(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
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
    impl Default for Cmod {
        #[inline(always)]
        fn default() -> Cmod {
            Cmod(0)
        }
    }
    impl core::fmt::Debug for Cmod {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Cmod")
                .field("cfie", &self.cfie())
                .field("cps", &self.cps())
                .field("wdte", &self.wdte())
                .field("cidl", &self.cidl())
                .field("rsv", &self.rsv())
                .finish()
        }
    }
    #[doc = "desc CNT."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Cnt(pub u32);
    impl Cnt {
        #[doc = "desc CNT."]
        #[must_use]
        #[inline(always)]
        pub const fn cnt(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[doc = "desc CNT."]
        #[inline(always)]
        pub const fn set_cnt(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
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
    impl Default for Cnt {
        #[inline(always)]
        fn default() -> Cnt {
            Cnt(0)
        }
    }
    impl core::fmt::Debug for Cnt {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Cnt")
                .field("cnt", &self.cnt())
                .field("rsv", &self.rsv())
                .finish()
        }
    }
    #[doc = "desc ICLR."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Iclr(pub u32);
    impl Iclr {
        #[doc = "desc CCF0."]
        #[must_use]
        #[inline(always)]
        pub const fn ccf0(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "desc CCF0."]
        #[inline(always)]
        pub const fn set_ccf0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "desc CCF1."]
        #[must_use]
        #[inline(always)]
        pub const fn ccf1(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "desc CCF1."]
        #[inline(always)]
        pub const fn set_ccf1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "desc CCF2."]
        #[must_use]
        #[inline(always)]
        pub const fn ccf2(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "desc CCF2."]
        #[inline(always)]
        pub const fn set_ccf2(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "desc CCF3."]
        #[must_use]
        #[inline(always)]
        pub const fn ccf3(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "desc CCF3."]
        #[inline(always)]
        pub const fn set_ccf3(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "desc CCF4."]
        #[must_use]
        #[inline(always)]
        pub const fn ccf4(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "desc CCF4."]
        #[inline(always)]
        pub const fn set_ccf4(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "desc CF."]
        #[must_use]
        #[inline(always)]
        pub const fn cf(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "desc CF."]
        #[inline(always)]
        pub const fn set_cf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
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
    impl Default for Iclr {
        #[inline(always)]
        fn default() -> Iclr {
            Iclr(0)
        }
    }
    impl core::fmt::Debug for Iclr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Iclr")
                .field("ccf0", &self.ccf0())
                .field("ccf1", &self.ccf1())
                .field("ccf2", &self.ccf2())
                .field("ccf3", &self.ccf3())
                .field("ccf4", &self.ccf4())
                .field("cf", &self.cf())
                .field("rsv", &self.rsv())
                .finish()
        }
    }
}
