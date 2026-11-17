#![allow(clippy::missing_safety_doc)]
#![allow(clippy::identity_op)]
#![allow(clippy::unnecessary_cast)]
#![allow(clippy::erasing_op)]

#[doc = "desc TMR6_COMMON."]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Tmr {
    ptr: *mut u8,
}
unsafe impl Send for Tmr {}
unsafe impl Sync for Tmr {}
impl Tmr {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "desc FCNTR."]
    #[inline(always)]
    pub const fn fcntr(self) -> crate::common::Reg<regs::Fcntr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xecusize) as _) }
    }
    #[doc = "desc SSTAR."]
    #[inline(always)]
    pub const fn sstar(self) -> crate::common::Reg<regs::Sstar, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xf0usize) as _) }
    }
    #[doc = "desc SSTPR."]
    #[inline(always)]
    pub const fn sstpr(self) -> crate::common::Reg<regs::Sstpr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xf4usize) as _) }
    }
    #[doc = "desc SCLRR."]
    #[inline(always)]
    pub const fn sclrr(self) -> crate::common::Reg<regs::Sclrr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xf8usize) as _) }
    }
    #[doc = "desc SUPDR."]
    #[inline(always)]
    pub const fn supdr(self) -> crate::common::Reg<regs::Supdr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xfcusize) as _) }
    }
}
pub mod regs {
    #[doc = "desc FCNTR."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Fcntr(pub u32);
    impl Fcntr {
        #[doc = "desc NOFIENTA."]
        #[must_use]
        #[inline(always)]
        pub const fn nofienta(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "desc NOFIENTA."]
        #[inline(always)]
        pub const fn set_nofienta(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "desc NOFICKTA."]
        #[must_use]
        #[inline(always)]
        pub const fn nofickta(&self) -> u8 {
            let val = (self.0 >> 1usize) & 0x03;
            val as u8
        }
        #[doc = "desc NOFICKTA."]
        #[inline(always)]
        pub const fn set_nofickta(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 1usize)) | (((val as u32) & 0x03) << 1usize);
        }
        #[doc = "desc NOFIENTB."]
        #[must_use]
        #[inline(always)]
        pub const fn nofientb(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "desc NOFIENTB."]
        #[inline(always)]
        pub const fn set_nofientb(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "desc NOFICKTB."]
        #[must_use]
        #[inline(always)]
        pub const fn noficktb(&self) -> u8 {
            let val = (self.0 >> 5usize) & 0x03;
            val as u8
        }
        #[doc = "desc NOFICKTB."]
        #[inline(always)]
        pub const fn set_noficktb(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 5usize)) | (((val as u32) & 0x03) << 5usize);
        }
        #[doc = "desc NOFIENTC."]
        #[must_use]
        #[inline(always)]
        pub const fn nofientc(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "desc NOFIENTC."]
        #[inline(always)]
        pub const fn set_nofientc(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "desc NOFICKTC."]
        #[must_use]
        #[inline(always)]
        pub const fn noficktc(&self) -> u8 {
            let val = (self.0 >> 9usize) & 0x03;
            val as u8
        }
        #[doc = "desc NOFICKTC."]
        #[inline(always)]
        pub const fn set_noficktc(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 9usize)) | (((val as u32) & 0x03) << 9usize);
        }
        #[doc = "desc NOFIENTD."]
        #[must_use]
        #[inline(always)]
        pub const fn nofientd(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "desc NOFIENTD."]
        #[inline(always)]
        pub const fn set_nofientd(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "desc NOFICKTD."]
        #[must_use]
        #[inline(always)]
        pub const fn noficktd(&self) -> u8 {
            let val = (self.0 >> 13usize) & 0x03;
            val as u8
        }
        #[doc = "desc NOFICKTD."]
        #[inline(always)]
        pub const fn set_noficktd(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 13usize)) | (((val as u32) & 0x03) << 13usize);
        }
    }
    impl Default for Fcntr {
        #[inline(always)]
        fn default() -> Fcntr {
            Fcntr(0)
        }
    }
    impl core::fmt::Debug for Fcntr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Fcntr")
                .field("nofienta", &self.nofienta())
                .field("nofickta", &self.nofickta())
                .field("nofientb", &self.nofientb())
                .field("noficktb", &self.noficktb())
                .field("nofientc", &self.nofientc())
                .field("noficktc", &self.noficktc())
                .field("nofientd", &self.nofientd())
                .field("noficktd", &self.noficktd())
                .finish()
        }
    }
    #[doc = "desc SCLRR."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Sclrr(pub u32);
    impl Sclrr {
        #[doc = "desc SCLE1."]
        #[must_use]
        #[inline(always)]
        pub const fn scle1(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "desc SCLE1."]
        #[inline(always)]
        pub const fn set_scle1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "desc SCLE2."]
        #[must_use]
        #[inline(always)]
        pub const fn scle2(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "desc SCLE2."]
        #[inline(always)]
        pub const fn set_scle2(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "desc SCLE3."]
        #[must_use]
        #[inline(always)]
        pub const fn scle3(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "desc SCLE3."]
        #[inline(always)]
        pub const fn set_scle3(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "desc SCLE4."]
        #[must_use]
        #[inline(always)]
        pub const fn scle4(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "desc SCLE4."]
        #[inline(always)]
        pub const fn set_scle4(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "desc SCLE5."]
        #[must_use]
        #[inline(always)]
        pub const fn scle5(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "desc SCLE5."]
        #[inline(always)]
        pub const fn set_scle5(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "desc SCLE6."]
        #[must_use]
        #[inline(always)]
        pub const fn scle6(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "desc SCLE6."]
        #[inline(always)]
        pub const fn set_scle6(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "desc SCLE7."]
        #[must_use]
        #[inline(always)]
        pub const fn scle7(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "desc SCLE7."]
        #[inline(always)]
        pub const fn set_scle7(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "desc SCLE8."]
        #[must_use]
        #[inline(always)]
        pub const fn scle8(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "desc SCLE8."]
        #[inline(always)]
        pub const fn set_scle8(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
    }
    impl Default for Sclrr {
        #[inline(always)]
        fn default() -> Sclrr {
            Sclrr(0)
        }
    }
    impl core::fmt::Debug for Sclrr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Sclrr")
                .field("scle1", &self.scle1())
                .field("scle2", &self.scle2())
                .field("scle3", &self.scle3())
                .field("scle4", &self.scle4())
                .field("scle5", &self.scle5())
                .field("scle6", &self.scle6())
                .field("scle7", &self.scle7())
                .field("scle8", &self.scle8())
                .finish()
        }
    }
    #[doc = "desc SSTAR."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Sstar(pub u32);
    impl Sstar {
        #[doc = "desc SSTA1."]
        #[must_use]
        #[inline(always)]
        pub const fn ssta1(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "desc SSTA1."]
        #[inline(always)]
        pub const fn set_ssta1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "desc SSTA2."]
        #[must_use]
        #[inline(always)]
        pub const fn ssta2(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "desc SSTA2."]
        #[inline(always)]
        pub const fn set_ssta2(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "desc SSTA3."]
        #[must_use]
        #[inline(always)]
        pub const fn ssta3(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "desc SSTA3."]
        #[inline(always)]
        pub const fn set_ssta3(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "desc SSTA4."]
        #[must_use]
        #[inline(always)]
        pub const fn ssta4(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "desc SSTA4."]
        #[inline(always)]
        pub const fn set_ssta4(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "desc SSTA5."]
        #[must_use]
        #[inline(always)]
        pub const fn ssta5(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "desc SSTA5."]
        #[inline(always)]
        pub const fn set_ssta5(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "desc SSTA6."]
        #[must_use]
        #[inline(always)]
        pub const fn ssta6(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "desc SSTA6."]
        #[inline(always)]
        pub const fn set_ssta6(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "desc SSTA7."]
        #[must_use]
        #[inline(always)]
        pub const fn ssta7(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "desc SSTA7."]
        #[inline(always)]
        pub const fn set_ssta7(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "desc SSTA8."]
        #[must_use]
        #[inline(always)]
        pub const fn ssta8(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "desc SSTA8."]
        #[inline(always)]
        pub const fn set_ssta8(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
    }
    impl Default for Sstar {
        #[inline(always)]
        fn default() -> Sstar {
            Sstar(0)
        }
    }
    impl core::fmt::Debug for Sstar {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Sstar")
                .field("ssta1", &self.ssta1())
                .field("ssta2", &self.ssta2())
                .field("ssta3", &self.ssta3())
                .field("ssta4", &self.ssta4())
                .field("ssta5", &self.ssta5())
                .field("ssta6", &self.ssta6())
                .field("ssta7", &self.ssta7())
                .field("ssta8", &self.ssta8())
                .finish()
        }
    }
    #[doc = "desc SSTPR."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Sstpr(pub u32);
    impl Sstpr {
        #[doc = "desc SSTP1."]
        #[must_use]
        #[inline(always)]
        pub const fn sstp1(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "desc SSTP1."]
        #[inline(always)]
        pub const fn set_sstp1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "desc SSTP2."]
        #[must_use]
        #[inline(always)]
        pub const fn sstp2(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "desc SSTP2."]
        #[inline(always)]
        pub const fn set_sstp2(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "desc SSTP3."]
        #[must_use]
        #[inline(always)]
        pub const fn sstp3(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "desc SSTP3."]
        #[inline(always)]
        pub const fn set_sstp3(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "desc SSTP4."]
        #[must_use]
        #[inline(always)]
        pub const fn sstp4(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "desc SSTP4."]
        #[inline(always)]
        pub const fn set_sstp4(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "desc SSTP5."]
        #[must_use]
        #[inline(always)]
        pub const fn sstp5(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "desc SSTP5."]
        #[inline(always)]
        pub const fn set_sstp5(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "desc SSTP6."]
        #[must_use]
        #[inline(always)]
        pub const fn sstp6(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "desc SSTP6."]
        #[inline(always)]
        pub const fn set_sstp6(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "desc SSTP7."]
        #[must_use]
        #[inline(always)]
        pub const fn sstp7(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "desc SSTP7."]
        #[inline(always)]
        pub const fn set_sstp7(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "desc SSTP8."]
        #[must_use]
        #[inline(always)]
        pub const fn sstp8(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "desc SSTP8."]
        #[inline(always)]
        pub const fn set_sstp8(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
    }
    impl Default for Sstpr {
        #[inline(always)]
        fn default() -> Sstpr {
            Sstpr(0)
        }
    }
    impl core::fmt::Debug for Sstpr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Sstpr")
                .field("sstp1", &self.sstp1())
                .field("sstp2", &self.sstp2())
                .field("sstp3", &self.sstp3())
                .field("sstp4", &self.sstp4())
                .field("sstp5", &self.sstp5())
                .field("sstp6", &self.sstp6())
                .field("sstp7", &self.sstp7())
                .field("sstp8", &self.sstp8())
                .finish()
        }
    }
    #[doc = "desc SUPDR."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Supdr(pub u32);
    impl Supdr {
        #[doc = "desc SUPD1."]
        #[must_use]
        #[inline(always)]
        pub const fn supd1(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "desc SUPD1."]
        #[inline(always)]
        pub const fn set_supd1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "desc SUPD2."]
        #[must_use]
        #[inline(always)]
        pub const fn supd2(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "desc SUPD2."]
        #[inline(always)]
        pub const fn set_supd2(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "desc SUPD3."]
        #[must_use]
        #[inline(always)]
        pub const fn supd3(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "desc SUPD3."]
        #[inline(always)]
        pub const fn set_supd3(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "desc SUPD4."]
        #[must_use]
        #[inline(always)]
        pub const fn supd4(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "desc SUPD4."]
        #[inline(always)]
        pub const fn set_supd4(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "desc SUPD5."]
        #[must_use]
        #[inline(always)]
        pub const fn supd5(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "desc SUPD5."]
        #[inline(always)]
        pub const fn set_supd5(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "desc SUPD6."]
        #[must_use]
        #[inline(always)]
        pub const fn supd6(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "desc SUPD6."]
        #[inline(always)]
        pub const fn set_supd6(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "desc SUPD7."]
        #[must_use]
        #[inline(always)]
        pub const fn supd7(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "desc SUPD7."]
        #[inline(always)]
        pub const fn set_supd7(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "desc SUPD8."]
        #[must_use]
        #[inline(always)]
        pub const fn supd8(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "desc SUPD8."]
        #[inline(always)]
        pub const fn set_supd8(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
    }
    impl Default for Supdr {
        #[inline(always)]
        fn default() -> Supdr {
            Supdr(0)
        }
    }
    impl core::fmt::Debug for Supdr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Supdr")
                .field("supd1", &self.supd1())
                .field("supd2", &self.supd2())
                .field("supd3", &self.supd3())
                .field("supd4", &self.supd4())
                .field("supd5", &self.supd5())
                .field("supd6", &self.supd6())
                .field("supd7", &self.supd7())
                .field("supd8", &self.supd8())
                .finish()
        }
    }
}
