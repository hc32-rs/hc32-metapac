#![allow(clippy::missing_safety_doc)]
#![allow(clippy::identity_op)]
#![allow(clippy::unnecessary_cast)]
#![allow(clippy::erasing_op)]

#[doc = "desc ADC."]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Adc {
    ptr: *mut u8,
}
unsafe impl Send for Adc {}
unsafe impl Sync for Adc {}
impl Adc {
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
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x04usize) as _) }
    }
    #[doc = "desc CR1."]
    #[inline(always)]
    pub const fn cr1(self) -> crate::common::Reg<regs::Cr1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x08usize) as _) }
    }
    #[doc = "desc SQR0."]
    #[inline(always)]
    pub const fn sqr0(self) -> crate::common::Reg<regs::Sqr0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x40usize) as _) }
    }
    #[doc = "desc SQR1."]
    #[inline(always)]
    pub const fn sqr1(self) -> crate::common::Reg<regs::Sqr1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x44usize) as _) }
    }
    #[doc = "desc SQR2."]
    #[inline(always)]
    pub const fn sqr2(self) -> crate::common::Reg<regs::Sqr2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x48usize) as _) }
    }
    #[doc = "desc JQR."]
    #[inline(always)]
    pub const fn jqr(self) -> crate::common::Reg<regs::Jqr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x4cusize) as _) }
    }
    #[doc = "desc SQRRESULT0."]
    #[inline(always)]
    pub const fn sqrresult0(self) -> crate::common::Reg<regs::Sqrresult0, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x50usize) as _) }
    }
    #[doc = "desc SQRRESULT1."]
    #[inline(always)]
    pub const fn sqrresult1(self) -> crate::common::Reg<regs::Sqrresult1, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x54usize) as _) }
    }
    #[doc = "desc SQRRESULT2."]
    #[inline(always)]
    pub const fn sqrresult2(self) -> crate::common::Reg<regs::Sqrresult2, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x58usize) as _) }
    }
    #[doc = "desc SQRRESULT3."]
    #[inline(always)]
    pub const fn sqrresult3(self) -> crate::common::Reg<regs::Sqrresult3, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x5cusize) as _) }
    }
    #[doc = "desc SQRRESULT4."]
    #[inline(always)]
    pub const fn sqrresult4(self) -> crate::common::Reg<regs::Sqrresult4, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x60usize) as _) }
    }
    #[doc = "desc SQRRESULT5."]
    #[inline(always)]
    pub const fn sqrresult5(self) -> crate::common::Reg<regs::Sqrresult5, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x64usize) as _) }
    }
    #[doc = "desc SQRRESULT6."]
    #[inline(always)]
    pub const fn sqrresult6(self) -> crate::common::Reg<regs::Sqrresult6, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x68usize) as _) }
    }
    #[doc = "desc SQRRESULT7."]
    #[inline(always)]
    pub const fn sqrresult7(self) -> crate::common::Reg<regs::Sqrresult7, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x6cusize) as _) }
    }
    #[doc = "desc SQRRESULT8."]
    #[inline(always)]
    pub const fn sqrresult8(self) -> crate::common::Reg<regs::Sqrresult8, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x70usize) as _) }
    }
    #[doc = "desc SQRRESULT9."]
    #[inline(always)]
    pub const fn sqrresult9(self) -> crate::common::Reg<regs::Sqrresult9, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x74usize) as _) }
    }
    #[doc = "desc SQRRESULT10."]
    #[inline(always)]
    pub const fn sqrresult10(self) -> crate::common::Reg<regs::Sqrresult10, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x78usize) as _) }
    }
    #[doc = "desc SQRRESULT11."]
    #[inline(always)]
    pub const fn sqrresult11(self) -> crate::common::Reg<regs::Sqrresult11, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x7cusize) as _) }
    }
    #[doc = "desc SQRRESULT12."]
    #[inline(always)]
    pub const fn sqrresult12(self) -> crate::common::Reg<regs::Sqrresult12, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x80usize) as _) }
    }
    #[doc = "desc SQRRESULT13."]
    #[inline(always)]
    pub const fn sqrresult13(self) -> crate::common::Reg<regs::Sqrresult13, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x84usize) as _) }
    }
    #[doc = "desc SQRRESULT14."]
    #[inline(always)]
    pub const fn sqrresult14(self) -> crate::common::Reg<regs::Sqrresult14, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x88usize) as _) }
    }
    #[doc = "desc SQRRESULT15."]
    #[inline(always)]
    pub const fn sqrresult15(self) -> crate::common::Reg<regs::Sqrresult15, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x8cusize) as _) }
    }
    #[doc = "desc JQRRESULT0."]
    #[inline(always)]
    pub const fn jqrresult0(self) -> crate::common::Reg<regs::Jqrresult0, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x90usize) as _) }
    }
    #[doc = "desc JQRRESULT1."]
    #[inline(always)]
    pub const fn jqrresult1(self) -> crate::common::Reg<regs::Jqrresult1, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x94usize) as _) }
    }
    #[doc = "desc JQRRESULT2."]
    #[inline(always)]
    pub const fn jqrresult2(self) -> crate::common::Reg<regs::Jqrresult2, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x98usize) as _) }
    }
    #[doc = "desc JQRRESULT3."]
    #[inline(always)]
    pub const fn jqrresult3(self) -> crate::common::Reg<regs::Jqrresult3, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x9cusize) as _) }
    }
    #[doc = "desc RESULT."]
    #[inline(always)]
    pub const fn result(self) -> crate::common::Reg<regs::Result, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xa0usize) as _) }
    }
    #[doc = "desc RESULTACC."]
    #[inline(always)]
    pub const fn resultacc(self) -> crate::common::Reg<regs::Resultacc, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xa4usize) as _) }
    }
    #[doc = "desc HT."]
    #[inline(always)]
    pub const fn ht(self) -> crate::common::Reg<regs::Ht, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xa8usize) as _) }
    }
    #[doc = "desc LT."]
    #[inline(always)]
    pub const fn lt(self) -> crate::common::Reg<regs::Lt, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xacusize) as _) }
    }
    #[doc = "desc IFR."]
    #[inline(always)]
    pub const fn ifr(self) -> crate::common::Reg<regs::Ifr, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xb0usize) as _) }
    }
    #[doc = "desc ICR."]
    #[inline(always)]
    pub const fn icr(self) -> crate::common::Reg<regs::Icr, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xb4usize) as _) }
    }
    #[doc = "desc EXTTRIGGER0."]
    #[inline(always)]
    pub const fn exttrigger0(self) -> crate::common::Reg<regs::Exttrigger0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xb8usize) as _) }
    }
    #[doc = "desc EXTTRIGGER1."]
    #[inline(always)]
    pub const fn exttrigger1(self) -> crate::common::Reg<regs::Exttrigger1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xbcusize) as _) }
    }
    #[doc = "desc SGLSTART."]
    #[inline(always)]
    pub const fn sglstart(self) -> crate::common::Reg<regs::Sglstart, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xc0usize) as _) }
    }
    #[doc = "desc SQRSTART."]
    #[inline(always)]
    pub const fn sqrstart(self) -> crate::common::Reg<regs::Sqrstart, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xc4usize) as _) }
    }
    #[doc = "desc JQRSTART."]
    #[inline(always)]
    pub const fn jqrstart(self) -> crate::common::Reg<regs::Jqrstart, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xc8usize) as _) }
    }
    #[doc = "desc ALLSTART."]
    #[inline(always)]
    pub const fn allstart(self) -> crate::common::Reg<regs::Allstart, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xccusize) as _) }
    }
}
pub mod regs {
    #[doc = "desc ALLSTART."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Allstart(pub u32);
    impl Allstart {
        #[doc = "desc START."]
        #[must_use]
        #[inline(always)]
        pub const fn start(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "desc START."]
        #[inline(always)]
        pub const fn set_start(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "desc RSV."]
        #[must_use]
        #[inline(always)]
        pub const fn rsv(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[doc = "desc RSV."]
        #[inline(always)]
        pub const fn set_rsv(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for Allstart {
        #[inline(always)]
        fn default() -> Allstart {
            Allstart(0)
        }
    }
    impl core::fmt::Debug for Allstart {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Allstart")
                .field("start", &self.start())
                .field("rsv", &self.rsv())
                .finish()
        }
    }
    #[doc = "desc CR0."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Cr0(pub u32);
    impl Cr0 {
        #[doc = "desc EN."]
        #[must_use]
        #[inline(always)]
        pub const fn en(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "desc EN."]
        #[inline(always)]
        pub const fn set_en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "desc CLKDIV."]
        #[must_use]
        #[inline(always)]
        pub const fn clkdiv(&self) -> u8 {
            let val = (self.0 >> 2usize) & 0x03;
            val as u8
        }
        #[doc = "desc CLKDIV."]
        #[inline(always)]
        pub const fn set_clkdiv(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 2usize)) | (((val as u32) & 0x03) << 2usize);
        }
        #[doc = "desc SGLMUX."]
        #[must_use]
        #[inline(always)]
        pub const fn sglmux(&self) -> u8 {
            let val = (self.0 >> 4usize) & 0x1f;
            val as u8
        }
        #[doc = "desc SGLMUX."]
        #[inline(always)]
        pub const fn set_sglmux(&mut self, val: u8) {
            self.0 = (self.0 & !(0x1f << 4usize)) | (((val as u32) & 0x1f) << 4usize);
        }
        #[doc = "desc REF."]
        #[must_use]
        #[inline(always)]
        pub const fn ref_(&self) -> u8 {
            let val = (self.0 >> 9usize) & 0x03;
            val as u8
        }
        #[doc = "desc REF."]
        #[inline(always)]
        pub const fn set_ref_(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 9usize)) | (((val as u32) & 0x03) << 9usize);
        }
        #[doc = "desc BUF."]
        #[must_use]
        #[inline(always)]
        pub const fn buf(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "desc BUF."]
        #[inline(always)]
        pub const fn set_buf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[doc = "desc SAM."]
        #[must_use]
        #[inline(always)]
        pub const fn sam(&self) -> u8 {
            let val = (self.0 >> 12usize) & 0x03;
            val as u8
        }
        #[doc = "desc SAM."]
        #[inline(always)]
        pub const fn set_sam(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 12usize)) | (((val as u32) & 0x03) << 12usize);
        }
        #[doc = "desc INREFEN."]
        #[must_use]
        #[inline(always)]
        pub const fn inrefen(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "desc INREFEN."]
        #[inline(always)]
        pub const fn set_inrefen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[doc = "desc IE."]
        #[must_use]
        #[inline(always)]
        pub const fn ie(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "desc IE."]
        #[inline(always)]
        pub const fn set_ie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
        #[doc = "desc RSV."]
        #[must_use]
        #[inline(always)]
        pub const fn rsv(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[doc = "desc RSV."]
        #[inline(always)]
        pub const fn set_rsv(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
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
                .field("en", &self.en())
                .field("clkdiv", &self.clkdiv())
                .field("sglmux", &self.sglmux())
                .field("ref_", &self.ref_())
                .field("buf", &self.buf())
                .field("sam", &self.sam())
                .field("inrefen", &self.inrefen())
                .field("ie", &self.ie())
                .field("rsv", &self.rsv())
                .finish()
        }
    }
    #[doc = "desc CR1."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Cr1(pub u32);
    impl Cr1 {
        #[doc = "desc ALIGN."]
        #[must_use]
        #[inline(always)]
        pub const fn align(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "desc ALIGN."]
        #[inline(always)]
        pub const fn set_align(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "desc THCH."]
        #[must_use]
        #[inline(always)]
        pub const fn thch(&self) -> u8 {
            let val = (self.0 >> 3usize) & 0x1f;
            val as u8
        }
        #[doc = "desc THCH."]
        #[inline(always)]
        pub const fn set_thch(&mut self, val: u8) {
            self.0 = (self.0 & !(0x1f << 3usize)) | (((val as u32) & 0x1f) << 3usize);
        }
        #[doc = "desc DMASQR."]
        #[must_use]
        #[inline(always)]
        pub const fn dmasqr(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "desc DMASQR."]
        #[inline(always)]
        pub const fn set_dmasqr(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "desc DMAJQR."]
        #[must_use]
        #[inline(always)]
        pub const fn dmajqr(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "desc DMAJQR."]
        #[inline(always)]
        pub const fn set_dmajqr(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "desc MODE."]
        #[must_use]
        #[inline(always)]
        pub const fn mode(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "desc MODE."]
        #[inline(always)]
        pub const fn set_mode(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[doc = "desc RACCEN."]
        #[must_use]
        #[inline(always)]
        pub const fn raccen(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "desc RACCEN."]
        #[inline(always)]
        pub const fn set_raccen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[doc = "desc LTCMP."]
        #[must_use]
        #[inline(always)]
        pub const fn ltcmp(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "desc LTCMP."]
        #[inline(always)]
        pub const fn set_ltcmp(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "desc HTCMP."]
        #[must_use]
        #[inline(always)]
        pub const fn htcmp(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[doc = "desc HTCMP."]
        #[inline(always)]
        pub const fn set_htcmp(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[doc = "desc REGCMP."]
        #[must_use]
        #[inline(always)]
        pub const fn regcmp(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "desc REGCMP."]
        #[inline(always)]
        pub const fn set_regcmp(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[doc = "desc RACCCLR."]
        #[must_use]
        #[inline(always)]
        pub const fn raccclr(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "desc RACCCLR."]
        #[inline(always)]
        pub const fn set_raccclr(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
        #[doc = "desc RSV."]
        #[must_use]
        #[inline(always)]
        pub const fn rsv(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[doc = "desc RSV."]
        #[inline(always)]
        pub const fn set_rsv(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
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
                .field("align", &self.align())
                .field("thch", &self.thch())
                .field("dmasqr", &self.dmasqr())
                .field("dmajqr", &self.dmajqr())
                .field("mode", &self.mode())
                .field("raccen", &self.raccen())
                .field("ltcmp", &self.ltcmp())
                .field("htcmp", &self.htcmp())
                .field("regcmp", &self.regcmp())
                .field("raccclr", &self.raccclr())
                .field("rsv", &self.rsv())
                .finish()
        }
    }
    #[doc = "desc EXTTRIGGER0."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Exttrigger0(pub u32);
    impl Exttrigger0 {
        #[doc = "desc TIM0."]
        #[must_use]
        #[inline(always)]
        pub const fn tim0(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "desc TIM0."]
        #[inline(always)]
        pub const fn set_tim0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "desc TIM1."]
        #[must_use]
        #[inline(always)]
        pub const fn tim1(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "desc TIM1."]
        #[inline(always)]
        pub const fn set_tim1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "desc TIM2."]
        #[must_use]
        #[inline(always)]
        pub const fn tim2(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "desc TIM2."]
        #[inline(always)]
        pub const fn set_tim2(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "desc TIM3."]
        #[must_use]
        #[inline(always)]
        pub const fn tim3(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "desc TIM3."]
        #[inline(always)]
        pub const fn set_tim3(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "desc TIM4."]
        #[must_use]
        #[inline(always)]
        pub const fn tim4(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "desc TIM4."]
        #[inline(always)]
        pub const fn set_tim4(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "desc TIM5."]
        #[must_use]
        #[inline(always)]
        pub const fn tim5(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "desc TIM5."]
        #[inline(always)]
        pub const fn set_tim5(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "desc TIM6."]
        #[must_use]
        #[inline(always)]
        pub const fn tim6(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "desc TIM6."]
        #[inline(always)]
        pub const fn set_tim6(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "desc UART0."]
        #[must_use]
        #[inline(always)]
        pub const fn uart0(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "desc UART0."]
        #[inline(always)]
        pub const fn set_uart0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "desc UART1."]
        #[must_use]
        #[inline(always)]
        pub const fn uart1(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "desc UART1."]
        #[inline(always)]
        pub const fn set_uart1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "desc LPUART0."]
        #[must_use]
        #[inline(always)]
        pub const fn lpuart0(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "desc LPUART0."]
        #[inline(always)]
        pub const fn set_lpuart0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "desc LPUART1."]
        #[must_use]
        #[inline(always)]
        pub const fn lpuart1(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "desc LPUART1."]
        #[inline(always)]
        pub const fn set_lpuart1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[doc = "desc VC0."]
        #[must_use]
        #[inline(always)]
        pub const fn vc0(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "desc VC0."]
        #[inline(always)]
        pub const fn set_vc0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[doc = "desc VC1."]
        #[must_use]
        #[inline(always)]
        pub const fn vc1(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "desc VC1."]
        #[inline(always)]
        pub const fn set_vc1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "desc RTC."]
        #[must_use]
        #[inline(always)]
        pub const fn rtc(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[doc = "desc RTC."]
        #[inline(always)]
        pub const fn set_rtc(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[doc = "desc PCA."]
        #[must_use]
        #[inline(always)]
        pub const fn pca(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "desc PCA."]
        #[inline(always)]
        pub const fn set_pca(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[doc = "desc SPI0."]
        #[must_use]
        #[inline(always)]
        pub const fn spi0(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "desc SPI0."]
        #[inline(always)]
        pub const fn set_spi0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
        #[doc = "desc SPI1."]
        #[must_use]
        #[inline(always)]
        pub const fn spi1(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "desc SPI1."]
        #[inline(always)]
        pub const fn set_spi1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "desc DMA."]
        #[must_use]
        #[inline(always)]
        pub const fn dma(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "desc DMA."]
        #[inline(always)]
        pub const fn set_dma(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[doc = "desc PA03."]
        #[must_use]
        #[inline(always)]
        pub const fn pa03(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[doc = "desc PA03."]
        #[inline(always)]
        pub const fn set_pa03(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
        #[doc = "desc PB03."]
        #[must_use]
        #[inline(always)]
        pub const fn pb03(&self) -> bool {
            let val = (self.0 >> 19usize) & 0x01;
            val != 0
        }
        #[doc = "desc PB03."]
        #[inline(always)]
        pub const fn set_pb03(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
        }
        #[doc = "desc PC03."]
        #[must_use]
        #[inline(always)]
        pub const fn pc03(&self) -> bool {
            let val = (self.0 >> 20usize) & 0x01;
            val != 0
        }
        #[doc = "desc PC03."]
        #[inline(always)]
        pub const fn set_pc03(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
        }
        #[doc = "desc PD03."]
        #[must_use]
        #[inline(always)]
        pub const fn pd03(&self) -> bool {
            let val = (self.0 >> 21usize) & 0x01;
            val != 0
        }
        #[doc = "desc PD03."]
        #[inline(always)]
        pub const fn set_pd03(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
        }
        #[doc = "desc PA07."]
        #[must_use]
        #[inline(always)]
        pub const fn pa07(&self) -> bool {
            let val = (self.0 >> 22usize) & 0x01;
            val != 0
        }
        #[doc = "desc PA07."]
        #[inline(always)]
        pub const fn set_pa07(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
        }
        #[doc = "desc PB07."]
        #[must_use]
        #[inline(always)]
        pub const fn pb07(&self) -> bool {
            let val = (self.0 >> 23usize) & 0x01;
            val != 0
        }
        #[doc = "desc PB07."]
        #[inline(always)]
        pub const fn set_pb07(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
        }
        #[doc = "desc PC07."]
        #[must_use]
        #[inline(always)]
        pub const fn pc07(&self) -> bool {
            let val = (self.0 >> 24usize) & 0x01;
            val != 0
        }
        #[doc = "desc PC07."]
        #[inline(always)]
        pub const fn set_pc07(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
        }
        #[doc = "desc PA11."]
        #[must_use]
        #[inline(always)]
        pub const fn pa11(&self) -> bool {
            let val = (self.0 >> 26usize) & 0x01;
            val != 0
        }
        #[doc = "desc PA11."]
        #[inline(always)]
        pub const fn set_pa11(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
        }
        #[doc = "desc PB11."]
        #[must_use]
        #[inline(always)]
        pub const fn pb11(&self) -> bool {
            let val = (self.0 >> 27usize) & 0x01;
            val != 0
        }
        #[doc = "desc PB11."]
        #[inline(always)]
        pub const fn set_pb11(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
        }
        #[doc = "desc PC11."]
        #[must_use]
        #[inline(always)]
        pub const fn pc11(&self) -> bool {
            let val = (self.0 >> 28usize) & 0x01;
            val != 0
        }
        #[doc = "desc PC11."]
        #[inline(always)]
        pub const fn set_pc11(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
        }
        #[doc = "desc PA15."]
        #[must_use]
        #[inline(always)]
        pub const fn pa15(&self) -> bool {
            let val = (self.0 >> 29usize) & 0x01;
            val != 0
        }
        #[doc = "desc PA15."]
        #[inline(always)]
        pub const fn set_pa15(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
        }
        #[doc = "desc PB15."]
        #[must_use]
        #[inline(always)]
        pub const fn pb15(&self) -> bool {
            let val = (self.0 >> 30usize) & 0x01;
            val != 0
        }
        #[doc = "desc PB15."]
        #[inline(always)]
        pub const fn set_pb15(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
        }
        #[doc = "desc PC15."]
        #[must_use]
        #[inline(always)]
        pub const fn pc15(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[doc = "desc PC15."]
        #[inline(always)]
        pub const fn set_pc15(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for Exttrigger0 {
        #[inline(always)]
        fn default() -> Exttrigger0 {
            Exttrigger0(0)
        }
    }
    impl core::fmt::Debug for Exttrigger0 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Exttrigger0")
                .field("tim0", &self.tim0())
                .field("tim1", &self.tim1())
                .field("tim2", &self.tim2())
                .field("tim3", &self.tim3())
                .field("tim4", &self.tim4())
                .field("tim5", &self.tim5())
                .field("tim6", &self.tim6())
                .field("uart0", &self.uart0())
                .field("uart1", &self.uart1())
                .field("lpuart0", &self.lpuart0())
                .field("lpuart1", &self.lpuart1())
                .field("vc0", &self.vc0())
                .field("vc1", &self.vc1())
                .field("rtc", &self.rtc())
                .field("pca", &self.pca())
                .field("spi0", &self.spi0())
                .field("spi1", &self.spi1())
                .field("dma", &self.dma())
                .field("pa03", &self.pa03())
                .field("pb03", &self.pb03())
                .field("pc03", &self.pc03())
                .field("pd03", &self.pd03())
                .field("pa07", &self.pa07())
                .field("pb07", &self.pb07())
                .field("pc07", &self.pc07())
                .field("pa11", &self.pa11())
                .field("pb11", &self.pb11())
                .field("pc11", &self.pc11())
                .field("pa15", &self.pa15())
                .field("pb15", &self.pb15())
                .field("pc15", &self.pc15())
                .finish()
        }
    }
    #[doc = "desc EXTTRIGGER1."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Exttrigger1(pub u32);
    impl Exttrigger1 {
        #[doc = "desc TIM0."]
        #[must_use]
        #[inline(always)]
        pub const fn tim0(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "desc TIM0."]
        #[inline(always)]
        pub const fn set_tim0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "desc TIM1."]
        #[must_use]
        #[inline(always)]
        pub const fn tim1(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "desc TIM1."]
        #[inline(always)]
        pub const fn set_tim1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "desc TIM2."]
        #[must_use]
        #[inline(always)]
        pub const fn tim2(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "desc TIM2."]
        #[inline(always)]
        pub const fn set_tim2(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "desc TIM3."]
        #[must_use]
        #[inline(always)]
        pub const fn tim3(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "desc TIM3."]
        #[inline(always)]
        pub const fn set_tim3(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "desc TIM4."]
        #[must_use]
        #[inline(always)]
        pub const fn tim4(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "desc TIM4."]
        #[inline(always)]
        pub const fn set_tim4(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "desc TIM5."]
        #[must_use]
        #[inline(always)]
        pub const fn tim5(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "desc TIM5."]
        #[inline(always)]
        pub const fn set_tim5(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "desc TIM6."]
        #[must_use]
        #[inline(always)]
        pub const fn tim6(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "desc TIM6."]
        #[inline(always)]
        pub const fn set_tim6(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "desc UART0."]
        #[must_use]
        #[inline(always)]
        pub const fn uart0(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "desc UART0."]
        #[inline(always)]
        pub const fn set_uart0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "desc UART1."]
        #[must_use]
        #[inline(always)]
        pub const fn uart1(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "desc UART1."]
        #[inline(always)]
        pub const fn set_uart1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "desc LPUART0."]
        #[must_use]
        #[inline(always)]
        pub const fn lpuart0(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "desc LPUART0."]
        #[inline(always)]
        pub const fn set_lpuart0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "desc LPUART1."]
        #[must_use]
        #[inline(always)]
        pub const fn lpuart1(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "desc LPUART1."]
        #[inline(always)]
        pub const fn set_lpuart1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[doc = "desc VC0."]
        #[must_use]
        #[inline(always)]
        pub const fn vc0(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "desc VC0."]
        #[inline(always)]
        pub const fn set_vc0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[doc = "desc VC1."]
        #[must_use]
        #[inline(always)]
        pub const fn vc1(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "desc VC1."]
        #[inline(always)]
        pub const fn set_vc1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "desc RTC."]
        #[must_use]
        #[inline(always)]
        pub const fn rtc(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[doc = "desc RTC."]
        #[inline(always)]
        pub const fn set_rtc(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[doc = "desc PCA."]
        #[must_use]
        #[inline(always)]
        pub const fn pca(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "desc PCA."]
        #[inline(always)]
        pub const fn set_pca(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[doc = "desc SPI0."]
        #[must_use]
        #[inline(always)]
        pub const fn spi0(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "desc SPI0."]
        #[inline(always)]
        pub const fn set_spi0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
        #[doc = "desc SPI1."]
        #[must_use]
        #[inline(always)]
        pub const fn spi1(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "desc SPI1."]
        #[inline(always)]
        pub const fn set_spi1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "desc DMA."]
        #[must_use]
        #[inline(always)]
        pub const fn dma(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "desc DMA."]
        #[inline(always)]
        pub const fn set_dma(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[doc = "desc PA03."]
        #[must_use]
        #[inline(always)]
        pub const fn pa03(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[doc = "desc PA03."]
        #[inline(always)]
        pub const fn set_pa03(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
        #[doc = "desc PB03."]
        #[must_use]
        #[inline(always)]
        pub const fn pb03(&self) -> bool {
            let val = (self.0 >> 19usize) & 0x01;
            val != 0
        }
        #[doc = "desc PB03."]
        #[inline(always)]
        pub const fn set_pb03(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
        }
        #[doc = "desc PC03."]
        #[must_use]
        #[inline(always)]
        pub const fn pc03(&self) -> bool {
            let val = (self.0 >> 20usize) & 0x01;
            val != 0
        }
        #[doc = "desc PC03."]
        #[inline(always)]
        pub const fn set_pc03(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
        }
        #[doc = "desc PD03."]
        #[must_use]
        #[inline(always)]
        pub const fn pd03(&self) -> bool {
            let val = (self.0 >> 21usize) & 0x01;
            val != 0
        }
        #[doc = "desc PD03."]
        #[inline(always)]
        pub const fn set_pd03(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
        }
        #[doc = "desc PA07."]
        #[must_use]
        #[inline(always)]
        pub const fn pa07(&self) -> bool {
            let val = (self.0 >> 22usize) & 0x01;
            val != 0
        }
        #[doc = "desc PA07."]
        #[inline(always)]
        pub const fn set_pa07(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
        }
        #[doc = "desc PB07."]
        #[must_use]
        #[inline(always)]
        pub const fn pb07(&self) -> bool {
            let val = (self.0 >> 23usize) & 0x01;
            val != 0
        }
        #[doc = "desc PB07."]
        #[inline(always)]
        pub const fn set_pb07(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
        }
        #[doc = "desc PC07."]
        #[must_use]
        #[inline(always)]
        pub const fn pc07(&self) -> bool {
            let val = (self.0 >> 24usize) & 0x01;
            val != 0
        }
        #[doc = "desc PC07."]
        #[inline(always)]
        pub const fn set_pc07(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
        }
        #[doc = "desc PA11."]
        #[must_use]
        #[inline(always)]
        pub const fn pa11(&self) -> bool {
            let val = (self.0 >> 26usize) & 0x01;
            val != 0
        }
        #[doc = "desc PA11."]
        #[inline(always)]
        pub const fn set_pa11(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
        }
        #[doc = "desc PB11."]
        #[must_use]
        #[inline(always)]
        pub const fn pb11(&self) -> bool {
            let val = (self.0 >> 27usize) & 0x01;
            val != 0
        }
        #[doc = "desc PB11."]
        #[inline(always)]
        pub const fn set_pb11(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
        }
        #[doc = "desc PC11."]
        #[must_use]
        #[inline(always)]
        pub const fn pc11(&self) -> bool {
            let val = (self.0 >> 28usize) & 0x01;
            val != 0
        }
        #[doc = "desc PC11."]
        #[inline(always)]
        pub const fn set_pc11(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
        }
        #[doc = "desc PA15."]
        #[must_use]
        #[inline(always)]
        pub const fn pa15(&self) -> bool {
            let val = (self.0 >> 29usize) & 0x01;
            val != 0
        }
        #[doc = "desc PA15."]
        #[inline(always)]
        pub const fn set_pa15(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
        }
        #[doc = "desc PB15."]
        #[must_use]
        #[inline(always)]
        pub const fn pb15(&self) -> bool {
            let val = (self.0 >> 30usize) & 0x01;
            val != 0
        }
        #[doc = "desc PB15."]
        #[inline(always)]
        pub const fn set_pb15(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
        }
        #[doc = "desc PC15."]
        #[must_use]
        #[inline(always)]
        pub const fn pc15(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[doc = "desc PC15."]
        #[inline(always)]
        pub const fn set_pc15(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for Exttrigger1 {
        #[inline(always)]
        fn default() -> Exttrigger1 {
            Exttrigger1(0)
        }
    }
    impl core::fmt::Debug for Exttrigger1 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Exttrigger1")
                .field("tim0", &self.tim0())
                .field("tim1", &self.tim1())
                .field("tim2", &self.tim2())
                .field("tim3", &self.tim3())
                .field("tim4", &self.tim4())
                .field("tim5", &self.tim5())
                .field("tim6", &self.tim6())
                .field("uart0", &self.uart0())
                .field("uart1", &self.uart1())
                .field("lpuart0", &self.lpuart0())
                .field("lpuart1", &self.lpuart1())
                .field("vc0", &self.vc0())
                .field("vc1", &self.vc1())
                .field("rtc", &self.rtc())
                .field("pca", &self.pca())
                .field("spi0", &self.spi0())
                .field("spi1", &self.spi1())
                .field("dma", &self.dma())
                .field("pa03", &self.pa03())
                .field("pb03", &self.pb03())
                .field("pc03", &self.pc03())
                .field("pd03", &self.pd03())
                .field("pa07", &self.pa07())
                .field("pb07", &self.pb07())
                .field("pc07", &self.pc07())
                .field("pa11", &self.pa11())
                .field("pb11", &self.pb11())
                .field("pc11", &self.pc11())
                .field("pa15", &self.pa15())
                .field("pb15", &self.pb15())
                .field("pc15", &self.pc15())
                .finish()
        }
    }
    #[doc = "desc HT."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ht(pub u32);
    impl Ht {
        #[doc = "desc HT."]
        #[must_use]
        #[inline(always)]
        pub const fn ht(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x0fff;
            val as u16
        }
        #[doc = "desc HT."]
        #[inline(always)]
        pub const fn set_ht(&mut self, val: u16) {
            self.0 = (self.0 & !(0x0fff << 0usize)) | (((val as u32) & 0x0fff) << 0usize);
        }
        #[doc = "desc RSV."]
        #[must_use]
        #[inline(always)]
        pub const fn rsv(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[doc = "desc RSV."]
        #[inline(always)]
        pub const fn set_rsv(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for Ht {
        #[inline(always)]
        fn default() -> Ht {
            Ht(0)
        }
    }
    impl core::fmt::Debug for Ht {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Ht")
                .field("ht", &self.ht())
                .field("rsv", &self.rsv())
                .finish()
        }
    }
    #[doc = "desc ICR."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Icr(pub u32);
    impl Icr {
        #[doc = "desc SGLIC."]
        #[must_use]
        #[inline(always)]
        pub const fn sglic(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "desc SGLIC."]
        #[inline(always)]
        pub const fn set_sglic(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "desc LTIC."]
        #[must_use]
        #[inline(always)]
        pub const fn ltic(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "desc LTIC."]
        #[inline(always)]
        pub const fn set_ltic(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "desc HTIC."]
        #[must_use]
        #[inline(always)]
        pub const fn htic(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "desc HTIC."]
        #[inline(always)]
        pub const fn set_htic(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "desc REGIC."]
        #[must_use]
        #[inline(always)]
        pub const fn regic(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "desc REGIC."]
        #[inline(always)]
        pub const fn set_regic(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "desc SQRIC."]
        #[must_use]
        #[inline(always)]
        pub const fn sqric(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "desc SQRIC."]
        #[inline(always)]
        pub const fn set_sqric(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "desc JQRIC."]
        #[must_use]
        #[inline(always)]
        pub const fn jqric(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "desc JQRIC."]
        #[inline(always)]
        pub const fn set_jqric(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "desc RSV."]
        #[must_use]
        #[inline(always)]
        pub const fn rsv(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[doc = "desc RSV."]
        #[inline(always)]
        pub const fn set_rsv(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
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
                .field("sglic", &self.sglic())
                .field("ltic", &self.ltic())
                .field("htic", &self.htic())
                .field("regic", &self.regic())
                .field("sqric", &self.sqric())
                .field("jqric", &self.jqric())
                .field("rsv", &self.rsv())
                .finish()
        }
    }
    #[doc = "desc IFR."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ifr(pub u32);
    impl Ifr {
        #[doc = "desc SGLIF."]
        #[must_use]
        #[inline(always)]
        pub const fn sglif(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "desc SGLIF."]
        #[inline(always)]
        pub const fn set_sglif(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "desc LTIF."]
        #[must_use]
        #[inline(always)]
        pub const fn ltif(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "desc LTIF."]
        #[inline(always)]
        pub const fn set_ltif(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "desc HTIF."]
        #[must_use]
        #[inline(always)]
        pub const fn htif(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "desc HTIF."]
        #[inline(always)]
        pub const fn set_htif(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "desc REGIF."]
        #[must_use]
        #[inline(always)]
        pub const fn regif(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "desc REGIF."]
        #[inline(always)]
        pub const fn set_regif(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "desc SQRIF."]
        #[must_use]
        #[inline(always)]
        pub const fn sqrif(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "desc SQRIF."]
        #[inline(always)]
        pub const fn set_sqrif(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "desc JQRIF."]
        #[must_use]
        #[inline(always)]
        pub const fn jqrif(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "desc JQRIF."]
        #[inline(always)]
        pub const fn set_jqrif(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "desc RSV."]
        #[must_use]
        #[inline(always)]
        pub const fn rsv(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[doc = "desc RSV."]
        #[inline(always)]
        pub const fn set_rsv(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
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
                .field("sglif", &self.sglif())
                .field("ltif", &self.ltif())
                .field("htif", &self.htif())
                .field("regif", &self.regif())
                .field("sqrif", &self.sqrif())
                .field("jqrif", &self.jqrif())
                .field("rsv", &self.rsv())
                .finish()
        }
    }
    #[doc = "desc JQR."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Jqr(pub u32);
    impl Jqr {
        #[doc = "desc CH0MUX."]
        #[must_use]
        #[inline(always)]
        pub const fn ch0mux(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x1f;
            val as u8
        }
        #[doc = "desc CH0MUX."]
        #[inline(always)]
        pub const fn set_ch0mux(&mut self, val: u8) {
            self.0 = (self.0 & !(0x1f << 0usize)) | (((val as u32) & 0x1f) << 0usize);
        }
        #[doc = "desc CH1MUX."]
        #[must_use]
        #[inline(always)]
        pub const fn ch1mux(&self) -> u8 {
            let val = (self.0 >> 5usize) & 0x1f;
            val as u8
        }
        #[doc = "desc CH1MUX."]
        #[inline(always)]
        pub const fn set_ch1mux(&mut self, val: u8) {
            self.0 = (self.0 & !(0x1f << 5usize)) | (((val as u32) & 0x1f) << 5usize);
        }
        #[doc = "desc CH2MUX."]
        #[must_use]
        #[inline(always)]
        pub const fn ch2mux(&self) -> u8 {
            let val = (self.0 >> 10usize) & 0x1f;
            val as u8
        }
        #[doc = "desc CH2MUX."]
        #[inline(always)]
        pub const fn set_ch2mux(&mut self, val: u8) {
            self.0 = (self.0 & !(0x1f << 10usize)) | (((val as u32) & 0x1f) << 10usize);
        }
        #[doc = "desc CH3MUX."]
        #[must_use]
        #[inline(always)]
        pub const fn ch3mux(&self) -> u8 {
            let val = (self.0 >> 15usize) & 0x1f;
            val as u8
        }
        #[doc = "desc CH3MUX."]
        #[inline(always)]
        pub const fn set_ch3mux(&mut self, val: u8) {
            self.0 = (self.0 & !(0x1f << 15usize)) | (((val as u32) & 0x1f) << 15usize);
        }
        #[doc = "desc CNT."]
        #[must_use]
        #[inline(always)]
        pub const fn cnt(&self) -> u8 {
            let val = (self.0 >> 20usize) & 0x03;
            val as u8
        }
        #[doc = "desc CNT."]
        #[inline(always)]
        pub const fn set_cnt(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 20usize)) | (((val as u32) & 0x03) << 20usize);
        }
        #[doc = "desc RSV."]
        #[must_use]
        #[inline(always)]
        pub const fn rsv(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[doc = "desc RSV."]
        #[inline(always)]
        pub const fn set_rsv(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for Jqr {
        #[inline(always)]
        fn default() -> Jqr {
            Jqr(0)
        }
    }
    impl core::fmt::Debug for Jqr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Jqr")
                .field("ch0mux", &self.ch0mux())
                .field("ch1mux", &self.ch1mux())
                .field("ch2mux", &self.ch2mux())
                .field("ch3mux", &self.ch3mux())
                .field("cnt", &self.cnt())
                .field("rsv", &self.rsv())
                .finish()
        }
    }
    #[doc = "desc JQRRESULT0."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Jqrresult0(pub u32);
    impl Jqrresult0 {
        #[doc = "desc RESULT."]
        #[must_use]
        #[inline(always)]
        pub const fn result(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x0fff;
            val as u16
        }
        #[doc = "desc RESULT."]
        #[inline(always)]
        pub const fn set_result(&mut self, val: u16) {
            self.0 = (self.0 & !(0x0fff << 0usize)) | (((val as u32) & 0x0fff) << 0usize);
        }
        #[doc = "desc RSV."]
        #[must_use]
        #[inline(always)]
        pub const fn rsv(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[doc = "desc RSV."]
        #[inline(always)]
        pub const fn set_rsv(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for Jqrresult0 {
        #[inline(always)]
        fn default() -> Jqrresult0 {
            Jqrresult0(0)
        }
    }
    impl core::fmt::Debug for Jqrresult0 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Jqrresult0")
                .field("result", &self.result())
                .field("rsv", &self.rsv())
                .finish()
        }
    }
    #[doc = "desc JQRRESULT1."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Jqrresult1(pub u32);
    impl Jqrresult1 {
        #[doc = "desc RESULT."]
        #[must_use]
        #[inline(always)]
        pub const fn result(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x0fff;
            val as u16
        }
        #[doc = "desc RESULT."]
        #[inline(always)]
        pub const fn set_result(&mut self, val: u16) {
            self.0 = (self.0 & !(0x0fff << 0usize)) | (((val as u32) & 0x0fff) << 0usize);
        }
        #[doc = "desc RSV."]
        #[must_use]
        #[inline(always)]
        pub const fn rsv(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[doc = "desc RSV."]
        #[inline(always)]
        pub const fn set_rsv(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for Jqrresult1 {
        #[inline(always)]
        fn default() -> Jqrresult1 {
            Jqrresult1(0)
        }
    }
    impl core::fmt::Debug for Jqrresult1 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Jqrresult1")
                .field("result", &self.result())
                .field("rsv", &self.rsv())
                .finish()
        }
    }
    #[doc = "desc JQRRESULT2."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Jqrresult2(pub u32);
    impl Jqrresult2 {
        #[doc = "desc RESULT."]
        #[must_use]
        #[inline(always)]
        pub const fn result(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x0fff;
            val as u16
        }
        #[doc = "desc RESULT."]
        #[inline(always)]
        pub const fn set_result(&mut self, val: u16) {
            self.0 = (self.0 & !(0x0fff << 0usize)) | (((val as u32) & 0x0fff) << 0usize);
        }
        #[doc = "desc RSV."]
        #[must_use]
        #[inline(always)]
        pub const fn rsv(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[doc = "desc RSV."]
        #[inline(always)]
        pub const fn set_rsv(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for Jqrresult2 {
        #[inline(always)]
        fn default() -> Jqrresult2 {
            Jqrresult2(0)
        }
    }
    impl core::fmt::Debug for Jqrresult2 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Jqrresult2")
                .field("result", &self.result())
                .field("rsv", &self.rsv())
                .finish()
        }
    }
    #[doc = "desc JQRRESULT3."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Jqrresult3(pub u32);
    impl Jqrresult3 {
        #[doc = "desc RESULT."]
        #[must_use]
        #[inline(always)]
        pub const fn result(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x0fff;
            val as u16
        }
        #[doc = "desc RESULT."]
        #[inline(always)]
        pub const fn set_result(&mut self, val: u16) {
            self.0 = (self.0 & !(0x0fff << 0usize)) | (((val as u32) & 0x0fff) << 0usize);
        }
        #[doc = "desc RSV."]
        #[must_use]
        #[inline(always)]
        pub const fn rsv(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[doc = "desc RSV."]
        #[inline(always)]
        pub const fn set_rsv(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for Jqrresult3 {
        #[inline(always)]
        fn default() -> Jqrresult3 {
            Jqrresult3(0)
        }
    }
    impl core::fmt::Debug for Jqrresult3 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Jqrresult3")
                .field("result", &self.result())
                .field("rsv", &self.rsv())
                .finish()
        }
    }
    #[doc = "desc JQRSTART."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Jqrstart(pub u32);
    impl Jqrstart {
        #[doc = "desc START."]
        #[must_use]
        #[inline(always)]
        pub const fn start(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "desc START."]
        #[inline(always)]
        pub const fn set_start(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "desc RSV."]
        #[must_use]
        #[inline(always)]
        pub const fn rsv(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[doc = "desc RSV."]
        #[inline(always)]
        pub const fn set_rsv(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for Jqrstart {
        #[inline(always)]
        fn default() -> Jqrstart {
            Jqrstart(0)
        }
    }
    impl core::fmt::Debug for Jqrstart {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Jqrstart")
                .field("start", &self.start())
                .field("rsv", &self.rsv())
                .finish()
        }
    }
    #[doc = "desc LT."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Lt(pub u32);
    impl Lt {
        #[doc = "desc LT."]
        #[must_use]
        #[inline(always)]
        pub const fn lt(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x0fff;
            val as u16
        }
        #[doc = "desc LT."]
        #[inline(always)]
        pub const fn set_lt(&mut self, val: u16) {
            self.0 = (self.0 & !(0x0fff << 0usize)) | (((val as u32) & 0x0fff) << 0usize);
        }
        #[doc = "desc RSV."]
        #[must_use]
        #[inline(always)]
        pub const fn rsv(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[doc = "desc RSV."]
        #[inline(always)]
        pub const fn set_rsv(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for Lt {
        #[inline(always)]
        fn default() -> Lt {
            Lt(0)
        }
    }
    impl core::fmt::Debug for Lt {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Lt")
                .field("lt", &self.lt())
                .field("rsv", &self.rsv())
                .finish()
        }
    }
    #[doc = "desc RESULT."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Result(pub u32);
    impl Result {
        #[doc = "desc RESULT."]
        #[must_use]
        #[inline(always)]
        pub const fn result(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x0fff;
            val as u16
        }
        #[doc = "desc RESULT."]
        #[inline(always)]
        pub const fn set_result(&mut self, val: u16) {
            self.0 = (self.0 & !(0x0fff << 0usize)) | (((val as u32) & 0x0fff) << 0usize);
        }
        #[doc = "desc RSV."]
        #[must_use]
        #[inline(always)]
        pub const fn rsv(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[doc = "desc RSV."]
        #[inline(always)]
        pub const fn set_rsv(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for Result {
        #[inline(always)]
        fn default() -> Result {
            Result(0)
        }
    }
    impl core::fmt::Debug for Result {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Result")
                .field("result", &self.result())
                .field("rsv", &self.rsv())
                .finish()
        }
    }
    #[doc = "desc RESULTACC."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Resultacc(pub u32);
    impl Resultacc {
        #[doc = "desc RESULTACC."]
        #[must_use]
        #[inline(always)]
        pub const fn resultacc(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0x000f_ffff;
            val as u32
        }
        #[doc = "desc RESULTACC."]
        #[inline(always)]
        pub const fn set_resultacc(&mut self, val: u32) {
            self.0 = (self.0 & !(0x000f_ffff << 0usize)) | (((val as u32) & 0x000f_ffff) << 0usize);
        }
        #[doc = "desc RSV."]
        #[must_use]
        #[inline(always)]
        pub const fn rsv(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[doc = "desc RSV."]
        #[inline(always)]
        pub const fn set_rsv(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for Resultacc {
        #[inline(always)]
        fn default() -> Resultacc {
            Resultacc(0)
        }
    }
    impl core::fmt::Debug for Resultacc {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Resultacc")
                .field("resultacc", &self.resultacc())
                .field("rsv", &self.rsv())
                .finish()
        }
    }
    #[doc = "desc SGLSTART."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Sglstart(pub u32);
    impl Sglstart {
        #[doc = "desc START."]
        #[must_use]
        #[inline(always)]
        pub const fn start(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "desc START."]
        #[inline(always)]
        pub const fn set_start(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "desc RSV."]
        #[must_use]
        #[inline(always)]
        pub const fn rsv(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[doc = "desc RSV."]
        #[inline(always)]
        pub const fn set_rsv(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for Sglstart {
        #[inline(always)]
        fn default() -> Sglstart {
            Sglstart(0)
        }
    }
    impl core::fmt::Debug for Sglstart {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Sglstart")
                .field("start", &self.start())
                .field("rsv", &self.rsv())
                .finish()
        }
    }
    #[doc = "desc SQR0."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Sqr0(pub u32);
    impl Sqr0 {
        #[doc = "desc CH0MUX."]
        #[must_use]
        #[inline(always)]
        pub const fn ch0mux(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x1f;
            val as u8
        }
        #[doc = "desc CH0MUX."]
        #[inline(always)]
        pub const fn set_ch0mux(&mut self, val: u8) {
            self.0 = (self.0 & !(0x1f << 0usize)) | (((val as u32) & 0x1f) << 0usize);
        }
        #[doc = "desc CH1MUX."]
        #[must_use]
        #[inline(always)]
        pub const fn ch1mux(&self) -> u8 {
            let val = (self.0 >> 5usize) & 0x1f;
            val as u8
        }
        #[doc = "desc CH1MUX."]
        #[inline(always)]
        pub const fn set_ch1mux(&mut self, val: u8) {
            self.0 = (self.0 & !(0x1f << 5usize)) | (((val as u32) & 0x1f) << 5usize);
        }
        #[doc = "desc CH2MUX."]
        #[must_use]
        #[inline(always)]
        pub const fn ch2mux(&self) -> u8 {
            let val = (self.0 >> 10usize) & 0x1f;
            val as u8
        }
        #[doc = "desc CH2MUX."]
        #[inline(always)]
        pub const fn set_ch2mux(&mut self, val: u8) {
            self.0 = (self.0 & !(0x1f << 10usize)) | (((val as u32) & 0x1f) << 10usize);
        }
        #[doc = "desc CH3MUX."]
        #[must_use]
        #[inline(always)]
        pub const fn ch3mux(&self) -> u8 {
            let val = (self.0 >> 15usize) & 0x1f;
            val as u8
        }
        #[doc = "desc CH3MUX."]
        #[inline(always)]
        pub const fn set_ch3mux(&mut self, val: u8) {
            self.0 = (self.0 & !(0x1f << 15usize)) | (((val as u32) & 0x1f) << 15usize);
        }
        #[doc = "desc CH4MUX."]
        #[must_use]
        #[inline(always)]
        pub const fn ch4mux(&self) -> u8 {
            let val = (self.0 >> 20usize) & 0x1f;
            val as u8
        }
        #[doc = "desc CH4MUX."]
        #[inline(always)]
        pub const fn set_ch4mux(&mut self, val: u8) {
            self.0 = (self.0 & !(0x1f << 20usize)) | (((val as u32) & 0x1f) << 20usize);
        }
        #[doc = "desc CH5MUX."]
        #[must_use]
        #[inline(always)]
        pub const fn ch5mux(&self) -> u8 {
            let val = (self.0 >> 25usize) & 0x1f;
            val as u8
        }
        #[doc = "desc CH5MUX."]
        #[inline(always)]
        pub const fn set_ch5mux(&mut self, val: u8) {
            self.0 = (self.0 & !(0x1f << 25usize)) | (((val as u32) & 0x1f) << 25usize);
        }
        #[doc = "desc RSV."]
        #[must_use]
        #[inline(always)]
        pub const fn rsv(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[doc = "desc RSV."]
        #[inline(always)]
        pub const fn set_rsv(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for Sqr0 {
        #[inline(always)]
        fn default() -> Sqr0 {
            Sqr0(0)
        }
    }
    impl core::fmt::Debug for Sqr0 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Sqr0")
                .field("ch0mux", &self.ch0mux())
                .field("ch1mux", &self.ch1mux())
                .field("ch2mux", &self.ch2mux())
                .field("ch3mux", &self.ch3mux())
                .field("ch4mux", &self.ch4mux())
                .field("ch5mux", &self.ch5mux())
                .field("rsv", &self.rsv())
                .finish()
        }
    }
    #[doc = "desc SQR1."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Sqr1(pub u32);
    impl Sqr1 {
        #[doc = "desc CH6MUX."]
        #[must_use]
        #[inline(always)]
        pub const fn ch6mux(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x1f;
            val as u8
        }
        #[doc = "desc CH6MUX."]
        #[inline(always)]
        pub const fn set_ch6mux(&mut self, val: u8) {
            self.0 = (self.0 & !(0x1f << 0usize)) | (((val as u32) & 0x1f) << 0usize);
        }
        #[doc = "desc CH7MUX."]
        #[must_use]
        #[inline(always)]
        pub const fn ch7mux(&self) -> u8 {
            let val = (self.0 >> 5usize) & 0x1f;
            val as u8
        }
        #[doc = "desc CH7MUX."]
        #[inline(always)]
        pub const fn set_ch7mux(&mut self, val: u8) {
            self.0 = (self.0 & !(0x1f << 5usize)) | (((val as u32) & 0x1f) << 5usize);
        }
        #[doc = "desc CH8MUX."]
        #[must_use]
        #[inline(always)]
        pub const fn ch8mux(&self) -> u8 {
            let val = (self.0 >> 10usize) & 0x1f;
            val as u8
        }
        #[doc = "desc CH8MUX."]
        #[inline(always)]
        pub const fn set_ch8mux(&mut self, val: u8) {
            self.0 = (self.0 & !(0x1f << 10usize)) | (((val as u32) & 0x1f) << 10usize);
        }
        #[doc = "desc CH9MUX."]
        #[must_use]
        #[inline(always)]
        pub const fn ch9mux(&self) -> u8 {
            let val = (self.0 >> 15usize) & 0x1f;
            val as u8
        }
        #[doc = "desc CH9MUX."]
        #[inline(always)]
        pub const fn set_ch9mux(&mut self, val: u8) {
            self.0 = (self.0 & !(0x1f << 15usize)) | (((val as u32) & 0x1f) << 15usize);
        }
        #[doc = "desc CH10MUX."]
        #[must_use]
        #[inline(always)]
        pub const fn ch10mux(&self) -> u8 {
            let val = (self.0 >> 20usize) & 0x1f;
            val as u8
        }
        #[doc = "desc CH10MUX."]
        #[inline(always)]
        pub const fn set_ch10mux(&mut self, val: u8) {
            self.0 = (self.0 & !(0x1f << 20usize)) | (((val as u32) & 0x1f) << 20usize);
        }
        #[doc = "desc CH11MUX."]
        #[must_use]
        #[inline(always)]
        pub const fn ch11mux(&self) -> u8 {
            let val = (self.0 >> 25usize) & 0x1f;
            val as u8
        }
        #[doc = "desc CH11MUX."]
        #[inline(always)]
        pub const fn set_ch11mux(&mut self, val: u8) {
            self.0 = (self.0 & !(0x1f << 25usize)) | (((val as u32) & 0x1f) << 25usize);
        }
        #[doc = "desc RSV."]
        #[must_use]
        #[inline(always)]
        pub const fn rsv(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[doc = "desc RSV."]
        #[inline(always)]
        pub const fn set_rsv(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for Sqr1 {
        #[inline(always)]
        fn default() -> Sqr1 {
            Sqr1(0)
        }
    }
    impl core::fmt::Debug for Sqr1 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Sqr1")
                .field("ch6mux", &self.ch6mux())
                .field("ch7mux", &self.ch7mux())
                .field("ch8mux", &self.ch8mux())
                .field("ch9mux", &self.ch9mux())
                .field("ch10mux", &self.ch10mux())
                .field("ch11mux", &self.ch11mux())
                .field("rsv", &self.rsv())
                .finish()
        }
    }
    #[doc = "desc SQR2."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Sqr2(pub u32);
    impl Sqr2 {
        #[doc = "desc CH12MUX."]
        #[must_use]
        #[inline(always)]
        pub const fn ch12mux(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x1f;
            val as u8
        }
        #[doc = "desc CH12MUX."]
        #[inline(always)]
        pub const fn set_ch12mux(&mut self, val: u8) {
            self.0 = (self.0 & !(0x1f << 0usize)) | (((val as u32) & 0x1f) << 0usize);
        }
        #[doc = "desc CH13MUX."]
        #[must_use]
        #[inline(always)]
        pub const fn ch13mux(&self) -> u8 {
            let val = (self.0 >> 5usize) & 0x1f;
            val as u8
        }
        #[doc = "desc CH13MUX."]
        #[inline(always)]
        pub const fn set_ch13mux(&mut self, val: u8) {
            self.0 = (self.0 & !(0x1f << 5usize)) | (((val as u32) & 0x1f) << 5usize);
        }
        #[doc = "desc CH14MUX."]
        #[must_use]
        #[inline(always)]
        pub const fn ch14mux(&self) -> u8 {
            let val = (self.0 >> 10usize) & 0x1f;
            val as u8
        }
        #[doc = "desc CH14MUX."]
        #[inline(always)]
        pub const fn set_ch14mux(&mut self, val: u8) {
            self.0 = (self.0 & !(0x1f << 10usize)) | (((val as u32) & 0x1f) << 10usize);
        }
        #[doc = "desc CH15MUX."]
        #[must_use]
        #[inline(always)]
        pub const fn ch15mux(&self) -> u8 {
            let val = (self.0 >> 15usize) & 0x1f;
            val as u8
        }
        #[doc = "desc CH15MUX."]
        #[inline(always)]
        pub const fn set_ch15mux(&mut self, val: u8) {
            self.0 = (self.0 & !(0x1f << 15usize)) | (((val as u32) & 0x1f) << 15usize);
        }
        #[doc = "desc CNT."]
        #[must_use]
        #[inline(always)]
        pub const fn cnt(&self) -> u8 {
            let val = (self.0 >> 20usize) & 0x0f;
            val as u8
        }
        #[doc = "desc CNT."]
        #[inline(always)]
        pub const fn set_cnt(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 20usize)) | (((val as u32) & 0x0f) << 20usize);
        }
        #[doc = "desc RSV."]
        #[must_use]
        #[inline(always)]
        pub const fn rsv(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[doc = "desc RSV."]
        #[inline(always)]
        pub const fn set_rsv(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for Sqr2 {
        #[inline(always)]
        fn default() -> Sqr2 {
            Sqr2(0)
        }
    }
    impl core::fmt::Debug for Sqr2 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Sqr2")
                .field("ch12mux", &self.ch12mux())
                .field("ch13mux", &self.ch13mux())
                .field("ch14mux", &self.ch14mux())
                .field("ch15mux", &self.ch15mux())
                .field("cnt", &self.cnt())
                .field("rsv", &self.rsv())
                .finish()
        }
    }
    #[doc = "desc SQRRESULT0."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Sqrresult0(pub u32);
    impl Sqrresult0 {
        #[doc = "desc RESULT."]
        #[must_use]
        #[inline(always)]
        pub const fn result(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x0fff;
            val as u16
        }
        #[doc = "desc RESULT."]
        #[inline(always)]
        pub const fn set_result(&mut self, val: u16) {
            self.0 = (self.0 & !(0x0fff << 0usize)) | (((val as u32) & 0x0fff) << 0usize);
        }
        #[doc = "desc RSV."]
        #[must_use]
        #[inline(always)]
        pub const fn rsv(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[doc = "desc RSV."]
        #[inline(always)]
        pub const fn set_rsv(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for Sqrresult0 {
        #[inline(always)]
        fn default() -> Sqrresult0 {
            Sqrresult0(0)
        }
    }
    impl core::fmt::Debug for Sqrresult0 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Sqrresult0")
                .field("result", &self.result())
                .field("rsv", &self.rsv())
                .finish()
        }
    }
    #[doc = "desc SQRRESULT1."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Sqrresult1(pub u32);
    impl Sqrresult1 {
        #[doc = "desc RESULT."]
        #[must_use]
        #[inline(always)]
        pub const fn result(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x0fff;
            val as u16
        }
        #[doc = "desc RESULT."]
        #[inline(always)]
        pub const fn set_result(&mut self, val: u16) {
            self.0 = (self.0 & !(0x0fff << 0usize)) | (((val as u32) & 0x0fff) << 0usize);
        }
        #[doc = "desc RSV."]
        #[must_use]
        #[inline(always)]
        pub const fn rsv(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[doc = "desc RSV."]
        #[inline(always)]
        pub const fn set_rsv(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for Sqrresult1 {
        #[inline(always)]
        fn default() -> Sqrresult1 {
            Sqrresult1(0)
        }
    }
    impl core::fmt::Debug for Sqrresult1 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Sqrresult1")
                .field("result", &self.result())
                .field("rsv", &self.rsv())
                .finish()
        }
    }
    #[doc = "desc SQRRESULT10."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Sqrresult10(pub u32);
    impl Sqrresult10 {
        #[doc = "desc RESULT."]
        #[must_use]
        #[inline(always)]
        pub const fn result(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x0fff;
            val as u16
        }
        #[doc = "desc RESULT."]
        #[inline(always)]
        pub const fn set_result(&mut self, val: u16) {
            self.0 = (self.0 & !(0x0fff << 0usize)) | (((val as u32) & 0x0fff) << 0usize);
        }
        #[doc = "desc RSV."]
        #[must_use]
        #[inline(always)]
        pub const fn rsv(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[doc = "desc RSV."]
        #[inline(always)]
        pub const fn set_rsv(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for Sqrresult10 {
        #[inline(always)]
        fn default() -> Sqrresult10 {
            Sqrresult10(0)
        }
    }
    impl core::fmt::Debug for Sqrresult10 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Sqrresult10")
                .field("result", &self.result())
                .field("rsv", &self.rsv())
                .finish()
        }
    }
    #[doc = "desc SQRRESULT11."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Sqrresult11(pub u32);
    impl Sqrresult11 {
        #[doc = "desc RESULT."]
        #[must_use]
        #[inline(always)]
        pub const fn result(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x0fff;
            val as u16
        }
        #[doc = "desc RESULT."]
        #[inline(always)]
        pub const fn set_result(&mut self, val: u16) {
            self.0 = (self.0 & !(0x0fff << 0usize)) | (((val as u32) & 0x0fff) << 0usize);
        }
        #[doc = "desc RSV."]
        #[must_use]
        #[inline(always)]
        pub const fn rsv(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[doc = "desc RSV."]
        #[inline(always)]
        pub const fn set_rsv(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for Sqrresult11 {
        #[inline(always)]
        fn default() -> Sqrresult11 {
            Sqrresult11(0)
        }
    }
    impl core::fmt::Debug for Sqrresult11 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Sqrresult11")
                .field("result", &self.result())
                .field("rsv", &self.rsv())
                .finish()
        }
    }
    #[doc = "desc SQRRESULT12."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Sqrresult12(pub u32);
    impl Sqrresult12 {
        #[doc = "desc RESULT."]
        #[must_use]
        #[inline(always)]
        pub const fn result(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x0fff;
            val as u16
        }
        #[doc = "desc RESULT."]
        #[inline(always)]
        pub const fn set_result(&mut self, val: u16) {
            self.0 = (self.0 & !(0x0fff << 0usize)) | (((val as u32) & 0x0fff) << 0usize);
        }
        #[doc = "desc RSV."]
        #[must_use]
        #[inline(always)]
        pub const fn rsv(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[doc = "desc RSV."]
        #[inline(always)]
        pub const fn set_rsv(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for Sqrresult12 {
        #[inline(always)]
        fn default() -> Sqrresult12 {
            Sqrresult12(0)
        }
    }
    impl core::fmt::Debug for Sqrresult12 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Sqrresult12")
                .field("result", &self.result())
                .field("rsv", &self.rsv())
                .finish()
        }
    }
    #[doc = "desc SQRRESULT13."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Sqrresult13(pub u32);
    impl Sqrresult13 {
        #[doc = "desc RESULT."]
        #[must_use]
        #[inline(always)]
        pub const fn result(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x0fff;
            val as u16
        }
        #[doc = "desc RESULT."]
        #[inline(always)]
        pub const fn set_result(&mut self, val: u16) {
            self.0 = (self.0 & !(0x0fff << 0usize)) | (((val as u32) & 0x0fff) << 0usize);
        }
        #[doc = "desc RSV."]
        #[must_use]
        #[inline(always)]
        pub const fn rsv(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[doc = "desc RSV."]
        #[inline(always)]
        pub const fn set_rsv(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for Sqrresult13 {
        #[inline(always)]
        fn default() -> Sqrresult13 {
            Sqrresult13(0)
        }
    }
    impl core::fmt::Debug for Sqrresult13 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Sqrresult13")
                .field("result", &self.result())
                .field("rsv", &self.rsv())
                .finish()
        }
    }
    #[doc = "desc SQRRESULT14."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Sqrresult14(pub u32);
    impl Sqrresult14 {
        #[doc = "desc RESULT."]
        #[must_use]
        #[inline(always)]
        pub const fn result(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x0fff;
            val as u16
        }
        #[doc = "desc RESULT."]
        #[inline(always)]
        pub const fn set_result(&mut self, val: u16) {
            self.0 = (self.0 & !(0x0fff << 0usize)) | (((val as u32) & 0x0fff) << 0usize);
        }
        #[doc = "desc RSV."]
        #[must_use]
        #[inline(always)]
        pub const fn rsv(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[doc = "desc RSV."]
        #[inline(always)]
        pub const fn set_rsv(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for Sqrresult14 {
        #[inline(always)]
        fn default() -> Sqrresult14 {
            Sqrresult14(0)
        }
    }
    impl core::fmt::Debug for Sqrresult14 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Sqrresult14")
                .field("result", &self.result())
                .field("rsv", &self.rsv())
                .finish()
        }
    }
    #[doc = "desc SQRRESULT15."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Sqrresult15(pub u32);
    impl Sqrresult15 {
        #[doc = "desc RESULT."]
        #[must_use]
        #[inline(always)]
        pub const fn result(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x0fff;
            val as u16
        }
        #[doc = "desc RESULT."]
        #[inline(always)]
        pub const fn set_result(&mut self, val: u16) {
            self.0 = (self.0 & !(0x0fff << 0usize)) | (((val as u32) & 0x0fff) << 0usize);
        }
        #[doc = "desc RSV."]
        #[must_use]
        #[inline(always)]
        pub const fn rsv(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[doc = "desc RSV."]
        #[inline(always)]
        pub const fn set_rsv(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for Sqrresult15 {
        #[inline(always)]
        fn default() -> Sqrresult15 {
            Sqrresult15(0)
        }
    }
    impl core::fmt::Debug for Sqrresult15 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Sqrresult15")
                .field("result", &self.result())
                .field("rsv", &self.rsv())
                .finish()
        }
    }
    #[doc = "desc SQRRESULT2."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Sqrresult2(pub u32);
    impl Sqrresult2 {
        #[doc = "desc RESULT."]
        #[must_use]
        #[inline(always)]
        pub const fn result(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x0fff;
            val as u16
        }
        #[doc = "desc RESULT."]
        #[inline(always)]
        pub const fn set_result(&mut self, val: u16) {
            self.0 = (self.0 & !(0x0fff << 0usize)) | (((val as u32) & 0x0fff) << 0usize);
        }
        #[doc = "desc RSV."]
        #[must_use]
        #[inline(always)]
        pub const fn rsv(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[doc = "desc RSV."]
        #[inline(always)]
        pub const fn set_rsv(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for Sqrresult2 {
        #[inline(always)]
        fn default() -> Sqrresult2 {
            Sqrresult2(0)
        }
    }
    impl core::fmt::Debug for Sqrresult2 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Sqrresult2")
                .field("result", &self.result())
                .field("rsv", &self.rsv())
                .finish()
        }
    }
    #[doc = "desc SQRRESULT3."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Sqrresult3(pub u32);
    impl Sqrresult3 {
        #[doc = "desc RESULT."]
        #[must_use]
        #[inline(always)]
        pub const fn result(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x0fff;
            val as u16
        }
        #[doc = "desc RESULT."]
        #[inline(always)]
        pub const fn set_result(&mut self, val: u16) {
            self.0 = (self.0 & !(0x0fff << 0usize)) | (((val as u32) & 0x0fff) << 0usize);
        }
        #[doc = "desc RSV."]
        #[must_use]
        #[inline(always)]
        pub const fn rsv(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[doc = "desc RSV."]
        #[inline(always)]
        pub const fn set_rsv(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for Sqrresult3 {
        #[inline(always)]
        fn default() -> Sqrresult3 {
            Sqrresult3(0)
        }
    }
    impl core::fmt::Debug for Sqrresult3 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Sqrresult3")
                .field("result", &self.result())
                .field("rsv", &self.rsv())
                .finish()
        }
    }
    #[doc = "desc SQRRESULT4."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Sqrresult4(pub u32);
    impl Sqrresult4 {
        #[doc = "desc RESULT."]
        #[must_use]
        #[inline(always)]
        pub const fn result(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x0fff;
            val as u16
        }
        #[doc = "desc RESULT."]
        #[inline(always)]
        pub const fn set_result(&mut self, val: u16) {
            self.0 = (self.0 & !(0x0fff << 0usize)) | (((val as u32) & 0x0fff) << 0usize);
        }
        #[doc = "desc RSV."]
        #[must_use]
        #[inline(always)]
        pub const fn rsv(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[doc = "desc RSV."]
        #[inline(always)]
        pub const fn set_rsv(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for Sqrresult4 {
        #[inline(always)]
        fn default() -> Sqrresult4 {
            Sqrresult4(0)
        }
    }
    impl core::fmt::Debug for Sqrresult4 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Sqrresult4")
                .field("result", &self.result())
                .field("rsv", &self.rsv())
                .finish()
        }
    }
    #[doc = "desc SQRRESULT5."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Sqrresult5(pub u32);
    impl Sqrresult5 {
        #[doc = "desc RESULT."]
        #[must_use]
        #[inline(always)]
        pub const fn result(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x0fff;
            val as u16
        }
        #[doc = "desc RESULT."]
        #[inline(always)]
        pub const fn set_result(&mut self, val: u16) {
            self.0 = (self.0 & !(0x0fff << 0usize)) | (((val as u32) & 0x0fff) << 0usize);
        }
        #[doc = "desc RSV."]
        #[must_use]
        #[inline(always)]
        pub const fn rsv(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[doc = "desc RSV."]
        #[inline(always)]
        pub const fn set_rsv(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for Sqrresult5 {
        #[inline(always)]
        fn default() -> Sqrresult5 {
            Sqrresult5(0)
        }
    }
    impl core::fmt::Debug for Sqrresult5 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Sqrresult5")
                .field("result", &self.result())
                .field("rsv", &self.rsv())
                .finish()
        }
    }
    #[doc = "desc SQRRESULT6."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Sqrresult6(pub u32);
    impl Sqrresult6 {
        #[doc = "desc RESULT."]
        #[must_use]
        #[inline(always)]
        pub const fn result(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x0fff;
            val as u16
        }
        #[doc = "desc RESULT."]
        #[inline(always)]
        pub const fn set_result(&mut self, val: u16) {
            self.0 = (self.0 & !(0x0fff << 0usize)) | (((val as u32) & 0x0fff) << 0usize);
        }
        #[doc = "desc RSV."]
        #[must_use]
        #[inline(always)]
        pub const fn rsv(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[doc = "desc RSV."]
        #[inline(always)]
        pub const fn set_rsv(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for Sqrresult6 {
        #[inline(always)]
        fn default() -> Sqrresult6 {
            Sqrresult6(0)
        }
    }
    impl core::fmt::Debug for Sqrresult6 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Sqrresult6")
                .field("result", &self.result())
                .field("rsv", &self.rsv())
                .finish()
        }
    }
    #[doc = "desc SQRRESULT7."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Sqrresult7(pub u32);
    impl Sqrresult7 {
        #[doc = "desc RESULT."]
        #[must_use]
        #[inline(always)]
        pub const fn result(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x0fff;
            val as u16
        }
        #[doc = "desc RESULT."]
        #[inline(always)]
        pub const fn set_result(&mut self, val: u16) {
            self.0 = (self.0 & !(0x0fff << 0usize)) | (((val as u32) & 0x0fff) << 0usize);
        }
        #[doc = "desc RSV."]
        #[must_use]
        #[inline(always)]
        pub const fn rsv(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[doc = "desc RSV."]
        #[inline(always)]
        pub const fn set_rsv(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for Sqrresult7 {
        #[inline(always)]
        fn default() -> Sqrresult7 {
            Sqrresult7(0)
        }
    }
    impl core::fmt::Debug for Sqrresult7 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Sqrresult7")
                .field("result", &self.result())
                .field("rsv", &self.rsv())
                .finish()
        }
    }
    #[doc = "desc SQRRESULT8."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Sqrresult8(pub u32);
    impl Sqrresult8 {
        #[doc = "desc RESULT."]
        #[must_use]
        #[inline(always)]
        pub const fn result(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x0fff;
            val as u16
        }
        #[doc = "desc RESULT."]
        #[inline(always)]
        pub const fn set_result(&mut self, val: u16) {
            self.0 = (self.0 & !(0x0fff << 0usize)) | (((val as u32) & 0x0fff) << 0usize);
        }
        #[doc = "desc RSV."]
        #[must_use]
        #[inline(always)]
        pub const fn rsv(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[doc = "desc RSV."]
        #[inline(always)]
        pub const fn set_rsv(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for Sqrresult8 {
        #[inline(always)]
        fn default() -> Sqrresult8 {
            Sqrresult8(0)
        }
    }
    impl core::fmt::Debug for Sqrresult8 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Sqrresult8")
                .field("result", &self.result())
                .field("rsv", &self.rsv())
                .finish()
        }
    }
    #[doc = "desc SQRRESULT9."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Sqrresult9(pub u32);
    impl Sqrresult9 {
        #[doc = "desc RESULT."]
        #[must_use]
        #[inline(always)]
        pub const fn result(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x0fff;
            val as u16
        }
        #[doc = "desc RESULT."]
        #[inline(always)]
        pub const fn set_result(&mut self, val: u16) {
            self.0 = (self.0 & !(0x0fff << 0usize)) | (((val as u32) & 0x0fff) << 0usize);
        }
        #[doc = "desc RSV."]
        #[must_use]
        #[inline(always)]
        pub const fn rsv(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[doc = "desc RSV."]
        #[inline(always)]
        pub const fn set_rsv(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for Sqrresult9 {
        #[inline(always)]
        fn default() -> Sqrresult9 {
            Sqrresult9(0)
        }
    }
    impl core::fmt::Debug for Sqrresult9 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Sqrresult9")
                .field("result", &self.result())
                .field("rsv", &self.rsv())
                .finish()
        }
    }
    #[doc = "desc SQRSTART."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Sqrstart(pub u32);
    impl Sqrstart {
        #[doc = "desc START."]
        #[must_use]
        #[inline(always)]
        pub const fn start(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "desc START."]
        #[inline(always)]
        pub const fn set_start(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "desc RSV."]
        #[must_use]
        #[inline(always)]
        pub const fn rsv(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[doc = "desc RSV."]
        #[inline(always)]
        pub const fn set_rsv(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for Sqrstart {
        #[inline(always)]
        fn default() -> Sqrstart {
            Sqrstart(0)
        }
    }
    impl core::fmt::Debug for Sqrstart {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Sqrstart")
                .field("start", &self.start())
                .field("rsv", &self.rsv())
                .finish()
        }
    }
}
