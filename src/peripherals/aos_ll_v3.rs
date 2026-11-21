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
    #[doc = "desc INTSFTTRG."]
    #[inline(always)]
    pub const fn intsfttrg(self) -> crate::common::Reg<regs::Intsfttrg, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0usize) as _) }
    }
    #[doc = "desc DMA_TRGSEL0."]
    #[inline(always)]
    pub const fn dma_trgsel0(self) -> crate::common::Reg<regs::DmaTrgsel0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x04usize) as _) }
    }
    #[doc = "desc DMA_TRGSEL1."]
    #[inline(always)]
    pub const fn dma_trgsel1(self) -> crate::common::Reg<regs::DmaTrgsel1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x08usize) as _) }
    }
    #[doc = "desc DMA_RC_TRGSEL."]
    #[inline(always)]
    pub const fn dma_rc_trgsel(self) -> crate::common::Reg<regs::DmaRcTrgsel, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0cusize) as _) }
    }
    #[doc = "desc ADC_TRGSEL0."]
    #[inline(always)]
    pub const fn adc_trgsel0(self) -> crate::common::Reg<regs::AdcTrgsel0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x10usize) as _) }
    }
    #[doc = "desc ADC_TRGSEL1."]
    #[inline(always)]
    pub const fn adc_trgsel1(self) -> crate::common::Reg<regs::AdcTrgsel1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x14usize) as _) }
    }
    #[doc = "desc ADC_TRGSEL2."]
    #[inline(always)]
    pub const fn adc_trgsel2(self) -> crate::common::Reg<regs::AdcTrgsel2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x18usize) as _) }
    }
    #[doc = "desc ADC_TRGSEL3."]
    #[inline(always)]
    pub const fn adc_trgsel3(self) -> crate::common::Reg<regs::AdcTrgsel3, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1cusize) as _) }
    }
    #[doc = "desc ADC_TRGSEL4."]
    #[inline(always)]
    pub const fn adc_trgsel4(self) -> crate::common::Reg<regs::AdcTrgsel4, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x20usize) as _) }
    }
    #[doc = "desc ADC_TRGSEL5."]
    #[inline(always)]
    pub const fn adc_trgsel5(self) -> crate::common::Reg<regs::AdcTrgsel5, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x24usize) as _) }
    }
    #[doc = "desc COMTRGSEL1."]
    #[inline(always)]
    pub const fn comtrgsel1(self) -> crate::common::Reg<regs::Comtrgsel1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x28usize) as _) }
    }
    #[doc = "desc COMTRGSEL2."]
    #[inline(always)]
    pub const fn comtrgsel2(self) -> crate::common::Reg<regs::Comtrgsel2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x2cusize) as _) }
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
        pub const fn trgsel(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x01ff;
            val as u16
        }
        #[doc = "desc TRGSEL."]
        #[inline(always)]
        pub const fn set_trgsel(&mut self, val: u16) {
            self.0 = (self.0 & !(0x01ff << 0usize)) | (((val as u32) & 0x01ff) << 0usize);
        }
        #[doc = "desc COMEN."]
        #[must_use]
        #[inline(always)]
        pub const fn comen(&self) -> u8 {
            let val = (self.0 >> 30usize) & 0x03;
            val as u8
        }
        #[doc = "desc COMEN."]
        #[inline(always)]
        pub const fn set_comen(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 30usize)) | (((val as u32) & 0x03) << 30usize);
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
                .field("comen", &self.comen())
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
        pub const fn trgsel(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x01ff;
            val as u16
        }
        #[doc = "desc TRGSEL."]
        #[inline(always)]
        pub const fn set_trgsel(&mut self, val: u16) {
            self.0 = (self.0 & !(0x01ff << 0usize)) | (((val as u32) & 0x01ff) << 0usize);
        }
        #[doc = "desc COMEN."]
        #[must_use]
        #[inline(always)]
        pub const fn comen(&self) -> u8 {
            let val = (self.0 >> 30usize) & 0x03;
            val as u8
        }
        #[doc = "desc COMEN."]
        #[inline(always)]
        pub const fn set_comen(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 30usize)) | (((val as u32) & 0x03) << 30usize);
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
                .field("comen", &self.comen())
                .finish()
        }
    }
    #[doc = "desc ADC_TRGSEL2."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct AdcTrgsel2(pub u32);
    impl AdcTrgsel2 {
        #[doc = "desc TRGSEL."]
        #[must_use]
        #[inline(always)]
        pub const fn trgsel(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x01ff;
            val as u16
        }
        #[doc = "desc TRGSEL."]
        #[inline(always)]
        pub const fn set_trgsel(&mut self, val: u16) {
            self.0 = (self.0 & !(0x01ff << 0usize)) | (((val as u32) & 0x01ff) << 0usize);
        }
        #[doc = "desc COMEN."]
        #[must_use]
        #[inline(always)]
        pub const fn comen(&self) -> u8 {
            let val = (self.0 >> 30usize) & 0x03;
            val as u8
        }
        #[doc = "desc COMEN."]
        #[inline(always)]
        pub const fn set_comen(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 30usize)) | (((val as u32) & 0x03) << 30usize);
        }
    }
    impl Default for AdcTrgsel2 {
        #[inline(always)]
        fn default() -> AdcTrgsel2 {
            AdcTrgsel2(0)
        }
    }
    impl core::fmt::Debug for AdcTrgsel2 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("AdcTrgsel2")
                .field("trgsel", &self.trgsel())
                .field("comen", &self.comen())
                .finish()
        }
    }
    #[doc = "desc ADC_TRGSEL3."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct AdcTrgsel3(pub u32);
    impl AdcTrgsel3 {
        #[doc = "desc TRGSEL."]
        #[must_use]
        #[inline(always)]
        pub const fn trgsel(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x01ff;
            val as u16
        }
        #[doc = "desc TRGSEL."]
        #[inline(always)]
        pub const fn set_trgsel(&mut self, val: u16) {
            self.0 = (self.0 & !(0x01ff << 0usize)) | (((val as u32) & 0x01ff) << 0usize);
        }
        #[doc = "desc COMEN."]
        #[must_use]
        #[inline(always)]
        pub const fn comen(&self) -> u8 {
            let val = (self.0 >> 30usize) & 0x03;
            val as u8
        }
        #[doc = "desc COMEN."]
        #[inline(always)]
        pub const fn set_comen(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 30usize)) | (((val as u32) & 0x03) << 30usize);
        }
    }
    impl Default for AdcTrgsel3 {
        #[inline(always)]
        fn default() -> AdcTrgsel3 {
            AdcTrgsel3(0)
        }
    }
    impl core::fmt::Debug for AdcTrgsel3 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("AdcTrgsel3")
                .field("trgsel", &self.trgsel())
                .field("comen", &self.comen())
                .finish()
        }
    }
    #[doc = "desc ADC_TRGSEL4."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct AdcTrgsel4(pub u32);
    impl AdcTrgsel4 {
        #[doc = "desc TRGSEL."]
        #[must_use]
        #[inline(always)]
        pub const fn trgsel(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x01ff;
            val as u16
        }
        #[doc = "desc TRGSEL."]
        #[inline(always)]
        pub const fn set_trgsel(&mut self, val: u16) {
            self.0 = (self.0 & !(0x01ff << 0usize)) | (((val as u32) & 0x01ff) << 0usize);
        }
        #[doc = "desc COMEN."]
        #[must_use]
        #[inline(always)]
        pub const fn comen(&self) -> u8 {
            let val = (self.0 >> 30usize) & 0x03;
            val as u8
        }
        #[doc = "desc COMEN."]
        #[inline(always)]
        pub const fn set_comen(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 30usize)) | (((val as u32) & 0x03) << 30usize);
        }
    }
    impl Default for AdcTrgsel4 {
        #[inline(always)]
        fn default() -> AdcTrgsel4 {
            AdcTrgsel4(0)
        }
    }
    impl core::fmt::Debug for AdcTrgsel4 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("AdcTrgsel4")
                .field("trgsel", &self.trgsel())
                .field("comen", &self.comen())
                .finish()
        }
    }
    #[doc = "desc ADC_TRGSEL5."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct AdcTrgsel5(pub u32);
    impl AdcTrgsel5 {
        #[doc = "desc TRGSEL."]
        #[must_use]
        #[inline(always)]
        pub const fn trgsel(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x01ff;
            val as u16
        }
        #[doc = "desc TRGSEL."]
        #[inline(always)]
        pub const fn set_trgsel(&mut self, val: u16) {
            self.0 = (self.0 & !(0x01ff << 0usize)) | (((val as u32) & 0x01ff) << 0usize);
        }
        #[doc = "desc COMEN."]
        #[must_use]
        #[inline(always)]
        pub const fn comen(&self) -> u8 {
            let val = (self.0 >> 30usize) & 0x03;
            val as u8
        }
        #[doc = "desc COMEN."]
        #[inline(always)]
        pub const fn set_comen(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 30usize)) | (((val as u32) & 0x03) << 30usize);
        }
    }
    impl Default for AdcTrgsel5 {
        #[inline(always)]
        fn default() -> AdcTrgsel5 {
            AdcTrgsel5(0)
        }
    }
    impl core::fmt::Debug for AdcTrgsel5 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("AdcTrgsel5")
                .field("trgsel", &self.trgsel())
                .field("comen", &self.comen())
                .finish()
        }
    }
    #[doc = "desc COMTRGSEL1."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Comtrgsel1(pub u32);
    impl Comtrgsel1 {
        #[doc = "desc TRGSEL."]
        #[must_use]
        #[inline(always)]
        pub const fn trgsel(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x01ff;
            val as u16
        }
        #[doc = "desc TRGSEL."]
        #[inline(always)]
        pub const fn set_trgsel(&mut self, val: u16) {
            self.0 = (self.0 & !(0x01ff << 0usize)) | (((val as u32) & 0x01ff) << 0usize);
        }
    }
    impl Default for Comtrgsel1 {
        #[inline(always)]
        fn default() -> Comtrgsel1 {
            Comtrgsel1(0)
        }
    }
    impl core::fmt::Debug for Comtrgsel1 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Comtrgsel1")
                .field("trgsel", &self.trgsel())
                .finish()
        }
    }
    #[doc = "desc COMTRGSEL2."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Comtrgsel2(pub u32);
    impl Comtrgsel2 {
        #[doc = "desc TRGSEL."]
        #[must_use]
        #[inline(always)]
        pub const fn trgsel(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x01ff;
            val as u16
        }
        #[doc = "desc TRGSEL."]
        #[inline(always)]
        pub const fn set_trgsel(&mut self, val: u16) {
            self.0 = (self.0 & !(0x01ff << 0usize)) | (((val as u32) & 0x01ff) << 0usize);
        }
    }
    impl Default for Comtrgsel2 {
        #[inline(always)]
        fn default() -> Comtrgsel2 {
            Comtrgsel2(0)
        }
    }
    impl core::fmt::Debug for Comtrgsel2 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Comtrgsel2")
                .field("trgsel", &self.trgsel())
                .finish()
        }
    }
    #[doc = "desc DMA_RC_TRGSEL."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct DmaRcTrgsel(pub u32);
    impl DmaRcTrgsel {
        #[doc = "desc TRGSEL."]
        #[must_use]
        #[inline(always)]
        pub const fn trgsel(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x01ff;
            val as u16
        }
        #[doc = "desc TRGSEL."]
        #[inline(always)]
        pub const fn set_trgsel(&mut self, val: u16) {
            self.0 = (self.0 & !(0x01ff << 0usize)) | (((val as u32) & 0x01ff) << 0usize);
        }
        #[doc = "desc COMEN."]
        #[must_use]
        #[inline(always)]
        pub const fn comen(&self) -> u8 {
            let val = (self.0 >> 30usize) & 0x03;
            val as u8
        }
        #[doc = "desc COMEN."]
        #[inline(always)]
        pub const fn set_comen(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 30usize)) | (((val as u32) & 0x03) << 30usize);
        }
    }
    impl Default for DmaRcTrgsel {
        #[inline(always)]
        fn default() -> DmaRcTrgsel {
            DmaRcTrgsel(0)
        }
    }
    impl core::fmt::Debug for DmaRcTrgsel {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("DmaRcTrgsel")
                .field("trgsel", &self.trgsel())
                .field("comen", &self.comen())
                .finish()
        }
    }
    #[doc = "desc DMA_TRGSEL0."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct DmaTrgsel0(pub u32);
    impl DmaTrgsel0 {
        #[doc = "desc TRGSEL."]
        #[must_use]
        #[inline(always)]
        pub const fn trgsel(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x01ff;
            val as u16
        }
        #[doc = "desc TRGSEL."]
        #[inline(always)]
        pub const fn set_trgsel(&mut self, val: u16) {
            self.0 = (self.0 & !(0x01ff << 0usize)) | (((val as u32) & 0x01ff) << 0usize);
        }
        #[doc = "desc COMEN."]
        #[must_use]
        #[inline(always)]
        pub const fn comen(&self) -> u8 {
            let val = (self.0 >> 30usize) & 0x03;
            val as u8
        }
        #[doc = "desc COMEN."]
        #[inline(always)]
        pub const fn set_comen(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 30usize)) | (((val as u32) & 0x03) << 30usize);
        }
    }
    impl Default for DmaTrgsel0 {
        #[inline(always)]
        fn default() -> DmaTrgsel0 {
            DmaTrgsel0(0)
        }
    }
    impl core::fmt::Debug for DmaTrgsel0 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("DmaTrgsel0")
                .field("trgsel", &self.trgsel())
                .field("comen", &self.comen())
                .finish()
        }
    }
    #[doc = "desc DMA_TRGSEL1."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct DmaTrgsel1(pub u32);
    impl DmaTrgsel1 {
        #[doc = "desc TRGSEL."]
        #[must_use]
        #[inline(always)]
        pub const fn trgsel(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x01ff;
            val as u16
        }
        #[doc = "desc TRGSEL."]
        #[inline(always)]
        pub const fn set_trgsel(&mut self, val: u16) {
            self.0 = (self.0 & !(0x01ff << 0usize)) | (((val as u32) & 0x01ff) << 0usize);
        }
        #[doc = "desc COMEN."]
        #[must_use]
        #[inline(always)]
        pub const fn comen(&self) -> u8 {
            let val = (self.0 >> 30usize) & 0x03;
            val as u8
        }
        #[doc = "desc COMEN."]
        #[inline(always)]
        pub const fn set_comen(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 30usize)) | (((val as u32) & 0x03) << 30usize);
        }
    }
    impl Default for DmaTrgsel1 {
        #[inline(always)]
        fn default() -> DmaTrgsel1 {
            DmaTrgsel1(0)
        }
    }
    impl core::fmt::Debug for DmaTrgsel1 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("DmaTrgsel1")
                .field("trgsel", &self.trgsel())
                .field("comen", &self.comen())
                .finish()
        }
    }
    #[doc = "desc INTSFTTRG."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Intsfttrg(pub u32);
    impl Intsfttrg {
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
    impl Default for Intsfttrg {
        #[inline(always)]
        fn default() -> Intsfttrg {
            Intsfttrg(0)
        }
    }
    impl core::fmt::Debug for Intsfttrg {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Intsfttrg")
                .field("strg", &self.strg())
                .finish()
        }
    }
}
