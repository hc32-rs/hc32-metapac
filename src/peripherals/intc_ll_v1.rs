#![allow(clippy::missing_safety_doc)]
#![allow(clippy::identity_op)]
#![allow(clippy::unnecessary_cast)]
#![allow(clippy::erasing_op)]

#[doc = "desc INTC."]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Intc {
    ptr: *mut u8,
}
unsafe impl Send for Intc {}
unsafe impl Sync for Intc {}
impl Intc {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "desc NMIER."]
    #[inline(always)]
    pub const fn nmier(self) -> crate::common::Reg<regs::Nmier, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x04usize) as _) }
    }
    #[doc = "desc NMIFR."]
    #[inline(always)]
    pub const fn nmifr(self) -> crate::common::Reg<regs::Nmifr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x08usize) as _) }
    }
    #[doc = "desc NMIFCR."]
    #[inline(always)]
    pub const fn nmifcr(self) -> crate::common::Reg<regs::Nmifcr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0cusize) as _) }
    }
    #[doc = "desc EIRQCR0."]
    #[inline(always)]
    pub const fn eirqcr0(self) -> crate::common::Reg<regs::Eirqcr0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x10usize) as _) }
    }
    #[doc = "desc EIRQCR1."]
    #[inline(always)]
    pub const fn eirqcr1(self) -> crate::common::Reg<regs::Eirqcr1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x14usize) as _) }
    }
    #[doc = "desc EIRQCR2."]
    #[inline(always)]
    pub const fn eirqcr2(self) -> crate::common::Reg<regs::Eirqcr2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x18usize) as _) }
    }
    #[doc = "desc EIRQCR3."]
    #[inline(always)]
    pub const fn eirqcr3(self) -> crate::common::Reg<regs::Eirqcr3, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1cusize) as _) }
    }
    #[doc = "desc EIRQCR4."]
    #[inline(always)]
    pub const fn eirqcr4(self) -> crate::common::Reg<regs::Eirqcr4, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x20usize) as _) }
    }
    #[doc = "desc EIRQCR5."]
    #[inline(always)]
    pub const fn eirqcr5(self) -> crate::common::Reg<regs::Eirqcr5, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x24usize) as _) }
    }
    #[doc = "desc EIRQCR6."]
    #[inline(always)]
    pub const fn eirqcr6(self) -> crate::common::Reg<regs::Eirqcr6, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x28usize) as _) }
    }
    #[doc = "desc EIRQCR7."]
    #[inline(always)]
    pub const fn eirqcr7(self) -> crate::common::Reg<regs::Eirqcr7, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x2cusize) as _) }
    }
    #[doc = "desc EIFR."]
    #[inline(always)]
    pub const fn eifr(self) -> crate::common::Reg<regs::Eifr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x50usize) as _) }
    }
    #[doc = "desc EIFCR."]
    #[inline(always)]
    pub const fn eifcr(self) -> crate::common::Reg<regs::Eifcr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x54usize) as _) }
    }
    #[doc = "desc EVTER."]
    #[inline(always)]
    pub const fn evter(self) -> crate::common::Reg<regs::Evter, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x58usize) as _) }
    }
    #[doc = "desc WKEN."]
    #[inline(always)]
    pub const fn wken(self) -> crate::common::Reg<regs::Wken, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x5cusize) as _) }
    }
    #[doc = "desc SWIER."]
    #[inline(always)]
    pub const fn swier(self) -> crate::common::Reg<regs::Swier, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x60usize) as _) }
    }
    #[doc = "desc IER."]
    #[inline(always)]
    pub const fn ier(self) -> crate::common::Reg<regs::Ier, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x64usize) as _) }
    }
    #[doc = "desc FPUIER."]
    #[inline(always)]
    pub const fn fpuier(self) -> crate::common::Reg<regs::Fpuier, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x68usize) as _) }
    }
}
pub mod regs {
    #[doc = "desc EIFCR."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Eifcr(pub u32);
    impl Eifcr {
        #[doc = "desc EIFCLR0."]
        #[must_use]
        #[inline(always)]
        pub const fn eifclr0(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "desc EIFCLR0."]
        #[inline(always)]
        pub const fn set_eifclr0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "desc EIFCLR1."]
        #[must_use]
        #[inline(always)]
        pub const fn eifclr1(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "desc EIFCLR1."]
        #[inline(always)]
        pub const fn set_eifclr1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "desc EIFCLR2."]
        #[must_use]
        #[inline(always)]
        pub const fn eifclr2(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "desc EIFCLR2."]
        #[inline(always)]
        pub const fn set_eifclr2(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "desc EIFCLR3."]
        #[must_use]
        #[inline(always)]
        pub const fn eifclr3(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "desc EIFCLR3."]
        #[inline(always)]
        pub const fn set_eifclr3(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "desc EIFCLR4."]
        #[must_use]
        #[inline(always)]
        pub const fn eifclr4(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "desc EIFCLR4."]
        #[inline(always)]
        pub const fn set_eifclr4(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "desc EIFCLR5."]
        #[must_use]
        #[inline(always)]
        pub const fn eifclr5(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "desc EIFCLR5."]
        #[inline(always)]
        pub const fn set_eifclr5(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "desc EIFCLR6."]
        #[must_use]
        #[inline(always)]
        pub const fn eifclr6(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "desc EIFCLR6."]
        #[inline(always)]
        pub const fn set_eifclr6(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "desc EIFCLR7."]
        #[must_use]
        #[inline(always)]
        pub const fn eifclr7(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "desc EIFCLR7."]
        #[inline(always)]
        pub const fn set_eifclr7(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
    }
    impl Default for Eifcr {
        #[inline(always)]
        fn default() -> Eifcr {
            Eifcr(0)
        }
    }
    impl core::fmt::Debug for Eifcr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Eifcr")
                .field("eifclr0", &self.eifclr0())
                .field("eifclr1", &self.eifclr1())
                .field("eifclr2", &self.eifclr2())
                .field("eifclr3", &self.eifclr3())
                .field("eifclr4", &self.eifclr4())
                .field("eifclr5", &self.eifclr5())
                .field("eifclr6", &self.eifclr6())
                .field("eifclr7", &self.eifclr7())
                .finish()
        }
    }
    #[doc = "desc EIFR."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Eifr(pub u32);
    impl Eifr {
        #[doc = "desc EIF0."]
        #[must_use]
        #[inline(always)]
        pub const fn eif0(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "desc EIF0."]
        #[inline(always)]
        pub const fn set_eif0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "desc EIF1."]
        #[must_use]
        #[inline(always)]
        pub const fn eif1(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "desc EIF1."]
        #[inline(always)]
        pub const fn set_eif1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "desc EIF2."]
        #[must_use]
        #[inline(always)]
        pub const fn eif2(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "desc EIF2."]
        #[inline(always)]
        pub const fn set_eif2(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "desc EIF3."]
        #[must_use]
        #[inline(always)]
        pub const fn eif3(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "desc EIF3."]
        #[inline(always)]
        pub const fn set_eif3(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "desc EIF4."]
        #[must_use]
        #[inline(always)]
        pub const fn eif4(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "desc EIF4."]
        #[inline(always)]
        pub const fn set_eif4(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "desc EIF5."]
        #[must_use]
        #[inline(always)]
        pub const fn eif5(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "desc EIF5."]
        #[inline(always)]
        pub const fn set_eif5(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "desc EIF6."]
        #[must_use]
        #[inline(always)]
        pub const fn eif6(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "desc EIF6."]
        #[inline(always)]
        pub const fn set_eif6(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "desc EIF7."]
        #[must_use]
        #[inline(always)]
        pub const fn eif7(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "desc EIF7."]
        #[inline(always)]
        pub const fn set_eif7(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
    }
    impl Default for Eifr {
        #[inline(always)]
        fn default() -> Eifr {
            Eifr(0)
        }
    }
    impl core::fmt::Debug for Eifr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Eifr")
                .field("eif0", &self.eif0())
                .field("eif1", &self.eif1())
                .field("eif2", &self.eif2())
                .field("eif3", &self.eif3())
                .field("eif4", &self.eif4())
                .field("eif5", &self.eif5())
                .field("eif6", &self.eif6())
                .field("eif7", &self.eif7())
                .finish()
        }
    }
    #[doc = "desc EIRQCR0."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Eirqcr0(pub u32);
    impl Eirqcr0 {
        #[doc = "desc EIRQTRG."]
        #[must_use]
        #[inline(always)]
        pub const fn eirqtrg(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x03;
            val as u8
        }
        #[doc = "desc EIRQTRG."]
        #[inline(always)]
        pub const fn set_eirqtrg(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 0usize)) | (((val as u32) & 0x03) << 0usize);
        }
        #[doc = "desc EISMPCLK."]
        #[must_use]
        #[inline(always)]
        pub const fn eismpclk(&self) -> u8 {
            let val = (self.0 >> 4usize) & 0x03;
            val as u8
        }
        #[doc = "desc EISMPCLK."]
        #[inline(always)]
        pub const fn set_eismpclk(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 4usize)) | (((val as u32) & 0x03) << 4usize);
        }
        #[doc = "desc EFEN."]
        #[must_use]
        #[inline(always)]
        pub const fn efen(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "desc EFEN."]
        #[inline(always)]
        pub const fn set_efen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "desc NOCSEL."]
        #[must_use]
        #[inline(always)]
        pub const fn nocsel(&self) -> u8 {
            let val = (self.0 >> 12usize) & 0x03;
            val as u8
        }
        #[doc = "desc NOCSEL."]
        #[inline(always)]
        pub const fn set_nocsel(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 12usize)) | (((val as u32) & 0x03) << 12usize);
        }
        #[doc = "desc NOCEN."]
        #[must_use]
        #[inline(always)]
        pub const fn nocen(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "desc NOCEN."]
        #[inline(always)]
        pub const fn set_nocen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
    }
    impl Default for Eirqcr0 {
        #[inline(always)]
        fn default() -> Eirqcr0 {
            Eirqcr0(0)
        }
    }
    impl core::fmt::Debug for Eirqcr0 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Eirqcr0")
                .field("eirqtrg", &self.eirqtrg())
                .field("eismpclk", &self.eismpclk())
                .field("efen", &self.efen())
                .field("nocsel", &self.nocsel())
                .field("nocen", &self.nocen())
                .finish()
        }
    }
    #[doc = "desc EIRQCR1."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Eirqcr1(pub u32);
    impl Eirqcr1 {
        #[doc = "desc EIRQTRG."]
        #[must_use]
        #[inline(always)]
        pub const fn eirqtrg(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x03;
            val as u8
        }
        #[doc = "desc EIRQTRG."]
        #[inline(always)]
        pub const fn set_eirqtrg(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 0usize)) | (((val as u32) & 0x03) << 0usize);
        }
        #[doc = "desc EISMPCLK."]
        #[must_use]
        #[inline(always)]
        pub const fn eismpclk(&self) -> u8 {
            let val = (self.0 >> 4usize) & 0x03;
            val as u8
        }
        #[doc = "desc EISMPCLK."]
        #[inline(always)]
        pub const fn set_eismpclk(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 4usize)) | (((val as u32) & 0x03) << 4usize);
        }
        #[doc = "desc EFEN."]
        #[must_use]
        #[inline(always)]
        pub const fn efen(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "desc EFEN."]
        #[inline(always)]
        pub const fn set_efen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "desc NOCSEL."]
        #[must_use]
        #[inline(always)]
        pub const fn nocsel(&self) -> u8 {
            let val = (self.0 >> 12usize) & 0x03;
            val as u8
        }
        #[doc = "desc NOCSEL."]
        #[inline(always)]
        pub const fn set_nocsel(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 12usize)) | (((val as u32) & 0x03) << 12usize);
        }
        #[doc = "desc NOCEN."]
        #[must_use]
        #[inline(always)]
        pub const fn nocen(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "desc NOCEN."]
        #[inline(always)]
        pub const fn set_nocen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
    }
    impl Default for Eirqcr1 {
        #[inline(always)]
        fn default() -> Eirqcr1 {
            Eirqcr1(0)
        }
    }
    impl core::fmt::Debug for Eirqcr1 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Eirqcr1")
                .field("eirqtrg", &self.eirqtrg())
                .field("eismpclk", &self.eismpclk())
                .field("efen", &self.efen())
                .field("nocsel", &self.nocsel())
                .field("nocen", &self.nocen())
                .finish()
        }
    }
    #[doc = "desc EIRQCR2."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Eirqcr2(pub u32);
    impl Eirqcr2 {
        #[doc = "desc EIRQTRG."]
        #[must_use]
        #[inline(always)]
        pub const fn eirqtrg(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x03;
            val as u8
        }
        #[doc = "desc EIRQTRG."]
        #[inline(always)]
        pub const fn set_eirqtrg(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 0usize)) | (((val as u32) & 0x03) << 0usize);
        }
        #[doc = "desc EISMPCLK."]
        #[must_use]
        #[inline(always)]
        pub const fn eismpclk(&self) -> u8 {
            let val = (self.0 >> 4usize) & 0x03;
            val as u8
        }
        #[doc = "desc EISMPCLK."]
        #[inline(always)]
        pub const fn set_eismpclk(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 4usize)) | (((val as u32) & 0x03) << 4usize);
        }
        #[doc = "desc EFEN."]
        #[must_use]
        #[inline(always)]
        pub const fn efen(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "desc EFEN."]
        #[inline(always)]
        pub const fn set_efen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "desc NOCSEL."]
        #[must_use]
        #[inline(always)]
        pub const fn nocsel(&self) -> u8 {
            let val = (self.0 >> 12usize) & 0x03;
            val as u8
        }
        #[doc = "desc NOCSEL."]
        #[inline(always)]
        pub const fn set_nocsel(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 12usize)) | (((val as u32) & 0x03) << 12usize);
        }
        #[doc = "desc NOCEN."]
        #[must_use]
        #[inline(always)]
        pub const fn nocen(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "desc NOCEN."]
        #[inline(always)]
        pub const fn set_nocen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
    }
    impl Default for Eirqcr2 {
        #[inline(always)]
        fn default() -> Eirqcr2 {
            Eirqcr2(0)
        }
    }
    impl core::fmt::Debug for Eirqcr2 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Eirqcr2")
                .field("eirqtrg", &self.eirqtrg())
                .field("eismpclk", &self.eismpclk())
                .field("efen", &self.efen())
                .field("nocsel", &self.nocsel())
                .field("nocen", &self.nocen())
                .finish()
        }
    }
    #[doc = "desc EIRQCR3."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Eirqcr3(pub u32);
    impl Eirqcr3 {
        #[doc = "desc EIRQTRG."]
        #[must_use]
        #[inline(always)]
        pub const fn eirqtrg(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x03;
            val as u8
        }
        #[doc = "desc EIRQTRG."]
        #[inline(always)]
        pub const fn set_eirqtrg(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 0usize)) | (((val as u32) & 0x03) << 0usize);
        }
        #[doc = "desc EISMPCLK."]
        #[must_use]
        #[inline(always)]
        pub const fn eismpclk(&self) -> u8 {
            let val = (self.0 >> 4usize) & 0x03;
            val as u8
        }
        #[doc = "desc EISMPCLK."]
        #[inline(always)]
        pub const fn set_eismpclk(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 4usize)) | (((val as u32) & 0x03) << 4usize);
        }
        #[doc = "desc EFEN."]
        #[must_use]
        #[inline(always)]
        pub const fn efen(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "desc EFEN."]
        #[inline(always)]
        pub const fn set_efen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "desc NOCSEL."]
        #[must_use]
        #[inline(always)]
        pub const fn nocsel(&self) -> u8 {
            let val = (self.0 >> 12usize) & 0x03;
            val as u8
        }
        #[doc = "desc NOCSEL."]
        #[inline(always)]
        pub const fn set_nocsel(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 12usize)) | (((val as u32) & 0x03) << 12usize);
        }
        #[doc = "desc NOCEN."]
        #[must_use]
        #[inline(always)]
        pub const fn nocen(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "desc NOCEN."]
        #[inline(always)]
        pub const fn set_nocen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
    }
    impl Default for Eirqcr3 {
        #[inline(always)]
        fn default() -> Eirqcr3 {
            Eirqcr3(0)
        }
    }
    impl core::fmt::Debug for Eirqcr3 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Eirqcr3")
                .field("eirqtrg", &self.eirqtrg())
                .field("eismpclk", &self.eismpclk())
                .field("efen", &self.efen())
                .field("nocsel", &self.nocsel())
                .field("nocen", &self.nocen())
                .finish()
        }
    }
    #[doc = "desc EIRQCR4."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Eirqcr4(pub u32);
    impl Eirqcr4 {
        #[doc = "desc EIRQTRG."]
        #[must_use]
        #[inline(always)]
        pub const fn eirqtrg(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x03;
            val as u8
        }
        #[doc = "desc EIRQTRG."]
        #[inline(always)]
        pub const fn set_eirqtrg(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 0usize)) | (((val as u32) & 0x03) << 0usize);
        }
        #[doc = "desc EISMPCLK."]
        #[must_use]
        #[inline(always)]
        pub const fn eismpclk(&self) -> u8 {
            let val = (self.0 >> 4usize) & 0x03;
            val as u8
        }
        #[doc = "desc EISMPCLK."]
        #[inline(always)]
        pub const fn set_eismpclk(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 4usize)) | (((val as u32) & 0x03) << 4usize);
        }
        #[doc = "desc EFEN."]
        #[must_use]
        #[inline(always)]
        pub const fn efen(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "desc EFEN."]
        #[inline(always)]
        pub const fn set_efen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "desc NOCSEL."]
        #[must_use]
        #[inline(always)]
        pub const fn nocsel(&self) -> u8 {
            let val = (self.0 >> 12usize) & 0x03;
            val as u8
        }
        #[doc = "desc NOCSEL."]
        #[inline(always)]
        pub const fn set_nocsel(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 12usize)) | (((val as u32) & 0x03) << 12usize);
        }
        #[doc = "desc NOCEN."]
        #[must_use]
        #[inline(always)]
        pub const fn nocen(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "desc NOCEN."]
        #[inline(always)]
        pub const fn set_nocen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
    }
    impl Default for Eirqcr4 {
        #[inline(always)]
        fn default() -> Eirqcr4 {
            Eirqcr4(0)
        }
    }
    impl core::fmt::Debug for Eirqcr4 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Eirqcr4")
                .field("eirqtrg", &self.eirqtrg())
                .field("eismpclk", &self.eismpclk())
                .field("efen", &self.efen())
                .field("nocsel", &self.nocsel())
                .field("nocen", &self.nocen())
                .finish()
        }
    }
    #[doc = "desc EIRQCR5."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Eirqcr5(pub u32);
    impl Eirqcr5 {
        #[doc = "desc EIRQTRG."]
        #[must_use]
        #[inline(always)]
        pub const fn eirqtrg(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x03;
            val as u8
        }
        #[doc = "desc EIRQTRG."]
        #[inline(always)]
        pub const fn set_eirqtrg(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 0usize)) | (((val as u32) & 0x03) << 0usize);
        }
        #[doc = "desc EISMPCLK."]
        #[must_use]
        #[inline(always)]
        pub const fn eismpclk(&self) -> u8 {
            let val = (self.0 >> 4usize) & 0x03;
            val as u8
        }
        #[doc = "desc EISMPCLK."]
        #[inline(always)]
        pub const fn set_eismpclk(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 4usize)) | (((val as u32) & 0x03) << 4usize);
        }
        #[doc = "desc EFEN."]
        #[must_use]
        #[inline(always)]
        pub const fn efen(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "desc EFEN."]
        #[inline(always)]
        pub const fn set_efen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "desc NOCSEL."]
        #[must_use]
        #[inline(always)]
        pub const fn nocsel(&self) -> u8 {
            let val = (self.0 >> 12usize) & 0x03;
            val as u8
        }
        #[doc = "desc NOCSEL."]
        #[inline(always)]
        pub const fn set_nocsel(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 12usize)) | (((val as u32) & 0x03) << 12usize);
        }
        #[doc = "desc NOCEN."]
        #[must_use]
        #[inline(always)]
        pub const fn nocen(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "desc NOCEN."]
        #[inline(always)]
        pub const fn set_nocen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
    }
    impl Default for Eirqcr5 {
        #[inline(always)]
        fn default() -> Eirqcr5 {
            Eirqcr5(0)
        }
    }
    impl core::fmt::Debug for Eirqcr5 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Eirqcr5")
                .field("eirqtrg", &self.eirqtrg())
                .field("eismpclk", &self.eismpclk())
                .field("efen", &self.efen())
                .field("nocsel", &self.nocsel())
                .field("nocen", &self.nocen())
                .finish()
        }
    }
    #[doc = "desc EIRQCR6."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Eirqcr6(pub u32);
    impl Eirqcr6 {
        #[doc = "desc EIRQTRG."]
        #[must_use]
        #[inline(always)]
        pub const fn eirqtrg(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x03;
            val as u8
        }
        #[doc = "desc EIRQTRG."]
        #[inline(always)]
        pub const fn set_eirqtrg(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 0usize)) | (((val as u32) & 0x03) << 0usize);
        }
        #[doc = "desc EISMPCLK."]
        #[must_use]
        #[inline(always)]
        pub const fn eismpclk(&self) -> u8 {
            let val = (self.0 >> 4usize) & 0x03;
            val as u8
        }
        #[doc = "desc EISMPCLK."]
        #[inline(always)]
        pub const fn set_eismpclk(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 4usize)) | (((val as u32) & 0x03) << 4usize);
        }
        #[doc = "desc EFEN."]
        #[must_use]
        #[inline(always)]
        pub const fn efen(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "desc EFEN."]
        #[inline(always)]
        pub const fn set_efen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "desc NOCSEL."]
        #[must_use]
        #[inline(always)]
        pub const fn nocsel(&self) -> u8 {
            let val = (self.0 >> 12usize) & 0x03;
            val as u8
        }
        #[doc = "desc NOCSEL."]
        #[inline(always)]
        pub const fn set_nocsel(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 12usize)) | (((val as u32) & 0x03) << 12usize);
        }
        #[doc = "desc NOCEN."]
        #[must_use]
        #[inline(always)]
        pub const fn nocen(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "desc NOCEN."]
        #[inline(always)]
        pub const fn set_nocen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
    }
    impl Default for Eirqcr6 {
        #[inline(always)]
        fn default() -> Eirqcr6 {
            Eirqcr6(0)
        }
    }
    impl core::fmt::Debug for Eirqcr6 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Eirqcr6")
                .field("eirqtrg", &self.eirqtrg())
                .field("eismpclk", &self.eismpclk())
                .field("efen", &self.efen())
                .field("nocsel", &self.nocsel())
                .field("nocen", &self.nocen())
                .finish()
        }
    }
    #[doc = "desc EIRQCR7."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Eirqcr7(pub u32);
    impl Eirqcr7 {
        #[doc = "desc EIRQTRG."]
        #[must_use]
        #[inline(always)]
        pub const fn eirqtrg(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x03;
            val as u8
        }
        #[doc = "desc EIRQTRG."]
        #[inline(always)]
        pub const fn set_eirqtrg(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 0usize)) | (((val as u32) & 0x03) << 0usize);
        }
        #[doc = "desc EISMPCLK."]
        #[must_use]
        #[inline(always)]
        pub const fn eismpclk(&self) -> u8 {
            let val = (self.0 >> 4usize) & 0x03;
            val as u8
        }
        #[doc = "desc EISMPCLK."]
        #[inline(always)]
        pub const fn set_eismpclk(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 4usize)) | (((val as u32) & 0x03) << 4usize);
        }
        #[doc = "desc EFEN."]
        #[must_use]
        #[inline(always)]
        pub const fn efen(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "desc EFEN."]
        #[inline(always)]
        pub const fn set_efen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "desc NOCSEL."]
        #[must_use]
        #[inline(always)]
        pub const fn nocsel(&self) -> u8 {
            let val = (self.0 >> 12usize) & 0x03;
            val as u8
        }
        #[doc = "desc NOCSEL."]
        #[inline(always)]
        pub const fn set_nocsel(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 12usize)) | (((val as u32) & 0x03) << 12usize);
        }
        #[doc = "desc NOCEN."]
        #[must_use]
        #[inline(always)]
        pub const fn nocen(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "desc NOCEN."]
        #[inline(always)]
        pub const fn set_nocen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
    }
    impl Default for Eirqcr7 {
        #[inline(always)]
        fn default() -> Eirqcr7 {
            Eirqcr7(0)
        }
    }
    impl core::fmt::Debug for Eirqcr7 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Eirqcr7")
                .field("eirqtrg", &self.eirqtrg())
                .field("eismpclk", &self.eismpclk())
                .field("efen", &self.efen())
                .field("nocsel", &self.nocsel())
                .field("nocen", &self.nocen())
                .finish()
        }
    }
    #[doc = "desc EVTER."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Evter(pub u32);
    impl Evter {
        #[doc = "desc EVTE0."]
        #[must_use]
        #[inline(always)]
        pub const fn evte0(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "desc EVTE0."]
        #[inline(always)]
        pub const fn set_evte0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "desc EVTE1."]
        #[must_use]
        #[inline(always)]
        pub const fn evte1(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "desc EVTE1."]
        #[inline(always)]
        pub const fn set_evte1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "desc EVTE2."]
        #[must_use]
        #[inline(always)]
        pub const fn evte2(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "desc EVTE2."]
        #[inline(always)]
        pub const fn set_evte2(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "desc EVTE3."]
        #[must_use]
        #[inline(always)]
        pub const fn evte3(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "desc EVTE3."]
        #[inline(always)]
        pub const fn set_evte3(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "desc EVTE4."]
        #[must_use]
        #[inline(always)]
        pub const fn evte4(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "desc EVTE4."]
        #[inline(always)]
        pub const fn set_evte4(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "desc EVTE5."]
        #[must_use]
        #[inline(always)]
        pub const fn evte5(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "desc EVTE5."]
        #[inline(always)]
        pub const fn set_evte5(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "desc EVTE6."]
        #[must_use]
        #[inline(always)]
        pub const fn evte6(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "desc EVTE6."]
        #[inline(always)]
        pub const fn set_evte6(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "desc EVTE7."]
        #[must_use]
        #[inline(always)]
        pub const fn evte7(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "desc EVTE7."]
        #[inline(always)]
        pub const fn set_evte7(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "desc EVTE8."]
        #[must_use]
        #[inline(always)]
        pub const fn evte8(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "desc EVTE8."]
        #[inline(always)]
        pub const fn set_evte8(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "desc EVTE9."]
        #[must_use]
        #[inline(always)]
        pub const fn evte9(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "desc EVTE9."]
        #[inline(always)]
        pub const fn set_evte9(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "desc EVTE10."]
        #[must_use]
        #[inline(always)]
        pub const fn evte10(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "desc EVTE10."]
        #[inline(always)]
        pub const fn set_evte10(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[doc = "desc EVTE11."]
        #[must_use]
        #[inline(always)]
        pub const fn evte11(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "desc EVTE11."]
        #[inline(always)]
        pub const fn set_evte11(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[doc = "desc EVTE12."]
        #[must_use]
        #[inline(always)]
        pub const fn evte12(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "desc EVTE12."]
        #[inline(always)]
        pub const fn set_evte12(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "desc EVTE13."]
        #[must_use]
        #[inline(always)]
        pub const fn evte13(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[doc = "desc EVTE13."]
        #[inline(always)]
        pub const fn set_evte13(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[doc = "desc EVTE14."]
        #[must_use]
        #[inline(always)]
        pub const fn evte14(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "desc EVTE14."]
        #[inline(always)]
        pub const fn set_evte14(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[doc = "desc EVTE15."]
        #[must_use]
        #[inline(always)]
        pub const fn evte15(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "desc EVTE15."]
        #[inline(always)]
        pub const fn set_evte15(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
        #[doc = "desc EVTE16."]
        #[must_use]
        #[inline(always)]
        pub const fn evte16(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "desc EVTE16."]
        #[inline(always)]
        pub const fn set_evte16(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "desc EVTE17."]
        #[must_use]
        #[inline(always)]
        pub const fn evte17(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "desc EVTE17."]
        #[inline(always)]
        pub const fn set_evte17(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[doc = "desc EVTE18."]
        #[must_use]
        #[inline(always)]
        pub const fn evte18(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[doc = "desc EVTE18."]
        #[inline(always)]
        pub const fn set_evte18(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
        #[doc = "desc EVTE19."]
        #[must_use]
        #[inline(always)]
        pub const fn evte19(&self) -> bool {
            let val = (self.0 >> 19usize) & 0x01;
            val != 0
        }
        #[doc = "desc EVTE19."]
        #[inline(always)]
        pub const fn set_evte19(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
        }
        #[doc = "desc EVTE20."]
        #[must_use]
        #[inline(always)]
        pub const fn evte20(&self) -> bool {
            let val = (self.0 >> 20usize) & 0x01;
            val != 0
        }
        #[doc = "desc EVTE20."]
        #[inline(always)]
        pub const fn set_evte20(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
        }
        #[doc = "desc EVTE21."]
        #[must_use]
        #[inline(always)]
        pub const fn evte21(&self) -> bool {
            let val = (self.0 >> 21usize) & 0x01;
            val != 0
        }
        #[doc = "desc EVTE21."]
        #[inline(always)]
        pub const fn set_evte21(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
        }
        #[doc = "desc EVTE22."]
        #[must_use]
        #[inline(always)]
        pub const fn evte22(&self) -> bool {
            let val = (self.0 >> 22usize) & 0x01;
            val != 0
        }
        #[doc = "desc EVTE22."]
        #[inline(always)]
        pub const fn set_evte22(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
        }
        #[doc = "desc EVTE23."]
        #[must_use]
        #[inline(always)]
        pub const fn evte23(&self) -> bool {
            let val = (self.0 >> 23usize) & 0x01;
            val != 0
        }
        #[doc = "desc EVTE23."]
        #[inline(always)]
        pub const fn set_evte23(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
        }
        #[doc = "desc EVTE24."]
        #[must_use]
        #[inline(always)]
        pub const fn evte24(&self) -> bool {
            let val = (self.0 >> 24usize) & 0x01;
            val != 0
        }
        #[doc = "desc EVTE24."]
        #[inline(always)]
        pub const fn set_evte24(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
        }
        #[doc = "desc EVTE25."]
        #[must_use]
        #[inline(always)]
        pub const fn evte25(&self) -> bool {
            let val = (self.0 >> 25usize) & 0x01;
            val != 0
        }
        #[doc = "desc EVTE25."]
        #[inline(always)]
        pub const fn set_evte25(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
        }
        #[doc = "desc EVTE26."]
        #[must_use]
        #[inline(always)]
        pub const fn evte26(&self) -> bool {
            let val = (self.0 >> 26usize) & 0x01;
            val != 0
        }
        #[doc = "desc EVTE26."]
        #[inline(always)]
        pub const fn set_evte26(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
        }
        #[doc = "desc EVTE27."]
        #[must_use]
        #[inline(always)]
        pub const fn evte27(&self) -> bool {
            let val = (self.0 >> 27usize) & 0x01;
            val != 0
        }
        #[doc = "desc EVTE27."]
        #[inline(always)]
        pub const fn set_evte27(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
        }
        #[doc = "desc EVTE28."]
        #[must_use]
        #[inline(always)]
        pub const fn evte28(&self) -> bool {
            let val = (self.0 >> 28usize) & 0x01;
            val != 0
        }
        #[doc = "desc EVTE28."]
        #[inline(always)]
        pub const fn set_evte28(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
        }
        #[doc = "desc EVTE29."]
        #[must_use]
        #[inline(always)]
        pub const fn evte29(&self) -> bool {
            let val = (self.0 >> 29usize) & 0x01;
            val != 0
        }
        #[doc = "desc EVTE29."]
        #[inline(always)]
        pub const fn set_evte29(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
        }
        #[doc = "desc EVTE30."]
        #[must_use]
        #[inline(always)]
        pub const fn evte30(&self) -> bool {
            let val = (self.0 >> 30usize) & 0x01;
            val != 0
        }
        #[doc = "desc EVTE30."]
        #[inline(always)]
        pub const fn set_evte30(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
        }
        #[doc = "desc EVTE31."]
        #[must_use]
        #[inline(always)]
        pub const fn evte31(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[doc = "desc EVTE31."]
        #[inline(always)]
        pub const fn set_evte31(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for Evter {
        #[inline(always)]
        fn default() -> Evter {
            Evter(0)
        }
    }
    impl core::fmt::Debug for Evter {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Evter")
                .field("evte0", &self.evte0())
                .field("evte1", &self.evte1())
                .field("evte2", &self.evte2())
                .field("evte3", &self.evte3())
                .field("evte4", &self.evte4())
                .field("evte5", &self.evte5())
                .field("evte6", &self.evte6())
                .field("evte7", &self.evte7())
                .field("evte8", &self.evte8())
                .field("evte9", &self.evte9())
                .field("evte10", &self.evte10())
                .field("evte11", &self.evte11())
                .field("evte12", &self.evte12())
                .field("evte13", &self.evte13())
                .field("evte14", &self.evte14())
                .field("evte15", &self.evte15())
                .field("evte16", &self.evte16())
                .field("evte17", &self.evte17())
                .field("evte18", &self.evte18())
                .field("evte19", &self.evte19())
                .field("evte20", &self.evte20())
                .field("evte21", &self.evte21())
                .field("evte22", &self.evte22())
                .field("evte23", &self.evte23())
                .field("evte24", &self.evte24())
                .field("evte25", &self.evte25())
                .field("evte26", &self.evte26())
                .field("evte27", &self.evte27())
                .field("evte28", &self.evte28())
                .field("evte29", &self.evte29())
                .field("evte30", &self.evte30())
                .field("evte31", &self.evte31())
                .finish()
        }
    }
    #[doc = "desc FPUIER."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Fpuier(pub u32);
    impl Fpuier {
        #[doc = "desc IOCIEN."]
        #[must_use]
        #[inline(always)]
        pub const fn iocien(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "desc IOCIEN."]
        #[inline(always)]
        pub const fn set_iocien(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "desc DZCIEN."]
        #[must_use]
        #[inline(always)]
        pub const fn dzcien(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "desc DZCIEN."]
        #[inline(always)]
        pub const fn set_dzcien(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "desc OFCIEN."]
        #[must_use]
        #[inline(always)]
        pub const fn ofcien(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "desc OFCIEN."]
        #[inline(always)]
        pub const fn set_ofcien(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "desc UFCIEN."]
        #[must_use]
        #[inline(always)]
        pub const fn ufcien(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "desc UFCIEN."]
        #[inline(always)]
        pub const fn set_ufcien(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "desc IXCIEN."]
        #[must_use]
        #[inline(always)]
        pub const fn ixcien(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "desc IXCIEN."]
        #[inline(always)]
        pub const fn set_ixcien(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "desc IDCIEN."]
        #[must_use]
        #[inline(always)]
        pub const fn idcien(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "desc IDCIEN."]
        #[inline(always)]
        pub const fn set_idcien(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
    }
    impl Default for Fpuier {
        #[inline(always)]
        fn default() -> Fpuier {
            Fpuier(0)
        }
    }
    impl core::fmt::Debug for Fpuier {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Fpuier")
                .field("iocien", &self.iocien())
                .field("dzcien", &self.dzcien())
                .field("ofcien", &self.ofcien())
                .field("ufcien", &self.ufcien())
                .field("ixcien", &self.ixcien())
                .field("idcien", &self.idcien())
                .finish()
        }
    }
    #[doc = "desc IER."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ier(pub u32);
    impl Ier {
        #[doc = "desc IEN0."]
        #[must_use]
        #[inline(always)]
        pub const fn ien0(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "desc IEN0."]
        #[inline(always)]
        pub const fn set_ien0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "desc IEN1."]
        #[must_use]
        #[inline(always)]
        pub const fn ien1(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "desc IEN1."]
        #[inline(always)]
        pub const fn set_ien1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "desc IEN2."]
        #[must_use]
        #[inline(always)]
        pub const fn ien2(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "desc IEN2."]
        #[inline(always)]
        pub const fn set_ien2(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "desc IEN3."]
        #[must_use]
        #[inline(always)]
        pub const fn ien3(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "desc IEN3."]
        #[inline(always)]
        pub const fn set_ien3(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "desc IEN4."]
        #[must_use]
        #[inline(always)]
        pub const fn ien4(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "desc IEN4."]
        #[inline(always)]
        pub const fn set_ien4(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "desc IEN5."]
        #[must_use]
        #[inline(always)]
        pub const fn ien5(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "desc IEN5."]
        #[inline(always)]
        pub const fn set_ien5(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "desc IEN6."]
        #[must_use]
        #[inline(always)]
        pub const fn ien6(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "desc IEN6."]
        #[inline(always)]
        pub const fn set_ien6(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "desc IEN7."]
        #[must_use]
        #[inline(always)]
        pub const fn ien7(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "desc IEN7."]
        #[inline(always)]
        pub const fn set_ien7(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "desc IEN8."]
        #[must_use]
        #[inline(always)]
        pub const fn ien8(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "desc IEN8."]
        #[inline(always)]
        pub const fn set_ien8(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "desc IEN9."]
        #[must_use]
        #[inline(always)]
        pub const fn ien9(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "desc IEN9."]
        #[inline(always)]
        pub const fn set_ien9(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "desc IEN10."]
        #[must_use]
        #[inline(always)]
        pub const fn ien10(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "desc IEN10."]
        #[inline(always)]
        pub const fn set_ien10(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[doc = "desc IEN11."]
        #[must_use]
        #[inline(always)]
        pub const fn ien11(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "desc IEN11."]
        #[inline(always)]
        pub const fn set_ien11(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[doc = "desc IEN12."]
        #[must_use]
        #[inline(always)]
        pub const fn ien12(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "desc IEN12."]
        #[inline(always)]
        pub const fn set_ien12(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "desc IEN13."]
        #[must_use]
        #[inline(always)]
        pub const fn ien13(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[doc = "desc IEN13."]
        #[inline(always)]
        pub const fn set_ien13(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[doc = "desc IEN14."]
        #[must_use]
        #[inline(always)]
        pub const fn ien14(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "desc IEN14."]
        #[inline(always)]
        pub const fn set_ien14(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[doc = "desc IEN15."]
        #[must_use]
        #[inline(always)]
        pub const fn ien15(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "desc IEN15."]
        #[inline(always)]
        pub const fn set_ien15(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
        #[doc = "desc IEN16."]
        #[must_use]
        #[inline(always)]
        pub const fn ien16(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "desc IEN16."]
        #[inline(always)]
        pub const fn set_ien16(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "desc IEN17."]
        #[must_use]
        #[inline(always)]
        pub const fn ien17(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "desc IEN17."]
        #[inline(always)]
        pub const fn set_ien17(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[doc = "desc IEN18."]
        #[must_use]
        #[inline(always)]
        pub const fn ien18(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[doc = "desc IEN18."]
        #[inline(always)]
        pub const fn set_ien18(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
        #[doc = "desc IEN19."]
        #[must_use]
        #[inline(always)]
        pub const fn ien19(&self) -> bool {
            let val = (self.0 >> 19usize) & 0x01;
            val != 0
        }
        #[doc = "desc IEN19."]
        #[inline(always)]
        pub const fn set_ien19(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
        }
        #[doc = "desc IEN20."]
        #[must_use]
        #[inline(always)]
        pub const fn ien20(&self) -> bool {
            let val = (self.0 >> 20usize) & 0x01;
            val != 0
        }
        #[doc = "desc IEN20."]
        #[inline(always)]
        pub const fn set_ien20(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
        }
        #[doc = "desc IEN21."]
        #[must_use]
        #[inline(always)]
        pub const fn ien21(&self) -> bool {
            let val = (self.0 >> 21usize) & 0x01;
            val != 0
        }
        #[doc = "desc IEN21."]
        #[inline(always)]
        pub const fn set_ien21(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
        }
        #[doc = "desc IEN22."]
        #[must_use]
        #[inline(always)]
        pub const fn ien22(&self) -> bool {
            let val = (self.0 >> 22usize) & 0x01;
            val != 0
        }
        #[doc = "desc IEN22."]
        #[inline(always)]
        pub const fn set_ien22(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
        }
        #[doc = "desc IEN23."]
        #[must_use]
        #[inline(always)]
        pub const fn ien23(&self) -> bool {
            let val = (self.0 >> 23usize) & 0x01;
            val != 0
        }
        #[doc = "desc IEN23."]
        #[inline(always)]
        pub const fn set_ien23(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
        }
        #[doc = "desc IEN24."]
        #[must_use]
        #[inline(always)]
        pub const fn ien24(&self) -> bool {
            let val = (self.0 >> 24usize) & 0x01;
            val != 0
        }
        #[doc = "desc IEN24."]
        #[inline(always)]
        pub const fn set_ien24(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
        }
        #[doc = "desc IEN25."]
        #[must_use]
        #[inline(always)]
        pub const fn ien25(&self) -> bool {
            let val = (self.0 >> 25usize) & 0x01;
            val != 0
        }
        #[doc = "desc IEN25."]
        #[inline(always)]
        pub const fn set_ien25(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
        }
        #[doc = "desc IEN26."]
        #[must_use]
        #[inline(always)]
        pub const fn ien26(&self) -> bool {
            let val = (self.0 >> 26usize) & 0x01;
            val != 0
        }
        #[doc = "desc IEN26."]
        #[inline(always)]
        pub const fn set_ien26(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
        }
        #[doc = "desc IEN27."]
        #[must_use]
        #[inline(always)]
        pub const fn ien27(&self) -> bool {
            let val = (self.0 >> 27usize) & 0x01;
            val != 0
        }
        #[doc = "desc IEN27."]
        #[inline(always)]
        pub const fn set_ien27(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
        }
        #[doc = "desc IEN28."]
        #[must_use]
        #[inline(always)]
        pub const fn ien28(&self) -> bool {
            let val = (self.0 >> 28usize) & 0x01;
            val != 0
        }
        #[doc = "desc IEN28."]
        #[inline(always)]
        pub const fn set_ien28(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
        }
        #[doc = "desc IEN29."]
        #[must_use]
        #[inline(always)]
        pub const fn ien29(&self) -> bool {
            let val = (self.0 >> 29usize) & 0x01;
            val != 0
        }
        #[doc = "desc IEN29."]
        #[inline(always)]
        pub const fn set_ien29(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
        }
        #[doc = "desc IEN30."]
        #[must_use]
        #[inline(always)]
        pub const fn ien30(&self) -> bool {
            let val = (self.0 >> 30usize) & 0x01;
            val != 0
        }
        #[doc = "desc IEN30."]
        #[inline(always)]
        pub const fn set_ien30(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
        }
        #[doc = "desc IEN31."]
        #[must_use]
        #[inline(always)]
        pub const fn ien31(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[doc = "desc IEN31."]
        #[inline(always)]
        pub const fn set_ien31(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
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
            f.debug_struct("Ier")
                .field("ien0", &self.ien0())
                .field("ien1", &self.ien1())
                .field("ien2", &self.ien2())
                .field("ien3", &self.ien3())
                .field("ien4", &self.ien4())
                .field("ien5", &self.ien5())
                .field("ien6", &self.ien6())
                .field("ien7", &self.ien7())
                .field("ien8", &self.ien8())
                .field("ien9", &self.ien9())
                .field("ien10", &self.ien10())
                .field("ien11", &self.ien11())
                .field("ien12", &self.ien12())
                .field("ien13", &self.ien13())
                .field("ien14", &self.ien14())
                .field("ien15", &self.ien15())
                .field("ien16", &self.ien16())
                .field("ien17", &self.ien17())
                .field("ien18", &self.ien18())
                .field("ien19", &self.ien19())
                .field("ien20", &self.ien20())
                .field("ien21", &self.ien21())
                .field("ien22", &self.ien22())
                .field("ien23", &self.ien23())
                .field("ien24", &self.ien24())
                .field("ien25", &self.ien25())
                .field("ien26", &self.ien26())
                .field("ien27", &self.ien27())
                .field("ien28", &self.ien28())
                .field("ien29", &self.ien29())
                .field("ien30", &self.ien30())
                .field("ien31", &self.ien31())
                .finish()
        }
    }
    #[doc = "desc NMIER."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Nmier(pub u32);
    impl Nmier {
        #[doc = "desc SWDTEN."]
        #[must_use]
        #[inline(always)]
        pub const fn swdten(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "desc SWDTEN."]
        #[inline(always)]
        pub const fn set_swdten(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "desc PVD1EN."]
        #[must_use]
        #[inline(always)]
        pub const fn pvd1en(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "desc PVD1EN."]
        #[inline(always)]
        pub const fn set_pvd1en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "desc PVD2EN."]
        #[must_use]
        #[inline(always)]
        pub const fn pvd2en(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "desc PVD2EN."]
        #[inline(always)]
        pub const fn set_pvd2en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "desc XTALSTPEN."]
        #[must_use]
        #[inline(always)]
        pub const fn xtalstpen(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "desc XTALSTPEN."]
        #[inline(always)]
        pub const fn set_xtalstpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "desc RECCERREN."]
        #[must_use]
        #[inline(always)]
        pub const fn reccerren(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "desc RECCERREN."]
        #[inline(always)]
        pub const fn set_reccerren(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "desc WDTEN."]
        #[must_use]
        #[inline(always)]
        pub const fn wdten(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "desc WDTEN."]
        #[inline(always)]
        pub const fn set_wdten(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[doc = "desc FECCERREN."]
        #[must_use]
        #[inline(always)]
        pub const fn feccerren(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "desc FECCERREN."]
        #[inline(always)]
        pub const fn set_feccerren(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
    }
    impl Default for Nmier {
        #[inline(always)]
        fn default() -> Nmier {
            Nmier(0)
        }
    }
    impl core::fmt::Debug for Nmier {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Nmier")
                .field("swdten", &self.swdten())
                .field("pvd1en", &self.pvd1en())
                .field("pvd2en", &self.pvd2en())
                .field("xtalstpen", &self.xtalstpen())
                .field("reccerren", &self.reccerren())
                .field("wdten", &self.wdten())
                .field("feccerren", &self.feccerren())
                .finish()
        }
    }
    #[doc = "desc NMIFCR."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Nmifcr(pub u32);
    impl Nmifcr {
        #[doc = "desc SWDTFCLR."]
        #[must_use]
        #[inline(always)]
        pub const fn swdtfclr(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "desc SWDTFCLR."]
        #[inline(always)]
        pub const fn set_swdtfclr(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "desc PVD1FCLR."]
        #[must_use]
        #[inline(always)]
        pub const fn pvd1fclr(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "desc PVD1FCLR."]
        #[inline(always)]
        pub const fn set_pvd1fclr(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "desc PVD2FCLR."]
        #[must_use]
        #[inline(always)]
        pub const fn pvd2fclr(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "desc PVD2FCLR."]
        #[inline(always)]
        pub const fn set_pvd2fclr(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "desc XTALSTPFCLR."]
        #[must_use]
        #[inline(always)]
        pub const fn xtalstpfclr(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "desc XTALSTPFCLR."]
        #[inline(always)]
        pub const fn set_xtalstpfclr(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "desc RECCERRFCLR."]
        #[must_use]
        #[inline(always)]
        pub const fn reccerrfclr(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "desc RECCERRFCLR."]
        #[inline(always)]
        pub const fn set_reccerrfclr(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "desc WDTFCLR."]
        #[must_use]
        #[inline(always)]
        pub const fn wdtfclr(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "desc WDTFCLR."]
        #[inline(always)]
        pub const fn set_wdtfclr(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[doc = "desc FECCERRFCLR."]
        #[must_use]
        #[inline(always)]
        pub const fn feccerrfclr(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "desc FECCERRFCLR."]
        #[inline(always)]
        pub const fn set_feccerrfclr(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
    }
    impl Default for Nmifcr {
        #[inline(always)]
        fn default() -> Nmifcr {
            Nmifcr(0)
        }
    }
    impl core::fmt::Debug for Nmifcr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Nmifcr")
                .field("swdtfclr", &self.swdtfclr())
                .field("pvd1fclr", &self.pvd1fclr())
                .field("pvd2fclr", &self.pvd2fclr())
                .field("xtalstpfclr", &self.xtalstpfclr())
                .field("reccerrfclr", &self.reccerrfclr())
                .field("wdtfclr", &self.wdtfclr())
                .field("feccerrfclr", &self.feccerrfclr())
                .finish()
        }
    }
    #[doc = "desc NMIFR."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Nmifr(pub u32);
    impl Nmifr {
        #[doc = "desc SWDTF."]
        #[must_use]
        #[inline(always)]
        pub const fn swdtf(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "desc SWDTF."]
        #[inline(always)]
        pub const fn set_swdtf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "desc PVD1F."]
        #[must_use]
        #[inline(always)]
        pub const fn pvd1f(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "desc PVD1F."]
        #[inline(always)]
        pub const fn set_pvd1f(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "desc PVD2F."]
        #[must_use]
        #[inline(always)]
        pub const fn pvd2f(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "desc PVD2F."]
        #[inline(always)]
        pub const fn set_pvd2f(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "desc XTALSTPF."]
        #[must_use]
        #[inline(always)]
        pub const fn xtalstpf(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "desc XTALSTPF."]
        #[inline(always)]
        pub const fn set_xtalstpf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "desc RECCERRF."]
        #[must_use]
        #[inline(always)]
        pub const fn reccerrf(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "desc RECCERRF."]
        #[inline(always)]
        pub const fn set_reccerrf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "desc WDTF."]
        #[must_use]
        #[inline(always)]
        pub const fn wdtf(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "desc WDTF."]
        #[inline(always)]
        pub const fn set_wdtf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[doc = "desc FECCERRF."]
        #[must_use]
        #[inline(always)]
        pub const fn feccerrf(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "desc FECCERRF."]
        #[inline(always)]
        pub const fn set_feccerrf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
    }
    impl Default for Nmifr {
        #[inline(always)]
        fn default() -> Nmifr {
            Nmifr(0)
        }
    }
    impl core::fmt::Debug for Nmifr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Nmifr")
                .field("swdtf", &self.swdtf())
                .field("pvd1f", &self.pvd1f())
                .field("pvd2f", &self.pvd2f())
                .field("xtalstpf", &self.xtalstpf())
                .field("reccerrf", &self.reccerrf())
                .field("wdtf", &self.wdtf())
                .field("feccerrf", &self.feccerrf())
                .finish()
        }
    }
    #[doc = "desc SWIER."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Swier(pub u32);
    impl Swier {
        #[doc = "desc SWIE0."]
        #[must_use]
        #[inline(always)]
        pub const fn swie0(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "desc SWIE0."]
        #[inline(always)]
        pub const fn set_swie0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "desc SWIE1."]
        #[must_use]
        #[inline(always)]
        pub const fn swie1(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "desc SWIE1."]
        #[inline(always)]
        pub const fn set_swie1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "desc SWIE2."]
        #[must_use]
        #[inline(always)]
        pub const fn swie2(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "desc SWIE2."]
        #[inline(always)]
        pub const fn set_swie2(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "desc SWIE3."]
        #[must_use]
        #[inline(always)]
        pub const fn swie3(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "desc SWIE3."]
        #[inline(always)]
        pub const fn set_swie3(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "desc SWIE4."]
        #[must_use]
        #[inline(always)]
        pub const fn swie4(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "desc SWIE4."]
        #[inline(always)]
        pub const fn set_swie4(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "desc SWIE5."]
        #[must_use]
        #[inline(always)]
        pub const fn swie5(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "desc SWIE5."]
        #[inline(always)]
        pub const fn set_swie5(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "desc SWIE6."]
        #[must_use]
        #[inline(always)]
        pub const fn swie6(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "desc SWIE6."]
        #[inline(always)]
        pub const fn set_swie6(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "desc SWIE7."]
        #[must_use]
        #[inline(always)]
        pub const fn swie7(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "desc SWIE7."]
        #[inline(always)]
        pub const fn set_swie7(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "desc SWIE8."]
        #[must_use]
        #[inline(always)]
        pub const fn swie8(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "desc SWIE8."]
        #[inline(always)]
        pub const fn set_swie8(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "desc SWIE9."]
        #[must_use]
        #[inline(always)]
        pub const fn swie9(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "desc SWIE9."]
        #[inline(always)]
        pub const fn set_swie9(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "desc SWIE10."]
        #[must_use]
        #[inline(always)]
        pub const fn swie10(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "desc SWIE10."]
        #[inline(always)]
        pub const fn set_swie10(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[doc = "desc SWIE11."]
        #[must_use]
        #[inline(always)]
        pub const fn swie11(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "desc SWIE11."]
        #[inline(always)]
        pub const fn set_swie11(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[doc = "desc SWIE12."]
        #[must_use]
        #[inline(always)]
        pub const fn swie12(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "desc SWIE12."]
        #[inline(always)]
        pub const fn set_swie12(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "desc SWIE13."]
        #[must_use]
        #[inline(always)]
        pub const fn swie13(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[doc = "desc SWIE13."]
        #[inline(always)]
        pub const fn set_swie13(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[doc = "desc SWIE14."]
        #[must_use]
        #[inline(always)]
        pub const fn swie14(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "desc SWIE14."]
        #[inline(always)]
        pub const fn set_swie14(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[doc = "desc SWIE15."]
        #[must_use]
        #[inline(always)]
        pub const fn swie15(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "desc SWIE15."]
        #[inline(always)]
        pub const fn set_swie15(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
        #[doc = "desc SWIE16."]
        #[must_use]
        #[inline(always)]
        pub const fn swie16(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "desc SWIE16."]
        #[inline(always)]
        pub const fn set_swie16(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "desc SWIE17."]
        #[must_use]
        #[inline(always)]
        pub const fn swie17(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "desc SWIE17."]
        #[inline(always)]
        pub const fn set_swie17(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[doc = "desc SWIE18."]
        #[must_use]
        #[inline(always)]
        pub const fn swie18(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[doc = "desc SWIE18."]
        #[inline(always)]
        pub const fn set_swie18(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
        #[doc = "desc SWIE19."]
        #[must_use]
        #[inline(always)]
        pub const fn swie19(&self) -> bool {
            let val = (self.0 >> 19usize) & 0x01;
            val != 0
        }
        #[doc = "desc SWIE19."]
        #[inline(always)]
        pub const fn set_swie19(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
        }
        #[doc = "desc SWIE20."]
        #[must_use]
        #[inline(always)]
        pub const fn swie20(&self) -> bool {
            let val = (self.0 >> 20usize) & 0x01;
            val != 0
        }
        #[doc = "desc SWIE20."]
        #[inline(always)]
        pub const fn set_swie20(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
        }
        #[doc = "desc SWIE21."]
        #[must_use]
        #[inline(always)]
        pub const fn swie21(&self) -> bool {
            let val = (self.0 >> 21usize) & 0x01;
            val != 0
        }
        #[doc = "desc SWIE21."]
        #[inline(always)]
        pub const fn set_swie21(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
        }
        #[doc = "desc SWIE22."]
        #[must_use]
        #[inline(always)]
        pub const fn swie22(&self) -> bool {
            let val = (self.0 >> 22usize) & 0x01;
            val != 0
        }
        #[doc = "desc SWIE22."]
        #[inline(always)]
        pub const fn set_swie22(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
        }
        #[doc = "desc SWIE23."]
        #[must_use]
        #[inline(always)]
        pub const fn swie23(&self) -> bool {
            let val = (self.0 >> 23usize) & 0x01;
            val != 0
        }
        #[doc = "desc SWIE23."]
        #[inline(always)]
        pub const fn set_swie23(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
        }
        #[doc = "desc SWIE24."]
        #[must_use]
        #[inline(always)]
        pub const fn swie24(&self) -> bool {
            let val = (self.0 >> 24usize) & 0x01;
            val != 0
        }
        #[doc = "desc SWIE24."]
        #[inline(always)]
        pub const fn set_swie24(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
        }
        #[doc = "desc SWIE25."]
        #[must_use]
        #[inline(always)]
        pub const fn swie25(&self) -> bool {
            let val = (self.0 >> 25usize) & 0x01;
            val != 0
        }
        #[doc = "desc SWIE25."]
        #[inline(always)]
        pub const fn set_swie25(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
        }
        #[doc = "desc SWIE26."]
        #[must_use]
        #[inline(always)]
        pub const fn swie26(&self) -> bool {
            let val = (self.0 >> 26usize) & 0x01;
            val != 0
        }
        #[doc = "desc SWIE26."]
        #[inline(always)]
        pub const fn set_swie26(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
        }
        #[doc = "desc SWIE27."]
        #[must_use]
        #[inline(always)]
        pub const fn swie27(&self) -> bool {
            let val = (self.0 >> 27usize) & 0x01;
            val != 0
        }
        #[doc = "desc SWIE27."]
        #[inline(always)]
        pub const fn set_swie27(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
        }
        #[doc = "desc SWIE28."]
        #[must_use]
        #[inline(always)]
        pub const fn swie28(&self) -> bool {
            let val = (self.0 >> 28usize) & 0x01;
            val != 0
        }
        #[doc = "desc SWIE28."]
        #[inline(always)]
        pub const fn set_swie28(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
        }
        #[doc = "desc SWIE29."]
        #[must_use]
        #[inline(always)]
        pub const fn swie29(&self) -> bool {
            let val = (self.0 >> 29usize) & 0x01;
            val != 0
        }
        #[doc = "desc SWIE29."]
        #[inline(always)]
        pub const fn set_swie29(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
        }
        #[doc = "desc SWIE30."]
        #[must_use]
        #[inline(always)]
        pub const fn swie30(&self) -> bool {
            let val = (self.0 >> 30usize) & 0x01;
            val != 0
        }
        #[doc = "desc SWIE30."]
        #[inline(always)]
        pub const fn set_swie30(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
        }
        #[doc = "desc SWIE31."]
        #[must_use]
        #[inline(always)]
        pub const fn swie31(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[doc = "desc SWIE31."]
        #[inline(always)]
        pub const fn set_swie31(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for Swier {
        #[inline(always)]
        fn default() -> Swier {
            Swier(0)
        }
    }
    impl core::fmt::Debug for Swier {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Swier")
                .field("swie0", &self.swie0())
                .field("swie1", &self.swie1())
                .field("swie2", &self.swie2())
                .field("swie3", &self.swie3())
                .field("swie4", &self.swie4())
                .field("swie5", &self.swie5())
                .field("swie6", &self.swie6())
                .field("swie7", &self.swie7())
                .field("swie8", &self.swie8())
                .field("swie9", &self.swie9())
                .field("swie10", &self.swie10())
                .field("swie11", &self.swie11())
                .field("swie12", &self.swie12())
                .field("swie13", &self.swie13())
                .field("swie14", &self.swie14())
                .field("swie15", &self.swie15())
                .field("swie16", &self.swie16())
                .field("swie17", &self.swie17())
                .field("swie18", &self.swie18())
                .field("swie19", &self.swie19())
                .field("swie20", &self.swie20())
                .field("swie21", &self.swie21())
                .field("swie22", &self.swie22())
                .field("swie23", &self.swie23())
                .field("swie24", &self.swie24())
                .field("swie25", &self.swie25())
                .field("swie26", &self.swie26())
                .field("swie27", &self.swie27())
                .field("swie28", &self.swie28())
                .field("swie29", &self.swie29())
                .field("swie30", &self.swie30())
                .field("swie31", &self.swie31())
                .finish()
        }
    }
    #[doc = "desc WKEN."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Wken(pub u32);
    impl Wken {
        #[doc = "desc EIRQWKEN."]
        #[must_use]
        #[inline(always)]
        pub const fn eirqwken(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[doc = "desc EIRQWKEN."]
        #[inline(always)]
        pub const fn set_eirqwken(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
        }
        #[doc = "desc SWDTWKEN."]
        #[must_use]
        #[inline(always)]
        pub const fn swdtwken(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "desc SWDTWKEN."]
        #[inline(always)]
        pub const fn set_swdtwken(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "desc PVD1WKEN."]
        #[must_use]
        #[inline(always)]
        pub const fn pvd1wken(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "desc PVD1WKEN."]
        #[inline(always)]
        pub const fn set_pvd1wken(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[doc = "desc PVD2WKEN."]
        #[must_use]
        #[inline(always)]
        pub const fn pvd2wken(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[doc = "desc PVD2WKEN."]
        #[inline(always)]
        pub const fn set_pvd2wken(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
        #[doc = "desc CMP1WKEN."]
        #[must_use]
        #[inline(always)]
        pub const fn cmp1wken(&self) -> bool {
            let val = (self.0 >> 19usize) & 0x01;
            val != 0
        }
        #[doc = "desc CMP1WKEN."]
        #[inline(always)]
        pub const fn set_cmp1wken(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
        }
        #[doc = "desc TMR0CMPWKEN."]
        #[must_use]
        #[inline(always)]
        pub const fn tmr0cmpwken(&self) -> bool {
            let val = (self.0 >> 23usize) & 0x01;
            val != 0
        }
        #[doc = "desc TMR0CMPWKEN."]
        #[inline(always)]
        pub const fn set_tmr0cmpwken(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
        }
        #[doc = "desc RXWKEN."]
        #[must_use]
        #[inline(always)]
        pub const fn rxwken(&self) -> bool {
            let val = (self.0 >> 26usize) & 0x01;
            val != 0
        }
        #[doc = "desc RXWKEN."]
        #[inline(always)]
        pub const fn set_rxwken(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
        }
        #[doc = "desc CMP2WKEN."]
        #[must_use]
        #[inline(always)]
        pub const fn cmp2wken(&self) -> bool {
            let val = (self.0 >> 29usize) & 0x01;
            val != 0
        }
        #[doc = "desc CMP2WKEN."]
        #[inline(always)]
        pub const fn set_cmp2wken(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
        }
    }
    impl Default for Wken {
        #[inline(always)]
        fn default() -> Wken {
            Wken(0)
        }
    }
    impl core::fmt::Debug for Wken {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Wken")
                .field("eirqwken", &self.eirqwken())
                .field("swdtwken", &self.swdtwken())
                .field("pvd1wken", &self.pvd1wken())
                .field("pvd2wken", &self.pvd2wken())
                .field("cmp1wken", &self.cmp1wken())
                .field("tmr0cmpwken", &self.tmr0cmpwken())
                .field("rxwken", &self.rxwken())
                .field("cmp2wken", &self.cmp2wken())
                .finish()
        }
    }
}
