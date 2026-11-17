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
    #[doc = "desc SMC_ENAR."]
    #[inline(always)]
    pub const fn smc_enar(self) -> crate::common::Reg<regs::SmcEnar, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0cusize) as _) }
    }
    #[doc = "desc TMR_SYNENR."]
    #[inline(always)]
    pub const fn tmr_synenr(self) -> crate::common::Reg<regs::TmrSynenr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x14usize) as _) }
    }
    #[doc = "desc USART1_NFC."]
    #[inline(always)]
    pub const fn usart1_nfc(self) -> crate::common::Reg<regs::Usart1Nfc, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1cusize) as _) }
    }
}
pub mod regs {
    #[doc = "desc SMC_ENAR."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct SmcEnar(pub u32);
    impl SmcEnar {
        #[doc = "desc SMCEN."]
        #[must_use]
        #[inline(always)]
        pub const fn smcen(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "desc SMCEN."]
        #[inline(always)]
        pub const fn set_smcen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
    }
    impl Default for SmcEnar {
        #[inline(always)]
        fn default() -> SmcEnar {
            SmcEnar(0)
        }
    }
    impl core::fmt::Debug for SmcEnar {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("SmcEnar")
                .field("smcen", &self.smcen())
                .finish()
        }
    }
    #[doc = "desc TMR_SYNENR."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct TmrSynenr(pub u32);
    impl TmrSynenr {
        #[doc = "desc TMR0_1_A."]
        #[must_use]
        #[inline(always)]
        pub const fn tmr0_1_a(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "desc TMR0_1_A."]
        #[inline(always)]
        pub const fn set_tmr0_1_a(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "desc TMR0_1_B."]
        #[must_use]
        #[inline(always)]
        pub const fn tmr0_1_b(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "desc TMR0_1_B."]
        #[inline(always)]
        pub const fn set_tmr0_1_b(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "desc TMR0_2_A."]
        #[must_use]
        #[inline(always)]
        pub const fn tmr0_2_a(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "desc TMR0_2_A."]
        #[inline(always)]
        pub const fn set_tmr0_2_a(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "desc TMR0_2_B."]
        #[must_use]
        #[inline(always)]
        pub const fn tmr0_2_b(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "desc TMR0_2_B."]
        #[inline(always)]
        pub const fn set_tmr0_2_b(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "desc TMR4_1."]
        #[must_use]
        #[inline(always)]
        pub const fn tmr4_1(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "desc TMR4_1."]
        #[inline(always)]
        pub const fn set_tmr4_1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "desc TMR4_2."]
        #[must_use]
        #[inline(always)]
        pub const fn tmr4_2(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "desc TMR4_2."]
        #[inline(always)]
        pub const fn set_tmr4_2(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "desc TMRA_1."]
        #[must_use]
        #[inline(always)]
        pub const fn tmra_1(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "desc TMRA_1."]
        #[inline(always)]
        pub const fn set_tmra_1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "desc TMRA_2."]
        #[must_use]
        #[inline(always)]
        pub const fn tmra_2(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "desc TMRA_2."]
        #[inline(always)]
        pub const fn set_tmra_2(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[doc = "desc TMRA_3."]
        #[must_use]
        #[inline(always)]
        pub const fn tmra_3(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[doc = "desc TMRA_3."]
        #[inline(always)]
        pub const fn set_tmra_3(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
        #[doc = "desc TMRA_4."]
        #[must_use]
        #[inline(always)]
        pub const fn tmra_4(&self) -> bool {
            let val = (self.0 >> 19usize) & 0x01;
            val != 0
        }
        #[doc = "desc TMRA_4."]
        #[inline(always)]
        pub const fn set_tmra_4(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
        }
        #[doc = "desc TMRA_5."]
        #[must_use]
        #[inline(always)]
        pub const fn tmra_5(&self) -> bool {
            let val = (self.0 >> 20usize) & 0x01;
            val != 0
        }
        #[doc = "desc TMRA_5."]
        #[inline(always)]
        pub const fn set_tmra_5(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
        }
    }
    impl Default for TmrSynenr {
        #[inline(always)]
        fn default() -> TmrSynenr {
            TmrSynenr(0)
        }
    }
    impl core::fmt::Debug for TmrSynenr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("TmrSynenr")
                .field("tmr0_1_a", &self.tmr0_1_a())
                .field("tmr0_1_b", &self.tmr0_1_b())
                .field("tmr0_2_a", &self.tmr0_2_a())
                .field("tmr0_2_b", &self.tmr0_2_b())
                .field("tmr4_1", &self.tmr4_1())
                .field("tmr4_2", &self.tmr4_2())
                .field("tmra_1", &self.tmra_1())
                .field("tmra_2", &self.tmra_2())
                .field("tmra_3", &self.tmra_3())
                .field("tmra_4", &self.tmra_4())
                .field("tmra_5", &self.tmra_5())
                .finish()
        }
    }
    #[doc = "desc USART1_NFC."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Usart1Nfc(pub u32);
    impl Usart1Nfc {
        #[doc = "desc USASRT1_NFS."]
        #[must_use]
        #[inline(always)]
        pub const fn usasrt1_nfs(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x03;
            val as u8
        }
        #[doc = "desc USASRT1_NFS."]
        #[inline(always)]
        pub const fn set_usasrt1_nfs(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 0usize)) | (((val as u32) & 0x03) << 0usize);
        }
        #[doc = "desc USART1_NFE."]
        #[must_use]
        #[inline(always)]
        pub const fn usart1_nfe(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "desc USART1_NFE."]
        #[inline(always)]
        pub const fn set_usart1_nfe(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
    }
    impl Default for Usart1Nfc {
        #[inline(always)]
        fn default() -> Usart1Nfc {
            Usart1Nfc(0)
        }
    }
    impl core::fmt::Debug for Usart1Nfc {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Usart1Nfc")
                .field("usasrt1_nfs", &self.usasrt1_nfs())
                .field("usart1_nfe", &self.usart1_nfe())
                .finish()
        }
    }
}
