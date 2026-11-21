#![allow(clippy::missing_safety_doc)]
#![allow(clippy::identity_op)]
#![allow(clippy::unnecessary_cast)]
#![allow(clippy::erasing_op)]

#[doc = "desc AOS."]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Aos {
    ptr: *mut u8,
}
unsafe impl Send for Aos {}
unsafe impl Sync for Aos {}
impl Aos {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "desc INTC_STRGCR."]
    #[inline(always)]
    pub const fn intc_strgcr(self) -> crate::common::Reg<regs::IntcStrgcr, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0usize) as _) }
    }
    #[doc = "desc TMR0_HTSSR."]
    #[inline(always)]
    pub const fn tmr0_htssr(self) -> crate::common::Reg<regs::Tmr0Htssr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x20usize) as _) }
    }
    #[doc = "desc TMRB_HTSSR."]
    #[inline(always)]
    pub const fn tmrb_htssr(self) -> crate::common::Reg<regs::TmrbHtssr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x50usize) as _) }
    }
    #[doc = "desc ADC_ITRGSELR0."]
    #[inline(always)]
    pub const fn adc_itrgselr0(self) -> crate::common::Reg<regs::AdcItrgselr0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x60usize) as _) }
    }
    #[doc = "desc ADC_ITRGSELR1."]
    #[inline(always)]
    pub const fn adc_itrgselr1(self) -> crate::common::Reg<regs::AdcItrgselr1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x64usize) as _) }
    }
}
pub mod regs {
    #[doc = "desc ADC_ITRGSELR0."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct AdcItrgselr0(pub u32);
    impl AdcItrgselr0 {
        #[doc = "desc TRGSEL."]
        #[must_use]
        #[inline(always)]
        pub const fn trgsel(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x7f;
            val as u8
        }
        #[doc = "desc TRGSEL."]
        #[inline(always)]
        pub const fn set_trgsel(&mut self, val: u8) {
            self.0 = (self.0 & !(0x7f << 0usize)) | (((val as u32) & 0x7f) << 0usize);
        }
    }
    impl Default for AdcItrgselr0 {
        #[inline(always)]
        fn default() -> AdcItrgselr0 {
            AdcItrgselr0(0)
        }
    }
    impl core::fmt::Debug for AdcItrgselr0 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("AdcItrgselr0")
                .field("trgsel", &self.trgsel())
                .finish()
        }
    }
    #[doc = "desc ADC_ITRGSELR1."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct AdcItrgselr1(pub u32);
    impl AdcItrgselr1 {
        #[doc = "desc TRGSEL."]
        #[must_use]
        #[inline(always)]
        pub const fn trgsel(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x7f;
            val as u8
        }
        #[doc = "desc TRGSEL."]
        #[inline(always)]
        pub const fn set_trgsel(&mut self, val: u8) {
            self.0 = (self.0 & !(0x7f << 0usize)) | (((val as u32) & 0x7f) << 0usize);
        }
    }
    impl Default for AdcItrgselr1 {
        #[inline(always)]
        fn default() -> AdcItrgselr1 {
            AdcItrgselr1(0)
        }
    }
    impl core::fmt::Debug for AdcItrgselr1 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("AdcItrgselr1")
                .field("trgsel", &self.trgsel())
                .finish()
        }
    }
    #[doc = "desc INTC_STRGCR."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct IntcStrgcr(pub u32);
    impl IntcStrgcr {
        #[doc = "desc STRG."]
        #[must_use]
        #[inline(always)]
        pub const fn strg(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "desc STRG."]
        #[inline(always)]
        pub const fn set_strg(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
    }
    impl Default for IntcStrgcr {
        #[inline(always)]
        fn default() -> IntcStrgcr {
            IntcStrgcr(0)
        }
    }
    impl core::fmt::Debug for IntcStrgcr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("IntcStrgcr")
                .field("strg", &self.strg())
                .finish()
        }
    }
    #[doc = "desc TMR0_HTSSR."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Tmr0Htssr(pub u32);
    impl Tmr0Htssr {
        #[doc = "desc TRGSEL."]
        #[must_use]
        #[inline(always)]
        pub const fn trgsel(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x7f;
            val as u8
        }
        #[doc = "desc TRGSEL."]
        #[inline(always)]
        pub const fn set_trgsel(&mut self, val: u8) {
            self.0 = (self.0 & !(0x7f << 0usize)) | (((val as u32) & 0x7f) << 0usize);
        }
    }
    impl Default for Tmr0Htssr {
        #[inline(always)]
        fn default() -> Tmr0Htssr {
            Tmr0Htssr(0)
        }
    }
    impl core::fmt::Debug for Tmr0Htssr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Tmr0Htssr")
                .field("trgsel", &self.trgsel())
                .finish()
        }
    }
    #[doc = "desc TMRB_HTSSR."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct TmrbHtssr(pub u32);
    impl TmrbHtssr {
        #[doc = "desc TRGSEL."]
        #[must_use]
        #[inline(always)]
        pub const fn trgsel(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x7f;
            val as u8
        }
        #[doc = "desc TRGSEL."]
        #[inline(always)]
        pub const fn set_trgsel(&mut self, val: u8) {
            self.0 = (self.0 & !(0x7f << 0usize)) | (((val as u32) & 0x7f) << 0usize);
        }
    }
    impl Default for TmrbHtssr {
        #[inline(always)]
        fn default() -> TmrbHtssr {
            TmrbHtssr(0)
        }
    }
    impl core::fmt::Debug for TmrbHtssr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("TmrbHtssr")
                .field("trgsel", &self.trgsel())
                .finish()
        }
    }
}
