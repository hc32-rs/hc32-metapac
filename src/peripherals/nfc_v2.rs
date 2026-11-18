#![allow(clippy::missing_safety_doc)]
#![allow(clippy::identity_op)]
#![allow(clippy::unnecessary_cast)]
#![allow(clippy::erasing_op)]

#[doc = "desc NFC."]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Nfc {
    ptr: *mut u8,
}
unsafe impl Send for Nfc {}
unsafe impl Sync for Nfc {}
impl Nfc {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "desc DATR_BASE."]
    #[inline(always)]
    pub const fn datr_base(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0usize) as _) }
    }
    #[doc = "desc CMDR."]
    #[inline(always)]
    pub const fn cmdr(self) -> crate::common::Reg<regs::Cmdr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x8000usize) as _) }
    }
    #[doc = "desc IDXR0."]
    #[inline(always)]
    pub const fn idxr0(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x8004usize) as _) }
    }
    #[doc = "desc IENR."]
    #[inline(always)]
    pub const fn ienr(self) -> crate::common::Reg<regs::Ienr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x8030usize) as _) }
    }
    #[doc = "desc ISTR."]
    #[inline(always)]
    pub const fn istr(self) -> crate::common::Reg<regs::Istr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x8034usize) as _) }
    }
    #[doc = "desc IRSR."]
    #[inline(always)]
    pub const fn irsr(self) -> crate::common::Reg<regs::Irsr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x8038usize) as _) }
    }
    #[doc = "desc IDXR1."]
    #[inline(always)]
    pub const fn idxr1(self) -> crate::common::Reg<regs::Idxr1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x8048usize) as _) }
    }
    #[doc = "desc TMCR0."]
    #[inline(always)]
    pub const fn tmcr0(self) -> crate::common::Reg<regs::Tmcr0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x804cusize) as _) }
    }
    #[doc = "desc TMCR1."]
    #[inline(always)]
    pub const fn tmcr1(self) -> crate::common::Reg<regs::Tmcr1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x8050usize) as _) }
    }
    #[doc = "desc BACR."]
    #[inline(always)]
    pub const fn bacr(self) -> crate::common::Reg<regs::Bacr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x8054usize) as _) }
    }
    #[doc = "desc TMCR2."]
    #[inline(always)]
    pub const fn tmcr2(self) -> crate::common::Reg<regs::Tmcr2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x805cusize) as _) }
    }
    #[doc = "desc ECCR0."]
    #[inline(always)]
    pub const fn eccr0(self) -> crate::common::Reg<regs::Eccr0, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x8060usize) as _) }
    }
    #[doc = "desc ECCR1."]
    #[inline(always)]
    pub const fn eccr1(self) -> crate::common::Reg<regs::Eccr1, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x8064usize) as _) }
    }
    #[doc = "desc ECCR2."]
    #[inline(always)]
    pub const fn eccr2(self) -> crate::common::Reg<regs::Eccr2, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x8068usize) as _) }
    }
    #[doc = "desc ECCR3."]
    #[inline(always)]
    pub const fn eccr3(self) -> crate::common::Reg<regs::Eccr3, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x806cusize) as _) }
    }
    #[doc = "desc ECCR4."]
    #[inline(always)]
    pub const fn eccr4(self) -> crate::common::Reg<regs::Eccr4, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x8070usize) as _) }
    }
    #[doc = "desc ECCR5."]
    #[inline(always)]
    pub const fn eccr5(self) -> crate::common::Reg<regs::Eccr5, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x8074usize) as _) }
    }
    #[doc = "desc ECCR6."]
    #[inline(always)]
    pub const fn eccr6(self) -> crate::common::Reg<regs::Eccr6, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x8078usize) as _) }
    }
    #[doc = "desc ECC_STAT."]
    #[inline(always)]
    pub const fn ecc_stat(self) -> crate::common::Reg<regs::EccStat, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x807cusize) as _) }
    }
    #[doc = "desc ECCR7."]
    #[inline(always)]
    pub const fn eccr7(self) -> crate::common::Reg<regs::Eccr7, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x807cusize) as _) }
    }
    #[doc = "desc ECC_SECTION0_SYND12."]
    #[inline(always)]
    pub const fn ecc_section0_synd12(
        self,
    ) -> crate::common::Reg<regs::EccSection0Synd12, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x8080usize) as _) }
    }
    #[doc = "desc ECCR8."]
    #[inline(always)]
    pub const fn eccr8(self) -> crate::common::Reg<regs::Eccr8, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x8080usize) as _) }
    }
    #[doc = "desc ECC_SECTION0_SYND34."]
    #[inline(always)]
    pub const fn ecc_section0_synd34(
        self,
    ) -> crate::common::Reg<regs::EccSection0Synd34, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x8084usize) as _) }
    }
    #[doc = "desc ECCR9."]
    #[inline(always)]
    pub const fn eccr9(self) -> crate::common::Reg<regs::Eccr9, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x8084usize) as _) }
    }
    #[doc = "desc ECC_SECTION0_SYND56."]
    #[inline(always)]
    pub const fn ecc_section0_synd56(
        self,
    ) -> crate::common::Reg<regs::EccSection0Synd56, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x8088usize) as _) }
    }
    #[doc = "desc ECCR10."]
    #[inline(always)]
    pub const fn eccr10(self) -> crate::common::Reg<regs::Eccr10, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x8088usize) as _) }
    }
    #[doc = "desc ECC_SECTION0_SYND78."]
    #[inline(always)]
    pub const fn ecc_section0_synd78(
        self,
    ) -> crate::common::Reg<regs::EccSection0Synd78, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x808cusize) as _) }
    }
    #[doc = "desc ECCR11."]
    #[inline(always)]
    pub const fn eccr11(self) -> crate::common::Reg<regs::Eccr11, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x808cusize) as _) }
    }
    #[doc = "desc ECC_SECTION1_SYND12."]
    #[inline(always)]
    pub const fn ecc_section1_synd12(
        self,
    ) -> crate::common::Reg<regs::EccSection1Synd12, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x8090usize) as _) }
    }
    #[doc = "desc ECCR12."]
    #[inline(always)]
    pub const fn eccr12(self) -> crate::common::Reg<regs::Eccr12, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x8090usize) as _) }
    }
    #[doc = "desc ECC_SECTION1_SYND34."]
    #[inline(always)]
    pub const fn ecc_section1_synd34(
        self,
    ) -> crate::common::Reg<regs::EccSection1Synd34, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x8094usize) as _) }
    }
    #[doc = "desc ECCR13."]
    #[inline(always)]
    pub const fn eccr13(self) -> crate::common::Reg<regs::Eccr13, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x8094usize) as _) }
    }
    #[doc = "desc ECC_SECTION1_SYND56."]
    #[inline(always)]
    pub const fn ecc_section1_synd56(
        self,
    ) -> crate::common::Reg<regs::EccSection1Synd56, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x8098usize) as _) }
    }
    #[doc = "desc ECCR14."]
    #[inline(always)]
    pub const fn eccr14(self) -> crate::common::Reg<regs::Eccr14, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x8098usize) as _) }
    }
    #[doc = "desc ECC_SECTION1_SYND78."]
    #[inline(always)]
    pub const fn ecc_section1_synd78(
        self,
    ) -> crate::common::Reg<regs::EccSection1Synd78, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x809cusize) as _) }
    }
    #[doc = "desc ECCR15."]
    #[inline(always)]
    pub const fn eccr15(self) -> crate::common::Reg<regs::Eccr15, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x809cusize) as _) }
    }
    #[doc = "desc ECC_SECTION2_SYND12."]
    #[inline(always)]
    pub const fn ecc_section2_synd12(
        self,
    ) -> crate::common::Reg<regs::EccSection2Synd12, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x80a0usize) as _) }
    }
    #[doc = "desc ECC_SECTION2_SYND34."]
    #[inline(always)]
    pub const fn ecc_section2_synd34(
        self,
    ) -> crate::common::Reg<regs::EccSection2Synd34, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x80a4usize) as _) }
    }
    #[doc = "desc ECC_SECTION2_SYND56."]
    #[inline(always)]
    pub const fn ecc_section2_synd56(
        self,
    ) -> crate::common::Reg<regs::EccSection2Synd56, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x80a8usize) as _) }
    }
    #[doc = "desc ECC_SECTION2_SYND78."]
    #[inline(always)]
    pub const fn ecc_section2_synd78(
        self,
    ) -> crate::common::Reg<regs::EccSection2Synd78, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x80acusize) as _) }
    }
    #[doc = "desc ECC_SECTION3_SYND12."]
    #[inline(always)]
    pub const fn ecc_section3_synd12(
        self,
    ) -> crate::common::Reg<regs::EccSection3Synd12, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x80b0usize) as _) }
    }
    #[doc = "desc ECC_SECTION3_SYND34."]
    #[inline(always)]
    pub const fn ecc_section3_synd34(
        self,
    ) -> crate::common::Reg<regs::EccSection3Synd34, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x80b4usize) as _) }
    }
    #[doc = "desc ECC_SECTION3_SYND56."]
    #[inline(always)]
    pub const fn ecc_section3_synd56(
        self,
    ) -> crate::common::Reg<regs::EccSection3Synd56, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x80b8usize) as _) }
    }
    #[doc = "desc ECC_SECTION3_SYND78."]
    #[inline(always)]
    pub const fn ecc_section3_synd78(
        self,
    ) -> crate::common::Reg<regs::EccSection3Synd78, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x80bcusize) as _) }
    }
    #[doc = "desc ECC_SECTION4_SYND12."]
    #[inline(always)]
    pub const fn ecc_section4_synd12(
        self,
    ) -> crate::common::Reg<regs::EccSection4Synd12, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x80c0usize) as _) }
    }
    #[doc = "desc ECC_SECTION4_SYND34."]
    #[inline(always)]
    pub const fn ecc_section4_synd34(
        self,
    ) -> crate::common::Reg<regs::EccSection4Synd34, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x80c4usize) as _) }
    }
    #[doc = "desc ECC_SECTION4_SYND56."]
    #[inline(always)]
    pub const fn ecc_section4_synd56(
        self,
    ) -> crate::common::Reg<regs::EccSection4Synd56, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x80c8usize) as _) }
    }
    #[doc = "desc ECC_SECTION4_SYND78."]
    #[inline(always)]
    pub const fn ecc_section4_synd78(
        self,
    ) -> crate::common::Reg<regs::EccSection4Synd78, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x80ccusize) as _) }
    }
    #[doc = "desc ECC_SECTION5_SYND12."]
    #[inline(always)]
    pub const fn ecc_section5_synd12(
        self,
    ) -> crate::common::Reg<regs::EccSection5Synd12, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x80d0usize) as _) }
    }
    #[doc = "desc ECC_SECTION5_SYND34."]
    #[inline(always)]
    pub const fn ecc_section5_synd34(
        self,
    ) -> crate::common::Reg<regs::EccSection5Synd34, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x80d4usize) as _) }
    }
    #[doc = "desc ECC_SECTION5_SYND56."]
    #[inline(always)]
    pub const fn ecc_section5_synd56(
        self,
    ) -> crate::common::Reg<regs::EccSection5Synd56, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x80d8usize) as _) }
    }
    #[doc = "desc ECC_SECTION5_SYND78."]
    #[inline(always)]
    pub const fn ecc_section5_synd78(
        self,
    ) -> crate::common::Reg<regs::EccSection5Synd78, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x80dcusize) as _) }
    }
    #[doc = "desc ECC_SECTION6_SYND12."]
    #[inline(always)]
    pub const fn ecc_section6_synd12(
        self,
    ) -> crate::common::Reg<regs::EccSection6Synd12, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x80e0usize) as _) }
    }
    #[doc = "desc ECC_SECTION6_SYND34."]
    #[inline(always)]
    pub const fn ecc_section6_synd34(
        self,
    ) -> crate::common::Reg<regs::EccSection6Synd34, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x80e4usize) as _) }
    }
    #[doc = "desc ECC_SECTION6_SYND56."]
    #[inline(always)]
    pub const fn ecc_section6_synd56(
        self,
    ) -> crate::common::Reg<regs::EccSection6Synd56, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x80e8usize) as _) }
    }
    #[doc = "desc ECC_SECTION6_SYND78."]
    #[inline(always)]
    pub const fn ecc_section6_synd78(
        self,
    ) -> crate::common::Reg<regs::EccSection6Synd78, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x80ecusize) as _) }
    }
    #[doc = "desc ECC_SECTION7_SYND12."]
    #[inline(always)]
    pub const fn ecc_section7_synd12(
        self,
    ) -> crate::common::Reg<regs::EccSection7Synd12, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x80f0usize) as _) }
    }
    #[doc = "desc ECC_SECTION7_SYND34."]
    #[inline(always)]
    pub const fn ecc_section7_synd34(
        self,
    ) -> crate::common::Reg<regs::EccSection7Synd34, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x80f4usize) as _) }
    }
    #[doc = "desc ECC_SECTION7_SYND56."]
    #[inline(always)]
    pub const fn ecc_section7_synd56(
        self,
    ) -> crate::common::Reg<regs::EccSection7Synd56, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x80f8usize) as _) }
    }
    #[doc = "desc ECC_SECTION7_SYND78."]
    #[inline(always)]
    pub const fn ecc_section7_synd78(
        self,
    ) -> crate::common::Reg<regs::EccSection7Synd78, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x80fcusize) as _) }
    }
    #[doc = "desc ECC_SECTION8_SYND12."]
    #[inline(always)]
    pub const fn ecc_section8_synd12(
        self,
    ) -> crate::common::Reg<regs::EccSection8Synd12, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x8100usize) as _) }
    }
    #[doc = "desc ECC_SECTION8_SYND34."]
    #[inline(always)]
    pub const fn ecc_section8_synd34(
        self,
    ) -> crate::common::Reg<regs::EccSection8Synd34, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x8104usize) as _) }
    }
    #[doc = "desc ECC_SECTION8_SYND56."]
    #[inline(always)]
    pub const fn ecc_section8_synd56(
        self,
    ) -> crate::common::Reg<regs::EccSection8Synd56, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x8108usize) as _) }
    }
    #[doc = "desc ECC_SECTION8_SYND78."]
    #[inline(always)]
    pub const fn ecc_section8_synd78(
        self,
    ) -> crate::common::Reg<regs::EccSection8Synd78, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x810cusize) as _) }
    }
    #[doc = "desc ECC_SECTION9_SYND12."]
    #[inline(always)]
    pub const fn ecc_section9_synd12(
        self,
    ) -> crate::common::Reg<regs::EccSection9Synd12, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x8110usize) as _) }
    }
    #[doc = "desc ECC_SECTION9_SYND34."]
    #[inline(always)]
    pub const fn ecc_section9_synd34(
        self,
    ) -> crate::common::Reg<regs::EccSection9Synd34, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x8114usize) as _) }
    }
    #[doc = "desc ECC_SECTION9_SYND56."]
    #[inline(always)]
    pub const fn ecc_section9_synd56(
        self,
    ) -> crate::common::Reg<regs::EccSection9Synd56, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x8118usize) as _) }
    }
    #[doc = "desc ECC_SECTION9_SYND78."]
    #[inline(always)]
    pub const fn ecc_section9_synd78(
        self,
    ) -> crate::common::Reg<regs::EccSection9Synd78, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x811cusize) as _) }
    }
    #[doc = "desc ECC_SECTION10_SYND12."]
    #[inline(always)]
    pub const fn ecc_section10_synd12(
        self,
    ) -> crate::common::Reg<regs::EccSection10Synd12, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x8120usize) as _) }
    }
    #[doc = "desc ECC_SECTION10_SYND34."]
    #[inline(always)]
    pub const fn ecc_section10_synd34(
        self,
    ) -> crate::common::Reg<regs::EccSection10Synd34, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x8124usize) as _) }
    }
    #[doc = "desc ECC_SECTION10_SYND56."]
    #[inline(always)]
    pub const fn ecc_section10_synd56(
        self,
    ) -> crate::common::Reg<regs::EccSection10Synd56, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x8128usize) as _) }
    }
    #[doc = "desc ECC_SECTION10_SYND78."]
    #[inline(always)]
    pub const fn ecc_section10_synd78(
        self,
    ) -> crate::common::Reg<regs::EccSection10Synd78, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x812cusize) as _) }
    }
    #[doc = "desc ECC_SECTION11_SYND12."]
    #[inline(always)]
    pub const fn ecc_section11_synd12(
        self,
    ) -> crate::common::Reg<regs::EccSection11Synd12, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x8130usize) as _) }
    }
    #[doc = "desc ECC_SECTION11_SYND34."]
    #[inline(always)]
    pub const fn ecc_section11_synd34(
        self,
    ) -> crate::common::Reg<regs::EccSection11Synd34, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x8134usize) as _) }
    }
    #[doc = "desc ECC_SECTION11_SYND56."]
    #[inline(always)]
    pub const fn ecc_section11_synd56(
        self,
    ) -> crate::common::Reg<regs::EccSection11Synd56, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x8138usize) as _) }
    }
    #[doc = "desc ECC_SECTION11_SYND78."]
    #[inline(always)]
    pub const fn ecc_section11_synd78(
        self,
    ) -> crate::common::Reg<regs::EccSection11Synd78, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x813cusize) as _) }
    }
    #[doc = "desc ECC_SECTION12_SYND12."]
    #[inline(always)]
    pub const fn ecc_section12_synd12(
        self,
    ) -> crate::common::Reg<regs::EccSection12Synd12, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x8140usize) as _) }
    }
    #[doc = "desc ECC_SECTION12_SYND34."]
    #[inline(always)]
    pub const fn ecc_section12_synd34(
        self,
    ) -> crate::common::Reg<regs::EccSection12Synd34, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x8144usize) as _) }
    }
    #[doc = "desc ECC_SECTION12_SYND56."]
    #[inline(always)]
    pub const fn ecc_section12_synd56(
        self,
    ) -> crate::common::Reg<regs::EccSection12Synd56, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x8148usize) as _) }
    }
    #[doc = "desc ECC_SECTION12_SYND78."]
    #[inline(always)]
    pub const fn ecc_section12_synd78(
        self,
    ) -> crate::common::Reg<regs::EccSection12Synd78, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x814cusize) as _) }
    }
    #[doc = "desc ECC_SECTION13_SYND12."]
    #[inline(always)]
    pub const fn ecc_section13_synd12(
        self,
    ) -> crate::common::Reg<regs::EccSection13Synd12, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x8150usize) as _) }
    }
    #[doc = "desc ECC_SECTION13_SYND34."]
    #[inline(always)]
    pub const fn ecc_section13_synd34(
        self,
    ) -> crate::common::Reg<regs::EccSection13Synd34, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x8154usize) as _) }
    }
    #[doc = "desc ECC_SECTION13_SYND56."]
    #[inline(always)]
    pub const fn ecc_section13_synd56(
        self,
    ) -> crate::common::Reg<regs::EccSection13Synd56, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x8158usize) as _) }
    }
    #[doc = "desc ECC_SECTION13_SYND78."]
    #[inline(always)]
    pub const fn ecc_section13_synd78(
        self,
    ) -> crate::common::Reg<regs::EccSection13Synd78, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x815cusize) as _) }
    }
    #[doc = "desc ECC_SECTION14_SYND12."]
    #[inline(always)]
    pub const fn ecc_section14_synd12(
        self,
    ) -> crate::common::Reg<regs::EccSection14Synd12, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x8160usize) as _) }
    }
    #[doc = "desc ECC_SECTION14_SYND34."]
    #[inline(always)]
    pub const fn ecc_section14_synd34(
        self,
    ) -> crate::common::Reg<regs::EccSection14Synd34, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x8164usize) as _) }
    }
    #[doc = "desc ECC_SECTION14_SYND56."]
    #[inline(always)]
    pub const fn ecc_section14_synd56(
        self,
    ) -> crate::common::Reg<regs::EccSection14Synd56, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x8168usize) as _) }
    }
    #[doc = "desc ECC_SECTION14_SYND78."]
    #[inline(always)]
    pub const fn ecc_section14_synd78(
        self,
    ) -> crate::common::Reg<regs::EccSection14Synd78, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x816cusize) as _) }
    }
    #[doc = "desc ECC_SECTION15_SYND12."]
    #[inline(always)]
    pub const fn ecc_section15_synd12(
        self,
    ) -> crate::common::Reg<regs::EccSection15Synd12, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x8170usize) as _) }
    }
    #[doc = "desc ECC_SECTION15_SYND34."]
    #[inline(always)]
    pub const fn ecc_section15_synd34(
        self,
    ) -> crate::common::Reg<regs::EccSection15Synd34, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x8174usize) as _) }
    }
    #[doc = "desc ECC_SECTION15_SYND56."]
    #[inline(always)]
    pub const fn ecc_section15_synd56(
        self,
    ) -> crate::common::Reg<regs::EccSection15Synd56, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x8178usize) as _) }
    }
    #[doc = "desc ECC_SECTION15_SYND78."]
    #[inline(always)]
    pub const fn ecc_section15_synd78(
        self,
    ) -> crate::common::Reg<regs::EccSection15Synd78, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x817cusize) as _) }
    }
}
pub mod regs {
    #[doc = "desc BACR."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Bacr(pub u32);
    impl Bacr {
        #[doc = "desc SIZE."]
        #[must_use]
        #[inline(always)]
        pub const fn size(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x07;
            val as u8
        }
        #[doc = "desc SIZE."]
        #[inline(always)]
        pub const fn set_size(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 0usize)) | (((val as u32) & 0x07) << 0usize);
        }
        #[doc = "desc B16BIT."]
        #[must_use]
        #[inline(always)]
        pub const fn b16bit(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "desc B16BIT."]
        #[inline(always)]
        pub const fn set_b16bit(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "desc BANK."]
        #[must_use]
        #[inline(always)]
        pub const fn bank(&self) -> u8 {
            let val = (self.0 >> 6usize) & 0x03;
            val as u8
        }
        #[doc = "desc BANK."]
        #[inline(always)]
        pub const fn set_bank(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 6usize)) | (((val as u32) & 0x03) << 6usize);
        }
        #[doc = "desc PAGE."]
        #[must_use]
        #[inline(always)]
        pub const fn page(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0x03;
            val as u8
        }
        #[doc = "desc PAGE."]
        #[inline(always)]
        pub const fn set_page(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 8usize)) | (((val as u32) & 0x03) << 8usize);
        }
        #[doc = "desc WP."]
        #[must_use]
        #[inline(always)]
        pub const fn wp(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "desc WP."]
        #[inline(always)]
        pub const fn set_wp(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[doc = "desc ECCM."]
        #[must_use]
        #[inline(always)]
        pub const fn eccm(&self) -> u8 {
            let val = (self.0 >> 11usize) & 0x03;
            val as u8
        }
        #[doc = "desc ECCM."]
        #[inline(always)]
        pub const fn set_eccm(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 11usize)) | (((val as u32) & 0x03) << 11usize);
        }
        #[doc = "desc RAC."]
        #[must_use]
        #[inline(always)]
        pub const fn rac(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[doc = "desc RAC."]
        #[inline(always)]
        pub const fn set_rac(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[doc = "desc SCS."]
        #[must_use]
        #[inline(always)]
        pub const fn scs(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0xff;
            val as u8
        }
        #[doc = "desc SCS."]
        #[inline(always)]
        pub const fn set_scs(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 16usize)) | (((val as u32) & 0xff) << 16usize);
        }
    }
    impl Default for Bacr {
        #[inline(always)]
        fn default() -> Bacr {
            Bacr(0)
        }
    }
    impl core::fmt::Debug for Bacr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Bacr")
                .field("size", &self.size())
                .field("b16bit", &self.b16bit())
                .field("bank", &self.bank())
                .field("page", &self.page())
                .field("wp", &self.wp())
                .field("eccm", &self.eccm())
                .field("rac", &self.rac())
                .field("scs", &self.scs())
                .finish()
        }
    }
    #[doc = "desc CMDR."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Cmdr(pub u32);
    impl Cmdr {
        #[doc = "desc CMD."]
        #[must_use]
        #[inline(always)]
        pub const fn cmd(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[doc = "desc CMD."]
        #[inline(always)]
        pub const fn set_cmd(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
        }
        #[doc = "desc ARG."]
        #[must_use]
        #[inline(always)]
        pub const fn arg(&self) -> u32 {
            let val = (self.0 >> 8usize) & 0x00ff_ffff;
            val as u32
        }
        #[doc = "desc ARG."]
        #[inline(always)]
        pub const fn set_arg(&mut self, val: u32) {
            self.0 = (self.0 & !(0x00ff_ffff << 8usize)) | (((val as u32) & 0x00ff_ffff) << 8usize);
        }
    }
    impl Default for Cmdr {
        #[inline(always)]
        fn default() -> Cmdr {
            Cmdr(0)
        }
    }
    impl core::fmt::Debug for Cmdr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Cmdr")
                .field("cmd", &self.cmd())
                .field("arg", &self.arg())
                .finish()
        }
    }
    #[doc = "desc ECC_SECTION0_SYND12."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct EccSection0Synd12(pub u32);
    impl EccSection0Synd12 {
        #[doc = "desc S1."]
        #[must_use]
        #[inline(always)]
        pub const fn s1(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x1fff;
            val as u16
        }
        #[doc = "desc S1."]
        #[inline(always)]
        pub const fn set_s1(&mut self, val: u16) {
            self.0 = (self.0 & !(0x1fff << 0usize)) | (((val as u32) & 0x1fff) << 0usize);
        }
        #[doc = "desc S2."]
        #[must_use]
        #[inline(always)]
        pub const fn s2(&self) -> u16 {
            let val = (self.0 >> 16usize) & 0x1fff;
            val as u16
        }
        #[doc = "desc S2."]
        #[inline(always)]
        pub const fn set_s2(&mut self, val: u16) {
            self.0 = (self.0 & !(0x1fff << 16usize)) | (((val as u32) & 0x1fff) << 16usize);
        }
    }
    impl Default for EccSection0Synd12 {
        #[inline(always)]
        fn default() -> EccSection0Synd12 {
            EccSection0Synd12(0)
        }
    }
    impl core::fmt::Debug for EccSection0Synd12 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("EccSection0Synd12")
                .field("s1", &self.s1())
                .field("s2", &self.s2())
                .finish()
        }
    }
    #[doc = "desc ECC_SECTION0_SYND34."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct EccSection0Synd34(pub u32);
    impl EccSection0Synd34 {
        #[doc = "desc S3."]
        #[must_use]
        #[inline(always)]
        pub const fn s3(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x1fff;
            val as u16
        }
        #[doc = "desc S3."]
        #[inline(always)]
        pub const fn set_s3(&mut self, val: u16) {
            self.0 = (self.0 & !(0x1fff << 0usize)) | (((val as u32) & 0x1fff) << 0usize);
        }
        #[doc = "desc S4."]
        #[must_use]
        #[inline(always)]
        pub const fn s4(&self) -> u16 {
            let val = (self.0 >> 16usize) & 0x1fff;
            val as u16
        }
        #[doc = "desc S4."]
        #[inline(always)]
        pub const fn set_s4(&mut self, val: u16) {
            self.0 = (self.0 & !(0x1fff << 16usize)) | (((val as u32) & 0x1fff) << 16usize);
        }
    }
    impl Default for EccSection0Synd34 {
        #[inline(always)]
        fn default() -> EccSection0Synd34 {
            EccSection0Synd34(0)
        }
    }
    impl core::fmt::Debug for EccSection0Synd34 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("EccSection0Synd34")
                .field("s3", &self.s3())
                .field("s4", &self.s4())
                .finish()
        }
    }
    #[doc = "desc ECC_SECTION0_SYND56."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct EccSection0Synd56(pub u32);
    impl EccSection0Synd56 {
        #[doc = "desc S5."]
        #[must_use]
        #[inline(always)]
        pub const fn s5(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x1fff;
            val as u16
        }
        #[doc = "desc S5."]
        #[inline(always)]
        pub const fn set_s5(&mut self, val: u16) {
            self.0 = (self.0 & !(0x1fff << 0usize)) | (((val as u32) & 0x1fff) << 0usize);
        }
        #[doc = "desc S6."]
        #[must_use]
        #[inline(always)]
        pub const fn s6(&self) -> u16 {
            let val = (self.0 >> 16usize) & 0x1fff;
            val as u16
        }
        #[doc = "desc S6."]
        #[inline(always)]
        pub const fn set_s6(&mut self, val: u16) {
            self.0 = (self.0 & !(0x1fff << 16usize)) | (((val as u32) & 0x1fff) << 16usize);
        }
    }
    impl Default for EccSection0Synd56 {
        #[inline(always)]
        fn default() -> EccSection0Synd56 {
            EccSection0Synd56(0)
        }
    }
    impl core::fmt::Debug for EccSection0Synd56 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("EccSection0Synd56")
                .field("s5", &self.s5())
                .field("s6", &self.s6())
                .finish()
        }
    }
    #[doc = "desc ECC_SECTION0_SYND78."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct EccSection0Synd78(pub u32);
    impl EccSection0Synd78 {
        #[doc = "desc S7."]
        #[must_use]
        #[inline(always)]
        pub const fn s7(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x1fff;
            val as u16
        }
        #[doc = "desc S7."]
        #[inline(always)]
        pub const fn set_s7(&mut self, val: u16) {
            self.0 = (self.0 & !(0x1fff << 0usize)) | (((val as u32) & 0x1fff) << 0usize);
        }
        #[doc = "desc S8."]
        #[must_use]
        #[inline(always)]
        pub const fn s8(&self) -> u16 {
            let val = (self.0 >> 16usize) & 0x1fff;
            val as u16
        }
        #[doc = "desc S8."]
        #[inline(always)]
        pub const fn set_s8(&mut self, val: u16) {
            self.0 = (self.0 & !(0x1fff << 16usize)) | (((val as u32) & 0x1fff) << 16usize);
        }
    }
    impl Default for EccSection0Synd78 {
        #[inline(always)]
        fn default() -> EccSection0Synd78 {
            EccSection0Synd78(0)
        }
    }
    impl core::fmt::Debug for EccSection0Synd78 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("EccSection0Synd78")
                .field("s7", &self.s7())
                .field("s8", &self.s8())
                .finish()
        }
    }
    #[doc = "desc ECC_SECTION10_SYND12."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct EccSection10Synd12(pub u32);
    impl EccSection10Synd12 {
        #[doc = "desc S1."]
        #[must_use]
        #[inline(always)]
        pub const fn s1(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x1fff;
            val as u16
        }
        #[doc = "desc S1."]
        #[inline(always)]
        pub const fn set_s1(&mut self, val: u16) {
            self.0 = (self.0 & !(0x1fff << 0usize)) | (((val as u32) & 0x1fff) << 0usize);
        }
        #[doc = "desc S2."]
        #[must_use]
        #[inline(always)]
        pub const fn s2(&self) -> u16 {
            let val = (self.0 >> 16usize) & 0x1fff;
            val as u16
        }
        #[doc = "desc S2."]
        #[inline(always)]
        pub const fn set_s2(&mut self, val: u16) {
            self.0 = (self.0 & !(0x1fff << 16usize)) | (((val as u32) & 0x1fff) << 16usize);
        }
    }
    impl Default for EccSection10Synd12 {
        #[inline(always)]
        fn default() -> EccSection10Synd12 {
            EccSection10Synd12(0)
        }
    }
    impl core::fmt::Debug for EccSection10Synd12 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("EccSection10Synd12")
                .field("s1", &self.s1())
                .field("s2", &self.s2())
                .finish()
        }
    }
    #[doc = "desc ECC_SECTION10_SYND34."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct EccSection10Synd34(pub u32);
    impl EccSection10Synd34 {
        #[doc = "desc S3."]
        #[must_use]
        #[inline(always)]
        pub const fn s3(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x1fff;
            val as u16
        }
        #[doc = "desc S3."]
        #[inline(always)]
        pub const fn set_s3(&mut self, val: u16) {
            self.0 = (self.0 & !(0x1fff << 0usize)) | (((val as u32) & 0x1fff) << 0usize);
        }
        #[doc = "desc S4."]
        #[must_use]
        #[inline(always)]
        pub const fn s4(&self) -> u16 {
            let val = (self.0 >> 16usize) & 0x1fff;
            val as u16
        }
        #[doc = "desc S4."]
        #[inline(always)]
        pub const fn set_s4(&mut self, val: u16) {
            self.0 = (self.0 & !(0x1fff << 16usize)) | (((val as u32) & 0x1fff) << 16usize);
        }
    }
    impl Default for EccSection10Synd34 {
        #[inline(always)]
        fn default() -> EccSection10Synd34 {
            EccSection10Synd34(0)
        }
    }
    impl core::fmt::Debug for EccSection10Synd34 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("EccSection10Synd34")
                .field("s3", &self.s3())
                .field("s4", &self.s4())
                .finish()
        }
    }
    #[doc = "desc ECC_SECTION10_SYND56."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct EccSection10Synd56(pub u32);
    impl EccSection10Synd56 {
        #[doc = "desc S5."]
        #[must_use]
        #[inline(always)]
        pub const fn s5(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x1fff;
            val as u16
        }
        #[doc = "desc S5."]
        #[inline(always)]
        pub const fn set_s5(&mut self, val: u16) {
            self.0 = (self.0 & !(0x1fff << 0usize)) | (((val as u32) & 0x1fff) << 0usize);
        }
        #[doc = "desc S6."]
        #[must_use]
        #[inline(always)]
        pub const fn s6(&self) -> u16 {
            let val = (self.0 >> 16usize) & 0x1fff;
            val as u16
        }
        #[doc = "desc S6."]
        #[inline(always)]
        pub const fn set_s6(&mut self, val: u16) {
            self.0 = (self.0 & !(0x1fff << 16usize)) | (((val as u32) & 0x1fff) << 16usize);
        }
    }
    impl Default for EccSection10Synd56 {
        #[inline(always)]
        fn default() -> EccSection10Synd56 {
            EccSection10Synd56(0)
        }
    }
    impl core::fmt::Debug for EccSection10Synd56 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("EccSection10Synd56")
                .field("s5", &self.s5())
                .field("s6", &self.s6())
                .finish()
        }
    }
    #[doc = "desc ECC_SECTION10_SYND78."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct EccSection10Synd78(pub u32);
    impl EccSection10Synd78 {
        #[doc = "desc S7."]
        #[must_use]
        #[inline(always)]
        pub const fn s7(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x1fff;
            val as u16
        }
        #[doc = "desc S7."]
        #[inline(always)]
        pub const fn set_s7(&mut self, val: u16) {
            self.0 = (self.0 & !(0x1fff << 0usize)) | (((val as u32) & 0x1fff) << 0usize);
        }
        #[doc = "desc S8."]
        #[must_use]
        #[inline(always)]
        pub const fn s8(&self) -> u16 {
            let val = (self.0 >> 16usize) & 0x1fff;
            val as u16
        }
        #[doc = "desc S8."]
        #[inline(always)]
        pub const fn set_s8(&mut self, val: u16) {
            self.0 = (self.0 & !(0x1fff << 16usize)) | (((val as u32) & 0x1fff) << 16usize);
        }
    }
    impl Default for EccSection10Synd78 {
        #[inline(always)]
        fn default() -> EccSection10Synd78 {
            EccSection10Synd78(0)
        }
    }
    impl core::fmt::Debug for EccSection10Synd78 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("EccSection10Synd78")
                .field("s7", &self.s7())
                .field("s8", &self.s8())
                .finish()
        }
    }
    #[doc = "desc ECC_SECTION11_SYND12."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct EccSection11Synd12(pub u32);
    impl EccSection11Synd12 {
        #[doc = "desc S1."]
        #[must_use]
        #[inline(always)]
        pub const fn s1(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x1fff;
            val as u16
        }
        #[doc = "desc S1."]
        #[inline(always)]
        pub const fn set_s1(&mut self, val: u16) {
            self.0 = (self.0 & !(0x1fff << 0usize)) | (((val as u32) & 0x1fff) << 0usize);
        }
        #[doc = "desc S2."]
        #[must_use]
        #[inline(always)]
        pub const fn s2(&self) -> u16 {
            let val = (self.0 >> 16usize) & 0x1fff;
            val as u16
        }
        #[doc = "desc S2."]
        #[inline(always)]
        pub const fn set_s2(&mut self, val: u16) {
            self.0 = (self.0 & !(0x1fff << 16usize)) | (((val as u32) & 0x1fff) << 16usize);
        }
    }
    impl Default for EccSection11Synd12 {
        #[inline(always)]
        fn default() -> EccSection11Synd12 {
            EccSection11Synd12(0)
        }
    }
    impl core::fmt::Debug for EccSection11Synd12 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("EccSection11Synd12")
                .field("s1", &self.s1())
                .field("s2", &self.s2())
                .finish()
        }
    }
    #[doc = "desc ECC_SECTION11_SYND34."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct EccSection11Synd34(pub u32);
    impl EccSection11Synd34 {
        #[doc = "desc S3."]
        #[must_use]
        #[inline(always)]
        pub const fn s3(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x1fff;
            val as u16
        }
        #[doc = "desc S3."]
        #[inline(always)]
        pub const fn set_s3(&mut self, val: u16) {
            self.0 = (self.0 & !(0x1fff << 0usize)) | (((val as u32) & 0x1fff) << 0usize);
        }
        #[doc = "desc S4."]
        #[must_use]
        #[inline(always)]
        pub const fn s4(&self) -> u16 {
            let val = (self.0 >> 16usize) & 0x1fff;
            val as u16
        }
        #[doc = "desc S4."]
        #[inline(always)]
        pub const fn set_s4(&mut self, val: u16) {
            self.0 = (self.0 & !(0x1fff << 16usize)) | (((val as u32) & 0x1fff) << 16usize);
        }
    }
    impl Default for EccSection11Synd34 {
        #[inline(always)]
        fn default() -> EccSection11Synd34 {
            EccSection11Synd34(0)
        }
    }
    impl core::fmt::Debug for EccSection11Synd34 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("EccSection11Synd34")
                .field("s3", &self.s3())
                .field("s4", &self.s4())
                .finish()
        }
    }
    #[doc = "desc ECC_SECTION11_SYND56."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct EccSection11Synd56(pub u32);
    impl EccSection11Synd56 {
        #[doc = "desc S5."]
        #[must_use]
        #[inline(always)]
        pub const fn s5(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x1fff;
            val as u16
        }
        #[doc = "desc S5."]
        #[inline(always)]
        pub const fn set_s5(&mut self, val: u16) {
            self.0 = (self.0 & !(0x1fff << 0usize)) | (((val as u32) & 0x1fff) << 0usize);
        }
        #[doc = "desc S6."]
        #[must_use]
        #[inline(always)]
        pub const fn s6(&self) -> u16 {
            let val = (self.0 >> 16usize) & 0x1fff;
            val as u16
        }
        #[doc = "desc S6."]
        #[inline(always)]
        pub const fn set_s6(&mut self, val: u16) {
            self.0 = (self.0 & !(0x1fff << 16usize)) | (((val as u32) & 0x1fff) << 16usize);
        }
    }
    impl Default for EccSection11Synd56 {
        #[inline(always)]
        fn default() -> EccSection11Synd56 {
            EccSection11Synd56(0)
        }
    }
    impl core::fmt::Debug for EccSection11Synd56 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("EccSection11Synd56")
                .field("s5", &self.s5())
                .field("s6", &self.s6())
                .finish()
        }
    }
    #[doc = "desc ECC_SECTION11_SYND78."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct EccSection11Synd78(pub u32);
    impl EccSection11Synd78 {
        #[doc = "desc S7."]
        #[must_use]
        #[inline(always)]
        pub const fn s7(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x1fff;
            val as u16
        }
        #[doc = "desc S7."]
        #[inline(always)]
        pub const fn set_s7(&mut self, val: u16) {
            self.0 = (self.0 & !(0x1fff << 0usize)) | (((val as u32) & 0x1fff) << 0usize);
        }
        #[doc = "desc S8."]
        #[must_use]
        #[inline(always)]
        pub const fn s8(&self) -> u16 {
            let val = (self.0 >> 16usize) & 0x1fff;
            val as u16
        }
        #[doc = "desc S8."]
        #[inline(always)]
        pub const fn set_s8(&mut self, val: u16) {
            self.0 = (self.0 & !(0x1fff << 16usize)) | (((val as u32) & 0x1fff) << 16usize);
        }
    }
    impl Default for EccSection11Synd78 {
        #[inline(always)]
        fn default() -> EccSection11Synd78 {
            EccSection11Synd78(0)
        }
    }
    impl core::fmt::Debug for EccSection11Synd78 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("EccSection11Synd78")
                .field("s7", &self.s7())
                .field("s8", &self.s8())
                .finish()
        }
    }
    #[doc = "desc ECC_SECTION12_SYND12."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct EccSection12Synd12(pub u32);
    impl EccSection12Synd12 {
        #[doc = "desc S1."]
        #[must_use]
        #[inline(always)]
        pub const fn s1(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x1fff;
            val as u16
        }
        #[doc = "desc S1."]
        #[inline(always)]
        pub const fn set_s1(&mut self, val: u16) {
            self.0 = (self.0 & !(0x1fff << 0usize)) | (((val as u32) & 0x1fff) << 0usize);
        }
        #[doc = "desc S2."]
        #[must_use]
        #[inline(always)]
        pub const fn s2(&self) -> u16 {
            let val = (self.0 >> 16usize) & 0x1fff;
            val as u16
        }
        #[doc = "desc S2."]
        #[inline(always)]
        pub const fn set_s2(&mut self, val: u16) {
            self.0 = (self.0 & !(0x1fff << 16usize)) | (((val as u32) & 0x1fff) << 16usize);
        }
    }
    impl Default for EccSection12Synd12 {
        #[inline(always)]
        fn default() -> EccSection12Synd12 {
            EccSection12Synd12(0)
        }
    }
    impl core::fmt::Debug for EccSection12Synd12 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("EccSection12Synd12")
                .field("s1", &self.s1())
                .field("s2", &self.s2())
                .finish()
        }
    }
    #[doc = "desc ECC_SECTION12_SYND34."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct EccSection12Synd34(pub u32);
    impl EccSection12Synd34 {
        #[doc = "desc S3."]
        #[must_use]
        #[inline(always)]
        pub const fn s3(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x1fff;
            val as u16
        }
        #[doc = "desc S3."]
        #[inline(always)]
        pub const fn set_s3(&mut self, val: u16) {
            self.0 = (self.0 & !(0x1fff << 0usize)) | (((val as u32) & 0x1fff) << 0usize);
        }
        #[doc = "desc S4."]
        #[must_use]
        #[inline(always)]
        pub const fn s4(&self) -> u16 {
            let val = (self.0 >> 16usize) & 0x1fff;
            val as u16
        }
        #[doc = "desc S4."]
        #[inline(always)]
        pub const fn set_s4(&mut self, val: u16) {
            self.0 = (self.0 & !(0x1fff << 16usize)) | (((val as u32) & 0x1fff) << 16usize);
        }
    }
    impl Default for EccSection12Synd34 {
        #[inline(always)]
        fn default() -> EccSection12Synd34 {
            EccSection12Synd34(0)
        }
    }
    impl core::fmt::Debug for EccSection12Synd34 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("EccSection12Synd34")
                .field("s3", &self.s3())
                .field("s4", &self.s4())
                .finish()
        }
    }
    #[doc = "desc ECC_SECTION12_SYND56."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct EccSection12Synd56(pub u32);
    impl EccSection12Synd56 {
        #[doc = "desc S5."]
        #[must_use]
        #[inline(always)]
        pub const fn s5(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x1fff;
            val as u16
        }
        #[doc = "desc S5."]
        #[inline(always)]
        pub const fn set_s5(&mut self, val: u16) {
            self.0 = (self.0 & !(0x1fff << 0usize)) | (((val as u32) & 0x1fff) << 0usize);
        }
        #[doc = "desc S6."]
        #[must_use]
        #[inline(always)]
        pub const fn s6(&self) -> u16 {
            let val = (self.0 >> 16usize) & 0x1fff;
            val as u16
        }
        #[doc = "desc S6."]
        #[inline(always)]
        pub const fn set_s6(&mut self, val: u16) {
            self.0 = (self.0 & !(0x1fff << 16usize)) | (((val as u32) & 0x1fff) << 16usize);
        }
    }
    impl Default for EccSection12Synd56 {
        #[inline(always)]
        fn default() -> EccSection12Synd56 {
            EccSection12Synd56(0)
        }
    }
    impl core::fmt::Debug for EccSection12Synd56 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("EccSection12Synd56")
                .field("s5", &self.s5())
                .field("s6", &self.s6())
                .finish()
        }
    }
    #[doc = "desc ECC_SECTION12_SYND78."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct EccSection12Synd78(pub u32);
    impl EccSection12Synd78 {
        #[doc = "desc S7."]
        #[must_use]
        #[inline(always)]
        pub const fn s7(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x1fff;
            val as u16
        }
        #[doc = "desc S7."]
        #[inline(always)]
        pub const fn set_s7(&mut self, val: u16) {
            self.0 = (self.0 & !(0x1fff << 0usize)) | (((val as u32) & 0x1fff) << 0usize);
        }
        #[doc = "desc S8."]
        #[must_use]
        #[inline(always)]
        pub const fn s8(&self) -> u16 {
            let val = (self.0 >> 16usize) & 0x1fff;
            val as u16
        }
        #[doc = "desc S8."]
        #[inline(always)]
        pub const fn set_s8(&mut self, val: u16) {
            self.0 = (self.0 & !(0x1fff << 16usize)) | (((val as u32) & 0x1fff) << 16usize);
        }
    }
    impl Default for EccSection12Synd78 {
        #[inline(always)]
        fn default() -> EccSection12Synd78 {
            EccSection12Synd78(0)
        }
    }
    impl core::fmt::Debug for EccSection12Synd78 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("EccSection12Synd78")
                .field("s7", &self.s7())
                .field("s8", &self.s8())
                .finish()
        }
    }
    #[doc = "desc ECC_SECTION13_SYND12."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct EccSection13Synd12(pub u32);
    impl EccSection13Synd12 {
        #[doc = "desc S1."]
        #[must_use]
        #[inline(always)]
        pub const fn s1(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x1fff;
            val as u16
        }
        #[doc = "desc S1."]
        #[inline(always)]
        pub const fn set_s1(&mut self, val: u16) {
            self.0 = (self.0 & !(0x1fff << 0usize)) | (((val as u32) & 0x1fff) << 0usize);
        }
        #[doc = "desc S2."]
        #[must_use]
        #[inline(always)]
        pub const fn s2(&self) -> u16 {
            let val = (self.0 >> 16usize) & 0x1fff;
            val as u16
        }
        #[doc = "desc S2."]
        #[inline(always)]
        pub const fn set_s2(&mut self, val: u16) {
            self.0 = (self.0 & !(0x1fff << 16usize)) | (((val as u32) & 0x1fff) << 16usize);
        }
    }
    impl Default for EccSection13Synd12 {
        #[inline(always)]
        fn default() -> EccSection13Synd12 {
            EccSection13Synd12(0)
        }
    }
    impl core::fmt::Debug for EccSection13Synd12 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("EccSection13Synd12")
                .field("s1", &self.s1())
                .field("s2", &self.s2())
                .finish()
        }
    }
    #[doc = "desc ECC_SECTION13_SYND34."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct EccSection13Synd34(pub u32);
    impl EccSection13Synd34 {
        #[doc = "desc S3."]
        #[must_use]
        #[inline(always)]
        pub const fn s3(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x1fff;
            val as u16
        }
        #[doc = "desc S3."]
        #[inline(always)]
        pub const fn set_s3(&mut self, val: u16) {
            self.0 = (self.0 & !(0x1fff << 0usize)) | (((val as u32) & 0x1fff) << 0usize);
        }
        #[doc = "desc S4."]
        #[must_use]
        #[inline(always)]
        pub const fn s4(&self) -> u16 {
            let val = (self.0 >> 16usize) & 0x1fff;
            val as u16
        }
        #[doc = "desc S4."]
        #[inline(always)]
        pub const fn set_s4(&mut self, val: u16) {
            self.0 = (self.0 & !(0x1fff << 16usize)) | (((val as u32) & 0x1fff) << 16usize);
        }
    }
    impl Default for EccSection13Synd34 {
        #[inline(always)]
        fn default() -> EccSection13Synd34 {
            EccSection13Synd34(0)
        }
    }
    impl core::fmt::Debug for EccSection13Synd34 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("EccSection13Synd34")
                .field("s3", &self.s3())
                .field("s4", &self.s4())
                .finish()
        }
    }
    #[doc = "desc ECC_SECTION13_SYND56."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct EccSection13Synd56(pub u32);
    impl EccSection13Synd56 {
        #[doc = "desc S5."]
        #[must_use]
        #[inline(always)]
        pub const fn s5(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x1fff;
            val as u16
        }
        #[doc = "desc S5."]
        #[inline(always)]
        pub const fn set_s5(&mut self, val: u16) {
            self.0 = (self.0 & !(0x1fff << 0usize)) | (((val as u32) & 0x1fff) << 0usize);
        }
        #[doc = "desc S6."]
        #[must_use]
        #[inline(always)]
        pub const fn s6(&self) -> u16 {
            let val = (self.0 >> 16usize) & 0x1fff;
            val as u16
        }
        #[doc = "desc S6."]
        #[inline(always)]
        pub const fn set_s6(&mut self, val: u16) {
            self.0 = (self.0 & !(0x1fff << 16usize)) | (((val as u32) & 0x1fff) << 16usize);
        }
    }
    impl Default for EccSection13Synd56 {
        #[inline(always)]
        fn default() -> EccSection13Synd56 {
            EccSection13Synd56(0)
        }
    }
    impl core::fmt::Debug for EccSection13Synd56 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("EccSection13Synd56")
                .field("s5", &self.s5())
                .field("s6", &self.s6())
                .finish()
        }
    }
    #[doc = "desc ECC_SECTION13_SYND78."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct EccSection13Synd78(pub u32);
    impl EccSection13Synd78 {
        #[doc = "desc S7."]
        #[must_use]
        #[inline(always)]
        pub const fn s7(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x1fff;
            val as u16
        }
        #[doc = "desc S7."]
        #[inline(always)]
        pub const fn set_s7(&mut self, val: u16) {
            self.0 = (self.0 & !(0x1fff << 0usize)) | (((val as u32) & 0x1fff) << 0usize);
        }
        #[doc = "desc S8."]
        #[must_use]
        #[inline(always)]
        pub const fn s8(&self) -> u16 {
            let val = (self.0 >> 16usize) & 0x1fff;
            val as u16
        }
        #[doc = "desc S8."]
        #[inline(always)]
        pub const fn set_s8(&mut self, val: u16) {
            self.0 = (self.0 & !(0x1fff << 16usize)) | (((val as u32) & 0x1fff) << 16usize);
        }
    }
    impl Default for EccSection13Synd78 {
        #[inline(always)]
        fn default() -> EccSection13Synd78 {
            EccSection13Synd78(0)
        }
    }
    impl core::fmt::Debug for EccSection13Synd78 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("EccSection13Synd78")
                .field("s7", &self.s7())
                .field("s8", &self.s8())
                .finish()
        }
    }
    #[doc = "desc ECC_SECTION14_SYND12."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct EccSection14Synd12(pub u32);
    impl EccSection14Synd12 {
        #[doc = "desc S1."]
        #[must_use]
        #[inline(always)]
        pub const fn s1(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x1fff;
            val as u16
        }
        #[doc = "desc S1."]
        #[inline(always)]
        pub const fn set_s1(&mut self, val: u16) {
            self.0 = (self.0 & !(0x1fff << 0usize)) | (((val as u32) & 0x1fff) << 0usize);
        }
        #[doc = "desc S2."]
        #[must_use]
        #[inline(always)]
        pub const fn s2(&self) -> u16 {
            let val = (self.0 >> 16usize) & 0x1fff;
            val as u16
        }
        #[doc = "desc S2."]
        #[inline(always)]
        pub const fn set_s2(&mut self, val: u16) {
            self.0 = (self.0 & !(0x1fff << 16usize)) | (((val as u32) & 0x1fff) << 16usize);
        }
    }
    impl Default for EccSection14Synd12 {
        #[inline(always)]
        fn default() -> EccSection14Synd12 {
            EccSection14Synd12(0)
        }
    }
    impl core::fmt::Debug for EccSection14Synd12 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("EccSection14Synd12")
                .field("s1", &self.s1())
                .field("s2", &self.s2())
                .finish()
        }
    }
    #[doc = "desc ECC_SECTION14_SYND34."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct EccSection14Synd34(pub u32);
    impl EccSection14Synd34 {
        #[doc = "desc S3."]
        #[must_use]
        #[inline(always)]
        pub const fn s3(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x1fff;
            val as u16
        }
        #[doc = "desc S3."]
        #[inline(always)]
        pub const fn set_s3(&mut self, val: u16) {
            self.0 = (self.0 & !(0x1fff << 0usize)) | (((val as u32) & 0x1fff) << 0usize);
        }
        #[doc = "desc S4."]
        #[must_use]
        #[inline(always)]
        pub const fn s4(&self) -> u16 {
            let val = (self.0 >> 16usize) & 0x1fff;
            val as u16
        }
        #[doc = "desc S4."]
        #[inline(always)]
        pub const fn set_s4(&mut self, val: u16) {
            self.0 = (self.0 & !(0x1fff << 16usize)) | (((val as u32) & 0x1fff) << 16usize);
        }
    }
    impl Default for EccSection14Synd34 {
        #[inline(always)]
        fn default() -> EccSection14Synd34 {
            EccSection14Synd34(0)
        }
    }
    impl core::fmt::Debug for EccSection14Synd34 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("EccSection14Synd34")
                .field("s3", &self.s3())
                .field("s4", &self.s4())
                .finish()
        }
    }
    #[doc = "desc ECC_SECTION14_SYND56."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct EccSection14Synd56(pub u32);
    impl EccSection14Synd56 {
        #[doc = "desc S5."]
        #[must_use]
        #[inline(always)]
        pub const fn s5(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x1fff;
            val as u16
        }
        #[doc = "desc S5."]
        #[inline(always)]
        pub const fn set_s5(&mut self, val: u16) {
            self.0 = (self.0 & !(0x1fff << 0usize)) | (((val as u32) & 0x1fff) << 0usize);
        }
        #[doc = "desc S6."]
        #[must_use]
        #[inline(always)]
        pub const fn s6(&self) -> u16 {
            let val = (self.0 >> 16usize) & 0x1fff;
            val as u16
        }
        #[doc = "desc S6."]
        #[inline(always)]
        pub const fn set_s6(&mut self, val: u16) {
            self.0 = (self.0 & !(0x1fff << 16usize)) | (((val as u32) & 0x1fff) << 16usize);
        }
    }
    impl Default for EccSection14Synd56 {
        #[inline(always)]
        fn default() -> EccSection14Synd56 {
            EccSection14Synd56(0)
        }
    }
    impl core::fmt::Debug for EccSection14Synd56 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("EccSection14Synd56")
                .field("s5", &self.s5())
                .field("s6", &self.s6())
                .finish()
        }
    }
    #[doc = "desc ECC_SECTION14_SYND78."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct EccSection14Synd78(pub u32);
    impl EccSection14Synd78 {
        #[doc = "desc S7."]
        #[must_use]
        #[inline(always)]
        pub const fn s7(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x1fff;
            val as u16
        }
        #[doc = "desc S7."]
        #[inline(always)]
        pub const fn set_s7(&mut self, val: u16) {
            self.0 = (self.0 & !(0x1fff << 0usize)) | (((val as u32) & 0x1fff) << 0usize);
        }
        #[doc = "desc S8."]
        #[must_use]
        #[inline(always)]
        pub const fn s8(&self) -> u16 {
            let val = (self.0 >> 16usize) & 0x1fff;
            val as u16
        }
        #[doc = "desc S8."]
        #[inline(always)]
        pub const fn set_s8(&mut self, val: u16) {
            self.0 = (self.0 & !(0x1fff << 16usize)) | (((val as u32) & 0x1fff) << 16usize);
        }
    }
    impl Default for EccSection14Synd78 {
        #[inline(always)]
        fn default() -> EccSection14Synd78 {
            EccSection14Synd78(0)
        }
    }
    impl core::fmt::Debug for EccSection14Synd78 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("EccSection14Synd78")
                .field("s7", &self.s7())
                .field("s8", &self.s8())
                .finish()
        }
    }
    #[doc = "desc ECC_SECTION15_SYND12."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct EccSection15Synd12(pub u32);
    impl EccSection15Synd12 {
        #[doc = "desc S1."]
        #[must_use]
        #[inline(always)]
        pub const fn s1(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x1fff;
            val as u16
        }
        #[doc = "desc S1."]
        #[inline(always)]
        pub const fn set_s1(&mut self, val: u16) {
            self.0 = (self.0 & !(0x1fff << 0usize)) | (((val as u32) & 0x1fff) << 0usize);
        }
        #[doc = "desc S2."]
        #[must_use]
        #[inline(always)]
        pub const fn s2(&self) -> u16 {
            let val = (self.0 >> 16usize) & 0x1fff;
            val as u16
        }
        #[doc = "desc S2."]
        #[inline(always)]
        pub const fn set_s2(&mut self, val: u16) {
            self.0 = (self.0 & !(0x1fff << 16usize)) | (((val as u32) & 0x1fff) << 16usize);
        }
    }
    impl Default for EccSection15Synd12 {
        #[inline(always)]
        fn default() -> EccSection15Synd12 {
            EccSection15Synd12(0)
        }
    }
    impl core::fmt::Debug for EccSection15Synd12 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("EccSection15Synd12")
                .field("s1", &self.s1())
                .field("s2", &self.s2())
                .finish()
        }
    }
    #[doc = "desc ECC_SECTION15_SYND34."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct EccSection15Synd34(pub u32);
    impl EccSection15Synd34 {
        #[doc = "desc S3."]
        #[must_use]
        #[inline(always)]
        pub const fn s3(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x1fff;
            val as u16
        }
        #[doc = "desc S3."]
        #[inline(always)]
        pub const fn set_s3(&mut self, val: u16) {
            self.0 = (self.0 & !(0x1fff << 0usize)) | (((val as u32) & 0x1fff) << 0usize);
        }
        #[doc = "desc S4."]
        #[must_use]
        #[inline(always)]
        pub const fn s4(&self) -> u16 {
            let val = (self.0 >> 16usize) & 0x1fff;
            val as u16
        }
        #[doc = "desc S4."]
        #[inline(always)]
        pub const fn set_s4(&mut self, val: u16) {
            self.0 = (self.0 & !(0x1fff << 16usize)) | (((val as u32) & 0x1fff) << 16usize);
        }
    }
    impl Default for EccSection15Synd34 {
        #[inline(always)]
        fn default() -> EccSection15Synd34 {
            EccSection15Synd34(0)
        }
    }
    impl core::fmt::Debug for EccSection15Synd34 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("EccSection15Synd34")
                .field("s3", &self.s3())
                .field("s4", &self.s4())
                .finish()
        }
    }
    #[doc = "desc ECC_SECTION15_SYND56."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct EccSection15Synd56(pub u32);
    impl EccSection15Synd56 {
        #[doc = "desc S5."]
        #[must_use]
        #[inline(always)]
        pub const fn s5(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x1fff;
            val as u16
        }
        #[doc = "desc S5."]
        #[inline(always)]
        pub const fn set_s5(&mut self, val: u16) {
            self.0 = (self.0 & !(0x1fff << 0usize)) | (((val as u32) & 0x1fff) << 0usize);
        }
        #[doc = "desc S6."]
        #[must_use]
        #[inline(always)]
        pub const fn s6(&self) -> u16 {
            let val = (self.0 >> 16usize) & 0x1fff;
            val as u16
        }
        #[doc = "desc S6."]
        #[inline(always)]
        pub const fn set_s6(&mut self, val: u16) {
            self.0 = (self.0 & !(0x1fff << 16usize)) | (((val as u32) & 0x1fff) << 16usize);
        }
    }
    impl Default for EccSection15Synd56 {
        #[inline(always)]
        fn default() -> EccSection15Synd56 {
            EccSection15Synd56(0)
        }
    }
    impl core::fmt::Debug for EccSection15Synd56 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("EccSection15Synd56")
                .field("s5", &self.s5())
                .field("s6", &self.s6())
                .finish()
        }
    }
    #[doc = "desc ECC_SECTION15_SYND78."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct EccSection15Synd78(pub u32);
    impl EccSection15Synd78 {
        #[doc = "desc S7."]
        #[must_use]
        #[inline(always)]
        pub const fn s7(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x1fff;
            val as u16
        }
        #[doc = "desc S7."]
        #[inline(always)]
        pub const fn set_s7(&mut self, val: u16) {
            self.0 = (self.0 & !(0x1fff << 0usize)) | (((val as u32) & 0x1fff) << 0usize);
        }
        #[doc = "desc S8."]
        #[must_use]
        #[inline(always)]
        pub const fn s8(&self) -> u16 {
            let val = (self.0 >> 16usize) & 0x1fff;
            val as u16
        }
        #[doc = "desc S8."]
        #[inline(always)]
        pub const fn set_s8(&mut self, val: u16) {
            self.0 = (self.0 & !(0x1fff << 16usize)) | (((val as u32) & 0x1fff) << 16usize);
        }
    }
    impl Default for EccSection15Synd78 {
        #[inline(always)]
        fn default() -> EccSection15Synd78 {
            EccSection15Synd78(0)
        }
    }
    impl core::fmt::Debug for EccSection15Synd78 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("EccSection15Synd78")
                .field("s7", &self.s7())
                .field("s8", &self.s8())
                .finish()
        }
    }
    #[doc = "desc ECC_SECTION1_SYND12."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct EccSection1Synd12(pub u32);
    impl EccSection1Synd12 {
        #[doc = "desc S1."]
        #[must_use]
        #[inline(always)]
        pub const fn s1(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x1fff;
            val as u16
        }
        #[doc = "desc S1."]
        #[inline(always)]
        pub const fn set_s1(&mut self, val: u16) {
            self.0 = (self.0 & !(0x1fff << 0usize)) | (((val as u32) & 0x1fff) << 0usize);
        }
        #[doc = "desc S2."]
        #[must_use]
        #[inline(always)]
        pub const fn s2(&self) -> u16 {
            let val = (self.0 >> 16usize) & 0x1fff;
            val as u16
        }
        #[doc = "desc S2."]
        #[inline(always)]
        pub const fn set_s2(&mut self, val: u16) {
            self.0 = (self.0 & !(0x1fff << 16usize)) | (((val as u32) & 0x1fff) << 16usize);
        }
    }
    impl Default for EccSection1Synd12 {
        #[inline(always)]
        fn default() -> EccSection1Synd12 {
            EccSection1Synd12(0)
        }
    }
    impl core::fmt::Debug for EccSection1Synd12 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("EccSection1Synd12")
                .field("s1", &self.s1())
                .field("s2", &self.s2())
                .finish()
        }
    }
    #[doc = "desc ECC_SECTION1_SYND34."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct EccSection1Synd34(pub u32);
    impl EccSection1Synd34 {
        #[doc = "desc S3."]
        #[must_use]
        #[inline(always)]
        pub const fn s3(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x1fff;
            val as u16
        }
        #[doc = "desc S3."]
        #[inline(always)]
        pub const fn set_s3(&mut self, val: u16) {
            self.0 = (self.0 & !(0x1fff << 0usize)) | (((val as u32) & 0x1fff) << 0usize);
        }
        #[doc = "desc S4."]
        #[must_use]
        #[inline(always)]
        pub const fn s4(&self) -> u16 {
            let val = (self.0 >> 16usize) & 0x1fff;
            val as u16
        }
        #[doc = "desc S4."]
        #[inline(always)]
        pub const fn set_s4(&mut self, val: u16) {
            self.0 = (self.0 & !(0x1fff << 16usize)) | (((val as u32) & 0x1fff) << 16usize);
        }
    }
    impl Default for EccSection1Synd34 {
        #[inline(always)]
        fn default() -> EccSection1Synd34 {
            EccSection1Synd34(0)
        }
    }
    impl core::fmt::Debug for EccSection1Synd34 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("EccSection1Synd34")
                .field("s3", &self.s3())
                .field("s4", &self.s4())
                .finish()
        }
    }
    #[doc = "desc ECC_SECTION1_SYND56."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct EccSection1Synd56(pub u32);
    impl EccSection1Synd56 {
        #[doc = "desc S5."]
        #[must_use]
        #[inline(always)]
        pub const fn s5(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x1fff;
            val as u16
        }
        #[doc = "desc S5."]
        #[inline(always)]
        pub const fn set_s5(&mut self, val: u16) {
            self.0 = (self.0 & !(0x1fff << 0usize)) | (((val as u32) & 0x1fff) << 0usize);
        }
        #[doc = "desc S6."]
        #[must_use]
        #[inline(always)]
        pub const fn s6(&self) -> u16 {
            let val = (self.0 >> 16usize) & 0x1fff;
            val as u16
        }
        #[doc = "desc S6."]
        #[inline(always)]
        pub const fn set_s6(&mut self, val: u16) {
            self.0 = (self.0 & !(0x1fff << 16usize)) | (((val as u32) & 0x1fff) << 16usize);
        }
    }
    impl Default for EccSection1Synd56 {
        #[inline(always)]
        fn default() -> EccSection1Synd56 {
            EccSection1Synd56(0)
        }
    }
    impl core::fmt::Debug for EccSection1Synd56 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("EccSection1Synd56")
                .field("s5", &self.s5())
                .field("s6", &self.s6())
                .finish()
        }
    }
    #[doc = "desc ECC_SECTION1_SYND78."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct EccSection1Synd78(pub u32);
    impl EccSection1Synd78 {
        #[doc = "desc S7."]
        #[must_use]
        #[inline(always)]
        pub const fn s7(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x1fff;
            val as u16
        }
        #[doc = "desc S7."]
        #[inline(always)]
        pub const fn set_s7(&mut self, val: u16) {
            self.0 = (self.0 & !(0x1fff << 0usize)) | (((val as u32) & 0x1fff) << 0usize);
        }
        #[doc = "desc S8."]
        #[must_use]
        #[inline(always)]
        pub const fn s8(&self) -> u16 {
            let val = (self.0 >> 16usize) & 0x1fff;
            val as u16
        }
        #[doc = "desc S8."]
        #[inline(always)]
        pub const fn set_s8(&mut self, val: u16) {
            self.0 = (self.0 & !(0x1fff << 16usize)) | (((val as u32) & 0x1fff) << 16usize);
        }
    }
    impl Default for EccSection1Synd78 {
        #[inline(always)]
        fn default() -> EccSection1Synd78 {
            EccSection1Synd78(0)
        }
    }
    impl core::fmt::Debug for EccSection1Synd78 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("EccSection1Synd78")
                .field("s7", &self.s7())
                .field("s8", &self.s8())
                .finish()
        }
    }
    #[doc = "desc ECC_SECTION2_SYND12."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct EccSection2Synd12(pub u32);
    impl EccSection2Synd12 {
        #[doc = "desc S1."]
        #[must_use]
        #[inline(always)]
        pub const fn s1(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x1fff;
            val as u16
        }
        #[doc = "desc S1."]
        #[inline(always)]
        pub const fn set_s1(&mut self, val: u16) {
            self.0 = (self.0 & !(0x1fff << 0usize)) | (((val as u32) & 0x1fff) << 0usize);
        }
        #[doc = "desc S2."]
        #[must_use]
        #[inline(always)]
        pub const fn s2(&self) -> u16 {
            let val = (self.0 >> 16usize) & 0x1fff;
            val as u16
        }
        #[doc = "desc S2."]
        #[inline(always)]
        pub const fn set_s2(&mut self, val: u16) {
            self.0 = (self.0 & !(0x1fff << 16usize)) | (((val as u32) & 0x1fff) << 16usize);
        }
    }
    impl Default for EccSection2Synd12 {
        #[inline(always)]
        fn default() -> EccSection2Synd12 {
            EccSection2Synd12(0)
        }
    }
    impl core::fmt::Debug for EccSection2Synd12 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("EccSection2Synd12")
                .field("s1", &self.s1())
                .field("s2", &self.s2())
                .finish()
        }
    }
    #[doc = "desc ECC_SECTION2_SYND34."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct EccSection2Synd34(pub u32);
    impl EccSection2Synd34 {
        #[doc = "desc S3."]
        #[must_use]
        #[inline(always)]
        pub const fn s3(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x1fff;
            val as u16
        }
        #[doc = "desc S3."]
        #[inline(always)]
        pub const fn set_s3(&mut self, val: u16) {
            self.0 = (self.0 & !(0x1fff << 0usize)) | (((val as u32) & 0x1fff) << 0usize);
        }
        #[doc = "desc S4."]
        #[must_use]
        #[inline(always)]
        pub const fn s4(&self) -> u16 {
            let val = (self.0 >> 16usize) & 0x1fff;
            val as u16
        }
        #[doc = "desc S4."]
        #[inline(always)]
        pub const fn set_s4(&mut self, val: u16) {
            self.0 = (self.0 & !(0x1fff << 16usize)) | (((val as u32) & 0x1fff) << 16usize);
        }
    }
    impl Default for EccSection2Synd34 {
        #[inline(always)]
        fn default() -> EccSection2Synd34 {
            EccSection2Synd34(0)
        }
    }
    impl core::fmt::Debug for EccSection2Synd34 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("EccSection2Synd34")
                .field("s3", &self.s3())
                .field("s4", &self.s4())
                .finish()
        }
    }
    #[doc = "desc ECC_SECTION2_SYND56."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct EccSection2Synd56(pub u32);
    impl EccSection2Synd56 {
        #[doc = "desc S5."]
        #[must_use]
        #[inline(always)]
        pub const fn s5(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x1fff;
            val as u16
        }
        #[doc = "desc S5."]
        #[inline(always)]
        pub const fn set_s5(&mut self, val: u16) {
            self.0 = (self.0 & !(0x1fff << 0usize)) | (((val as u32) & 0x1fff) << 0usize);
        }
        #[doc = "desc S6."]
        #[must_use]
        #[inline(always)]
        pub const fn s6(&self) -> u16 {
            let val = (self.0 >> 16usize) & 0x1fff;
            val as u16
        }
        #[doc = "desc S6."]
        #[inline(always)]
        pub const fn set_s6(&mut self, val: u16) {
            self.0 = (self.0 & !(0x1fff << 16usize)) | (((val as u32) & 0x1fff) << 16usize);
        }
    }
    impl Default for EccSection2Synd56 {
        #[inline(always)]
        fn default() -> EccSection2Synd56 {
            EccSection2Synd56(0)
        }
    }
    impl core::fmt::Debug for EccSection2Synd56 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("EccSection2Synd56")
                .field("s5", &self.s5())
                .field("s6", &self.s6())
                .finish()
        }
    }
    #[doc = "desc ECC_SECTION2_SYND78."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct EccSection2Synd78(pub u32);
    impl EccSection2Synd78 {
        #[doc = "desc S7."]
        #[must_use]
        #[inline(always)]
        pub const fn s7(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x1fff;
            val as u16
        }
        #[doc = "desc S7."]
        #[inline(always)]
        pub const fn set_s7(&mut self, val: u16) {
            self.0 = (self.0 & !(0x1fff << 0usize)) | (((val as u32) & 0x1fff) << 0usize);
        }
        #[doc = "desc S8."]
        #[must_use]
        #[inline(always)]
        pub const fn s8(&self) -> u16 {
            let val = (self.0 >> 16usize) & 0x1fff;
            val as u16
        }
        #[doc = "desc S8."]
        #[inline(always)]
        pub const fn set_s8(&mut self, val: u16) {
            self.0 = (self.0 & !(0x1fff << 16usize)) | (((val as u32) & 0x1fff) << 16usize);
        }
    }
    impl Default for EccSection2Synd78 {
        #[inline(always)]
        fn default() -> EccSection2Synd78 {
            EccSection2Synd78(0)
        }
    }
    impl core::fmt::Debug for EccSection2Synd78 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("EccSection2Synd78")
                .field("s7", &self.s7())
                .field("s8", &self.s8())
                .finish()
        }
    }
    #[doc = "desc ECC_SECTION3_SYND12."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct EccSection3Synd12(pub u32);
    impl EccSection3Synd12 {
        #[doc = "desc S1."]
        #[must_use]
        #[inline(always)]
        pub const fn s1(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x1fff;
            val as u16
        }
        #[doc = "desc S1."]
        #[inline(always)]
        pub const fn set_s1(&mut self, val: u16) {
            self.0 = (self.0 & !(0x1fff << 0usize)) | (((val as u32) & 0x1fff) << 0usize);
        }
        #[doc = "desc S2."]
        #[must_use]
        #[inline(always)]
        pub const fn s2(&self) -> u16 {
            let val = (self.0 >> 16usize) & 0x1fff;
            val as u16
        }
        #[doc = "desc S2."]
        #[inline(always)]
        pub const fn set_s2(&mut self, val: u16) {
            self.0 = (self.0 & !(0x1fff << 16usize)) | (((val as u32) & 0x1fff) << 16usize);
        }
    }
    impl Default for EccSection3Synd12 {
        #[inline(always)]
        fn default() -> EccSection3Synd12 {
            EccSection3Synd12(0)
        }
    }
    impl core::fmt::Debug for EccSection3Synd12 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("EccSection3Synd12")
                .field("s1", &self.s1())
                .field("s2", &self.s2())
                .finish()
        }
    }
    #[doc = "desc ECC_SECTION3_SYND34."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct EccSection3Synd34(pub u32);
    impl EccSection3Synd34 {
        #[doc = "desc S3."]
        #[must_use]
        #[inline(always)]
        pub const fn s3(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x1fff;
            val as u16
        }
        #[doc = "desc S3."]
        #[inline(always)]
        pub const fn set_s3(&mut self, val: u16) {
            self.0 = (self.0 & !(0x1fff << 0usize)) | (((val as u32) & 0x1fff) << 0usize);
        }
        #[doc = "desc S4."]
        #[must_use]
        #[inline(always)]
        pub const fn s4(&self) -> u16 {
            let val = (self.0 >> 16usize) & 0x1fff;
            val as u16
        }
        #[doc = "desc S4."]
        #[inline(always)]
        pub const fn set_s4(&mut self, val: u16) {
            self.0 = (self.0 & !(0x1fff << 16usize)) | (((val as u32) & 0x1fff) << 16usize);
        }
    }
    impl Default for EccSection3Synd34 {
        #[inline(always)]
        fn default() -> EccSection3Synd34 {
            EccSection3Synd34(0)
        }
    }
    impl core::fmt::Debug for EccSection3Synd34 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("EccSection3Synd34")
                .field("s3", &self.s3())
                .field("s4", &self.s4())
                .finish()
        }
    }
    #[doc = "desc ECC_SECTION3_SYND56."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct EccSection3Synd56(pub u32);
    impl EccSection3Synd56 {
        #[doc = "desc S5."]
        #[must_use]
        #[inline(always)]
        pub const fn s5(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x1fff;
            val as u16
        }
        #[doc = "desc S5."]
        #[inline(always)]
        pub const fn set_s5(&mut self, val: u16) {
            self.0 = (self.0 & !(0x1fff << 0usize)) | (((val as u32) & 0x1fff) << 0usize);
        }
        #[doc = "desc S6."]
        #[must_use]
        #[inline(always)]
        pub const fn s6(&self) -> u16 {
            let val = (self.0 >> 16usize) & 0x1fff;
            val as u16
        }
        #[doc = "desc S6."]
        #[inline(always)]
        pub const fn set_s6(&mut self, val: u16) {
            self.0 = (self.0 & !(0x1fff << 16usize)) | (((val as u32) & 0x1fff) << 16usize);
        }
    }
    impl Default for EccSection3Synd56 {
        #[inline(always)]
        fn default() -> EccSection3Synd56 {
            EccSection3Synd56(0)
        }
    }
    impl core::fmt::Debug for EccSection3Synd56 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("EccSection3Synd56")
                .field("s5", &self.s5())
                .field("s6", &self.s6())
                .finish()
        }
    }
    #[doc = "desc ECC_SECTION3_SYND78."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct EccSection3Synd78(pub u32);
    impl EccSection3Synd78 {
        #[doc = "desc S7."]
        #[must_use]
        #[inline(always)]
        pub const fn s7(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x1fff;
            val as u16
        }
        #[doc = "desc S7."]
        #[inline(always)]
        pub const fn set_s7(&mut self, val: u16) {
            self.0 = (self.0 & !(0x1fff << 0usize)) | (((val as u32) & 0x1fff) << 0usize);
        }
        #[doc = "desc S8."]
        #[must_use]
        #[inline(always)]
        pub const fn s8(&self) -> u16 {
            let val = (self.0 >> 16usize) & 0x1fff;
            val as u16
        }
        #[doc = "desc S8."]
        #[inline(always)]
        pub const fn set_s8(&mut self, val: u16) {
            self.0 = (self.0 & !(0x1fff << 16usize)) | (((val as u32) & 0x1fff) << 16usize);
        }
    }
    impl Default for EccSection3Synd78 {
        #[inline(always)]
        fn default() -> EccSection3Synd78 {
            EccSection3Synd78(0)
        }
    }
    impl core::fmt::Debug for EccSection3Synd78 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("EccSection3Synd78")
                .field("s7", &self.s7())
                .field("s8", &self.s8())
                .finish()
        }
    }
    #[doc = "desc ECC_SECTION4_SYND12."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct EccSection4Synd12(pub u32);
    impl EccSection4Synd12 {
        #[doc = "desc S1."]
        #[must_use]
        #[inline(always)]
        pub const fn s1(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x1fff;
            val as u16
        }
        #[doc = "desc S1."]
        #[inline(always)]
        pub const fn set_s1(&mut self, val: u16) {
            self.0 = (self.0 & !(0x1fff << 0usize)) | (((val as u32) & 0x1fff) << 0usize);
        }
        #[doc = "desc S2."]
        #[must_use]
        #[inline(always)]
        pub const fn s2(&self) -> u16 {
            let val = (self.0 >> 16usize) & 0x1fff;
            val as u16
        }
        #[doc = "desc S2."]
        #[inline(always)]
        pub const fn set_s2(&mut self, val: u16) {
            self.0 = (self.0 & !(0x1fff << 16usize)) | (((val as u32) & 0x1fff) << 16usize);
        }
    }
    impl Default for EccSection4Synd12 {
        #[inline(always)]
        fn default() -> EccSection4Synd12 {
            EccSection4Synd12(0)
        }
    }
    impl core::fmt::Debug for EccSection4Synd12 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("EccSection4Synd12")
                .field("s1", &self.s1())
                .field("s2", &self.s2())
                .finish()
        }
    }
    #[doc = "desc ECC_SECTION4_SYND34."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct EccSection4Synd34(pub u32);
    impl EccSection4Synd34 {
        #[doc = "desc S3."]
        #[must_use]
        #[inline(always)]
        pub const fn s3(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x1fff;
            val as u16
        }
        #[doc = "desc S3."]
        #[inline(always)]
        pub const fn set_s3(&mut self, val: u16) {
            self.0 = (self.0 & !(0x1fff << 0usize)) | (((val as u32) & 0x1fff) << 0usize);
        }
        #[doc = "desc S4."]
        #[must_use]
        #[inline(always)]
        pub const fn s4(&self) -> u16 {
            let val = (self.0 >> 16usize) & 0x1fff;
            val as u16
        }
        #[doc = "desc S4."]
        #[inline(always)]
        pub const fn set_s4(&mut self, val: u16) {
            self.0 = (self.0 & !(0x1fff << 16usize)) | (((val as u32) & 0x1fff) << 16usize);
        }
    }
    impl Default for EccSection4Synd34 {
        #[inline(always)]
        fn default() -> EccSection4Synd34 {
            EccSection4Synd34(0)
        }
    }
    impl core::fmt::Debug for EccSection4Synd34 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("EccSection4Synd34")
                .field("s3", &self.s3())
                .field("s4", &self.s4())
                .finish()
        }
    }
    #[doc = "desc ECC_SECTION4_SYND56."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct EccSection4Synd56(pub u32);
    impl EccSection4Synd56 {
        #[doc = "desc S5."]
        #[must_use]
        #[inline(always)]
        pub const fn s5(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x1fff;
            val as u16
        }
        #[doc = "desc S5."]
        #[inline(always)]
        pub const fn set_s5(&mut self, val: u16) {
            self.0 = (self.0 & !(0x1fff << 0usize)) | (((val as u32) & 0x1fff) << 0usize);
        }
        #[doc = "desc S6."]
        #[must_use]
        #[inline(always)]
        pub const fn s6(&self) -> u16 {
            let val = (self.0 >> 16usize) & 0x1fff;
            val as u16
        }
        #[doc = "desc S6."]
        #[inline(always)]
        pub const fn set_s6(&mut self, val: u16) {
            self.0 = (self.0 & !(0x1fff << 16usize)) | (((val as u32) & 0x1fff) << 16usize);
        }
    }
    impl Default for EccSection4Synd56 {
        #[inline(always)]
        fn default() -> EccSection4Synd56 {
            EccSection4Synd56(0)
        }
    }
    impl core::fmt::Debug for EccSection4Synd56 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("EccSection4Synd56")
                .field("s5", &self.s5())
                .field("s6", &self.s6())
                .finish()
        }
    }
    #[doc = "desc ECC_SECTION4_SYND78."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct EccSection4Synd78(pub u32);
    impl EccSection4Synd78 {
        #[doc = "desc S7."]
        #[must_use]
        #[inline(always)]
        pub const fn s7(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x1fff;
            val as u16
        }
        #[doc = "desc S7."]
        #[inline(always)]
        pub const fn set_s7(&mut self, val: u16) {
            self.0 = (self.0 & !(0x1fff << 0usize)) | (((val as u32) & 0x1fff) << 0usize);
        }
        #[doc = "desc S8."]
        #[must_use]
        #[inline(always)]
        pub const fn s8(&self) -> u16 {
            let val = (self.0 >> 16usize) & 0x1fff;
            val as u16
        }
        #[doc = "desc S8."]
        #[inline(always)]
        pub const fn set_s8(&mut self, val: u16) {
            self.0 = (self.0 & !(0x1fff << 16usize)) | (((val as u32) & 0x1fff) << 16usize);
        }
    }
    impl Default for EccSection4Synd78 {
        #[inline(always)]
        fn default() -> EccSection4Synd78 {
            EccSection4Synd78(0)
        }
    }
    impl core::fmt::Debug for EccSection4Synd78 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("EccSection4Synd78")
                .field("s7", &self.s7())
                .field("s8", &self.s8())
                .finish()
        }
    }
    #[doc = "desc ECC_SECTION5_SYND12."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct EccSection5Synd12(pub u32);
    impl EccSection5Synd12 {
        #[doc = "desc S1."]
        #[must_use]
        #[inline(always)]
        pub const fn s1(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x1fff;
            val as u16
        }
        #[doc = "desc S1."]
        #[inline(always)]
        pub const fn set_s1(&mut self, val: u16) {
            self.0 = (self.0 & !(0x1fff << 0usize)) | (((val as u32) & 0x1fff) << 0usize);
        }
        #[doc = "desc S2."]
        #[must_use]
        #[inline(always)]
        pub const fn s2(&self) -> u16 {
            let val = (self.0 >> 16usize) & 0x1fff;
            val as u16
        }
        #[doc = "desc S2."]
        #[inline(always)]
        pub const fn set_s2(&mut self, val: u16) {
            self.0 = (self.0 & !(0x1fff << 16usize)) | (((val as u32) & 0x1fff) << 16usize);
        }
    }
    impl Default for EccSection5Synd12 {
        #[inline(always)]
        fn default() -> EccSection5Synd12 {
            EccSection5Synd12(0)
        }
    }
    impl core::fmt::Debug for EccSection5Synd12 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("EccSection5Synd12")
                .field("s1", &self.s1())
                .field("s2", &self.s2())
                .finish()
        }
    }
    #[doc = "desc ECC_SECTION5_SYND34."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct EccSection5Synd34(pub u32);
    impl EccSection5Synd34 {
        #[doc = "desc S3."]
        #[must_use]
        #[inline(always)]
        pub const fn s3(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x1fff;
            val as u16
        }
        #[doc = "desc S3."]
        #[inline(always)]
        pub const fn set_s3(&mut self, val: u16) {
            self.0 = (self.0 & !(0x1fff << 0usize)) | (((val as u32) & 0x1fff) << 0usize);
        }
        #[doc = "desc S4."]
        #[must_use]
        #[inline(always)]
        pub const fn s4(&self) -> u16 {
            let val = (self.0 >> 16usize) & 0x1fff;
            val as u16
        }
        #[doc = "desc S4."]
        #[inline(always)]
        pub const fn set_s4(&mut self, val: u16) {
            self.0 = (self.0 & !(0x1fff << 16usize)) | (((val as u32) & 0x1fff) << 16usize);
        }
    }
    impl Default for EccSection5Synd34 {
        #[inline(always)]
        fn default() -> EccSection5Synd34 {
            EccSection5Synd34(0)
        }
    }
    impl core::fmt::Debug for EccSection5Synd34 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("EccSection5Synd34")
                .field("s3", &self.s3())
                .field("s4", &self.s4())
                .finish()
        }
    }
    #[doc = "desc ECC_SECTION5_SYND56."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct EccSection5Synd56(pub u32);
    impl EccSection5Synd56 {
        #[doc = "desc S5."]
        #[must_use]
        #[inline(always)]
        pub const fn s5(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x1fff;
            val as u16
        }
        #[doc = "desc S5."]
        #[inline(always)]
        pub const fn set_s5(&mut self, val: u16) {
            self.0 = (self.0 & !(0x1fff << 0usize)) | (((val as u32) & 0x1fff) << 0usize);
        }
        #[doc = "desc S6."]
        #[must_use]
        #[inline(always)]
        pub const fn s6(&self) -> u16 {
            let val = (self.0 >> 16usize) & 0x1fff;
            val as u16
        }
        #[doc = "desc S6."]
        #[inline(always)]
        pub const fn set_s6(&mut self, val: u16) {
            self.0 = (self.0 & !(0x1fff << 16usize)) | (((val as u32) & 0x1fff) << 16usize);
        }
    }
    impl Default for EccSection5Synd56 {
        #[inline(always)]
        fn default() -> EccSection5Synd56 {
            EccSection5Synd56(0)
        }
    }
    impl core::fmt::Debug for EccSection5Synd56 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("EccSection5Synd56")
                .field("s5", &self.s5())
                .field("s6", &self.s6())
                .finish()
        }
    }
    #[doc = "desc ECC_SECTION5_SYND78."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct EccSection5Synd78(pub u32);
    impl EccSection5Synd78 {
        #[doc = "desc S7."]
        #[must_use]
        #[inline(always)]
        pub const fn s7(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x1fff;
            val as u16
        }
        #[doc = "desc S7."]
        #[inline(always)]
        pub const fn set_s7(&mut self, val: u16) {
            self.0 = (self.0 & !(0x1fff << 0usize)) | (((val as u32) & 0x1fff) << 0usize);
        }
        #[doc = "desc S8."]
        #[must_use]
        #[inline(always)]
        pub const fn s8(&self) -> u16 {
            let val = (self.0 >> 16usize) & 0x1fff;
            val as u16
        }
        #[doc = "desc S8."]
        #[inline(always)]
        pub const fn set_s8(&mut self, val: u16) {
            self.0 = (self.0 & !(0x1fff << 16usize)) | (((val as u32) & 0x1fff) << 16usize);
        }
    }
    impl Default for EccSection5Synd78 {
        #[inline(always)]
        fn default() -> EccSection5Synd78 {
            EccSection5Synd78(0)
        }
    }
    impl core::fmt::Debug for EccSection5Synd78 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("EccSection5Synd78")
                .field("s7", &self.s7())
                .field("s8", &self.s8())
                .finish()
        }
    }
    #[doc = "desc ECC_SECTION6_SYND12."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct EccSection6Synd12(pub u32);
    impl EccSection6Synd12 {
        #[doc = "desc S1."]
        #[must_use]
        #[inline(always)]
        pub const fn s1(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x1fff;
            val as u16
        }
        #[doc = "desc S1."]
        #[inline(always)]
        pub const fn set_s1(&mut self, val: u16) {
            self.0 = (self.0 & !(0x1fff << 0usize)) | (((val as u32) & 0x1fff) << 0usize);
        }
        #[doc = "desc S2."]
        #[must_use]
        #[inline(always)]
        pub const fn s2(&self) -> u16 {
            let val = (self.0 >> 16usize) & 0x1fff;
            val as u16
        }
        #[doc = "desc S2."]
        #[inline(always)]
        pub const fn set_s2(&mut self, val: u16) {
            self.0 = (self.0 & !(0x1fff << 16usize)) | (((val as u32) & 0x1fff) << 16usize);
        }
    }
    impl Default for EccSection6Synd12 {
        #[inline(always)]
        fn default() -> EccSection6Synd12 {
            EccSection6Synd12(0)
        }
    }
    impl core::fmt::Debug for EccSection6Synd12 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("EccSection6Synd12")
                .field("s1", &self.s1())
                .field("s2", &self.s2())
                .finish()
        }
    }
    #[doc = "desc ECC_SECTION6_SYND34."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct EccSection6Synd34(pub u32);
    impl EccSection6Synd34 {
        #[doc = "desc S3."]
        #[must_use]
        #[inline(always)]
        pub const fn s3(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x1fff;
            val as u16
        }
        #[doc = "desc S3."]
        #[inline(always)]
        pub const fn set_s3(&mut self, val: u16) {
            self.0 = (self.0 & !(0x1fff << 0usize)) | (((val as u32) & 0x1fff) << 0usize);
        }
        #[doc = "desc S4."]
        #[must_use]
        #[inline(always)]
        pub const fn s4(&self) -> u16 {
            let val = (self.0 >> 16usize) & 0x1fff;
            val as u16
        }
        #[doc = "desc S4."]
        #[inline(always)]
        pub const fn set_s4(&mut self, val: u16) {
            self.0 = (self.0 & !(0x1fff << 16usize)) | (((val as u32) & 0x1fff) << 16usize);
        }
    }
    impl Default for EccSection6Synd34 {
        #[inline(always)]
        fn default() -> EccSection6Synd34 {
            EccSection6Synd34(0)
        }
    }
    impl core::fmt::Debug for EccSection6Synd34 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("EccSection6Synd34")
                .field("s3", &self.s3())
                .field("s4", &self.s4())
                .finish()
        }
    }
    #[doc = "desc ECC_SECTION6_SYND56."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct EccSection6Synd56(pub u32);
    impl EccSection6Synd56 {
        #[doc = "desc S5."]
        #[must_use]
        #[inline(always)]
        pub const fn s5(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x1fff;
            val as u16
        }
        #[doc = "desc S5."]
        #[inline(always)]
        pub const fn set_s5(&mut self, val: u16) {
            self.0 = (self.0 & !(0x1fff << 0usize)) | (((val as u32) & 0x1fff) << 0usize);
        }
        #[doc = "desc S6."]
        #[must_use]
        #[inline(always)]
        pub const fn s6(&self) -> u16 {
            let val = (self.0 >> 16usize) & 0x1fff;
            val as u16
        }
        #[doc = "desc S6."]
        #[inline(always)]
        pub const fn set_s6(&mut self, val: u16) {
            self.0 = (self.0 & !(0x1fff << 16usize)) | (((val as u32) & 0x1fff) << 16usize);
        }
    }
    impl Default for EccSection6Synd56 {
        #[inline(always)]
        fn default() -> EccSection6Synd56 {
            EccSection6Synd56(0)
        }
    }
    impl core::fmt::Debug for EccSection6Synd56 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("EccSection6Synd56")
                .field("s5", &self.s5())
                .field("s6", &self.s6())
                .finish()
        }
    }
    #[doc = "desc ECC_SECTION6_SYND78."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct EccSection6Synd78(pub u32);
    impl EccSection6Synd78 {
        #[doc = "desc S7."]
        #[must_use]
        #[inline(always)]
        pub const fn s7(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x1fff;
            val as u16
        }
        #[doc = "desc S7."]
        #[inline(always)]
        pub const fn set_s7(&mut self, val: u16) {
            self.0 = (self.0 & !(0x1fff << 0usize)) | (((val as u32) & 0x1fff) << 0usize);
        }
        #[doc = "desc S8."]
        #[must_use]
        #[inline(always)]
        pub const fn s8(&self) -> u16 {
            let val = (self.0 >> 16usize) & 0x1fff;
            val as u16
        }
        #[doc = "desc S8."]
        #[inline(always)]
        pub const fn set_s8(&mut self, val: u16) {
            self.0 = (self.0 & !(0x1fff << 16usize)) | (((val as u32) & 0x1fff) << 16usize);
        }
    }
    impl Default for EccSection6Synd78 {
        #[inline(always)]
        fn default() -> EccSection6Synd78 {
            EccSection6Synd78(0)
        }
    }
    impl core::fmt::Debug for EccSection6Synd78 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("EccSection6Synd78")
                .field("s7", &self.s7())
                .field("s8", &self.s8())
                .finish()
        }
    }
    #[doc = "desc ECC_SECTION7_SYND12."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct EccSection7Synd12(pub u32);
    impl EccSection7Synd12 {
        #[doc = "desc S1."]
        #[must_use]
        #[inline(always)]
        pub const fn s1(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x1fff;
            val as u16
        }
        #[doc = "desc S1."]
        #[inline(always)]
        pub const fn set_s1(&mut self, val: u16) {
            self.0 = (self.0 & !(0x1fff << 0usize)) | (((val as u32) & 0x1fff) << 0usize);
        }
        #[doc = "desc S2."]
        #[must_use]
        #[inline(always)]
        pub const fn s2(&self) -> u16 {
            let val = (self.0 >> 16usize) & 0x1fff;
            val as u16
        }
        #[doc = "desc S2."]
        #[inline(always)]
        pub const fn set_s2(&mut self, val: u16) {
            self.0 = (self.0 & !(0x1fff << 16usize)) | (((val as u32) & 0x1fff) << 16usize);
        }
    }
    impl Default for EccSection7Synd12 {
        #[inline(always)]
        fn default() -> EccSection7Synd12 {
            EccSection7Synd12(0)
        }
    }
    impl core::fmt::Debug for EccSection7Synd12 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("EccSection7Synd12")
                .field("s1", &self.s1())
                .field("s2", &self.s2())
                .finish()
        }
    }
    #[doc = "desc ECC_SECTION7_SYND34."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct EccSection7Synd34(pub u32);
    impl EccSection7Synd34 {
        #[doc = "desc S3."]
        #[must_use]
        #[inline(always)]
        pub const fn s3(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x1fff;
            val as u16
        }
        #[doc = "desc S3."]
        #[inline(always)]
        pub const fn set_s3(&mut self, val: u16) {
            self.0 = (self.0 & !(0x1fff << 0usize)) | (((val as u32) & 0x1fff) << 0usize);
        }
        #[doc = "desc S4."]
        #[must_use]
        #[inline(always)]
        pub const fn s4(&self) -> u16 {
            let val = (self.0 >> 16usize) & 0x1fff;
            val as u16
        }
        #[doc = "desc S4."]
        #[inline(always)]
        pub const fn set_s4(&mut self, val: u16) {
            self.0 = (self.0 & !(0x1fff << 16usize)) | (((val as u32) & 0x1fff) << 16usize);
        }
    }
    impl Default for EccSection7Synd34 {
        #[inline(always)]
        fn default() -> EccSection7Synd34 {
            EccSection7Synd34(0)
        }
    }
    impl core::fmt::Debug for EccSection7Synd34 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("EccSection7Synd34")
                .field("s3", &self.s3())
                .field("s4", &self.s4())
                .finish()
        }
    }
    #[doc = "desc ECC_SECTION7_SYND56."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct EccSection7Synd56(pub u32);
    impl EccSection7Synd56 {
        #[doc = "desc S5."]
        #[must_use]
        #[inline(always)]
        pub const fn s5(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x1fff;
            val as u16
        }
        #[doc = "desc S5."]
        #[inline(always)]
        pub const fn set_s5(&mut self, val: u16) {
            self.0 = (self.0 & !(0x1fff << 0usize)) | (((val as u32) & 0x1fff) << 0usize);
        }
        #[doc = "desc S6."]
        #[must_use]
        #[inline(always)]
        pub const fn s6(&self) -> u16 {
            let val = (self.0 >> 16usize) & 0x1fff;
            val as u16
        }
        #[doc = "desc S6."]
        #[inline(always)]
        pub const fn set_s6(&mut self, val: u16) {
            self.0 = (self.0 & !(0x1fff << 16usize)) | (((val as u32) & 0x1fff) << 16usize);
        }
    }
    impl Default for EccSection7Synd56 {
        #[inline(always)]
        fn default() -> EccSection7Synd56 {
            EccSection7Synd56(0)
        }
    }
    impl core::fmt::Debug for EccSection7Synd56 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("EccSection7Synd56")
                .field("s5", &self.s5())
                .field("s6", &self.s6())
                .finish()
        }
    }
    #[doc = "desc ECC_SECTION7_SYND78."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct EccSection7Synd78(pub u32);
    impl EccSection7Synd78 {
        #[doc = "desc S7."]
        #[must_use]
        #[inline(always)]
        pub const fn s7(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x1fff;
            val as u16
        }
        #[doc = "desc S7."]
        #[inline(always)]
        pub const fn set_s7(&mut self, val: u16) {
            self.0 = (self.0 & !(0x1fff << 0usize)) | (((val as u32) & 0x1fff) << 0usize);
        }
        #[doc = "desc S8."]
        #[must_use]
        #[inline(always)]
        pub const fn s8(&self) -> u16 {
            let val = (self.0 >> 16usize) & 0x1fff;
            val as u16
        }
        #[doc = "desc S8."]
        #[inline(always)]
        pub const fn set_s8(&mut self, val: u16) {
            self.0 = (self.0 & !(0x1fff << 16usize)) | (((val as u32) & 0x1fff) << 16usize);
        }
    }
    impl Default for EccSection7Synd78 {
        #[inline(always)]
        fn default() -> EccSection7Synd78 {
            EccSection7Synd78(0)
        }
    }
    impl core::fmt::Debug for EccSection7Synd78 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("EccSection7Synd78")
                .field("s7", &self.s7())
                .field("s8", &self.s8())
                .finish()
        }
    }
    #[doc = "desc ECC_SECTION8_SYND12."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct EccSection8Synd12(pub u32);
    impl EccSection8Synd12 {
        #[doc = "desc S1."]
        #[must_use]
        #[inline(always)]
        pub const fn s1(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x1fff;
            val as u16
        }
        #[doc = "desc S1."]
        #[inline(always)]
        pub const fn set_s1(&mut self, val: u16) {
            self.0 = (self.0 & !(0x1fff << 0usize)) | (((val as u32) & 0x1fff) << 0usize);
        }
        #[doc = "desc S2."]
        #[must_use]
        #[inline(always)]
        pub const fn s2(&self) -> u16 {
            let val = (self.0 >> 16usize) & 0x1fff;
            val as u16
        }
        #[doc = "desc S2."]
        #[inline(always)]
        pub const fn set_s2(&mut self, val: u16) {
            self.0 = (self.0 & !(0x1fff << 16usize)) | (((val as u32) & 0x1fff) << 16usize);
        }
    }
    impl Default for EccSection8Synd12 {
        #[inline(always)]
        fn default() -> EccSection8Synd12 {
            EccSection8Synd12(0)
        }
    }
    impl core::fmt::Debug for EccSection8Synd12 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("EccSection8Synd12")
                .field("s1", &self.s1())
                .field("s2", &self.s2())
                .finish()
        }
    }
    #[doc = "desc ECC_SECTION8_SYND34."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct EccSection8Synd34(pub u32);
    impl EccSection8Synd34 {
        #[doc = "desc S3."]
        #[must_use]
        #[inline(always)]
        pub const fn s3(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x1fff;
            val as u16
        }
        #[doc = "desc S3."]
        #[inline(always)]
        pub const fn set_s3(&mut self, val: u16) {
            self.0 = (self.0 & !(0x1fff << 0usize)) | (((val as u32) & 0x1fff) << 0usize);
        }
        #[doc = "desc S4."]
        #[must_use]
        #[inline(always)]
        pub const fn s4(&self) -> u16 {
            let val = (self.0 >> 16usize) & 0x1fff;
            val as u16
        }
        #[doc = "desc S4."]
        #[inline(always)]
        pub const fn set_s4(&mut self, val: u16) {
            self.0 = (self.0 & !(0x1fff << 16usize)) | (((val as u32) & 0x1fff) << 16usize);
        }
    }
    impl Default for EccSection8Synd34 {
        #[inline(always)]
        fn default() -> EccSection8Synd34 {
            EccSection8Synd34(0)
        }
    }
    impl core::fmt::Debug for EccSection8Synd34 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("EccSection8Synd34")
                .field("s3", &self.s3())
                .field("s4", &self.s4())
                .finish()
        }
    }
    #[doc = "desc ECC_SECTION8_SYND56."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct EccSection8Synd56(pub u32);
    impl EccSection8Synd56 {
        #[doc = "desc S5."]
        #[must_use]
        #[inline(always)]
        pub const fn s5(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x1fff;
            val as u16
        }
        #[doc = "desc S5."]
        #[inline(always)]
        pub const fn set_s5(&mut self, val: u16) {
            self.0 = (self.0 & !(0x1fff << 0usize)) | (((val as u32) & 0x1fff) << 0usize);
        }
        #[doc = "desc S6."]
        #[must_use]
        #[inline(always)]
        pub const fn s6(&self) -> u16 {
            let val = (self.0 >> 16usize) & 0x1fff;
            val as u16
        }
        #[doc = "desc S6."]
        #[inline(always)]
        pub const fn set_s6(&mut self, val: u16) {
            self.0 = (self.0 & !(0x1fff << 16usize)) | (((val as u32) & 0x1fff) << 16usize);
        }
    }
    impl Default for EccSection8Synd56 {
        #[inline(always)]
        fn default() -> EccSection8Synd56 {
            EccSection8Synd56(0)
        }
    }
    impl core::fmt::Debug for EccSection8Synd56 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("EccSection8Synd56")
                .field("s5", &self.s5())
                .field("s6", &self.s6())
                .finish()
        }
    }
    #[doc = "desc ECC_SECTION8_SYND78."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct EccSection8Synd78(pub u32);
    impl EccSection8Synd78 {
        #[doc = "desc S7."]
        #[must_use]
        #[inline(always)]
        pub const fn s7(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x1fff;
            val as u16
        }
        #[doc = "desc S7."]
        #[inline(always)]
        pub const fn set_s7(&mut self, val: u16) {
            self.0 = (self.0 & !(0x1fff << 0usize)) | (((val as u32) & 0x1fff) << 0usize);
        }
        #[doc = "desc S8."]
        #[must_use]
        #[inline(always)]
        pub const fn s8(&self) -> u16 {
            let val = (self.0 >> 16usize) & 0x1fff;
            val as u16
        }
        #[doc = "desc S8."]
        #[inline(always)]
        pub const fn set_s8(&mut self, val: u16) {
            self.0 = (self.0 & !(0x1fff << 16usize)) | (((val as u32) & 0x1fff) << 16usize);
        }
    }
    impl Default for EccSection8Synd78 {
        #[inline(always)]
        fn default() -> EccSection8Synd78 {
            EccSection8Synd78(0)
        }
    }
    impl core::fmt::Debug for EccSection8Synd78 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("EccSection8Synd78")
                .field("s7", &self.s7())
                .field("s8", &self.s8())
                .finish()
        }
    }
    #[doc = "desc ECC_SECTION9_SYND12."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct EccSection9Synd12(pub u32);
    impl EccSection9Synd12 {
        #[doc = "desc S1."]
        #[must_use]
        #[inline(always)]
        pub const fn s1(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x1fff;
            val as u16
        }
        #[doc = "desc S1."]
        #[inline(always)]
        pub const fn set_s1(&mut self, val: u16) {
            self.0 = (self.0 & !(0x1fff << 0usize)) | (((val as u32) & 0x1fff) << 0usize);
        }
        #[doc = "desc S2."]
        #[must_use]
        #[inline(always)]
        pub const fn s2(&self) -> u16 {
            let val = (self.0 >> 16usize) & 0x1fff;
            val as u16
        }
        #[doc = "desc S2."]
        #[inline(always)]
        pub const fn set_s2(&mut self, val: u16) {
            self.0 = (self.0 & !(0x1fff << 16usize)) | (((val as u32) & 0x1fff) << 16usize);
        }
    }
    impl Default for EccSection9Synd12 {
        #[inline(always)]
        fn default() -> EccSection9Synd12 {
            EccSection9Synd12(0)
        }
    }
    impl core::fmt::Debug for EccSection9Synd12 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("EccSection9Synd12")
                .field("s1", &self.s1())
                .field("s2", &self.s2())
                .finish()
        }
    }
    #[doc = "desc ECC_SECTION9_SYND34."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct EccSection9Synd34(pub u32);
    impl EccSection9Synd34 {
        #[doc = "desc S3."]
        #[must_use]
        #[inline(always)]
        pub const fn s3(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x1fff;
            val as u16
        }
        #[doc = "desc S3."]
        #[inline(always)]
        pub const fn set_s3(&mut self, val: u16) {
            self.0 = (self.0 & !(0x1fff << 0usize)) | (((val as u32) & 0x1fff) << 0usize);
        }
        #[doc = "desc S4."]
        #[must_use]
        #[inline(always)]
        pub const fn s4(&self) -> u16 {
            let val = (self.0 >> 16usize) & 0x1fff;
            val as u16
        }
        #[doc = "desc S4."]
        #[inline(always)]
        pub const fn set_s4(&mut self, val: u16) {
            self.0 = (self.0 & !(0x1fff << 16usize)) | (((val as u32) & 0x1fff) << 16usize);
        }
    }
    impl Default for EccSection9Synd34 {
        #[inline(always)]
        fn default() -> EccSection9Synd34 {
            EccSection9Synd34(0)
        }
    }
    impl core::fmt::Debug for EccSection9Synd34 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("EccSection9Synd34")
                .field("s3", &self.s3())
                .field("s4", &self.s4())
                .finish()
        }
    }
    #[doc = "desc ECC_SECTION9_SYND56."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct EccSection9Synd56(pub u32);
    impl EccSection9Synd56 {
        #[doc = "desc S5."]
        #[must_use]
        #[inline(always)]
        pub const fn s5(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x1fff;
            val as u16
        }
        #[doc = "desc S5."]
        #[inline(always)]
        pub const fn set_s5(&mut self, val: u16) {
            self.0 = (self.0 & !(0x1fff << 0usize)) | (((val as u32) & 0x1fff) << 0usize);
        }
        #[doc = "desc S6."]
        #[must_use]
        #[inline(always)]
        pub const fn s6(&self) -> u16 {
            let val = (self.0 >> 16usize) & 0x1fff;
            val as u16
        }
        #[doc = "desc S6."]
        #[inline(always)]
        pub const fn set_s6(&mut self, val: u16) {
            self.0 = (self.0 & !(0x1fff << 16usize)) | (((val as u32) & 0x1fff) << 16usize);
        }
    }
    impl Default for EccSection9Synd56 {
        #[inline(always)]
        fn default() -> EccSection9Synd56 {
            EccSection9Synd56(0)
        }
    }
    impl core::fmt::Debug for EccSection9Synd56 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("EccSection9Synd56")
                .field("s5", &self.s5())
                .field("s6", &self.s6())
                .finish()
        }
    }
    #[doc = "desc ECC_SECTION9_SYND78."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct EccSection9Synd78(pub u32);
    impl EccSection9Synd78 {
        #[doc = "desc S7."]
        #[must_use]
        #[inline(always)]
        pub const fn s7(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x1fff;
            val as u16
        }
        #[doc = "desc S7."]
        #[inline(always)]
        pub const fn set_s7(&mut self, val: u16) {
            self.0 = (self.0 & !(0x1fff << 0usize)) | (((val as u32) & 0x1fff) << 0usize);
        }
        #[doc = "desc S8."]
        #[must_use]
        #[inline(always)]
        pub const fn s8(&self) -> u16 {
            let val = (self.0 >> 16usize) & 0x1fff;
            val as u16
        }
        #[doc = "desc S8."]
        #[inline(always)]
        pub const fn set_s8(&mut self, val: u16) {
            self.0 = (self.0 & !(0x1fff << 16usize)) | (((val as u32) & 0x1fff) << 16usize);
        }
    }
    impl Default for EccSection9Synd78 {
        #[inline(always)]
        fn default() -> EccSection9Synd78 {
            EccSection9Synd78(0)
        }
    }
    impl core::fmt::Debug for EccSection9Synd78 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("EccSection9Synd78")
                .field("s7", &self.s7())
                .field("s8", &self.s8())
                .finish()
        }
    }
    #[doc = "desc ECC_STAT."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct EccStat(pub u32);
    impl EccStat {
        #[doc = "desc ERRSEC."]
        #[must_use]
        #[inline(always)]
        pub const fn errsec(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[doc = "desc ERRSEC."]
        #[inline(always)]
        pub const fn set_errsec(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
        }
    }
    impl Default for EccStat {
        #[inline(always)]
        fn default() -> EccStat {
            EccStat(0)
        }
    }
    impl core::fmt::Debug for EccStat {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("EccStat")
                .field("errsec", &self.errsec())
                .finish()
        }
    }
    #[doc = "desc ECCR0."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Eccr0(pub u32);
    impl Eccr0 {
        #[doc = "desc ERRLOC."]
        #[must_use]
        #[inline(always)]
        pub const fn errloc(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x0fff;
            val as u16
        }
        #[doc = "desc ERRLOC."]
        #[inline(always)]
        pub const fn set_errloc(&mut self, val: u16) {
            self.0 = (self.0 & !(0x0fff << 0usize)) | (((val as u32) & 0x0fff) << 0usize);
        }
        #[doc = "desc SE."]
        #[must_use]
        #[inline(always)]
        pub const fn se(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "desc SE."]
        #[inline(always)]
        pub const fn set_se(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "desc ME."]
        #[must_use]
        #[inline(always)]
        pub const fn me(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[doc = "desc ME."]
        #[inline(always)]
        pub const fn set_me(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
    }
    impl Default for Eccr0 {
        #[inline(always)]
        fn default() -> Eccr0 {
            Eccr0(0)
        }
    }
    impl core::fmt::Debug for Eccr0 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Eccr0")
                .field("errloc", &self.errloc())
                .field("se", &self.se())
                .field("me", &self.me())
                .finish()
        }
    }
    #[doc = "desc ECCR1."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Eccr1(pub u32);
    impl Eccr1 {
        #[doc = "desc ERRLOC."]
        #[must_use]
        #[inline(always)]
        pub const fn errloc(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x0fff;
            val as u16
        }
        #[doc = "desc ERRLOC."]
        #[inline(always)]
        pub const fn set_errloc(&mut self, val: u16) {
            self.0 = (self.0 & !(0x0fff << 0usize)) | (((val as u32) & 0x0fff) << 0usize);
        }
        #[doc = "desc SE."]
        #[must_use]
        #[inline(always)]
        pub const fn se(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "desc SE."]
        #[inline(always)]
        pub const fn set_se(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "desc ME."]
        #[must_use]
        #[inline(always)]
        pub const fn me(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[doc = "desc ME."]
        #[inline(always)]
        pub const fn set_me(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
    }
    impl Default for Eccr1 {
        #[inline(always)]
        fn default() -> Eccr1 {
            Eccr1(0)
        }
    }
    impl core::fmt::Debug for Eccr1 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Eccr1")
                .field("errloc", &self.errloc())
                .field("se", &self.se())
                .field("me", &self.me())
                .finish()
        }
    }
    #[doc = "desc ECCR10."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Eccr10(pub u32);
    impl Eccr10 {
        #[doc = "desc ERRLOC."]
        #[must_use]
        #[inline(always)]
        pub const fn errloc(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x0fff;
            val as u16
        }
        #[doc = "desc ERRLOC."]
        #[inline(always)]
        pub const fn set_errloc(&mut self, val: u16) {
            self.0 = (self.0 & !(0x0fff << 0usize)) | (((val as u32) & 0x0fff) << 0usize);
        }
        #[doc = "desc SE."]
        #[must_use]
        #[inline(always)]
        pub const fn se(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "desc SE."]
        #[inline(always)]
        pub const fn set_se(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "desc ME."]
        #[must_use]
        #[inline(always)]
        pub const fn me(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[doc = "desc ME."]
        #[inline(always)]
        pub const fn set_me(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
    }
    impl Default for Eccr10 {
        #[inline(always)]
        fn default() -> Eccr10 {
            Eccr10(0)
        }
    }
    impl core::fmt::Debug for Eccr10 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Eccr10")
                .field("errloc", &self.errloc())
                .field("se", &self.se())
                .field("me", &self.me())
                .finish()
        }
    }
    #[doc = "desc ECCR11."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Eccr11(pub u32);
    impl Eccr11 {
        #[doc = "desc ERRLOC."]
        #[must_use]
        #[inline(always)]
        pub const fn errloc(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x0fff;
            val as u16
        }
        #[doc = "desc ERRLOC."]
        #[inline(always)]
        pub const fn set_errloc(&mut self, val: u16) {
            self.0 = (self.0 & !(0x0fff << 0usize)) | (((val as u32) & 0x0fff) << 0usize);
        }
        #[doc = "desc SE."]
        #[must_use]
        #[inline(always)]
        pub const fn se(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "desc SE."]
        #[inline(always)]
        pub const fn set_se(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "desc ME."]
        #[must_use]
        #[inline(always)]
        pub const fn me(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[doc = "desc ME."]
        #[inline(always)]
        pub const fn set_me(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
    }
    impl Default for Eccr11 {
        #[inline(always)]
        fn default() -> Eccr11 {
            Eccr11(0)
        }
    }
    impl core::fmt::Debug for Eccr11 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Eccr11")
                .field("errloc", &self.errloc())
                .field("se", &self.se())
                .field("me", &self.me())
                .finish()
        }
    }
    #[doc = "desc ECCR12."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Eccr12(pub u32);
    impl Eccr12 {
        #[doc = "desc ERRLOC."]
        #[must_use]
        #[inline(always)]
        pub const fn errloc(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x0fff;
            val as u16
        }
        #[doc = "desc ERRLOC."]
        #[inline(always)]
        pub const fn set_errloc(&mut self, val: u16) {
            self.0 = (self.0 & !(0x0fff << 0usize)) | (((val as u32) & 0x0fff) << 0usize);
        }
        #[doc = "desc SE."]
        #[must_use]
        #[inline(always)]
        pub const fn se(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "desc SE."]
        #[inline(always)]
        pub const fn set_se(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "desc ME."]
        #[must_use]
        #[inline(always)]
        pub const fn me(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[doc = "desc ME."]
        #[inline(always)]
        pub const fn set_me(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
    }
    impl Default for Eccr12 {
        #[inline(always)]
        fn default() -> Eccr12 {
            Eccr12(0)
        }
    }
    impl core::fmt::Debug for Eccr12 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Eccr12")
                .field("errloc", &self.errloc())
                .field("se", &self.se())
                .field("me", &self.me())
                .finish()
        }
    }
    #[doc = "desc ECCR13."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Eccr13(pub u32);
    impl Eccr13 {
        #[doc = "desc ERRLOC."]
        #[must_use]
        #[inline(always)]
        pub const fn errloc(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x0fff;
            val as u16
        }
        #[doc = "desc ERRLOC."]
        #[inline(always)]
        pub const fn set_errloc(&mut self, val: u16) {
            self.0 = (self.0 & !(0x0fff << 0usize)) | (((val as u32) & 0x0fff) << 0usize);
        }
        #[doc = "desc SE."]
        #[must_use]
        #[inline(always)]
        pub const fn se(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "desc SE."]
        #[inline(always)]
        pub const fn set_se(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "desc ME."]
        #[must_use]
        #[inline(always)]
        pub const fn me(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[doc = "desc ME."]
        #[inline(always)]
        pub const fn set_me(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
    }
    impl Default for Eccr13 {
        #[inline(always)]
        fn default() -> Eccr13 {
            Eccr13(0)
        }
    }
    impl core::fmt::Debug for Eccr13 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Eccr13")
                .field("errloc", &self.errloc())
                .field("se", &self.se())
                .field("me", &self.me())
                .finish()
        }
    }
    #[doc = "desc ECCR14."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Eccr14(pub u32);
    impl Eccr14 {
        #[doc = "desc ERRLOC."]
        #[must_use]
        #[inline(always)]
        pub const fn errloc(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x0fff;
            val as u16
        }
        #[doc = "desc ERRLOC."]
        #[inline(always)]
        pub const fn set_errloc(&mut self, val: u16) {
            self.0 = (self.0 & !(0x0fff << 0usize)) | (((val as u32) & 0x0fff) << 0usize);
        }
        #[doc = "desc SE."]
        #[must_use]
        #[inline(always)]
        pub const fn se(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "desc SE."]
        #[inline(always)]
        pub const fn set_se(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "desc ME."]
        #[must_use]
        #[inline(always)]
        pub const fn me(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[doc = "desc ME."]
        #[inline(always)]
        pub const fn set_me(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
    }
    impl Default for Eccr14 {
        #[inline(always)]
        fn default() -> Eccr14 {
            Eccr14(0)
        }
    }
    impl core::fmt::Debug for Eccr14 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Eccr14")
                .field("errloc", &self.errloc())
                .field("se", &self.se())
                .field("me", &self.me())
                .finish()
        }
    }
    #[doc = "desc ECCR15."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Eccr15(pub u32);
    impl Eccr15 {
        #[doc = "desc ERRLOC."]
        #[must_use]
        #[inline(always)]
        pub const fn errloc(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x0fff;
            val as u16
        }
        #[doc = "desc ERRLOC."]
        #[inline(always)]
        pub const fn set_errloc(&mut self, val: u16) {
            self.0 = (self.0 & !(0x0fff << 0usize)) | (((val as u32) & 0x0fff) << 0usize);
        }
        #[doc = "desc SE."]
        #[must_use]
        #[inline(always)]
        pub const fn se(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "desc SE."]
        #[inline(always)]
        pub const fn set_se(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "desc ME."]
        #[must_use]
        #[inline(always)]
        pub const fn me(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[doc = "desc ME."]
        #[inline(always)]
        pub const fn set_me(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
    }
    impl Default for Eccr15 {
        #[inline(always)]
        fn default() -> Eccr15 {
            Eccr15(0)
        }
    }
    impl core::fmt::Debug for Eccr15 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Eccr15")
                .field("errloc", &self.errloc())
                .field("se", &self.se())
                .field("me", &self.me())
                .finish()
        }
    }
    #[doc = "desc ECCR2."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Eccr2(pub u32);
    impl Eccr2 {
        #[doc = "desc ERRLOC."]
        #[must_use]
        #[inline(always)]
        pub const fn errloc(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x0fff;
            val as u16
        }
        #[doc = "desc ERRLOC."]
        #[inline(always)]
        pub const fn set_errloc(&mut self, val: u16) {
            self.0 = (self.0 & !(0x0fff << 0usize)) | (((val as u32) & 0x0fff) << 0usize);
        }
        #[doc = "desc SE."]
        #[must_use]
        #[inline(always)]
        pub const fn se(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "desc SE."]
        #[inline(always)]
        pub const fn set_se(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "desc ME."]
        #[must_use]
        #[inline(always)]
        pub const fn me(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[doc = "desc ME."]
        #[inline(always)]
        pub const fn set_me(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
    }
    impl Default for Eccr2 {
        #[inline(always)]
        fn default() -> Eccr2 {
            Eccr2(0)
        }
    }
    impl core::fmt::Debug for Eccr2 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Eccr2")
                .field("errloc", &self.errloc())
                .field("se", &self.se())
                .field("me", &self.me())
                .finish()
        }
    }
    #[doc = "desc ECCR3."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Eccr3(pub u32);
    impl Eccr3 {
        #[doc = "desc ERRLOC."]
        #[must_use]
        #[inline(always)]
        pub const fn errloc(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x0fff;
            val as u16
        }
        #[doc = "desc ERRLOC."]
        #[inline(always)]
        pub const fn set_errloc(&mut self, val: u16) {
            self.0 = (self.0 & !(0x0fff << 0usize)) | (((val as u32) & 0x0fff) << 0usize);
        }
        #[doc = "desc SE."]
        #[must_use]
        #[inline(always)]
        pub const fn se(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "desc SE."]
        #[inline(always)]
        pub const fn set_se(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "desc ME."]
        #[must_use]
        #[inline(always)]
        pub const fn me(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[doc = "desc ME."]
        #[inline(always)]
        pub const fn set_me(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
    }
    impl Default for Eccr3 {
        #[inline(always)]
        fn default() -> Eccr3 {
            Eccr3(0)
        }
    }
    impl core::fmt::Debug for Eccr3 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Eccr3")
                .field("errloc", &self.errloc())
                .field("se", &self.se())
                .field("me", &self.me())
                .finish()
        }
    }
    #[doc = "desc ECCR4."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Eccr4(pub u32);
    impl Eccr4 {
        #[doc = "desc ERRLOC."]
        #[must_use]
        #[inline(always)]
        pub const fn errloc(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x0fff;
            val as u16
        }
        #[doc = "desc ERRLOC."]
        #[inline(always)]
        pub const fn set_errloc(&mut self, val: u16) {
            self.0 = (self.0 & !(0x0fff << 0usize)) | (((val as u32) & 0x0fff) << 0usize);
        }
        #[doc = "desc SE."]
        #[must_use]
        #[inline(always)]
        pub const fn se(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "desc SE."]
        #[inline(always)]
        pub const fn set_se(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "desc ME."]
        #[must_use]
        #[inline(always)]
        pub const fn me(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[doc = "desc ME."]
        #[inline(always)]
        pub const fn set_me(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
    }
    impl Default for Eccr4 {
        #[inline(always)]
        fn default() -> Eccr4 {
            Eccr4(0)
        }
    }
    impl core::fmt::Debug for Eccr4 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Eccr4")
                .field("errloc", &self.errloc())
                .field("se", &self.se())
                .field("me", &self.me())
                .finish()
        }
    }
    #[doc = "desc ECCR5."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Eccr5(pub u32);
    impl Eccr5 {
        #[doc = "desc ERRLOC."]
        #[must_use]
        #[inline(always)]
        pub const fn errloc(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x0fff;
            val as u16
        }
        #[doc = "desc ERRLOC."]
        #[inline(always)]
        pub const fn set_errloc(&mut self, val: u16) {
            self.0 = (self.0 & !(0x0fff << 0usize)) | (((val as u32) & 0x0fff) << 0usize);
        }
        #[doc = "desc SE."]
        #[must_use]
        #[inline(always)]
        pub const fn se(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "desc SE."]
        #[inline(always)]
        pub const fn set_se(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "desc ME."]
        #[must_use]
        #[inline(always)]
        pub const fn me(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[doc = "desc ME."]
        #[inline(always)]
        pub const fn set_me(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
    }
    impl Default for Eccr5 {
        #[inline(always)]
        fn default() -> Eccr5 {
            Eccr5(0)
        }
    }
    impl core::fmt::Debug for Eccr5 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Eccr5")
                .field("errloc", &self.errloc())
                .field("se", &self.se())
                .field("me", &self.me())
                .finish()
        }
    }
    #[doc = "desc ECCR6."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Eccr6(pub u32);
    impl Eccr6 {
        #[doc = "desc ERRLOC."]
        #[must_use]
        #[inline(always)]
        pub const fn errloc(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x0fff;
            val as u16
        }
        #[doc = "desc ERRLOC."]
        #[inline(always)]
        pub const fn set_errloc(&mut self, val: u16) {
            self.0 = (self.0 & !(0x0fff << 0usize)) | (((val as u32) & 0x0fff) << 0usize);
        }
        #[doc = "desc SE."]
        #[must_use]
        #[inline(always)]
        pub const fn se(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "desc SE."]
        #[inline(always)]
        pub const fn set_se(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "desc ME."]
        #[must_use]
        #[inline(always)]
        pub const fn me(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[doc = "desc ME."]
        #[inline(always)]
        pub const fn set_me(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
    }
    impl Default for Eccr6 {
        #[inline(always)]
        fn default() -> Eccr6 {
            Eccr6(0)
        }
    }
    impl core::fmt::Debug for Eccr6 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Eccr6")
                .field("errloc", &self.errloc())
                .field("se", &self.se())
                .field("me", &self.me())
                .finish()
        }
    }
    #[doc = "desc ECCR7."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Eccr7(pub u32);
    impl Eccr7 {
        #[doc = "desc ERRLOC."]
        #[must_use]
        #[inline(always)]
        pub const fn errloc(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x0fff;
            val as u16
        }
        #[doc = "desc ERRLOC."]
        #[inline(always)]
        pub const fn set_errloc(&mut self, val: u16) {
            self.0 = (self.0 & !(0x0fff << 0usize)) | (((val as u32) & 0x0fff) << 0usize);
        }
        #[doc = "desc SE."]
        #[must_use]
        #[inline(always)]
        pub const fn se(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "desc SE."]
        #[inline(always)]
        pub const fn set_se(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "desc ME."]
        #[must_use]
        #[inline(always)]
        pub const fn me(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[doc = "desc ME."]
        #[inline(always)]
        pub const fn set_me(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
    }
    impl Default for Eccr7 {
        #[inline(always)]
        fn default() -> Eccr7 {
            Eccr7(0)
        }
    }
    impl core::fmt::Debug for Eccr7 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Eccr7")
                .field("errloc", &self.errloc())
                .field("se", &self.se())
                .field("me", &self.me())
                .finish()
        }
    }
    #[doc = "desc ECCR8."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Eccr8(pub u32);
    impl Eccr8 {
        #[doc = "desc ERRLOC."]
        #[must_use]
        #[inline(always)]
        pub const fn errloc(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x0fff;
            val as u16
        }
        #[doc = "desc ERRLOC."]
        #[inline(always)]
        pub const fn set_errloc(&mut self, val: u16) {
            self.0 = (self.0 & !(0x0fff << 0usize)) | (((val as u32) & 0x0fff) << 0usize);
        }
        #[doc = "desc SE."]
        #[must_use]
        #[inline(always)]
        pub const fn se(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "desc SE."]
        #[inline(always)]
        pub const fn set_se(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "desc ME."]
        #[must_use]
        #[inline(always)]
        pub const fn me(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[doc = "desc ME."]
        #[inline(always)]
        pub const fn set_me(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
    }
    impl Default for Eccr8 {
        #[inline(always)]
        fn default() -> Eccr8 {
            Eccr8(0)
        }
    }
    impl core::fmt::Debug for Eccr8 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Eccr8")
                .field("errloc", &self.errloc())
                .field("se", &self.se())
                .field("me", &self.me())
                .finish()
        }
    }
    #[doc = "desc ECCR9."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Eccr9(pub u32);
    impl Eccr9 {
        #[doc = "desc ERRLOC."]
        #[must_use]
        #[inline(always)]
        pub const fn errloc(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x0fff;
            val as u16
        }
        #[doc = "desc ERRLOC."]
        #[inline(always)]
        pub const fn set_errloc(&mut self, val: u16) {
            self.0 = (self.0 & !(0x0fff << 0usize)) | (((val as u32) & 0x0fff) << 0usize);
        }
        #[doc = "desc SE."]
        #[must_use]
        #[inline(always)]
        pub const fn se(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "desc SE."]
        #[inline(always)]
        pub const fn set_se(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "desc ME."]
        #[must_use]
        #[inline(always)]
        pub const fn me(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[doc = "desc ME."]
        #[inline(always)]
        pub const fn set_me(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
    }
    impl Default for Eccr9 {
        #[inline(always)]
        fn default() -> Eccr9 {
            Eccr9(0)
        }
    }
    impl core::fmt::Debug for Eccr9 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Eccr9")
                .field("errloc", &self.errloc())
                .field("se", &self.se())
                .field("me", &self.me())
                .finish()
        }
    }
    #[doc = "desc IDXR1."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Idxr1(pub u32);
    impl Idxr1 {
        #[doc = "desc IDX1."]
        #[must_use]
        #[inline(always)]
        pub const fn idx1(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[doc = "desc IDX1."]
        #[inline(always)]
        pub const fn set_idx1(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
        }
    }
    impl Default for Idxr1 {
        #[inline(always)]
        fn default() -> Idxr1 {
            Idxr1(0)
        }
    }
    impl core::fmt::Debug for Idxr1 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Idxr1").field("idx1", &self.idx1()).finish()
        }
    }
    #[doc = "desc IENR."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ienr(pub u32);
    impl Ienr {
        #[doc = "desc ECCEUEN."]
        #[must_use]
        #[inline(always)]
        pub const fn ecceuen(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "desc ECCEUEN."]
        #[inline(always)]
        pub const fn set_ecceuen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "desc ECCECEN."]
        #[must_use]
        #[inline(always)]
        pub const fn eccecen(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "desc ECCECEN."]
        #[inline(always)]
        pub const fn set_eccecen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "desc ECCCEN."]
        #[must_use]
        #[inline(always)]
        pub const fn ecccen(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "desc ECCCEN."]
        #[inline(always)]
        pub const fn set_ecccen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "desc ECCEEN."]
        #[must_use]
        #[inline(always)]
        pub const fn ecceen(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "desc ECCEEN."]
        #[inline(always)]
        pub const fn set_ecceen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "desc ECCDIS."]
        #[must_use]
        #[inline(always)]
        pub const fn eccdis(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "desc ECCDIS."]
        #[inline(always)]
        pub const fn set_eccdis(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "desc RBEN."]
        #[must_use]
        #[inline(always)]
        pub const fn rben(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0xff;
            val as u8
        }
        #[doc = "desc RBEN."]
        #[inline(always)]
        pub const fn set_rben(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
        }
    }
    impl Default for Ienr {
        #[inline(always)]
        fn default() -> Ienr {
            Ienr(0)
        }
    }
    impl core::fmt::Debug for Ienr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Ienr")
                .field("ecceuen", &self.ecceuen())
                .field("eccecen", &self.eccecen())
                .field("ecccen", &self.ecccen())
                .field("ecceen", &self.ecceen())
                .field("eccdis", &self.eccdis())
                .field("rben", &self.rben())
                .finish()
        }
    }
    #[doc = "desc IRSR."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Irsr(pub u32);
    impl Irsr {
        #[doc = "desc ECCEURS."]
        #[must_use]
        #[inline(always)]
        pub const fn ecceurs(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "desc ECCEURS."]
        #[inline(always)]
        pub const fn set_ecceurs(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "desc ECCECRS."]
        #[must_use]
        #[inline(always)]
        pub const fn eccecrs(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "desc ECCECRS."]
        #[inline(always)]
        pub const fn set_eccecrs(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "desc ECCCRS."]
        #[must_use]
        #[inline(always)]
        pub const fn ecccrs(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "desc ECCCRS."]
        #[inline(always)]
        pub const fn set_ecccrs(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "desc ECCERS."]
        #[must_use]
        #[inline(always)]
        pub const fn eccers(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "desc ECCERS."]
        #[inline(always)]
        pub const fn set_eccers(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "desc RBRS."]
        #[must_use]
        #[inline(always)]
        pub const fn rbrs(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0xff;
            val as u8
        }
        #[doc = "desc RBRS."]
        #[inline(always)]
        pub const fn set_rbrs(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
        }
    }
    impl Default for Irsr {
        #[inline(always)]
        fn default() -> Irsr {
            Irsr(0)
        }
    }
    impl core::fmt::Debug for Irsr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Irsr")
                .field("ecceurs", &self.ecceurs())
                .field("eccecrs", &self.eccecrs())
                .field("ecccrs", &self.ecccrs())
                .field("eccers", &self.eccers())
                .field("rbrs", &self.rbrs())
                .finish()
        }
    }
    #[doc = "desc ISTR."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Istr(pub u32);
    impl Istr {
        #[doc = "desc ECCEUST."]
        #[must_use]
        #[inline(always)]
        pub const fn ecceust(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "desc ECCEUST."]
        #[inline(always)]
        pub const fn set_ecceust(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "desc ECCECST."]
        #[must_use]
        #[inline(always)]
        pub const fn eccecst(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "desc ECCECST."]
        #[inline(always)]
        pub const fn set_eccecst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "desc ECCCST."]
        #[must_use]
        #[inline(always)]
        pub const fn ecccst(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "desc ECCCST."]
        #[inline(always)]
        pub const fn set_ecccst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "desc ECCEST."]
        #[must_use]
        #[inline(always)]
        pub const fn eccest(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "desc ECCEST."]
        #[inline(always)]
        pub const fn set_eccest(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "desc RBST."]
        #[must_use]
        #[inline(always)]
        pub const fn rbst(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0xff;
            val as u8
        }
        #[doc = "desc RBST."]
        #[inline(always)]
        pub const fn set_rbst(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
        }
    }
    impl Default for Istr {
        #[inline(always)]
        fn default() -> Istr {
            Istr(0)
        }
    }
    impl core::fmt::Debug for Istr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Istr")
                .field("ecceust", &self.ecceust())
                .field("eccecst", &self.eccecst())
                .field("ecccst", &self.ecccst())
                .field("eccest", &self.eccest())
                .field("rbst", &self.rbst())
                .finish()
        }
    }
    #[doc = "desc TMCR0."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Tmcr0(pub u32);
    impl Tmcr0 {
        #[doc = "desc TS."]
        #[must_use]
        #[inline(always)]
        pub const fn ts(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[doc = "desc TS."]
        #[inline(always)]
        pub const fn set_ts(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
        }
        #[doc = "desc TWP."]
        #[must_use]
        #[inline(always)]
        pub const fn twp(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0xff;
            val as u8
        }
        #[doc = "desc TWP."]
        #[inline(always)]
        pub const fn set_twp(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
        }
        #[doc = "desc TRP."]
        #[must_use]
        #[inline(always)]
        pub const fn trp(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0xff;
            val as u8
        }
        #[doc = "desc TRP."]
        #[inline(always)]
        pub const fn set_trp(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 16usize)) | (((val as u32) & 0xff) << 16usize);
        }
        #[doc = "desc TH."]
        #[must_use]
        #[inline(always)]
        pub const fn th(&self) -> u8 {
            let val = (self.0 >> 24usize) & 0xff;
            val as u8
        }
        #[doc = "desc TH."]
        #[inline(always)]
        pub const fn set_th(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 24usize)) | (((val as u32) & 0xff) << 24usize);
        }
    }
    impl Default for Tmcr0 {
        #[inline(always)]
        fn default() -> Tmcr0 {
            Tmcr0(0)
        }
    }
    impl core::fmt::Debug for Tmcr0 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Tmcr0")
                .field("ts", &self.ts())
                .field("twp", &self.twp())
                .field("trp", &self.trp())
                .field("th", &self.th())
                .finish()
        }
    }
    #[doc = "desc TMCR1."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Tmcr1(pub u32);
    impl Tmcr1 {
        #[doc = "desc TWH."]
        #[must_use]
        #[inline(always)]
        pub const fn twh(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[doc = "desc TWH."]
        #[inline(always)]
        pub const fn set_twh(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
        }
        #[doc = "desc TRH."]
        #[must_use]
        #[inline(always)]
        pub const fn trh(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0xff;
            val as u8
        }
        #[doc = "desc TRH."]
        #[inline(always)]
        pub const fn set_trh(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
        }
        #[doc = "desc TRR."]
        #[must_use]
        #[inline(always)]
        pub const fn trr(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0xff;
            val as u8
        }
        #[doc = "desc TRR."]
        #[inline(always)]
        pub const fn set_trr(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 16usize)) | (((val as u32) & 0xff) << 16usize);
        }
        #[doc = "desc TWB."]
        #[must_use]
        #[inline(always)]
        pub const fn twb(&self) -> u8 {
            let val = (self.0 >> 24usize) & 0xff;
            val as u8
        }
        #[doc = "desc TWB."]
        #[inline(always)]
        pub const fn set_twb(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 24usize)) | (((val as u32) & 0xff) << 24usize);
        }
    }
    impl Default for Tmcr1 {
        #[inline(always)]
        fn default() -> Tmcr1 {
            Tmcr1(0)
        }
    }
    impl core::fmt::Debug for Tmcr1 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Tmcr1")
                .field("twh", &self.twh())
                .field("trh", &self.trh())
                .field("trr", &self.trr())
                .field("twb", &self.twb())
                .finish()
        }
    }
    #[doc = "desc TMCR2."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Tmcr2(pub u32);
    impl Tmcr2 {
        #[doc = "desc TCCS."]
        #[must_use]
        #[inline(always)]
        pub const fn tccs(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[doc = "desc TCCS."]
        #[inline(always)]
        pub const fn set_tccs(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
        }
        #[doc = "desc TWTR."]
        #[must_use]
        #[inline(always)]
        pub const fn twtr(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0xff;
            val as u8
        }
        #[doc = "desc TWTR."]
        #[inline(always)]
        pub const fn set_twtr(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
        }
        #[doc = "desc TRTW."]
        #[must_use]
        #[inline(always)]
        pub const fn trtw(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0xff;
            val as u8
        }
        #[doc = "desc TRTW."]
        #[inline(always)]
        pub const fn set_trtw(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 16usize)) | (((val as u32) & 0xff) << 16usize);
        }
        #[doc = "desc TADL."]
        #[must_use]
        #[inline(always)]
        pub const fn tadl(&self) -> u8 {
            let val = (self.0 >> 24usize) & 0xff;
            val as u8
        }
        #[doc = "desc TADL."]
        #[inline(always)]
        pub const fn set_tadl(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 24usize)) | (((val as u32) & 0xff) << 24usize);
        }
    }
    impl Default for Tmcr2 {
        #[inline(always)]
        fn default() -> Tmcr2 {
            Tmcr2(0)
        }
    }
    impl core::fmt::Debug for Tmcr2 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Tmcr2")
                .field("tccs", &self.tccs())
                .field("twtr", &self.twtr())
                .field("trtw", &self.trtw())
                .field("tadl", &self.tadl())
                .finish()
        }
    }
}
