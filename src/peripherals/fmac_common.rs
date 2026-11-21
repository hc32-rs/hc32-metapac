#![allow(clippy::missing_safety_doc)]
#![allow(clippy::identity_op)]
#![allow(clippy::unnecessary_cast)]
#![allow(clippy::erasing_op)]

#[doc = "desc FMAC."]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Fmac {
    ptr: *mut u8,
}
unsafe impl Send for Fmac {}
unsafe impl Sync for Fmac {}
impl Fmac {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "desc ENR."]
    #[inline(always)]
    pub const fn enr(self) -> crate::common::Reg<regs::Enr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0usize) as _) }
    }
    #[doc = "desc CTR."]
    #[inline(always)]
    pub const fn ctr(self) -> crate::common::Reg<regs::Ctr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x04usize) as _) }
    }
    #[doc = "desc IER."]
    #[inline(always)]
    pub const fn ier(self) -> crate::common::Reg<regs::Ier, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x08usize) as _) }
    }
    #[doc = "desc DTR."]
    #[inline(always)]
    pub const fn dtr(self) -> crate::common::Reg<regs::Dtr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0cusize) as _) }
    }
    #[doc = "desc RTR0."]
    #[inline(always)]
    pub const fn rtr0(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x10usize) as _) }
    }
    #[doc = "desc RTR1."]
    #[inline(always)]
    pub const fn rtr1(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x14usize) as _) }
    }
    #[doc = "desc STR."]
    #[inline(always)]
    pub const fn str(self) -> crate::common::Reg<regs::Str, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x18usize) as _) }
    }
    #[doc = "desc COR0."]
    #[inline(always)]
    pub const fn cor0(self) -> crate::common::Reg<regs::Cor0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x20usize) as _) }
    }
    #[doc = "desc COR1."]
    #[inline(always)]
    pub const fn cor1(self) -> crate::common::Reg<regs::Cor1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x24usize) as _) }
    }
    #[doc = "desc COR2."]
    #[inline(always)]
    pub const fn cor2(self) -> crate::common::Reg<regs::Cor2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x28usize) as _) }
    }
    #[doc = "desc COR3."]
    #[inline(always)]
    pub const fn cor3(self) -> crate::common::Reg<regs::Cor3, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x2cusize) as _) }
    }
    #[doc = "desc COR4."]
    #[inline(always)]
    pub const fn cor4(self) -> crate::common::Reg<regs::Cor4, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x30usize) as _) }
    }
    #[doc = "desc COR5."]
    #[inline(always)]
    pub const fn cor5(self) -> crate::common::Reg<regs::Cor5, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x34usize) as _) }
    }
    #[doc = "desc COR6."]
    #[inline(always)]
    pub const fn cor6(self) -> crate::common::Reg<regs::Cor6, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x38usize) as _) }
    }
    #[doc = "desc COR7."]
    #[inline(always)]
    pub const fn cor7(self) -> crate::common::Reg<regs::Cor7, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x3cusize) as _) }
    }
    #[doc = "desc COR8."]
    #[inline(always)]
    pub const fn cor8(self) -> crate::common::Reg<regs::Cor8, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x40usize) as _) }
    }
    #[doc = "desc COR9."]
    #[inline(always)]
    pub const fn cor9(self) -> crate::common::Reg<regs::Cor9, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x44usize) as _) }
    }
    #[doc = "desc COR10."]
    #[inline(always)]
    pub const fn cor10(self) -> crate::common::Reg<regs::Cor10, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x48usize) as _) }
    }
    #[doc = "desc COR11."]
    #[inline(always)]
    pub const fn cor11(self) -> crate::common::Reg<regs::Cor11, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x4cusize) as _) }
    }
    #[doc = "desc COR12."]
    #[inline(always)]
    pub const fn cor12(self) -> crate::common::Reg<regs::Cor12, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x50usize) as _) }
    }
    #[doc = "desc COR13."]
    #[inline(always)]
    pub const fn cor13(self) -> crate::common::Reg<regs::Cor13, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x54usize) as _) }
    }
    #[doc = "desc COR14."]
    #[inline(always)]
    pub const fn cor14(self) -> crate::common::Reg<regs::Cor14, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x58usize) as _) }
    }
    #[doc = "desc COR15."]
    #[inline(always)]
    pub const fn cor15(self) -> crate::common::Reg<regs::Cor15, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x5cusize) as _) }
    }
    #[doc = "desc COR16."]
    #[inline(always)]
    pub const fn cor16(self) -> crate::common::Reg<regs::Cor16, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x60usize) as _) }
    }
}
pub mod regs {
    #[doc = "desc COR0."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Cor0(pub u32);
    impl Cor0 {
        #[doc = "desc CIN."]
        #[must_use]
        #[inline(always)]
        pub const fn cin(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[doc = "desc CIN."]
        #[inline(always)]
        pub const fn set_cin(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
        }
    }
    impl Default for Cor0 {
        #[inline(always)]
        fn default() -> Cor0 {
            Cor0(0)
        }
    }
    impl core::fmt::Debug for Cor0 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Cor0").field("cin", &self.cin()).finish()
        }
    }
    #[doc = "desc COR1."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Cor1(pub u32);
    impl Cor1 {
        #[doc = "desc CIN."]
        #[must_use]
        #[inline(always)]
        pub const fn cin(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[doc = "desc CIN."]
        #[inline(always)]
        pub const fn set_cin(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
        }
    }
    impl Default for Cor1 {
        #[inline(always)]
        fn default() -> Cor1 {
            Cor1(0)
        }
    }
    impl core::fmt::Debug for Cor1 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Cor1").field("cin", &self.cin()).finish()
        }
    }
    #[doc = "desc COR10."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Cor10(pub u32);
    impl Cor10 {
        #[doc = "desc CIN."]
        #[must_use]
        #[inline(always)]
        pub const fn cin(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[doc = "desc CIN."]
        #[inline(always)]
        pub const fn set_cin(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
        }
    }
    impl Default for Cor10 {
        #[inline(always)]
        fn default() -> Cor10 {
            Cor10(0)
        }
    }
    impl core::fmt::Debug for Cor10 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Cor10").field("cin", &self.cin()).finish()
        }
    }
    #[doc = "desc COR11."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Cor11(pub u32);
    impl Cor11 {
        #[doc = "desc CIN."]
        #[must_use]
        #[inline(always)]
        pub const fn cin(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[doc = "desc CIN."]
        #[inline(always)]
        pub const fn set_cin(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
        }
    }
    impl Default for Cor11 {
        #[inline(always)]
        fn default() -> Cor11 {
            Cor11(0)
        }
    }
    impl core::fmt::Debug for Cor11 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Cor11").field("cin", &self.cin()).finish()
        }
    }
    #[doc = "desc COR12."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Cor12(pub u32);
    impl Cor12 {
        #[doc = "desc CIN."]
        #[must_use]
        #[inline(always)]
        pub const fn cin(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[doc = "desc CIN."]
        #[inline(always)]
        pub const fn set_cin(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
        }
    }
    impl Default for Cor12 {
        #[inline(always)]
        fn default() -> Cor12 {
            Cor12(0)
        }
    }
    impl core::fmt::Debug for Cor12 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Cor12").field("cin", &self.cin()).finish()
        }
    }
    #[doc = "desc COR13."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Cor13(pub u32);
    impl Cor13 {
        #[doc = "desc CIN."]
        #[must_use]
        #[inline(always)]
        pub const fn cin(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[doc = "desc CIN."]
        #[inline(always)]
        pub const fn set_cin(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
        }
    }
    impl Default for Cor13 {
        #[inline(always)]
        fn default() -> Cor13 {
            Cor13(0)
        }
    }
    impl core::fmt::Debug for Cor13 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Cor13").field("cin", &self.cin()).finish()
        }
    }
    #[doc = "desc COR14."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Cor14(pub u32);
    impl Cor14 {
        #[doc = "desc CIN."]
        #[must_use]
        #[inline(always)]
        pub const fn cin(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[doc = "desc CIN."]
        #[inline(always)]
        pub const fn set_cin(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
        }
    }
    impl Default for Cor14 {
        #[inline(always)]
        fn default() -> Cor14 {
            Cor14(0)
        }
    }
    impl core::fmt::Debug for Cor14 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Cor14").field("cin", &self.cin()).finish()
        }
    }
    #[doc = "desc COR15."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Cor15(pub u32);
    impl Cor15 {
        #[doc = "desc CIN."]
        #[must_use]
        #[inline(always)]
        pub const fn cin(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[doc = "desc CIN."]
        #[inline(always)]
        pub const fn set_cin(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
        }
    }
    impl Default for Cor15 {
        #[inline(always)]
        fn default() -> Cor15 {
            Cor15(0)
        }
    }
    impl core::fmt::Debug for Cor15 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Cor15").field("cin", &self.cin()).finish()
        }
    }
    #[doc = "desc COR16."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Cor16(pub u32);
    impl Cor16 {
        #[doc = "desc CIN."]
        #[must_use]
        #[inline(always)]
        pub const fn cin(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[doc = "desc CIN."]
        #[inline(always)]
        pub const fn set_cin(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
        }
    }
    impl Default for Cor16 {
        #[inline(always)]
        fn default() -> Cor16 {
            Cor16(0)
        }
    }
    impl core::fmt::Debug for Cor16 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Cor16").field("cin", &self.cin()).finish()
        }
    }
    #[doc = "desc COR2."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Cor2(pub u32);
    impl Cor2 {
        #[doc = "desc CIN."]
        #[must_use]
        #[inline(always)]
        pub const fn cin(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[doc = "desc CIN."]
        #[inline(always)]
        pub const fn set_cin(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
        }
    }
    impl Default for Cor2 {
        #[inline(always)]
        fn default() -> Cor2 {
            Cor2(0)
        }
    }
    impl core::fmt::Debug for Cor2 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Cor2").field("cin", &self.cin()).finish()
        }
    }
    #[doc = "desc COR3."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Cor3(pub u32);
    impl Cor3 {
        #[doc = "desc CIN."]
        #[must_use]
        #[inline(always)]
        pub const fn cin(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[doc = "desc CIN."]
        #[inline(always)]
        pub const fn set_cin(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
        }
    }
    impl Default for Cor3 {
        #[inline(always)]
        fn default() -> Cor3 {
            Cor3(0)
        }
    }
    impl core::fmt::Debug for Cor3 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Cor3").field("cin", &self.cin()).finish()
        }
    }
    #[doc = "desc COR4."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Cor4(pub u32);
    impl Cor4 {
        #[doc = "desc CIN."]
        #[must_use]
        #[inline(always)]
        pub const fn cin(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[doc = "desc CIN."]
        #[inline(always)]
        pub const fn set_cin(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
        }
    }
    impl Default for Cor4 {
        #[inline(always)]
        fn default() -> Cor4 {
            Cor4(0)
        }
    }
    impl core::fmt::Debug for Cor4 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Cor4").field("cin", &self.cin()).finish()
        }
    }
    #[doc = "desc COR5."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Cor5(pub u32);
    impl Cor5 {
        #[doc = "desc CIN."]
        #[must_use]
        #[inline(always)]
        pub const fn cin(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[doc = "desc CIN."]
        #[inline(always)]
        pub const fn set_cin(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
        }
    }
    impl Default for Cor5 {
        #[inline(always)]
        fn default() -> Cor5 {
            Cor5(0)
        }
    }
    impl core::fmt::Debug for Cor5 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Cor5").field("cin", &self.cin()).finish()
        }
    }
    #[doc = "desc COR6."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Cor6(pub u32);
    impl Cor6 {
        #[doc = "desc CIN."]
        #[must_use]
        #[inline(always)]
        pub const fn cin(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[doc = "desc CIN."]
        #[inline(always)]
        pub const fn set_cin(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
        }
    }
    impl Default for Cor6 {
        #[inline(always)]
        fn default() -> Cor6 {
            Cor6(0)
        }
    }
    impl core::fmt::Debug for Cor6 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Cor6").field("cin", &self.cin()).finish()
        }
    }
    #[doc = "desc COR7."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Cor7(pub u32);
    impl Cor7 {
        #[doc = "desc CIN."]
        #[must_use]
        #[inline(always)]
        pub const fn cin(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[doc = "desc CIN."]
        #[inline(always)]
        pub const fn set_cin(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
        }
    }
    impl Default for Cor7 {
        #[inline(always)]
        fn default() -> Cor7 {
            Cor7(0)
        }
    }
    impl core::fmt::Debug for Cor7 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Cor7").field("cin", &self.cin()).finish()
        }
    }
    #[doc = "desc COR8."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Cor8(pub u32);
    impl Cor8 {
        #[doc = "desc CIN."]
        #[must_use]
        #[inline(always)]
        pub const fn cin(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[doc = "desc CIN."]
        #[inline(always)]
        pub const fn set_cin(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
        }
    }
    impl Default for Cor8 {
        #[inline(always)]
        fn default() -> Cor8 {
            Cor8(0)
        }
    }
    impl core::fmt::Debug for Cor8 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Cor8").field("cin", &self.cin()).finish()
        }
    }
    #[doc = "desc COR9."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Cor9(pub u32);
    impl Cor9 {
        #[doc = "desc CIN."]
        #[must_use]
        #[inline(always)]
        pub const fn cin(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[doc = "desc CIN."]
        #[inline(always)]
        pub const fn set_cin(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
        }
    }
    impl Default for Cor9 {
        #[inline(always)]
        fn default() -> Cor9 {
            Cor9(0)
        }
    }
    impl core::fmt::Debug for Cor9 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Cor9").field("cin", &self.cin()).finish()
        }
    }
    #[doc = "desc CTR."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ctr(pub u32);
    impl Ctr {
        #[doc = "desc STAGE_NUM."]
        #[must_use]
        #[inline(always)]
        pub const fn stage_num(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x1f;
            val as u8
        }
        #[doc = "desc STAGE_NUM."]
        #[inline(always)]
        pub const fn set_stage_num(&mut self, val: u8) {
            self.0 = (self.0 & !(0x1f << 0usize)) | (((val as u32) & 0x1f) << 0usize);
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
    impl Default for Ctr {
        #[inline(always)]
        fn default() -> Ctr {
            Ctr(0)
        }
    }
    impl core::fmt::Debug for Ctr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Ctr")
                .field("stage_num", &self.stage_num())
                .field("shift", &self.shift())
                .finish()
        }
    }
    #[doc = "desc DTR."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Dtr(pub u32);
    impl Dtr {
        #[doc = "desc DIN."]
        #[must_use]
        #[inline(always)]
        pub const fn din(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[doc = "desc DIN."]
        #[inline(always)]
        pub const fn set_din(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
        }
    }
    impl Default for Dtr {
        #[inline(always)]
        fn default() -> Dtr {
            Dtr(0)
        }
    }
    impl core::fmt::Debug for Dtr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Dtr").field("din", &self.din()).finish()
        }
    }
    #[doc = "desc ENR."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Enr(pub u32);
    impl Enr {
        #[doc = "desc FMACEN."]
        #[must_use]
        #[inline(always)]
        pub const fn fmacen(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "desc FMACEN."]
        #[inline(always)]
        pub const fn set_fmacen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
    }
    impl Default for Enr {
        #[inline(always)]
        fn default() -> Enr {
            Enr(0)
        }
    }
    impl core::fmt::Debug for Enr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Enr")
                .field("fmacen", &self.fmacen())
                .finish()
        }
    }
    #[doc = "desc IER."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ier(pub u32);
    impl Ier {
        #[doc = "desc INTEN."]
        #[must_use]
        #[inline(always)]
        pub const fn inten(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "desc INTEN."]
        #[inline(always)]
        pub const fn set_inten(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
    }
    impl Default for Ier {
        #[inline(always)]
        fn default() -> Ier {
            Ier(0)
        }
    }
    impl core::fmt::Debug for Ier {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Ier").field("inten", &self.inten()).finish()
        }
    }
    #[doc = "desc STR."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Str(pub u32);
    impl Str {
        #[doc = "desc READY."]
        #[must_use]
        #[inline(always)]
        pub const fn ready(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[doc = "desc READY."]
        #[inline(always)]
        pub const fn set_ready(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for Str {
        #[inline(always)]
        fn default() -> Str {
            Str(0)
        }
    }
    impl core::fmt::Debug for Str {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Str").field("ready", &self.ready()).finish()
        }
    }
}
