#![allow(clippy::missing_safety_doc)]
#![allow(clippy::identity_op)]
#![allow(clippy::unnecessary_cast)]
#![allow(clippy::erasing_op)]

#[doc = "desc EXTI."]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Exti {
    ptr: *mut u8,
}
unsafe impl Send for Exti {}
unsafe impl Sync for Exti {}
impl Exti {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "desc INTPSELR0."]
    #[inline(always)]
    pub const fn intpselr0(self) -> crate::common::Reg<regs::Intpselr0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0usize) as _) }
    }
    #[doc = "desc INTPSELR1."]
    #[inline(always)]
    pub const fn intpselr1(self) -> crate::common::Reg<regs::Intpselr1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x04usize) as _) }
    }
    #[doc = "desc INTPDFLTCR."]
    #[inline(always)]
    pub const fn intpdfltcr(self) -> crate::common::Reg<regs::Intpdfltcr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x10usize) as _) }
    }
    #[doc = "desc INTPDFLTER."]
    #[inline(always)]
    pub const fn intpdflter(self) -> crate::common::Reg<regs::Intpdflter, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x14usize) as _) }
    }
    #[doc = "desc INTPIESELR."]
    #[inline(always)]
    pub const fn intpieselr(self) -> crate::common::Reg<regs::Intpieselr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x18usize) as _) }
    }
    #[doc = "desc INTPHIEER."]
    #[inline(always)]
    pub const fn intphieer(self) -> crate::common::Reg<regs::Intphieer, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1cusize) as _) }
    }
    #[doc = "desc INTPLIEER."]
    #[inline(always)]
    pub const fn intplieer(self) -> crate::common::Reg<regs::Intplieer, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x20usize) as _) }
    }
    #[doc = "desc INTPRIEER."]
    #[inline(always)]
    pub const fn intprieer(self) -> crate::common::Reg<regs::Intprieer, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x24usize) as _) }
    }
    #[doc = "desc INTPFIEER."]
    #[inline(always)]
    pub const fn intpfieer(self) -> crate::common::Reg<regs::Intpfieer, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x28usize) as _) }
    }
    #[doc = "desc INTPIEFR."]
    #[inline(always)]
    pub const fn intpiefr(self) -> crate::common::Reg<regs::Intpiefr, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x2cusize) as _) }
    }
    #[doc = "desc INTPIECLR."]
    #[inline(always)]
    pub const fn intpieclr(self) -> crate::common::Reg<regs::Intpieclr, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x30usize) as _) }
    }
    #[doc = "desc WKUPRIER."]
    #[inline(always)]
    pub const fn wkuprier(self) -> crate::common::Reg<regs::Wkuprier, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x34usize) as _) }
    }
    #[doc = "desc WKUPFIER."]
    #[inline(always)]
    pub const fn wkupfier(self) -> crate::common::Reg<regs::Wkupfier, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x38usize) as _) }
    }
    #[doc = "desc WKUP_IFR."]
    #[inline(always)]
    pub const fn wkup_ifr(self) -> crate::common::Reg<regs::WkupIfr, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x3cusize) as _) }
    }
    #[doc = "desc WKUP_ICLR."]
    #[inline(always)]
    pub const fn wkup_iclr(self) -> crate::common::Reg<regs::WkupIclr, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x40usize) as _) }
    }
}
pub mod regs {
    #[doc = "desc INTPDFLTCR."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Intpdfltcr(pub u32);
    impl Intpdfltcr {
        #[doc = "desc CLKSEL."]
        #[must_use]
        #[inline(always)]
        pub const fn clksel(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "desc CLKSEL."]
        #[inline(always)]
        pub const fn set_clksel(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
    }
    impl Default for Intpdfltcr {
        #[inline(always)]
        fn default() -> Intpdfltcr {
            Intpdfltcr(0)
        }
    }
    impl core::fmt::Debug for Intpdfltcr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Intpdfltcr")
                .field("clksel", &self.clksel())
                .finish()
        }
    }
    #[doc = "desc INTPDFLTER."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Intpdflter(pub u32);
    impl Intpdflter {
        #[doc = "desc EN."]
        #[must_use]
        #[inline(always)]
        pub const fn en(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[doc = "desc EN."]
        #[inline(always)]
        pub const fn set_en(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
        }
    }
    impl Default for Intpdflter {
        #[inline(always)]
        fn default() -> Intpdflter {
            Intpdflter(0)
        }
    }
    impl core::fmt::Debug for Intpdflter {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Intpdflter")
                .field("en", &self.en())
                .finish()
        }
    }
    #[doc = "desc INTPFIEER."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Intpfieer(pub u32);
    impl Intpfieer {
        #[doc = "desc EN."]
        #[must_use]
        #[inline(always)]
        pub const fn en(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[doc = "desc EN."]
        #[inline(always)]
        pub const fn set_en(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
        }
    }
    impl Default for Intpfieer {
        #[inline(always)]
        fn default() -> Intpfieer {
            Intpfieer(0)
        }
    }
    impl core::fmt::Debug for Intpfieer {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Intpfieer").field("en", &self.en()).finish()
        }
    }
    #[doc = "desc INTPHIEER."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Intphieer(pub u32);
    impl Intphieer {
        #[doc = "desc EN."]
        #[must_use]
        #[inline(always)]
        pub const fn en(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[doc = "desc EN."]
        #[inline(always)]
        pub const fn set_en(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
        }
    }
    impl Default for Intphieer {
        #[inline(always)]
        fn default() -> Intphieer {
            Intphieer(0)
        }
    }
    impl core::fmt::Debug for Intphieer {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Intphieer").field("en", &self.en()).finish()
        }
    }
    #[doc = "desc INTPIECLR."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Intpieclr(pub u32);
    impl Intpieclr {
        #[doc = "desc IECL."]
        #[must_use]
        #[inline(always)]
        pub const fn iecl(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[doc = "desc IECL."]
        #[inline(always)]
        pub const fn set_iecl(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
        }
    }
    impl Default for Intpieclr {
        #[inline(always)]
        fn default() -> Intpieclr {
            Intpieclr(0)
        }
    }
    impl core::fmt::Debug for Intpieclr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Intpieclr")
                .field("iecl", &self.iecl())
                .finish()
        }
    }
    #[doc = "desc INTPIEFR."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Intpiefr(pub u32);
    impl Intpiefr {
        #[doc = "desc IEF."]
        #[must_use]
        #[inline(always)]
        pub const fn ief(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[doc = "desc IEF."]
        #[inline(always)]
        pub const fn set_ief(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
        }
    }
    impl Default for Intpiefr {
        #[inline(always)]
        fn default() -> Intpiefr {
            Intpiefr(0)
        }
    }
    impl core::fmt::Debug for Intpiefr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Intpiefr")
                .field("ief", &self.ief())
                .finish()
        }
    }
    #[doc = "desc INTPIESELR."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Intpieselr(pub u32);
    impl Intpieselr {
        #[doc = "desc IESEL."]
        #[must_use]
        #[inline(always)]
        pub const fn iesel(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[doc = "desc IESEL."]
        #[inline(always)]
        pub const fn set_iesel(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
        }
    }
    impl Default for Intpieselr {
        #[inline(always)]
        fn default() -> Intpieselr {
            Intpieselr(0)
        }
    }
    impl core::fmt::Debug for Intpieselr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Intpieselr")
                .field("iesel", &self.iesel())
                .finish()
        }
    }
    #[doc = "desc INTPLIEER."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Intplieer(pub u32);
    impl Intplieer {
        #[doc = "desc EN."]
        #[must_use]
        #[inline(always)]
        pub const fn en(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[doc = "desc EN."]
        #[inline(always)]
        pub const fn set_en(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
        }
    }
    impl Default for Intplieer {
        #[inline(always)]
        fn default() -> Intplieer {
            Intplieer(0)
        }
    }
    impl core::fmt::Debug for Intplieer {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Intplieer").field("en", &self.en()).finish()
        }
    }
    #[doc = "desc INTPRIEER."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Intprieer(pub u32);
    impl Intprieer {
        #[doc = "desc EN."]
        #[must_use]
        #[inline(always)]
        pub const fn en(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[doc = "desc EN."]
        #[inline(always)]
        pub const fn set_en(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
        }
    }
    impl Default for Intprieer {
        #[inline(always)]
        fn default() -> Intprieer {
            Intprieer(0)
        }
    }
    impl core::fmt::Debug for Intprieer {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Intprieer").field("en", &self.en()).finish()
        }
    }
    #[doc = "desc INTPSELR0."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Intpselr0(pub u32);
    impl Intpselr0 {
        #[doc = "desc INSEL0."]
        #[must_use]
        #[inline(always)]
        pub const fn insel0(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x07;
            val as u8
        }
        #[doc = "desc INSEL0."]
        #[inline(always)]
        pub const fn set_insel0(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 0usize)) | (((val as u32) & 0x07) << 0usize);
        }
        #[doc = "desc INSEL1."]
        #[must_use]
        #[inline(always)]
        pub const fn insel1(&self) -> u8 {
            let val = (self.0 >> 4usize) & 0x07;
            val as u8
        }
        #[doc = "desc INSEL1."]
        #[inline(always)]
        pub const fn set_insel1(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 4usize)) | (((val as u32) & 0x07) << 4usize);
        }
        #[doc = "desc INSEL2."]
        #[must_use]
        #[inline(always)]
        pub const fn insel2(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0x07;
            val as u8
        }
        #[doc = "desc INSEL2."]
        #[inline(always)]
        pub const fn set_insel2(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 8usize)) | (((val as u32) & 0x07) << 8usize);
        }
        #[doc = "desc INSEL3."]
        #[must_use]
        #[inline(always)]
        pub const fn insel3(&self) -> u8 {
            let val = (self.0 >> 12usize) & 0x07;
            val as u8
        }
        #[doc = "desc INSEL3."]
        #[inline(always)]
        pub const fn set_insel3(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 12usize)) | (((val as u32) & 0x07) << 12usize);
        }
        #[doc = "desc INSEL4."]
        #[must_use]
        #[inline(always)]
        pub const fn insel4(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x07;
            val as u8
        }
        #[doc = "desc INSEL4."]
        #[inline(always)]
        pub const fn set_insel4(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 16usize)) | (((val as u32) & 0x07) << 16usize);
        }
        #[doc = "desc INSEL5."]
        #[must_use]
        #[inline(always)]
        pub const fn insel5(&self) -> u8 {
            let val = (self.0 >> 20usize) & 0x07;
            val as u8
        }
        #[doc = "desc INSEL5."]
        #[inline(always)]
        pub const fn set_insel5(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 20usize)) | (((val as u32) & 0x07) << 20usize);
        }
        #[doc = "desc INSEL6."]
        #[must_use]
        #[inline(always)]
        pub const fn insel6(&self) -> u8 {
            let val = (self.0 >> 24usize) & 0x07;
            val as u8
        }
        #[doc = "desc INSEL6."]
        #[inline(always)]
        pub const fn set_insel6(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 24usize)) | (((val as u32) & 0x07) << 24usize);
        }
        #[doc = "desc INSEL7."]
        #[must_use]
        #[inline(always)]
        pub const fn insel7(&self) -> u8 {
            let val = (self.0 >> 28usize) & 0x07;
            val as u8
        }
        #[doc = "desc INSEL7."]
        #[inline(always)]
        pub const fn set_insel7(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 28usize)) | (((val as u32) & 0x07) << 28usize);
        }
    }
    impl Default for Intpselr0 {
        #[inline(always)]
        fn default() -> Intpselr0 {
            Intpselr0(0)
        }
    }
    impl core::fmt::Debug for Intpselr0 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Intpselr0")
                .field("insel0", &self.insel0())
                .field("insel1", &self.insel1())
                .field("insel2", &self.insel2())
                .field("insel3", &self.insel3())
                .field("insel4", &self.insel4())
                .field("insel5", &self.insel5())
                .field("insel6", &self.insel6())
                .field("insel7", &self.insel7())
                .finish()
        }
    }
    #[doc = "desc INTPSELR1."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Intpselr1(pub u32);
    impl Intpselr1 {
        #[doc = "desc INSEL8."]
        #[must_use]
        #[inline(always)]
        pub const fn insel8(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x07;
            val as u8
        }
        #[doc = "desc INSEL8."]
        #[inline(always)]
        pub const fn set_insel8(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 0usize)) | (((val as u32) & 0x07) << 0usize);
        }
        #[doc = "desc INSEL9."]
        #[must_use]
        #[inline(always)]
        pub const fn insel9(&self) -> u8 {
            let val = (self.0 >> 4usize) & 0x07;
            val as u8
        }
        #[doc = "desc INSEL9."]
        #[inline(always)]
        pub const fn set_insel9(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 4usize)) | (((val as u32) & 0x07) << 4usize);
        }
        #[doc = "desc INSEL10."]
        #[must_use]
        #[inline(always)]
        pub const fn insel10(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0x07;
            val as u8
        }
        #[doc = "desc INSEL10."]
        #[inline(always)]
        pub const fn set_insel10(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 8usize)) | (((val as u32) & 0x07) << 8usize);
        }
        #[doc = "desc INSEL11."]
        #[must_use]
        #[inline(always)]
        pub const fn insel11(&self) -> u8 {
            let val = (self.0 >> 12usize) & 0x07;
            val as u8
        }
        #[doc = "desc INSEL11."]
        #[inline(always)]
        pub const fn set_insel11(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 12usize)) | (((val as u32) & 0x07) << 12usize);
        }
        #[doc = "desc INSEL12."]
        #[must_use]
        #[inline(always)]
        pub const fn insel12(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x07;
            val as u8
        }
        #[doc = "desc INSEL12."]
        #[inline(always)]
        pub const fn set_insel12(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 16usize)) | (((val as u32) & 0x07) << 16usize);
        }
        #[doc = "desc INSEL13."]
        #[must_use]
        #[inline(always)]
        pub const fn insel13(&self) -> u8 {
            let val = (self.0 >> 20usize) & 0x07;
            val as u8
        }
        #[doc = "desc INSEL13."]
        #[inline(always)]
        pub const fn set_insel13(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 20usize)) | (((val as u32) & 0x07) << 20usize);
        }
        #[doc = "desc INSEL14."]
        #[must_use]
        #[inline(always)]
        pub const fn insel14(&self) -> u8 {
            let val = (self.0 >> 24usize) & 0x07;
            val as u8
        }
        #[doc = "desc INSEL14."]
        #[inline(always)]
        pub const fn set_insel14(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 24usize)) | (((val as u32) & 0x07) << 24usize);
        }
        #[doc = "desc INSEL15."]
        #[must_use]
        #[inline(always)]
        pub const fn insel15(&self) -> u8 {
            let val = (self.0 >> 28usize) & 0x07;
            val as u8
        }
        #[doc = "desc INSEL15."]
        #[inline(always)]
        pub const fn set_insel15(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 28usize)) | (((val as u32) & 0x07) << 28usize);
        }
    }
    impl Default for Intpselr1 {
        #[inline(always)]
        fn default() -> Intpselr1 {
            Intpselr1(0)
        }
    }
    impl core::fmt::Debug for Intpselr1 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Intpselr1")
                .field("insel8", &self.insel8())
                .field("insel9", &self.insel9())
                .field("insel10", &self.insel10())
                .field("insel11", &self.insel11())
                .field("insel12", &self.insel12())
                .field("insel13", &self.insel13())
                .field("insel14", &self.insel14())
                .field("insel15", &self.insel15())
                .finish()
        }
    }
    #[doc = "desc WKUP_ICLR."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct WkupIclr(pub u32);
    impl WkupIclr {
        #[doc = "desc ICL."]
        #[must_use]
        #[inline(always)]
        pub const fn icl(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[doc = "desc ICL."]
        #[inline(always)]
        pub const fn set_icl(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
        }
    }
    impl Default for WkupIclr {
        #[inline(always)]
        fn default() -> WkupIclr {
            WkupIclr(0)
        }
    }
    impl core::fmt::Debug for WkupIclr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("WkupIclr")
                .field("icl", &self.icl())
                .finish()
        }
    }
    #[doc = "desc WKUP_IFR."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct WkupIfr(pub u32);
    impl WkupIfr {
        #[doc = "desc IF."]
        #[must_use]
        #[inline(always)]
        pub const fn if_(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[doc = "desc IF."]
        #[inline(always)]
        pub const fn set_if_(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
        }
    }
    impl Default for WkupIfr {
        #[inline(always)]
        fn default() -> WkupIfr {
            WkupIfr(0)
        }
    }
    impl core::fmt::Debug for WkupIfr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("WkupIfr").field("if_", &self.if_()).finish()
        }
    }
    #[doc = "desc WKUPFIER."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Wkupfier(pub u32);
    impl Wkupfier {
        #[doc = "desc EN."]
        #[must_use]
        #[inline(always)]
        pub const fn en(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[doc = "desc EN."]
        #[inline(always)]
        pub const fn set_en(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
        }
    }
    impl Default for Wkupfier {
        #[inline(always)]
        fn default() -> Wkupfier {
            Wkupfier(0)
        }
    }
    impl core::fmt::Debug for Wkupfier {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Wkupfier").field("en", &self.en()).finish()
        }
    }
    #[doc = "desc WKUPRIER."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Wkuprier(pub u32);
    impl Wkuprier {
        #[doc = "desc EN."]
        #[must_use]
        #[inline(always)]
        pub const fn en(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[doc = "desc EN."]
        #[inline(always)]
        pub const fn set_en(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
        }
    }
    impl Default for Wkuprier {
        #[inline(always)]
        fn default() -> Wkuprier {
            Wkuprier(0)
        }
    }
    impl core::fmt::Debug for Wkuprier {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Wkuprier").field("en", &self.en()).finish()
        }
    }
}
