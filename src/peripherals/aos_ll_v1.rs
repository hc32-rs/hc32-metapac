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
    #[doc = "desc INTC_SFTTRG."]
    #[inline(always)]
    pub const fn intc_sfttrg(self) -> crate::common::Reg<regs::IntcSfttrg, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0usize) as _) }
    }
    #[doc = "desc TMR0_TRGSEL."]
    #[inline(always)]
    pub const fn tmr0_trgsel(self) -> crate::common::Reg<regs::Tmr0Trgsel, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x20usize) as _) }
    }
    #[doc = "desc TMRB_TRGSEL."]
    #[inline(always)]
    pub const fn tmrb_trgsel(self) -> crate::common::Reg<regs::TmrbTrgsel, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x50usize) as _) }
    }
    #[doc = "desc ADC_TRGSEL0."]
    #[inline(always)]
    pub const fn adc_trgsel0(self) -> crate::common::Reg<regs::AdcTrgsel0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x60usize) as _) }
    }
    #[doc = "desc ADC_TRGSEL1."]
    #[inline(always)]
    pub const fn adc_trgsel1(self) -> crate::common::Reg<regs::AdcTrgsel1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x64usize) as _) }
    }
}
pub mod regs {
    #[doc = "desc ADC_TRGSEL0."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct AdcTrgsel0(pub u32);
    impl AdcTrgsel0 {
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
    impl Default for AdcTrgsel0 {
        #[inline(always)]
        fn default() -> AdcTrgsel0 {
            AdcTrgsel0(0)
        }
    }
    impl core::fmt::Debug for AdcTrgsel0 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("AdcTrgsel0")
                .field("trgsel", &self.trgsel())
                .finish()
        }
    }
    #[doc = "desc ADC_TRGSEL1."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct AdcTrgsel1(pub u32);
    impl AdcTrgsel1 {
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
    impl Default for AdcTrgsel1 {
        #[inline(always)]
        fn default() -> AdcTrgsel1 {
            AdcTrgsel1(0)
        }
    }
    impl core::fmt::Debug for AdcTrgsel1 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("AdcTrgsel1")
                .field("trgsel", &self.trgsel())
                .finish()
        }
    }
    #[doc = "desc INTC_SFTTRG."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct IntcSfttrg(pub u32);
    impl IntcSfttrg {
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
    impl Default for IntcSfttrg {
        #[inline(always)]
        fn default() -> IntcSfttrg {
            IntcSfttrg(0)
        }
    }
    impl core::fmt::Debug for IntcSfttrg {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("IntcSfttrg")
                .field("strg", &self.strg())
                .finish()
        }
    }
    #[doc = "desc TMR0_TRGSEL."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Tmr0Trgsel(pub u32);
    impl Tmr0Trgsel {
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
    impl Default for Tmr0Trgsel {
        #[inline(always)]
        fn default() -> Tmr0Trgsel {
            Tmr0Trgsel(0)
        }
    }
    impl core::fmt::Debug for Tmr0Trgsel {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Tmr0Trgsel")
                .field("trgsel", &self.trgsel())
                .finish()
        }
    }
    #[doc = "desc TMRB_TRGSEL."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct TmrbTrgsel(pub u32);
    impl TmrbTrgsel {
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
    impl Default for TmrbTrgsel {
        #[inline(always)]
        fn default() -> TmrbTrgsel {
            TmrbTrgsel(0)
        }
    }
    impl core::fmt::Debug for TmrbTrgsel {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("TmrbTrgsel")
                .field("trgsel", &self.trgsel())
                .finish()
        }
    }
}
