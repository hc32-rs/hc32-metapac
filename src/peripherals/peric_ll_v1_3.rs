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
    #[doc = "desc TMR_SYNENR."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct TmrSynenr(pub u32);
    impl TmrSynenr {
        #[doc = "desc TMR0U1A."]
        #[must_use]
        #[inline(always)]
        pub const fn tmr0u1a(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "desc TMR0U1A."]
        #[inline(always)]
        pub const fn set_tmr0u1a(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "desc TMR0U1B."]
        #[must_use]
        #[inline(always)]
        pub const fn tmr0u1b(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "desc TMR0U1B."]
        #[inline(always)]
        pub const fn set_tmr0u1b(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "desc TMR0U2A."]
        #[must_use]
        #[inline(always)]
        pub const fn tmr0u2a(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "desc TMR0U2A."]
        #[inline(always)]
        pub const fn set_tmr0u2a(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "desc TMR0U2B."]
        #[must_use]
        #[inline(always)]
        pub const fn tmr0u2b(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "desc TMR0U2B."]
        #[inline(always)]
        pub const fn set_tmr0u2b(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "desc TMR4U1."]
        #[must_use]
        #[inline(always)]
        pub const fn tmr4u1(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "desc TMR4U1."]
        #[inline(always)]
        pub const fn set_tmr4u1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "desc TMR6U1."]
        #[must_use]
        #[inline(always)]
        pub const fn tmr6u1(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "desc TMR6U1."]
        #[inline(always)]
        pub const fn set_tmr6u1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "desc TMR6U2."]
        #[must_use]
        #[inline(always)]
        pub const fn tmr6u2(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "desc TMR6U2."]
        #[inline(always)]
        pub const fn set_tmr6u2(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "desc TMR6U3."]
        #[must_use]
        #[inline(always)]
        pub const fn tmr6u3(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "desc TMR6U3."]
        #[inline(always)]
        pub const fn set_tmr6u3(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[doc = "desc TMR6U4."]
        #[must_use]
        #[inline(always)]
        pub const fn tmr6u4(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "desc TMR6U4."]
        #[inline(always)]
        pub const fn set_tmr6u4(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[doc = "desc TMRAU1."]
        #[must_use]
        #[inline(always)]
        pub const fn tmrau1(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "desc TMRAU1."]
        #[inline(always)]
        pub const fn set_tmrau1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "desc TMRAU2."]
        #[must_use]
        #[inline(always)]
        pub const fn tmrau2(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "desc TMRAU2."]
        #[inline(always)]
        pub const fn set_tmrau2(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[doc = "desc TMRAU3."]
        #[must_use]
        #[inline(always)]
        pub const fn tmrau3(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[doc = "desc TMRAU3."]
        #[inline(always)]
        pub const fn set_tmrau3(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
        #[doc = "desc TMRAU4."]
        #[must_use]
        #[inline(always)]
        pub const fn tmrau4(&self) -> bool {
            let val = (self.0 >> 19usize) & 0x01;
            val != 0
        }
        #[doc = "desc TMRAU4."]
        #[inline(always)]
        pub const fn set_tmrau4(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
        }
        #[doc = "desc TMRAU5."]
        #[must_use]
        #[inline(always)]
        pub const fn tmrau5(&self) -> bool {
            let val = (self.0 >> 20usize) & 0x01;
            val != 0
        }
        #[doc = "desc TMRAU5."]
        #[inline(always)]
        pub const fn set_tmrau5(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
        }
        #[doc = "desc HRPWMU1."]
        #[must_use]
        #[inline(always)]
        pub const fn hrpwmu1(&self) -> bool {
            let val = (self.0 >> 24usize) & 0x01;
            val != 0
        }
        #[doc = "desc HRPWMU1."]
        #[inline(always)]
        pub const fn set_hrpwmu1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
        }
        #[doc = "desc HRPWMU2."]
        #[must_use]
        #[inline(always)]
        pub const fn hrpwmu2(&self) -> bool {
            let val = (self.0 >> 25usize) & 0x01;
            val != 0
        }
        #[doc = "desc HRPWMU2."]
        #[inline(always)]
        pub const fn set_hrpwmu2(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
        }
        #[doc = "desc HRPWMU3."]
        #[must_use]
        #[inline(always)]
        pub const fn hrpwmu3(&self) -> bool {
            let val = (self.0 >> 26usize) & 0x01;
            val != 0
        }
        #[doc = "desc HRPWMU3."]
        #[inline(always)]
        pub const fn set_hrpwmu3(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
        }
        #[doc = "desc HRPWMU4."]
        #[must_use]
        #[inline(always)]
        pub const fn hrpwmu4(&self) -> bool {
            let val = (self.0 >> 27usize) & 0x01;
            val != 0
        }
        #[doc = "desc HRPWMU4."]
        #[inline(always)]
        pub const fn set_hrpwmu4(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
        }
        #[doc = "desc HRPWMU5."]
        #[must_use]
        #[inline(always)]
        pub const fn hrpwmu5(&self) -> bool {
            let val = (self.0 >> 28usize) & 0x01;
            val != 0
        }
        #[doc = "desc HRPWMU5."]
        #[inline(always)]
        pub const fn set_hrpwmu5(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
        }
        #[doc = "desc HRPWMU6."]
        #[must_use]
        #[inline(always)]
        pub const fn hrpwmu6(&self) -> bool {
            let val = (self.0 >> 29usize) & 0x01;
            val != 0
        }
        #[doc = "desc HRPWMU6."]
        #[inline(always)]
        pub const fn set_hrpwmu6(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
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
                .field("tmr0u1a", &self.tmr0u1a())
                .field("tmr0u1b", &self.tmr0u1b())
                .field("tmr0u2a", &self.tmr0u2a())
                .field("tmr0u2b", &self.tmr0u2b())
                .field("tmr4u1", &self.tmr4u1())
                .field("tmr6u1", &self.tmr6u1())
                .field("tmr6u2", &self.tmr6u2())
                .field("tmr6u3", &self.tmr6u3())
                .field("tmr6u4", &self.tmr6u4())
                .field("tmrau1", &self.tmrau1())
                .field("tmrau2", &self.tmrau2())
                .field("tmrau3", &self.tmrau3())
                .field("tmrau4", &self.tmrau4())
                .field("tmrau5", &self.tmrau5())
                .field("hrpwmu1", &self.hrpwmu1())
                .field("hrpwmu2", &self.hrpwmu2())
                .field("hrpwmu3", &self.hrpwmu3())
                .field("hrpwmu4", &self.hrpwmu4())
                .field("hrpwmu5", &self.hrpwmu5())
                .field("hrpwmu6", &self.hrpwmu6())
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
