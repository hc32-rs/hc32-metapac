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
    #[doc = "desc CR2."]
    #[inline(always)]
    pub const fn cr2(self) -> crate::common::Reg<regs::Cr2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x08usize) as _) }
    }
    #[doc = "desc CR3."]
    #[inline(always)]
    pub const fn cr3(self) -> crate::common::Reg<regs::Cr3, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0cusize) as _) }
    }
    #[doc = "desc SEC."]
    #[inline(always)]
    pub const fn sec(self) -> crate::common::Reg<regs::Sec, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x10usize) as _) }
    }
    #[doc = "desc MIN."]
    #[inline(always)]
    pub const fn min(self) -> crate::common::Reg<regs::Min, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x14usize) as _) }
    }
    #[doc = "desc HOUR."]
    #[inline(always)]
    pub const fn hour(self) -> crate::common::Reg<regs::Hour, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x18usize) as _) }
    }
    #[doc = "desc WEEK."]
    #[inline(always)]
    pub const fn week(self) -> crate::common::Reg<regs::Week, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1cusize) as _) }
    }
    #[doc = "desc DAY."]
    #[inline(always)]
    pub const fn day(self) -> crate::common::Reg<regs::Day, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x20usize) as _) }
    }
    #[doc = "desc MON."]
    #[inline(always)]
    pub const fn mon(self) -> crate::common::Reg<regs::Mon, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x24usize) as _) }
    }
    #[doc = "desc YEAR."]
    #[inline(always)]
    pub const fn year(self) -> crate::common::Reg<regs::Year, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x28usize) as _) }
    }
    #[doc = "desc ALMMIN."]
    #[inline(always)]
    pub const fn almmin(self) -> crate::common::Reg<regs::Almmin, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x2cusize) as _) }
    }
    #[doc = "desc ALMHOUR."]
    #[inline(always)]
    pub const fn almhour(self) -> crate::common::Reg<regs::Almhour, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x30usize) as _) }
    }
    #[doc = "desc ALMWEEK."]
    #[inline(always)]
    pub const fn almweek(self) -> crate::common::Reg<regs::Almweek, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x34usize) as _) }
    }
    #[doc = "desc ERRCRH."]
    #[inline(always)]
    pub const fn errcrh(self) -> crate::common::Reg<regs::Errcrh, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x38usize) as _) }
    }
    #[doc = "desc ERRCRL."]
    #[inline(always)]
    pub const fn errcrl(self) -> crate::common::Reg<regs::Errcrl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x3cusize) as _) }
    }
    #[doc = "desc TPCR0."]
    #[inline(always)]
    pub const fn tpcr0(self) -> crate::common::Reg<regs::Tpcr0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x40usize) as _) }
    }
    #[doc = "desc TPCR1."]
    #[inline(always)]
    pub const fn tpcr1(self) -> crate::common::Reg<regs::Tpcr1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x44usize) as _) }
    }
    #[doc = "desc TPSR."]
    #[inline(always)]
    pub const fn tpsr(self) -> crate::common::Reg<regs::Tpsr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x48usize) as _) }
    }
    #[doc = "desc SECTP."]
    #[inline(always)]
    pub const fn sectp(self) -> crate::common::Reg<regs::Sectp, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x4cusize) as _) }
    }
    #[doc = "desc MINTP."]
    #[inline(always)]
    pub const fn mintp(self) -> crate::common::Reg<regs::Mintp, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x50usize) as _) }
    }
    #[doc = "desc HOURTP."]
    #[inline(always)]
    pub const fn hourtp(self) -> crate::common::Reg<regs::Hourtp, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x54usize) as _) }
    }
    #[doc = "desc DAYTP."]
    #[inline(always)]
    pub const fn daytp(self) -> crate::common::Reg<regs::Daytp, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x58usize) as _) }
    }
    #[doc = "desc MONTP."]
    #[inline(always)]
    pub const fn montp(self) -> crate::common::Reg<regs::Montp, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x5cusize) as _) }
    }
}
pub mod regs {
    #[doc = "desc ALMHOUR."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Almhour(pub u8);
    impl Almhour {
        #[doc = "desc ALMHOURU."]
        #[must_use]
        #[inline(always)]
        pub const fn almhouru(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x0f;
            val as u8
        }
        #[doc = "desc ALMHOURU."]
        #[inline(always)]
        pub const fn set_almhouru(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u8) & 0x0f) << 0usize);
        }
        #[doc = "desc ALMHOURD."]
        #[must_use]
        #[inline(always)]
        pub const fn almhourd(&self) -> u8 {
            let val = (self.0 >> 4usize) & 0x03;
            val as u8
        }
        #[doc = "desc ALMHOURD."]
        #[inline(always)]
        pub const fn set_almhourd(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 4usize)) | (((val as u8) & 0x03) << 4usize);
        }
    }
    impl Default for Almhour {
        #[inline(always)]
        fn default() -> Almhour {
            Almhour(0)
        }
    }
    impl core::fmt::Debug for Almhour {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Almhour")
                .field("almhouru", &self.almhouru())
                .field("almhourd", &self.almhourd())
                .finish()
        }
    }
    #[doc = "desc ALMMIN."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Almmin(pub u8);
    impl Almmin {
        #[doc = "desc ALMMINU."]
        #[must_use]
        #[inline(always)]
        pub const fn almminu(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x0f;
            val as u8
        }
        #[doc = "desc ALMMINU."]
        #[inline(always)]
        pub const fn set_almminu(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u8) & 0x0f) << 0usize);
        }
        #[doc = "desc ALMMIND."]
        #[must_use]
        #[inline(always)]
        pub const fn almmind(&self) -> u8 {
            let val = (self.0 >> 4usize) & 0x07;
            val as u8
        }
        #[doc = "desc ALMMIND."]
        #[inline(always)]
        pub const fn set_almmind(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 4usize)) | (((val as u8) & 0x07) << 4usize);
        }
    }
    impl Default for Almmin {
        #[inline(always)]
        fn default() -> Almmin {
            Almmin(0)
        }
    }
    impl core::fmt::Debug for Almmin {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Almmin")
                .field("almminu", &self.almminu())
                .field("almmind", &self.almmind())
                .finish()
        }
    }
    #[doc = "desc ALMWEEK."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Almweek(pub u8);
    impl Almweek {
        #[doc = "desc ALMWEEK."]
        #[must_use]
        #[inline(always)]
        pub const fn almweek(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x7f;
            val as u8
        }
        #[doc = "desc ALMWEEK."]
        #[inline(always)]
        pub const fn set_almweek(&mut self, val: u8) {
            self.0 = (self.0 & !(0x7f << 0usize)) | (((val as u8) & 0x7f) << 0usize);
        }
    }
    impl Default for Almweek {
        #[inline(always)]
        fn default() -> Almweek {
            Almweek(0)
        }
    }
    impl core::fmt::Debug for Almweek {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Almweek")
                .field("almweek", &self.almweek())
                .finish()
        }
    }
    #[doc = "desc CR0."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Cr0(pub u8);
    impl Cr0 {
        #[doc = "desc RESET."]
        #[must_use]
        #[inline(always)]
        pub const fn reset(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "desc RESET."]
        #[inline(always)]
        pub const fn set_reset(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u8) & 0x01) << 0usize);
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
            f.debug_struct("Cr0").field("reset", &self.reset()).finish()
        }
    }
    #[doc = "desc CR1."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Cr1(pub u8);
    impl Cr1 {
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
            self.0 = (self.0 & !(0x07 << 0usize)) | (((val as u8) & 0x07) << 0usize);
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
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u8) & 0x01) << 3usize);
        }
        #[doc = "desc ONEHZOE."]
        #[must_use]
        #[inline(always)]
        pub const fn onehzoe(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "desc ONEHZOE."]
        #[inline(always)]
        pub const fn set_onehzoe(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u8) & 0x01) << 5usize);
        }
        #[doc = "desc ONEHZSEL."]
        #[must_use]
        #[inline(always)]
        pub const fn onehzsel(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "desc ONEHZSEL."]
        #[inline(always)]
        pub const fn set_onehzsel(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u8) & 0x01) << 6usize);
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
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u8) & 0x01) << 7usize);
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
                .field("prds", &self.prds())
                .field("ampm", &self.ampm())
                .field("onehzoe", &self.onehzoe())
                .field("onehzsel", &self.onehzsel())
                .field("start", &self.start())
                .finish()
        }
    }
    #[doc = "desc CR2."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Cr2(pub u8);
    impl Cr2 {
        #[doc = "desc RWREQ."]
        #[must_use]
        #[inline(always)]
        pub const fn rwreq(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "desc RWREQ."]
        #[inline(always)]
        pub const fn set_rwreq(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u8) & 0x01) << 0usize);
        }
        #[doc = "desc RWEN."]
        #[must_use]
        #[inline(always)]
        pub const fn rwen(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "desc RWEN."]
        #[inline(always)]
        pub const fn set_rwen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u8) & 0x01) << 1usize);
        }
        #[doc = "desc PRDF."]
        #[must_use]
        #[inline(always)]
        pub const fn prdf(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "desc PRDF."]
        #[inline(always)]
        pub const fn set_prdf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u8) & 0x01) << 2usize);
        }
        #[doc = "desc ALMF."]
        #[must_use]
        #[inline(always)]
        pub const fn almf(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "desc ALMF."]
        #[inline(always)]
        pub const fn set_almf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u8) & 0x01) << 3usize);
        }
        #[doc = "desc PRDIE."]
        #[must_use]
        #[inline(always)]
        pub const fn prdie(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "desc PRDIE."]
        #[inline(always)]
        pub const fn set_prdie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u8) & 0x01) << 5usize);
        }
        #[doc = "desc ALMIE."]
        #[must_use]
        #[inline(always)]
        pub const fn almie(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "desc ALMIE."]
        #[inline(always)]
        pub const fn set_almie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u8) & 0x01) << 6usize);
        }
        #[doc = "desc ALME."]
        #[must_use]
        #[inline(always)]
        pub const fn alme(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "desc ALME."]
        #[inline(always)]
        pub const fn set_alme(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u8) & 0x01) << 7usize);
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
                .field("rwreq", &self.rwreq())
                .field("rwen", &self.rwen())
                .field("prdf", &self.prdf())
                .field("almf", &self.almf())
                .field("prdie", &self.prdie())
                .field("almie", &self.almie())
                .field("alme", &self.alme())
                .finish()
        }
    }
    #[doc = "desc CR3."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Cr3(pub u8);
    impl Cr3 {
        #[doc = "desc LRCEN."]
        #[must_use]
        #[inline(always)]
        pub const fn lrcen(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "desc LRCEN."]
        #[inline(always)]
        pub const fn set_lrcen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u8) & 0x01) << 4usize);
        }
        #[doc = "desc RCKSEL."]
        #[must_use]
        #[inline(always)]
        pub const fn rcksel(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "desc RCKSEL."]
        #[inline(always)]
        pub const fn set_rcksel(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u8) & 0x01) << 7usize);
        }
    }
    impl Default for Cr3 {
        #[inline(always)]
        fn default() -> Cr3 {
            Cr3(0)
        }
    }
    impl core::fmt::Debug for Cr3 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Cr3")
                .field("lrcen", &self.lrcen())
                .field("rcksel", &self.rcksel())
                .finish()
        }
    }
    #[doc = "desc DAY."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Day(pub u8);
    impl Day {
        #[doc = "desc DAYU."]
        #[must_use]
        #[inline(always)]
        pub const fn dayu(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x0f;
            val as u8
        }
        #[doc = "desc DAYU."]
        #[inline(always)]
        pub const fn set_dayu(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u8) & 0x0f) << 0usize);
        }
        #[doc = "desc DAYD."]
        #[must_use]
        #[inline(always)]
        pub const fn dayd(&self) -> u8 {
            let val = (self.0 >> 4usize) & 0x03;
            val as u8
        }
        #[doc = "desc DAYD."]
        #[inline(always)]
        pub const fn set_dayd(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 4usize)) | (((val as u8) & 0x03) << 4usize);
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
                .field("dayu", &self.dayu())
                .field("dayd", &self.dayd())
                .finish()
        }
    }
    #[doc = "desc DAYTP."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Daytp(pub u8);
    impl Daytp {
        #[doc = "desc DAYTPU."]
        #[must_use]
        #[inline(always)]
        pub const fn daytpu(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x0f;
            val as u8
        }
        #[doc = "desc DAYTPU."]
        #[inline(always)]
        pub const fn set_daytpu(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u8) & 0x0f) << 0usize);
        }
        #[doc = "desc DAYTPD."]
        #[must_use]
        #[inline(always)]
        pub const fn daytpd(&self) -> u8 {
            let val = (self.0 >> 4usize) & 0x03;
            val as u8
        }
        #[doc = "desc DAYTPD."]
        #[inline(always)]
        pub const fn set_daytpd(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 4usize)) | (((val as u8) & 0x03) << 4usize);
        }
    }
    impl Default for Daytp {
        #[inline(always)]
        fn default() -> Daytp {
            Daytp(0)
        }
    }
    impl core::fmt::Debug for Daytp {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Daytp")
                .field("daytpu", &self.daytpu())
                .field("daytpd", &self.daytpd())
                .finish()
        }
    }
    #[doc = "desc ERRCRH."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Errcrh(pub u8);
    impl Errcrh {
        #[doc = "desc COMP8."]
        #[must_use]
        #[inline(always)]
        pub const fn comp8(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "desc COMP8."]
        #[inline(always)]
        pub const fn set_comp8(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u8) & 0x01) << 0usize);
        }
        #[doc = "desc COMPEN."]
        #[must_use]
        #[inline(always)]
        pub const fn compen(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "desc COMPEN."]
        #[inline(always)]
        pub const fn set_compen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u8) & 0x01) << 7usize);
        }
    }
    impl Default for Errcrh {
        #[inline(always)]
        fn default() -> Errcrh {
            Errcrh(0)
        }
    }
    impl core::fmt::Debug for Errcrh {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Errcrh")
                .field("comp8", &self.comp8())
                .field("compen", &self.compen())
                .finish()
        }
    }
    #[doc = "desc ERRCRL."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Errcrl(pub u8);
    impl Errcrl {
        #[doc = "desc COMP."]
        #[must_use]
        #[inline(always)]
        pub const fn comp(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[doc = "desc COMP."]
        #[inline(always)]
        pub const fn set_comp(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u8) & 0xff) << 0usize);
        }
    }
    impl Default for Errcrl {
        #[inline(always)]
        fn default() -> Errcrl {
            Errcrl(0)
        }
    }
    impl core::fmt::Debug for Errcrl {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Errcrl")
                .field("comp", &self.comp())
                .finish()
        }
    }
    #[doc = "desc HOUR."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Hour(pub u8);
    impl Hour {
        #[doc = "desc HOURU."]
        #[must_use]
        #[inline(always)]
        pub const fn houru(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x0f;
            val as u8
        }
        #[doc = "desc HOURU."]
        #[inline(always)]
        pub const fn set_houru(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u8) & 0x0f) << 0usize);
        }
        #[doc = "desc HOURD."]
        #[must_use]
        #[inline(always)]
        pub const fn hourd(&self) -> u8 {
            let val = (self.0 >> 4usize) & 0x03;
            val as u8
        }
        #[doc = "desc HOURD."]
        #[inline(always)]
        pub const fn set_hourd(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 4usize)) | (((val as u8) & 0x03) << 4usize);
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
                .field("houru", &self.houru())
                .field("hourd", &self.hourd())
                .finish()
        }
    }
    #[doc = "desc HOURTP."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Hourtp(pub u8);
    impl Hourtp {
        #[doc = "desc HOURTPU."]
        #[must_use]
        #[inline(always)]
        pub const fn hourtpu(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x0f;
            val as u8
        }
        #[doc = "desc HOURTPU."]
        #[inline(always)]
        pub const fn set_hourtpu(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u8) & 0x0f) << 0usize);
        }
        #[doc = "desc HOURTPD."]
        #[must_use]
        #[inline(always)]
        pub const fn hourtpd(&self) -> u8 {
            let val = (self.0 >> 4usize) & 0x03;
            val as u8
        }
        #[doc = "desc HOURTPD."]
        #[inline(always)]
        pub const fn set_hourtpd(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 4usize)) | (((val as u8) & 0x03) << 4usize);
        }
    }
    impl Default for Hourtp {
        #[inline(always)]
        fn default() -> Hourtp {
            Hourtp(0)
        }
    }
    impl core::fmt::Debug for Hourtp {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Hourtp")
                .field("hourtpu", &self.hourtpu())
                .field("hourtpd", &self.hourtpd())
                .finish()
        }
    }
    #[doc = "desc MIN."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Min(pub u8);
    impl Min {
        #[doc = "desc MINU."]
        #[must_use]
        #[inline(always)]
        pub const fn minu(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x0f;
            val as u8
        }
        #[doc = "desc MINU."]
        #[inline(always)]
        pub const fn set_minu(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u8) & 0x0f) << 0usize);
        }
        #[doc = "desc MIND."]
        #[must_use]
        #[inline(always)]
        pub const fn mind(&self) -> u8 {
            let val = (self.0 >> 4usize) & 0x07;
            val as u8
        }
        #[doc = "desc MIND."]
        #[inline(always)]
        pub const fn set_mind(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 4usize)) | (((val as u8) & 0x07) << 4usize);
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
                .field("minu", &self.minu())
                .field("mind", &self.mind())
                .finish()
        }
    }
    #[doc = "desc MINTP."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Mintp(pub u8);
    impl Mintp {
        #[doc = "desc MINTPU."]
        #[must_use]
        #[inline(always)]
        pub const fn mintpu(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x0f;
            val as u8
        }
        #[doc = "desc MINTPU."]
        #[inline(always)]
        pub const fn set_mintpu(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u8) & 0x0f) << 0usize);
        }
        #[doc = "desc MINTPD."]
        #[must_use]
        #[inline(always)]
        pub const fn mintpd(&self) -> u8 {
            let val = (self.0 >> 4usize) & 0x07;
            val as u8
        }
        #[doc = "desc MINTPD."]
        #[inline(always)]
        pub const fn set_mintpd(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 4usize)) | (((val as u8) & 0x07) << 4usize);
        }
    }
    impl Default for Mintp {
        #[inline(always)]
        fn default() -> Mintp {
            Mintp(0)
        }
    }
    impl core::fmt::Debug for Mintp {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Mintp")
                .field("mintpu", &self.mintpu())
                .field("mintpd", &self.mintpd())
                .finish()
        }
    }
    #[doc = "desc MON."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Mon(pub u8);
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
            self.0 = (self.0 & !(0x1f << 0usize)) | (((val as u8) & 0x1f) << 0usize);
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
    #[doc = "desc MONTP."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Montp(pub u8);
    impl Montp {
        #[doc = "desc MONTP."]
        #[must_use]
        #[inline(always)]
        pub const fn montp(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x1f;
            val as u8
        }
        #[doc = "desc MONTP."]
        #[inline(always)]
        pub const fn set_montp(&mut self, val: u8) {
            self.0 = (self.0 & !(0x1f << 0usize)) | (((val as u8) & 0x1f) << 0usize);
        }
    }
    impl Default for Montp {
        #[inline(always)]
        fn default() -> Montp {
            Montp(0)
        }
    }
    impl core::fmt::Debug for Montp {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Montp")
                .field("montp", &self.montp())
                .finish()
        }
    }
    #[doc = "desc SEC."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Sec(pub u8);
    impl Sec {
        #[doc = "desc SECU."]
        #[must_use]
        #[inline(always)]
        pub const fn secu(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x0f;
            val as u8
        }
        #[doc = "desc SECU."]
        #[inline(always)]
        pub const fn set_secu(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u8) & 0x0f) << 0usize);
        }
        #[doc = "desc SECD."]
        #[must_use]
        #[inline(always)]
        pub const fn secd(&self) -> u8 {
            let val = (self.0 >> 4usize) & 0x07;
            val as u8
        }
        #[doc = "desc SECD."]
        #[inline(always)]
        pub const fn set_secd(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 4usize)) | (((val as u8) & 0x07) << 4usize);
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
                .field("secu", &self.secu())
                .field("secd", &self.secd())
                .finish()
        }
    }
    #[doc = "desc SECTP."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Sectp(pub u8);
    impl Sectp {
        #[doc = "desc SECTPU."]
        #[must_use]
        #[inline(always)]
        pub const fn sectpu(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x0f;
            val as u8
        }
        #[doc = "desc SECTPU."]
        #[inline(always)]
        pub const fn set_sectpu(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u8) & 0x0f) << 0usize);
        }
        #[doc = "desc SECTPD."]
        #[must_use]
        #[inline(always)]
        pub const fn sectpd(&self) -> u8 {
            let val = (self.0 >> 4usize) & 0x07;
            val as u8
        }
        #[doc = "desc SECTPD."]
        #[inline(always)]
        pub const fn set_sectpd(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 4usize)) | (((val as u8) & 0x07) << 4usize);
        }
    }
    impl Default for Sectp {
        #[inline(always)]
        fn default() -> Sectp {
            Sectp(0)
        }
    }
    impl core::fmt::Debug for Sectp {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Sectp")
                .field("sectpu", &self.sectpu())
                .field("sectpd", &self.sectpd())
                .finish()
        }
    }
    #[doc = "desc TPCR0."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Tpcr0(pub u8);
    impl Tpcr0 {
        #[doc = "desc TPCT0."]
        #[must_use]
        #[inline(always)]
        pub const fn tpct0(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x03;
            val as u8
        }
        #[doc = "desc TPCT0."]
        #[inline(always)]
        pub const fn set_tpct0(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 0usize)) | (((val as u8) & 0x03) << 0usize);
        }
        #[doc = "desc TPNF0."]
        #[must_use]
        #[inline(always)]
        pub const fn tpnf0(&self) -> u8 {
            let val = (self.0 >> 2usize) & 0x03;
            val as u8
        }
        #[doc = "desc TPNF0."]
        #[inline(always)]
        pub const fn set_tpnf0(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 2usize)) | (((val as u8) & 0x03) << 2usize);
        }
        #[doc = "desc TPRSTE0."]
        #[must_use]
        #[inline(always)]
        pub const fn tprste0(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "desc TPRSTE0."]
        #[inline(always)]
        pub const fn set_tprste0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u8) & 0x01) << 4usize);
        }
        #[doc = "desc TPIE0."]
        #[must_use]
        #[inline(always)]
        pub const fn tpie0(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "desc TPIE0."]
        #[inline(always)]
        pub const fn set_tpie0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u8) & 0x01) << 5usize);
        }
        #[doc = "desc TSTPE0."]
        #[must_use]
        #[inline(always)]
        pub const fn tstpe0(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "desc TSTPE0."]
        #[inline(always)]
        pub const fn set_tstpe0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u8) & 0x01) << 6usize);
        }
        #[doc = "desc TPEN0."]
        #[must_use]
        #[inline(always)]
        pub const fn tpen0(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "desc TPEN0."]
        #[inline(always)]
        pub const fn set_tpen0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u8) & 0x01) << 7usize);
        }
    }
    impl Default for Tpcr0 {
        #[inline(always)]
        fn default() -> Tpcr0 {
            Tpcr0(0)
        }
    }
    impl core::fmt::Debug for Tpcr0 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Tpcr0")
                .field("tpct0", &self.tpct0())
                .field("tpnf0", &self.tpnf0())
                .field("tprste0", &self.tprste0())
                .field("tpie0", &self.tpie0())
                .field("tstpe0", &self.tstpe0())
                .field("tpen0", &self.tpen0())
                .finish()
        }
    }
    #[doc = "desc TPCR1."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Tpcr1(pub u8);
    impl Tpcr1 {
        #[doc = "desc TPCT1."]
        #[must_use]
        #[inline(always)]
        pub const fn tpct1(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x03;
            val as u8
        }
        #[doc = "desc TPCT1."]
        #[inline(always)]
        pub const fn set_tpct1(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 0usize)) | (((val as u8) & 0x03) << 0usize);
        }
        #[doc = "desc TPNF1."]
        #[must_use]
        #[inline(always)]
        pub const fn tpnf1(&self) -> u8 {
            let val = (self.0 >> 2usize) & 0x03;
            val as u8
        }
        #[doc = "desc TPNF1."]
        #[inline(always)]
        pub const fn set_tpnf1(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 2usize)) | (((val as u8) & 0x03) << 2usize);
        }
        #[doc = "desc TPRSTE1."]
        #[must_use]
        #[inline(always)]
        pub const fn tprste1(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "desc TPRSTE1."]
        #[inline(always)]
        pub const fn set_tprste1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u8) & 0x01) << 4usize);
        }
        #[doc = "desc TPIE1."]
        #[must_use]
        #[inline(always)]
        pub const fn tpie1(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "desc TPIE1."]
        #[inline(always)]
        pub const fn set_tpie1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u8) & 0x01) << 5usize);
        }
        #[doc = "desc TSTPE1."]
        #[must_use]
        #[inline(always)]
        pub const fn tstpe1(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "desc TSTPE1."]
        #[inline(always)]
        pub const fn set_tstpe1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u8) & 0x01) << 6usize);
        }
        #[doc = "desc TPEN1."]
        #[must_use]
        #[inline(always)]
        pub const fn tpen1(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "desc TPEN1."]
        #[inline(always)]
        pub const fn set_tpen1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u8) & 0x01) << 7usize);
        }
    }
    impl Default for Tpcr1 {
        #[inline(always)]
        fn default() -> Tpcr1 {
            Tpcr1(0)
        }
    }
    impl core::fmt::Debug for Tpcr1 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Tpcr1")
                .field("tpct1", &self.tpct1())
                .field("tpnf1", &self.tpnf1())
                .field("tprste1", &self.tprste1())
                .field("tpie1", &self.tpie1())
                .field("tstpe1", &self.tstpe1())
                .field("tpen1", &self.tpen1())
                .finish()
        }
    }
    #[doc = "desc TPSR."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Tpsr(pub u8);
    impl Tpsr {
        #[doc = "desc TPF0."]
        #[must_use]
        #[inline(always)]
        pub const fn tpf0(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "desc TPF0."]
        #[inline(always)]
        pub const fn set_tpf0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u8) & 0x01) << 0usize);
        }
        #[doc = "desc TPF1."]
        #[must_use]
        #[inline(always)]
        pub const fn tpf1(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "desc TPF1."]
        #[inline(always)]
        pub const fn set_tpf1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u8) & 0x01) << 1usize);
        }
        #[doc = "desc TPOVF."]
        #[must_use]
        #[inline(always)]
        pub const fn tpovf(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "desc TPOVF."]
        #[inline(always)]
        pub const fn set_tpovf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u8) & 0x01) << 2usize);
        }
    }
    impl Default for Tpsr {
        #[inline(always)]
        fn default() -> Tpsr {
            Tpsr(0)
        }
    }
    impl core::fmt::Debug for Tpsr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Tpsr")
                .field("tpf0", &self.tpf0())
                .field("tpf1", &self.tpf1())
                .field("tpovf", &self.tpovf())
                .finish()
        }
    }
    #[doc = "desc WEEK."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Week(pub u8);
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
            self.0 = (self.0 & !(0x07 << 0usize)) | (((val as u8) & 0x07) << 0usize);
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
    #[doc = "desc YEAR."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Year(pub u8);
    impl Year {
        #[doc = "desc YEARU."]
        #[must_use]
        #[inline(always)]
        pub const fn yearu(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x0f;
            val as u8
        }
        #[doc = "desc YEARU."]
        #[inline(always)]
        pub const fn set_yearu(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u8) & 0x0f) << 0usize);
        }
        #[doc = "desc YEARD."]
        #[must_use]
        #[inline(always)]
        pub const fn yeard(&self) -> u8 {
            let val = (self.0 >> 4usize) & 0x0f;
            val as u8
        }
        #[doc = "desc YEARD."]
        #[inline(always)]
        pub const fn set_yeard(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 4usize)) | (((val as u8) & 0x0f) << 4usize);
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
                .field("yearu", &self.yearu())
                .field("yeard", &self.yeard())
                .finish()
        }
    }
}
