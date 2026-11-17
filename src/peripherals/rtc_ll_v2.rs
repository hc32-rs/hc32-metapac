#![allow(clippy::missing_safety_doc)]
#![allow(clippy::identity_op)]
#![allow(clippy::unnecessary_cast)]
#![allow(clippy::erasing_op)]

#[doc = "desc RTC."]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Rtc {
    ptr: *mut u8,
}
unsafe impl Send for Rtc {}
unsafe impl Sync for Rtc {}
impl Rtc {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "desc CR0."]
    #[inline(always)]
    pub const fn cr0(self) -> crate::common::Reg<regs::Cr0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0usize) as _) }
    }
    #[doc = "desc CR1."]
    #[inline(always)]
    pub const fn cr1(self) -> crate::common::Reg<regs::Cr1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x04usize) as _) }
    }
    #[doc = "desc SEC."]
    #[inline(always)]
    pub const fn sec(self) -> crate::common::Reg<regs::Sec, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x08usize) as _) }
    }
    #[doc = "desc MIN."]
    #[inline(always)]
    pub const fn min(self) -> crate::common::Reg<regs::Min, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0cusize) as _) }
    }
    #[doc = "desc HOUR."]
    #[inline(always)]
    pub const fn hour(self) -> crate::common::Reg<regs::Hour, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x10usize) as _) }
    }
    #[doc = "desc WEEK."]
    #[inline(always)]
    pub const fn week(self) -> crate::common::Reg<regs::Week, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x14usize) as _) }
    }
    #[doc = "desc DAY."]
    #[inline(always)]
    pub const fn day(self) -> crate::common::Reg<regs::Day, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x18usize) as _) }
    }
    #[doc = "desc MON."]
    #[inline(always)]
    pub const fn mon(self) -> crate::common::Reg<regs::Mon, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1cusize) as _) }
    }
    #[doc = "desc YEAR."]
    #[inline(always)]
    pub const fn year(self) -> crate::common::Reg<regs::Year, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x20usize) as _) }
    }
    #[doc = "desc COMPEN."]
    #[inline(always)]
    pub const fn compen(self) -> crate::common::Reg<regs::Compen, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x30usize) as _) }
    }
    #[doc = "desc HCOMP."]
    #[inline(always)]
    pub const fn hcomp(self) -> crate::common::Reg<regs::Hcomp, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x34usize) as _) }
    }
    #[doc = "desc CR2."]
    #[inline(always)]
    pub const fn cr2(self) -> crate::common::Reg<regs::Cr2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x3cusize) as _) }
    }
    #[doc = "desc ALMA."]
    #[inline(always)]
    pub const fn alma(self) -> crate::common::Reg<regs::Alma, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x40usize) as _) }
    }
    #[doc = "desc ALMB."]
    #[inline(always)]
    pub const fn almb(self) -> crate::common::Reg<regs::Almb, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x44usize) as _) }
    }
    #[doc = "desc TSTR."]
    #[inline(always)]
    pub const fn tstr(self) -> crate::common::Reg<regs::Tstr, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x48usize) as _) }
    }
    #[doc = "desc TSDR."]
    #[inline(always)]
    pub const fn tsdr(self) -> crate::common::Reg<regs::Tsdr, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x4cusize) as _) }
    }
    #[doc = "desc WUTR."]
    #[inline(always)]
    pub const fn wutr(self) -> crate::common::Reg<regs::Wutr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x50usize) as _) }
    }
    #[doc = "desc IER."]
    #[inline(always)]
    pub const fn ier(self) -> crate::common::Reg<regs::Ier, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x54usize) as _) }
    }
    #[doc = "desc IFR."]
    #[inline(always)]
    pub const fn ifr(self) -> crate::common::Reg<regs::Ifr, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x58usize) as _) }
    }
    #[doc = "desc ICR."]
    #[inline(always)]
    pub const fn icr(self) -> crate::common::Reg<regs::Icr, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x5cusize) as _) }
    }
    #[doc = "desc WPR."]
    #[inline(always)]
    pub const fn wpr(self) -> crate::common::Reg<regs::Wpr, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x60usize) as _) }
    }
}
pub mod regs {
    #[doc = "desc ALMA."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Alma(pub u32);
    impl Alma {
        #[doc = "desc SECL."]
        #[must_use]
        #[inline(always)]
        pub const fn secl(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x0f;
            val as u8
        }
        #[doc = "desc SECL."]
        #[inline(always)]
        pub const fn set_secl(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
        }
        #[doc = "desc SECH."]
        #[must_use]
        #[inline(always)]
        pub const fn sech(&self) -> u8 {
            let val = (self.0 >> 4usize) & 0x07;
            val as u8
        }
        #[doc = "desc SECH."]
        #[inline(always)]
        pub const fn set_sech(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 4usize)) | (((val as u32) & 0x07) << 4usize);
        }
        #[doc = "desc MSKS."]
        #[must_use]
        #[inline(always)]
        pub const fn msks(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "desc MSKS."]
        #[inline(always)]
        pub const fn set_msks(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "desc MINL."]
        #[must_use]
        #[inline(always)]
        pub const fn minl(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0x0f;
            val as u8
        }
        #[doc = "desc MINL."]
        #[inline(always)]
        pub const fn set_minl(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 8usize)) | (((val as u32) & 0x0f) << 8usize);
        }
        #[doc = "desc MINH."]
        #[must_use]
        #[inline(always)]
        pub const fn minh(&self) -> u8 {
            let val = (self.0 >> 12usize) & 0x07;
            val as u8
        }
        #[doc = "desc MINH."]
        #[inline(always)]
        pub const fn set_minh(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 12usize)) | (((val as u32) & 0x07) << 12usize);
        }
        #[doc = "desc MSKM."]
        #[must_use]
        #[inline(always)]
        pub const fn mskm(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "desc MSKM."]
        #[inline(always)]
        pub const fn set_mskm(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
        #[doc = "desc HOURL."]
        #[must_use]
        #[inline(always)]
        pub const fn hourl(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x0f;
            val as u8
        }
        #[doc = "desc HOURL."]
        #[inline(always)]
        pub const fn set_hourl(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 16usize)) | (((val as u32) & 0x0f) << 16usize);
        }
        #[doc = "desc HOURH."]
        #[must_use]
        #[inline(always)]
        pub const fn hourh(&self) -> u8 {
            let val = (self.0 >> 20usize) & 0x03;
            val as u8
        }
        #[doc = "desc HOURH."]
        #[inline(always)]
        pub const fn set_hourh(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 20usize)) | (((val as u32) & 0x03) << 20usize);
        }
        #[doc = "desc MSKH."]
        #[must_use]
        #[inline(always)]
        pub const fn mskh(&self) -> bool {
            let val = (self.0 >> 23usize) & 0x01;
            val != 0
        }
        #[doc = "desc MSKH."]
        #[inline(always)]
        pub const fn set_mskh(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
        }
        #[doc = "desc WEEK."]
        #[must_use]
        #[inline(always)]
        pub const fn week(&self) -> u8 {
            let val = (self.0 >> 24usize) & 0x7f;
            val as u8
        }
        #[doc = "desc WEEK."]
        #[inline(always)]
        pub const fn set_week(&mut self, val: u8) {
            self.0 = (self.0 & !(0x7f << 24usize)) | (((val as u32) & 0x7f) << 24usize);
        }
    }
    impl Default for Alma {
        #[inline(always)]
        fn default() -> Alma {
            Alma(0)
        }
    }
    impl core::fmt::Debug for Alma {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Alma")
                .field("secl", &self.secl())
                .field("sech", &self.sech())
                .field("msks", &self.msks())
                .field("minl", &self.minl())
                .field("minh", &self.minh())
                .field("mskm", &self.mskm())
                .field("hourl", &self.hourl())
                .field("hourh", &self.hourh())
                .field("mskh", &self.mskh())
                .field("week", &self.week())
                .finish()
        }
    }
    #[doc = "desc ALMB."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Almb(pub u32);
    impl Almb {
        #[doc = "desc SECL."]
        #[must_use]
        #[inline(always)]
        pub const fn secl(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x0f;
            val as u8
        }
        #[doc = "desc SECL."]
        #[inline(always)]
        pub const fn set_secl(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
        }
        #[doc = "desc SECH."]
        #[must_use]
        #[inline(always)]
        pub const fn sech(&self) -> u8 {
            let val = (self.0 >> 4usize) & 0x07;
            val as u8
        }
        #[doc = "desc SECH."]
        #[inline(always)]
        pub const fn set_sech(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 4usize)) | (((val as u32) & 0x07) << 4usize);
        }
        #[doc = "desc MSKS."]
        #[must_use]
        #[inline(always)]
        pub const fn msks(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "desc MSKS."]
        #[inline(always)]
        pub const fn set_msks(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "desc MINL."]
        #[must_use]
        #[inline(always)]
        pub const fn minl(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0x0f;
            val as u8
        }
        #[doc = "desc MINL."]
        #[inline(always)]
        pub const fn set_minl(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 8usize)) | (((val as u32) & 0x0f) << 8usize);
        }
        #[doc = "desc MINH."]
        #[must_use]
        #[inline(always)]
        pub const fn minh(&self) -> u8 {
            let val = (self.0 >> 12usize) & 0x07;
            val as u8
        }
        #[doc = "desc MINH."]
        #[inline(always)]
        pub const fn set_minh(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 12usize)) | (((val as u32) & 0x07) << 12usize);
        }
        #[doc = "desc MSKM."]
        #[must_use]
        #[inline(always)]
        pub const fn mskm(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "desc MSKM."]
        #[inline(always)]
        pub const fn set_mskm(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
        #[doc = "desc HOURL."]
        #[must_use]
        #[inline(always)]
        pub const fn hourl(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x0f;
            val as u8
        }
        #[doc = "desc HOURL."]
        #[inline(always)]
        pub const fn set_hourl(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 16usize)) | (((val as u32) & 0x0f) << 16usize);
        }
        #[doc = "desc HOURH."]
        #[must_use]
        #[inline(always)]
        pub const fn hourh(&self) -> u8 {
            let val = (self.0 >> 20usize) & 0x03;
            val as u8
        }
        #[doc = "desc HOURH."]
        #[inline(always)]
        pub const fn set_hourh(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 20usize)) | (((val as u32) & 0x03) << 20usize);
        }
        #[doc = "desc MSKH."]
        #[must_use]
        #[inline(always)]
        pub const fn mskh(&self) -> bool {
            let val = (self.0 >> 23usize) & 0x01;
            val != 0
        }
        #[doc = "desc MSKH."]
        #[inline(always)]
        pub const fn set_mskh(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
        }
        #[doc = "desc WEEK."]
        #[must_use]
        #[inline(always)]
        pub const fn week(&self) -> u8 {
            let val = (self.0 >> 24usize) & 0x7f;
            val as u8
        }
        #[doc = "desc WEEK."]
        #[inline(always)]
        pub const fn set_week(&mut self, val: u8) {
            self.0 = (self.0 & !(0x7f << 24usize)) | (((val as u32) & 0x7f) << 24usize);
        }
    }
    impl Default for Almb {
        #[inline(always)]
        fn default() -> Almb {
            Almb(0)
        }
    }
    impl core::fmt::Debug for Almb {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Almb")
                .field("secl", &self.secl())
                .field("sech", &self.sech())
                .field("msks", &self.msks())
                .field("minl", &self.minl())
                .field("minh", &self.minh())
                .field("mskm", &self.mskm())
                .field("hourl", &self.hourl())
                .field("hourh", &self.hourh())
                .field("mskh", &self.mskh())
                .field("week", &self.week())
                .finish()
        }
    }
    #[doc = "desc COMPEN."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Compen(pub u32);
    impl Compen {
        #[doc = "desc CR."]
        #[must_use]
        #[inline(always)]
        pub const fn cr(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x1fff;
            val as u16
        }
        #[doc = "desc CR."]
        #[inline(always)]
        pub const fn set_cr(&mut self, val: u16) {
            self.0 = (self.0 & !(0x1fff << 0usize)) | (((val as u32) & 0x1fff) << 0usize);
        }
        #[doc = "desc CSTEPS."]
        #[must_use]
        #[inline(always)]
        pub const fn csteps(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "desc CSTEPS."]
        #[inline(always)]
        pub const fn set_csteps(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[doc = "desc CREN."]
        #[must_use]
        #[inline(always)]
        pub const fn cren(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "desc CREN."]
        #[inline(always)]
        pub const fn set_cren(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
    }
    impl Default for Compen {
        #[inline(always)]
        fn default() -> Compen {
            Compen(0)
        }
    }
    impl core::fmt::Debug for Compen {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Compen")
                .field("cr", &self.cr())
                .field("csteps", &self.csteps())
                .field("cren", &self.cren())
                .finish()
        }
    }
    #[doc = "desc CR0."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Cr0(pub u32);
    impl Cr0 {
        #[doc = "desc PRDS."]
        #[must_use]
        #[inline(always)]
        pub const fn prds(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x07;
            val as u8
        }
        #[doc = "desc PRDS."]
        #[inline(always)]
        pub const fn set_prds(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 0usize)) | (((val as u32) & 0x07) << 0usize);
        }
        #[doc = "desc AMPM."]
        #[must_use]
        #[inline(always)]
        pub const fn ampm(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "desc AMPM."]
        #[inline(always)]
        pub const fn set_ampm(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "desc HZ1OE."]
        #[must_use]
        #[inline(always)]
        pub const fn hz1oe(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "desc HZ1OE."]
        #[inline(always)]
        pub const fn set_hz1oe(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "desc HZ1SEL."]
        #[must_use]
        #[inline(always)]
        pub const fn hz1sel(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "desc HZ1SEL."]
        #[inline(always)]
        pub const fn set_hz1sel(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "desc START."]
        #[must_use]
        #[inline(always)]
        pub const fn start(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "desc START."]
        #[inline(always)]
        pub const fn set_start(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "desc PRDX."]
        #[must_use]
        #[inline(always)]
        pub const fn prdx(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0x3f;
            val as u8
        }
        #[doc = "desc PRDX."]
        #[inline(always)]
        pub const fn set_prdx(&mut self, val: u8) {
            self.0 = (self.0 & !(0x3f << 8usize)) | (((val as u32) & 0x3f) << 8usize);
        }
        #[doc = "desc PRDSEL."]
        #[must_use]
        #[inline(always)]
        pub const fn prdsel(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "desc PRDSEL."]
        #[inline(always)]
        pub const fn set_prdsel(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[doc = "desc TEST."]
        #[must_use]
        #[inline(always)]
        pub const fn test(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "desc TEST."]
        #[inline(always)]
        pub const fn set_test(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
    }
    impl Default for Cr0 {
        #[inline(always)]
        fn default() -> Cr0 {
            Cr0(0)
        }
    }
    impl core::fmt::Debug for Cr0 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Cr0")
                .field("prds", &self.prds())
                .field("ampm", &self.ampm())
                .field("hz1oe", &self.hz1oe())
                .field("hz1sel", &self.hz1sel())
                .field("start", &self.start())
                .field("prdx", &self.prdx())
                .field("prdsel", &self.prdsel())
                .field("test", &self.test())
                .finish()
        }
    }
    #[doc = "desc CR1."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Cr1(pub u32);
    impl Cr1 {
        #[doc = "desc WAIT."]
        #[must_use]
        #[inline(always)]
        pub const fn wait(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "desc WAIT."]
        #[inline(always)]
        pub const fn set_wait(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "desc WAITF."]
        #[must_use]
        #[inline(always)]
        pub const fn waitf(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "desc WAITF."]
        #[inline(always)]
        pub const fn set_waitf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
    }
    impl Default for Cr1 {
        #[inline(always)]
        fn default() -> Cr1 {
            Cr1(0)
        }
    }
    impl core::fmt::Debug for Cr1 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Cr1")
                .field("wait", &self.wait())
                .field("waitf", &self.waitf())
                .finish()
        }
    }
    #[doc = "desc CR2."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Cr2(pub u32);
    impl Cr2 {
        #[doc = "desc WUCKSEL."]
        #[must_use]
        #[inline(always)]
        pub const fn wucksel(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x07;
            val as u8
        }
        #[doc = "desc WUCKSEL."]
        #[inline(always)]
        pub const fn set_wucksel(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 0usize)) | (((val as u32) & 0x07) << 0usize);
        }
        #[doc = "desc TSEDG."]
        #[must_use]
        #[inline(always)]
        pub const fn tsedg(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "desc TSEDG."]
        #[inline(always)]
        pub const fn set_tsedg(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "desc O2SEL."]
        #[must_use]
        #[inline(always)]
        pub const fn o2sel(&self) -> u8 {
            let val = (self.0 >> 4usize) & 0x03;
            val as u8
        }
        #[doc = "desc O2SEL."]
        #[inline(always)]
        pub const fn set_o2sel(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 4usize)) | (((val as u32) & 0x03) << 4usize);
        }
        #[doc = "desc TSE."]
        #[must_use]
        #[inline(always)]
        pub const fn tse(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "desc TSE."]
        #[inline(always)]
        pub const fn set_tse(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "desc WUTE."]
        #[must_use]
        #[inline(always)]
        pub const fn wute(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "desc WUTE."]
        #[inline(always)]
        pub const fn set_wute(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "desc ALMAE."]
        #[must_use]
        #[inline(always)]
        pub const fn almae(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "desc ALMAE."]
        #[inline(always)]
        pub const fn set_almae(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "desc ALMBE."]
        #[must_use]
        #[inline(always)]
        pub const fn almbe(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "desc ALMBE."]
        #[inline(always)]
        pub const fn set_almbe(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
    }
    impl Default for Cr2 {
        #[inline(always)]
        fn default() -> Cr2 {
            Cr2(0)
        }
    }
    impl core::fmt::Debug for Cr2 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Cr2")
                .field("wucksel", &self.wucksel())
                .field("tsedg", &self.tsedg())
                .field("o2sel", &self.o2sel())
                .field("tse", &self.tse())
                .field("wute", &self.wute())
                .field("almae", &self.almae())
                .field("almbe", &self.almbe())
                .finish()
        }
    }
    #[doc = "desc DAY."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Day(pub u32);
    impl Day {
        #[doc = "desc DAYL."]
        #[must_use]
        #[inline(always)]
        pub const fn dayl(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x0f;
            val as u8
        }
        #[doc = "desc DAYL."]
        #[inline(always)]
        pub const fn set_dayl(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
        }
        #[doc = "desc DAYH."]
        #[must_use]
        #[inline(always)]
        pub const fn dayh(&self) -> u8 {
            let val = (self.0 >> 4usize) & 0x03;
            val as u8
        }
        #[doc = "desc DAYH."]
        #[inline(always)]
        pub const fn set_dayh(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 4usize)) | (((val as u32) & 0x03) << 4usize);
        }
    }
    impl Default for Day {
        #[inline(always)]
        fn default() -> Day {
            Day(0)
        }
    }
    impl core::fmt::Debug for Day {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Day")
                .field("dayl", &self.dayl())
                .field("dayh", &self.dayh())
                .finish()
        }
    }
    #[doc = "desc HCOMP."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Hcomp(pub u32);
    impl Hcomp {
        #[doc = "desc PARA."]
        #[must_use]
        #[inline(always)]
        pub const fn para(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x3f;
            val as u8
        }
        #[doc = "desc PARA."]
        #[inline(always)]
        pub const fn set_para(&mut self, val: u8) {
            self.0 = (self.0 & !(0x3f << 0usize)) | (((val as u32) & 0x3f) << 0usize);
        }
    }
    impl Default for Hcomp {
        #[inline(always)]
        fn default() -> Hcomp {
            Hcomp(0)
        }
    }
    impl core::fmt::Debug for Hcomp {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Hcomp").field("para", &self.para()).finish()
        }
    }
    #[doc = "desc HOUR."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Hour(pub u32);
    impl Hour {
        #[doc = "desc HOURL."]
        #[must_use]
        #[inline(always)]
        pub const fn hourl(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x0f;
            val as u8
        }
        #[doc = "desc HOURL."]
        #[inline(always)]
        pub const fn set_hourl(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
        }
        #[doc = "desc HOURH."]
        #[must_use]
        #[inline(always)]
        pub const fn hourh(&self) -> u8 {
            let val = (self.0 >> 4usize) & 0x03;
            val as u8
        }
        #[doc = "desc HOURH."]
        #[inline(always)]
        pub const fn set_hourh(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 4usize)) | (((val as u32) & 0x03) << 4usize);
        }
    }
    impl Default for Hour {
        #[inline(always)]
        fn default() -> Hour {
            Hour(0)
        }
    }
    impl core::fmt::Debug for Hour {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Hour")
                .field("hourl", &self.hourl())
                .field("hourh", &self.hourh())
                .finish()
        }
    }
    #[doc = "desc ICR."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Icr(pub u32);
    impl Icr {
        #[doc = "desc ALMA."]
        #[must_use]
        #[inline(always)]
        pub const fn alma(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "desc ALMA."]
        #[inline(always)]
        pub const fn set_alma(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "desc ALMB."]
        #[must_use]
        #[inline(always)]
        pub const fn almb(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "desc ALMB."]
        #[inline(always)]
        pub const fn set_almb(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "desc WU."]
        #[must_use]
        #[inline(always)]
        pub const fn wu(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "desc WU."]
        #[inline(always)]
        pub const fn set_wu(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "desc TS."]
        #[must_use]
        #[inline(always)]
        pub const fn ts(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "desc TS."]
        #[inline(always)]
        pub const fn set_ts(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "desc TSOV."]
        #[must_use]
        #[inline(always)]
        pub const fn tsov(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "desc TSOV."]
        #[inline(always)]
        pub const fn set_tsov(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "desc PRD."]
        #[must_use]
        #[inline(always)]
        pub const fn prd(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "desc PRD."]
        #[inline(always)]
        pub const fn set_prd(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
    }
    impl Default for Icr {
        #[inline(always)]
        fn default() -> Icr {
            Icr(0)
        }
    }
    impl core::fmt::Debug for Icr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Icr")
                .field("alma", &self.alma())
                .field("almb", &self.almb())
                .field("wu", &self.wu())
                .field("ts", &self.ts())
                .field("tsov", &self.tsov())
                .field("prd", &self.prd())
                .finish()
        }
    }
    #[doc = "desc IER."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ier(pub u32);
    impl Ier {
        #[doc = "desc ALMA."]
        #[must_use]
        #[inline(always)]
        pub const fn alma(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "desc ALMA."]
        #[inline(always)]
        pub const fn set_alma(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "desc ALMB."]
        #[must_use]
        #[inline(always)]
        pub const fn almb(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "desc ALMB."]
        #[inline(always)]
        pub const fn set_almb(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "desc WU."]
        #[must_use]
        #[inline(always)]
        pub const fn wu(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "desc WU."]
        #[inline(always)]
        pub const fn set_wu(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "desc TS."]
        #[must_use]
        #[inline(always)]
        pub const fn ts(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "desc TS."]
        #[inline(always)]
        pub const fn set_ts(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "desc PRD."]
        #[must_use]
        #[inline(always)]
        pub const fn prd(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "desc PRD."]
        #[inline(always)]
        pub const fn set_prd(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
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
                .field("alma", &self.alma())
                .field("almb", &self.almb())
                .field("wu", &self.wu())
                .field("ts", &self.ts())
                .field("prd", &self.prd())
                .finish()
        }
    }
    #[doc = "desc IFR."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ifr(pub u32);
    impl Ifr {
        #[doc = "desc ALMA."]
        #[must_use]
        #[inline(always)]
        pub const fn alma(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "desc ALMA."]
        #[inline(always)]
        pub const fn set_alma(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "desc ALMB."]
        #[must_use]
        #[inline(always)]
        pub const fn almb(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "desc ALMB."]
        #[inline(always)]
        pub const fn set_almb(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "desc WU."]
        #[must_use]
        #[inline(always)]
        pub const fn wu(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "desc WU."]
        #[inline(always)]
        pub const fn set_wu(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "desc TS."]
        #[must_use]
        #[inline(always)]
        pub const fn ts(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "desc TS."]
        #[inline(always)]
        pub const fn set_ts(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "desc TSOV."]
        #[must_use]
        #[inline(always)]
        pub const fn tsov(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "desc TSOV."]
        #[inline(always)]
        pub const fn set_tsov(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "desc PRD."]
        #[must_use]
        #[inline(always)]
        pub const fn prd(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "desc PRD."]
        #[inline(always)]
        pub const fn set_prd(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
    }
    impl Default for Ifr {
        #[inline(always)]
        fn default() -> Ifr {
            Ifr(0)
        }
    }
    impl core::fmt::Debug for Ifr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Ifr")
                .field("alma", &self.alma())
                .field("almb", &self.almb())
                .field("wu", &self.wu())
                .field("ts", &self.ts())
                .field("tsov", &self.tsov())
                .field("prd", &self.prd())
                .finish()
        }
    }
    #[doc = "desc MIN."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Min(pub u32);
    impl Min {
        #[doc = "desc MINL."]
        #[must_use]
        #[inline(always)]
        pub const fn minl(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x0f;
            val as u8
        }
        #[doc = "desc MINL."]
        #[inline(always)]
        pub const fn set_minl(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
        }
        #[doc = "desc MINH."]
        #[must_use]
        #[inline(always)]
        pub const fn minh(&self) -> u8 {
            let val = (self.0 >> 4usize) & 0x07;
            val as u8
        }
        #[doc = "desc MINH."]
        #[inline(always)]
        pub const fn set_minh(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 4usize)) | (((val as u32) & 0x07) << 4usize);
        }
    }
    impl Default for Min {
        #[inline(always)]
        fn default() -> Min {
            Min(0)
        }
    }
    impl core::fmt::Debug for Min {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Min")
                .field("minl", &self.minl())
                .field("minh", &self.minh())
                .finish()
        }
    }
    #[doc = "desc MON."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Mon(pub u32);
    impl Mon {
        #[doc = "desc MON."]
        #[must_use]
        #[inline(always)]
        pub const fn mon(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x1f;
            val as u8
        }
        #[doc = "desc MON."]
        #[inline(always)]
        pub const fn set_mon(&mut self, val: u8) {
            self.0 = (self.0 & !(0x1f << 0usize)) | (((val as u32) & 0x1f) << 0usize);
        }
    }
    impl Default for Mon {
        #[inline(always)]
        fn default() -> Mon {
            Mon(0)
        }
    }
    impl core::fmt::Debug for Mon {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Mon").field("mon", &self.mon()).finish()
        }
    }
    #[doc = "desc SEC."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Sec(pub u32);
    impl Sec {
        #[doc = "desc SECL."]
        #[must_use]
        #[inline(always)]
        pub const fn secl(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x0f;
            val as u8
        }
        #[doc = "desc SECL."]
        #[inline(always)]
        pub const fn set_secl(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
        }
        #[doc = "desc SECH."]
        #[must_use]
        #[inline(always)]
        pub const fn sech(&self) -> u8 {
            let val = (self.0 >> 4usize) & 0x07;
            val as u8
        }
        #[doc = "desc SECH."]
        #[inline(always)]
        pub const fn set_sech(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 4usize)) | (((val as u32) & 0x07) << 4usize);
        }
    }
    impl Default for Sec {
        #[inline(always)]
        fn default() -> Sec {
            Sec(0)
        }
    }
    impl core::fmt::Debug for Sec {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Sec")
                .field("secl", &self.secl())
                .field("sech", &self.sech())
                .finish()
        }
    }
    #[doc = "desc TSDR."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Tsdr(pub u32);
    impl Tsdr {
        #[doc = "desc DAYL."]
        #[must_use]
        #[inline(always)]
        pub const fn dayl(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x0f;
            val as u8
        }
        #[doc = "desc DAYL."]
        #[inline(always)]
        pub const fn set_dayl(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
        }
        #[doc = "desc DAYH."]
        #[must_use]
        #[inline(always)]
        pub const fn dayh(&self) -> u8 {
            let val = (self.0 >> 4usize) & 0x03;
            val as u8
        }
        #[doc = "desc DAYH."]
        #[inline(always)]
        pub const fn set_dayh(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 4usize)) | (((val as u32) & 0x03) << 4usize);
        }
        #[doc = "desc MON."]
        #[must_use]
        #[inline(always)]
        pub const fn mon(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0x1f;
            val as u8
        }
        #[doc = "desc MON."]
        #[inline(always)]
        pub const fn set_mon(&mut self, val: u8) {
            self.0 = (self.0 & !(0x1f << 8usize)) | (((val as u32) & 0x1f) << 8usize);
        }
        #[doc = "desc WEEK."]
        #[must_use]
        #[inline(always)]
        pub const fn week(&self) -> u8 {
            let val = (self.0 >> 13usize) & 0x07;
            val as u8
        }
        #[doc = "desc WEEK."]
        #[inline(always)]
        pub const fn set_week(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 13usize)) | (((val as u32) & 0x07) << 13usize);
        }
    }
    impl Default for Tsdr {
        #[inline(always)]
        fn default() -> Tsdr {
            Tsdr(0)
        }
    }
    impl core::fmt::Debug for Tsdr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Tsdr")
                .field("dayl", &self.dayl())
                .field("dayh", &self.dayh())
                .field("mon", &self.mon())
                .field("week", &self.week())
                .finish()
        }
    }
    #[doc = "desc TSTR."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Tstr(pub u32);
    impl Tstr {
        #[doc = "desc SECL."]
        #[must_use]
        #[inline(always)]
        pub const fn secl(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x0f;
            val as u8
        }
        #[doc = "desc SECL."]
        #[inline(always)]
        pub const fn set_secl(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
        }
        #[doc = "desc SECH."]
        #[must_use]
        #[inline(always)]
        pub const fn sech(&self) -> u8 {
            let val = (self.0 >> 4usize) & 0x07;
            val as u8
        }
        #[doc = "desc SECH."]
        #[inline(always)]
        pub const fn set_sech(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 4usize)) | (((val as u32) & 0x07) << 4usize);
        }
        #[doc = "desc MINL."]
        #[must_use]
        #[inline(always)]
        pub const fn minl(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0x0f;
            val as u8
        }
        #[doc = "desc MINL."]
        #[inline(always)]
        pub const fn set_minl(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 8usize)) | (((val as u32) & 0x0f) << 8usize);
        }
        #[doc = "desc MINH."]
        #[must_use]
        #[inline(always)]
        pub const fn minh(&self) -> u8 {
            let val = (self.0 >> 12usize) & 0x07;
            val as u8
        }
        #[doc = "desc MINH."]
        #[inline(always)]
        pub const fn set_minh(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 12usize)) | (((val as u32) & 0x07) << 12usize);
        }
        #[doc = "desc HOURL."]
        #[must_use]
        #[inline(always)]
        pub const fn hourl(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x0f;
            val as u8
        }
        #[doc = "desc HOURL."]
        #[inline(always)]
        pub const fn set_hourl(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 16usize)) | (((val as u32) & 0x0f) << 16usize);
        }
        #[doc = "desc HOURH."]
        #[must_use]
        #[inline(always)]
        pub const fn hourh(&self) -> u8 {
            let val = (self.0 >> 20usize) & 0x03;
            val as u8
        }
        #[doc = "desc HOURH."]
        #[inline(always)]
        pub const fn set_hourh(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 20usize)) | (((val as u32) & 0x03) << 20usize);
        }
    }
    impl Default for Tstr {
        #[inline(always)]
        fn default() -> Tstr {
            Tstr(0)
        }
    }
    impl core::fmt::Debug for Tstr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Tstr")
                .field("secl", &self.secl())
                .field("sech", &self.sech())
                .field("minl", &self.minl())
                .field("minh", &self.minh())
                .field("hourl", &self.hourl())
                .field("hourh", &self.hourh())
                .finish()
        }
    }
    #[doc = "desc WEEK."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Week(pub u32);
    impl Week {
        #[doc = "desc WEEK."]
        #[must_use]
        #[inline(always)]
        pub const fn week(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x07;
            val as u8
        }
        #[doc = "desc WEEK."]
        #[inline(always)]
        pub const fn set_week(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 0usize)) | (((val as u32) & 0x07) << 0usize);
        }
    }
    impl Default for Week {
        #[inline(always)]
        fn default() -> Week {
            Week(0)
        }
    }
    impl core::fmt::Debug for Week {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Week").field("week", &self.week()).finish()
        }
    }
    #[doc = "desc WPR."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Wpr(pub u32);
    impl Wpr {
        #[doc = "desc WPR."]
        #[must_use]
        #[inline(always)]
        pub const fn wpr(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[doc = "desc WPR."]
        #[inline(always)]
        pub const fn set_wpr(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
        }
    }
    impl Default for Wpr {
        #[inline(always)]
        fn default() -> Wpr {
            Wpr(0)
        }
    }
    impl core::fmt::Debug for Wpr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Wpr").field("wpr", &self.wpr()).finish()
        }
    }
    #[doc = "desc WUTR."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Wutr(pub u32);
    impl Wutr {
        #[doc = "desc ARR."]
        #[must_use]
        #[inline(always)]
        pub const fn arr(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[doc = "desc ARR."]
        #[inline(always)]
        pub const fn set_arr(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
        }
    }
    impl Default for Wutr {
        #[inline(always)]
        fn default() -> Wutr {
            Wutr(0)
        }
    }
    impl core::fmt::Debug for Wutr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Wutr").field("arr", &self.arr()).finish()
        }
    }
    #[doc = "desc YEAR."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Year(pub u32);
    impl Year {
        #[doc = "desc YEARL."]
        #[must_use]
        #[inline(always)]
        pub const fn yearl(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x0f;
            val as u8
        }
        #[doc = "desc YEARL."]
        #[inline(always)]
        pub const fn set_yearl(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
        }
        #[doc = "desc YEARH."]
        #[must_use]
        #[inline(always)]
        pub const fn yearh(&self) -> u8 {
            let val = (self.0 >> 4usize) & 0x0f;
            val as u8
        }
        #[doc = "desc YEARH."]
        #[inline(always)]
        pub const fn set_yearh(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 4usize)) | (((val as u32) & 0x0f) << 4usize);
        }
    }
    impl Default for Year {
        #[inline(always)]
        fn default() -> Year {
            Year(0)
        }
    }
    impl core::fmt::Debug for Year {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Year")
                .field("yearl", &self.yearl())
                .field("yearh", &self.yearh())
                .finish()
        }
    }
}
