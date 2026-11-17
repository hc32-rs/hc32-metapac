#![allow(clippy::missing_safety_doc)]
#![allow(clippy::identity_op)]
#![allow(clippy::unnecessary_cast)]
#![allow(clippy::erasing_op)]

#[doc = "desc CMP2."]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cmp {
    ptr: *mut u8,
}
unsafe impl Send for Cmp {}
unsafe impl Sync for Cmp {}
impl Cmp {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "desc MDR."]
    #[inline(always)]
    pub const fn mdr(self) -> crate::common::Reg<regs::Mdr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0usize) as _) }
    }
    #[doc = "desc FIR."]
    #[inline(always)]
    pub const fn fir(self) -> crate::common::Reg<regs::Fir, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x01usize) as _) }
    }
    #[doc = "desc OCR."]
    #[inline(always)]
    pub const fn ocr(self) -> crate::common::Reg<regs::Ocr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x02usize) as _) }
    }
    #[doc = "desc VSR."]
    #[inline(always)]
    pub const fn vsr(self) -> crate::common::Reg<regs::Vsr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x03usize) as _) }
    }
    #[doc = "desc TWR1."]
    #[inline(always)]
    pub const fn twr1(self) -> crate::common::Reg<regs::Twr1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x04usize) as _) }
    }
    #[doc = "desc TWR2."]
    #[inline(always)]
    pub const fn twr2(self) -> crate::common::Reg<regs::Twr2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x05usize) as _) }
    }
}
pub mod regs {
    #[doc = "desc FIR."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Fir(pub u8);
    impl Fir {
        #[doc = "desc FCKS."]
        #[must_use]
        #[inline(always)]
        pub const fn fcks(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x03;
            val as u8
        }
        #[doc = "desc FCKS."]
        #[inline(always)]
        pub const fn set_fcks(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 0usize)) | (((val as u8) & 0x03) << 0usize);
        }
        #[doc = "desc EDGS."]
        #[must_use]
        #[inline(always)]
        pub const fn edgs(&self) -> u8 {
            let val = (self.0 >> 4usize) & 0x03;
            val as u8
        }
        #[doc = "desc EDGS."]
        #[inline(always)]
        pub const fn set_edgs(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 4usize)) | (((val as u8) & 0x03) << 4usize);
        }
        #[doc = "desc CIEN."]
        #[must_use]
        #[inline(always)]
        pub const fn cien(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "desc CIEN."]
        #[inline(always)]
        pub const fn set_cien(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u8) & 0x01) << 6usize);
        }
    }
    impl Default for Fir {
        #[inline(always)]
        fn default() -> Fir {
            Fir(0)
        }
    }
    impl core::fmt::Debug for Fir {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Fir")
                .field("fcks", &self.fcks())
                .field("edgs", &self.edgs())
                .field("cien", &self.cien())
                .finish()
        }
    }
    #[doc = "desc MDR."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Mdr(pub u8);
    impl Mdr {
        #[doc = "desc CENB."]
        #[must_use]
        #[inline(always)]
        pub const fn cenb(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "desc CENB."]
        #[inline(always)]
        pub const fn set_cenb(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u8) & 0x01) << 0usize);
        }
        #[doc = "desc CWDE."]
        #[must_use]
        #[inline(always)]
        pub const fn cwde(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "desc CWDE."]
        #[inline(always)]
        pub const fn set_cwde(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u8) & 0x01) << 1usize);
        }
        #[doc = "desc CMON."]
        #[must_use]
        #[inline(always)]
        pub const fn cmon(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "desc CMON."]
        #[inline(always)]
        pub const fn set_cmon(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u8) & 0x01) << 7usize);
        }
    }
    impl Default for Mdr {
        #[inline(always)]
        fn default() -> Mdr {
            Mdr(0)
        }
    }
    impl core::fmt::Debug for Mdr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Mdr")
                .field("cenb", &self.cenb())
                .field("cwde", &self.cwde())
                .field("cmon", &self.cmon())
                .finish()
        }
    }
    #[doc = "desc OCR."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ocr(pub u8);
    impl Ocr {
        #[doc = "desc COEN."]
        #[must_use]
        #[inline(always)]
        pub const fn coen(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "desc COEN."]
        #[inline(always)]
        pub const fn set_coen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u8) & 0x01) << 0usize);
        }
        #[doc = "desc COPS."]
        #[must_use]
        #[inline(always)]
        pub const fn cops(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "desc COPS."]
        #[inline(always)]
        pub const fn set_cops(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u8) & 0x01) << 1usize);
        }
        #[doc = "desc CPOE."]
        #[must_use]
        #[inline(always)]
        pub const fn cpoe(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "desc CPOE."]
        #[inline(always)]
        pub const fn set_cpoe(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u8) & 0x01) << 2usize);
        }
        #[doc = "desc TWOE."]
        #[must_use]
        #[inline(always)]
        pub const fn twoe(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "desc TWOE."]
        #[inline(always)]
        pub const fn set_twoe(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u8) & 0x01) << 3usize);
        }
        #[doc = "desc TWOL."]
        #[must_use]
        #[inline(always)]
        pub const fn twol(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "desc TWOL."]
        #[inline(always)]
        pub const fn set_twol(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u8) & 0x01) << 4usize);
        }
    }
    impl Default for Ocr {
        #[inline(always)]
        fn default() -> Ocr {
            Ocr(0)
        }
    }
    impl core::fmt::Debug for Ocr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Ocr")
                .field("coen", &self.coen())
                .field("cops", &self.cops())
                .field("cpoe", &self.cpoe())
                .field("twoe", &self.twoe())
                .field("twol", &self.twol())
                .finish()
        }
    }
    #[doc = "desc TWR1."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Twr1(pub u8);
    impl Twr1 {
        #[doc = "desc CTWS0."]
        #[must_use]
        #[inline(always)]
        pub const fn ctws0(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "desc CTWS0."]
        #[inline(always)]
        pub const fn set_ctws0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u8) & 0x01) << 0usize);
        }
        #[doc = "desc CTWS1."]
        #[must_use]
        #[inline(always)]
        pub const fn ctws1(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "desc CTWS1."]
        #[inline(always)]
        pub const fn set_ctws1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u8) & 0x01) << 1usize);
        }
        #[doc = "desc CTWS2."]
        #[must_use]
        #[inline(always)]
        pub const fn ctws2(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "desc CTWS2."]
        #[inline(always)]
        pub const fn set_ctws2(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u8) & 0x01) << 2usize);
        }
        #[doc = "desc CTWS3."]
        #[must_use]
        #[inline(always)]
        pub const fn ctws3(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "desc CTWS3."]
        #[inline(always)]
        pub const fn set_ctws3(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u8) & 0x01) << 3usize);
        }
        #[doc = "desc CTWS4."]
        #[must_use]
        #[inline(always)]
        pub const fn ctws4(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "desc CTWS4."]
        #[inline(always)]
        pub const fn set_ctws4(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u8) & 0x01) << 4usize);
        }
        #[doc = "desc CTWS5."]
        #[must_use]
        #[inline(always)]
        pub const fn ctws5(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "desc CTWS5."]
        #[inline(always)]
        pub const fn set_ctws5(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u8) & 0x01) << 5usize);
        }
        #[doc = "desc CTWS6."]
        #[must_use]
        #[inline(always)]
        pub const fn ctws6(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "desc CTWS6."]
        #[inline(always)]
        pub const fn set_ctws6(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u8) & 0x01) << 6usize);
        }
        #[doc = "desc CTWS7."]
        #[must_use]
        #[inline(always)]
        pub const fn ctws7(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "desc CTWS7."]
        #[inline(always)]
        pub const fn set_ctws7(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u8) & 0x01) << 7usize);
        }
    }
    impl Default for Twr1 {
        #[inline(always)]
        fn default() -> Twr1 {
            Twr1(0)
        }
    }
    impl core::fmt::Debug for Twr1 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Twr1")
                .field("ctws0", &self.ctws0())
                .field("ctws1", &self.ctws1())
                .field("ctws2", &self.ctws2())
                .field("ctws3", &self.ctws3())
                .field("ctws4", &self.ctws4())
                .field("ctws5", &self.ctws5())
                .field("ctws6", &self.ctws6())
                .field("ctws7", &self.ctws7())
                .finish()
        }
    }
    #[doc = "desc TWR2."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Twr2(pub u8);
    impl Twr2 {
        #[doc = "desc CTWP0."]
        #[must_use]
        #[inline(always)]
        pub const fn ctwp0(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "desc CTWP0."]
        #[inline(always)]
        pub const fn set_ctwp0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u8) & 0x01) << 0usize);
        }
        #[doc = "desc CTWP1."]
        #[must_use]
        #[inline(always)]
        pub const fn ctwp1(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "desc CTWP1."]
        #[inline(always)]
        pub const fn set_ctwp1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u8) & 0x01) << 1usize);
        }
        #[doc = "desc CTWP2."]
        #[must_use]
        #[inline(always)]
        pub const fn ctwp2(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "desc CTWP2."]
        #[inline(always)]
        pub const fn set_ctwp2(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u8) & 0x01) << 2usize);
        }
        #[doc = "desc CTWP3."]
        #[must_use]
        #[inline(always)]
        pub const fn ctwp3(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "desc CTWP3."]
        #[inline(always)]
        pub const fn set_ctwp3(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u8) & 0x01) << 3usize);
        }
        #[doc = "desc CTWP4."]
        #[must_use]
        #[inline(always)]
        pub const fn ctwp4(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "desc CTWP4."]
        #[inline(always)]
        pub const fn set_ctwp4(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u8) & 0x01) << 4usize);
        }
        #[doc = "desc CTWP5."]
        #[must_use]
        #[inline(always)]
        pub const fn ctwp5(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "desc CTWP5."]
        #[inline(always)]
        pub const fn set_ctwp5(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u8) & 0x01) << 5usize);
        }
        #[doc = "desc CTWP6."]
        #[must_use]
        #[inline(always)]
        pub const fn ctwp6(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "desc CTWP6."]
        #[inline(always)]
        pub const fn set_ctwp6(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u8) & 0x01) << 6usize);
        }
        #[doc = "desc CTWP7."]
        #[must_use]
        #[inline(always)]
        pub const fn ctwp7(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "desc CTWP7."]
        #[inline(always)]
        pub const fn set_ctwp7(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u8) & 0x01) << 7usize);
        }
    }
    impl Default for Twr2 {
        #[inline(always)]
        fn default() -> Twr2 {
            Twr2(0)
        }
    }
    impl core::fmt::Debug for Twr2 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Twr2")
                .field("ctwp0", &self.ctwp0())
                .field("ctwp1", &self.ctwp1())
                .field("ctwp2", &self.ctwp2())
                .field("ctwp3", &self.ctwp3())
                .field("ctwp4", &self.ctwp4())
                .field("ctwp5", &self.ctwp5())
                .field("ctwp6", &self.ctwp6())
                .field("ctwp7", &self.ctwp7())
                .finish()
        }
    }
    #[doc = "desc VSR."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Vsr(pub u8);
    impl Vsr {
        #[doc = "desc RVSL."]
        #[must_use]
        #[inline(always)]
        pub const fn rvsl(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x07;
            val as u8
        }
        #[doc = "desc RVSL."]
        #[inline(always)]
        pub const fn set_rvsl(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 0usize)) | (((val as u8) & 0x07) << 0usize);
        }
        #[doc = "desc CVSL."]
        #[must_use]
        #[inline(always)]
        pub const fn cvsl(&self) -> u8 {
            let val = (self.0 >> 4usize) & 0x07;
            val as u8
        }
        #[doc = "desc CVSL."]
        #[inline(always)]
        pub const fn set_cvsl(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 4usize)) | (((val as u8) & 0x07) << 4usize);
        }
    }
    impl Default for Vsr {
        #[inline(always)]
        fn default() -> Vsr {
            Vsr(0)
        }
    }
    impl core::fmt::Debug for Vsr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Vsr")
                .field("rvsl", &self.rvsl())
                .field("cvsl", &self.cvsl())
                .finish()
        }
    }
}
