#![allow(clippy::missing_safety_doc)]
#![allow(clippy::identity_op)]
#![allow(clippy::unnecessary_cast)]
#![allow(clippy::erasing_op)]

#[doc = "desc INTC."]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Intc {
    ptr: *mut u8,
}
unsafe impl Send for Intc {}
unsafe impl Sync for Intc {}
impl Intc {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "desc NOCCR."]
    #[inline(always)]
    pub const fn noccr(self) -> crate::common::Reg<regs::Noccr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0usize) as _) }
    }
    #[doc = "desc NMIER."]
    #[inline(always)]
    pub const fn nmier(self) -> crate::common::Reg<regs::Nmier, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x04usize) as _) }
    }
    #[doc = "desc NMIFR."]
    #[inline(always)]
    pub const fn nmifr(self) -> crate::common::Reg<regs::Nmifr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x08usize) as _) }
    }
    #[doc = "desc NMIFCR."]
    #[inline(always)]
    pub const fn nmifcr(self) -> crate::common::Reg<regs::Nmifcr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0cusize) as _) }
    }
    #[doc = "desc EIRQCR0."]
    #[inline(always)]
    pub const fn eirqcr0(self) -> crate::common::Reg<regs::Eirqcr0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x10usize) as _) }
    }
    #[doc = "desc EIRQCR1."]
    #[inline(always)]
    pub const fn eirqcr1(self) -> crate::common::Reg<regs::Eirqcr1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x14usize) as _) }
    }
    #[doc = "desc EIRQCR2."]
    #[inline(always)]
    pub const fn eirqcr2(self) -> crate::common::Reg<regs::Eirqcr2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x18usize) as _) }
    }
    #[doc = "desc EIRQCR3."]
    #[inline(always)]
    pub const fn eirqcr3(self) -> crate::common::Reg<regs::Eirqcr3, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1cusize) as _) }
    }
    #[doc = "desc EIRQCR4."]
    #[inline(always)]
    pub const fn eirqcr4(self) -> crate::common::Reg<regs::Eirqcr4, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x20usize) as _) }
    }
    #[doc = "desc EIRQCR5."]
    #[inline(always)]
    pub const fn eirqcr5(self) -> crate::common::Reg<regs::Eirqcr5, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x24usize) as _) }
    }
    #[doc = "desc EIRQCR6."]
    #[inline(always)]
    pub const fn eirqcr6(self) -> crate::common::Reg<regs::Eirqcr6, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x28usize) as _) }
    }
    #[doc = "desc EIRQCR7."]
    #[inline(always)]
    pub const fn eirqcr7(self) -> crate::common::Reg<regs::Eirqcr7, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x2cusize) as _) }
    }
    #[doc = "desc EIRQCR8."]
    #[inline(always)]
    pub const fn eirqcr8(self) -> crate::common::Reg<regs::Eirqcr8, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x30usize) as _) }
    }
    #[doc = "desc EIRQCR9."]
    #[inline(always)]
    pub const fn eirqcr9(self) -> crate::common::Reg<regs::Eirqcr9, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x34usize) as _) }
    }
    #[doc = "desc EIRQCR10."]
    #[inline(always)]
    pub const fn eirqcr10(self) -> crate::common::Reg<regs::Eirqcr10, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x38usize) as _) }
    }
    #[doc = "desc EIRQCR11."]
    #[inline(always)]
    pub const fn eirqcr11(self) -> crate::common::Reg<regs::Eirqcr11, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x3cusize) as _) }
    }
    #[doc = "desc EIRQCR12."]
    #[inline(always)]
    pub const fn eirqcr12(self) -> crate::common::Reg<regs::Eirqcr12, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x40usize) as _) }
    }
    #[doc = "desc EIRQCR13."]
    #[inline(always)]
    pub const fn eirqcr13(self) -> crate::common::Reg<regs::Eirqcr13, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x44usize) as _) }
    }
    #[doc = "desc EIRQCR14."]
    #[inline(always)]
    pub const fn eirqcr14(self) -> crate::common::Reg<regs::Eirqcr14, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x48usize) as _) }
    }
    #[doc = "desc EIRQCR15."]
    #[inline(always)]
    pub const fn eirqcr15(self) -> crate::common::Reg<regs::Eirqcr15, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x4cusize) as _) }
    }
    #[doc = "desc WKEN."]
    #[inline(always)]
    pub const fn wken(self) -> crate::common::Reg<regs::Wken, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x50usize) as _) }
    }
    #[doc = "desc EIFR."]
    #[inline(always)]
    pub const fn eifr(self) -> crate::common::Reg<regs::Eifr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x54usize) as _) }
    }
    #[doc = "desc EIFCR."]
    #[inline(always)]
    pub const fn eifcr(self) -> crate::common::Reg<regs::Eifcr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x58usize) as _) }
    }
    #[doc = "desc INTSEL0."]
    #[inline(always)]
    pub const fn intsel0(self) -> crate::common::Reg<regs::Intsel0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x5cusize) as _) }
    }
    #[doc = "desc INTSEL1."]
    #[inline(always)]
    pub const fn intsel1(self) -> crate::common::Reg<regs::Intsel1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x60usize) as _) }
    }
    #[doc = "desc INTSEL2."]
    #[inline(always)]
    pub const fn intsel2(self) -> crate::common::Reg<regs::Intsel2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x64usize) as _) }
    }
    #[doc = "desc INTSEL3."]
    #[inline(always)]
    pub const fn intsel3(self) -> crate::common::Reg<regs::Intsel3, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x68usize) as _) }
    }
    #[doc = "desc INTSEL4."]
    #[inline(always)]
    pub const fn intsel4(self) -> crate::common::Reg<regs::Intsel4, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x6cusize) as _) }
    }
    #[doc = "desc INTSEL5."]
    #[inline(always)]
    pub const fn intsel5(self) -> crate::common::Reg<regs::Intsel5, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x70usize) as _) }
    }
    #[doc = "desc INTSEL6."]
    #[inline(always)]
    pub const fn intsel6(self) -> crate::common::Reg<regs::Intsel6, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x74usize) as _) }
    }
    #[doc = "desc INTSEL7."]
    #[inline(always)]
    pub const fn intsel7(self) -> crate::common::Reg<regs::Intsel7, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x78usize) as _) }
    }
    #[doc = "desc INTSEL8."]
    #[inline(always)]
    pub const fn intsel8(self) -> crate::common::Reg<regs::Intsel8, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x7cusize) as _) }
    }
    #[doc = "desc INTSEL9."]
    #[inline(always)]
    pub const fn intsel9(self) -> crate::common::Reg<regs::Intsel9, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x80usize) as _) }
    }
    #[doc = "desc INTSEL10."]
    #[inline(always)]
    pub const fn intsel10(self) -> crate::common::Reg<regs::Intsel10, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x84usize) as _) }
    }
    #[doc = "desc INTSEL11."]
    #[inline(always)]
    pub const fn intsel11(self) -> crate::common::Reg<regs::Intsel11, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x88usize) as _) }
    }
    #[doc = "desc INTSEL12."]
    #[inline(always)]
    pub const fn intsel12(self) -> crate::common::Reg<regs::Intsel12, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x8cusize) as _) }
    }
    #[doc = "desc INTSEL13."]
    #[inline(always)]
    pub const fn intsel13(self) -> crate::common::Reg<regs::Intsel13, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x90usize) as _) }
    }
    #[doc = "desc INTSEL14."]
    #[inline(always)]
    pub const fn intsel14(self) -> crate::common::Reg<regs::Intsel14, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x94usize) as _) }
    }
    #[doc = "desc INTSEL15."]
    #[inline(always)]
    pub const fn intsel15(self) -> crate::common::Reg<regs::Intsel15, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x98usize) as _) }
    }
    #[doc = "desc INTSEL16."]
    #[inline(always)]
    pub const fn intsel16(self) -> crate::common::Reg<regs::Intsel16, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x9cusize) as _) }
    }
    #[doc = "desc INTSEL17."]
    #[inline(always)]
    pub const fn intsel17(self) -> crate::common::Reg<regs::Intsel17, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xa0usize) as _) }
    }
    #[doc = "desc INTSEL18."]
    #[inline(always)]
    pub const fn intsel18(self) -> crate::common::Reg<regs::Intsel18, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xa4usize) as _) }
    }
    #[doc = "desc INTSEL19."]
    #[inline(always)]
    pub const fn intsel19(self) -> crate::common::Reg<regs::Intsel19, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xa8usize) as _) }
    }
    #[doc = "desc INTSEL20."]
    #[inline(always)]
    pub const fn intsel20(self) -> crate::common::Reg<regs::Intsel20, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xacusize) as _) }
    }
    #[doc = "desc INTSEL21."]
    #[inline(always)]
    pub const fn intsel21(self) -> crate::common::Reg<regs::Intsel21, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xb0usize) as _) }
    }
    #[doc = "desc INTSEL22."]
    #[inline(always)]
    pub const fn intsel22(self) -> crate::common::Reg<regs::Intsel22, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xb4usize) as _) }
    }
    #[doc = "desc INTSEL23."]
    #[inline(always)]
    pub const fn intsel23(self) -> crate::common::Reg<regs::Intsel23, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xb8usize) as _) }
    }
    #[doc = "desc INTSEL24."]
    #[inline(always)]
    pub const fn intsel24(self) -> crate::common::Reg<regs::Intsel24, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xbcusize) as _) }
    }
    #[doc = "desc INTSEL25."]
    #[inline(always)]
    pub const fn intsel25(self) -> crate::common::Reg<regs::Intsel25, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xc0usize) as _) }
    }
    #[doc = "desc INTSEL26."]
    #[inline(always)]
    pub const fn intsel26(self) -> crate::common::Reg<regs::Intsel26, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xc4usize) as _) }
    }
    #[doc = "desc INTSEL27."]
    #[inline(always)]
    pub const fn intsel27(self) -> crate::common::Reg<regs::Intsel27, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xc8usize) as _) }
    }
    #[doc = "desc INTSEL28."]
    #[inline(always)]
    pub const fn intsel28(self) -> crate::common::Reg<regs::Intsel28, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xccusize) as _) }
    }
    #[doc = "desc INTSEL29."]
    #[inline(always)]
    pub const fn intsel29(self) -> crate::common::Reg<regs::Intsel29, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xd0usize) as _) }
    }
    #[doc = "desc INTSEL30."]
    #[inline(always)]
    pub const fn intsel30(self) -> crate::common::Reg<regs::Intsel30, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xd4usize) as _) }
    }
    #[doc = "desc INTSEL31."]
    #[inline(always)]
    pub const fn intsel31(self) -> crate::common::Reg<regs::Intsel31, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xd8usize) as _) }
    }
    #[doc = "desc INTSEL32."]
    #[inline(always)]
    pub const fn intsel32(self) -> crate::common::Reg<regs::Intsel32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xdcusize) as _) }
    }
    #[doc = "desc INTSEL33."]
    #[inline(always)]
    pub const fn intsel33(self) -> crate::common::Reg<regs::Intsel33, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xe0usize) as _) }
    }
    #[doc = "desc INTSEL34."]
    #[inline(always)]
    pub const fn intsel34(self) -> crate::common::Reg<regs::Intsel34, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xe4usize) as _) }
    }
    #[doc = "desc INTSEL35."]
    #[inline(always)]
    pub const fn intsel35(self) -> crate::common::Reg<regs::Intsel35, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xe8usize) as _) }
    }
    #[doc = "desc INTSEL36."]
    #[inline(always)]
    pub const fn intsel36(self) -> crate::common::Reg<regs::Intsel36, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xecusize) as _) }
    }
    #[doc = "desc INTSEL37."]
    #[inline(always)]
    pub const fn intsel37(self) -> crate::common::Reg<regs::Intsel37, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xf0usize) as _) }
    }
    #[doc = "desc INTSEL38."]
    #[inline(always)]
    pub const fn intsel38(self) -> crate::common::Reg<regs::Intsel38, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xf4usize) as _) }
    }
    #[doc = "desc INTSEL39."]
    #[inline(always)]
    pub const fn intsel39(self) -> crate::common::Reg<regs::Intsel39, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xf8usize) as _) }
    }
    #[doc = "desc INTSEL40."]
    #[inline(always)]
    pub const fn intsel40(self) -> crate::common::Reg<regs::Intsel40, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xfcusize) as _) }
    }
    #[doc = "desc INTSEL41."]
    #[inline(always)]
    pub const fn intsel41(self) -> crate::common::Reg<regs::Intsel41, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0100usize) as _) }
    }
    #[doc = "desc INTSEL42."]
    #[inline(always)]
    pub const fn intsel42(self) -> crate::common::Reg<regs::Intsel42, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0104usize) as _) }
    }
    #[doc = "desc INTSEL43."]
    #[inline(always)]
    pub const fn intsel43(self) -> crate::common::Reg<regs::Intsel43, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0108usize) as _) }
    }
    #[doc = "desc INTSEL44."]
    #[inline(always)]
    pub const fn intsel44(self) -> crate::common::Reg<regs::Intsel44, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x010cusize) as _) }
    }
    #[doc = "desc INTSEL45."]
    #[inline(always)]
    pub const fn intsel45(self) -> crate::common::Reg<regs::Intsel45, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0110usize) as _) }
    }
    #[doc = "desc INTSEL46."]
    #[inline(always)]
    pub const fn intsel46(self) -> crate::common::Reg<regs::Intsel46, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0114usize) as _) }
    }
    #[doc = "desc INTSEL47."]
    #[inline(always)]
    pub const fn intsel47(self) -> crate::common::Reg<regs::Intsel47, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0118usize) as _) }
    }
    #[doc = "desc INTSEL48."]
    #[inline(always)]
    pub const fn intsel48(self) -> crate::common::Reg<regs::Intsel48, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x011cusize) as _) }
    }
    #[doc = "desc INTSEL49."]
    #[inline(always)]
    pub const fn intsel49(self) -> crate::common::Reg<regs::Intsel49, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0120usize) as _) }
    }
    #[doc = "desc INTSEL50."]
    #[inline(always)]
    pub const fn intsel50(self) -> crate::common::Reg<regs::Intsel50, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0124usize) as _) }
    }
    #[doc = "desc INTSEL51."]
    #[inline(always)]
    pub const fn intsel51(self) -> crate::common::Reg<regs::Intsel51, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0128usize) as _) }
    }
    #[doc = "desc INTSEL52."]
    #[inline(always)]
    pub const fn intsel52(self) -> crate::common::Reg<regs::Intsel52, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x012cusize) as _) }
    }
    #[doc = "desc INTSEL53."]
    #[inline(always)]
    pub const fn intsel53(self) -> crate::common::Reg<regs::Intsel53, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0130usize) as _) }
    }
    #[doc = "desc INTSEL54."]
    #[inline(always)]
    pub const fn intsel54(self) -> crate::common::Reg<regs::Intsel54, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0134usize) as _) }
    }
    #[doc = "desc INTSEL55."]
    #[inline(always)]
    pub const fn intsel55(self) -> crate::common::Reg<regs::Intsel55, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0138usize) as _) }
    }
    #[doc = "desc INTSEL56."]
    #[inline(always)]
    pub const fn intsel56(self) -> crate::common::Reg<regs::Intsel56, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x013cusize) as _) }
    }
    #[doc = "desc INTSEL57."]
    #[inline(always)]
    pub const fn intsel57(self) -> crate::common::Reg<regs::Intsel57, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0140usize) as _) }
    }
    #[doc = "desc INTSEL58."]
    #[inline(always)]
    pub const fn intsel58(self) -> crate::common::Reg<regs::Intsel58, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0144usize) as _) }
    }
    #[doc = "desc INTSEL59."]
    #[inline(always)]
    pub const fn intsel59(self) -> crate::common::Reg<regs::Intsel59, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0148usize) as _) }
    }
    #[doc = "desc INTSEL60."]
    #[inline(always)]
    pub const fn intsel60(self) -> crate::common::Reg<regs::Intsel60, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x014cusize) as _) }
    }
    #[doc = "desc INTSEL61."]
    #[inline(always)]
    pub const fn intsel61(self) -> crate::common::Reg<regs::Intsel61, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0150usize) as _) }
    }
    #[doc = "desc INTSEL62."]
    #[inline(always)]
    pub const fn intsel62(self) -> crate::common::Reg<regs::Intsel62, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0154usize) as _) }
    }
    #[doc = "desc INTSEL63."]
    #[inline(always)]
    pub const fn intsel63(self) -> crate::common::Reg<regs::Intsel63, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0158usize) as _) }
    }
    #[doc = "desc INTSEL64."]
    #[inline(always)]
    pub const fn intsel64(self) -> crate::common::Reg<regs::Intsel64, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x015cusize) as _) }
    }
    #[doc = "desc INTSEL65."]
    #[inline(always)]
    pub const fn intsel65(self) -> crate::common::Reg<regs::Intsel65, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0160usize) as _) }
    }
    #[doc = "desc INTSEL66."]
    #[inline(always)]
    pub const fn intsel66(self) -> crate::common::Reg<regs::Intsel66, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0164usize) as _) }
    }
    #[doc = "desc INTSEL67."]
    #[inline(always)]
    pub const fn intsel67(self) -> crate::common::Reg<regs::Intsel67, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0168usize) as _) }
    }
    #[doc = "desc INTSEL68."]
    #[inline(always)]
    pub const fn intsel68(self) -> crate::common::Reg<regs::Intsel68, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x016cusize) as _) }
    }
    #[doc = "desc INTSEL69."]
    #[inline(always)]
    pub const fn intsel69(self) -> crate::common::Reg<regs::Intsel69, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0170usize) as _) }
    }
    #[doc = "desc INTSEL70."]
    #[inline(always)]
    pub const fn intsel70(self) -> crate::common::Reg<regs::Intsel70, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0174usize) as _) }
    }
    #[doc = "desc INTSEL71."]
    #[inline(always)]
    pub const fn intsel71(self) -> crate::common::Reg<regs::Intsel71, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0178usize) as _) }
    }
    #[doc = "desc INTSEL72."]
    #[inline(always)]
    pub const fn intsel72(self) -> crate::common::Reg<regs::Intsel72, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x017cusize) as _) }
    }
    #[doc = "desc INTSEL73."]
    #[inline(always)]
    pub const fn intsel73(self) -> crate::common::Reg<regs::Intsel73, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0180usize) as _) }
    }
    #[doc = "desc INTSEL74."]
    #[inline(always)]
    pub const fn intsel74(self) -> crate::common::Reg<regs::Intsel74, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0184usize) as _) }
    }
    #[doc = "desc INTSEL75."]
    #[inline(always)]
    pub const fn intsel75(self) -> crate::common::Reg<regs::Intsel75, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0188usize) as _) }
    }
    #[doc = "desc INTSEL76."]
    #[inline(always)]
    pub const fn intsel76(self) -> crate::common::Reg<regs::Intsel76, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x018cusize) as _) }
    }
    #[doc = "desc INTSEL77."]
    #[inline(always)]
    pub const fn intsel77(self) -> crate::common::Reg<regs::Intsel77, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0190usize) as _) }
    }
    #[doc = "desc INTSEL78."]
    #[inline(always)]
    pub const fn intsel78(self) -> crate::common::Reg<regs::Intsel78, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0194usize) as _) }
    }
    #[doc = "desc INTSEL79."]
    #[inline(always)]
    pub const fn intsel79(self) -> crate::common::Reg<regs::Intsel79, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0198usize) as _) }
    }
    #[doc = "desc INTSEL80."]
    #[inline(always)]
    pub const fn intsel80(self) -> crate::common::Reg<regs::Intsel80, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x019cusize) as _) }
    }
    #[doc = "desc INTSEL81."]
    #[inline(always)]
    pub const fn intsel81(self) -> crate::common::Reg<regs::Intsel81, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x01a0usize) as _) }
    }
    #[doc = "desc INTSEL82."]
    #[inline(always)]
    pub const fn intsel82(self) -> crate::common::Reg<regs::Intsel82, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x01a4usize) as _) }
    }
    #[doc = "desc INTSEL83."]
    #[inline(always)]
    pub const fn intsel83(self) -> crate::common::Reg<regs::Intsel83, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x01a8usize) as _) }
    }
    #[doc = "desc INTSEL84."]
    #[inline(always)]
    pub const fn intsel84(self) -> crate::common::Reg<regs::Intsel84, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x01acusize) as _) }
    }
    #[doc = "desc INTSEL85."]
    #[inline(always)]
    pub const fn intsel85(self) -> crate::common::Reg<regs::Intsel85, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x01b0usize) as _) }
    }
    #[doc = "desc INTSEL86."]
    #[inline(always)]
    pub const fn intsel86(self) -> crate::common::Reg<regs::Intsel86, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x01b4usize) as _) }
    }
    #[doc = "desc INTSEL87."]
    #[inline(always)]
    pub const fn intsel87(self) -> crate::common::Reg<regs::Intsel87, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x01b8usize) as _) }
    }
    #[doc = "desc INTSEL88."]
    #[inline(always)]
    pub const fn intsel88(self) -> crate::common::Reg<regs::Intsel88, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x01bcusize) as _) }
    }
    #[doc = "desc INTSEL89."]
    #[inline(always)]
    pub const fn intsel89(self) -> crate::common::Reg<regs::Intsel89, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x01c0usize) as _) }
    }
    #[doc = "desc INTSEL90."]
    #[inline(always)]
    pub const fn intsel90(self) -> crate::common::Reg<regs::Intsel90, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x01c4usize) as _) }
    }
    #[doc = "desc INTSEL91."]
    #[inline(always)]
    pub const fn intsel91(self) -> crate::common::Reg<regs::Intsel91, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x01c8usize) as _) }
    }
    #[doc = "desc INTSEL92."]
    #[inline(always)]
    pub const fn intsel92(self) -> crate::common::Reg<regs::Intsel92, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x01ccusize) as _) }
    }
    #[doc = "desc INTSEL93."]
    #[inline(always)]
    pub const fn intsel93(self) -> crate::common::Reg<regs::Intsel93, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x01d0usize) as _) }
    }
    #[doc = "desc INTSEL94."]
    #[inline(always)]
    pub const fn intsel94(self) -> crate::common::Reg<regs::Intsel94, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x01d4usize) as _) }
    }
    #[doc = "desc INTSEL95."]
    #[inline(always)]
    pub const fn intsel95(self) -> crate::common::Reg<regs::Intsel95, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x01d8usize) as _) }
    }
    #[doc = "desc INTSEL96."]
    #[inline(always)]
    pub const fn intsel96(self) -> crate::common::Reg<regs::Intsel96, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x01dcusize) as _) }
    }
    #[doc = "desc INTSEL97."]
    #[inline(always)]
    pub const fn intsel97(self) -> crate::common::Reg<regs::Intsel97, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x01e0usize) as _) }
    }
    #[doc = "desc INTSEL98."]
    #[inline(always)]
    pub const fn intsel98(self) -> crate::common::Reg<regs::Intsel98, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x01e4usize) as _) }
    }
    #[doc = "desc INTSEL99."]
    #[inline(always)]
    pub const fn intsel99(self) -> crate::common::Reg<regs::Intsel99, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x01e8usize) as _) }
    }
    #[doc = "desc INTSEL100."]
    #[inline(always)]
    pub const fn intsel100(self) -> crate::common::Reg<regs::Intsel100, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x01ecusize) as _) }
    }
    #[doc = "desc INTSEL101."]
    #[inline(always)]
    pub const fn intsel101(self) -> crate::common::Reg<regs::Intsel101, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x01f0usize) as _) }
    }
    #[doc = "desc INTSEL102."]
    #[inline(always)]
    pub const fn intsel102(self) -> crate::common::Reg<regs::Intsel102, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x01f4usize) as _) }
    }
    #[doc = "desc INTSEL103."]
    #[inline(always)]
    pub const fn intsel103(self) -> crate::common::Reg<regs::Intsel103, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x01f8usize) as _) }
    }
    #[doc = "desc INTSEL104."]
    #[inline(always)]
    pub const fn intsel104(self) -> crate::common::Reg<regs::Intsel104, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x01fcusize) as _) }
    }
    #[doc = "desc INTSEL105."]
    #[inline(always)]
    pub const fn intsel105(self) -> crate::common::Reg<regs::Intsel105, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0200usize) as _) }
    }
    #[doc = "desc INTSEL106."]
    #[inline(always)]
    pub const fn intsel106(self) -> crate::common::Reg<regs::Intsel106, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0204usize) as _) }
    }
    #[doc = "desc INTSEL107."]
    #[inline(always)]
    pub const fn intsel107(self) -> crate::common::Reg<regs::Intsel107, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0208usize) as _) }
    }
    #[doc = "desc INTSEL108."]
    #[inline(always)]
    pub const fn intsel108(self) -> crate::common::Reg<regs::Intsel108, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x020cusize) as _) }
    }
    #[doc = "desc INTSEL109."]
    #[inline(always)]
    pub const fn intsel109(self) -> crate::common::Reg<regs::Intsel109, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0210usize) as _) }
    }
    #[doc = "desc INTSEL110."]
    #[inline(always)]
    pub const fn intsel110(self) -> crate::common::Reg<regs::Intsel110, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0214usize) as _) }
    }
    #[doc = "desc INTSEL111."]
    #[inline(always)]
    pub const fn intsel111(self) -> crate::common::Reg<regs::Intsel111, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0218usize) as _) }
    }
    #[doc = "desc INTSEL112."]
    #[inline(always)]
    pub const fn intsel112(self) -> crate::common::Reg<regs::Intsel112, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x021cusize) as _) }
    }
    #[doc = "desc INTSEL113."]
    #[inline(always)]
    pub const fn intsel113(self) -> crate::common::Reg<regs::Intsel113, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0220usize) as _) }
    }
    #[doc = "desc INTSEL114."]
    #[inline(always)]
    pub const fn intsel114(self) -> crate::common::Reg<regs::Intsel114, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0224usize) as _) }
    }
    #[doc = "desc INTSEL115."]
    #[inline(always)]
    pub const fn intsel115(self) -> crate::common::Reg<regs::Intsel115, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0228usize) as _) }
    }
    #[doc = "desc INTSEL116."]
    #[inline(always)]
    pub const fn intsel116(self) -> crate::common::Reg<regs::Intsel116, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x022cusize) as _) }
    }
    #[doc = "desc INTSEL117."]
    #[inline(always)]
    pub const fn intsel117(self) -> crate::common::Reg<regs::Intsel117, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0230usize) as _) }
    }
    #[doc = "desc INTSEL118."]
    #[inline(always)]
    pub const fn intsel118(self) -> crate::common::Reg<regs::Intsel118, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0234usize) as _) }
    }
    #[doc = "desc INTSEL119."]
    #[inline(always)]
    pub const fn intsel119(self) -> crate::common::Reg<regs::Intsel119, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0238usize) as _) }
    }
    #[doc = "desc INTSEL120."]
    #[inline(always)]
    pub const fn intsel120(self) -> crate::common::Reg<regs::Intsel120, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x023cusize) as _) }
    }
    #[doc = "desc INTSEL121."]
    #[inline(always)]
    pub const fn intsel121(self) -> crate::common::Reg<regs::Intsel121, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0240usize) as _) }
    }
    #[doc = "desc INTSEL122."]
    #[inline(always)]
    pub const fn intsel122(self) -> crate::common::Reg<regs::Intsel122, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0244usize) as _) }
    }
    #[doc = "desc INTSEL123."]
    #[inline(always)]
    pub const fn intsel123(self) -> crate::common::Reg<regs::Intsel123, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0248usize) as _) }
    }
    #[doc = "desc INTSEL124."]
    #[inline(always)]
    pub const fn intsel124(self) -> crate::common::Reg<regs::Intsel124, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x024cusize) as _) }
    }
    #[doc = "desc INTSEL125."]
    #[inline(always)]
    pub const fn intsel125(self) -> crate::common::Reg<regs::Intsel125, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0250usize) as _) }
    }
    #[doc = "desc INTSEL126."]
    #[inline(always)]
    pub const fn intsel126(self) -> crate::common::Reg<regs::Intsel126, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0254usize) as _) }
    }
    #[doc = "desc INTSEL127."]
    #[inline(always)]
    pub const fn intsel127(self) -> crate::common::Reg<regs::Intsel127, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0258usize) as _) }
    }
    #[doc = "desc VSSEL128."]
    #[inline(always)]
    pub const fn vssel128(self) -> crate::common::Reg<regs::Vssel128, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x025cusize) as _) }
    }
    #[doc = "desc VSSEL129."]
    #[inline(always)]
    pub const fn vssel129(self) -> crate::common::Reg<regs::Vssel129, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0260usize) as _) }
    }
    #[doc = "desc VSSEL130."]
    #[inline(always)]
    pub const fn vssel130(self) -> crate::common::Reg<regs::Vssel130, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0264usize) as _) }
    }
    #[doc = "desc VSSEL131."]
    #[inline(always)]
    pub const fn vssel131(self) -> crate::common::Reg<regs::Vssel131, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0268usize) as _) }
    }
    #[doc = "desc VSSEL132."]
    #[inline(always)]
    pub const fn vssel132(self) -> crate::common::Reg<regs::Vssel132, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x026cusize) as _) }
    }
    #[doc = "desc VSSEL133."]
    #[inline(always)]
    pub const fn vssel133(self) -> crate::common::Reg<regs::Vssel133, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0270usize) as _) }
    }
    #[doc = "desc VSSEL134."]
    #[inline(always)]
    pub const fn vssel134(self) -> crate::common::Reg<regs::Vssel134, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0274usize) as _) }
    }
    #[doc = "desc VSSEL135."]
    #[inline(always)]
    pub const fn vssel135(self) -> crate::common::Reg<regs::Vssel135, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0278usize) as _) }
    }
    #[doc = "desc VSSEL136."]
    #[inline(always)]
    pub const fn vssel136(self) -> crate::common::Reg<regs::Vssel136, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x027cusize) as _) }
    }
    #[doc = "desc VSSEL137."]
    #[inline(always)]
    pub const fn vssel137(self) -> crate::common::Reg<regs::Vssel137, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0280usize) as _) }
    }
    #[doc = "desc VSSEL138."]
    #[inline(always)]
    pub const fn vssel138(self) -> crate::common::Reg<regs::Vssel138, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0284usize) as _) }
    }
    #[doc = "desc VSSEL139."]
    #[inline(always)]
    pub const fn vssel139(self) -> crate::common::Reg<regs::Vssel139, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0288usize) as _) }
    }
    #[doc = "desc VSSEL140."]
    #[inline(always)]
    pub const fn vssel140(self) -> crate::common::Reg<regs::Vssel140, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x028cusize) as _) }
    }
    #[doc = "desc VSSEL141."]
    #[inline(always)]
    pub const fn vssel141(self) -> crate::common::Reg<regs::Vssel141, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0290usize) as _) }
    }
    #[doc = "desc VSSEL142."]
    #[inline(always)]
    pub const fn vssel142(self) -> crate::common::Reg<regs::Vssel142, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0294usize) as _) }
    }
    #[doc = "desc VSSEL143."]
    #[inline(always)]
    pub const fn vssel143(self) -> crate::common::Reg<regs::Vssel143, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0298usize) as _) }
    }
    #[doc = "desc SWIER."]
    #[inline(always)]
    pub const fn swier(self) -> crate::common::Reg<regs::Swier, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x029cusize) as _) }
    }
    #[doc = "desc EVTER."]
    #[inline(always)]
    pub const fn evter(self) -> crate::common::Reg<regs::Evter, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x02a0usize) as _) }
    }
    #[doc = "desc IER."]
    #[inline(always)]
    pub const fn ier(self) -> crate::common::Reg<regs::Ier, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x02a4usize) as _) }
    }
    #[doc = "desc FPUIER."]
    #[inline(always)]
    pub const fn fpuier(self) -> crate::common::Reg<regs::Fpuier, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x02a8usize) as _) }
    }
}
pub mod regs {
    #[doc = "desc EIFCR."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Eifcr(pub u32);
    impl Eifcr {
        #[doc = "desc EIFCR0."]
        #[must_use]
        #[inline(always)]
        pub const fn eifcr0(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "desc EIFCR0."]
        #[inline(always)]
        pub const fn set_eifcr0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "desc EIFCR1."]
        #[must_use]
        #[inline(always)]
        pub const fn eifcr1(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "desc EIFCR1."]
        #[inline(always)]
        pub const fn set_eifcr1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "desc EIFCR2."]
        #[must_use]
        #[inline(always)]
        pub const fn eifcr2(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "desc EIFCR2."]
        #[inline(always)]
        pub const fn set_eifcr2(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "desc EIFCR3."]
        #[must_use]
        #[inline(always)]
        pub const fn eifcr3(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "desc EIFCR3."]
        #[inline(always)]
        pub const fn set_eifcr3(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "desc EIFCR4."]
        #[must_use]
        #[inline(always)]
        pub const fn eifcr4(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "desc EIFCR4."]
        #[inline(always)]
        pub const fn set_eifcr4(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "desc EIFCR5."]
        #[must_use]
        #[inline(always)]
        pub const fn eifcr5(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "desc EIFCR5."]
        #[inline(always)]
        pub const fn set_eifcr5(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "desc EIFCR6."]
        #[must_use]
        #[inline(always)]
        pub const fn eifcr6(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "desc EIFCR6."]
        #[inline(always)]
        pub const fn set_eifcr6(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "desc EIFCR7."]
        #[must_use]
        #[inline(always)]
        pub const fn eifcr7(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "desc EIFCR7."]
        #[inline(always)]
        pub const fn set_eifcr7(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "desc EIFCR8."]
        #[must_use]
        #[inline(always)]
        pub const fn eifcr8(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "desc EIFCR8."]
        #[inline(always)]
        pub const fn set_eifcr8(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "desc EIFCR9."]
        #[must_use]
        #[inline(always)]
        pub const fn eifcr9(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "desc EIFCR9."]
        #[inline(always)]
        pub const fn set_eifcr9(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "desc EIFCR10."]
        #[must_use]
        #[inline(always)]
        pub const fn eifcr10(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "desc EIFCR10."]
        #[inline(always)]
        pub const fn set_eifcr10(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[doc = "desc EIFCR11."]
        #[must_use]
        #[inline(always)]
        pub const fn eifcr11(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "desc EIFCR11."]
        #[inline(always)]
        pub const fn set_eifcr11(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[doc = "desc EIFCR12."]
        #[must_use]
        #[inline(always)]
        pub const fn eifcr12(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "desc EIFCR12."]
        #[inline(always)]
        pub const fn set_eifcr12(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "desc EIFCR13."]
        #[must_use]
        #[inline(always)]
        pub const fn eifcr13(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[doc = "desc EIFCR13."]
        #[inline(always)]
        pub const fn set_eifcr13(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[doc = "desc EIFCR14."]
        #[must_use]
        #[inline(always)]
        pub const fn eifcr14(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "desc EIFCR14."]
        #[inline(always)]
        pub const fn set_eifcr14(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[doc = "desc EIFCR15."]
        #[must_use]
        #[inline(always)]
        pub const fn eifcr15(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "desc EIFCR15."]
        #[inline(always)]
        pub const fn set_eifcr15(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
    }
    impl Default for Eifcr {
        #[inline(always)]
        fn default() -> Eifcr {
            Eifcr(0)
        }
    }
    impl core::fmt::Debug for Eifcr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Eifcr")
                .field("eifcr0", &self.eifcr0())
                .field("eifcr1", &self.eifcr1())
                .field("eifcr2", &self.eifcr2())
                .field("eifcr3", &self.eifcr3())
                .field("eifcr4", &self.eifcr4())
                .field("eifcr5", &self.eifcr5())
                .field("eifcr6", &self.eifcr6())
                .field("eifcr7", &self.eifcr7())
                .field("eifcr8", &self.eifcr8())
                .field("eifcr9", &self.eifcr9())
                .field("eifcr10", &self.eifcr10())
                .field("eifcr11", &self.eifcr11())
                .field("eifcr12", &self.eifcr12())
                .field("eifcr13", &self.eifcr13())
                .field("eifcr14", &self.eifcr14())
                .field("eifcr15", &self.eifcr15())
                .finish()
        }
    }
    #[doc = "desc EIFR."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Eifr(pub u32);
    impl Eifr {
        #[doc = "desc EIFR0."]
        #[must_use]
        #[inline(always)]
        pub const fn eifr0(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "desc EIFR0."]
        #[inline(always)]
        pub const fn set_eifr0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "desc EIFR1."]
        #[must_use]
        #[inline(always)]
        pub const fn eifr1(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "desc EIFR1."]
        #[inline(always)]
        pub const fn set_eifr1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "desc EIFR2."]
        #[must_use]
        #[inline(always)]
        pub const fn eifr2(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "desc EIFR2."]
        #[inline(always)]
        pub const fn set_eifr2(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "desc EIFR3."]
        #[must_use]
        #[inline(always)]
        pub const fn eifr3(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "desc EIFR3."]
        #[inline(always)]
        pub const fn set_eifr3(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "desc EIFR4."]
        #[must_use]
        #[inline(always)]
        pub const fn eifr4(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "desc EIFR4."]
        #[inline(always)]
        pub const fn set_eifr4(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "desc EIFR5."]
        #[must_use]
        #[inline(always)]
        pub const fn eifr5(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "desc EIFR5."]
        #[inline(always)]
        pub const fn set_eifr5(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "desc EIFR6."]
        #[must_use]
        #[inline(always)]
        pub const fn eifr6(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "desc EIFR6."]
        #[inline(always)]
        pub const fn set_eifr6(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "desc EIFR7."]
        #[must_use]
        #[inline(always)]
        pub const fn eifr7(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "desc EIFR7."]
        #[inline(always)]
        pub const fn set_eifr7(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "desc EIFR8."]
        #[must_use]
        #[inline(always)]
        pub const fn eifr8(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "desc EIFR8."]
        #[inline(always)]
        pub const fn set_eifr8(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "desc EIFR9."]
        #[must_use]
        #[inline(always)]
        pub const fn eifr9(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "desc EIFR9."]
        #[inline(always)]
        pub const fn set_eifr9(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "desc EIFR10."]
        #[must_use]
        #[inline(always)]
        pub const fn eifr10(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "desc EIFR10."]
        #[inline(always)]
        pub const fn set_eifr10(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[doc = "desc EIFR11."]
        #[must_use]
        #[inline(always)]
        pub const fn eifr11(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "desc EIFR11."]
        #[inline(always)]
        pub const fn set_eifr11(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[doc = "desc EIFR12."]
        #[must_use]
        #[inline(always)]
        pub const fn eifr12(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "desc EIFR12."]
        #[inline(always)]
        pub const fn set_eifr12(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "desc EIFR13."]
        #[must_use]
        #[inline(always)]
        pub const fn eifr13(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[doc = "desc EIFR13."]
        #[inline(always)]
        pub const fn set_eifr13(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[doc = "desc EIFR14."]
        #[must_use]
        #[inline(always)]
        pub const fn eifr14(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "desc EIFR14."]
        #[inline(always)]
        pub const fn set_eifr14(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[doc = "desc EIFR15."]
        #[must_use]
        #[inline(always)]
        pub const fn eifr15(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "desc EIFR15."]
        #[inline(always)]
        pub const fn set_eifr15(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
    }
    impl Default for Eifr {
        #[inline(always)]
        fn default() -> Eifr {
            Eifr(0)
        }
    }
    impl core::fmt::Debug for Eifr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Eifr")
                .field("eifr0", &self.eifr0())
                .field("eifr1", &self.eifr1())
                .field("eifr2", &self.eifr2())
                .field("eifr3", &self.eifr3())
                .field("eifr4", &self.eifr4())
                .field("eifr5", &self.eifr5())
                .field("eifr6", &self.eifr6())
                .field("eifr7", &self.eifr7())
                .field("eifr8", &self.eifr8())
                .field("eifr9", &self.eifr9())
                .field("eifr10", &self.eifr10())
                .field("eifr11", &self.eifr11())
                .field("eifr12", &self.eifr12())
                .field("eifr13", &self.eifr13())
                .field("eifr14", &self.eifr14())
                .field("eifr15", &self.eifr15())
                .finish()
        }
    }
    #[doc = "desc EIRQCR0."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Eirqcr0(pub u32);
    impl Eirqcr0 {
        #[doc = "desc EIRQTRG."]
        #[must_use]
        #[inline(always)]
        pub const fn eirqtrg(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x03;
            val as u8
        }
        #[doc = "desc EIRQTRG."]
        #[inline(always)]
        pub const fn set_eirqtrg(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 0usize)) | (((val as u32) & 0x03) << 0usize);
        }
        #[doc = "desc EISMPCLK."]
        #[must_use]
        #[inline(always)]
        pub const fn eismpclk(&self) -> u8 {
            let val = (self.0 >> 4usize) & 0x03;
            val as u8
        }
        #[doc = "desc EISMPCLK."]
        #[inline(always)]
        pub const fn set_eismpclk(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 4usize)) | (((val as u32) & 0x03) << 4usize);
        }
        #[doc = "desc EFEN."]
        #[must_use]
        #[inline(always)]
        pub const fn efen(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "desc EFEN."]
        #[inline(always)]
        pub const fn set_efen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "desc NOCEN."]
        #[must_use]
        #[inline(always)]
        pub const fn nocen(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "desc NOCEN."]
        #[inline(always)]
        pub const fn set_nocen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
        #[doc = "desc NFSEL."]
        #[must_use]
        #[inline(always)]
        pub const fn nfsel(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0xff;
            val as u8
        }
        #[doc = "desc NFSEL."]
        #[inline(always)]
        pub const fn set_nfsel(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 16usize)) | (((val as u32) & 0xff) << 16usize);
        }
    }
    impl Default for Eirqcr0 {
        #[inline(always)]
        fn default() -> Eirqcr0 {
            Eirqcr0(0)
        }
    }
    impl core::fmt::Debug for Eirqcr0 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Eirqcr0")
                .field("eirqtrg", &self.eirqtrg())
                .field("eismpclk", &self.eismpclk())
                .field("efen", &self.efen())
                .field("nocen", &self.nocen())
                .field("nfsel", &self.nfsel())
                .finish()
        }
    }
    #[doc = "desc EIRQCR1."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Eirqcr1(pub u32);
    impl Eirqcr1 {
        #[doc = "desc EIRQTRG."]
        #[must_use]
        #[inline(always)]
        pub const fn eirqtrg(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x03;
            val as u8
        }
        #[doc = "desc EIRQTRG."]
        #[inline(always)]
        pub const fn set_eirqtrg(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 0usize)) | (((val as u32) & 0x03) << 0usize);
        }
        #[doc = "desc EISMPCLK."]
        #[must_use]
        #[inline(always)]
        pub const fn eismpclk(&self) -> u8 {
            let val = (self.0 >> 4usize) & 0x03;
            val as u8
        }
        #[doc = "desc EISMPCLK."]
        #[inline(always)]
        pub const fn set_eismpclk(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 4usize)) | (((val as u32) & 0x03) << 4usize);
        }
        #[doc = "desc EFEN."]
        #[must_use]
        #[inline(always)]
        pub const fn efen(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "desc EFEN."]
        #[inline(always)]
        pub const fn set_efen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "desc NOCEN."]
        #[must_use]
        #[inline(always)]
        pub const fn nocen(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "desc NOCEN."]
        #[inline(always)]
        pub const fn set_nocen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
        #[doc = "desc NFSEL."]
        #[must_use]
        #[inline(always)]
        pub const fn nfsel(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0xff;
            val as u8
        }
        #[doc = "desc NFSEL."]
        #[inline(always)]
        pub const fn set_nfsel(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 16usize)) | (((val as u32) & 0xff) << 16usize);
        }
    }
    impl Default for Eirqcr1 {
        #[inline(always)]
        fn default() -> Eirqcr1 {
            Eirqcr1(0)
        }
    }
    impl core::fmt::Debug for Eirqcr1 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Eirqcr1")
                .field("eirqtrg", &self.eirqtrg())
                .field("eismpclk", &self.eismpclk())
                .field("efen", &self.efen())
                .field("nocen", &self.nocen())
                .field("nfsel", &self.nfsel())
                .finish()
        }
    }
    #[doc = "desc EIRQCR10."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Eirqcr10(pub u32);
    impl Eirqcr10 {
        #[doc = "desc EIRQTRG."]
        #[must_use]
        #[inline(always)]
        pub const fn eirqtrg(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x03;
            val as u8
        }
        #[doc = "desc EIRQTRG."]
        #[inline(always)]
        pub const fn set_eirqtrg(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 0usize)) | (((val as u32) & 0x03) << 0usize);
        }
        #[doc = "desc EISMPCLK."]
        #[must_use]
        #[inline(always)]
        pub const fn eismpclk(&self) -> u8 {
            let val = (self.0 >> 4usize) & 0x03;
            val as u8
        }
        #[doc = "desc EISMPCLK."]
        #[inline(always)]
        pub const fn set_eismpclk(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 4usize)) | (((val as u32) & 0x03) << 4usize);
        }
        #[doc = "desc EFEN."]
        #[must_use]
        #[inline(always)]
        pub const fn efen(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "desc EFEN."]
        #[inline(always)]
        pub const fn set_efen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "desc NOCEN."]
        #[must_use]
        #[inline(always)]
        pub const fn nocen(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "desc NOCEN."]
        #[inline(always)]
        pub const fn set_nocen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
        #[doc = "desc NFSEL."]
        #[must_use]
        #[inline(always)]
        pub const fn nfsel(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0xff;
            val as u8
        }
        #[doc = "desc NFSEL."]
        #[inline(always)]
        pub const fn set_nfsel(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 16usize)) | (((val as u32) & 0xff) << 16usize);
        }
    }
    impl Default for Eirqcr10 {
        #[inline(always)]
        fn default() -> Eirqcr10 {
            Eirqcr10(0)
        }
    }
    impl core::fmt::Debug for Eirqcr10 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Eirqcr10")
                .field("eirqtrg", &self.eirqtrg())
                .field("eismpclk", &self.eismpclk())
                .field("efen", &self.efen())
                .field("nocen", &self.nocen())
                .field("nfsel", &self.nfsel())
                .finish()
        }
    }
    #[doc = "desc EIRQCR11."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Eirqcr11(pub u32);
    impl Eirqcr11 {
        #[doc = "desc EIRQTRG."]
        #[must_use]
        #[inline(always)]
        pub const fn eirqtrg(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x03;
            val as u8
        }
        #[doc = "desc EIRQTRG."]
        #[inline(always)]
        pub const fn set_eirqtrg(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 0usize)) | (((val as u32) & 0x03) << 0usize);
        }
        #[doc = "desc EISMPCLK."]
        #[must_use]
        #[inline(always)]
        pub const fn eismpclk(&self) -> u8 {
            let val = (self.0 >> 4usize) & 0x03;
            val as u8
        }
        #[doc = "desc EISMPCLK."]
        #[inline(always)]
        pub const fn set_eismpclk(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 4usize)) | (((val as u32) & 0x03) << 4usize);
        }
        #[doc = "desc EFEN."]
        #[must_use]
        #[inline(always)]
        pub const fn efen(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "desc EFEN."]
        #[inline(always)]
        pub const fn set_efen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "desc NOCEN."]
        #[must_use]
        #[inline(always)]
        pub const fn nocen(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "desc NOCEN."]
        #[inline(always)]
        pub const fn set_nocen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
        #[doc = "desc NFSEL."]
        #[must_use]
        #[inline(always)]
        pub const fn nfsel(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0xff;
            val as u8
        }
        #[doc = "desc NFSEL."]
        #[inline(always)]
        pub const fn set_nfsel(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 16usize)) | (((val as u32) & 0xff) << 16usize);
        }
    }
    impl Default for Eirqcr11 {
        #[inline(always)]
        fn default() -> Eirqcr11 {
            Eirqcr11(0)
        }
    }
    impl core::fmt::Debug for Eirqcr11 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Eirqcr11")
                .field("eirqtrg", &self.eirqtrg())
                .field("eismpclk", &self.eismpclk())
                .field("efen", &self.efen())
                .field("nocen", &self.nocen())
                .field("nfsel", &self.nfsel())
                .finish()
        }
    }
    #[doc = "desc EIRQCR12."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Eirqcr12(pub u32);
    impl Eirqcr12 {
        #[doc = "desc EIRQTRG."]
        #[must_use]
        #[inline(always)]
        pub const fn eirqtrg(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x03;
            val as u8
        }
        #[doc = "desc EIRQTRG."]
        #[inline(always)]
        pub const fn set_eirqtrg(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 0usize)) | (((val as u32) & 0x03) << 0usize);
        }
        #[doc = "desc EISMPCLK."]
        #[must_use]
        #[inline(always)]
        pub const fn eismpclk(&self) -> u8 {
            let val = (self.0 >> 4usize) & 0x03;
            val as u8
        }
        #[doc = "desc EISMPCLK."]
        #[inline(always)]
        pub const fn set_eismpclk(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 4usize)) | (((val as u32) & 0x03) << 4usize);
        }
        #[doc = "desc EFEN."]
        #[must_use]
        #[inline(always)]
        pub const fn efen(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "desc EFEN."]
        #[inline(always)]
        pub const fn set_efen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "desc NOCEN."]
        #[must_use]
        #[inline(always)]
        pub const fn nocen(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "desc NOCEN."]
        #[inline(always)]
        pub const fn set_nocen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
        #[doc = "desc NFSEL."]
        #[must_use]
        #[inline(always)]
        pub const fn nfsel(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0xff;
            val as u8
        }
        #[doc = "desc NFSEL."]
        #[inline(always)]
        pub const fn set_nfsel(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 16usize)) | (((val as u32) & 0xff) << 16usize);
        }
    }
    impl Default for Eirqcr12 {
        #[inline(always)]
        fn default() -> Eirqcr12 {
            Eirqcr12(0)
        }
    }
    impl core::fmt::Debug for Eirqcr12 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Eirqcr12")
                .field("eirqtrg", &self.eirqtrg())
                .field("eismpclk", &self.eismpclk())
                .field("efen", &self.efen())
                .field("nocen", &self.nocen())
                .field("nfsel", &self.nfsel())
                .finish()
        }
    }
    #[doc = "desc EIRQCR13."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Eirqcr13(pub u32);
    impl Eirqcr13 {
        #[doc = "desc EIRQTRG."]
        #[must_use]
        #[inline(always)]
        pub const fn eirqtrg(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x03;
            val as u8
        }
        #[doc = "desc EIRQTRG."]
        #[inline(always)]
        pub const fn set_eirqtrg(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 0usize)) | (((val as u32) & 0x03) << 0usize);
        }
        #[doc = "desc EISMPCLK."]
        #[must_use]
        #[inline(always)]
        pub const fn eismpclk(&self) -> u8 {
            let val = (self.0 >> 4usize) & 0x03;
            val as u8
        }
        #[doc = "desc EISMPCLK."]
        #[inline(always)]
        pub const fn set_eismpclk(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 4usize)) | (((val as u32) & 0x03) << 4usize);
        }
        #[doc = "desc EFEN."]
        #[must_use]
        #[inline(always)]
        pub const fn efen(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "desc EFEN."]
        #[inline(always)]
        pub const fn set_efen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "desc NOCEN."]
        #[must_use]
        #[inline(always)]
        pub const fn nocen(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "desc NOCEN."]
        #[inline(always)]
        pub const fn set_nocen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
        #[doc = "desc NFSEL."]
        #[must_use]
        #[inline(always)]
        pub const fn nfsel(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0xff;
            val as u8
        }
        #[doc = "desc NFSEL."]
        #[inline(always)]
        pub const fn set_nfsel(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 16usize)) | (((val as u32) & 0xff) << 16usize);
        }
    }
    impl Default for Eirqcr13 {
        #[inline(always)]
        fn default() -> Eirqcr13 {
            Eirqcr13(0)
        }
    }
    impl core::fmt::Debug for Eirqcr13 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Eirqcr13")
                .field("eirqtrg", &self.eirqtrg())
                .field("eismpclk", &self.eismpclk())
                .field("efen", &self.efen())
                .field("nocen", &self.nocen())
                .field("nfsel", &self.nfsel())
                .finish()
        }
    }
    #[doc = "desc EIRQCR14."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Eirqcr14(pub u32);
    impl Eirqcr14 {
        #[doc = "desc EIRQTRG."]
        #[must_use]
        #[inline(always)]
        pub const fn eirqtrg(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x03;
            val as u8
        }
        #[doc = "desc EIRQTRG."]
        #[inline(always)]
        pub const fn set_eirqtrg(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 0usize)) | (((val as u32) & 0x03) << 0usize);
        }
        #[doc = "desc EISMPCLK."]
        #[must_use]
        #[inline(always)]
        pub const fn eismpclk(&self) -> u8 {
            let val = (self.0 >> 4usize) & 0x03;
            val as u8
        }
        #[doc = "desc EISMPCLK."]
        #[inline(always)]
        pub const fn set_eismpclk(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 4usize)) | (((val as u32) & 0x03) << 4usize);
        }
        #[doc = "desc EFEN."]
        #[must_use]
        #[inline(always)]
        pub const fn efen(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "desc EFEN."]
        #[inline(always)]
        pub const fn set_efen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "desc NOCEN."]
        #[must_use]
        #[inline(always)]
        pub const fn nocen(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "desc NOCEN."]
        #[inline(always)]
        pub const fn set_nocen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
        #[doc = "desc NFSEL."]
        #[must_use]
        #[inline(always)]
        pub const fn nfsel(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0xff;
            val as u8
        }
        #[doc = "desc NFSEL."]
        #[inline(always)]
        pub const fn set_nfsel(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 16usize)) | (((val as u32) & 0xff) << 16usize);
        }
    }
    impl Default for Eirqcr14 {
        #[inline(always)]
        fn default() -> Eirqcr14 {
            Eirqcr14(0)
        }
    }
    impl core::fmt::Debug for Eirqcr14 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Eirqcr14")
                .field("eirqtrg", &self.eirqtrg())
                .field("eismpclk", &self.eismpclk())
                .field("efen", &self.efen())
                .field("nocen", &self.nocen())
                .field("nfsel", &self.nfsel())
                .finish()
        }
    }
    #[doc = "desc EIRQCR15."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Eirqcr15(pub u32);
    impl Eirqcr15 {
        #[doc = "desc EIRQTRG."]
        #[must_use]
        #[inline(always)]
        pub const fn eirqtrg(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x03;
            val as u8
        }
        #[doc = "desc EIRQTRG."]
        #[inline(always)]
        pub const fn set_eirqtrg(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 0usize)) | (((val as u32) & 0x03) << 0usize);
        }
        #[doc = "desc EISMPCLK."]
        #[must_use]
        #[inline(always)]
        pub const fn eismpclk(&self) -> u8 {
            let val = (self.0 >> 4usize) & 0x03;
            val as u8
        }
        #[doc = "desc EISMPCLK."]
        #[inline(always)]
        pub const fn set_eismpclk(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 4usize)) | (((val as u32) & 0x03) << 4usize);
        }
        #[doc = "desc EFEN."]
        #[must_use]
        #[inline(always)]
        pub const fn efen(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "desc EFEN."]
        #[inline(always)]
        pub const fn set_efen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "desc NOCEN."]
        #[must_use]
        #[inline(always)]
        pub const fn nocen(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "desc NOCEN."]
        #[inline(always)]
        pub const fn set_nocen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
        #[doc = "desc NFSEL."]
        #[must_use]
        #[inline(always)]
        pub const fn nfsel(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0xff;
            val as u8
        }
        #[doc = "desc NFSEL."]
        #[inline(always)]
        pub const fn set_nfsel(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 16usize)) | (((val as u32) & 0xff) << 16usize);
        }
    }
    impl Default for Eirqcr15 {
        #[inline(always)]
        fn default() -> Eirqcr15 {
            Eirqcr15(0)
        }
    }
    impl core::fmt::Debug for Eirqcr15 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Eirqcr15")
                .field("eirqtrg", &self.eirqtrg())
                .field("eismpclk", &self.eismpclk())
                .field("efen", &self.efen())
                .field("nocen", &self.nocen())
                .field("nfsel", &self.nfsel())
                .finish()
        }
    }
    #[doc = "desc EIRQCR2."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Eirqcr2(pub u32);
    impl Eirqcr2 {
        #[doc = "desc EIRQTRG."]
        #[must_use]
        #[inline(always)]
        pub const fn eirqtrg(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x03;
            val as u8
        }
        #[doc = "desc EIRQTRG."]
        #[inline(always)]
        pub const fn set_eirqtrg(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 0usize)) | (((val as u32) & 0x03) << 0usize);
        }
        #[doc = "desc EISMPCLK."]
        #[must_use]
        #[inline(always)]
        pub const fn eismpclk(&self) -> u8 {
            let val = (self.0 >> 4usize) & 0x03;
            val as u8
        }
        #[doc = "desc EISMPCLK."]
        #[inline(always)]
        pub const fn set_eismpclk(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 4usize)) | (((val as u32) & 0x03) << 4usize);
        }
        #[doc = "desc EFEN."]
        #[must_use]
        #[inline(always)]
        pub const fn efen(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "desc EFEN."]
        #[inline(always)]
        pub const fn set_efen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "desc NOCEN."]
        #[must_use]
        #[inline(always)]
        pub const fn nocen(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "desc NOCEN."]
        #[inline(always)]
        pub const fn set_nocen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
        #[doc = "desc NFSEL."]
        #[must_use]
        #[inline(always)]
        pub const fn nfsel(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0xff;
            val as u8
        }
        #[doc = "desc NFSEL."]
        #[inline(always)]
        pub const fn set_nfsel(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 16usize)) | (((val as u32) & 0xff) << 16usize);
        }
    }
    impl Default for Eirqcr2 {
        #[inline(always)]
        fn default() -> Eirqcr2 {
            Eirqcr2(0)
        }
    }
    impl core::fmt::Debug for Eirqcr2 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Eirqcr2")
                .field("eirqtrg", &self.eirqtrg())
                .field("eismpclk", &self.eismpclk())
                .field("efen", &self.efen())
                .field("nocen", &self.nocen())
                .field("nfsel", &self.nfsel())
                .finish()
        }
    }
    #[doc = "desc EIRQCR3."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Eirqcr3(pub u32);
    impl Eirqcr3 {
        #[doc = "desc EIRQTRG."]
        #[must_use]
        #[inline(always)]
        pub const fn eirqtrg(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x03;
            val as u8
        }
        #[doc = "desc EIRQTRG."]
        #[inline(always)]
        pub const fn set_eirqtrg(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 0usize)) | (((val as u32) & 0x03) << 0usize);
        }
        #[doc = "desc EISMPCLK."]
        #[must_use]
        #[inline(always)]
        pub const fn eismpclk(&self) -> u8 {
            let val = (self.0 >> 4usize) & 0x03;
            val as u8
        }
        #[doc = "desc EISMPCLK."]
        #[inline(always)]
        pub const fn set_eismpclk(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 4usize)) | (((val as u32) & 0x03) << 4usize);
        }
        #[doc = "desc EFEN."]
        #[must_use]
        #[inline(always)]
        pub const fn efen(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "desc EFEN."]
        #[inline(always)]
        pub const fn set_efen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "desc NOCEN."]
        #[must_use]
        #[inline(always)]
        pub const fn nocen(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "desc NOCEN."]
        #[inline(always)]
        pub const fn set_nocen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
        #[doc = "desc NFSEL."]
        #[must_use]
        #[inline(always)]
        pub const fn nfsel(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0xff;
            val as u8
        }
        #[doc = "desc NFSEL."]
        #[inline(always)]
        pub const fn set_nfsel(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 16usize)) | (((val as u32) & 0xff) << 16usize);
        }
    }
    impl Default for Eirqcr3 {
        #[inline(always)]
        fn default() -> Eirqcr3 {
            Eirqcr3(0)
        }
    }
    impl core::fmt::Debug for Eirqcr3 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Eirqcr3")
                .field("eirqtrg", &self.eirqtrg())
                .field("eismpclk", &self.eismpclk())
                .field("efen", &self.efen())
                .field("nocen", &self.nocen())
                .field("nfsel", &self.nfsel())
                .finish()
        }
    }
    #[doc = "desc EIRQCR4."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Eirqcr4(pub u32);
    impl Eirqcr4 {
        #[doc = "desc EIRQTRG."]
        #[must_use]
        #[inline(always)]
        pub const fn eirqtrg(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x03;
            val as u8
        }
        #[doc = "desc EIRQTRG."]
        #[inline(always)]
        pub const fn set_eirqtrg(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 0usize)) | (((val as u32) & 0x03) << 0usize);
        }
        #[doc = "desc EISMPCLK."]
        #[must_use]
        #[inline(always)]
        pub const fn eismpclk(&self) -> u8 {
            let val = (self.0 >> 4usize) & 0x03;
            val as u8
        }
        #[doc = "desc EISMPCLK."]
        #[inline(always)]
        pub const fn set_eismpclk(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 4usize)) | (((val as u32) & 0x03) << 4usize);
        }
        #[doc = "desc EFEN."]
        #[must_use]
        #[inline(always)]
        pub const fn efen(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "desc EFEN."]
        #[inline(always)]
        pub const fn set_efen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "desc NOCEN."]
        #[must_use]
        #[inline(always)]
        pub const fn nocen(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "desc NOCEN."]
        #[inline(always)]
        pub const fn set_nocen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
        #[doc = "desc NFSEL."]
        #[must_use]
        #[inline(always)]
        pub const fn nfsel(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0xff;
            val as u8
        }
        #[doc = "desc NFSEL."]
        #[inline(always)]
        pub const fn set_nfsel(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 16usize)) | (((val as u32) & 0xff) << 16usize);
        }
    }
    impl Default for Eirqcr4 {
        #[inline(always)]
        fn default() -> Eirqcr4 {
            Eirqcr4(0)
        }
    }
    impl core::fmt::Debug for Eirqcr4 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Eirqcr4")
                .field("eirqtrg", &self.eirqtrg())
                .field("eismpclk", &self.eismpclk())
                .field("efen", &self.efen())
                .field("nocen", &self.nocen())
                .field("nfsel", &self.nfsel())
                .finish()
        }
    }
    #[doc = "desc EIRQCR5."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Eirqcr5(pub u32);
    impl Eirqcr5 {
        #[doc = "desc EIRQTRG."]
        #[must_use]
        #[inline(always)]
        pub const fn eirqtrg(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x03;
            val as u8
        }
        #[doc = "desc EIRQTRG."]
        #[inline(always)]
        pub const fn set_eirqtrg(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 0usize)) | (((val as u32) & 0x03) << 0usize);
        }
        #[doc = "desc EISMPCLK."]
        #[must_use]
        #[inline(always)]
        pub const fn eismpclk(&self) -> u8 {
            let val = (self.0 >> 4usize) & 0x03;
            val as u8
        }
        #[doc = "desc EISMPCLK."]
        #[inline(always)]
        pub const fn set_eismpclk(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 4usize)) | (((val as u32) & 0x03) << 4usize);
        }
        #[doc = "desc EFEN."]
        #[must_use]
        #[inline(always)]
        pub const fn efen(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "desc EFEN."]
        #[inline(always)]
        pub const fn set_efen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "desc NOCEN."]
        #[must_use]
        #[inline(always)]
        pub const fn nocen(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "desc NOCEN."]
        #[inline(always)]
        pub const fn set_nocen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
        #[doc = "desc NFSEL."]
        #[must_use]
        #[inline(always)]
        pub const fn nfsel(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0xff;
            val as u8
        }
        #[doc = "desc NFSEL."]
        #[inline(always)]
        pub const fn set_nfsel(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 16usize)) | (((val as u32) & 0xff) << 16usize);
        }
    }
    impl Default for Eirqcr5 {
        #[inline(always)]
        fn default() -> Eirqcr5 {
            Eirqcr5(0)
        }
    }
    impl core::fmt::Debug for Eirqcr5 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Eirqcr5")
                .field("eirqtrg", &self.eirqtrg())
                .field("eismpclk", &self.eismpclk())
                .field("efen", &self.efen())
                .field("nocen", &self.nocen())
                .field("nfsel", &self.nfsel())
                .finish()
        }
    }
    #[doc = "desc EIRQCR6."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Eirqcr6(pub u32);
    impl Eirqcr6 {
        #[doc = "desc EIRQTRG."]
        #[must_use]
        #[inline(always)]
        pub const fn eirqtrg(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x03;
            val as u8
        }
        #[doc = "desc EIRQTRG."]
        #[inline(always)]
        pub const fn set_eirqtrg(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 0usize)) | (((val as u32) & 0x03) << 0usize);
        }
        #[doc = "desc EISMPCLK."]
        #[must_use]
        #[inline(always)]
        pub const fn eismpclk(&self) -> u8 {
            let val = (self.0 >> 4usize) & 0x03;
            val as u8
        }
        #[doc = "desc EISMPCLK."]
        #[inline(always)]
        pub const fn set_eismpclk(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 4usize)) | (((val as u32) & 0x03) << 4usize);
        }
        #[doc = "desc EFEN."]
        #[must_use]
        #[inline(always)]
        pub const fn efen(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "desc EFEN."]
        #[inline(always)]
        pub const fn set_efen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "desc NOCEN."]
        #[must_use]
        #[inline(always)]
        pub const fn nocen(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "desc NOCEN."]
        #[inline(always)]
        pub const fn set_nocen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
        #[doc = "desc NFSEL."]
        #[must_use]
        #[inline(always)]
        pub const fn nfsel(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0xff;
            val as u8
        }
        #[doc = "desc NFSEL."]
        #[inline(always)]
        pub const fn set_nfsel(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 16usize)) | (((val as u32) & 0xff) << 16usize);
        }
    }
    impl Default for Eirqcr6 {
        #[inline(always)]
        fn default() -> Eirqcr6 {
            Eirqcr6(0)
        }
    }
    impl core::fmt::Debug for Eirqcr6 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Eirqcr6")
                .field("eirqtrg", &self.eirqtrg())
                .field("eismpclk", &self.eismpclk())
                .field("efen", &self.efen())
                .field("nocen", &self.nocen())
                .field("nfsel", &self.nfsel())
                .finish()
        }
    }
    #[doc = "desc EIRQCR7."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Eirqcr7(pub u32);
    impl Eirqcr7 {
        #[doc = "desc EIRQTRG."]
        #[must_use]
        #[inline(always)]
        pub const fn eirqtrg(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x03;
            val as u8
        }
        #[doc = "desc EIRQTRG."]
        #[inline(always)]
        pub const fn set_eirqtrg(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 0usize)) | (((val as u32) & 0x03) << 0usize);
        }
        #[doc = "desc EISMPCLK."]
        #[must_use]
        #[inline(always)]
        pub const fn eismpclk(&self) -> u8 {
            let val = (self.0 >> 4usize) & 0x03;
            val as u8
        }
        #[doc = "desc EISMPCLK."]
        #[inline(always)]
        pub const fn set_eismpclk(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 4usize)) | (((val as u32) & 0x03) << 4usize);
        }
        #[doc = "desc EFEN."]
        #[must_use]
        #[inline(always)]
        pub const fn efen(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "desc EFEN."]
        #[inline(always)]
        pub const fn set_efen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "desc NOCEN."]
        #[must_use]
        #[inline(always)]
        pub const fn nocen(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "desc NOCEN."]
        #[inline(always)]
        pub const fn set_nocen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
        #[doc = "desc NFSEL."]
        #[must_use]
        #[inline(always)]
        pub const fn nfsel(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0xff;
            val as u8
        }
        #[doc = "desc NFSEL."]
        #[inline(always)]
        pub const fn set_nfsel(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 16usize)) | (((val as u32) & 0xff) << 16usize);
        }
    }
    impl Default for Eirqcr7 {
        #[inline(always)]
        fn default() -> Eirqcr7 {
            Eirqcr7(0)
        }
    }
    impl core::fmt::Debug for Eirqcr7 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Eirqcr7")
                .field("eirqtrg", &self.eirqtrg())
                .field("eismpclk", &self.eismpclk())
                .field("efen", &self.efen())
                .field("nocen", &self.nocen())
                .field("nfsel", &self.nfsel())
                .finish()
        }
    }
    #[doc = "desc EIRQCR8."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Eirqcr8(pub u32);
    impl Eirqcr8 {
        #[doc = "desc EIRQTRG."]
        #[must_use]
        #[inline(always)]
        pub const fn eirqtrg(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x03;
            val as u8
        }
        #[doc = "desc EIRQTRG."]
        #[inline(always)]
        pub const fn set_eirqtrg(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 0usize)) | (((val as u32) & 0x03) << 0usize);
        }
        #[doc = "desc EISMPCLK."]
        #[must_use]
        #[inline(always)]
        pub const fn eismpclk(&self) -> u8 {
            let val = (self.0 >> 4usize) & 0x03;
            val as u8
        }
        #[doc = "desc EISMPCLK."]
        #[inline(always)]
        pub const fn set_eismpclk(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 4usize)) | (((val as u32) & 0x03) << 4usize);
        }
        #[doc = "desc EFEN."]
        #[must_use]
        #[inline(always)]
        pub const fn efen(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "desc EFEN."]
        #[inline(always)]
        pub const fn set_efen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "desc NOCEN."]
        #[must_use]
        #[inline(always)]
        pub const fn nocen(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "desc NOCEN."]
        #[inline(always)]
        pub const fn set_nocen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
        #[doc = "desc NFSEL."]
        #[must_use]
        #[inline(always)]
        pub const fn nfsel(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0xff;
            val as u8
        }
        #[doc = "desc NFSEL."]
        #[inline(always)]
        pub const fn set_nfsel(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 16usize)) | (((val as u32) & 0xff) << 16usize);
        }
    }
    impl Default for Eirqcr8 {
        #[inline(always)]
        fn default() -> Eirqcr8 {
            Eirqcr8(0)
        }
    }
    impl core::fmt::Debug for Eirqcr8 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Eirqcr8")
                .field("eirqtrg", &self.eirqtrg())
                .field("eismpclk", &self.eismpclk())
                .field("efen", &self.efen())
                .field("nocen", &self.nocen())
                .field("nfsel", &self.nfsel())
                .finish()
        }
    }
    #[doc = "desc EIRQCR9."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Eirqcr9(pub u32);
    impl Eirqcr9 {
        #[doc = "desc EIRQTRG."]
        #[must_use]
        #[inline(always)]
        pub const fn eirqtrg(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x03;
            val as u8
        }
        #[doc = "desc EIRQTRG."]
        #[inline(always)]
        pub const fn set_eirqtrg(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 0usize)) | (((val as u32) & 0x03) << 0usize);
        }
        #[doc = "desc EISMPCLK."]
        #[must_use]
        #[inline(always)]
        pub const fn eismpclk(&self) -> u8 {
            let val = (self.0 >> 4usize) & 0x03;
            val as u8
        }
        #[doc = "desc EISMPCLK."]
        #[inline(always)]
        pub const fn set_eismpclk(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 4usize)) | (((val as u32) & 0x03) << 4usize);
        }
        #[doc = "desc EFEN."]
        #[must_use]
        #[inline(always)]
        pub const fn efen(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "desc EFEN."]
        #[inline(always)]
        pub const fn set_efen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "desc NOCEN."]
        #[must_use]
        #[inline(always)]
        pub const fn nocen(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "desc NOCEN."]
        #[inline(always)]
        pub const fn set_nocen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
        #[doc = "desc NFSEL."]
        #[must_use]
        #[inline(always)]
        pub const fn nfsel(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0xff;
            val as u8
        }
        #[doc = "desc NFSEL."]
        #[inline(always)]
        pub const fn set_nfsel(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 16usize)) | (((val as u32) & 0xff) << 16usize);
        }
    }
    impl Default for Eirqcr9 {
        #[inline(always)]
        fn default() -> Eirqcr9 {
            Eirqcr9(0)
        }
    }
    impl core::fmt::Debug for Eirqcr9 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Eirqcr9")
                .field("eirqtrg", &self.eirqtrg())
                .field("eismpclk", &self.eismpclk())
                .field("efen", &self.efen())
                .field("nocen", &self.nocen())
                .field("nfsel", &self.nfsel())
                .finish()
        }
    }
    #[doc = "desc EVTER."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Evter(pub u32);
    impl Evter {
        #[doc = "desc EVTE0."]
        #[must_use]
        #[inline(always)]
        pub const fn evte0(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "desc EVTE0."]
        #[inline(always)]
        pub const fn set_evte0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "desc EVTE1."]
        #[must_use]
        #[inline(always)]
        pub const fn evte1(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "desc EVTE1."]
        #[inline(always)]
        pub const fn set_evte1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "desc EVTE2."]
        #[must_use]
        #[inline(always)]
        pub const fn evte2(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "desc EVTE2."]
        #[inline(always)]
        pub const fn set_evte2(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "desc EVTE3."]
        #[must_use]
        #[inline(always)]
        pub const fn evte3(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "desc EVTE3."]
        #[inline(always)]
        pub const fn set_evte3(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "desc EVTE4."]
        #[must_use]
        #[inline(always)]
        pub const fn evte4(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "desc EVTE4."]
        #[inline(always)]
        pub const fn set_evte4(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "desc EVTE5."]
        #[must_use]
        #[inline(always)]
        pub const fn evte5(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "desc EVTE5."]
        #[inline(always)]
        pub const fn set_evte5(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "desc EVTE6."]
        #[must_use]
        #[inline(always)]
        pub const fn evte6(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "desc EVTE6."]
        #[inline(always)]
        pub const fn set_evte6(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "desc EVTE7."]
        #[must_use]
        #[inline(always)]
        pub const fn evte7(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "desc EVTE7."]
        #[inline(always)]
        pub const fn set_evte7(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "desc EVTE8."]
        #[must_use]
        #[inline(always)]
        pub const fn evte8(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "desc EVTE8."]
        #[inline(always)]
        pub const fn set_evte8(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "desc EVTE9."]
        #[must_use]
        #[inline(always)]
        pub const fn evte9(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "desc EVTE9."]
        #[inline(always)]
        pub const fn set_evte9(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "desc EVTE10."]
        #[must_use]
        #[inline(always)]
        pub const fn evte10(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "desc EVTE10."]
        #[inline(always)]
        pub const fn set_evte10(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[doc = "desc EVTE11."]
        #[must_use]
        #[inline(always)]
        pub const fn evte11(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "desc EVTE11."]
        #[inline(always)]
        pub const fn set_evte11(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[doc = "desc EVTE12."]
        #[must_use]
        #[inline(always)]
        pub const fn evte12(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "desc EVTE12."]
        #[inline(always)]
        pub const fn set_evte12(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "desc EVTE13."]
        #[must_use]
        #[inline(always)]
        pub const fn evte13(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[doc = "desc EVTE13."]
        #[inline(always)]
        pub const fn set_evte13(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[doc = "desc EVTE14."]
        #[must_use]
        #[inline(always)]
        pub const fn evte14(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "desc EVTE14."]
        #[inline(always)]
        pub const fn set_evte14(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[doc = "desc EVTE15."]
        #[must_use]
        #[inline(always)]
        pub const fn evte15(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "desc EVTE15."]
        #[inline(always)]
        pub const fn set_evte15(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
        #[doc = "desc EVTE16."]
        #[must_use]
        #[inline(always)]
        pub const fn evte16(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "desc EVTE16."]
        #[inline(always)]
        pub const fn set_evte16(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "desc EVTE17."]
        #[must_use]
        #[inline(always)]
        pub const fn evte17(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "desc EVTE17."]
        #[inline(always)]
        pub const fn set_evte17(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[doc = "desc EVTE18."]
        #[must_use]
        #[inline(always)]
        pub const fn evte18(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[doc = "desc EVTE18."]
        #[inline(always)]
        pub const fn set_evte18(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
        #[doc = "desc EVTE19."]
        #[must_use]
        #[inline(always)]
        pub const fn evte19(&self) -> bool {
            let val = (self.0 >> 19usize) & 0x01;
            val != 0
        }
        #[doc = "desc EVTE19."]
        #[inline(always)]
        pub const fn set_evte19(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
        }
        #[doc = "desc EVTE20."]
        #[must_use]
        #[inline(always)]
        pub const fn evte20(&self) -> bool {
            let val = (self.0 >> 20usize) & 0x01;
            val != 0
        }
        #[doc = "desc EVTE20."]
        #[inline(always)]
        pub const fn set_evte20(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
        }
        #[doc = "desc EVTE21."]
        #[must_use]
        #[inline(always)]
        pub const fn evte21(&self) -> bool {
            let val = (self.0 >> 21usize) & 0x01;
            val != 0
        }
        #[doc = "desc EVTE21."]
        #[inline(always)]
        pub const fn set_evte21(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
        }
        #[doc = "desc EVTE22."]
        #[must_use]
        #[inline(always)]
        pub const fn evte22(&self) -> bool {
            let val = (self.0 >> 22usize) & 0x01;
            val != 0
        }
        #[doc = "desc EVTE22."]
        #[inline(always)]
        pub const fn set_evte22(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
        }
        #[doc = "desc EVTE23."]
        #[must_use]
        #[inline(always)]
        pub const fn evte23(&self) -> bool {
            let val = (self.0 >> 23usize) & 0x01;
            val != 0
        }
        #[doc = "desc EVTE23."]
        #[inline(always)]
        pub const fn set_evte23(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
        }
        #[doc = "desc EVTE24."]
        #[must_use]
        #[inline(always)]
        pub const fn evte24(&self) -> bool {
            let val = (self.0 >> 24usize) & 0x01;
            val != 0
        }
        #[doc = "desc EVTE24."]
        #[inline(always)]
        pub const fn set_evte24(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
        }
        #[doc = "desc EVTE25."]
        #[must_use]
        #[inline(always)]
        pub const fn evte25(&self) -> bool {
            let val = (self.0 >> 25usize) & 0x01;
            val != 0
        }
        #[doc = "desc EVTE25."]
        #[inline(always)]
        pub const fn set_evte25(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
        }
        #[doc = "desc EVTE26."]
        #[must_use]
        #[inline(always)]
        pub const fn evte26(&self) -> bool {
            let val = (self.0 >> 26usize) & 0x01;
            val != 0
        }
        #[doc = "desc EVTE26."]
        #[inline(always)]
        pub const fn set_evte26(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
        }
        #[doc = "desc EVTE27."]
        #[must_use]
        #[inline(always)]
        pub const fn evte27(&self) -> bool {
            let val = (self.0 >> 27usize) & 0x01;
            val != 0
        }
        #[doc = "desc EVTE27."]
        #[inline(always)]
        pub const fn set_evte27(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
        }
        #[doc = "desc EVTE28."]
        #[must_use]
        #[inline(always)]
        pub const fn evte28(&self) -> bool {
            let val = (self.0 >> 28usize) & 0x01;
            val != 0
        }
        #[doc = "desc EVTE28."]
        #[inline(always)]
        pub const fn set_evte28(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
        }
        #[doc = "desc EVTE29."]
        #[must_use]
        #[inline(always)]
        pub const fn evte29(&self) -> bool {
            let val = (self.0 >> 29usize) & 0x01;
            val != 0
        }
        #[doc = "desc EVTE29."]
        #[inline(always)]
        pub const fn set_evte29(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
        }
        #[doc = "desc EVTE30."]
        #[must_use]
        #[inline(always)]
        pub const fn evte30(&self) -> bool {
            let val = (self.0 >> 30usize) & 0x01;
            val != 0
        }
        #[doc = "desc EVTE30."]
        #[inline(always)]
        pub const fn set_evte30(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
        }
        #[doc = "desc EVTE31."]
        #[must_use]
        #[inline(always)]
        pub const fn evte31(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[doc = "desc EVTE31."]
        #[inline(always)]
        pub const fn set_evte31(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for Evter {
        #[inline(always)]
        fn default() -> Evter {
            Evter(0)
        }
    }
    impl core::fmt::Debug for Evter {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Evter")
                .field("evte0", &self.evte0())
                .field("evte1", &self.evte1())
                .field("evte2", &self.evte2())
                .field("evte3", &self.evte3())
                .field("evte4", &self.evte4())
                .field("evte5", &self.evte5())
                .field("evte6", &self.evte6())
                .field("evte7", &self.evte7())
                .field("evte8", &self.evte8())
                .field("evte9", &self.evte9())
                .field("evte10", &self.evte10())
                .field("evte11", &self.evte11())
                .field("evte12", &self.evte12())
                .field("evte13", &self.evte13())
                .field("evte14", &self.evte14())
                .field("evte15", &self.evte15())
                .field("evte16", &self.evte16())
                .field("evte17", &self.evte17())
                .field("evte18", &self.evte18())
                .field("evte19", &self.evte19())
                .field("evte20", &self.evte20())
                .field("evte21", &self.evte21())
                .field("evte22", &self.evte22())
                .field("evte23", &self.evte23())
                .field("evte24", &self.evte24())
                .field("evte25", &self.evte25())
                .field("evte26", &self.evte26())
                .field("evte27", &self.evte27())
                .field("evte28", &self.evte28())
                .field("evte29", &self.evte29())
                .field("evte30", &self.evte30())
                .field("evte31", &self.evte31())
                .finish()
        }
    }
    #[doc = "desc FPUIER."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Fpuier(pub u32);
    impl Fpuier {
        #[doc = "desc IOCIEN."]
        #[must_use]
        #[inline(always)]
        pub const fn iocien(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "desc IOCIEN."]
        #[inline(always)]
        pub const fn set_iocien(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "desc DZCIEN."]
        #[must_use]
        #[inline(always)]
        pub const fn dzcien(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "desc DZCIEN."]
        #[inline(always)]
        pub const fn set_dzcien(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "desc OFCIEN."]
        #[must_use]
        #[inline(always)]
        pub const fn ofcien(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "desc OFCIEN."]
        #[inline(always)]
        pub const fn set_ofcien(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "desc UFCIEN."]
        #[must_use]
        #[inline(always)]
        pub const fn ufcien(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "desc UFCIEN."]
        #[inline(always)]
        pub const fn set_ufcien(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "desc IXCIEN."]
        #[must_use]
        #[inline(always)]
        pub const fn ixcien(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "desc IXCIEN."]
        #[inline(always)]
        pub const fn set_ixcien(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "desc IDCIEN."]
        #[must_use]
        #[inline(always)]
        pub const fn idcien(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "desc IDCIEN."]
        #[inline(always)]
        pub const fn set_idcien(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
    }
    impl Default for Fpuier {
        #[inline(always)]
        fn default() -> Fpuier {
            Fpuier(0)
        }
    }
    impl core::fmt::Debug for Fpuier {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Fpuier")
                .field("iocien", &self.iocien())
                .field("dzcien", &self.dzcien())
                .field("ofcien", &self.ofcien())
                .field("ufcien", &self.ufcien())
                .field("ixcien", &self.ixcien())
                .field("idcien", &self.idcien())
                .finish()
        }
    }
    #[doc = "desc IER."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ier(pub u32);
    impl Ier {
        #[doc = "desc IER0."]
        #[must_use]
        #[inline(always)]
        pub const fn ier0(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "desc IER0."]
        #[inline(always)]
        pub const fn set_ier0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "desc IER1."]
        #[must_use]
        #[inline(always)]
        pub const fn ier1(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "desc IER1."]
        #[inline(always)]
        pub const fn set_ier1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "desc IER2."]
        #[must_use]
        #[inline(always)]
        pub const fn ier2(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "desc IER2."]
        #[inline(always)]
        pub const fn set_ier2(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "desc IER3."]
        #[must_use]
        #[inline(always)]
        pub const fn ier3(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "desc IER3."]
        #[inline(always)]
        pub const fn set_ier3(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "desc IER4."]
        #[must_use]
        #[inline(always)]
        pub const fn ier4(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "desc IER4."]
        #[inline(always)]
        pub const fn set_ier4(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "desc IER5."]
        #[must_use]
        #[inline(always)]
        pub const fn ier5(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "desc IER5."]
        #[inline(always)]
        pub const fn set_ier5(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "desc IER6."]
        #[must_use]
        #[inline(always)]
        pub const fn ier6(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "desc IER6."]
        #[inline(always)]
        pub const fn set_ier6(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "desc IER7."]
        #[must_use]
        #[inline(always)]
        pub const fn ier7(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "desc IER7."]
        #[inline(always)]
        pub const fn set_ier7(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "desc IER8."]
        #[must_use]
        #[inline(always)]
        pub const fn ier8(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "desc IER8."]
        #[inline(always)]
        pub const fn set_ier8(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "desc IER9."]
        #[must_use]
        #[inline(always)]
        pub const fn ier9(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "desc IER9."]
        #[inline(always)]
        pub const fn set_ier9(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "desc IER10."]
        #[must_use]
        #[inline(always)]
        pub const fn ier10(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "desc IER10."]
        #[inline(always)]
        pub const fn set_ier10(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[doc = "desc IER11."]
        #[must_use]
        #[inline(always)]
        pub const fn ier11(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "desc IER11."]
        #[inline(always)]
        pub const fn set_ier11(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[doc = "desc IER12."]
        #[must_use]
        #[inline(always)]
        pub const fn ier12(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "desc IER12."]
        #[inline(always)]
        pub const fn set_ier12(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "desc IER13."]
        #[must_use]
        #[inline(always)]
        pub const fn ier13(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[doc = "desc IER13."]
        #[inline(always)]
        pub const fn set_ier13(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[doc = "desc IER14."]
        #[must_use]
        #[inline(always)]
        pub const fn ier14(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "desc IER14."]
        #[inline(always)]
        pub const fn set_ier14(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[doc = "desc IER15."]
        #[must_use]
        #[inline(always)]
        pub const fn ier15(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "desc IER15."]
        #[inline(always)]
        pub const fn set_ier15(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
        #[doc = "desc IER16."]
        #[must_use]
        #[inline(always)]
        pub const fn ier16(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "desc IER16."]
        #[inline(always)]
        pub const fn set_ier16(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "desc IER17."]
        #[must_use]
        #[inline(always)]
        pub const fn ier17(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "desc IER17."]
        #[inline(always)]
        pub const fn set_ier17(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[doc = "desc IER18."]
        #[must_use]
        #[inline(always)]
        pub const fn ier18(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[doc = "desc IER18."]
        #[inline(always)]
        pub const fn set_ier18(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
        #[doc = "desc IER19."]
        #[must_use]
        #[inline(always)]
        pub const fn ier19(&self) -> bool {
            let val = (self.0 >> 19usize) & 0x01;
            val != 0
        }
        #[doc = "desc IER19."]
        #[inline(always)]
        pub const fn set_ier19(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
        }
        #[doc = "desc IER20."]
        #[must_use]
        #[inline(always)]
        pub const fn ier20(&self) -> bool {
            let val = (self.0 >> 20usize) & 0x01;
            val != 0
        }
        #[doc = "desc IER20."]
        #[inline(always)]
        pub const fn set_ier20(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
        }
        #[doc = "desc IER21."]
        #[must_use]
        #[inline(always)]
        pub const fn ier21(&self) -> bool {
            let val = (self.0 >> 21usize) & 0x01;
            val != 0
        }
        #[doc = "desc IER21."]
        #[inline(always)]
        pub const fn set_ier21(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
        }
        #[doc = "desc IER22."]
        #[must_use]
        #[inline(always)]
        pub const fn ier22(&self) -> bool {
            let val = (self.0 >> 22usize) & 0x01;
            val != 0
        }
        #[doc = "desc IER22."]
        #[inline(always)]
        pub const fn set_ier22(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
        }
        #[doc = "desc IER23."]
        #[must_use]
        #[inline(always)]
        pub const fn ier23(&self) -> bool {
            let val = (self.0 >> 23usize) & 0x01;
            val != 0
        }
        #[doc = "desc IER23."]
        #[inline(always)]
        pub const fn set_ier23(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
        }
        #[doc = "desc IER24."]
        #[must_use]
        #[inline(always)]
        pub const fn ier24(&self) -> bool {
            let val = (self.0 >> 24usize) & 0x01;
            val != 0
        }
        #[doc = "desc IER24."]
        #[inline(always)]
        pub const fn set_ier24(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
        }
        #[doc = "desc IER25."]
        #[must_use]
        #[inline(always)]
        pub const fn ier25(&self) -> bool {
            let val = (self.0 >> 25usize) & 0x01;
            val != 0
        }
        #[doc = "desc IER25."]
        #[inline(always)]
        pub const fn set_ier25(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
        }
        #[doc = "desc IER26."]
        #[must_use]
        #[inline(always)]
        pub const fn ier26(&self) -> bool {
            let val = (self.0 >> 26usize) & 0x01;
            val != 0
        }
        #[doc = "desc IER26."]
        #[inline(always)]
        pub const fn set_ier26(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
        }
        #[doc = "desc IER27."]
        #[must_use]
        #[inline(always)]
        pub const fn ier27(&self) -> bool {
            let val = (self.0 >> 27usize) & 0x01;
            val != 0
        }
        #[doc = "desc IER27."]
        #[inline(always)]
        pub const fn set_ier27(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
        }
        #[doc = "desc IER28."]
        #[must_use]
        #[inline(always)]
        pub const fn ier28(&self) -> bool {
            let val = (self.0 >> 28usize) & 0x01;
            val != 0
        }
        #[doc = "desc IER28."]
        #[inline(always)]
        pub const fn set_ier28(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
        }
        #[doc = "desc IER29."]
        #[must_use]
        #[inline(always)]
        pub const fn ier29(&self) -> bool {
            let val = (self.0 >> 29usize) & 0x01;
            val != 0
        }
        #[doc = "desc IER29."]
        #[inline(always)]
        pub const fn set_ier29(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
        }
        #[doc = "desc IER30."]
        #[must_use]
        #[inline(always)]
        pub const fn ier30(&self) -> bool {
            let val = (self.0 >> 30usize) & 0x01;
            val != 0
        }
        #[doc = "desc IER30."]
        #[inline(always)]
        pub const fn set_ier30(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
        }
        #[doc = "desc IER31."]
        #[must_use]
        #[inline(always)]
        pub const fn ier31(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[doc = "desc IER31."]
        #[inline(always)]
        pub const fn set_ier31(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
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
                .field("ier0", &self.ier0())
                .field("ier1", &self.ier1())
                .field("ier2", &self.ier2())
                .field("ier3", &self.ier3())
                .field("ier4", &self.ier4())
                .field("ier5", &self.ier5())
                .field("ier6", &self.ier6())
                .field("ier7", &self.ier7())
                .field("ier8", &self.ier8())
                .field("ier9", &self.ier9())
                .field("ier10", &self.ier10())
                .field("ier11", &self.ier11())
                .field("ier12", &self.ier12())
                .field("ier13", &self.ier13())
                .field("ier14", &self.ier14())
                .field("ier15", &self.ier15())
                .field("ier16", &self.ier16())
                .field("ier17", &self.ier17())
                .field("ier18", &self.ier18())
                .field("ier19", &self.ier19())
                .field("ier20", &self.ier20())
                .field("ier21", &self.ier21())
                .field("ier22", &self.ier22())
                .field("ier23", &self.ier23())
                .field("ier24", &self.ier24())
                .field("ier25", &self.ier25())
                .field("ier26", &self.ier26())
                .field("ier27", &self.ier27())
                .field("ier28", &self.ier28())
                .field("ier29", &self.ier29())
                .field("ier30", &self.ier30())
                .field("ier31", &self.ier31())
                .finish()
        }
    }
    #[doc = "desc INTSEL0."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Intsel0(pub u32);
    impl Intsel0 {
        #[doc = "desc INTSEL."]
        #[must_use]
        #[inline(always)]
        pub const fn intsel(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x01ff;
            val as u16
        }
        #[doc = "desc INTSEL."]
        #[inline(always)]
        pub const fn set_intsel(&mut self, val: u16) {
            self.0 = (self.0 & !(0x01ff << 0usize)) | (((val as u32) & 0x01ff) << 0usize);
        }
    }
    impl Default for Intsel0 {
        #[inline(always)]
        fn default() -> Intsel0 {
            Intsel0(0)
        }
    }
    impl core::fmt::Debug for Intsel0 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Intsel0")
                .field("intsel", &self.intsel())
                .finish()
        }
    }
    #[doc = "desc INTSEL1."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Intsel1(pub u32);
    impl Intsel1 {
        #[doc = "desc INTSEL."]
        #[must_use]
        #[inline(always)]
        pub const fn intsel(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x01ff;
            val as u16
        }
        #[doc = "desc INTSEL."]
        #[inline(always)]
        pub const fn set_intsel(&mut self, val: u16) {
            self.0 = (self.0 & !(0x01ff << 0usize)) | (((val as u32) & 0x01ff) << 0usize);
        }
    }
    impl Default for Intsel1 {
        #[inline(always)]
        fn default() -> Intsel1 {
            Intsel1(0)
        }
    }
    impl core::fmt::Debug for Intsel1 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Intsel1")
                .field("intsel", &self.intsel())
                .finish()
        }
    }
    #[doc = "desc INTSEL10."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Intsel10(pub u32);
    impl Intsel10 {
        #[doc = "desc INTSEL."]
        #[must_use]
        #[inline(always)]
        pub const fn intsel(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x01ff;
            val as u16
        }
        #[doc = "desc INTSEL."]
        #[inline(always)]
        pub const fn set_intsel(&mut self, val: u16) {
            self.0 = (self.0 & !(0x01ff << 0usize)) | (((val as u32) & 0x01ff) << 0usize);
        }
    }
    impl Default for Intsel10 {
        #[inline(always)]
        fn default() -> Intsel10 {
            Intsel10(0)
        }
    }
    impl core::fmt::Debug for Intsel10 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Intsel10")
                .field("intsel", &self.intsel())
                .finish()
        }
    }
    #[doc = "desc INTSEL100."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Intsel100(pub u32);
    impl Intsel100 {
        #[doc = "desc INTSEL."]
        #[must_use]
        #[inline(always)]
        pub const fn intsel(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x01ff;
            val as u16
        }
        #[doc = "desc INTSEL."]
        #[inline(always)]
        pub const fn set_intsel(&mut self, val: u16) {
            self.0 = (self.0 & !(0x01ff << 0usize)) | (((val as u32) & 0x01ff) << 0usize);
        }
    }
    impl Default for Intsel100 {
        #[inline(always)]
        fn default() -> Intsel100 {
            Intsel100(0)
        }
    }
    impl core::fmt::Debug for Intsel100 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Intsel100")
                .field("intsel", &self.intsel())
                .finish()
        }
    }
    #[doc = "desc INTSEL101."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Intsel101(pub u32);
    impl Intsel101 {
        #[doc = "desc INTSEL."]
        #[must_use]
        #[inline(always)]
        pub const fn intsel(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x01ff;
            val as u16
        }
        #[doc = "desc INTSEL."]
        #[inline(always)]
        pub const fn set_intsel(&mut self, val: u16) {
            self.0 = (self.0 & !(0x01ff << 0usize)) | (((val as u32) & 0x01ff) << 0usize);
        }
    }
    impl Default for Intsel101 {
        #[inline(always)]
        fn default() -> Intsel101 {
            Intsel101(0)
        }
    }
    impl core::fmt::Debug for Intsel101 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Intsel101")
                .field("intsel", &self.intsel())
                .finish()
        }
    }
    #[doc = "desc INTSEL102."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Intsel102(pub u32);
    impl Intsel102 {
        #[doc = "desc INTSEL."]
        #[must_use]
        #[inline(always)]
        pub const fn intsel(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x01ff;
            val as u16
        }
        #[doc = "desc INTSEL."]
        #[inline(always)]
        pub const fn set_intsel(&mut self, val: u16) {
            self.0 = (self.0 & !(0x01ff << 0usize)) | (((val as u32) & 0x01ff) << 0usize);
        }
    }
    impl Default for Intsel102 {
        #[inline(always)]
        fn default() -> Intsel102 {
            Intsel102(0)
        }
    }
    impl core::fmt::Debug for Intsel102 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Intsel102")
                .field("intsel", &self.intsel())
                .finish()
        }
    }
    #[doc = "desc INTSEL103."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Intsel103(pub u32);
    impl Intsel103 {
        #[doc = "desc INTSEL."]
        #[must_use]
        #[inline(always)]
        pub const fn intsel(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x01ff;
            val as u16
        }
        #[doc = "desc INTSEL."]
        #[inline(always)]
        pub const fn set_intsel(&mut self, val: u16) {
            self.0 = (self.0 & !(0x01ff << 0usize)) | (((val as u32) & 0x01ff) << 0usize);
        }
    }
    impl Default for Intsel103 {
        #[inline(always)]
        fn default() -> Intsel103 {
            Intsel103(0)
        }
    }
    impl core::fmt::Debug for Intsel103 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Intsel103")
                .field("intsel", &self.intsel())
                .finish()
        }
    }
    #[doc = "desc INTSEL104."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Intsel104(pub u32);
    impl Intsel104 {
        #[doc = "desc INTSEL."]
        #[must_use]
        #[inline(always)]
        pub const fn intsel(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x01ff;
            val as u16
        }
        #[doc = "desc INTSEL."]
        #[inline(always)]
        pub const fn set_intsel(&mut self, val: u16) {
            self.0 = (self.0 & !(0x01ff << 0usize)) | (((val as u32) & 0x01ff) << 0usize);
        }
    }
    impl Default for Intsel104 {
        #[inline(always)]
        fn default() -> Intsel104 {
            Intsel104(0)
        }
    }
    impl core::fmt::Debug for Intsel104 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Intsel104")
                .field("intsel", &self.intsel())
                .finish()
        }
    }
    #[doc = "desc INTSEL105."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Intsel105(pub u32);
    impl Intsel105 {
        #[doc = "desc INTSEL."]
        #[must_use]
        #[inline(always)]
        pub const fn intsel(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x01ff;
            val as u16
        }
        #[doc = "desc INTSEL."]
        #[inline(always)]
        pub const fn set_intsel(&mut self, val: u16) {
            self.0 = (self.0 & !(0x01ff << 0usize)) | (((val as u32) & 0x01ff) << 0usize);
        }
    }
    impl Default for Intsel105 {
        #[inline(always)]
        fn default() -> Intsel105 {
            Intsel105(0)
        }
    }
    impl core::fmt::Debug for Intsel105 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Intsel105")
                .field("intsel", &self.intsel())
                .finish()
        }
    }
    #[doc = "desc INTSEL106."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Intsel106(pub u32);
    impl Intsel106 {
        #[doc = "desc INTSEL."]
        #[must_use]
        #[inline(always)]
        pub const fn intsel(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x01ff;
            val as u16
        }
        #[doc = "desc INTSEL."]
        #[inline(always)]
        pub const fn set_intsel(&mut self, val: u16) {
            self.0 = (self.0 & !(0x01ff << 0usize)) | (((val as u32) & 0x01ff) << 0usize);
        }
    }
    impl Default for Intsel106 {
        #[inline(always)]
        fn default() -> Intsel106 {
            Intsel106(0)
        }
    }
    impl core::fmt::Debug for Intsel106 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Intsel106")
                .field("intsel", &self.intsel())
                .finish()
        }
    }
    #[doc = "desc INTSEL107."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Intsel107(pub u32);
    impl Intsel107 {
        #[doc = "desc INTSEL."]
        #[must_use]
        #[inline(always)]
        pub const fn intsel(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x01ff;
            val as u16
        }
        #[doc = "desc INTSEL."]
        #[inline(always)]
        pub const fn set_intsel(&mut self, val: u16) {
            self.0 = (self.0 & !(0x01ff << 0usize)) | (((val as u32) & 0x01ff) << 0usize);
        }
    }
    impl Default for Intsel107 {
        #[inline(always)]
        fn default() -> Intsel107 {
            Intsel107(0)
        }
    }
    impl core::fmt::Debug for Intsel107 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Intsel107")
                .field("intsel", &self.intsel())
                .finish()
        }
    }
    #[doc = "desc INTSEL108."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Intsel108(pub u32);
    impl Intsel108 {
        #[doc = "desc INTSEL."]
        #[must_use]
        #[inline(always)]
        pub const fn intsel(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x01ff;
            val as u16
        }
        #[doc = "desc INTSEL."]
        #[inline(always)]
        pub const fn set_intsel(&mut self, val: u16) {
            self.0 = (self.0 & !(0x01ff << 0usize)) | (((val as u32) & 0x01ff) << 0usize);
        }
    }
    impl Default for Intsel108 {
        #[inline(always)]
        fn default() -> Intsel108 {
            Intsel108(0)
        }
    }
    impl core::fmt::Debug for Intsel108 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Intsel108")
                .field("intsel", &self.intsel())
                .finish()
        }
    }
    #[doc = "desc INTSEL109."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Intsel109(pub u32);
    impl Intsel109 {
        #[doc = "desc INTSEL."]
        #[must_use]
        #[inline(always)]
        pub const fn intsel(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x01ff;
            val as u16
        }
        #[doc = "desc INTSEL."]
        #[inline(always)]
        pub const fn set_intsel(&mut self, val: u16) {
            self.0 = (self.0 & !(0x01ff << 0usize)) | (((val as u32) & 0x01ff) << 0usize);
        }
    }
    impl Default for Intsel109 {
        #[inline(always)]
        fn default() -> Intsel109 {
            Intsel109(0)
        }
    }
    impl core::fmt::Debug for Intsel109 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Intsel109")
                .field("intsel", &self.intsel())
                .finish()
        }
    }
    #[doc = "desc INTSEL11."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Intsel11(pub u32);
    impl Intsel11 {
        #[doc = "desc INTSEL."]
        #[must_use]
        #[inline(always)]
        pub const fn intsel(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x01ff;
            val as u16
        }
        #[doc = "desc INTSEL."]
        #[inline(always)]
        pub const fn set_intsel(&mut self, val: u16) {
            self.0 = (self.0 & !(0x01ff << 0usize)) | (((val as u32) & 0x01ff) << 0usize);
        }
    }
    impl Default for Intsel11 {
        #[inline(always)]
        fn default() -> Intsel11 {
            Intsel11(0)
        }
    }
    impl core::fmt::Debug for Intsel11 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Intsel11")
                .field("intsel", &self.intsel())
                .finish()
        }
    }
    #[doc = "desc INTSEL110."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Intsel110(pub u32);
    impl Intsel110 {
        #[doc = "desc INTSEL."]
        #[must_use]
        #[inline(always)]
        pub const fn intsel(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x01ff;
            val as u16
        }
        #[doc = "desc INTSEL."]
        #[inline(always)]
        pub const fn set_intsel(&mut self, val: u16) {
            self.0 = (self.0 & !(0x01ff << 0usize)) | (((val as u32) & 0x01ff) << 0usize);
        }
    }
    impl Default for Intsel110 {
        #[inline(always)]
        fn default() -> Intsel110 {
            Intsel110(0)
        }
    }
    impl core::fmt::Debug for Intsel110 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Intsel110")
                .field("intsel", &self.intsel())
                .finish()
        }
    }
    #[doc = "desc INTSEL111."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Intsel111(pub u32);
    impl Intsel111 {
        #[doc = "desc INTSEL."]
        #[must_use]
        #[inline(always)]
        pub const fn intsel(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x01ff;
            val as u16
        }
        #[doc = "desc INTSEL."]
        #[inline(always)]
        pub const fn set_intsel(&mut self, val: u16) {
            self.0 = (self.0 & !(0x01ff << 0usize)) | (((val as u32) & 0x01ff) << 0usize);
        }
    }
    impl Default for Intsel111 {
        #[inline(always)]
        fn default() -> Intsel111 {
            Intsel111(0)
        }
    }
    impl core::fmt::Debug for Intsel111 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Intsel111")
                .field("intsel", &self.intsel())
                .finish()
        }
    }
    #[doc = "desc INTSEL112."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Intsel112(pub u32);
    impl Intsel112 {
        #[doc = "desc INTSEL."]
        #[must_use]
        #[inline(always)]
        pub const fn intsel(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x01ff;
            val as u16
        }
        #[doc = "desc INTSEL."]
        #[inline(always)]
        pub const fn set_intsel(&mut self, val: u16) {
            self.0 = (self.0 & !(0x01ff << 0usize)) | (((val as u32) & 0x01ff) << 0usize);
        }
    }
    impl Default for Intsel112 {
        #[inline(always)]
        fn default() -> Intsel112 {
            Intsel112(0)
        }
    }
    impl core::fmt::Debug for Intsel112 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Intsel112")
                .field("intsel", &self.intsel())
                .finish()
        }
    }
    #[doc = "desc INTSEL113."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Intsel113(pub u32);
    impl Intsel113 {
        #[doc = "desc INTSEL."]
        #[must_use]
        #[inline(always)]
        pub const fn intsel(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x01ff;
            val as u16
        }
        #[doc = "desc INTSEL."]
        #[inline(always)]
        pub const fn set_intsel(&mut self, val: u16) {
            self.0 = (self.0 & !(0x01ff << 0usize)) | (((val as u32) & 0x01ff) << 0usize);
        }
    }
    impl Default for Intsel113 {
        #[inline(always)]
        fn default() -> Intsel113 {
            Intsel113(0)
        }
    }
    impl core::fmt::Debug for Intsel113 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Intsel113")
                .field("intsel", &self.intsel())
                .finish()
        }
    }
    #[doc = "desc INTSEL114."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Intsel114(pub u32);
    impl Intsel114 {
        #[doc = "desc INTSEL."]
        #[must_use]
        #[inline(always)]
        pub const fn intsel(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x01ff;
            val as u16
        }
        #[doc = "desc INTSEL."]
        #[inline(always)]
        pub const fn set_intsel(&mut self, val: u16) {
            self.0 = (self.0 & !(0x01ff << 0usize)) | (((val as u32) & 0x01ff) << 0usize);
        }
    }
    impl Default for Intsel114 {
        #[inline(always)]
        fn default() -> Intsel114 {
            Intsel114(0)
        }
    }
    impl core::fmt::Debug for Intsel114 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Intsel114")
                .field("intsel", &self.intsel())
                .finish()
        }
    }
    #[doc = "desc INTSEL115."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Intsel115(pub u32);
    impl Intsel115 {
        #[doc = "desc INTSEL."]
        #[must_use]
        #[inline(always)]
        pub const fn intsel(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x01ff;
            val as u16
        }
        #[doc = "desc INTSEL."]
        #[inline(always)]
        pub const fn set_intsel(&mut self, val: u16) {
            self.0 = (self.0 & !(0x01ff << 0usize)) | (((val as u32) & 0x01ff) << 0usize);
        }
    }
    impl Default for Intsel115 {
        #[inline(always)]
        fn default() -> Intsel115 {
            Intsel115(0)
        }
    }
    impl core::fmt::Debug for Intsel115 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Intsel115")
                .field("intsel", &self.intsel())
                .finish()
        }
    }
    #[doc = "desc INTSEL116."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Intsel116(pub u32);
    impl Intsel116 {
        #[doc = "desc INTSEL."]
        #[must_use]
        #[inline(always)]
        pub const fn intsel(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x01ff;
            val as u16
        }
        #[doc = "desc INTSEL."]
        #[inline(always)]
        pub const fn set_intsel(&mut self, val: u16) {
            self.0 = (self.0 & !(0x01ff << 0usize)) | (((val as u32) & 0x01ff) << 0usize);
        }
    }
    impl Default for Intsel116 {
        #[inline(always)]
        fn default() -> Intsel116 {
            Intsel116(0)
        }
    }
    impl core::fmt::Debug for Intsel116 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Intsel116")
                .field("intsel", &self.intsel())
                .finish()
        }
    }
    #[doc = "desc INTSEL117."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Intsel117(pub u32);
    impl Intsel117 {
        #[doc = "desc INTSEL."]
        #[must_use]
        #[inline(always)]
        pub const fn intsel(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x01ff;
            val as u16
        }
        #[doc = "desc INTSEL."]
        #[inline(always)]
        pub const fn set_intsel(&mut self, val: u16) {
            self.0 = (self.0 & !(0x01ff << 0usize)) | (((val as u32) & 0x01ff) << 0usize);
        }
    }
    impl Default for Intsel117 {
        #[inline(always)]
        fn default() -> Intsel117 {
            Intsel117(0)
        }
    }
    impl core::fmt::Debug for Intsel117 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Intsel117")
                .field("intsel", &self.intsel())
                .finish()
        }
    }
    #[doc = "desc INTSEL118."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Intsel118(pub u32);
    impl Intsel118 {
        #[doc = "desc INTSEL."]
        #[must_use]
        #[inline(always)]
        pub const fn intsel(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x01ff;
            val as u16
        }
        #[doc = "desc INTSEL."]
        #[inline(always)]
        pub const fn set_intsel(&mut self, val: u16) {
            self.0 = (self.0 & !(0x01ff << 0usize)) | (((val as u32) & 0x01ff) << 0usize);
        }
    }
    impl Default for Intsel118 {
        #[inline(always)]
        fn default() -> Intsel118 {
            Intsel118(0)
        }
    }
    impl core::fmt::Debug for Intsel118 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Intsel118")
                .field("intsel", &self.intsel())
                .finish()
        }
    }
    #[doc = "desc INTSEL119."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Intsel119(pub u32);
    impl Intsel119 {
        #[doc = "desc INTSEL."]
        #[must_use]
        #[inline(always)]
        pub const fn intsel(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x01ff;
            val as u16
        }
        #[doc = "desc INTSEL."]
        #[inline(always)]
        pub const fn set_intsel(&mut self, val: u16) {
            self.0 = (self.0 & !(0x01ff << 0usize)) | (((val as u32) & 0x01ff) << 0usize);
        }
    }
    impl Default for Intsel119 {
        #[inline(always)]
        fn default() -> Intsel119 {
            Intsel119(0)
        }
    }
    impl core::fmt::Debug for Intsel119 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Intsel119")
                .field("intsel", &self.intsel())
                .finish()
        }
    }
    #[doc = "desc INTSEL12."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Intsel12(pub u32);
    impl Intsel12 {
        #[doc = "desc INTSEL."]
        #[must_use]
        #[inline(always)]
        pub const fn intsel(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x01ff;
            val as u16
        }
        #[doc = "desc INTSEL."]
        #[inline(always)]
        pub const fn set_intsel(&mut self, val: u16) {
            self.0 = (self.0 & !(0x01ff << 0usize)) | (((val as u32) & 0x01ff) << 0usize);
        }
    }
    impl Default for Intsel12 {
        #[inline(always)]
        fn default() -> Intsel12 {
            Intsel12(0)
        }
    }
    impl core::fmt::Debug for Intsel12 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Intsel12")
                .field("intsel", &self.intsel())
                .finish()
        }
    }
    #[doc = "desc INTSEL120."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Intsel120(pub u32);
    impl Intsel120 {
        #[doc = "desc INTSEL."]
        #[must_use]
        #[inline(always)]
        pub const fn intsel(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x01ff;
            val as u16
        }
        #[doc = "desc INTSEL."]
        #[inline(always)]
        pub const fn set_intsel(&mut self, val: u16) {
            self.0 = (self.0 & !(0x01ff << 0usize)) | (((val as u32) & 0x01ff) << 0usize);
        }
    }
    impl Default for Intsel120 {
        #[inline(always)]
        fn default() -> Intsel120 {
            Intsel120(0)
        }
    }
    impl core::fmt::Debug for Intsel120 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Intsel120")
                .field("intsel", &self.intsel())
                .finish()
        }
    }
    #[doc = "desc INTSEL121."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Intsel121(pub u32);
    impl Intsel121 {
        #[doc = "desc INTSEL."]
        #[must_use]
        #[inline(always)]
        pub const fn intsel(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x01ff;
            val as u16
        }
        #[doc = "desc INTSEL."]
        #[inline(always)]
        pub const fn set_intsel(&mut self, val: u16) {
            self.0 = (self.0 & !(0x01ff << 0usize)) | (((val as u32) & 0x01ff) << 0usize);
        }
    }
    impl Default for Intsel121 {
        #[inline(always)]
        fn default() -> Intsel121 {
            Intsel121(0)
        }
    }
    impl core::fmt::Debug for Intsel121 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Intsel121")
                .field("intsel", &self.intsel())
                .finish()
        }
    }
    #[doc = "desc INTSEL122."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Intsel122(pub u32);
    impl Intsel122 {
        #[doc = "desc INTSEL."]
        #[must_use]
        #[inline(always)]
        pub const fn intsel(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x01ff;
            val as u16
        }
        #[doc = "desc INTSEL."]
        #[inline(always)]
        pub const fn set_intsel(&mut self, val: u16) {
            self.0 = (self.0 & !(0x01ff << 0usize)) | (((val as u32) & 0x01ff) << 0usize);
        }
    }
    impl Default for Intsel122 {
        #[inline(always)]
        fn default() -> Intsel122 {
            Intsel122(0)
        }
    }
    impl core::fmt::Debug for Intsel122 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Intsel122")
                .field("intsel", &self.intsel())
                .finish()
        }
    }
    #[doc = "desc INTSEL123."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Intsel123(pub u32);
    impl Intsel123 {
        #[doc = "desc INTSEL."]
        #[must_use]
        #[inline(always)]
        pub const fn intsel(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x01ff;
            val as u16
        }
        #[doc = "desc INTSEL."]
        #[inline(always)]
        pub const fn set_intsel(&mut self, val: u16) {
            self.0 = (self.0 & !(0x01ff << 0usize)) | (((val as u32) & 0x01ff) << 0usize);
        }
    }
    impl Default for Intsel123 {
        #[inline(always)]
        fn default() -> Intsel123 {
            Intsel123(0)
        }
    }
    impl core::fmt::Debug for Intsel123 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Intsel123")
                .field("intsel", &self.intsel())
                .finish()
        }
    }
    #[doc = "desc INTSEL124."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Intsel124(pub u32);
    impl Intsel124 {
        #[doc = "desc INTSEL."]
        #[must_use]
        #[inline(always)]
        pub const fn intsel(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x01ff;
            val as u16
        }
        #[doc = "desc INTSEL."]
        #[inline(always)]
        pub const fn set_intsel(&mut self, val: u16) {
            self.0 = (self.0 & !(0x01ff << 0usize)) | (((val as u32) & 0x01ff) << 0usize);
        }
    }
    impl Default for Intsel124 {
        #[inline(always)]
        fn default() -> Intsel124 {
            Intsel124(0)
        }
    }
    impl core::fmt::Debug for Intsel124 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Intsel124")
                .field("intsel", &self.intsel())
                .finish()
        }
    }
    #[doc = "desc INTSEL125."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Intsel125(pub u32);
    impl Intsel125 {
        #[doc = "desc INTSEL."]
        #[must_use]
        #[inline(always)]
        pub const fn intsel(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x01ff;
            val as u16
        }
        #[doc = "desc INTSEL."]
        #[inline(always)]
        pub const fn set_intsel(&mut self, val: u16) {
            self.0 = (self.0 & !(0x01ff << 0usize)) | (((val as u32) & 0x01ff) << 0usize);
        }
    }
    impl Default for Intsel125 {
        #[inline(always)]
        fn default() -> Intsel125 {
            Intsel125(0)
        }
    }
    impl core::fmt::Debug for Intsel125 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Intsel125")
                .field("intsel", &self.intsel())
                .finish()
        }
    }
    #[doc = "desc INTSEL126."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Intsel126(pub u32);
    impl Intsel126 {
        #[doc = "desc INTSEL."]
        #[must_use]
        #[inline(always)]
        pub const fn intsel(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x01ff;
            val as u16
        }
        #[doc = "desc INTSEL."]
        #[inline(always)]
        pub const fn set_intsel(&mut self, val: u16) {
            self.0 = (self.0 & !(0x01ff << 0usize)) | (((val as u32) & 0x01ff) << 0usize);
        }
    }
    impl Default for Intsel126 {
        #[inline(always)]
        fn default() -> Intsel126 {
            Intsel126(0)
        }
    }
    impl core::fmt::Debug for Intsel126 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Intsel126")
                .field("intsel", &self.intsel())
                .finish()
        }
    }
    #[doc = "desc INTSEL127."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Intsel127(pub u32);
    impl Intsel127 {
        #[doc = "desc INTSEL."]
        #[must_use]
        #[inline(always)]
        pub const fn intsel(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x01ff;
            val as u16
        }
        #[doc = "desc INTSEL."]
        #[inline(always)]
        pub const fn set_intsel(&mut self, val: u16) {
            self.0 = (self.0 & !(0x01ff << 0usize)) | (((val as u32) & 0x01ff) << 0usize);
        }
    }
    impl Default for Intsel127 {
        #[inline(always)]
        fn default() -> Intsel127 {
            Intsel127(0)
        }
    }
    impl core::fmt::Debug for Intsel127 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Intsel127")
                .field("intsel", &self.intsel())
                .finish()
        }
    }
    #[doc = "desc INTSEL13."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Intsel13(pub u32);
    impl Intsel13 {
        #[doc = "desc INTSEL."]
        #[must_use]
        #[inline(always)]
        pub const fn intsel(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x01ff;
            val as u16
        }
        #[doc = "desc INTSEL."]
        #[inline(always)]
        pub const fn set_intsel(&mut self, val: u16) {
            self.0 = (self.0 & !(0x01ff << 0usize)) | (((val as u32) & 0x01ff) << 0usize);
        }
    }
    impl Default for Intsel13 {
        #[inline(always)]
        fn default() -> Intsel13 {
            Intsel13(0)
        }
    }
    impl core::fmt::Debug for Intsel13 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Intsel13")
                .field("intsel", &self.intsel())
                .finish()
        }
    }
    #[doc = "desc INTSEL14."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Intsel14(pub u32);
    impl Intsel14 {
        #[doc = "desc INTSEL."]
        #[must_use]
        #[inline(always)]
        pub const fn intsel(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x01ff;
            val as u16
        }
        #[doc = "desc INTSEL."]
        #[inline(always)]
        pub const fn set_intsel(&mut self, val: u16) {
            self.0 = (self.0 & !(0x01ff << 0usize)) | (((val as u32) & 0x01ff) << 0usize);
        }
    }
    impl Default for Intsel14 {
        #[inline(always)]
        fn default() -> Intsel14 {
            Intsel14(0)
        }
    }
    impl core::fmt::Debug for Intsel14 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Intsel14")
                .field("intsel", &self.intsel())
                .finish()
        }
    }
    #[doc = "desc INTSEL15."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Intsel15(pub u32);
    impl Intsel15 {
        #[doc = "desc INTSEL."]
        #[must_use]
        #[inline(always)]
        pub const fn intsel(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x01ff;
            val as u16
        }
        #[doc = "desc INTSEL."]
        #[inline(always)]
        pub const fn set_intsel(&mut self, val: u16) {
            self.0 = (self.0 & !(0x01ff << 0usize)) | (((val as u32) & 0x01ff) << 0usize);
        }
    }
    impl Default for Intsel15 {
        #[inline(always)]
        fn default() -> Intsel15 {
            Intsel15(0)
        }
    }
    impl core::fmt::Debug for Intsel15 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Intsel15")
                .field("intsel", &self.intsel())
                .finish()
        }
    }
    #[doc = "desc INTSEL16."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Intsel16(pub u32);
    impl Intsel16 {
        #[doc = "desc INTSEL."]
        #[must_use]
        #[inline(always)]
        pub const fn intsel(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x01ff;
            val as u16
        }
        #[doc = "desc INTSEL."]
        #[inline(always)]
        pub const fn set_intsel(&mut self, val: u16) {
            self.0 = (self.0 & !(0x01ff << 0usize)) | (((val as u32) & 0x01ff) << 0usize);
        }
    }
    impl Default for Intsel16 {
        #[inline(always)]
        fn default() -> Intsel16 {
            Intsel16(0)
        }
    }
    impl core::fmt::Debug for Intsel16 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Intsel16")
                .field("intsel", &self.intsel())
                .finish()
        }
    }
    #[doc = "desc INTSEL17."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Intsel17(pub u32);
    impl Intsel17 {
        #[doc = "desc INTSEL."]
        #[must_use]
        #[inline(always)]
        pub const fn intsel(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x01ff;
            val as u16
        }
        #[doc = "desc INTSEL."]
        #[inline(always)]
        pub const fn set_intsel(&mut self, val: u16) {
            self.0 = (self.0 & !(0x01ff << 0usize)) | (((val as u32) & 0x01ff) << 0usize);
        }
    }
    impl Default for Intsel17 {
        #[inline(always)]
        fn default() -> Intsel17 {
            Intsel17(0)
        }
    }
    impl core::fmt::Debug for Intsel17 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Intsel17")
                .field("intsel", &self.intsel())
                .finish()
        }
    }
    #[doc = "desc INTSEL18."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Intsel18(pub u32);
    impl Intsel18 {
        #[doc = "desc INTSEL."]
        #[must_use]
        #[inline(always)]
        pub const fn intsel(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x01ff;
            val as u16
        }
        #[doc = "desc INTSEL."]
        #[inline(always)]
        pub const fn set_intsel(&mut self, val: u16) {
            self.0 = (self.0 & !(0x01ff << 0usize)) | (((val as u32) & 0x01ff) << 0usize);
        }
    }
    impl Default for Intsel18 {
        #[inline(always)]
        fn default() -> Intsel18 {
            Intsel18(0)
        }
    }
    impl core::fmt::Debug for Intsel18 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Intsel18")
                .field("intsel", &self.intsel())
                .finish()
        }
    }
    #[doc = "desc INTSEL19."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Intsel19(pub u32);
    impl Intsel19 {
        #[doc = "desc INTSEL."]
        #[must_use]
        #[inline(always)]
        pub const fn intsel(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x01ff;
            val as u16
        }
        #[doc = "desc INTSEL."]
        #[inline(always)]
        pub const fn set_intsel(&mut self, val: u16) {
            self.0 = (self.0 & !(0x01ff << 0usize)) | (((val as u32) & 0x01ff) << 0usize);
        }
    }
    impl Default for Intsel19 {
        #[inline(always)]
        fn default() -> Intsel19 {
            Intsel19(0)
        }
    }
    impl core::fmt::Debug for Intsel19 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Intsel19")
                .field("intsel", &self.intsel())
                .finish()
        }
    }
    #[doc = "desc INTSEL2."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Intsel2(pub u32);
    impl Intsel2 {
        #[doc = "desc INTSEL."]
        #[must_use]
        #[inline(always)]
        pub const fn intsel(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x01ff;
            val as u16
        }
        #[doc = "desc INTSEL."]
        #[inline(always)]
        pub const fn set_intsel(&mut self, val: u16) {
            self.0 = (self.0 & !(0x01ff << 0usize)) | (((val as u32) & 0x01ff) << 0usize);
        }
    }
    impl Default for Intsel2 {
        #[inline(always)]
        fn default() -> Intsel2 {
            Intsel2(0)
        }
    }
    impl core::fmt::Debug for Intsel2 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Intsel2")
                .field("intsel", &self.intsel())
                .finish()
        }
    }
    #[doc = "desc INTSEL20."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Intsel20(pub u32);
    impl Intsel20 {
        #[doc = "desc INTSEL."]
        #[must_use]
        #[inline(always)]
        pub const fn intsel(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x01ff;
            val as u16
        }
        #[doc = "desc INTSEL."]
        #[inline(always)]
        pub const fn set_intsel(&mut self, val: u16) {
            self.0 = (self.0 & !(0x01ff << 0usize)) | (((val as u32) & 0x01ff) << 0usize);
        }
    }
    impl Default for Intsel20 {
        #[inline(always)]
        fn default() -> Intsel20 {
            Intsel20(0)
        }
    }
    impl core::fmt::Debug for Intsel20 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Intsel20")
                .field("intsel", &self.intsel())
                .finish()
        }
    }
    #[doc = "desc INTSEL21."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Intsel21(pub u32);
    impl Intsel21 {
        #[doc = "desc INTSEL."]
        #[must_use]
        #[inline(always)]
        pub const fn intsel(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x01ff;
            val as u16
        }
        #[doc = "desc INTSEL."]
        #[inline(always)]
        pub const fn set_intsel(&mut self, val: u16) {
            self.0 = (self.0 & !(0x01ff << 0usize)) | (((val as u32) & 0x01ff) << 0usize);
        }
    }
    impl Default for Intsel21 {
        #[inline(always)]
        fn default() -> Intsel21 {
            Intsel21(0)
        }
    }
    impl core::fmt::Debug for Intsel21 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Intsel21")
                .field("intsel", &self.intsel())
                .finish()
        }
    }
    #[doc = "desc INTSEL22."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Intsel22(pub u32);
    impl Intsel22 {
        #[doc = "desc INTSEL."]
        #[must_use]
        #[inline(always)]
        pub const fn intsel(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x01ff;
            val as u16
        }
        #[doc = "desc INTSEL."]
        #[inline(always)]
        pub const fn set_intsel(&mut self, val: u16) {
            self.0 = (self.0 & !(0x01ff << 0usize)) | (((val as u32) & 0x01ff) << 0usize);
        }
    }
    impl Default for Intsel22 {
        #[inline(always)]
        fn default() -> Intsel22 {
            Intsel22(0)
        }
    }
    impl core::fmt::Debug for Intsel22 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Intsel22")
                .field("intsel", &self.intsel())
                .finish()
        }
    }
    #[doc = "desc INTSEL23."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Intsel23(pub u32);
    impl Intsel23 {
        #[doc = "desc INTSEL."]
        #[must_use]
        #[inline(always)]
        pub const fn intsel(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x01ff;
            val as u16
        }
        #[doc = "desc INTSEL."]
        #[inline(always)]
        pub const fn set_intsel(&mut self, val: u16) {
            self.0 = (self.0 & !(0x01ff << 0usize)) | (((val as u32) & 0x01ff) << 0usize);
        }
    }
    impl Default for Intsel23 {
        #[inline(always)]
        fn default() -> Intsel23 {
            Intsel23(0)
        }
    }
    impl core::fmt::Debug for Intsel23 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Intsel23")
                .field("intsel", &self.intsel())
                .finish()
        }
    }
    #[doc = "desc INTSEL24."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Intsel24(pub u32);
    impl Intsel24 {
        #[doc = "desc INTSEL."]
        #[must_use]
        #[inline(always)]
        pub const fn intsel(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x01ff;
            val as u16
        }
        #[doc = "desc INTSEL."]
        #[inline(always)]
        pub const fn set_intsel(&mut self, val: u16) {
            self.0 = (self.0 & !(0x01ff << 0usize)) | (((val as u32) & 0x01ff) << 0usize);
        }
    }
    impl Default for Intsel24 {
        #[inline(always)]
        fn default() -> Intsel24 {
            Intsel24(0)
        }
    }
    impl core::fmt::Debug for Intsel24 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Intsel24")
                .field("intsel", &self.intsel())
                .finish()
        }
    }
    #[doc = "desc INTSEL25."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Intsel25(pub u32);
    impl Intsel25 {
        #[doc = "desc INTSEL."]
        #[must_use]
        #[inline(always)]
        pub const fn intsel(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x01ff;
            val as u16
        }
        #[doc = "desc INTSEL."]
        #[inline(always)]
        pub const fn set_intsel(&mut self, val: u16) {
            self.0 = (self.0 & !(0x01ff << 0usize)) | (((val as u32) & 0x01ff) << 0usize);
        }
    }
    impl Default for Intsel25 {
        #[inline(always)]
        fn default() -> Intsel25 {
            Intsel25(0)
        }
    }
    impl core::fmt::Debug for Intsel25 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Intsel25")
                .field("intsel", &self.intsel())
                .finish()
        }
    }
    #[doc = "desc INTSEL26."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Intsel26(pub u32);
    impl Intsel26 {
        #[doc = "desc INTSEL."]
        #[must_use]
        #[inline(always)]
        pub const fn intsel(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x01ff;
            val as u16
        }
        #[doc = "desc INTSEL."]
        #[inline(always)]
        pub const fn set_intsel(&mut self, val: u16) {
            self.0 = (self.0 & !(0x01ff << 0usize)) | (((val as u32) & 0x01ff) << 0usize);
        }
    }
    impl Default for Intsel26 {
        #[inline(always)]
        fn default() -> Intsel26 {
            Intsel26(0)
        }
    }
    impl core::fmt::Debug for Intsel26 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Intsel26")
                .field("intsel", &self.intsel())
                .finish()
        }
    }
    #[doc = "desc INTSEL27."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Intsel27(pub u32);
    impl Intsel27 {
        #[doc = "desc INTSEL."]
        #[must_use]
        #[inline(always)]
        pub const fn intsel(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x01ff;
            val as u16
        }
        #[doc = "desc INTSEL."]
        #[inline(always)]
        pub const fn set_intsel(&mut self, val: u16) {
            self.0 = (self.0 & !(0x01ff << 0usize)) | (((val as u32) & 0x01ff) << 0usize);
        }
    }
    impl Default for Intsel27 {
        #[inline(always)]
        fn default() -> Intsel27 {
            Intsel27(0)
        }
    }
    impl core::fmt::Debug for Intsel27 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Intsel27")
                .field("intsel", &self.intsel())
                .finish()
        }
    }
    #[doc = "desc INTSEL28."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Intsel28(pub u32);
    impl Intsel28 {
        #[doc = "desc INTSEL."]
        #[must_use]
        #[inline(always)]
        pub const fn intsel(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x01ff;
            val as u16
        }
        #[doc = "desc INTSEL."]
        #[inline(always)]
        pub const fn set_intsel(&mut self, val: u16) {
            self.0 = (self.0 & !(0x01ff << 0usize)) | (((val as u32) & 0x01ff) << 0usize);
        }
    }
    impl Default for Intsel28 {
        #[inline(always)]
        fn default() -> Intsel28 {
            Intsel28(0)
        }
    }
    impl core::fmt::Debug for Intsel28 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Intsel28")
                .field("intsel", &self.intsel())
                .finish()
        }
    }
    #[doc = "desc INTSEL29."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Intsel29(pub u32);
    impl Intsel29 {
        #[doc = "desc INTSEL."]
        #[must_use]
        #[inline(always)]
        pub const fn intsel(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x01ff;
            val as u16
        }
        #[doc = "desc INTSEL."]
        #[inline(always)]
        pub const fn set_intsel(&mut self, val: u16) {
            self.0 = (self.0 & !(0x01ff << 0usize)) | (((val as u32) & 0x01ff) << 0usize);
        }
    }
    impl Default for Intsel29 {
        #[inline(always)]
        fn default() -> Intsel29 {
            Intsel29(0)
        }
    }
    impl core::fmt::Debug for Intsel29 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Intsel29")
                .field("intsel", &self.intsel())
                .finish()
        }
    }
    #[doc = "desc INTSEL3."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Intsel3(pub u32);
    impl Intsel3 {
        #[doc = "desc INTSEL."]
        #[must_use]
        #[inline(always)]
        pub const fn intsel(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x01ff;
            val as u16
        }
        #[doc = "desc INTSEL."]
        #[inline(always)]
        pub const fn set_intsel(&mut self, val: u16) {
            self.0 = (self.0 & !(0x01ff << 0usize)) | (((val as u32) & 0x01ff) << 0usize);
        }
    }
    impl Default for Intsel3 {
        #[inline(always)]
        fn default() -> Intsel3 {
            Intsel3(0)
        }
    }
    impl core::fmt::Debug for Intsel3 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Intsel3")
                .field("intsel", &self.intsel())
                .finish()
        }
    }
    #[doc = "desc INTSEL30."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Intsel30(pub u32);
    impl Intsel30 {
        #[doc = "desc INTSEL."]
        #[must_use]
        #[inline(always)]
        pub const fn intsel(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x01ff;
            val as u16
        }
        #[doc = "desc INTSEL."]
        #[inline(always)]
        pub const fn set_intsel(&mut self, val: u16) {
            self.0 = (self.0 & !(0x01ff << 0usize)) | (((val as u32) & 0x01ff) << 0usize);
        }
    }
    impl Default for Intsel30 {
        #[inline(always)]
        fn default() -> Intsel30 {
            Intsel30(0)
        }
    }
    impl core::fmt::Debug for Intsel30 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Intsel30")
                .field("intsel", &self.intsel())
                .finish()
        }
    }
    #[doc = "desc INTSEL31."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Intsel31(pub u32);
    impl Intsel31 {
        #[doc = "desc INTSEL."]
        #[must_use]
        #[inline(always)]
        pub const fn intsel(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x01ff;
            val as u16
        }
        #[doc = "desc INTSEL."]
        #[inline(always)]
        pub const fn set_intsel(&mut self, val: u16) {
            self.0 = (self.0 & !(0x01ff << 0usize)) | (((val as u32) & 0x01ff) << 0usize);
        }
    }
    impl Default for Intsel31 {
        #[inline(always)]
        fn default() -> Intsel31 {
            Intsel31(0)
        }
    }
    impl core::fmt::Debug for Intsel31 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Intsel31")
                .field("intsel", &self.intsel())
                .finish()
        }
    }
    #[doc = "desc INTSEL32."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Intsel32(pub u32);
    impl Intsel32 {
        #[doc = "desc INTSEL."]
        #[must_use]
        #[inline(always)]
        pub const fn intsel(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x01ff;
            val as u16
        }
        #[doc = "desc INTSEL."]
        #[inline(always)]
        pub const fn set_intsel(&mut self, val: u16) {
            self.0 = (self.0 & !(0x01ff << 0usize)) | (((val as u32) & 0x01ff) << 0usize);
        }
    }
    impl Default for Intsel32 {
        #[inline(always)]
        fn default() -> Intsel32 {
            Intsel32(0)
        }
    }
    impl core::fmt::Debug for Intsel32 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Intsel32")
                .field("intsel", &self.intsel())
                .finish()
        }
    }
    #[doc = "desc INTSEL33."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Intsel33(pub u32);
    impl Intsel33 {
        #[doc = "desc INTSEL."]
        #[must_use]
        #[inline(always)]
        pub const fn intsel(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x01ff;
            val as u16
        }
        #[doc = "desc INTSEL."]
        #[inline(always)]
        pub const fn set_intsel(&mut self, val: u16) {
            self.0 = (self.0 & !(0x01ff << 0usize)) | (((val as u32) & 0x01ff) << 0usize);
        }
    }
    impl Default for Intsel33 {
        #[inline(always)]
        fn default() -> Intsel33 {
            Intsel33(0)
        }
    }
    impl core::fmt::Debug for Intsel33 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Intsel33")
                .field("intsel", &self.intsel())
                .finish()
        }
    }
    #[doc = "desc INTSEL34."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Intsel34(pub u32);
    impl Intsel34 {
        #[doc = "desc INTSEL."]
        #[must_use]
        #[inline(always)]
        pub const fn intsel(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x01ff;
            val as u16
        }
        #[doc = "desc INTSEL."]
        #[inline(always)]
        pub const fn set_intsel(&mut self, val: u16) {
            self.0 = (self.0 & !(0x01ff << 0usize)) | (((val as u32) & 0x01ff) << 0usize);
        }
    }
    impl Default for Intsel34 {
        #[inline(always)]
        fn default() -> Intsel34 {
            Intsel34(0)
        }
    }
    impl core::fmt::Debug for Intsel34 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Intsel34")
                .field("intsel", &self.intsel())
                .finish()
        }
    }
    #[doc = "desc INTSEL35."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Intsel35(pub u32);
    impl Intsel35 {
        #[doc = "desc INTSEL."]
        #[must_use]
        #[inline(always)]
        pub const fn intsel(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x01ff;
            val as u16
        }
        #[doc = "desc INTSEL."]
        #[inline(always)]
        pub const fn set_intsel(&mut self, val: u16) {
            self.0 = (self.0 & !(0x01ff << 0usize)) | (((val as u32) & 0x01ff) << 0usize);
        }
    }
    impl Default for Intsel35 {
        #[inline(always)]
        fn default() -> Intsel35 {
            Intsel35(0)
        }
    }
    impl core::fmt::Debug for Intsel35 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Intsel35")
                .field("intsel", &self.intsel())
                .finish()
        }
    }
    #[doc = "desc INTSEL36."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Intsel36(pub u32);
    impl Intsel36 {
        #[doc = "desc INTSEL."]
        #[must_use]
        #[inline(always)]
        pub const fn intsel(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x01ff;
            val as u16
        }
        #[doc = "desc INTSEL."]
        #[inline(always)]
        pub const fn set_intsel(&mut self, val: u16) {
            self.0 = (self.0 & !(0x01ff << 0usize)) | (((val as u32) & 0x01ff) << 0usize);
        }
    }
    impl Default for Intsel36 {
        #[inline(always)]
        fn default() -> Intsel36 {
            Intsel36(0)
        }
    }
    impl core::fmt::Debug for Intsel36 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Intsel36")
                .field("intsel", &self.intsel())
                .finish()
        }
    }
    #[doc = "desc INTSEL37."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Intsel37(pub u32);
    impl Intsel37 {
        #[doc = "desc INTSEL."]
        #[must_use]
        #[inline(always)]
        pub const fn intsel(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x01ff;
            val as u16
        }
        #[doc = "desc INTSEL."]
        #[inline(always)]
        pub const fn set_intsel(&mut self, val: u16) {
            self.0 = (self.0 & !(0x01ff << 0usize)) | (((val as u32) & 0x01ff) << 0usize);
        }
    }
    impl Default for Intsel37 {
        #[inline(always)]
        fn default() -> Intsel37 {
            Intsel37(0)
        }
    }
    impl core::fmt::Debug for Intsel37 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Intsel37")
                .field("intsel", &self.intsel())
                .finish()
        }
    }
    #[doc = "desc INTSEL38."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Intsel38(pub u32);
    impl Intsel38 {
        #[doc = "desc INTSEL."]
        #[must_use]
        #[inline(always)]
        pub const fn intsel(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x01ff;
            val as u16
        }
        #[doc = "desc INTSEL."]
        #[inline(always)]
        pub const fn set_intsel(&mut self, val: u16) {
            self.0 = (self.0 & !(0x01ff << 0usize)) | (((val as u32) & 0x01ff) << 0usize);
        }
    }
    impl Default for Intsel38 {
        #[inline(always)]
        fn default() -> Intsel38 {
            Intsel38(0)
        }
    }
    impl core::fmt::Debug for Intsel38 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Intsel38")
                .field("intsel", &self.intsel())
                .finish()
        }
    }
    #[doc = "desc INTSEL39."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Intsel39(pub u32);
    impl Intsel39 {
        #[doc = "desc INTSEL."]
        #[must_use]
        #[inline(always)]
        pub const fn intsel(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x01ff;
            val as u16
        }
        #[doc = "desc INTSEL."]
        #[inline(always)]
        pub const fn set_intsel(&mut self, val: u16) {
            self.0 = (self.0 & !(0x01ff << 0usize)) | (((val as u32) & 0x01ff) << 0usize);
        }
    }
    impl Default for Intsel39 {
        #[inline(always)]
        fn default() -> Intsel39 {
            Intsel39(0)
        }
    }
    impl core::fmt::Debug for Intsel39 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Intsel39")
                .field("intsel", &self.intsel())
                .finish()
        }
    }
    #[doc = "desc INTSEL4."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Intsel4(pub u32);
    impl Intsel4 {
        #[doc = "desc INTSEL."]
        #[must_use]
        #[inline(always)]
        pub const fn intsel(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x01ff;
            val as u16
        }
        #[doc = "desc INTSEL."]
        #[inline(always)]
        pub const fn set_intsel(&mut self, val: u16) {
            self.0 = (self.0 & !(0x01ff << 0usize)) | (((val as u32) & 0x01ff) << 0usize);
        }
    }
    impl Default for Intsel4 {
        #[inline(always)]
        fn default() -> Intsel4 {
            Intsel4(0)
        }
    }
    impl core::fmt::Debug for Intsel4 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Intsel4")
                .field("intsel", &self.intsel())
                .finish()
        }
    }
    #[doc = "desc INTSEL40."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Intsel40(pub u32);
    impl Intsel40 {
        #[doc = "desc INTSEL."]
        #[must_use]
        #[inline(always)]
        pub const fn intsel(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x01ff;
            val as u16
        }
        #[doc = "desc INTSEL."]
        #[inline(always)]
        pub const fn set_intsel(&mut self, val: u16) {
            self.0 = (self.0 & !(0x01ff << 0usize)) | (((val as u32) & 0x01ff) << 0usize);
        }
    }
    impl Default for Intsel40 {
        #[inline(always)]
        fn default() -> Intsel40 {
            Intsel40(0)
        }
    }
    impl core::fmt::Debug for Intsel40 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Intsel40")
                .field("intsel", &self.intsel())
                .finish()
        }
    }
    #[doc = "desc INTSEL41."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Intsel41(pub u32);
    impl Intsel41 {
        #[doc = "desc INTSEL."]
        #[must_use]
        #[inline(always)]
        pub const fn intsel(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x01ff;
            val as u16
        }
        #[doc = "desc INTSEL."]
        #[inline(always)]
        pub const fn set_intsel(&mut self, val: u16) {
            self.0 = (self.0 & !(0x01ff << 0usize)) | (((val as u32) & 0x01ff) << 0usize);
        }
    }
    impl Default for Intsel41 {
        #[inline(always)]
        fn default() -> Intsel41 {
            Intsel41(0)
        }
    }
    impl core::fmt::Debug for Intsel41 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Intsel41")
                .field("intsel", &self.intsel())
                .finish()
        }
    }
    #[doc = "desc INTSEL42."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Intsel42(pub u32);
    impl Intsel42 {
        #[doc = "desc INTSEL."]
        #[must_use]
        #[inline(always)]
        pub const fn intsel(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x01ff;
            val as u16
        }
        #[doc = "desc INTSEL."]
        #[inline(always)]
        pub const fn set_intsel(&mut self, val: u16) {
            self.0 = (self.0 & !(0x01ff << 0usize)) | (((val as u32) & 0x01ff) << 0usize);
        }
    }
    impl Default for Intsel42 {
        #[inline(always)]
        fn default() -> Intsel42 {
            Intsel42(0)
        }
    }
    impl core::fmt::Debug for Intsel42 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Intsel42")
                .field("intsel", &self.intsel())
                .finish()
        }
    }
    #[doc = "desc INTSEL43."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Intsel43(pub u32);
    impl Intsel43 {
        #[doc = "desc INTSEL."]
        #[must_use]
        #[inline(always)]
        pub const fn intsel(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x01ff;
            val as u16
        }
        #[doc = "desc INTSEL."]
        #[inline(always)]
        pub const fn set_intsel(&mut self, val: u16) {
            self.0 = (self.0 & !(0x01ff << 0usize)) | (((val as u32) & 0x01ff) << 0usize);
        }
    }
    impl Default for Intsel43 {
        #[inline(always)]
        fn default() -> Intsel43 {
            Intsel43(0)
        }
    }
    impl core::fmt::Debug for Intsel43 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Intsel43")
                .field("intsel", &self.intsel())
                .finish()
        }
    }
    #[doc = "desc INTSEL44."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Intsel44(pub u32);
    impl Intsel44 {
        #[doc = "desc INTSEL."]
        #[must_use]
        #[inline(always)]
        pub const fn intsel(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x01ff;
            val as u16
        }
        #[doc = "desc INTSEL."]
        #[inline(always)]
        pub const fn set_intsel(&mut self, val: u16) {
            self.0 = (self.0 & !(0x01ff << 0usize)) | (((val as u32) & 0x01ff) << 0usize);
        }
    }
    impl Default for Intsel44 {
        #[inline(always)]
        fn default() -> Intsel44 {
            Intsel44(0)
        }
    }
    impl core::fmt::Debug for Intsel44 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Intsel44")
                .field("intsel", &self.intsel())
                .finish()
        }
    }
    #[doc = "desc INTSEL45."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Intsel45(pub u32);
    impl Intsel45 {
        #[doc = "desc INTSEL."]
        #[must_use]
        #[inline(always)]
        pub const fn intsel(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x01ff;
            val as u16
        }
        #[doc = "desc INTSEL."]
        #[inline(always)]
        pub const fn set_intsel(&mut self, val: u16) {
            self.0 = (self.0 & !(0x01ff << 0usize)) | (((val as u32) & 0x01ff) << 0usize);
        }
    }
    impl Default for Intsel45 {
        #[inline(always)]
        fn default() -> Intsel45 {
            Intsel45(0)
        }
    }
    impl core::fmt::Debug for Intsel45 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Intsel45")
                .field("intsel", &self.intsel())
                .finish()
        }
    }
    #[doc = "desc INTSEL46."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Intsel46(pub u32);
    impl Intsel46 {
        #[doc = "desc INTSEL."]
        #[must_use]
        #[inline(always)]
        pub const fn intsel(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x01ff;
            val as u16
        }
        #[doc = "desc INTSEL."]
        #[inline(always)]
        pub const fn set_intsel(&mut self, val: u16) {
            self.0 = (self.0 & !(0x01ff << 0usize)) | (((val as u32) & 0x01ff) << 0usize);
        }
    }
    impl Default for Intsel46 {
        #[inline(always)]
        fn default() -> Intsel46 {
            Intsel46(0)
        }
    }
    impl core::fmt::Debug for Intsel46 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Intsel46")
                .field("intsel", &self.intsel())
                .finish()
        }
    }
    #[doc = "desc INTSEL47."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Intsel47(pub u32);
    impl Intsel47 {
        #[doc = "desc INTSEL."]
        #[must_use]
        #[inline(always)]
        pub const fn intsel(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x01ff;
            val as u16
        }
        #[doc = "desc INTSEL."]
        #[inline(always)]
        pub const fn set_intsel(&mut self, val: u16) {
            self.0 = (self.0 & !(0x01ff << 0usize)) | (((val as u32) & 0x01ff) << 0usize);
        }
    }
    impl Default for Intsel47 {
        #[inline(always)]
        fn default() -> Intsel47 {
            Intsel47(0)
        }
    }
    impl core::fmt::Debug for Intsel47 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Intsel47")
                .field("intsel", &self.intsel())
                .finish()
        }
    }
    #[doc = "desc INTSEL48."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Intsel48(pub u32);
    impl Intsel48 {
        #[doc = "desc INTSEL."]
        #[must_use]
        #[inline(always)]
        pub const fn intsel(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x01ff;
            val as u16
        }
        #[doc = "desc INTSEL."]
        #[inline(always)]
        pub const fn set_intsel(&mut self, val: u16) {
            self.0 = (self.0 & !(0x01ff << 0usize)) | (((val as u32) & 0x01ff) << 0usize);
        }
    }
    impl Default for Intsel48 {
        #[inline(always)]
        fn default() -> Intsel48 {
            Intsel48(0)
        }
    }
    impl core::fmt::Debug for Intsel48 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Intsel48")
                .field("intsel", &self.intsel())
                .finish()
        }
    }
    #[doc = "desc INTSEL49."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Intsel49(pub u32);
    impl Intsel49 {
        #[doc = "desc INTSEL."]
        #[must_use]
        #[inline(always)]
        pub const fn intsel(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x01ff;
            val as u16
        }
        #[doc = "desc INTSEL."]
        #[inline(always)]
        pub const fn set_intsel(&mut self, val: u16) {
            self.0 = (self.0 & !(0x01ff << 0usize)) | (((val as u32) & 0x01ff) << 0usize);
        }
    }
    impl Default for Intsel49 {
        #[inline(always)]
        fn default() -> Intsel49 {
            Intsel49(0)
        }
    }
    impl core::fmt::Debug for Intsel49 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Intsel49")
                .field("intsel", &self.intsel())
                .finish()
        }
    }
    #[doc = "desc INTSEL5."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Intsel5(pub u32);
    impl Intsel5 {
        #[doc = "desc INTSEL."]
        #[must_use]
        #[inline(always)]
        pub const fn intsel(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x01ff;
            val as u16
        }
        #[doc = "desc INTSEL."]
        #[inline(always)]
        pub const fn set_intsel(&mut self, val: u16) {
            self.0 = (self.0 & !(0x01ff << 0usize)) | (((val as u32) & 0x01ff) << 0usize);
        }
    }
    impl Default for Intsel5 {
        #[inline(always)]
        fn default() -> Intsel5 {
            Intsel5(0)
        }
    }
    impl core::fmt::Debug for Intsel5 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Intsel5")
                .field("intsel", &self.intsel())
                .finish()
        }
    }
    #[doc = "desc INTSEL50."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Intsel50(pub u32);
    impl Intsel50 {
        #[doc = "desc INTSEL."]
        #[must_use]
        #[inline(always)]
        pub const fn intsel(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x01ff;
            val as u16
        }
        #[doc = "desc INTSEL."]
        #[inline(always)]
        pub const fn set_intsel(&mut self, val: u16) {
            self.0 = (self.0 & !(0x01ff << 0usize)) | (((val as u32) & 0x01ff) << 0usize);
        }
    }
    impl Default for Intsel50 {
        #[inline(always)]
        fn default() -> Intsel50 {
            Intsel50(0)
        }
    }
    impl core::fmt::Debug for Intsel50 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Intsel50")
                .field("intsel", &self.intsel())
                .finish()
        }
    }
    #[doc = "desc INTSEL51."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Intsel51(pub u32);
    impl Intsel51 {
        #[doc = "desc INTSEL."]
        #[must_use]
        #[inline(always)]
        pub const fn intsel(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x01ff;
            val as u16
        }
        #[doc = "desc INTSEL."]
        #[inline(always)]
        pub const fn set_intsel(&mut self, val: u16) {
            self.0 = (self.0 & !(0x01ff << 0usize)) | (((val as u32) & 0x01ff) << 0usize);
        }
    }
    impl Default for Intsel51 {
        #[inline(always)]
        fn default() -> Intsel51 {
            Intsel51(0)
        }
    }
    impl core::fmt::Debug for Intsel51 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Intsel51")
                .field("intsel", &self.intsel())
                .finish()
        }
    }
    #[doc = "desc INTSEL52."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Intsel52(pub u32);
    impl Intsel52 {
        #[doc = "desc INTSEL."]
        #[must_use]
        #[inline(always)]
        pub const fn intsel(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x01ff;
            val as u16
        }
        #[doc = "desc INTSEL."]
        #[inline(always)]
        pub const fn set_intsel(&mut self, val: u16) {
            self.0 = (self.0 & !(0x01ff << 0usize)) | (((val as u32) & 0x01ff) << 0usize);
        }
    }
    impl Default for Intsel52 {
        #[inline(always)]
        fn default() -> Intsel52 {
            Intsel52(0)
        }
    }
    impl core::fmt::Debug for Intsel52 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Intsel52")
                .field("intsel", &self.intsel())
                .finish()
        }
    }
    #[doc = "desc INTSEL53."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Intsel53(pub u32);
    impl Intsel53 {
        #[doc = "desc INTSEL."]
        #[must_use]
        #[inline(always)]
        pub const fn intsel(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x01ff;
            val as u16
        }
        #[doc = "desc INTSEL."]
        #[inline(always)]
        pub const fn set_intsel(&mut self, val: u16) {
            self.0 = (self.0 & !(0x01ff << 0usize)) | (((val as u32) & 0x01ff) << 0usize);
        }
    }
    impl Default for Intsel53 {
        #[inline(always)]
        fn default() -> Intsel53 {
            Intsel53(0)
        }
    }
    impl core::fmt::Debug for Intsel53 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Intsel53")
                .field("intsel", &self.intsel())
                .finish()
        }
    }
    #[doc = "desc INTSEL54."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Intsel54(pub u32);
    impl Intsel54 {
        #[doc = "desc INTSEL."]
        #[must_use]
        #[inline(always)]
        pub const fn intsel(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x01ff;
            val as u16
        }
        #[doc = "desc INTSEL."]
        #[inline(always)]
        pub const fn set_intsel(&mut self, val: u16) {
            self.0 = (self.0 & !(0x01ff << 0usize)) | (((val as u32) & 0x01ff) << 0usize);
        }
    }
    impl Default for Intsel54 {
        #[inline(always)]
        fn default() -> Intsel54 {
            Intsel54(0)
        }
    }
    impl core::fmt::Debug for Intsel54 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Intsel54")
                .field("intsel", &self.intsel())
                .finish()
        }
    }
    #[doc = "desc INTSEL55."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Intsel55(pub u32);
    impl Intsel55 {
        #[doc = "desc INTSEL."]
        #[must_use]
        #[inline(always)]
        pub const fn intsel(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x01ff;
            val as u16
        }
        #[doc = "desc INTSEL."]
        #[inline(always)]
        pub const fn set_intsel(&mut self, val: u16) {
            self.0 = (self.0 & !(0x01ff << 0usize)) | (((val as u32) & 0x01ff) << 0usize);
        }
    }
    impl Default for Intsel55 {
        #[inline(always)]
        fn default() -> Intsel55 {
            Intsel55(0)
        }
    }
    impl core::fmt::Debug for Intsel55 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Intsel55")
                .field("intsel", &self.intsel())
                .finish()
        }
    }
    #[doc = "desc INTSEL56."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Intsel56(pub u32);
    impl Intsel56 {
        #[doc = "desc INTSEL."]
        #[must_use]
        #[inline(always)]
        pub const fn intsel(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x01ff;
            val as u16
        }
        #[doc = "desc INTSEL."]
        #[inline(always)]
        pub const fn set_intsel(&mut self, val: u16) {
            self.0 = (self.0 & !(0x01ff << 0usize)) | (((val as u32) & 0x01ff) << 0usize);
        }
    }
    impl Default for Intsel56 {
        #[inline(always)]
        fn default() -> Intsel56 {
            Intsel56(0)
        }
    }
    impl core::fmt::Debug for Intsel56 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Intsel56")
                .field("intsel", &self.intsel())
                .finish()
        }
    }
    #[doc = "desc INTSEL57."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Intsel57(pub u32);
    impl Intsel57 {
        #[doc = "desc INTSEL."]
        #[must_use]
        #[inline(always)]
        pub const fn intsel(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x01ff;
            val as u16
        }
        #[doc = "desc INTSEL."]
        #[inline(always)]
        pub const fn set_intsel(&mut self, val: u16) {
            self.0 = (self.0 & !(0x01ff << 0usize)) | (((val as u32) & 0x01ff) << 0usize);
        }
    }
    impl Default for Intsel57 {
        #[inline(always)]
        fn default() -> Intsel57 {
            Intsel57(0)
        }
    }
    impl core::fmt::Debug for Intsel57 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Intsel57")
                .field("intsel", &self.intsel())
                .finish()
        }
    }
    #[doc = "desc INTSEL58."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Intsel58(pub u32);
    impl Intsel58 {
        #[doc = "desc INTSEL."]
        #[must_use]
        #[inline(always)]
        pub const fn intsel(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x01ff;
            val as u16
        }
        #[doc = "desc INTSEL."]
        #[inline(always)]
        pub const fn set_intsel(&mut self, val: u16) {
            self.0 = (self.0 & !(0x01ff << 0usize)) | (((val as u32) & 0x01ff) << 0usize);
        }
    }
    impl Default for Intsel58 {
        #[inline(always)]
        fn default() -> Intsel58 {
            Intsel58(0)
        }
    }
    impl core::fmt::Debug for Intsel58 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Intsel58")
                .field("intsel", &self.intsel())
                .finish()
        }
    }
    #[doc = "desc INTSEL59."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Intsel59(pub u32);
    impl Intsel59 {
        #[doc = "desc INTSEL."]
        #[must_use]
        #[inline(always)]
        pub const fn intsel(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x01ff;
            val as u16
        }
        #[doc = "desc INTSEL."]
        #[inline(always)]
        pub const fn set_intsel(&mut self, val: u16) {
            self.0 = (self.0 & !(0x01ff << 0usize)) | (((val as u32) & 0x01ff) << 0usize);
        }
    }
    impl Default for Intsel59 {
        #[inline(always)]
        fn default() -> Intsel59 {
            Intsel59(0)
        }
    }
    impl core::fmt::Debug for Intsel59 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Intsel59")
                .field("intsel", &self.intsel())
                .finish()
        }
    }
    #[doc = "desc INTSEL6."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Intsel6(pub u32);
    impl Intsel6 {
        #[doc = "desc INTSEL."]
        #[must_use]
        #[inline(always)]
        pub const fn intsel(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x01ff;
            val as u16
        }
        #[doc = "desc INTSEL."]
        #[inline(always)]
        pub const fn set_intsel(&mut self, val: u16) {
            self.0 = (self.0 & !(0x01ff << 0usize)) | (((val as u32) & 0x01ff) << 0usize);
        }
    }
    impl Default for Intsel6 {
        #[inline(always)]
        fn default() -> Intsel6 {
            Intsel6(0)
        }
    }
    impl core::fmt::Debug for Intsel6 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Intsel6")
                .field("intsel", &self.intsel())
                .finish()
        }
    }
    #[doc = "desc INTSEL60."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Intsel60(pub u32);
    impl Intsel60 {
        #[doc = "desc INTSEL."]
        #[must_use]
        #[inline(always)]
        pub const fn intsel(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x01ff;
            val as u16
        }
        #[doc = "desc INTSEL."]
        #[inline(always)]
        pub const fn set_intsel(&mut self, val: u16) {
            self.0 = (self.0 & !(0x01ff << 0usize)) | (((val as u32) & 0x01ff) << 0usize);
        }
    }
    impl Default for Intsel60 {
        #[inline(always)]
        fn default() -> Intsel60 {
            Intsel60(0)
        }
    }
    impl core::fmt::Debug for Intsel60 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Intsel60")
                .field("intsel", &self.intsel())
                .finish()
        }
    }
    #[doc = "desc INTSEL61."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Intsel61(pub u32);
    impl Intsel61 {
        #[doc = "desc INTSEL."]
        #[must_use]
        #[inline(always)]
        pub const fn intsel(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x01ff;
            val as u16
        }
        #[doc = "desc INTSEL."]
        #[inline(always)]
        pub const fn set_intsel(&mut self, val: u16) {
            self.0 = (self.0 & !(0x01ff << 0usize)) | (((val as u32) & 0x01ff) << 0usize);
        }
    }
    impl Default for Intsel61 {
        #[inline(always)]
        fn default() -> Intsel61 {
            Intsel61(0)
        }
    }
    impl core::fmt::Debug for Intsel61 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Intsel61")
                .field("intsel", &self.intsel())
                .finish()
        }
    }
    #[doc = "desc INTSEL62."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Intsel62(pub u32);
    impl Intsel62 {
        #[doc = "desc INTSEL."]
        #[must_use]
        #[inline(always)]
        pub const fn intsel(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x01ff;
            val as u16
        }
        #[doc = "desc INTSEL."]
        #[inline(always)]
        pub const fn set_intsel(&mut self, val: u16) {
            self.0 = (self.0 & !(0x01ff << 0usize)) | (((val as u32) & 0x01ff) << 0usize);
        }
    }
    impl Default for Intsel62 {
        #[inline(always)]
        fn default() -> Intsel62 {
            Intsel62(0)
        }
    }
    impl core::fmt::Debug for Intsel62 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Intsel62")
                .field("intsel", &self.intsel())
                .finish()
        }
    }
    #[doc = "desc INTSEL63."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Intsel63(pub u32);
    impl Intsel63 {
        #[doc = "desc INTSEL."]
        #[must_use]
        #[inline(always)]
        pub const fn intsel(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x01ff;
            val as u16
        }
        #[doc = "desc INTSEL."]
        #[inline(always)]
        pub const fn set_intsel(&mut self, val: u16) {
            self.0 = (self.0 & !(0x01ff << 0usize)) | (((val as u32) & 0x01ff) << 0usize);
        }
    }
    impl Default for Intsel63 {
        #[inline(always)]
        fn default() -> Intsel63 {
            Intsel63(0)
        }
    }
    impl core::fmt::Debug for Intsel63 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Intsel63")
                .field("intsel", &self.intsel())
                .finish()
        }
    }
    #[doc = "desc INTSEL64."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Intsel64(pub u32);
    impl Intsel64 {
        #[doc = "desc INTSEL."]
        #[must_use]
        #[inline(always)]
        pub const fn intsel(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x01ff;
            val as u16
        }
        #[doc = "desc INTSEL."]
        #[inline(always)]
        pub const fn set_intsel(&mut self, val: u16) {
            self.0 = (self.0 & !(0x01ff << 0usize)) | (((val as u32) & 0x01ff) << 0usize);
        }
    }
    impl Default for Intsel64 {
        #[inline(always)]
        fn default() -> Intsel64 {
            Intsel64(0)
        }
    }
    impl core::fmt::Debug for Intsel64 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Intsel64")
                .field("intsel", &self.intsel())
                .finish()
        }
    }
    #[doc = "desc INTSEL65."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Intsel65(pub u32);
    impl Intsel65 {
        #[doc = "desc INTSEL."]
        #[must_use]
        #[inline(always)]
        pub const fn intsel(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x01ff;
            val as u16
        }
        #[doc = "desc INTSEL."]
        #[inline(always)]
        pub const fn set_intsel(&mut self, val: u16) {
            self.0 = (self.0 & !(0x01ff << 0usize)) | (((val as u32) & 0x01ff) << 0usize);
        }
    }
    impl Default for Intsel65 {
        #[inline(always)]
        fn default() -> Intsel65 {
            Intsel65(0)
        }
    }
    impl core::fmt::Debug for Intsel65 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Intsel65")
                .field("intsel", &self.intsel())
                .finish()
        }
    }
    #[doc = "desc INTSEL66."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Intsel66(pub u32);
    impl Intsel66 {
        #[doc = "desc INTSEL."]
        #[must_use]
        #[inline(always)]
        pub const fn intsel(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x01ff;
            val as u16
        }
        #[doc = "desc INTSEL."]
        #[inline(always)]
        pub const fn set_intsel(&mut self, val: u16) {
            self.0 = (self.0 & !(0x01ff << 0usize)) | (((val as u32) & 0x01ff) << 0usize);
        }
    }
    impl Default for Intsel66 {
        #[inline(always)]
        fn default() -> Intsel66 {
            Intsel66(0)
        }
    }
    impl core::fmt::Debug for Intsel66 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Intsel66")
                .field("intsel", &self.intsel())
                .finish()
        }
    }
    #[doc = "desc INTSEL67."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Intsel67(pub u32);
    impl Intsel67 {
        #[doc = "desc INTSEL."]
        #[must_use]
        #[inline(always)]
        pub const fn intsel(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x01ff;
            val as u16
        }
        #[doc = "desc INTSEL."]
        #[inline(always)]
        pub const fn set_intsel(&mut self, val: u16) {
            self.0 = (self.0 & !(0x01ff << 0usize)) | (((val as u32) & 0x01ff) << 0usize);
        }
    }
    impl Default for Intsel67 {
        #[inline(always)]
        fn default() -> Intsel67 {
            Intsel67(0)
        }
    }
    impl core::fmt::Debug for Intsel67 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Intsel67")
                .field("intsel", &self.intsel())
                .finish()
        }
    }
    #[doc = "desc INTSEL68."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Intsel68(pub u32);
    impl Intsel68 {
        #[doc = "desc INTSEL."]
        #[must_use]
        #[inline(always)]
        pub const fn intsel(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x01ff;
            val as u16
        }
        #[doc = "desc INTSEL."]
        #[inline(always)]
        pub const fn set_intsel(&mut self, val: u16) {
            self.0 = (self.0 & !(0x01ff << 0usize)) | (((val as u32) & 0x01ff) << 0usize);
        }
    }
    impl Default for Intsel68 {
        #[inline(always)]
        fn default() -> Intsel68 {
            Intsel68(0)
        }
    }
    impl core::fmt::Debug for Intsel68 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Intsel68")
                .field("intsel", &self.intsel())
                .finish()
        }
    }
    #[doc = "desc INTSEL69."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Intsel69(pub u32);
    impl Intsel69 {
        #[doc = "desc INTSEL."]
        #[must_use]
        #[inline(always)]
        pub const fn intsel(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x01ff;
            val as u16
        }
        #[doc = "desc INTSEL."]
        #[inline(always)]
        pub const fn set_intsel(&mut self, val: u16) {
            self.0 = (self.0 & !(0x01ff << 0usize)) | (((val as u32) & 0x01ff) << 0usize);
        }
    }
    impl Default for Intsel69 {
        #[inline(always)]
        fn default() -> Intsel69 {
            Intsel69(0)
        }
    }
    impl core::fmt::Debug for Intsel69 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Intsel69")
                .field("intsel", &self.intsel())
                .finish()
        }
    }
    #[doc = "desc INTSEL7."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Intsel7(pub u32);
    impl Intsel7 {
        #[doc = "desc INTSEL."]
        #[must_use]
        #[inline(always)]
        pub const fn intsel(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x01ff;
            val as u16
        }
        #[doc = "desc INTSEL."]
        #[inline(always)]
        pub const fn set_intsel(&mut self, val: u16) {
            self.0 = (self.0 & !(0x01ff << 0usize)) | (((val as u32) & 0x01ff) << 0usize);
        }
    }
    impl Default for Intsel7 {
        #[inline(always)]
        fn default() -> Intsel7 {
            Intsel7(0)
        }
    }
    impl core::fmt::Debug for Intsel7 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Intsel7")
                .field("intsel", &self.intsel())
                .finish()
        }
    }
    #[doc = "desc INTSEL70."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Intsel70(pub u32);
    impl Intsel70 {
        #[doc = "desc INTSEL."]
        #[must_use]
        #[inline(always)]
        pub const fn intsel(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x01ff;
            val as u16
        }
        #[doc = "desc INTSEL."]
        #[inline(always)]
        pub const fn set_intsel(&mut self, val: u16) {
            self.0 = (self.0 & !(0x01ff << 0usize)) | (((val as u32) & 0x01ff) << 0usize);
        }
    }
    impl Default for Intsel70 {
        #[inline(always)]
        fn default() -> Intsel70 {
            Intsel70(0)
        }
    }
    impl core::fmt::Debug for Intsel70 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Intsel70")
                .field("intsel", &self.intsel())
                .finish()
        }
    }
    #[doc = "desc INTSEL71."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Intsel71(pub u32);
    impl Intsel71 {
        #[doc = "desc INTSEL."]
        #[must_use]
        #[inline(always)]
        pub const fn intsel(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x01ff;
            val as u16
        }
        #[doc = "desc INTSEL."]
        #[inline(always)]
        pub const fn set_intsel(&mut self, val: u16) {
            self.0 = (self.0 & !(0x01ff << 0usize)) | (((val as u32) & 0x01ff) << 0usize);
        }
    }
    impl Default for Intsel71 {
        #[inline(always)]
        fn default() -> Intsel71 {
            Intsel71(0)
        }
    }
    impl core::fmt::Debug for Intsel71 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Intsel71")
                .field("intsel", &self.intsel())
                .finish()
        }
    }
    #[doc = "desc INTSEL72."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Intsel72(pub u32);
    impl Intsel72 {
        #[doc = "desc INTSEL."]
        #[must_use]
        #[inline(always)]
        pub const fn intsel(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x01ff;
            val as u16
        }
        #[doc = "desc INTSEL."]
        #[inline(always)]
        pub const fn set_intsel(&mut self, val: u16) {
            self.0 = (self.0 & !(0x01ff << 0usize)) | (((val as u32) & 0x01ff) << 0usize);
        }
    }
    impl Default for Intsel72 {
        #[inline(always)]
        fn default() -> Intsel72 {
            Intsel72(0)
        }
    }
    impl core::fmt::Debug for Intsel72 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Intsel72")
                .field("intsel", &self.intsel())
                .finish()
        }
    }
    #[doc = "desc INTSEL73."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Intsel73(pub u32);
    impl Intsel73 {
        #[doc = "desc INTSEL."]
        #[must_use]
        #[inline(always)]
        pub const fn intsel(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x01ff;
            val as u16
        }
        #[doc = "desc INTSEL."]
        #[inline(always)]
        pub const fn set_intsel(&mut self, val: u16) {
            self.0 = (self.0 & !(0x01ff << 0usize)) | (((val as u32) & 0x01ff) << 0usize);
        }
    }
    impl Default for Intsel73 {
        #[inline(always)]
        fn default() -> Intsel73 {
            Intsel73(0)
        }
    }
    impl core::fmt::Debug for Intsel73 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Intsel73")
                .field("intsel", &self.intsel())
                .finish()
        }
    }
    #[doc = "desc INTSEL74."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Intsel74(pub u32);
    impl Intsel74 {
        #[doc = "desc INTSEL."]
        #[must_use]
        #[inline(always)]
        pub const fn intsel(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x01ff;
            val as u16
        }
        #[doc = "desc INTSEL."]
        #[inline(always)]
        pub const fn set_intsel(&mut self, val: u16) {
            self.0 = (self.0 & !(0x01ff << 0usize)) | (((val as u32) & 0x01ff) << 0usize);
        }
    }
    impl Default for Intsel74 {
        #[inline(always)]
        fn default() -> Intsel74 {
            Intsel74(0)
        }
    }
    impl core::fmt::Debug for Intsel74 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Intsel74")
                .field("intsel", &self.intsel())
                .finish()
        }
    }
    #[doc = "desc INTSEL75."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Intsel75(pub u32);
    impl Intsel75 {
        #[doc = "desc INTSEL."]
        #[must_use]
        #[inline(always)]
        pub const fn intsel(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x01ff;
            val as u16
        }
        #[doc = "desc INTSEL."]
        #[inline(always)]
        pub const fn set_intsel(&mut self, val: u16) {
            self.0 = (self.0 & !(0x01ff << 0usize)) | (((val as u32) & 0x01ff) << 0usize);
        }
    }
    impl Default for Intsel75 {
        #[inline(always)]
        fn default() -> Intsel75 {
            Intsel75(0)
        }
    }
    impl core::fmt::Debug for Intsel75 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Intsel75")
                .field("intsel", &self.intsel())
                .finish()
        }
    }
    #[doc = "desc INTSEL76."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Intsel76(pub u32);
    impl Intsel76 {
        #[doc = "desc INTSEL."]
        #[must_use]
        #[inline(always)]
        pub const fn intsel(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x01ff;
            val as u16
        }
        #[doc = "desc INTSEL."]
        #[inline(always)]
        pub const fn set_intsel(&mut self, val: u16) {
            self.0 = (self.0 & !(0x01ff << 0usize)) | (((val as u32) & 0x01ff) << 0usize);
        }
    }
    impl Default for Intsel76 {
        #[inline(always)]
        fn default() -> Intsel76 {
            Intsel76(0)
        }
    }
    impl core::fmt::Debug for Intsel76 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Intsel76")
                .field("intsel", &self.intsel())
                .finish()
        }
    }
    #[doc = "desc INTSEL77."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Intsel77(pub u32);
    impl Intsel77 {
        #[doc = "desc INTSEL."]
        #[must_use]
        #[inline(always)]
        pub const fn intsel(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x01ff;
            val as u16
        }
        #[doc = "desc INTSEL."]
        #[inline(always)]
        pub const fn set_intsel(&mut self, val: u16) {
            self.0 = (self.0 & !(0x01ff << 0usize)) | (((val as u32) & 0x01ff) << 0usize);
        }
    }
    impl Default for Intsel77 {
        #[inline(always)]
        fn default() -> Intsel77 {
            Intsel77(0)
        }
    }
    impl core::fmt::Debug for Intsel77 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Intsel77")
                .field("intsel", &self.intsel())
                .finish()
        }
    }
    #[doc = "desc INTSEL78."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Intsel78(pub u32);
    impl Intsel78 {
        #[doc = "desc INTSEL."]
        #[must_use]
        #[inline(always)]
        pub const fn intsel(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x01ff;
            val as u16
        }
        #[doc = "desc INTSEL."]
        #[inline(always)]
        pub const fn set_intsel(&mut self, val: u16) {
            self.0 = (self.0 & !(0x01ff << 0usize)) | (((val as u32) & 0x01ff) << 0usize);
        }
    }
    impl Default for Intsel78 {
        #[inline(always)]
        fn default() -> Intsel78 {
            Intsel78(0)
        }
    }
    impl core::fmt::Debug for Intsel78 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Intsel78")
                .field("intsel", &self.intsel())
                .finish()
        }
    }
    #[doc = "desc INTSEL79."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Intsel79(pub u32);
    impl Intsel79 {
        #[doc = "desc INTSEL."]
        #[must_use]
        #[inline(always)]
        pub const fn intsel(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x01ff;
            val as u16
        }
        #[doc = "desc INTSEL."]
        #[inline(always)]
        pub const fn set_intsel(&mut self, val: u16) {
            self.0 = (self.0 & !(0x01ff << 0usize)) | (((val as u32) & 0x01ff) << 0usize);
        }
    }
    impl Default for Intsel79 {
        #[inline(always)]
        fn default() -> Intsel79 {
            Intsel79(0)
        }
    }
    impl core::fmt::Debug for Intsel79 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Intsel79")
                .field("intsel", &self.intsel())
                .finish()
        }
    }
    #[doc = "desc INTSEL8."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Intsel8(pub u32);
    impl Intsel8 {
        #[doc = "desc INTSEL."]
        #[must_use]
        #[inline(always)]
        pub const fn intsel(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x01ff;
            val as u16
        }
        #[doc = "desc INTSEL."]
        #[inline(always)]
        pub const fn set_intsel(&mut self, val: u16) {
            self.0 = (self.0 & !(0x01ff << 0usize)) | (((val as u32) & 0x01ff) << 0usize);
        }
    }
    impl Default for Intsel8 {
        #[inline(always)]
        fn default() -> Intsel8 {
            Intsel8(0)
        }
    }
    impl core::fmt::Debug for Intsel8 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Intsel8")
                .field("intsel", &self.intsel())
                .finish()
        }
    }
    #[doc = "desc INTSEL80."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Intsel80(pub u32);
    impl Intsel80 {
        #[doc = "desc INTSEL."]
        #[must_use]
        #[inline(always)]
        pub const fn intsel(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x01ff;
            val as u16
        }
        #[doc = "desc INTSEL."]
        #[inline(always)]
        pub const fn set_intsel(&mut self, val: u16) {
            self.0 = (self.0 & !(0x01ff << 0usize)) | (((val as u32) & 0x01ff) << 0usize);
        }
    }
    impl Default for Intsel80 {
        #[inline(always)]
        fn default() -> Intsel80 {
            Intsel80(0)
        }
    }
    impl core::fmt::Debug for Intsel80 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Intsel80")
                .field("intsel", &self.intsel())
                .finish()
        }
    }
    #[doc = "desc INTSEL81."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Intsel81(pub u32);
    impl Intsel81 {
        #[doc = "desc INTSEL."]
        #[must_use]
        #[inline(always)]
        pub const fn intsel(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x01ff;
            val as u16
        }
        #[doc = "desc INTSEL."]
        #[inline(always)]
        pub const fn set_intsel(&mut self, val: u16) {
            self.0 = (self.0 & !(0x01ff << 0usize)) | (((val as u32) & 0x01ff) << 0usize);
        }
    }
    impl Default for Intsel81 {
        #[inline(always)]
        fn default() -> Intsel81 {
            Intsel81(0)
        }
    }
    impl core::fmt::Debug for Intsel81 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Intsel81")
                .field("intsel", &self.intsel())
                .finish()
        }
    }
    #[doc = "desc INTSEL82."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Intsel82(pub u32);
    impl Intsel82 {
        #[doc = "desc INTSEL."]
        #[must_use]
        #[inline(always)]
        pub const fn intsel(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x01ff;
            val as u16
        }
        #[doc = "desc INTSEL."]
        #[inline(always)]
        pub const fn set_intsel(&mut self, val: u16) {
            self.0 = (self.0 & !(0x01ff << 0usize)) | (((val as u32) & 0x01ff) << 0usize);
        }
    }
    impl Default for Intsel82 {
        #[inline(always)]
        fn default() -> Intsel82 {
            Intsel82(0)
        }
    }
    impl core::fmt::Debug for Intsel82 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Intsel82")
                .field("intsel", &self.intsel())
                .finish()
        }
    }
    #[doc = "desc INTSEL83."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Intsel83(pub u32);
    impl Intsel83 {
        #[doc = "desc INTSEL."]
        #[must_use]
        #[inline(always)]
        pub const fn intsel(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x01ff;
            val as u16
        }
        #[doc = "desc INTSEL."]
        #[inline(always)]
        pub const fn set_intsel(&mut self, val: u16) {
            self.0 = (self.0 & !(0x01ff << 0usize)) | (((val as u32) & 0x01ff) << 0usize);
        }
    }
    impl Default for Intsel83 {
        #[inline(always)]
        fn default() -> Intsel83 {
            Intsel83(0)
        }
    }
    impl core::fmt::Debug for Intsel83 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Intsel83")
                .field("intsel", &self.intsel())
                .finish()
        }
    }
    #[doc = "desc INTSEL84."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Intsel84(pub u32);
    impl Intsel84 {
        #[doc = "desc INTSEL."]
        #[must_use]
        #[inline(always)]
        pub const fn intsel(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x01ff;
            val as u16
        }
        #[doc = "desc INTSEL."]
        #[inline(always)]
        pub const fn set_intsel(&mut self, val: u16) {
            self.0 = (self.0 & !(0x01ff << 0usize)) | (((val as u32) & 0x01ff) << 0usize);
        }
    }
    impl Default for Intsel84 {
        #[inline(always)]
        fn default() -> Intsel84 {
            Intsel84(0)
        }
    }
    impl core::fmt::Debug for Intsel84 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Intsel84")
                .field("intsel", &self.intsel())
                .finish()
        }
    }
    #[doc = "desc INTSEL85."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Intsel85(pub u32);
    impl Intsel85 {
        #[doc = "desc INTSEL."]
        #[must_use]
        #[inline(always)]
        pub const fn intsel(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x01ff;
            val as u16
        }
        #[doc = "desc INTSEL."]
        #[inline(always)]
        pub const fn set_intsel(&mut self, val: u16) {
            self.0 = (self.0 & !(0x01ff << 0usize)) | (((val as u32) & 0x01ff) << 0usize);
        }
    }
    impl Default for Intsel85 {
        #[inline(always)]
        fn default() -> Intsel85 {
            Intsel85(0)
        }
    }
    impl core::fmt::Debug for Intsel85 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Intsel85")
                .field("intsel", &self.intsel())
                .finish()
        }
    }
    #[doc = "desc INTSEL86."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Intsel86(pub u32);
    impl Intsel86 {
        #[doc = "desc INTSEL."]
        #[must_use]
        #[inline(always)]
        pub const fn intsel(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x01ff;
            val as u16
        }
        #[doc = "desc INTSEL."]
        #[inline(always)]
        pub const fn set_intsel(&mut self, val: u16) {
            self.0 = (self.0 & !(0x01ff << 0usize)) | (((val as u32) & 0x01ff) << 0usize);
        }
    }
    impl Default for Intsel86 {
        #[inline(always)]
        fn default() -> Intsel86 {
            Intsel86(0)
        }
    }
    impl core::fmt::Debug for Intsel86 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Intsel86")
                .field("intsel", &self.intsel())
                .finish()
        }
    }
    #[doc = "desc INTSEL87."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Intsel87(pub u32);
    impl Intsel87 {
        #[doc = "desc INTSEL."]
        #[must_use]
        #[inline(always)]
        pub const fn intsel(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x01ff;
            val as u16
        }
        #[doc = "desc INTSEL."]
        #[inline(always)]
        pub const fn set_intsel(&mut self, val: u16) {
            self.0 = (self.0 & !(0x01ff << 0usize)) | (((val as u32) & 0x01ff) << 0usize);
        }
    }
    impl Default for Intsel87 {
        #[inline(always)]
        fn default() -> Intsel87 {
            Intsel87(0)
        }
    }
    impl core::fmt::Debug for Intsel87 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Intsel87")
                .field("intsel", &self.intsel())
                .finish()
        }
    }
    #[doc = "desc INTSEL88."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Intsel88(pub u32);
    impl Intsel88 {
        #[doc = "desc INTSEL."]
        #[must_use]
        #[inline(always)]
        pub const fn intsel(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x01ff;
            val as u16
        }
        #[doc = "desc INTSEL."]
        #[inline(always)]
        pub const fn set_intsel(&mut self, val: u16) {
            self.0 = (self.0 & !(0x01ff << 0usize)) | (((val as u32) & 0x01ff) << 0usize);
        }
    }
    impl Default for Intsel88 {
        #[inline(always)]
        fn default() -> Intsel88 {
            Intsel88(0)
        }
    }
    impl core::fmt::Debug for Intsel88 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Intsel88")
                .field("intsel", &self.intsel())
                .finish()
        }
    }
    #[doc = "desc INTSEL89."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Intsel89(pub u32);
    impl Intsel89 {
        #[doc = "desc INTSEL."]
        #[must_use]
        #[inline(always)]
        pub const fn intsel(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x01ff;
            val as u16
        }
        #[doc = "desc INTSEL."]
        #[inline(always)]
        pub const fn set_intsel(&mut self, val: u16) {
            self.0 = (self.0 & !(0x01ff << 0usize)) | (((val as u32) & 0x01ff) << 0usize);
        }
    }
    impl Default for Intsel89 {
        #[inline(always)]
        fn default() -> Intsel89 {
            Intsel89(0)
        }
    }
    impl core::fmt::Debug for Intsel89 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Intsel89")
                .field("intsel", &self.intsel())
                .finish()
        }
    }
    #[doc = "desc INTSEL9."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Intsel9(pub u32);
    impl Intsel9 {
        #[doc = "desc INTSEL."]
        #[must_use]
        #[inline(always)]
        pub const fn intsel(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x01ff;
            val as u16
        }
        #[doc = "desc INTSEL."]
        #[inline(always)]
        pub const fn set_intsel(&mut self, val: u16) {
            self.0 = (self.0 & !(0x01ff << 0usize)) | (((val as u32) & 0x01ff) << 0usize);
        }
    }
    impl Default for Intsel9 {
        #[inline(always)]
        fn default() -> Intsel9 {
            Intsel9(0)
        }
    }
    impl core::fmt::Debug for Intsel9 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Intsel9")
                .field("intsel", &self.intsel())
                .finish()
        }
    }
    #[doc = "desc INTSEL90."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Intsel90(pub u32);
    impl Intsel90 {
        #[doc = "desc INTSEL."]
        #[must_use]
        #[inline(always)]
        pub const fn intsel(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x01ff;
            val as u16
        }
        #[doc = "desc INTSEL."]
        #[inline(always)]
        pub const fn set_intsel(&mut self, val: u16) {
            self.0 = (self.0 & !(0x01ff << 0usize)) | (((val as u32) & 0x01ff) << 0usize);
        }
    }
    impl Default for Intsel90 {
        #[inline(always)]
        fn default() -> Intsel90 {
            Intsel90(0)
        }
    }
    impl core::fmt::Debug for Intsel90 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Intsel90")
                .field("intsel", &self.intsel())
                .finish()
        }
    }
    #[doc = "desc INTSEL91."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Intsel91(pub u32);
    impl Intsel91 {
        #[doc = "desc INTSEL."]
        #[must_use]
        #[inline(always)]
        pub const fn intsel(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x01ff;
            val as u16
        }
        #[doc = "desc INTSEL."]
        #[inline(always)]
        pub const fn set_intsel(&mut self, val: u16) {
            self.0 = (self.0 & !(0x01ff << 0usize)) | (((val as u32) & 0x01ff) << 0usize);
        }
    }
    impl Default for Intsel91 {
        #[inline(always)]
        fn default() -> Intsel91 {
            Intsel91(0)
        }
    }
    impl core::fmt::Debug for Intsel91 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Intsel91")
                .field("intsel", &self.intsel())
                .finish()
        }
    }
    #[doc = "desc INTSEL92."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Intsel92(pub u32);
    impl Intsel92 {
        #[doc = "desc INTSEL."]
        #[must_use]
        #[inline(always)]
        pub const fn intsel(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x01ff;
            val as u16
        }
        #[doc = "desc INTSEL."]
        #[inline(always)]
        pub const fn set_intsel(&mut self, val: u16) {
            self.0 = (self.0 & !(0x01ff << 0usize)) | (((val as u32) & 0x01ff) << 0usize);
        }
    }
    impl Default for Intsel92 {
        #[inline(always)]
        fn default() -> Intsel92 {
            Intsel92(0)
        }
    }
    impl core::fmt::Debug for Intsel92 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Intsel92")
                .field("intsel", &self.intsel())
                .finish()
        }
    }
    #[doc = "desc INTSEL93."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Intsel93(pub u32);
    impl Intsel93 {
        #[doc = "desc INTSEL."]
        #[must_use]
        #[inline(always)]
        pub const fn intsel(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x01ff;
            val as u16
        }
        #[doc = "desc INTSEL."]
        #[inline(always)]
        pub const fn set_intsel(&mut self, val: u16) {
            self.0 = (self.0 & !(0x01ff << 0usize)) | (((val as u32) & 0x01ff) << 0usize);
        }
    }
    impl Default for Intsel93 {
        #[inline(always)]
        fn default() -> Intsel93 {
            Intsel93(0)
        }
    }
    impl core::fmt::Debug for Intsel93 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Intsel93")
                .field("intsel", &self.intsel())
                .finish()
        }
    }
    #[doc = "desc INTSEL94."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Intsel94(pub u32);
    impl Intsel94 {
        #[doc = "desc INTSEL."]
        #[must_use]
        #[inline(always)]
        pub const fn intsel(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x01ff;
            val as u16
        }
        #[doc = "desc INTSEL."]
        #[inline(always)]
        pub const fn set_intsel(&mut self, val: u16) {
            self.0 = (self.0 & !(0x01ff << 0usize)) | (((val as u32) & 0x01ff) << 0usize);
        }
    }
    impl Default for Intsel94 {
        #[inline(always)]
        fn default() -> Intsel94 {
            Intsel94(0)
        }
    }
    impl core::fmt::Debug for Intsel94 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Intsel94")
                .field("intsel", &self.intsel())
                .finish()
        }
    }
    #[doc = "desc INTSEL95."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Intsel95(pub u32);
    impl Intsel95 {
        #[doc = "desc INTSEL."]
        #[must_use]
        #[inline(always)]
        pub const fn intsel(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x01ff;
            val as u16
        }
        #[doc = "desc INTSEL."]
        #[inline(always)]
        pub const fn set_intsel(&mut self, val: u16) {
            self.0 = (self.0 & !(0x01ff << 0usize)) | (((val as u32) & 0x01ff) << 0usize);
        }
    }
    impl Default for Intsel95 {
        #[inline(always)]
        fn default() -> Intsel95 {
            Intsel95(0)
        }
    }
    impl core::fmt::Debug for Intsel95 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Intsel95")
                .field("intsel", &self.intsel())
                .finish()
        }
    }
    #[doc = "desc INTSEL96."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Intsel96(pub u32);
    impl Intsel96 {
        #[doc = "desc INTSEL."]
        #[must_use]
        #[inline(always)]
        pub const fn intsel(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x01ff;
            val as u16
        }
        #[doc = "desc INTSEL."]
        #[inline(always)]
        pub const fn set_intsel(&mut self, val: u16) {
            self.0 = (self.0 & !(0x01ff << 0usize)) | (((val as u32) & 0x01ff) << 0usize);
        }
    }
    impl Default for Intsel96 {
        #[inline(always)]
        fn default() -> Intsel96 {
            Intsel96(0)
        }
    }
    impl core::fmt::Debug for Intsel96 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Intsel96")
                .field("intsel", &self.intsel())
                .finish()
        }
    }
    #[doc = "desc INTSEL97."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Intsel97(pub u32);
    impl Intsel97 {
        #[doc = "desc INTSEL."]
        #[must_use]
        #[inline(always)]
        pub const fn intsel(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x01ff;
            val as u16
        }
        #[doc = "desc INTSEL."]
        #[inline(always)]
        pub const fn set_intsel(&mut self, val: u16) {
            self.0 = (self.0 & !(0x01ff << 0usize)) | (((val as u32) & 0x01ff) << 0usize);
        }
    }
    impl Default for Intsel97 {
        #[inline(always)]
        fn default() -> Intsel97 {
            Intsel97(0)
        }
    }
    impl core::fmt::Debug for Intsel97 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Intsel97")
                .field("intsel", &self.intsel())
                .finish()
        }
    }
    #[doc = "desc INTSEL98."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Intsel98(pub u32);
    impl Intsel98 {
        #[doc = "desc INTSEL."]
        #[must_use]
        #[inline(always)]
        pub const fn intsel(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x01ff;
            val as u16
        }
        #[doc = "desc INTSEL."]
        #[inline(always)]
        pub const fn set_intsel(&mut self, val: u16) {
            self.0 = (self.0 & !(0x01ff << 0usize)) | (((val as u32) & 0x01ff) << 0usize);
        }
    }
    impl Default for Intsel98 {
        #[inline(always)]
        fn default() -> Intsel98 {
            Intsel98(0)
        }
    }
    impl core::fmt::Debug for Intsel98 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Intsel98")
                .field("intsel", &self.intsel())
                .finish()
        }
    }
    #[doc = "desc INTSEL99."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Intsel99(pub u32);
    impl Intsel99 {
        #[doc = "desc INTSEL."]
        #[must_use]
        #[inline(always)]
        pub const fn intsel(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x01ff;
            val as u16
        }
        #[doc = "desc INTSEL."]
        #[inline(always)]
        pub const fn set_intsel(&mut self, val: u16) {
            self.0 = (self.0 & !(0x01ff << 0usize)) | (((val as u32) & 0x01ff) << 0usize);
        }
    }
    impl Default for Intsel99 {
        #[inline(always)]
        fn default() -> Intsel99 {
            Intsel99(0)
        }
    }
    impl core::fmt::Debug for Intsel99 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Intsel99")
                .field("intsel", &self.intsel())
                .finish()
        }
    }
    #[doc = "desc NMIER."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Nmier(pub u32);
    impl Nmier {
        #[doc = "desc SWDTEN."]
        #[must_use]
        #[inline(always)]
        pub const fn swdten(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "desc SWDTEN."]
        #[inline(always)]
        pub const fn set_swdten(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "desc PVD1EN."]
        #[must_use]
        #[inline(always)]
        pub const fn pvd1en(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "desc PVD1EN."]
        #[inline(always)]
        pub const fn set_pvd1en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "desc PVD2EN."]
        #[must_use]
        #[inline(always)]
        pub const fn pvd2en(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "desc PVD2EN."]
        #[inline(always)]
        pub const fn set_pvd2en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "desc XTALSTPEN."]
        #[must_use]
        #[inline(always)]
        pub const fn xtalstpen(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "desc XTALSTPEN."]
        #[inline(always)]
        pub const fn set_xtalstpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "desc RECCERREN."]
        #[must_use]
        #[inline(always)]
        pub const fn reccerren(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "desc RECCERREN."]
        #[inline(always)]
        pub const fn set_reccerren(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "desc BUSERREN."]
        #[must_use]
        #[inline(always)]
        pub const fn buserren(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "desc BUSERREN."]
        #[inline(always)]
        pub const fn set_buserren(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[doc = "desc WDTEN."]
        #[must_use]
        #[inline(always)]
        pub const fn wdten(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "desc WDTEN."]
        #[inline(always)]
        pub const fn set_wdten(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[doc = "desc FECCERREN."]
        #[must_use]
        #[inline(always)]
        pub const fn feccerren(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "desc FECCERREN."]
        #[inline(always)]
        pub const fn set_feccerren(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "desc ERMUEN."]
        #[must_use]
        #[inline(always)]
        pub const fn ermuen(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[doc = "desc ERMUEN."]
        #[inline(always)]
        pub const fn set_ermuen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
    }
    impl Default for Nmier {
        #[inline(always)]
        fn default() -> Nmier {
            Nmier(0)
        }
    }
    impl core::fmt::Debug for Nmier {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Nmier")
                .field("swdten", &self.swdten())
                .field("pvd1en", &self.pvd1en())
                .field("pvd2en", &self.pvd2en())
                .field("xtalstpen", &self.xtalstpen())
                .field("reccerren", &self.reccerren())
                .field("buserren", &self.buserren())
                .field("wdten", &self.wdten())
                .field("feccerren", &self.feccerren())
                .field("ermuen", &self.ermuen())
                .finish()
        }
    }
    #[doc = "desc NMIFCR."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Nmifcr(pub u32);
    impl Nmifcr {
        #[doc = "desc SWDTCLR."]
        #[must_use]
        #[inline(always)]
        pub const fn swdtclr(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "desc SWDTCLR."]
        #[inline(always)]
        pub const fn set_swdtclr(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "desc PVD1CLR."]
        #[must_use]
        #[inline(always)]
        pub const fn pvd1clr(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "desc PVD1CLR."]
        #[inline(always)]
        pub const fn set_pvd1clr(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "desc PVD2CLR."]
        #[must_use]
        #[inline(always)]
        pub const fn pvd2clr(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "desc PVD2CLR."]
        #[inline(always)]
        pub const fn set_pvd2clr(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "desc XTALSTPCLR."]
        #[must_use]
        #[inline(always)]
        pub const fn xtalstpclr(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "desc XTALSTPCLR."]
        #[inline(always)]
        pub const fn set_xtalstpclr(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "desc RECCERRCLR."]
        #[must_use]
        #[inline(always)]
        pub const fn reccerrclr(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "desc RECCERRCLR."]
        #[inline(always)]
        pub const fn set_reccerrclr(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "desc BUSERRCLR."]
        #[must_use]
        #[inline(always)]
        pub const fn buserrclr(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "desc BUSERRCLR."]
        #[inline(always)]
        pub const fn set_buserrclr(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[doc = "desc WDTCLR."]
        #[must_use]
        #[inline(always)]
        pub const fn wdtclr(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "desc WDTCLR."]
        #[inline(always)]
        pub const fn set_wdtclr(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[doc = "desc FECCERRCLR."]
        #[must_use]
        #[inline(always)]
        pub const fn feccerrclr(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "desc FECCERRCLR."]
        #[inline(always)]
        pub const fn set_feccerrclr(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "desc ERMUCLR."]
        #[must_use]
        #[inline(always)]
        pub const fn ermuclr(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[doc = "desc ERMUCLR."]
        #[inline(always)]
        pub const fn set_ermuclr(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
    }
    impl Default for Nmifcr {
        #[inline(always)]
        fn default() -> Nmifcr {
            Nmifcr(0)
        }
    }
    impl core::fmt::Debug for Nmifcr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Nmifcr")
                .field("swdtclr", &self.swdtclr())
                .field("pvd1clr", &self.pvd1clr())
                .field("pvd2clr", &self.pvd2clr())
                .field("xtalstpclr", &self.xtalstpclr())
                .field("reccerrclr", &self.reccerrclr())
                .field("buserrclr", &self.buserrclr())
                .field("wdtclr", &self.wdtclr())
                .field("feccerrclr", &self.feccerrclr())
                .field("ermuclr", &self.ermuclr())
                .finish()
        }
    }
    #[doc = "desc NMIFR."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Nmifr(pub u32);
    impl Nmifr {
        #[doc = "desc SWDTF."]
        #[must_use]
        #[inline(always)]
        pub const fn swdtf(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "desc SWDTF."]
        #[inline(always)]
        pub const fn set_swdtf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "desc PVD1F."]
        #[must_use]
        #[inline(always)]
        pub const fn pvd1f(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "desc PVD1F."]
        #[inline(always)]
        pub const fn set_pvd1f(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "desc PVD2F."]
        #[must_use]
        #[inline(always)]
        pub const fn pvd2f(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "desc PVD2F."]
        #[inline(always)]
        pub const fn set_pvd2f(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "desc XTALSTPF."]
        #[must_use]
        #[inline(always)]
        pub const fn xtalstpf(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "desc XTALSTPF."]
        #[inline(always)]
        pub const fn set_xtalstpf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "desc RECCERRF."]
        #[must_use]
        #[inline(always)]
        pub const fn reccerrf(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "desc RECCERRF."]
        #[inline(always)]
        pub const fn set_reccerrf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "desc BUSERRF."]
        #[must_use]
        #[inline(always)]
        pub const fn buserrf(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "desc BUSERRF."]
        #[inline(always)]
        pub const fn set_buserrf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[doc = "desc WDTF."]
        #[must_use]
        #[inline(always)]
        pub const fn wdtf(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "desc WDTF."]
        #[inline(always)]
        pub const fn set_wdtf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[doc = "desc FECCERRF."]
        #[must_use]
        #[inline(always)]
        pub const fn feccerrf(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "desc FECCERRF."]
        #[inline(always)]
        pub const fn set_feccerrf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "desc ERMUF."]
        #[must_use]
        #[inline(always)]
        pub const fn ermuf(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[doc = "desc ERMUF."]
        #[inline(always)]
        pub const fn set_ermuf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
    }
    impl Default for Nmifr {
        #[inline(always)]
        fn default() -> Nmifr {
            Nmifr(0)
        }
    }
    impl core::fmt::Debug for Nmifr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Nmifr")
                .field("swdtf", &self.swdtf())
                .field("pvd1f", &self.pvd1f())
                .field("pvd2f", &self.pvd2f())
                .field("xtalstpf", &self.xtalstpf())
                .field("reccerrf", &self.reccerrf())
                .field("buserrf", &self.buserrf())
                .field("wdtf", &self.wdtf())
                .field("feccerrf", &self.feccerrf())
                .field("ermuf", &self.ermuf())
                .finish()
        }
    }
    #[doc = "desc NOCCR."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Noccr(pub u32);
    impl Noccr {
        #[doc = "desc NOCSEL."]
        #[must_use]
        #[inline(always)]
        pub const fn nocsel(&self) -> u8 {
            let val = (self.0 >> 12usize) & 0x03;
            val as u8
        }
        #[doc = "desc NOCSEL."]
        #[inline(always)]
        pub const fn set_nocsel(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 12usize)) | (((val as u32) & 0x03) << 12usize);
        }
    }
    impl Default for Noccr {
        #[inline(always)]
        fn default() -> Noccr {
            Noccr(0)
        }
    }
    impl core::fmt::Debug for Noccr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Noccr")
                .field("nocsel", &self.nocsel())
                .finish()
        }
    }
    #[doc = "desc SWIER."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Swier(pub u32);
    impl Swier {
        #[doc = "desc SWIE0."]
        #[must_use]
        #[inline(always)]
        pub const fn swie0(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "desc SWIE0."]
        #[inline(always)]
        pub const fn set_swie0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "desc SWIE1."]
        #[must_use]
        #[inline(always)]
        pub const fn swie1(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "desc SWIE1."]
        #[inline(always)]
        pub const fn set_swie1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "desc SWIE2."]
        #[must_use]
        #[inline(always)]
        pub const fn swie2(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "desc SWIE2."]
        #[inline(always)]
        pub const fn set_swie2(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "desc SWIE3."]
        #[must_use]
        #[inline(always)]
        pub const fn swie3(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "desc SWIE3."]
        #[inline(always)]
        pub const fn set_swie3(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "desc SWIE4."]
        #[must_use]
        #[inline(always)]
        pub const fn swie4(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "desc SWIE4."]
        #[inline(always)]
        pub const fn set_swie4(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "desc SWIE5."]
        #[must_use]
        #[inline(always)]
        pub const fn swie5(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "desc SWIE5."]
        #[inline(always)]
        pub const fn set_swie5(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "desc SWIE6."]
        #[must_use]
        #[inline(always)]
        pub const fn swie6(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "desc SWIE6."]
        #[inline(always)]
        pub const fn set_swie6(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "desc SWIE7."]
        #[must_use]
        #[inline(always)]
        pub const fn swie7(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "desc SWIE7."]
        #[inline(always)]
        pub const fn set_swie7(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "desc SWIE8."]
        #[must_use]
        #[inline(always)]
        pub const fn swie8(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "desc SWIE8."]
        #[inline(always)]
        pub const fn set_swie8(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "desc SWIE9."]
        #[must_use]
        #[inline(always)]
        pub const fn swie9(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "desc SWIE9."]
        #[inline(always)]
        pub const fn set_swie9(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "desc SWIE10."]
        #[must_use]
        #[inline(always)]
        pub const fn swie10(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "desc SWIE10."]
        #[inline(always)]
        pub const fn set_swie10(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[doc = "desc SWIE11."]
        #[must_use]
        #[inline(always)]
        pub const fn swie11(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "desc SWIE11."]
        #[inline(always)]
        pub const fn set_swie11(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[doc = "desc SWIE12."]
        #[must_use]
        #[inline(always)]
        pub const fn swie12(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "desc SWIE12."]
        #[inline(always)]
        pub const fn set_swie12(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "desc SWIE13."]
        #[must_use]
        #[inline(always)]
        pub const fn swie13(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[doc = "desc SWIE13."]
        #[inline(always)]
        pub const fn set_swie13(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[doc = "desc SWIE14."]
        #[must_use]
        #[inline(always)]
        pub const fn swie14(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "desc SWIE14."]
        #[inline(always)]
        pub const fn set_swie14(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[doc = "desc SWIE15."]
        #[must_use]
        #[inline(always)]
        pub const fn swie15(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "desc SWIE15."]
        #[inline(always)]
        pub const fn set_swie15(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
        #[doc = "desc SWIE16."]
        #[must_use]
        #[inline(always)]
        pub const fn swie16(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "desc SWIE16."]
        #[inline(always)]
        pub const fn set_swie16(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "desc SWIE17."]
        #[must_use]
        #[inline(always)]
        pub const fn swie17(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "desc SWIE17."]
        #[inline(always)]
        pub const fn set_swie17(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[doc = "desc SWIE18."]
        #[must_use]
        #[inline(always)]
        pub const fn swie18(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[doc = "desc SWIE18."]
        #[inline(always)]
        pub const fn set_swie18(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
        #[doc = "desc SWIE19."]
        #[must_use]
        #[inline(always)]
        pub const fn swie19(&self) -> bool {
            let val = (self.0 >> 19usize) & 0x01;
            val != 0
        }
        #[doc = "desc SWIE19."]
        #[inline(always)]
        pub const fn set_swie19(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
        }
        #[doc = "desc SWIE20."]
        #[must_use]
        #[inline(always)]
        pub const fn swie20(&self) -> bool {
            let val = (self.0 >> 20usize) & 0x01;
            val != 0
        }
        #[doc = "desc SWIE20."]
        #[inline(always)]
        pub const fn set_swie20(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
        }
        #[doc = "desc SWIE21."]
        #[must_use]
        #[inline(always)]
        pub const fn swie21(&self) -> bool {
            let val = (self.0 >> 21usize) & 0x01;
            val != 0
        }
        #[doc = "desc SWIE21."]
        #[inline(always)]
        pub const fn set_swie21(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
        }
        #[doc = "desc SWIE22."]
        #[must_use]
        #[inline(always)]
        pub const fn swie22(&self) -> bool {
            let val = (self.0 >> 22usize) & 0x01;
            val != 0
        }
        #[doc = "desc SWIE22."]
        #[inline(always)]
        pub const fn set_swie22(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
        }
        #[doc = "desc SWIE23."]
        #[must_use]
        #[inline(always)]
        pub const fn swie23(&self) -> bool {
            let val = (self.0 >> 23usize) & 0x01;
            val != 0
        }
        #[doc = "desc SWIE23."]
        #[inline(always)]
        pub const fn set_swie23(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
        }
        #[doc = "desc SWIE24."]
        #[must_use]
        #[inline(always)]
        pub const fn swie24(&self) -> bool {
            let val = (self.0 >> 24usize) & 0x01;
            val != 0
        }
        #[doc = "desc SWIE24."]
        #[inline(always)]
        pub const fn set_swie24(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
        }
        #[doc = "desc SWIE25."]
        #[must_use]
        #[inline(always)]
        pub const fn swie25(&self) -> bool {
            let val = (self.0 >> 25usize) & 0x01;
            val != 0
        }
        #[doc = "desc SWIE25."]
        #[inline(always)]
        pub const fn set_swie25(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
        }
        #[doc = "desc SWIE26."]
        #[must_use]
        #[inline(always)]
        pub const fn swie26(&self) -> bool {
            let val = (self.0 >> 26usize) & 0x01;
            val != 0
        }
        #[doc = "desc SWIE26."]
        #[inline(always)]
        pub const fn set_swie26(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
        }
        #[doc = "desc SWIE27."]
        #[must_use]
        #[inline(always)]
        pub const fn swie27(&self) -> bool {
            let val = (self.0 >> 27usize) & 0x01;
            val != 0
        }
        #[doc = "desc SWIE27."]
        #[inline(always)]
        pub const fn set_swie27(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
        }
        #[doc = "desc SWIE28."]
        #[must_use]
        #[inline(always)]
        pub const fn swie28(&self) -> bool {
            let val = (self.0 >> 28usize) & 0x01;
            val != 0
        }
        #[doc = "desc SWIE28."]
        #[inline(always)]
        pub const fn set_swie28(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
        }
        #[doc = "desc SWIE29."]
        #[must_use]
        #[inline(always)]
        pub const fn swie29(&self) -> bool {
            let val = (self.0 >> 29usize) & 0x01;
            val != 0
        }
        #[doc = "desc SWIE29."]
        #[inline(always)]
        pub const fn set_swie29(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
        }
        #[doc = "desc SWIE30."]
        #[must_use]
        #[inline(always)]
        pub const fn swie30(&self) -> bool {
            let val = (self.0 >> 30usize) & 0x01;
            val != 0
        }
        #[doc = "desc SWIE30."]
        #[inline(always)]
        pub const fn set_swie30(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
        }
        #[doc = "desc SWIE31."]
        #[must_use]
        #[inline(always)]
        pub const fn swie31(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[doc = "desc SWIE31."]
        #[inline(always)]
        pub const fn set_swie31(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for Swier {
        #[inline(always)]
        fn default() -> Swier {
            Swier(0)
        }
    }
    impl core::fmt::Debug for Swier {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Swier")
                .field("swie0", &self.swie0())
                .field("swie1", &self.swie1())
                .field("swie2", &self.swie2())
                .field("swie3", &self.swie3())
                .field("swie4", &self.swie4())
                .field("swie5", &self.swie5())
                .field("swie6", &self.swie6())
                .field("swie7", &self.swie7())
                .field("swie8", &self.swie8())
                .field("swie9", &self.swie9())
                .field("swie10", &self.swie10())
                .field("swie11", &self.swie11())
                .field("swie12", &self.swie12())
                .field("swie13", &self.swie13())
                .field("swie14", &self.swie14())
                .field("swie15", &self.swie15())
                .field("swie16", &self.swie16())
                .field("swie17", &self.swie17())
                .field("swie18", &self.swie18())
                .field("swie19", &self.swie19())
                .field("swie20", &self.swie20())
                .field("swie21", &self.swie21())
                .field("swie22", &self.swie22())
                .field("swie23", &self.swie23())
                .field("swie24", &self.swie24())
                .field("swie25", &self.swie25())
                .field("swie26", &self.swie26())
                .field("swie27", &self.swie27())
                .field("swie28", &self.swie28())
                .field("swie29", &self.swie29())
                .field("swie30", &self.swie30())
                .field("swie31", &self.swie31())
                .finish()
        }
    }
    #[doc = "desc VSSEL128."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Vssel128(pub u32);
    impl Vssel128 {
        #[doc = "desc VSEL0."]
        #[must_use]
        #[inline(always)]
        pub const fn vsel0(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "desc VSEL0."]
        #[inline(always)]
        pub const fn set_vsel0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "desc VSEL1."]
        #[must_use]
        #[inline(always)]
        pub const fn vsel1(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "desc VSEL1."]
        #[inline(always)]
        pub const fn set_vsel1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "desc VSEL2."]
        #[must_use]
        #[inline(always)]
        pub const fn vsel2(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "desc VSEL2."]
        #[inline(always)]
        pub const fn set_vsel2(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "desc VSEL3."]
        #[must_use]
        #[inline(always)]
        pub const fn vsel3(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "desc VSEL3."]
        #[inline(always)]
        pub const fn set_vsel3(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "desc VSEL4."]
        #[must_use]
        #[inline(always)]
        pub const fn vsel4(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "desc VSEL4."]
        #[inline(always)]
        pub const fn set_vsel4(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "desc VSEL5."]
        #[must_use]
        #[inline(always)]
        pub const fn vsel5(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "desc VSEL5."]
        #[inline(always)]
        pub const fn set_vsel5(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "desc VSEL6."]
        #[must_use]
        #[inline(always)]
        pub const fn vsel6(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "desc VSEL6."]
        #[inline(always)]
        pub const fn set_vsel6(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "desc VSEL7."]
        #[must_use]
        #[inline(always)]
        pub const fn vsel7(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "desc VSEL7."]
        #[inline(always)]
        pub const fn set_vsel7(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "desc VSEL8."]
        #[must_use]
        #[inline(always)]
        pub const fn vsel8(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "desc VSEL8."]
        #[inline(always)]
        pub const fn set_vsel8(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "desc VSEL9."]
        #[must_use]
        #[inline(always)]
        pub const fn vsel9(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "desc VSEL9."]
        #[inline(always)]
        pub const fn set_vsel9(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "desc VSEL10."]
        #[must_use]
        #[inline(always)]
        pub const fn vsel10(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "desc VSEL10."]
        #[inline(always)]
        pub const fn set_vsel10(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[doc = "desc VSEL11."]
        #[must_use]
        #[inline(always)]
        pub const fn vsel11(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "desc VSEL11."]
        #[inline(always)]
        pub const fn set_vsel11(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[doc = "desc VSEL12."]
        #[must_use]
        #[inline(always)]
        pub const fn vsel12(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "desc VSEL12."]
        #[inline(always)]
        pub const fn set_vsel12(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "desc VSEL13."]
        #[must_use]
        #[inline(always)]
        pub const fn vsel13(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[doc = "desc VSEL13."]
        #[inline(always)]
        pub const fn set_vsel13(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[doc = "desc VSEL14."]
        #[must_use]
        #[inline(always)]
        pub const fn vsel14(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "desc VSEL14."]
        #[inline(always)]
        pub const fn set_vsel14(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[doc = "desc VSEL15."]
        #[must_use]
        #[inline(always)]
        pub const fn vsel15(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "desc VSEL15."]
        #[inline(always)]
        pub const fn set_vsel15(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
        #[doc = "desc VSEL16."]
        #[must_use]
        #[inline(always)]
        pub const fn vsel16(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "desc VSEL16."]
        #[inline(always)]
        pub const fn set_vsel16(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "desc VSEL17."]
        #[must_use]
        #[inline(always)]
        pub const fn vsel17(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "desc VSEL17."]
        #[inline(always)]
        pub const fn set_vsel17(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[doc = "desc VSEL18."]
        #[must_use]
        #[inline(always)]
        pub const fn vsel18(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[doc = "desc VSEL18."]
        #[inline(always)]
        pub const fn set_vsel18(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
        #[doc = "desc VSEL19."]
        #[must_use]
        #[inline(always)]
        pub const fn vsel19(&self) -> bool {
            let val = (self.0 >> 19usize) & 0x01;
            val != 0
        }
        #[doc = "desc VSEL19."]
        #[inline(always)]
        pub const fn set_vsel19(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
        }
        #[doc = "desc VSEL20."]
        #[must_use]
        #[inline(always)]
        pub const fn vsel20(&self) -> bool {
            let val = (self.0 >> 20usize) & 0x01;
            val != 0
        }
        #[doc = "desc VSEL20."]
        #[inline(always)]
        pub const fn set_vsel20(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
        }
        #[doc = "desc VSEL21."]
        #[must_use]
        #[inline(always)]
        pub const fn vsel21(&self) -> bool {
            let val = (self.0 >> 21usize) & 0x01;
            val != 0
        }
        #[doc = "desc VSEL21."]
        #[inline(always)]
        pub const fn set_vsel21(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
        }
        #[doc = "desc VSEL22."]
        #[must_use]
        #[inline(always)]
        pub const fn vsel22(&self) -> bool {
            let val = (self.0 >> 22usize) & 0x01;
            val != 0
        }
        #[doc = "desc VSEL22."]
        #[inline(always)]
        pub const fn set_vsel22(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
        }
        #[doc = "desc VSEL23."]
        #[must_use]
        #[inline(always)]
        pub const fn vsel23(&self) -> bool {
            let val = (self.0 >> 23usize) & 0x01;
            val != 0
        }
        #[doc = "desc VSEL23."]
        #[inline(always)]
        pub const fn set_vsel23(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
        }
        #[doc = "desc VSEL24."]
        #[must_use]
        #[inline(always)]
        pub const fn vsel24(&self) -> bool {
            let val = (self.0 >> 24usize) & 0x01;
            val != 0
        }
        #[doc = "desc VSEL24."]
        #[inline(always)]
        pub const fn set_vsel24(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
        }
        #[doc = "desc VSEL25."]
        #[must_use]
        #[inline(always)]
        pub const fn vsel25(&self) -> bool {
            let val = (self.0 >> 25usize) & 0x01;
            val != 0
        }
        #[doc = "desc VSEL25."]
        #[inline(always)]
        pub const fn set_vsel25(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
        }
        #[doc = "desc VSEL26."]
        #[must_use]
        #[inline(always)]
        pub const fn vsel26(&self) -> bool {
            let val = (self.0 >> 26usize) & 0x01;
            val != 0
        }
        #[doc = "desc VSEL26."]
        #[inline(always)]
        pub const fn set_vsel26(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
        }
        #[doc = "desc VSEL27."]
        #[must_use]
        #[inline(always)]
        pub const fn vsel27(&self) -> bool {
            let val = (self.0 >> 27usize) & 0x01;
            val != 0
        }
        #[doc = "desc VSEL27."]
        #[inline(always)]
        pub const fn set_vsel27(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
        }
        #[doc = "desc VSEL28."]
        #[must_use]
        #[inline(always)]
        pub const fn vsel28(&self) -> bool {
            let val = (self.0 >> 28usize) & 0x01;
            val != 0
        }
        #[doc = "desc VSEL28."]
        #[inline(always)]
        pub const fn set_vsel28(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
        }
        #[doc = "desc VSEL29."]
        #[must_use]
        #[inline(always)]
        pub const fn vsel29(&self) -> bool {
            let val = (self.0 >> 29usize) & 0x01;
            val != 0
        }
        #[doc = "desc VSEL29."]
        #[inline(always)]
        pub const fn set_vsel29(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
        }
        #[doc = "desc VSEL30."]
        #[must_use]
        #[inline(always)]
        pub const fn vsel30(&self) -> bool {
            let val = (self.0 >> 30usize) & 0x01;
            val != 0
        }
        #[doc = "desc VSEL30."]
        #[inline(always)]
        pub const fn set_vsel30(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
        }
        #[doc = "desc VSEL31."]
        #[must_use]
        #[inline(always)]
        pub const fn vsel31(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[doc = "desc VSEL31."]
        #[inline(always)]
        pub const fn set_vsel31(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for Vssel128 {
        #[inline(always)]
        fn default() -> Vssel128 {
            Vssel128(0)
        }
    }
    impl core::fmt::Debug for Vssel128 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Vssel128")
                .field("vsel0", &self.vsel0())
                .field("vsel1", &self.vsel1())
                .field("vsel2", &self.vsel2())
                .field("vsel3", &self.vsel3())
                .field("vsel4", &self.vsel4())
                .field("vsel5", &self.vsel5())
                .field("vsel6", &self.vsel6())
                .field("vsel7", &self.vsel7())
                .field("vsel8", &self.vsel8())
                .field("vsel9", &self.vsel9())
                .field("vsel10", &self.vsel10())
                .field("vsel11", &self.vsel11())
                .field("vsel12", &self.vsel12())
                .field("vsel13", &self.vsel13())
                .field("vsel14", &self.vsel14())
                .field("vsel15", &self.vsel15())
                .field("vsel16", &self.vsel16())
                .field("vsel17", &self.vsel17())
                .field("vsel18", &self.vsel18())
                .field("vsel19", &self.vsel19())
                .field("vsel20", &self.vsel20())
                .field("vsel21", &self.vsel21())
                .field("vsel22", &self.vsel22())
                .field("vsel23", &self.vsel23())
                .field("vsel24", &self.vsel24())
                .field("vsel25", &self.vsel25())
                .field("vsel26", &self.vsel26())
                .field("vsel27", &self.vsel27())
                .field("vsel28", &self.vsel28())
                .field("vsel29", &self.vsel29())
                .field("vsel30", &self.vsel30())
                .field("vsel31", &self.vsel31())
                .finish()
        }
    }
    #[doc = "desc VSSEL129."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Vssel129(pub u32);
    impl Vssel129 {
        #[doc = "desc VSEL0."]
        #[must_use]
        #[inline(always)]
        pub const fn vsel0(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "desc VSEL0."]
        #[inline(always)]
        pub const fn set_vsel0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "desc VSEL1."]
        #[must_use]
        #[inline(always)]
        pub const fn vsel1(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "desc VSEL1."]
        #[inline(always)]
        pub const fn set_vsel1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "desc VSEL2."]
        #[must_use]
        #[inline(always)]
        pub const fn vsel2(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "desc VSEL2."]
        #[inline(always)]
        pub const fn set_vsel2(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "desc VSEL3."]
        #[must_use]
        #[inline(always)]
        pub const fn vsel3(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "desc VSEL3."]
        #[inline(always)]
        pub const fn set_vsel3(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "desc VSEL4."]
        #[must_use]
        #[inline(always)]
        pub const fn vsel4(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "desc VSEL4."]
        #[inline(always)]
        pub const fn set_vsel4(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "desc VSEL5."]
        #[must_use]
        #[inline(always)]
        pub const fn vsel5(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "desc VSEL5."]
        #[inline(always)]
        pub const fn set_vsel5(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "desc VSEL6."]
        #[must_use]
        #[inline(always)]
        pub const fn vsel6(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "desc VSEL6."]
        #[inline(always)]
        pub const fn set_vsel6(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "desc VSEL7."]
        #[must_use]
        #[inline(always)]
        pub const fn vsel7(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "desc VSEL7."]
        #[inline(always)]
        pub const fn set_vsel7(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "desc VSEL8."]
        #[must_use]
        #[inline(always)]
        pub const fn vsel8(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "desc VSEL8."]
        #[inline(always)]
        pub const fn set_vsel8(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "desc VSEL9."]
        #[must_use]
        #[inline(always)]
        pub const fn vsel9(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "desc VSEL9."]
        #[inline(always)]
        pub const fn set_vsel9(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "desc VSEL10."]
        #[must_use]
        #[inline(always)]
        pub const fn vsel10(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "desc VSEL10."]
        #[inline(always)]
        pub const fn set_vsel10(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[doc = "desc VSEL11."]
        #[must_use]
        #[inline(always)]
        pub const fn vsel11(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "desc VSEL11."]
        #[inline(always)]
        pub const fn set_vsel11(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[doc = "desc VSEL12."]
        #[must_use]
        #[inline(always)]
        pub const fn vsel12(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "desc VSEL12."]
        #[inline(always)]
        pub const fn set_vsel12(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "desc VSEL13."]
        #[must_use]
        #[inline(always)]
        pub const fn vsel13(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[doc = "desc VSEL13."]
        #[inline(always)]
        pub const fn set_vsel13(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[doc = "desc VSEL14."]
        #[must_use]
        #[inline(always)]
        pub const fn vsel14(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "desc VSEL14."]
        #[inline(always)]
        pub const fn set_vsel14(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[doc = "desc VSEL15."]
        #[must_use]
        #[inline(always)]
        pub const fn vsel15(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "desc VSEL15."]
        #[inline(always)]
        pub const fn set_vsel15(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
        #[doc = "desc VSEL16."]
        #[must_use]
        #[inline(always)]
        pub const fn vsel16(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "desc VSEL16."]
        #[inline(always)]
        pub const fn set_vsel16(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "desc VSEL17."]
        #[must_use]
        #[inline(always)]
        pub const fn vsel17(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "desc VSEL17."]
        #[inline(always)]
        pub const fn set_vsel17(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[doc = "desc VSEL18."]
        #[must_use]
        #[inline(always)]
        pub const fn vsel18(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[doc = "desc VSEL18."]
        #[inline(always)]
        pub const fn set_vsel18(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
        #[doc = "desc VSEL19."]
        #[must_use]
        #[inline(always)]
        pub const fn vsel19(&self) -> bool {
            let val = (self.0 >> 19usize) & 0x01;
            val != 0
        }
        #[doc = "desc VSEL19."]
        #[inline(always)]
        pub const fn set_vsel19(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
        }
        #[doc = "desc VSEL20."]
        #[must_use]
        #[inline(always)]
        pub const fn vsel20(&self) -> bool {
            let val = (self.0 >> 20usize) & 0x01;
            val != 0
        }
        #[doc = "desc VSEL20."]
        #[inline(always)]
        pub const fn set_vsel20(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
        }
        #[doc = "desc VSEL21."]
        #[must_use]
        #[inline(always)]
        pub const fn vsel21(&self) -> bool {
            let val = (self.0 >> 21usize) & 0x01;
            val != 0
        }
        #[doc = "desc VSEL21."]
        #[inline(always)]
        pub const fn set_vsel21(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
        }
        #[doc = "desc VSEL22."]
        #[must_use]
        #[inline(always)]
        pub const fn vsel22(&self) -> bool {
            let val = (self.0 >> 22usize) & 0x01;
            val != 0
        }
        #[doc = "desc VSEL22."]
        #[inline(always)]
        pub const fn set_vsel22(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
        }
        #[doc = "desc VSEL23."]
        #[must_use]
        #[inline(always)]
        pub const fn vsel23(&self) -> bool {
            let val = (self.0 >> 23usize) & 0x01;
            val != 0
        }
        #[doc = "desc VSEL23."]
        #[inline(always)]
        pub const fn set_vsel23(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
        }
        #[doc = "desc VSEL24."]
        #[must_use]
        #[inline(always)]
        pub const fn vsel24(&self) -> bool {
            let val = (self.0 >> 24usize) & 0x01;
            val != 0
        }
        #[doc = "desc VSEL24."]
        #[inline(always)]
        pub const fn set_vsel24(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
        }
        #[doc = "desc VSEL25."]
        #[must_use]
        #[inline(always)]
        pub const fn vsel25(&self) -> bool {
            let val = (self.0 >> 25usize) & 0x01;
            val != 0
        }
        #[doc = "desc VSEL25."]
        #[inline(always)]
        pub const fn set_vsel25(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
        }
        #[doc = "desc VSEL26."]
        #[must_use]
        #[inline(always)]
        pub const fn vsel26(&self) -> bool {
            let val = (self.0 >> 26usize) & 0x01;
            val != 0
        }
        #[doc = "desc VSEL26."]
        #[inline(always)]
        pub const fn set_vsel26(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
        }
        #[doc = "desc VSEL27."]
        #[must_use]
        #[inline(always)]
        pub const fn vsel27(&self) -> bool {
            let val = (self.0 >> 27usize) & 0x01;
            val != 0
        }
        #[doc = "desc VSEL27."]
        #[inline(always)]
        pub const fn set_vsel27(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
        }
        #[doc = "desc VSEL28."]
        #[must_use]
        #[inline(always)]
        pub const fn vsel28(&self) -> bool {
            let val = (self.0 >> 28usize) & 0x01;
            val != 0
        }
        #[doc = "desc VSEL28."]
        #[inline(always)]
        pub const fn set_vsel28(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
        }
        #[doc = "desc VSEL29."]
        #[must_use]
        #[inline(always)]
        pub const fn vsel29(&self) -> bool {
            let val = (self.0 >> 29usize) & 0x01;
            val != 0
        }
        #[doc = "desc VSEL29."]
        #[inline(always)]
        pub const fn set_vsel29(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
        }
        #[doc = "desc VSEL30."]
        #[must_use]
        #[inline(always)]
        pub const fn vsel30(&self) -> bool {
            let val = (self.0 >> 30usize) & 0x01;
            val != 0
        }
        #[doc = "desc VSEL30."]
        #[inline(always)]
        pub const fn set_vsel30(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
        }
        #[doc = "desc VSEL31."]
        #[must_use]
        #[inline(always)]
        pub const fn vsel31(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[doc = "desc VSEL31."]
        #[inline(always)]
        pub const fn set_vsel31(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for Vssel129 {
        #[inline(always)]
        fn default() -> Vssel129 {
            Vssel129(0)
        }
    }
    impl core::fmt::Debug for Vssel129 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Vssel129")
                .field("vsel0", &self.vsel0())
                .field("vsel1", &self.vsel1())
                .field("vsel2", &self.vsel2())
                .field("vsel3", &self.vsel3())
                .field("vsel4", &self.vsel4())
                .field("vsel5", &self.vsel5())
                .field("vsel6", &self.vsel6())
                .field("vsel7", &self.vsel7())
                .field("vsel8", &self.vsel8())
                .field("vsel9", &self.vsel9())
                .field("vsel10", &self.vsel10())
                .field("vsel11", &self.vsel11())
                .field("vsel12", &self.vsel12())
                .field("vsel13", &self.vsel13())
                .field("vsel14", &self.vsel14())
                .field("vsel15", &self.vsel15())
                .field("vsel16", &self.vsel16())
                .field("vsel17", &self.vsel17())
                .field("vsel18", &self.vsel18())
                .field("vsel19", &self.vsel19())
                .field("vsel20", &self.vsel20())
                .field("vsel21", &self.vsel21())
                .field("vsel22", &self.vsel22())
                .field("vsel23", &self.vsel23())
                .field("vsel24", &self.vsel24())
                .field("vsel25", &self.vsel25())
                .field("vsel26", &self.vsel26())
                .field("vsel27", &self.vsel27())
                .field("vsel28", &self.vsel28())
                .field("vsel29", &self.vsel29())
                .field("vsel30", &self.vsel30())
                .field("vsel31", &self.vsel31())
                .finish()
        }
    }
    #[doc = "desc VSSEL130."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Vssel130(pub u32);
    impl Vssel130 {
        #[doc = "desc VSEL0."]
        #[must_use]
        #[inline(always)]
        pub const fn vsel0(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "desc VSEL0."]
        #[inline(always)]
        pub const fn set_vsel0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "desc VSEL1."]
        #[must_use]
        #[inline(always)]
        pub const fn vsel1(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "desc VSEL1."]
        #[inline(always)]
        pub const fn set_vsel1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "desc VSEL2."]
        #[must_use]
        #[inline(always)]
        pub const fn vsel2(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "desc VSEL2."]
        #[inline(always)]
        pub const fn set_vsel2(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "desc VSEL3."]
        #[must_use]
        #[inline(always)]
        pub const fn vsel3(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "desc VSEL3."]
        #[inline(always)]
        pub const fn set_vsel3(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "desc VSEL4."]
        #[must_use]
        #[inline(always)]
        pub const fn vsel4(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "desc VSEL4."]
        #[inline(always)]
        pub const fn set_vsel4(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "desc VSEL5."]
        #[must_use]
        #[inline(always)]
        pub const fn vsel5(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "desc VSEL5."]
        #[inline(always)]
        pub const fn set_vsel5(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "desc VSEL6."]
        #[must_use]
        #[inline(always)]
        pub const fn vsel6(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "desc VSEL6."]
        #[inline(always)]
        pub const fn set_vsel6(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "desc VSEL7."]
        #[must_use]
        #[inline(always)]
        pub const fn vsel7(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "desc VSEL7."]
        #[inline(always)]
        pub const fn set_vsel7(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "desc VSEL8."]
        #[must_use]
        #[inline(always)]
        pub const fn vsel8(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "desc VSEL8."]
        #[inline(always)]
        pub const fn set_vsel8(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "desc VSEL9."]
        #[must_use]
        #[inline(always)]
        pub const fn vsel9(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "desc VSEL9."]
        #[inline(always)]
        pub const fn set_vsel9(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "desc VSEL10."]
        #[must_use]
        #[inline(always)]
        pub const fn vsel10(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "desc VSEL10."]
        #[inline(always)]
        pub const fn set_vsel10(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[doc = "desc VSEL11."]
        #[must_use]
        #[inline(always)]
        pub const fn vsel11(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "desc VSEL11."]
        #[inline(always)]
        pub const fn set_vsel11(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[doc = "desc VSEL12."]
        #[must_use]
        #[inline(always)]
        pub const fn vsel12(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "desc VSEL12."]
        #[inline(always)]
        pub const fn set_vsel12(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "desc VSEL13."]
        #[must_use]
        #[inline(always)]
        pub const fn vsel13(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[doc = "desc VSEL13."]
        #[inline(always)]
        pub const fn set_vsel13(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[doc = "desc VSEL14."]
        #[must_use]
        #[inline(always)]
        pub const fn vsel14(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "desc VSEL14."]
        #[inline(always)]
        pub const fn set_vsel14(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[doc = "desc VSEL15."]
        #[must_use]
        #[inline(always)]
        pub const fn vsel15(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "desc VSEL15."]
        #[inline(always)]
        pub const fn set_vsel15(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
        #[doc = "desc VSEL16."]
        #[must_use]
        #[inline(always)]
        pub const fn vsel16(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "desc VSEL16."]
        #[inline(always)]
        pub const fn set_vsel16(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "desc VSEL17."]
        #[must_use]
        #[inline(always)]
        pub const fn vsel17(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "desc VSEL17."]
        #[inline(always)]
        pub const fn set_vsel17(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[doc = "desc VSEL18."]
        #[must_use]
        #[inline(always)]
        pub const fn vsel18(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[doc = "desc VSEL18."]
        #[inline(always)]
        pub const fn set_vsel18(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
        #[doc = "desc VSEL19."]
        #[must_use]
        #[inline(always)]
        pub const fn vsel19(&self) -> bool {
            let val = (self.0 >> 19usize) & 0x01;
            val != 0
        }
        #[doc = "desc VSEL19."]
        #[inline(always)]
        pub const fn set_vsel19(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
        }
        #[doc = "desc VSEL20."]
        #[must_use]
        #[inline(always)]
        pub const fn vsel20(&self) -> bool {
            let val = (self.0 >> 20usize) & 0x01;
            val != 0
        }
        #[doc = "desc VSEL20."]
        #[inline(always)]
        pub const fn set_vsel20(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
        }
        #[doc = "desc VSEL21."]
        #[must_use]
        #[inline(always)]
        pub const fn vsel21(&self) -> bool {
            let val = (self.0 >> 21usize) & 0x01;
            val != 0
        }
        #[doc = "desc VSEL21."]
        #[inline(always)]
        pub const fn set_vsel21(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
        }
        #[doc = "desc VSEL22."]
        #[must_use]
        #[inline(always)]
        pub const fn vsel22(&self) -> bool {
            let val = (self.0 >> 22usize) & 0x01;
            val != 0
        }
        #[doc = "desc VSEL22."]
        #[inline(always)]
        pub const fn set_vsel22(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
        }
        #[doc = "desc VSEL23."]
        #[must_use]
        #[inline(always)]
        pub const fn vsel23(&self) -> bool {
            let val = (self.0 >> 23usize) & 0x01;
            val != 0
        }
        #[doc = "desc VSEL23."]
        #[inline(always)]
        pub const fn set_vsel23(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
        }
        #[doc = "desc VSEL24."]
        #[must_use]
        #[inline(always)]
        pub const fn vsel24(&self) -> bool {
            let val = (self.0 >> 24usize) & 0x01;
            val != 0
        }
        #[doc = "desc VSEL24."]
        #[inline(always)]
        pub const fn set_vsel24(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
        }
        #[doc = "desc VSEL25."]
        #[must_use]
        #[inline(always)]
        pub const fn vsel25(&self) -> bool {
            let val = (self.0 >> 25usize) & 0x01;
            val != 0
        }
        #[doc = "desc VSEL25."]
        #[inline(always)]
        pub const fn set_vsel25(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
        }
        #[doc = "desc VSEL26."]
        #[must_use]
        #[inline(always)]
        pub const fn vsel26(&self) -> bool {
            let val = (self.0 >> 26usize) & 0x01;
            val != 0
        }
        #[doc = "desc VSEL26."]
        #[inline(always)]
        pub const fn set_vsel26(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
        }
        #[doc = "desc VSEL27."]
        #[must_use]
        #[inline(always)]
        pub const fn vsel27(&self) -> bool {
            let val = (self.0 >> 27usize) & 0x01;
            val != 0
        }
        #[doc = "desc VSEL27."]
        #[inline(always)]
        pub const fn set_vsel27(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
        }
        #[doc = "desc VSEL28."]
        #[must_use]
        #[inline(always)]
        pub const fn vsel28(&self) -> bool {
            let val = (self.0 >> 28usize) & 0x01;
            val != 0
        }
        #[doc = "desc VSEL28."]
        #[inline(always)]
        pub const fn set_vsel28(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
        }
        #[doc = "desc VSEL29."]
        #[must_use]
        #[inline(always)]
        pub const fn vsel29(&self) -> bool {
            let val = (self.0 >> 29usize) & 0x01;
            val != 0
        }
        #[doc = "desc VSEL29."]
        #[inline(always)]
        pub const fn set_vsel29(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
        }
        #[doc = "desc VSEL30."]
        #[must_use]
        #[inline(always)]
        pub const fn vsel30(&self) -> bool {
            let val = (self.0 >> 30usize) & 0x01;
            val != 0
        }
        #[doc = "desc VSEL30."]
        #[inline(always)]
        pub const fn set_vsel30(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
        }
        #[doc = "desc VSEL31."]
        #[must_use]
        #[inline(always)]
        pub const fn vsel31(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[doc = "desc VSEL31."]
        #[inline(always)]
        pub const fn set_vsel31(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for Vssel130 {
        #[inline(always)]
        fn default() -> Vssel130 {
            Vssel130(0)
        }
    }
    impl core::fmt::Debug for Vssel130 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Vssel130")
                .field("vsel0", &self.vsel0())
                .field("vsel1", &self.vsel1())
                .field("vsel2", &self.vsel2())
                .field("vsel3", &self.vsel3())
                .field("vsel4", &self.vsel4())
                .field("vsel5", &self.vsel5())
                .field("vsel6", &self.vsel6())
                .field("vsel7", &self.vsel7())
                .field("vsel8", &self.vsel8())
                .field("vsel9", &self.vsel9())
                .field("vsel10", &self.vsel10())
                .field("vsel11", &self.vsel11())
                .field("vsel12", &self.vsel12())
                .field("vsel13", &self.vsel13())
                .field("vsel14", &self.vsel14())
                .field("vsel15", &self.vsel15())
                .field("vsel16", &self.vsel16())
                .field("vsel17", &self.vsel17())
                .field("vsel18", &self.vsel18())
                .field("vsel19", &self.vsel19())
                .field("vsel20", &self.vsel20())
                .field("vsel21", &self.vsel21())
                .field("vsel22", &self.vsel22())
                .field("vsel23", &self.vsel23())
                .field("vsel24", &self.vsel24())
                .field("vsel25", &self.vsel25())
                .field("vsel26", &self.vsel26())
                .field("vsel27", &self.vsel27())
                .field("vsel28", &self.vsel28())
                .field("vsel29", &self.vsel29())
                .field("vsel30", &self.vsel30())
                .field("vsel31", &self.vsel31())
                .finish()
        }
    }
    #[doc = "desc VSSEL131."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Vssel131(pub u32);
    impl Vssel131 {
        #[doc = "desc VSEL0."]
        #[must_use]
        #[inline(always)]
        pub const fn vsel0(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "desc VSEL0."]
        #[inline(always)]
        pub const fn set_vsel0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "desc VSEL1."]
        #[must_use]
        #[inline(always)]
        pub const fn vsel1(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "desc VSEL1."]
        #[inline(always)]
        pub const fn set_vsel1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "desc VSEL2."]
        #[must_use]
        #[inline(always)]
        pub const fn vsel2(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "desc VSEL2."]
        #[inline(always)]
        pub const fn set_vsel2(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "desc VSEL3."]
        #[must_use]
        #[inline(always)]
        pub const fn vsel3(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "desc VSEL3."]
        #[inline(always)]
        pub const fn set_vsel3(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "desc VSEL4."]
        #[must_use]
        #[inline(always)]
        pub const fn vsel4(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "desc VSEL4."]
        #[inline(always)]
        pub const fn set_vsel4(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "desc VSEL5."]
        #[must_use]
        #[inline(always)]
        pub const fn vsel5(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "desc VSEL5."]
        #[inline(always)]
        pub const fn set_vsel5(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "desc VSEL6."]
        #[must_use]
        #[inline(always)]
        pub const fn vsel6(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "desc VSEL6."]
        #[inline(always)]
        pub const fn set_vsel6(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "desc VSEL7."]
        #[must_use]
        #[inline(always)]
        pub const fn vsel7(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "desc VSEL7."]
        #[inline(always)]
        pub const fn set_vsel7(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "desc VSEL8."]
        #[must_use]
        #[inline(always)]
        pub const fn vsel8(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "desc VSEL8."]
        #[inline(always)]
        pub const fn set_vsel8(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "desc VSEL9."]
        #[must_use]
        #[inline(always)]
        pub const fn vsel9(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "desc VSEL9."]
        #[inline(always)]
        pub const fn set_vsel9(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "desc VSEL10."]
        #[must_use]
        #[inline(always)]
        pub const fn vsel10(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "desc VSEL10."]
        #[inline(always)]
        pub const fn set_vsel10(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[doc = "desc VSEL11."]
        #[must_use]
        #[inline(always)]
        pub const fn vsel11(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "desc VSEL11."]
        #[inline(always)]
        pub const fn set_vsel11(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[doc = "desc VSEL12."]
        #[must_use]
        #[inline(always)]
        pub const fn vsel12(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "desc VSEL12."]
        #[inline(always)]
        pub const fn set_vsel12(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "desc VSEL13."]
        #[must_use]
        #[inline(always)]
        pub const fn vsel13(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[doc = "desc VSEL13."]
        #[inline(always)]
        pub const fn set_vsel13(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[doc = "desc VSEL14."]
        #[must_use]
        #[inline(always)]
        pub const fn vsel14(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "desc VSEL14."]
        #[inline(always)]
        pub const fn set_vsel14(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[doc = "desc VSEL15."]
        #[must_use]
        #[inline(always)]
        pub const fn vsel15(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "desc VSEL15."]
        #[inline(always)]
        pub const fn set_vsel15(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
        #[doc = "desc VSEL16."]
        #[must_use]
        #[inline(always)]
        pub const fn vsel16(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "desc VSEL16."]
        #[inline(always)]
        pub const fn set_vsel16(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "desc VSEL17."]
        #[must_use]
        #[inline(always)]
        pub const fn vsel17(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "desc VSEL17."]
        #[inline(always)]
        pub const fn set_vsel17(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[doc = "desc VSEL18."]
        #[must_use]
        #[inline(always)]
        pub const fn vsel18(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[doc = "desc VSEL18."]
        #[inline(always)]
        pub const fn set_vsel18(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
        #[doc = "desc VSEL19."]
        #[must_use]
        #[inline(always)]
        pub const fn vsel19(&self) -> bool {
            let val = (self.0 >> 19usize) & 0x01;
            val != 0
        }
        #[doc = "desc VSEL19."]
        #[inline(always)]
        pub const fn set_vsel19(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
        }
        #[doc = "desc VSEL20."]
        #[must_use]
        #[inline(always)]
        pub const fn vsel20(&self) -> bool {
            let val = (self.0 >> 20usize) & 0x01;
            val != 0
        }
        #[doc = "desc VSEL20."]
        #[inline(always)]
        pub const fn set_vsel20(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
        }
        #[doc = "desc VSEL21."]
        #[must_use]
        #[inline(always)]
        pub const fn vsel21(&self) -> bool {
            let val = (self.0 >> 21usize) & 0x01;
            val != 0
        }
        #[doc = "desc VSEL21."]
        #[inline(always)]
        pub const fn set_vsel21(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
        }
        #[doc = "desc VSEL22."]
        #[must_use]
        #[inline(always)]
        pub const fn vsel22(&self) -> bool {
            let val = (self.0 >> 22usize) & 0x01;
            val != 0
        }
        #[doc = "desc VSEL22."]
        #[inline(always)]
        pub const fn set_vsel22(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
        }
        #[doc = "desc VSEL23."]
        #[must_use]
        #[inline(always)]
        pub const fn vsel23(&self) -> bool {
            let val = (self.0 >> 23usize) & 0x01;
            val != 0
        }
        #[doc = "desc VSEL23."]
        #[inline(always)]
        pub const fn set_vsel23(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
        }
        #[doc = "desc VSEL24."]
        #[must_use]
        #[inline(always)]
        pub const fn vsel24(&self) -> bool {
            let val = (self.0 >> 24usize) & 0x01;
            val != 0
        }
        #[doc = "desc VSEL24."]
        #[inline(always)]
        pub const fn set_vsel24(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
        }
        #[doc = "desc VSEL25."]
        #[must_use]
        #[inline(always)]
        pub const fn vsel25(&self) -> bool {
            let val = (self.0 >> 25usize) & 0x01;
            val != 0
        }
        #[doc = "desc VSEL25."]
        #[inline(always)]
        pub const fn set_vsel25(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
        }
        #[doc = "desc VSEL26."]
        #[must_use]
        #[inline(always)]
        pub const fn vsel26(&self) -> bool {
            let val = (self.0 >> 26usize) & 0x01;
            val != 0
        }
        #[doc = "desc VSEL26."]
        #[inline(always)]
        pub const fn set_vsel26(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
        }
        #[doc = "desc VSEL27."]
        #[must_use]
        #[inline(always)]
        pub const fn vsel27(&self) -> bool {
            let val = (self.0 >> 27usize) & 0x01;
            val != 0
        }
        #[doc = "desc VSEL27."]
        #[inline(always)]
        pub const fn set_vsel27(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
        }
        #[doc = "desc VSEL28."]
        #[must_use]
        #[inline(always)]
        pub const fn vsel28(&self) -> bool {
            let val = (self.0 >> 28usize) & 0x01;
            val != 0
        }
        #[doc = "desc VSEL28."]
        #[inline(always)]
        pub const fn set_vsel28(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
        }
        #[doc = "desc VSEL29."]
        #[must_use]
        #[inline(always)]
        pub const fn vsel29(&self) -> bool {
            let val = (self.0 >> 29usize) & 0x01;
            val != 0
        }
        #[doc = "desc VSEL29."]
        #[inline(always)]
        pub const fn set_vsel29(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
        }
        #[doc = "desc VSEL30."]
        #[must_use]
        #[inline(always)]
        pub const fn vsel30(&self) -> bool {
            let val = (self.0 >> 30usize) & 0x01;
            val != 0
        }
        #[doc = "desc VSEL30."]
        #[inline(always)]
        pub const fn set_vsel30(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
        }
        #[doc = "desc VSEL31."]
        #[must_use]
        #[inline(always)]
        pub const fn vsel31(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[doc = "desc VSEL31."]
        #[inline(always)]
        pub const fn set_vsel31(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for Vssel131 {
        #[inline(always)]
        fn default() -> Vssel131 {
            Vssel131(0)
        }
    }
    impl core::fmt::Debug for Vssel131 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Vssel131")
                .field("vsel0", &self.vsel0())
                .field("vsel1", &self.vsel1())
                .field("vsel2", &self.vsel2())
                .field("vsel3", &self.vsel3())
                .field("vsel4", &self.vsel4())
                .field("vsel5", &self.vsel5())
                .field("vsel6", &self.vsel6())
                .field("vsel7", &self.vsel7())
                .field("vsel8", &self.vsel8())
                .field("vsel9", &self.vsel9())
                .field("vsel10", &self.vsel10())
                .field("vsel11", &self.vsel11())
                .field("vsel12", &self.vsel12())
                .field("vsel13", &self.vsel13())
                .field("vsel14", &self.vsel14())
                .field("vsel15", &self.vsel15())
                .field("vsel16", &self.vsel16())
                .field("vsel17", &self.vsel17())
                .field("vsel18", &self.vsel18())
                .field("vsel19", &self.vsel19())
                .field("vsel20", &self.vsel20())
                .field("vsel21", &self.vsel21())
                .field("vsel22", &self.vsel22())
                .field("vsel23", &self.vsel23())
                .field("vsel24", &self.vsel24())
                .field("vsel25", &self.vsel25())
                .field("vsel26", &self.vsel26())
                .field("vsel27", &self.vsel27())
                .field("vsel28", &self.vsel28())
                .field("vsel29", &self.vsel29())
                .field("vsel30", &self.vsel30())
                .field("vsel31", &self.vsel31())
                .finish()
        }
    }
    #[doc = "desc VSSEL132."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Vssel132(pub u32);
    impl Vssel132 {
        #[doc = "desc VSEL0."]
        #[must_use]
        #[inline(always)]
        pub const fn vsel0(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "desc VSEL0."]
        #[inline(always)]
        pub const fn set_vsel0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "desc VSEL1."]
        #[must_use]
        #[inline(always)]
        pub const fn vsel1(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "desc VSEL1."]
        #[inline(always)]
        pub const fn set_vsel1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "desc VSEL2."]
        #[must_use]
        #[inline(always)]
        pub const fn vsel2(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "desc VSEL2."]
        #[inline(always)]
        pub const fn set_vsel2(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "desc VSEL3."]
        #[must_use]
        #[inline(always)]
        pub const fn vsel3(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "desc VSEL3."]
        #[inline(always)]
        pub const fn set_vsel3(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "desc VSEL4."]
        #[must_use]
        #[inline(always)]
        pub const fn vsel4(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "desc VSEL4."]
        #[inline(always)]
        pub const fn set_vsel4(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "desc VSEL5."]
        #[must_use]
        #[inline(always)]
        pub const fn vsel5(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "desc VSEL5."]
        #[inline(always)]
        pub const fn set_vsel5(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "desc VSEL6."]
        #[must_use]
        #[inline(always)]
        pub const fn vsel6(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "desc VSEL6."]
        #[inline(always)]
        pub const fn set_vsel6(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "desc VSEL7."]
        #[must_use]
        #[inline(always)]
        pub const fn vsel7(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "desc VSEL7."]
        #[inline(always)]
        pub const fn set_vsel7(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "desc VSEL8."]
        #[must_use]
        #[inline(always)]
        pub const fn vsel8(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "desc VSEL8."]
        #[inline(always)]
        pub const fn set_vsel8(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "desc VSEL9."]
        #[must_use]
        #[inline(always)]
        pub const fn vsel9(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "desc VSEL9."]
        #[inline(always)]
        pub const fn set_vsel9(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "desc VSEL10."]
        #[must_use]
        #[inline(always)]
        pub const fn vsel10(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "desc VSEL10."]
        #[inline(always)]
        pub const fn set_vsel10(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[doc = "desc VSEL11."]
        #[must_use]
        #[inline(always)]
        pub const fn vsel11(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "desc VSEL11."]
        #[inline(always)]
        pub const fn set_vsel11(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[doc = "desc VSEL12."]
        #[must_use]
        #[inline(always)]
        pub const fn vsel12(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "desc VSEL12."]
        #[inline(always)]
        pub const fn set_vsel12(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "desc VSEL13."]
        #[must_use]
        #[inline(always)]
        pub const fn vsel13(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[doc = "desc VSEL13."]
        #[inline(always)]
        pub const fn set_vsel13(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[doc = "desc VSEL14."]
        #[must_use]
        #[inline(always)]
        pub const fn vsel14(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "desc VSEL14."]
        #[inline(always)]
        pub const fn set_vsel14(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[doc = "desc VSEL15."]
        #[must_use]
        #[inline(always)]
        pub const fn vsel15(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "desc VSEL15."]
        #[inline(always)]
        pub const fn set_vsel15(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
        #[doc = "desc VSEL16."]
        #[must_use]
        #[inline(always)]
        pub const fn vsel16(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "desc VSEL16."]
        #[inline(always)]
        pub const fn set_vsel16(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "desc VSEL17."]
        #[must_use]
        #[inline(always)]
        pub const fn vsel17(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "desc VSEL17."]
        #[inline(always)]
        pub const fn set_vsel17(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[doc = "desc VSEL18."]
        #[must_use]
        #[inline(always)]
        pub const fn vsel18(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[doc = "desc VSEL18."]
        #[inline(always)]
        pub const fn set_vsel18(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
        #[doc = "desc VSEL19."]
        #[must_use]
        #[inline(always)]
        pub const fn vsel19(&self) -> bool {
            let val = (self.0 >> 19usize) & 0x01;
            val != 0
        }
        #[doc = "desc VSEL19."]
        #[inline(always)]
        pub const fn set_vsel19(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
        }
        #[doc = "desc VSEL20."]
        #[must_use]
        #[inline(always)]
        pub const fn vsel20(&self) -> bool {
            let val = (self.0 >> 20usize) & 0x01;
            val != 0
        }
        #[doc = "desc VSEL20."]
        #[inline(always)]
        pub const fn set_vsel20(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
        }
        #[doc = "desc VSEL21."]
        #[must_use]
        #[inline(always)]
        pub const fn vsel21(&self) -> bool {
            let val = (self.0 >> 21usize) & 0x01;
            val != 0
        }
        #[doc = "desc VSEL21."]
        #[inline(always)]
        pub const fn set_vsel21(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
        }
        #[doc = "desc VSEL22."]
        #[must_use]
        #[inline(always)]
        pub const fn vsel22(&self) -> bool {
            let val = (self.0 >> 22usize) & 0x01;
            val != 0
        }
        #[doc = "desc VSEL22."]
        #[inline(always)]
        pub const fn set_vsel22(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
        }
        #[doc = "desc VSEL23."]
        #[must_use]
        #[inline(always)]
        pub const fn vsel23(&self) -> bool {
            let val = (self.0 >> 23usize) & 0x01;
            val != 0
        }
        #[doc = "desc VSEL23."]
        #[inline(always)]
        pub const fn set_vsel23(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
        }
        #[doc = "desc VSEL24."]
        #[must_use]
        #[inline(always)]
        pub const fn vsel24(&self) -> bool {
            let val = (self.0 >> 24usize) & 0x01;
            val != 0
        }
        #[doc = "desc VSEL24."]
        #[inline(always)]
        pub const fn set_vsel24(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
        }
        #[doc = "desc VSEL25."]
        #[must_use]
        #[inline(always)]
        pub const fn vsel25(&self) -> bool {
            let val = (self.0 >> 25usize) & 0x01;
            val != 0
        }
        #[doc = "desc VSEL25."]
        #[inline(always)]
        pub const fn set_vsel25(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
        }
        #[doc = "desc VSEL26."]
        #[must_use]
        #[inline(always)]
        pub const fn vsel26(&self) -> bool {
            let val = (self.0 >> 26usize) & 0x01;
            val != 0
        }
        #[doc = "desc VSEL26."]
        #[inline(always)]
        pub const fn set_vsel26(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
        }
        #[doc = "desc VSEL27."]
        #[must_use]
        #[inline(always)]
        pub const fn vsel27(&self) -> bool {
            let val = (self.0 >> 27usize) & 0x01;
            val != 0
        }
        #[doc = "desc VSEL27."]
        #[inline(always)]
        pub const fn set_vsel27(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
        }
        #[doc = "desc VSEL28."]
        #[must_use]
        #[inline(always)]
        pub const fn vsel28(&self) -> bool {
            let val = (self.0 >> 28usize) & 0x01;
            val != 0
        }
        #[doc = "desc VSEL28."]
        #[inline(always)]
        pub const fn set_vsel28(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
        }
        #[doc = "desc VSEL29."]
        #[must_use]
        #[inline(always)]
        pub const fn vsel29(&self) -> bool {
            let val = (self.0 >> 29usize) & 0x01;
            val != 0
        }
        #[doc = "desc VSEL29."]
        #[inline(always)]
        pub const fn set_vsel29(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
        }
        #[doc = "desc VSEL30."]
        #[must_use]
        #[inline(always)]
        pub const fn vsel30(&self) -> bool {
            let val = (self.0 >> 30usize) & 0x01;
            val != 0
        }
        #[doc = "desc VSEL30."]
        #[inline(always)]
        pub const fn set_vsel30(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
        }
        #[doc = "desc VSEL31."]
        #[must_use]
        #[inline(always)]
        pub const fn vsel31(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[doc = "desc VSEL31."]
        #[inline(always)]
        pub const fn set_vsel31(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for Vssel132 {
        #[inline(always)]
        fn default() -> Vssel132 {
            Vssel132(0)
        }
    }
    impl core::fmt::Debug for Vssel132 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Vssel132")
                .field("vsel0", &self.vsel0())
                .field("vsel1", &self.vsel1())
                .field("vsel2", &self.vsel2())
                .field("vsel3", &self.vsel3())
                .field("vsel4", &self.vsel4())
                .field("vsel5", &self.vsel5())
                .field("vsel6", &self.vsel6())
                .field("vsel7", &self.vsel7())
                .field("vsel8", &self.vsel8())
                .field("vsel9", &self.vsel9())
                .field("vsel10", &self.vsel10())
                .field("vsel11", &self.vsel11())
                .field("vsel12", &self.vsel12())
                .field("vsel13", &self.vsel13())
                .field("vsel14", &self.vsel14())
                .field("vsel15", &self.vsel15())
                .field("vsel16", &self.vsel16())
                .field("vsel17", &self.vsel17())
                .field("vsel18", &self.vsel18())
                .field("vsel19", &self.vsel19())
                .field("vsel20", &self.vsel20())
                .field("vsel21", &self.vsel21())
                .field("vsel22", &self.vsel22())
                .field("vsel23", &self.vsel23())
                .field("vsel24", &self.vsel24())
                .field("vsel25", &self.vsel25())
                .field("vsel26", &self.vsel26())
                .field("vsel27", &self.vsel27())
                .field("vsel28", &self.vsel28())
                .field("vsel29", &self.vsel29())
                .field("vsel30", &self.vsel30())
                .field("vsel31", &self.vsel31())
                .finish()
        }
    }
    #[doc = "desc VSSEL133."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Vssel133(pub u32);
    impl Vssel133 {
        #[doc = "desc VSEL0."]
        #[must_use]
        #[inline(always)]
        pub const fn vsel0(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "desc VSEL0."]
        #[inline(always)]
        pub const fn set_vsel0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "desc VSEL1."]
        #[must_use]
        #[inline(always)]
        pub const fn vsel1(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "desc VSEL1."]
        #[inline(always)]
        pub const fn set_vsel1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "desc VSEL2."]
        #[must_use]
        #[inline(always)]
        pub const fn vsel2(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "desc VSEL2."]
        #[inline(always)]
        pub const fn set_vsel2(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "desc VSEL3."]
        #[must_use]
        #[inline(always)]
        pub const fn vsel3(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "desc VSEL3."]
        #[inline(always)]
        pub const fn set_vsel3(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "desc VSEL4."]
        #[must_use]
        #[inline(always)]
        pub const fn vsel4(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "desc VSEL4."]
        #[inline(always)]
        pub const fn set_vsel4(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "desc VSEL5."]
        #[must_use]
        #[inline(always)]
        pub const fn vsel5(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "desc VSEL5."]
        #[inline(always)]
        pub const fn set_vsel5(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "desc VSEL6."]
        #[must_use]
        #[inline(always)]
        pub const fn vsel6(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "desc VSEL6."]
        #[inline(always)]
        pub const fn set_vsel6(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "desc VSEL7."]
        #[must_use]
        #[inline(always)]
        pub const fn vsel7(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "desc VSEL7."]
        #[inline(always)]
        pub const fn set_vsel7(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "desc VSEL8."]
        #[must_use]
        #[inline(always)]
        pub const fn vsel8(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "desc VSEL8."]
        #[inline(always)]
        pub const fn set_vsel8(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "desc VSEL9."]
        #[must_use]
        #[inline(always)]
        pub const fn vsel9(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "desc VSEL9."]
        #[inline(always)]
        pub const fn set_vsel9(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "desc VSEL10."]
        #[must_use]
        #[inline(always)]
        pub const fn vsel10(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "desc VSEL10."]
        #[inline(always)]
        pub const fn set_vsel10(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[doc = "desc VSEL11."]
        #[must_use]
        #[inline(always)]
        pub const fn vsel11(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "desc VSEL11."]
        #[inline(always)]
        pub const fn set_vsel11(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[doc = "desc VSEL12."]
        #[must_use]
        #[inline(always)]
        pub const fn vsel12(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "desc VSEL12."]
        #[inline(always)]
        pub const fn set_vsel12(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "desc VSEL13."]
        #[must_use]
        #[inline(always)]
        pub const fn vsel13(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[doc = "desc VSEL13."]
        #[inline(always)]
        pub const fn set_vsel13(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[doc = "desc VSEL14."]
        #[must_use]
        #[inline(always)]
        pub const fn vsel14(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "desc VSEL14."]
        #[inline(always)]
        pub const fn set_vsel14(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[doc = "desc VSEL15."]
        #[must_use]
        #[inline(always)]
        pub const fn vsel15(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "desc VSEL15."]
        #[inline(always)]
        pub const fn set_vsel15(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
        #[doc = "desc VSEL16."]
        #[must_use]
        #[inline(always)]
        pub const fn vsel16(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "desc VSEL16."]
        #[inline(always)]
        pub const fn set_vsel16(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "desc VSEL17."]
        #[must_use]
        #[inline(always)]
        pub const fn vsel17(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "desc VSEL17."]
        #[inline(always)]
        pub const fn set_vsel17(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[doc = "desc VSEL18."]
        #[must_use]
        #[inline(always)]
        pub const fn vsel18(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[doc = "desc VSEL18."]
        #[inline(always)]
        pub const fn set_vsel18(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
        #[doc = "desc VSEL19."]
        #[must_use]
        #[inline(always)]
        pub const fn vsel19(&self) -> bool {
            let val = (self.0 >> 19usize) & 0x01;
            val != 0
        }
        #[doc = "desc VSEL19."]
        #[inline(always)]
        pub const fn set_vsel19(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
        }
        #[doc = "desc VSEL20."]
        #[must_use]
        #[inline(always)]
        pub const fn vsel20(&self) -> bool {
            let val = (self.0 >> 20usize) & 0x01;
            val != 0
        }
        #[doc = "desc VSEL20."]
        #[inline(always)]
        pub const fn set_vsel20(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
        }
        #[doc = "desc VSEL21."]
        #[must_use]
        #[inline(always)]
        pub const fn vsel21(&self) -> bool {
            let val = (self.0 >> 21usize) & 0x01;
            val != 0
        }
        #[doc = "desc VSEL21."]
        #[inline(always)]
        pub const fn set_vsel21(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
        }
        #[doc = "desc VSEL22."]
        #[must_use]
        #[inline(always)]
        pub const fn vsel22(&self) -> bool {
            let val = (self.0 >> 22usize) & 0x01;
            val != 0
        }
        #[doc = "desc VSEL22."]
        #[inline(always)]
        pub const fn set_vsel22(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
        }
        #[doc = "desc VSEL23."]
        #[must_use]
        #[inline(always)]
        pub const fn vsel23(&self) -> bool {
            let val = (self.0 >> 23usize) & 0x01;
            val != 0
        }
        #[doc = "desc VSEL23."]
        #[inline(always)]
        pub const fn set_vsel23(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
        }
        #[doc = "desc VSEL24."]
        #[must_use]
        #[inline(always)]
        pub const fn vsel24(&self) -> bool {
            let val = (self.0 >> 24usize) & 0x01;
            val != 0
        }
        #[doc = "desc VSEL24."]
        #[inline(always)]
        pub const fn set_vsel24(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
        }
        #[doc = "desc VSEL25."]
        #[must_use]
        #[inline(always)]
        pub const fn vsel25(&self) -> bool {
            let val = (self.0 >> 25usize) & 0x01;
            val != 0
        }
        #[doc = "desc VSEL25."]
        #[inline(always)]
        pub const fn set_vsel25(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
        }
        #[doc = "desc VSEL26."]
        #[must_use]
        #[inline(always)]
        pub const fn vsel26(&self) -> bool {
            let val = (self.0 >> 26usize) & 0x01;
            val != 0
        }
        #[doc = "desc VSEL26."]
        #[inline(always)]
        pub const fn set_vsel26(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
        }
        #[doc = "desc VSEL27."]
        #[must_use]
        #[inline(always)]
        pub const fn vsel27(&self) -> bool {
            let val = (self.0 >> 27usize) & 0x01;
            val != 0
        }
        #[doc = "desc VSEL27."]
        #[inline(always)]
        pub const fn set_vsel27(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
        }
        #[doc = "desc VSEL28."]
        #[must_use]
        #[inline(always)]
        pub const fn vsel28(&self) -> bool {
            let val = (self.0 >> 28usize) & 0x01;
            val != 0
        }
        #[doc = "desc VSEL28."]
        #[inline(always)]
        pub const fn set_vsel28(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
        }
        #[doc = "desc VSEL29."]
        #[must_use]
        #[inline(always)]
        pub const fn vsel29(&self) -> bool {
            let val = (self.0 >> 29usize) & 0x01;
            val != 0
        }
        #[doc = "desc VSEL29."]
        #[inline(always)]
        pub const fn set_vsel29(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
        }
        #[doc = "desc VSEL30."]
        #[must_use]
        #[inline(always)]
        pub const fn vsel30(&self) -> bool {
            let val = (self.0 >> 30usize) & 0x01;
            val != 0
        }
        #[doc = "desc VSEL30."]
        #[inline(always)]
        pub const fn set_vsel30(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
        }
        #[doc = "desc VSEL31."]
        #[must_use]
        #[inline(always)]
        pub const fn vsel31(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[doc = "desc VSEL31."]
        #[inline(always)]
        pub const fn set_vsel31(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for Vssel133 {
        #[inline(always)]
        fn default() -> Vssel133 {
            Vssel133(0)
        }
    }
    impl core::fmt::Debug for Vssel133 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Vssel133")
                .field("vsel0", &self.vsel0())
                .field("vsel1", &self.vsel1())
                .field("vsel2", &self.vsel2())
                .field("vsel3", &self.vsel3())
                .field("vsel4", &self.vsel4())
                .field("vsel5", &self.vsel5())
                .field("vsel6", &self.vsel6())
                .field("vsel7", &self.vsel7())
                .field("vsel8", &self.vsel8())
                .field("vsel9", &self.vsel9())
                .field("vsel10", &self.vsel10())
                .field("vsel11", &self.vsel11())
                .field("vsel12", &self.vsel12())
                .field("vsel13", &self.vsel13())
                .field("vsel14", &self.vsel14())
                .field("vsel15", &self.vsel15())
                .field("vsel16", &self.vsel16())
                .field("vsel17", &self.vsel17())
                .field("vsel18", &self.vsel18())
                .field("vsel19", &self.vsel19())
                .field("vsel20", &self.vsel20())
                .field("vsel21", &self.vsel21())
                .field("vsel22", &self.vsel22())
                .field("vsel23", &self.vsel23())
                .field("vsel24", &self.vsel24())
                .field("vsel25", &self.vsel25())
                .field("vsel26", &self.vsel26())
                .field("vsel27", &self.vsel27())
                .field("vsel28", &self.vsel28())
                .field("vsel29", &self.vsel29())
                .field("vsel30", &self.vsel30())
                .field("vsel31", &self.vsel31())
                .finish()
        }
    }
    #[doc = "desc VSSEL134."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Vssel134(pub u32);
    impl Vssel134 {
        #[doc = "desc VSEL0."]
        #[must_use]
        #[inline(always)]
        pub const fn vsel0(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "desc VSEL0."]
        #[inline(always)]
        pub const fn set_vsel0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "desc VSEL1."]
        #[must_use]
        #[inline(always)]
        pub const fn vsel1(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "desc VSEL1."]
        #[inline(always)]
        pub const fn set_vsel1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "desc VSEL2."]
        #[must_use]
        #[inline(always)]
        pub const fn vsel2(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "desc VSEL2."]
        #[inline(always)]
        pub const fn set_vsel2(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "desc VSEL3."]
        #[must_use]
        #[inline(always)]
        pub const fn vsel3(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "desc VSEL3."]
        #[inline(always)]
        pub const fn set_vsel3(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "desc VSEL4."]
        #[must_use]
        #[inline(always)]
        pub const fn vsel4(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "desc VSEL4."]
        #[inline(always)]
        pub const fn set_vsel4(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "desc VSEL5."]
        #[must_use]
        #[inline(always)]
        pub const fn vsel5(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "desc VSEL5."]
        #[inline(always)]
        pub const fn set_vsel5(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "desc VSEL6."]
        #[must_use]
        #[inline(always)]
        pub const fn vsel6(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "desc VSEL6."]
        #[inline(always)]
        pub const fn set_vsel6(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "desc VSEL7."]
        #[must_use]
        #[inline(always)]
        pub const fn vsel7(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "desc VSEL7."]
        #[inline(always)]
        pub const fn set_vsel7(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "desc VSEL8."]
        #[must_use]
        #[inline(always)]
        pub const fn vsel8(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "desc VSEL8."]
        #[inline(always)]
        pub const fn set_vsel8(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "desc VSEL9."]
        #[must_use]
        #[inline(always)]
        pub const fn vsel9(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "desc VSEL9."]
        #[inline(always)]
        pub const fn set_vsel9(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "desc VSEL10."]
        #[must_use]
        #[inline(always)]
        pub const fn vsel10(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "desc VSEL10."]
        #[inline(always)]
        pub const fn set_vsel10(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[doc = "desc VSEL11."]
        #[must_use]
        #[inline(always)]
        pub const fn vsel11(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "desc VSEL11."]
        #[inline(always)]
        pub const fn set_vsel11(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[doc = "desc VSEL12."]
        #[must_use]
        #[inline(always)]
        pub const fn vsel12(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "desc VSEL12."]
        #[inline(always)]
        pub const fn set_vsel12(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "desc VSEL13."]
        #[must_use]
        #[inline(always)]
        pub const fn vsel13(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[doc = "desc VSEL13."]
        #[inline(always)]
        pub const fn set_vsel13(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[doc = "desc VSEL14."]
        #[must_use]
        #[inline(always)]
        pub const fn vsel14(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "desc VSEL14."]
        #[inline(always)]
        pub const fn set_vsel14(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[doc = "desc VSEL15."]
        #[must_use]
        #[inline(always)]
        pub const fn vsel15(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "desc VSEL15."]
        #[inline(always)]
        pub const fn set_vsel15(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
        #[doc = "desc VSEL16."]
        #[must_use]
        #[inline(always)]
        pub const fn vsel16(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "desc VSEL16."]
        #[inline(always)]
        pub const fn set_vsel16(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "desc VSEL17."]
        #[must_use]
        #[inline(always)]
        pub const fn vsel17(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "desc VSEL17."]
        #[inline(always)]
        pub const fn set_vsel17(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[doc = "desc VSEL18."]
        #[must_use]
        #[inline(always)]
        pub const fn vsel18(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[doc = "desc VSEL18."]
        #[inline(always)]
        pub const fn set_vsel18(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
        #[doc = "desc VSEL19."]
        #[must_use]
        #[inline(always)]
        pub const fn vsel19(&self) -> bool {
            let val = (self.0 >> 19usize) & 0x01;
            val != 0
        }
        #[doc = "desc VSEL19."]
        #[inline(always)]
        pub const fn set_vsel19(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
        }
        #[doc = "desc VSEL20."]
        #[must_use]
        #[inline(always)]
        pub const fn vsel20(&self) -> bool {
            let val = (self.0 >> 20usize) & 0x01;
            val != 0
        }
        #[doc = "desc VSEL20."]
        #[inline(always)]
        pub const fn set_vsel20(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
        }
        #[doc = "desc VSEL21."]
        #[must_use]
        #[inline(always)]
        pub const fn vsel21(&self) -> bool {
            let val = (self.0 >> 21usize) & 0x01;
            val != 0
        }
        #[doc = "desc VSEL21."]
        #[inline(always)]
        pub const fn set_vsel21(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
        }
        #[doc = "desc VSEL22."]
        #[must_use]
        #[inline(always)]
        pub const fn vsel22(&self) -> bool {
            let val = (self.0 >> 22usize) & 0x01;
            val != 0
        }
        #[doc = "desc VSEL22."]
        #[inline(always)]
        pub const fn set_vsel22(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
        }
        #[doc = "desc VSEL23."]
        #[must_use]
        #[inline(always)]
        pub const fn vsel23(&self) -> bool {
            let val = (self.0 >> 23usize) & 0x01;
            val != 0
        }
        #[doc = "desc VSEL23."]
        #[inline(always)]
        pub const fn set_vsel23(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
        }
        #[doc = "desc VSEL24."]
        #[must_use]
        #[inline(always)]
        pub const fn vsel24(&self) -> bool {
            let val = (self.0 >> 24usize) & 0x01;
            val != 0
        }
        #[doc = "desc VSEL24."]
        #[inline(always)]
        pub const fn set_vsel24(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
        }
        #[doc = "desc VSEL25."]
        #[must_use]
        #[inline(always)]
        pub const fn vsel25(&self) -> bool {
            let val = (self.0 >> 25usize) & 0x01;
            val != 0
        }
        #[doc = "desc VSEL25."]
        #[inline(always)]
        pub const fn set_vsel25(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
        }
        #[doc = "desc VSEL26."]
        #[must_use]
        #[inline(always)]
        pub const fn vsel26(&self) -> bool {
            let val = (self.0 >> 26usize) & 0x01;
            val != 0
        }
        #[doc = "desc VSEL26."]
        #[inline(always)]
        pub const fn set_vsel26(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
        }
        #[doc = "desc VSEL27."]
        #[must_use]
        #[inline(always)]
        pub const fn vsel27(&self) -> bool {
            let val = (self.0 >> 27usize) & 0x01;
            val != 0
        }
        #[doc = "desc VSEL27."]
        #[inline(always)]
        pub const fn set_vsel27(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
        }
        #[doc = "desc VSEL28."]
        #[must_use]
        #[inline(always)]
        pub const fn vsel28(&self) -> bool {
            let val = (self.0 >> 28usize) & 0x01;
            val != 0
        }
        #[doc = "desc VSEL28."]
        #[inline(always)]
        pub const fn set_vsel28(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
        }
        #[doc = "desc VSEL29."]
        #[must_use]
        #[inline(always)]
        pub const fn vsel29(&self) -> bool {
            let val = (self.0 >> 29usize) & 0x01;
            val != 0
        }
        #[doc = "desc VSEL29."]
        #[inline(always)]
        pub const fn set_vsel29(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
        }
        #[doc = "desc VSEL30."]
        #[must_use]
        #[inline(always)]
        pub const fn vsel30(&self) -> bool {
            let val = (self.0 >> 30usize) & 0x01;
            val != 0
        }
        #[doc = "desc VSEL30."]
        #[inline(always)]
        pub const fn set_vsel30(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
        }
        #[doc = "desc VSEL31."]
        #[must_use]
        #[inline(always)]
        pub const fn vsel31(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[doc = "desc VSEL31."]
        #[inline(always)]
        pub const fn set_vsel31(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for Vssel134 {
        #[inline(always)]
        fn default() -> Vssel134 {
            Vssel134(0)
        }
    }
    impl core::fmt::Debug for Vssel134 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Vssel134")
                .field("vsel0", &self.vsel0())
                .field("vsel1", &self.vsel1())
                .field("vsel2", &self.vsel2())
                .field("vsel3", &self.vsel3())
                .field("vsel4", &self.vsel4())
                .field("vsel5", &self.vsel5())
                .field("vsel6", &self.vsel6())
                .field("vsel7", &self.vsel7())
                .field("vsel8", &self.vsel8())
                .field("vsel9", &self.vsel9())
                .field("vsel10", &self.vsel10())
                .field("vsel11", &self.vsel11())
                .field("vsel12", &self.vsel12())
                .field("vsel13", &self.vsel13())
                .field("vsel14", &self.vsel14())
                .field("vsel15", &self.vsel15())
                .field("vsel16", &self.vsel16())
                .field("vsel17", &self.vsel17())
                .field("vsel18", &self.vsel18())
                .field("vsel19", &self.vsel19())
                .field("vsel20", &self.vsel20())
                .field("vsel21", &self.vsel21())
                .field("vsel22", &self.vsel22())
                .field("vsel23", &self.vsel23())
                .field("vsel24", &self.vsel24())
                .field("vsel25", &self.vsel25())
                .field("vsel26", &self.vsel26())
                .field("vsel27", &self.vsel27())
                .field("vsel28", &self.vsel28())
                .field("vsel29", &self.vsel29())
                .field("vsel30", &self.vsel30())
                .field("vsel31", &self.vsel31())
                .finish()
        }
    }
    #[doc = "desc VSSEL135."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Vssel135(pub u32);
    impl Vssel135 {
        #[doc = "desc VSEL0."]
        #[must_use]
        #[inline(always)]
        pub const fn vsel0(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "desc VSEL0."]
        #[inline(always)]
        pub const fn set_vsel0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "desc VSEL1."]
        #[must_use]
        #[inline(always)]
        pub const fn vsel1(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "desc VSEL1."]
        #[inline(always)]
        pub const fn set_vsel1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "desc VSEL2."]
        #[must_use]
        #[inline(always)]
        pub const fn vsel2(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "desc VSEL2."]
        #[inline(always)]
        pub const fn set_vsel2(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "desc VSEL3."]
        #[must_use]
        #[inline(always)]
        pub const fn vsel3(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "desc VSEL3."]
        #[inline(always)]
        pub const fn set_vsel3(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "desc VSEL4."]
        #[must_use]
        #[inline(always)]
        pub const fn vsel4(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "desc VSEL4."]
        #[inline(always)]
        pub const fn set_vsel4(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "desc VSEL5."]
        #[must_use]
        #[inline(always)]
        pub const fn vsel5(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "desc VSEL5."]
        #[inline(always)]
        pub const fn set_vsel5(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "desc VSEL6."]
        #[must_use]
        #[inline(always)]
        pub const fn vsel6(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "desc VSEL6."]
        #[inline(always)]
        pub const fn set_vsel6(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "desc VSEL7."]
        #[must_use]
        #[inline(always)]
        pub const fn vsel7(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "desc VSEL7."]
        #[inline(always)]
        pub const fn set_vsel7(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "desc VSEL8."]
        #[must_use]
        #[inline(always)]
        pub const fn vsel8(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "desc VSEL8."]
        #[inline(always)]
        pub const fn set_vsel8(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "desc VSEL9."]
        #[must_use]
        #[inline(always)]
        pub const fn vsel9(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "desc VSEL9."]
        #[inline(always)]
        pub const fn set_vsel9(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "desc VSEL10."]
        #[must_use]
        #[inline(always)]
        pub const fn vsel10(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "desc VSEL10."]
        #[inline(always)]
        pub const fn set_vsel10(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[doc = "desc VSEL11."]
        #[must_use]
        #[inline(always)]
        pub const fn vsel11(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "desc VSEL11."]
        #[inline(always)]
        pub const fn set_vsel11(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[doc = "desc VSEL12."]
        #[must_use]
        #[inline(always)]
        pub const fn vsel12(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "desc VSEL12."]
        #[inline(always)]
        pub const fn set_vsel12(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "desc VSEL13."]
        #[must_use]
        #[inline(always)]
        pub const fn vsel13(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[doc = "desc VSEL13."]
        #[inline(always)]
        pub const fn set_vsel13(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[doc = "desc VSEL14."]
        #[must_use]
        #[inline(always)]
        pub const fn vsel14(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "desc VSEL14."]
        #[inline(always)]
        pub const fn set_vsel14(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[doc = "desc VSEL15."]
        #[must_use]
        #[inline(always)]
        pub const fn vsel15(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "desc VSEL15."]
        #[inline(always)]
        pub const fn set_vsel15(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
        #[doc = "desc VSEL16."]
        #[must_use]
        #[inline(always)]
        pub const fn vsel16(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "desc VSEL16."]
        #[inline(always)]
        pub const fn set_vsel16(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "desc VSEL17."]
        #[must_use]
        #[inline(always)]
        pub const fn vsel17(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "desc VSEL17."]
        #[inline(always)]
        pub const fn set_vsel17(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[doc = "desc VSEL18."]
        #[must_use]
        #[inline(always)]
        pub const fn vsel18(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[doc = "desc VSEL18."]
        #[inline(always)]
        pub const fn set_vsel18(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
        #[doc = "desc VSEL19."]
        #[must_use]
        #[inline(always)]
        pub const fn vsel19(&self) -> bool {
            let val = (self.0 >> 19usize) & 0x01;
            val != 0
        }
        #[doc = "desc VSEL19."]
        #[inline(always)]
        pub const fn set_vsel19(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
        }
        #[doc = "desc VSEL20."]
        #[must_use]
        #[inline(always)]
        pub const fn vsel20(&self) -> bool {
            let val = (self.0 >> 20usize) & 0x01;
            val != 0
        }
        #[doc = "desc VSEL20."]
        #[inline(always)]
        pub const fn set_vsel20(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
        }
        #[doc = "desc VSEL21."]
        #[must_use]
        #[inline(always)]
        pub const fn vsel21(&self) -> bool {
            let val = (self.0 >> 21usize) & 0x01;
            val != 0
        }
        #[doc = "desc VSEL21."]
        #[inline(always)]
        pub const fn set_vsel21(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
        }
        #[doc = "desc VSEL22."]
        #[must_use]
        #[inline(always)]
        pub const fn vsel22(&self) -> bool {
            let val = (self.0 >> 22usize) & 0x01;
            val != 0
        }
        #[doc = "desc VSEL22."]
        #[inline(always)]
        pub const fn set_vsel22(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
        }
        #[doc = "desc VSEL23."]
        #[must_use]
        #[inline(always)]
        pub const fn vsel23(&self) -> bool {
            let val = (self.0 >> 23usize) & 0x01;
            val != 0
        }
        #[doc = "desc VSEL23."]
        #[inline(always)]
        pub const fn set_vsel23(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
        }
        #[doc = "desc VSEL24."]
        #[must_use]
        #[inline(always)]
        pub const fn vsel24(&self) -> bool {
            let val = (self.0 >> 24usize) & 0x01;
            val != 0
        }
        #[doc = "desc VSEL24."]
        #[inline(always)]
        pub const fn set_vsel24(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
        }
        #[doc = "desc VSEL25."]
        #[must_use]
        #[inline(always)]
        pub const fn vsel25(&self) -> bool {
            let val = (self.0 >> 25usize) & 0x01;
            val != 0
        }
        #[doc = "desc VSEL25."]
        #[inline(always)]
        pub const fn set_vsel25(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
        }
        #[doc = "desc VSEL26."]
        #[must_use]
        #[inline(always)]
        pub const fn vsel26(&self) -> bool {
            let val = (self.0 >> 26usize) & 0x01;
            val != 0
        }
        #[doc = "desc VSEL26."]
        #[inline(always)]
        pub const fn set_vsel26(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
        }
        #[doc = "desc VSEL27."]
        #[must_use]
        #[inline(always)]
        pub const fn vsel27(&self) -> bool {
            let val = (self.0 >> 27usize) & 0x01;
            val != 0
        }
        #[doc = "desc VSEL27."]
        #[inline(always)]
        pub const fn set_vsel27(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
        }
        #[doc = "desc VSEL28."]
        #[must_use]
        #[inline(always)]
        pub const fn vsel28(&self) -> bool {
            let val = (self.0 >> 28usize) & 0x01;
            val != 0
        }
        #[doc = "desc VSEL28."]
        #[inline(always)]
        pub const fn set_vsel28(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
        }
        #[doc = "desc VSEL29."]
        #[must_use]
        #[inline(always)]
        pub const fn vsel29(&self) -> bool {
            let val = (self.0 >> 29usize) & 0x01;
            val != 0
        }
        #[doc = "desc VSEL29."]
        #[inline(always)]
        pub const fn set_vsel29(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
        }
        #[doc = "desc VSEL30."]
        #[must_use]
        #[inline(always)]
        pub const fn vsel30(&self) -> bool {
            let val = (self.0 >> 30usize) & 0x01;
            val != 0
        }
        #[doc = "desc VSEL30."]
        #[inline(always)]
        pub const fn set_vsel30(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
        }
        #[doc = "desc VSEL31."]
        #[must_use]
        #[inline(always)]
        pub const fn vsel31(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[doc = "desc VSEL31."]
        #[inline(always)]
        pub const fn set_vsel31(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for Vssel135 {
        #[inline(always)]
        fn default() -> Vssel135 {
            Vssel135(0)
        }
    }
    impl core::fmt::Debug for Vssel135 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Vssel135")
                .field("vsel0", &self.vsel0())
                .field("vsel1", &self.vsel1())
                .field("vsel2", &self.vsel2())
                .field("vsel3", &self.vsel3())
                .field("vsel4", &self.vsel4())
                .field("vsel5", &self.vsel5())
                .field("vsel6", &self.vsel6())
                .field("vsel7", &self.vsel7())
                .field("vsel8", &self.vsel8())
                .field("vsel9", &self.vsel9())
                .field("vsel10", &self.vsel10())
                .field("vsel11", &self.vsel11())
                .field("vsel12", &self.vsel12())
                .field("vsel13", &self.vsel13())
                .field("vsel14", &self.vsel14())
                .field("vsel15", &self.vsel15())
                .field("vsel16", &self.vsel16())
                .field("vsel17", &self.vsel17())
                .field("vsel18", &self.vsel18())
                .field("vsel19", &self.vsel19())
                .field("vsel20", &self.vsel20())
                .field("vsel21", &self.vsel21())
                .field("vsel22", &self.vsel22())
                .field("vsel23", &self.vsel23())
                .field("vsel24", &self.vsel24())
                .field("vsel25", &self.vsel25())
                .field("vsel26", &self.vsel26())
                .field("vsel27", &self.vsel27())
                .field("vsel28", &self.vsel28())
                .field("vsel29", &self.vsel29())
                .field("vsel30", &self.vsel30())
                .field("vsel31", &self.vsel31())
                .finish()
        }
    }
    #[doc = "desc VSSEL136."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Vssel136(pub u32);
    impl Vssel136 {
        #[doc = "desc VSEL0."]
        #[must_use]
        #[inline(always)]
        pub const fn vsel0(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "desc VSEL0."]
        #[inline(always)]
        pub const fn set_vsel0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "desc VSEL1."]
        #[must_use]
        #[inline(always)]
        pub const fn vsel1(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "desc VSEL1."]
        #[inline(always)]
        pub const fn set_vsel1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "desc VSEL2."]
        #[must_use]
        #[inline(always)]
        pub const fn vsel2(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "desc VSEL2."]
        #[inline(always)]
        pub const fn set_vsel2(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "desc VSEL3."]
        #[must_use]
        #[inline(always)]
        pub const fn vsel3(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "desc VSEL3."]
        #[inline(always)]
        pub const fn set_vsel3(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "desc VSEL4."]
        #[must_use]
        #[inline(always)]
        pub const fn vsel4(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "desc VSEL4."]
        #[inline(always)]
        pub const fn set_vsel4(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "desc VSEL5."]
        #[must_use]
        #[inline(always)]
        pub const fn vsel5(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "desc VSEL5."]
        #[inline(always)]
        pub const fn set_vsel5(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "desc VSEL6."]
        #[must_use]
        #[inline(always)]
        pub const fn vsel6(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "desc VSEL6."]
        #[inline(always)]
        pub const fn set_vsel6(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "desc VSEL7."]
        #[must_use]
        #[inline(always)]
        pub const fn vsel7(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "desc VSEL7."]
        #[inline(always)]
        pub const fn set_vsel7(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "desc VSEL8."]
        #[must_use]
        #[inline(always)]
        pub const fn vsel8(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "desc VSEL8."]
        #[inline(always)]
        pub const fn set_vsel8(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "desc VSEL9."]
        #[must_use]
        #[inline(always)]
        pub const fn vsel9(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "desc VSEL9."]
        #[inline(always)]
        pub const fn set_vsel9(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "desc VSEL10."]
        #[must_use]
        #[inline(always)]
        pub const fn vsel10(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "desc VSEL10."]
        #[inline(always)]
        pub const fn set_vsel10(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[doc = "desc VSEL11."]
        #[must_use]
        #[inline(always)]
        pub const fn vsel11(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "desc VSEL11."]
        #[inline(always)]
        pub const fn set_vsel11(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[doc = "desc VSEL12."]
        #[must_use]
        #[inline(always)]
        pub const fn vsel12(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "desc VSEL12."]
        #[inline(always)]
        pub const fn set_vsel12(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "desc VSEL13."]
        #[must_use]
        #[inline(always)]
        pub const fn vsel13(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[doc = "desc VSEL13."]
        #[inline(always)]
        pub const fn set_vsel13(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[doc = "desc VSEL14."]
        #[must_use]
        #[inline(always)]
        pub const fn vsel14(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "desc VSEL14."]
        #[inline(always)]
        pub const fn set_vsel14(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[doc = "desc VSEL15."]
        #[must_use]
        #[inline(always)]
        pub const fn vsel15(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "desc VSEL15."]
        #[inline(always)]
        pub const fn set_vsel15(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
        #[doc = "desc VSEL16."]
        #[must_use]
        #[inline(always)]
        pub const fn vsel16(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "desc VSEL16."]
        #[inline(always)]
        pub const fn set_vsel16(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "desc VSEL17."]
        #[must_use]
        #[inline(always)]
        pub const fn vsel17(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "desc VSEL17."]
        #[inline(always)]
        pub const fn set_vsel17(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[doc = "desc VSEL18."]
        #[must_use]
        #[inline(always)]
        pub const fn vsel18(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[doc = "desc VSEL18."]
        #[inline(always)]
        pub const fn set_vsel18(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
        #[doc = "desc VSEL19."]
        #[must_use]
        #[inline(always)]
        pub const fn vsel19(&self) -> bool {
            let val = (self.0 >> 19usize) & 0x01;
            val != 0
        }
        #[doc = "desc VSEL19."]
        #[inline(always)]
        pub const fn set_vsel19(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
        }
        #[doc = "desc VSEL20."]
        #[must_use]
        #[inline(always)]
        pub const fn vsel20(&self) -> bool {
            let val = (self.0 >> 20usize) & 0x01;
            val != 0
        }
        #[doc = "desc VSEL20."]
        #[inline(always)]
        pub const fn set_vsel20(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
        }
        #[doc = "desc VSEL21."]
        #[must_use]
        #[inline(always)]
        pub const fn vsel21(&self) -> bool {
            let val = (self.0 >> 21usize) & 0x01;
            val != 0
        }
        #[doc = "desc VSEL21."]
        #[inline(always)]
        pub const fn set_vsel21(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
        }
        #[doc = "desc VSEL22."]
        #[must_use]
        #[inline(always)]
        pub const fn vsel22(&self) -> bool {
            let val = (self.0 >> 22usize) & 0x01;
            val != 0
        }
        #[doc = "desc VSEL22."]
        #[inline(always)]
        pub const fn set_vsel22(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
        }
        #[doc = "desc VSEL23."]
        #[must_use]
        #[inline(always)]
        pub const fn vsel23(&self) -> bool {
            let val = (self.0 >> 23usize) & 0x01;
            val != 0
        }
        #[doc = "desc VSEL23."]
        #[inline(always)]
        pub const fn set_vsel23(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
        }
        #[doc = "desc VSEL24."]
        #[must_use]
        #[inline(always)]
        pub const fn vsel24(&self) -> bool {
            let val = (self.0 >> 24usize) & 0x01;
            val != 0
        }
        #[doc = "desc VSEL24."]
        #[inline(always)]
        pub const fn set_vsel24(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
        }
        #[doc = "desc VSEL25."]
        #[must_use]
        #[inline(always)]
        pub const fn vsel25(&self) -> bool {
            let val = (self.0 >> 25usize) & 0x01;
            val != 0
        }
        #[doc = "desc VSEL25."]
        #[inline(always)]
        pub const fn set_vsel25(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
        }
        #[doc = "desc VSEL26."]
        #[must_use]
        #[inline(always)]
        pub const fn vsel26(&self) -> bool {
            let val = (self.0 >> 26usize) & 0x01;
            val != 0
        }
        #[doc = "desc VSEL26."]
        #[inline(always)]
        pub const fn set_vsel26(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
        }
        #[doc = "desc VSEL27."]
        #[must_use]
        #[inline(always)]
        pub const fn vsel27(&self) -> bool {
            let val = (self.0 >> 27usize) & 0x01;
            val != 0
        }
        #[doc = "desc VSEL27."]
        #[inline(always)]
        pub const fn set_vsel27(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
        }
        #[doc = "desc VSEL28."]
        #[must_use]
        #[inline(always)]
        pub const fn vsel28(&self) -> bool {
            let val = (self.0 >> 28usize) & 0x01;
            val != 0
        }
        #[doc = "desc VSEL28."]
        #[inline(always)]
        pub const fn set_vsel28(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
        }
        #[doc = "desc VSEL29."]
        #[must_use]
        #[inline(always)]
        pub const fn vsel29(&self) -> bool {
            let val = (self.0 >> 29usize) & 0x01;
            val != 0
        }
        #[doc = "desc VSEL29."]
        #[inline(always)]
        pub const fn set_vsel29(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
        }
        #[doc = "desc VSEL30."]
        #[must_use]
        #[inline(always)]
        pub const fn vsel30(&self) -> bool {
            let val = (self.0 >> 30usize) & 0x01;
            val != 0
        }
        #[doc = "desc VSEL30."]
        #[inline(always)]
        pub const fn set_vsel30(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
        }
        #[doc = "desc VSEL31."]
        #[must_use]
        #[inline(always)]
        pub const fn vsel31(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[doc = "desc VSEL31."]
        #[inline(always)]
        pub const fn set_vsel31(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for Vssel136 {
        #[inline(always)]
        fn default() -> Vssel136 {
            Vssel136(0)
        }
    }
    impl core::fmt::Debug for Vssel136 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Vssel136")
                .field("vsel0", &self.vsel0())
                .field("vsel1", &self.vsel1())
                .field("vsel2", &self.vsel2())
                .field("vsel3", &self.vsel3())
                .field("vsel4", &self.vsel4())
                .field("vsel5", &self.vsel5())
                .field("vsel6", &self.vsel6())
                .field("vsel7", &self.vsel7())
                .field("vsel8", &self.vsel8())
                .field("vsel9", &self.vsel9())
                .field("vsel10", &self.vsel10())
                .field("vsel11", &self.vsel11())
                .field("vsel12", &self.vsel12())
                .field("vsel13", &self.vsel13())
                .field("vsel14", &self.vsel14())
                .field("vsel15", &self.vsel15())
                .field("vsel16", &self.vsel16())
                .field("vsel17", &self.vsel17())
                .field("vsel18", &self.vsel18())
                .field("vsel19", &self.vsel19())
                .field("vsel20", &self.vsel20())
                .field("vsel21", &self.vsel21())
                .field("vsel22", &self.vsel22())
                .field("vsel23", &self.vsel23())
                .field("vsel24", &self.vsel24())
                .field("vsel25", &self.vsel25())
                .field("vsel26", &self.vsel26())
                .field("vsel27", &self.vsel27())
                .field("vsel28", &self.vsel28())
                .field("vsel29", &self.vsel29())
                .field("vsel30", &self.vsel30())
                .field("vsel31", &self.vsel31())
                .finish()
        }
    }
    #[doc = "desc VSSEL137."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Vssel137(pub u32);
    impl Vssel137 {
        #[doc = "desc VSEL0."]
        #[must_use]
        #[inline(always)]
        pub const fn vsel0(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "desc VSEL0."]
        #[inline(always)]
        pub const fn set_vsel0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "desc VSEL1."]
        #[must_use]
        #[inline(always)]
        pub const fn vsel1(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "desc VSEL1."]
        #[inline(always)]
        pub const fn set_vsel1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "desc VSEL2."]
        #[must_use]
        #[inline(always)]
        pub const fn vsel2(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "desc VSEL2."]
        #[inline(always)]
        pub const fn set_vsel2(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "desc VSEL3."]
        #[must_use]
        #[inline(always)]
        pub const fn vsel3(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "desc VSEL3."]
        #[inline(always)]
        pub const fn set_vsel3(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "desc VSEL4."]
        #[must_use]
        #[inline(always)]
        pub const fn vsel4(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "desc VSEL4."]
        #[inline(always)]
        pub const fn set_vsel4(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "desc VSEL5."]
        #[must_use]
        #[inline(always)]
        pub const fn vsel5(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "desc VSEL5."]
        #[inline(always)]
        pub const fn set_vsel5(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "desc VSEL6."]
        #[must_use]
        #[inline(always)]
        pub const fn vsel6(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "desc VSEL6."]
        #[inline(always)]
        pub const fn set_vsel6(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "desc VSEL7."]
        #[must_use]
        #[inline(always)]
        pub const fn vsel7(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "desc VSEL7."]
        #[inline(always)]
        pub const fn set_vsel7(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "desc VSEL8."]
        #[must_use]
        #[inline(always)]
        pub const fn vsel8(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "desc VSEL8."]
        #[inline(always)]
        pub const fn set_vsel8(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "desc VSEL9."]
        #[must_use]
        #[inline(always)]
        pub const fn vsel9(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "desc VSEL9."]
        #[inline(always)]
        pub const fn set_vsel9(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "desc VSEL10."]
        #[must_use]
        #[inline(always)]
        pub const fn vsel10(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "desc VSEL10."]
        #[inline(always)]
        pub const fn set_vsel10(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[doc = "desc VSEL11."]
        #[must_use]
        #[inline(always)]
        pub const fn vsel11(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "desc VSEL11."]
        #[inline(always)]
        pub const fn set_vsel11(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[doc = "desc VSEL12."]
        #[must_use]
        #[inline(always)]
        pub const fn vsel12(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "desc VSEL12."]
        #[inline(always)]
        pub const fn set_vsel12(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "desc VSEL13."]
        #[must_use]
        #[inline(always)]
        pub const fn vsel13(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[doc = "desc VSEL13."]
        #[inline(always)]
        pub const fn set_vsel13(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[doc = "desc VSEL14."]
        #[must_use]
        #[inline(always)]
        pub const fn vsel14(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "desc VSEL14."]
        #[inline(always)]
        pub const fn set_vsel14(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[doc = "desc VSEL15."]
        #[must_use]
        #[inline(always)]
        pub const fn vsel15(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "desc VSEL15."]
        #[inline(always)]
        pub const fn set_vsel15(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
        #[doc = "desc VSEL16."]
        #[must_use]
        #[inline(always)]
        pub const fn vsel16(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "desc VSEL16."]
        #[inline(always)]
        pub const fn set_vsel16(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "desc VSEL17."]
        #[must_use]
        #[inline(always)]
        pub const fn vsel17(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "desc VSEL17."]
        #[inline(always)]
        pub const fn set_vsel17(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[doc = "desc VSEL18."]
        #[must_use]
        #[inline(always)]
        pub const fn vsel18(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[doc = "desc VSEL18."]
        #[inline(always)]
        pub const fn set_vsel18(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
        #[doc = "desc VSEL19."]
        #[must_use]
        #[inline(always)]
        pub const fn vsel19(&self) -> bool {
            let val = (self.0 >> 19usize) & 0x01;
            val != 0
        }
        #[doc = "desc VSEL19."]
        #[inline(always)]
        pub const fn set_vsel19(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
        }
        #[doc = "desc VSEL20."]
        #[must_use]
        #[inline(always)]
        pub const fn vsel20(&self) -> bool {
            let val = (self.0 >> 20usize) & 0x01;
            val != 0
        }
        #[doc = "desc VSEL20."]
        #[inline(always)]
        pub const fn set_vsel20(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
        }
        #[doc = "desc VSEL21."]
        #[must_use]
        #[inline(always)]
        pub const fn vsel21(&self) -> bool {
            let val = (self.0 >> 21usize) & 0x01;
            val != 0
        }
        #[doc = "desc VSEL21."]
        #[inline(always)]
        pub const fn set_vsel21(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
        }
        #[doc = "desc VSEL22."]
        #[must_use]
        #[inline(always)]
        pub const fn vsel22(&self) -> bool {
            let val = (self.0 >> 22usize) & 0x01;
            val != 0
        }
        #[doc = "desc VSEL22."]
        #[inline(always)]
        pub const fn set_vsel22(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
        }
        #[doc = "desc VSEL23."]
        #[must_use]
        #[inline(always)]
        pub const fn vsel23(&self) -> bool {
            let val = (self.0 >> 23usize) & 0x01;
            val != 0
        }
        #[doc = "desc VSEL23."]
        #[inline(always)]
        pub const fn set_vsel23(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
        }
        #[doc = "desc VSEL24."]
        #[must_use]
        #[inline(always)]
        pub const fn vsel24(&self) -> bool {
            let val = (self.0 >> 24usize) & 0x01;
            val != 0
        }
        #[doc = "desc VSEL24."]
        #[inline(always)]
        pub const fn set_vsel24(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
        }
        #[doc = "desc VSEL25."]
        #[must_use]
        #[inline(always)]
        pub const fn vsel25(&self) -> bool {
            let val = (self.0 >> 25usize) & 0x01;
            val != 0
        }
        #[doc = "desc VSEL25."]
        #[inline(always)]
        pub const fn set_vsel25(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
        }
        #[doc = "desc VSEL26."]
        #[must_use]
        #[inline(always)]
        pub const fn vsel26(&self) -> bool {
            let val = (self.0 >> 26usize) & 0x01;
            val != 0
        }
        #[doc = "desc VSEL26."]
        #[inline(always)]
        pub const fn set_vsel26(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
        }
        #[doc = "desc VSEL27."]
        #[must_use]
        #[inline(always)]
        pub const fn vsel27(&self) -> bool {
            let val = (self.0 >> 27usize) & 0x01;
            val != 0
        }
        #[doc = "desc VSEL27."]
        #[inline(always)]
        pub const fn set_vsel27(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
        }
        #[doc = "desc VSEL28."]
        #[must_use]
        #[inline(always)]
        pub const fn vsel28(&self) -> bool {
            let val = (self.0 >> 28usize) & 0x01;
            val != 0
        }
        #[doc = "desc VSEL28."]
        #[inline(always)]
        pub const fn set_vsel28(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
        }
        #[doc = "desc VSEL29."]
        #[must_use]
        #[inline(always)]
        pub const fn vsel29(&self) -> bool {
            let val = (self.0 >> 29usize) & 0x01;
            val != 0
        }
        #[doc = "desc VSEL29."]
        #[inline(always)]
        pub const fn set_vsel29(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
        }
        #[doc = "desc VSEL30."]
        #[must_use]
        #[inline(always)]
        pub const fn vsel30(&self) -> bool {
            let val = (self.0 >> 30usize) & 0x01;
            val != 0
        }
        #[doc = "desc VSEL30."]
        #[inline(always)]
        pub const fn set_vsel30(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
        }
        #[doc = "desc VSEL31."]
        #[must_use]
        #[inline(always)]
        pub const fn vsel31(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[doc = "desc VSEL31."]
        #[inline(always)]
        pub const fn set_vsel31(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for Vssel137 {
        #[inline(always)]
        fn default() -> Vssel137 {
            Vssel137(0)
        }
    }
    impl core::fmt::Debug for Vssel137 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Vssel137")
                .field("vsel0", &self.vsel0())
                .field("vsel1", &self.vsel1())
                .field("vsel2", &self.vsel2())
                .field("vsel3", &self.vsel3())
                .field("vsel4", &self.vsel4())
                .field("vsel5", &self.vsel5())
                .field("vsel6", &self.vsel6())
                .field("vsel7", &self.vsel7())
                .field("vsel8", &self.vsel8())
                .field("vsel9", &self.vsel9())
                .field("vsel10", &self.vsel10())
                .field("vsel11", &self.vsel11())
                .field("vsel12", &self.vsel12())
                .field("vsel13", &self.vsel13())
                .field("vsel14", &self.vsel14())
                .field("vsel15", &self.vsel15())
                .field("vsel16", &self.vsel16())
                .field("vsel17", &self.vsel17())
                .field("vsel18", &self.vsel18())
                .field("vsel19", &self.vsel19())
                .field("vsel20", &self.vsel20())
                .field("vsel21", &self.vsel21())
                .field("vsel22", &self.vsel22())
                .field("vsel23", &self.vsel23())
                .field("vsel24", &self.vsel24())
                .field("vsel25", &self.vsel25())
                .field("vsel26", &self.vsel26())
                .field("vsel27", &self.vsel27())
                .field("vsel28", &self.vsel28())
                .field("vsel29", &self.vsel29())
                .field("vsel30", &self.vsel30())
                .field("vsel31", &self.vsel31())
                .finish()
        }
    }
    #[doc = "desc VSSEL138."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Vssel138(pub u32);
    impl Vssel138 {
        #[doc = "desc VSEL0."]
        #[must_use]
        #[inline(always)]
        pub const fn vsel0(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "desc VSEL0."]
        #[inline(always)]
        pub const fn set_vsel0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "desc VSEL1."]
        #[must_use]
        #[inline(always)]
        pub const fn vsel1(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "desc VSEL1."]
        #[inline(always)]
        pub const fn set_vsel1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "desc VSEL2."]
        #[must_use]
        #[inline(always)]
        pub const fn vsel2(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "desc VSEL2."]
        #[inline(always)]
        pub const fn set_vsel2(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "desc VSEL3."]
        #[must_use]
        #[inline(always)]
        pub const fn vsel3(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "desc VSEL3."]
        #[inline(always)]
        pub const fn set_vsel3(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "desc VSEL4."]
        #[must_use]
        #[inline(always)]
        pub const fn vsel4(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "desc VSEL4."]
        #[inline(always)]
        pub const fn set_vsel4(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "desc VSEL5."]
        #[must_use]
        #[inline(always)]
        pub const fn vsel5(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "desc VSEL5."]
        #[inline(always)]
        pub const fn set_vsel5(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "desc VSEL6."]
        #[must_use]
        #[inline(always)]
        pub const fn vsel6(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "desc VSEL6."]
        #[inline(always)]
        pub const fn set_vsel6(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "desc VSEL7."]
        #[must_use]
        #[inline(always)]
        pub const fn vsel7(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "desc VSEL7."]
        #[inline(always)]
        pub const fn set_vsel7(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "desc VSEL8."]
        #[must_use]
        #[inline(always)]
        pub const fn vsel8(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "desc VSEL8."]
        #[inline(always)]
        pub const fn set_vsel8(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "desc VSEL9."]
        #[must_use]
        #[inline(always)]
        pub const fn vsel9(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "desc VSEL9."]
        #[inline(always)]
        pub const fn set_vsel9(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "desc VSEL10."]
        #[must_use]
        #[inline(always)]
        pub const fn vsel10(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "desc VSEL10."]
        #[inline(always)]
        pub const fn set_vsel10(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[doc = "desc VSEL11."]
        #[must_use]
        #[inline(always)]
        pub const fn vsel11(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "desc VSEL11."]
        #[inline(always)]
        pub const fn set_vsel11(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[doc = "desc VSEL12."]
        #[must_use]
        #[inline(always)]
        pub const fn vsel12(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "desc VSEL12."]
        #[inline(always)]
        pub const fn set_vsel12(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "desc VSEL13."]
        #[must_use]
        #[inline(always)]
        pub const fn vsel13(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[doc = "desc VSEL13."]
        #[inline(always)]
        pub const fn set_vsel13(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[doc = "desc VSEL14."]
        #[must_use]
        #[inline(always)]
        pub const fn vsel14(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "desc VSEL14."]
        #[inline(always)]
        pub const fn set_vsel14(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[doc = "desc VSEL15."]
        #[must_use]
        #[inline(always)]
        pub const fn vsel15(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "desc VSEL15."]
        #[inline(always)]
        pub const fn set_vsel15(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
        #[doc = "desc VSEL16."]
        #[must_use]
        #[inline(always)]
        pub const fn vsel16(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "desc VSEL16."]
        #[inline(always)]
        pub const fn set_vsel16(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "desc VSEL17."]
        #[must_use]
        #[inline(always)]
        pub const fn vsel17(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "desc VSEL17."]
        #[inline(always)]
        pub const fn set_vsel17(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[doc = "desc VSEL18."]
        #[must_use]
        #[inline(always)]
        pub const fn vsel18(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[doc = "desc VSEL18."]
        #[inline(always)]
        pub const fn set_vsel18(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
        #[doc = "desc VSEL19."]
        #[must_use]
        #[inline(always)]
        pub const fn vsel19(&self) -> bool {
            let val = (self.0 >> 19usize) & 0x01;
            val != 0
        }
        #[doc = "desc VSEL19."]
        #[inline(always)]
        pub const fn set_vsel19(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
        }
        #[doc = "desc VSEL20."]
        #[must_use]
        #[inline(always)]
        pub const fn vsel20(&self) -> bool {
            let val = (self.0 >> 20usize) & 0x01;
            val != 0
        }
        #[doc = "desc VSEL20."]
        #[inline(always)]
        pub const fn set_vsel20(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
        }
        #[doc = "desc VSEL21."]
        #[must_use]
        #[inline(always)]
        pub const fn vsel21(&self) -> bool {
            let val = (self.0 >> 21usize) & 0x01;
            val != 0
        }
        #[doc = "desc VSEL21."]
        #[inline(always)]
        pub const fn set_vsel21(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
        }
        #[doc = "desc VSEL22."]
        #[must_use]
        #[inline(always)]
        pub const fn vsel22(&self) -> bool {
            let val = (self.0 >> 22usize) & 0x01;
            val != 0
        }
        #[doc = "desc VSEL22."]
        #[inline(always)]
        pub const fn set_vsel22(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
        }
        #[doc = "desc VSEL23."]
        #[must_use]
        #[inline(always)]
        pub const fn vsel23(&self) -> bool {
            let val = (self.0 >> 23usize) & 0x01;
            val != 0
        }
        #[doc = "desc VSEL23."]
        #[inline(always)]
        pub const fn set_vsel23(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
        }
        #[doc = "desc VSEL24."]
        #[must_use]
        #[inline(always)]
        pub const fn vsel24(&self) -> bool {
            let val = (self.0 >> 24usize) & 0x01;
            val != 0
        }
        #[doc = "desc VSEL24."]
        #[inline(always)]
        pub const fn set_vsel24(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
        }
        #[doc = "desc VSEL25."]
        #[must_use]
        #[inline(always)]
        pub const fn vsel25(&self) -> bool {
            let val = (self.0 >> 25usize) & 0x01;
            val != 0
        }
        #[doc = "desc VSEL25."]
        #[inline(always)]
        pub const fn set_vsel25(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
        }
        #[doc = "desc VSEL26."]
        #[must_use]
        #[inline(always)]
        pub const fn vsel26(&self) -> bool {
            let val = (self.0 >> 26usize) & 0x01;
            val != 0
        }
        #[doc = "desc VSEL26."]
        #[inline(always)]
        pub const fn set_vsel26(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
        }
        #[doc = "desc VSEL27."]
        #[must_use]
        #[inline(always)]
        pub const fn vsel27(&self) -> bool {
            let val = (self.0 >> 27usize) & 0x01;
            val != 0
        }
        #[doc = "desc VSEL27."]
        #[inline(always)]
        pub const fn set_vsel27(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
        }
        #[doc = "desc VSEL28."]
        #[must_use]
        #[inline(always)]
        pub const fn vsel28(&self) -> bool {
            let val = (self.0 >> 28usize) & 0x01;
            val != 0
        }
        #[doc = "desc VSEL28."]
        #[inline(always)]
        pub const fn set_vsel28(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
        }
        #[doc = "desc VSEL29."]
        #[must_use]
        #[inline(always)]
        pub const fn vsel29(&self) -> bool {
            let val = (self.0 >> 29usize) & 0x01;
            val != 0
        }
        #[doc = "desc VSEL29."]
        #[inline(always)]
        pub const fn set_vsel29(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
        }
        #[doc = "desc VSEL30."]
        #[must_use]
        #[inline(always)]
        pub const fn vsel30(&self) -> bool {
            let val = (self.0 >> 30usize) & 0x01;
            val != 0
        }
        #[doc = "desc VSEL30."]
        #[inline(always)]
        pub const fn set_vsel30(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
        }
        #[doc = "desc VSEL31."]
        #[must_use]
        #[inline(always)]
        pub const fn vsel31(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[doc = "desc VSEL31."]
        #[inline(always)]
        pub const fn set_vsel31(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for Vssel138 {
        #[inline(always)]
        fn default() -> Vssel138 {
            Vssel138(0)
        }
    }
    impl core::fmt::Debug for Vssel138 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Vssel138")
                .field("vsel0", &self.vsel0())
                .field("vsel1", &self.vsel1())
                .field("vsel2", &self.vsel2())
                .field("vsel3", &self.vsel3())
                .field("vsel4", &self.vsel4())
                .field("vsel5", &self.vsel5())
                .field("vsel6", &self.vsel6())
                .field("vsel7", &self.vsel7())
                .field("vsel8", &self.vsel8())
                .field("vsel9", &self.vsel9())
                .field("vsel10", &self.vsel10())
                .field("vsel11", &self.vsel11())
                .field("vsel12", &self.vsel12())
                .field("vsel13", &self.vsel13())
                .field("vsel14", &self.vsel14())
                .field("vsel15", &self.vsel15())
                .field("vsel16", &self.vsel16())
                .field("vsel17", &self.vsel17())
                .field("vsel18", &self.vsel18())
                .field("vsel19", &self.vsel19())
                .field("vsel20", &self.vsel20())
                .field("vsel21", &self.vsel21())
                .field("vsel22", &self.vsel22())
                .field("vsel23", &self.vsel23())
                .field("vsel24", &self.vsel24())
                .field("vsel25", &self.vsel25())
                .field("vsel26", &self.vsel26())
                .field("vsel27", &self.vsel27())
                .field("vsel28", &self.vsel28())
                .field("vsel29", &self.vsel29())
                .field("vsel30", &self.vsel30())
                .field("vsel31", &self.vsel31())
                .finish()
        }
    }
    #[doc = "desc VSSEL139."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Vssel139(pub u32);
    impl Vssel139 {
        #[doc = "desc VSEL0."]
        #[must_use]
        #[inline(always)]
        pub const fn vsel0(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "desc VSEL0."]
        #[inline(always)]
        pub const fn set_vsel0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "desc VSEL1."]
        #[must_use]
        #[inline(always)]
        pub const fn vsel1(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "desc VSEL1."]
        #[inline(always)]
        pub const fn set_vsel1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "desc VSEL2."]
        #[must_use]
        #[inline(always)]
        pub const fn vsel2(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "desc VSEL2."]
        #[inline(always)]
        pub const fn set_vsel2(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "desc VSEL3."]
        #[must_use]
        #[inline(always)]
        pub const fn vsel3(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "desc VSEL3."]
        #[inline(always)]
        pub const fn set_vsel3(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "desc VSEL4."]
        #[must_use]
        #[inline(always)]
        pub const fn vsel4(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "desc VSEL4."]
        #[inline(always)]
        pub const fn set_vsel4(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "desc VSEL5."]
        #[must_use]
        #[inline(always)]
        pub const fn vsel5(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "desc VSEL5."]
        #[inline(always)]
        pub const fn set_vsel5(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "desc VSEL6."]
        #[must_use]
        #[inline(always)]
        pub const fn vsel6(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "desc VSEL6."]
        #[inline(always)]
        pub const fn set_vsel6(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "desc VSEL7."]
        #[must_use]
        #[inline(always)]
        pub const fn vsel7(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "desc VSEL7."]
        #[inline(always)]
        pub const fn set_vsel7(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "desc VSEL8."]
        #[must_use]
        #[inline(always)]
        pub const fn vsel8(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "desc VSEL8."]
        #[inline(always)]
        pub const fn set_vsel8(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "desc VSEL9."]
        #[must_use]
        #[inline(always)]
        pub const fn vsel9(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "desc VSEL9."]
        #[inline(always)]
        pub const fn set_vsel9(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "desc VSEL10."]
        #[must_use]
        #[inline(always)]
        pub const fn vsel10(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "desc VSEL10."]
        #[inline(always)]
        pub const fn set_vsel10(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[doc = "desc VSEL11."]
        #[must_use]
        #[inline(always)]
        pub const fn vsel11(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "desc VSEL11."]
        #[inline(always)]
        pub const fn set_vsel11(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[doc = "desc VSEL12."]
        #[must_use]
        #[inline(always)]
        pub const fn vsel12(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "desc VSEL12."]
        #[inline(always)]
        pub const fn set_vsel12(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "desc VSEL13."]
        #[must_use]
        #[inline(always)]
        pub const fn vsel13(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[doc = "desc VSEL13."]
        #[inline(always)]
        pub const fn set_vsel13(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[doc = "desc VSEL14."]
        #[must_use]
        #[inline(always)]
        pub const fn vsel14(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "desc VSEL14."]
        #[inline(always)]
        pub const fn set_vsel14(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[doc = "desc VSEL15."]
        #[must_use]
        #[inline(always)]
        pub const fn vsel15(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "desc VSEL15."]
        #[inline(always)]
        pub const fn set_vsel15(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
        #[doc = "desc VSEL16."]
        #[must_use]
        #[inline(always)]
        pub const fn vsel16(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "desc VSEL16."]
        #[inline(always)]
        pub const fn set_vsel16(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "desc VSEL17."]
        #[must_use]
        #[inline(always)]
        pub const fn vsel17(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "desc VSEL17."]
        #[inline(always)]
        pub const fn set_vsel17(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[doc = "desc VSEL18."]
        #[must_use]
        #[inline(always)]
        pub const fn vsel18(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[doc = "desc VSEL18."]
        #[inline(always)]
        pub const fn set_vsel18(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
        #[doc = "desc VSEL19."]
        #[must_use]
        #[inline(always)]
        pub const fn vsel19(&self) -> bool {
            let val = (self.0 >> 19usize) & 0x01;
            val != 0
        }
        #[doc = "desc VSEL19."]
        #[inline(always)]
        pub const fn set_vsel19(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
        }
        #[doc = "desc VSEL20."]
        #[must_use]
        #[inline(always)]
        pub const fn vsel20(&self) -> bool {
            let val = (self.0 >> 20usize) & 0x01;
            val != 0
        }
        #[doc = "desc VSEL20."]
        #[inline(always)]
        pub const fn set_vsel20(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
        }
        #[doc = "desc VSEL21."]
        #[must_use]
        #[inline(always)]
        pub const fn vsel21(&self) -> bool {
            let val = (self.0 >> 21usize) & 0x01;
            val != 0
        }
        #[doc = "desc VSEL21."]
        #[inline(always)]
        pub const fn set_vsel21(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
        }
        #[doc = "desc VSEL22."]
        #[must_use]
        #[inline(always)]
        pub const fn vsel22(&self) -> bool {
            let val = (self.0 >> 22usize) & 0x01;
            val != 0
        }
        #[doc = "desc VSEL22."]
        #[inline(always)]
        pub const fn set_vsel22(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
        }
        #[doc = "desc VSEL23."]
        #[must_use]
        #[inline(always)]
        pub const fn vsel23(&self) -> bool {
            let val = (self.0 >> 23usize) & 0x01;
            val != 0
        }
        #[doc = "desc VSEL23."]
        #[inline(always)]
        pub const fn set_vsel23(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
        }
        #[doc = "desc VSEL24."]
        #[must_use]
        #[inline(always)]
        pub const fn vsel24(&self) -> bool {
            let val = (self.0 >> 24usize) & 0x01;
            val != 0
        }
        #[doc = "desc VSEL24."]
        #[inline(always)]
        pub const fn set_vsel24(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
        }
        #[doc = "desc VSEL25."]
        #[must_use]
        #[inline(always)]
        pub const fn vsel25(&self) -> bool {
            let val = (self.0 >> 25usize) & 0x01;
            val != 0
        }
        #[doc = "desc VSEL25."]
        #[inline(always)]
        pub const fn set_vsel25(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
        }
        #[doc = "desc VSEL26."]
        #[must_use]
        #[inline(always)]
        pub const fn vsel26(&self) -> bool {
            let val = (self.0 >> 26usize) & 0x01;
            val != 0
        }
        #[doc = "desc VSEL26."]
        #[inline(always)]
        pub const fn set_vsel26(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
        }
        #[doc = "desc VSEL27."]
        #[must_use]
        #[inline(always)]
        pub const fn vsel27(&self) -> bool {
            let val = (self.0 >> 27usize) & 0x01;
            val != 0
        }
        #[doc = "desc VSEL27."]
        #[inline(always)]
        pub const fn set_vsel27(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
        }
        #[doc = "desc VSEL28."]
        #[must_use]
        #[inline(always)]
        pub const fn vsel28(&self) -> bool {
            let val = (self.0 >> 28usize) & 0x01;
            val != 0
        }
        #[doc = "desc VSEL28."]
        #[inline(always)]
        pub const fn set_vsel28(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
        }
        #[doc = "desc VSEL29."]
        #[must_use]
        #[inline(always)]
        pub const fn vsel29(&self) -> bool {
            let val = (self.0 >> 29usize) & 0x01;
            val != 0
        }
        #[doc = "desc VSEL29."]
        #[inline(always)]
        pub const fn set_vsel29(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
        }
        #[doc = "desc VSEL30."]
        #[must_use]
        #[inline(always)]
        pub const fn vsel30(&self) -> bool {
            let val = (self.0 >> 30usize) & 0x01;
            val != 0
        }
        #[doc = "desc VSEL30."]
        #[inline(always)]
        pub const fn set_vsel30(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
        }
        #[doc = "desc VSEL31."]
        #[must_use]
        #[inline(always)]
        pub const fn vsel31(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[doc = "desc VSEL31."]
        #[inline(always)]
        pub const fn set_vsel31(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for Vssel139 {
        #[inline(always)]
        fn default() -> Vssel139 {
            Vssel139(0)
        }
    }
    impl core::fmt::Debug for Vssel139 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Vssel139")
                .field("vsel0", &self.vsel0())
                .field("vsel1", &self.vsel1())
                .field("vsel2", &self.vsel2())
                .field("vsel3", &self.vsel3())
                .field("vsel4", &self.vsel4())
                .field("vsel5", &self.vsel5())
                .field("vsel6", &self.vsel6())
                .field("vsel7", &self.vsel7())
                .field("vsel8", &self.vsel8())
                .field("vsel9", &self.vsel9())
                .field("vsel10", &self.vsel10())
                .field("vsel11", &self.vsel11())
                .field("vsel12", &self.vsel12())
                .field("vsel13", &self.vsel13())
                .field("vsel14", &self.vsel14())
                .field("vsel15", &self.vsel15())
                .field("vsel16", &self.vsel16())
                .field("vsel17", &self.vsel17())
                .field("vsel18", &self.vsel18())
                .field("vsel19", &self.vsel19())
                .field("vsel20", &self.vsel20())
                .field("vsel21", &self.vsel21())
                .field("vsel22", &self.vsel22())
                .field("vsel23", &self.vsel23())
                .field("vsel24", &self.vsel24())
                .field("vsel25", &self.vsel25())
                .field("vsel26", &self.vsel26())
                .field("vsel27", &self.vsel27())
                .field("vsel28", &self.vsel28())
                .field("vsel29", &self.vsel29())
                .field("vsel30", &self.vsel30())
                .field("vsel31", &self.vsel31())
                .finish()
        }
    }
    #[doc = "desc VSSEL140."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Vssel140(pub u32);
    impl Vssel140 {
        #[doc = "desc VSEL0."]
        #[must_use]
        #[inline(always)]
        pub const fn vsel0(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "desc VSEL0."]
        #[inline(always)]
        pub const fn set_vsel0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "desc VSEL1."]
        #[must_use]
        #[inline(always)]
        pub const fn vsel1(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "desc VSEL1."]
        #[inline(always)]
        pub const fn set_vsel1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "desc VSEL2."]
        #[must_use]
        #[inline(always)]
        pub const fn vsel2(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "desc VSEL2."]
        #[inline(always)]
        pub const fn set_vsel2(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "desc VSEL3."]
        #[must_use]
        #[inline(always)]
        pub const fn vsel3(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "desc VSEL3."]
        #[inline(always)]
        pub const fn set_vsel3(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "desc VSEL4."]
        #[must_use]
        #[inline(always)]
        pub const fn vsel4(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "desc VSEL4."]
        #[inline(always)]
        pub const fn set_vsel4(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "desc VSEL5."]
        #[must_use]
        #[inline(always)]
        pub const fn vsel5(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "desc VSEL5."]
        #[inline(always)]
        pub const fn set_vsel5(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "desc VSEL6."]
        #[must_use]
        #[inline(always)]
        pub const fn vsel6(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "desc VSEL6."]
        #[inline(always)]
        pub const fn set_vsel6(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "desc VSEL7."]
        #[must_use]
        #[inline(always)]
        pub const fn vsel7(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "desc VSEL7."]
        #[inline(always)]
        pub const fn set_vsel7(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "desc VSEL8."]
        #[must_use]
        #[inline(always)]
        pub const fn vsel8(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "desc VSEL8."]
        #[inline(always)]
        pub const fn set_vsel8(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "desc VSEL9."]
        #[must_use]
        #[inline(always)]
        pub const fn vsel9(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "desc VSEL9."]
        #[inline(always)]
        pub const fn set_vsel9(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "desc VSEL10."]
        #[must_use]
        #[inline(always)]
        pub const fn vsel10(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "desc VSEL10."]
        #[inline(always)]
        pub const fn set_vsel10(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[doc = "desc VSEL11."]
        #[must_use]
        #[inline(always)]
        pub const fn vsel11(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "desc VSEL11."]
        #[inline(always)]
        pub const fn set_vsel11(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[doc = "desc VSEL12."]
        #[must_use]
        #[inline(always)]
        pub const fn vsel12(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "desc VSEL12."]
        #[inline(always)]
        pub const fn set_vsel12(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "desc VSEL13."]
        #[must_use]
        #[inline(always)]
        pub const fn vsel13(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[doc = "desc VSEL13."]
        #[inline(always)]
        pub const fn set_vsel13(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[doc = "desc VSEL14."]
        #[must_use]
        #[inline(always)]
        pub const fn vsel14(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "desc VSEL14."]
        #[inline(always)]
        pub const fn set_vsel14(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[doc = "desc VSEL15."]
        #[must_use]
        #[inline(always)]
        pub const fn vsel15(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "desc VSEL15."]
        #[inline(always)]
        pub const fn set_vsel15(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
        #[doc = "desc VSEL16."]
        #[must_use]
        #[inline(always)]
        pub const fn vsel16(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "desc VSEL16."]
        #[inline(always)]
        pub const fn set_vsel16(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "desc VSEL17."]
        #[must_use]
        #[inline(always)]
        pub const fn vsel17(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "desc VSEL17."]
        #[inline(always)]
        pub const fn set_vsel17(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[doc = "desc VSEL18."]
        #[must_use]
        #[inline(always)]
        pub const fn vsel18(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[doc = "desc VSEL18."]
        #[inline(always)]
        pub const fn set_vsel18(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
        #[doc = "desc VSEL19."]
        #[must_use]
        #[inline(always)]
        pub const fn vsel19(&self) -> bool {
            let val = (self.0 >> 19usize) & 0x01;
            val != 0
        }
        #[doc = "desc VSEL19."]
        #[inline(always)]
        pub const fn set_vsel19(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
        }
        #[doc = "desc VSEL20."]
        #[must_use]
        #[inline(always)]
        pub const fn vsel20(&self) -> bool {
            let val = (self.0 >> 20usize) & 0x01;
            val != 0
        }
        #[doc = "desc VSEL20."]
        #[inline(always)]
        pub const fn set_vsel20(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
        }
        #[doc = "desc VSEL21."]
        #[must_use]
        #[inline(always)]
        pub const fn vsel21(&self) -> bool {
            let val = (self.0 >> 21usize) & 0x01;
            val != 0
        }
        #[doc = "desc VSEL21."]
        #[inline(always)]
        pub const fn set_vsel21(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
        }
        #[doc = "desc VSEL22."]
        #[must_use]
        #[inline(always)]
        pub const fn vsel22(&self) -> bool {
            let val = (self.0 >> 22usize) & 0x01;
            val != 0
        }
        #[doc = "desc VSEL22."]
        #[inline(always)]
        pub const fn set_vsel22(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
        }
        #[doc = "desc VSEL23."]
        #[must_use]
        #[inline(always)]
        pub const fn vsel23(&self) -> bool {
            let val = (self.0 >> 23usize) & 0x01;
            val != 0
        }
        #[doc = "desc VSEL23."]
        #[inline(always)]
        pub const fn set_vsel23(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
        }
        #[doc = "desc VSEL24."]
        #[must_use]
        #[inline(always)]
        pub const fn vsel24(&self) -> bool {
            let val = (self.0 >> 24usize) & 0x01;
            val != 0
        }
        #[doc = "desc VSEL24."]
        #[inline(always)]
        pub const fn set_vsel24(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
        }
        #[doc = "desc VSEL25."]
        #[must_use]
        #[inline(always)]
        pub const fn vsel25(&self) -> bool {
            let val = (self.0 >> 25usize) & 0x01;
            val != 0
        }
        #[doc = "desc VSEL25."]
        #[inline(always)]
        pub const fn set_vsel25(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
        }
        #[doc = "desc VSEL26."]
        #[must_use]
        #[inline(always)]
        pub const fn vsel26(&self) -> bool {
            let val = (self.0 >> 26usize) & 0x01;
            val != 0
        }
        #[doc = "desc VSEL26."]
        #[inline(always)]
        pub const fn set_vsel26(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
        }
        #[doc = "desc VSEL27."]
        #[must_use]
        #[inline(always)]
        pub const fn vsel27(&self) -> bool {
            let val = (self.0 >> 27usize) & 0x01;
            val != 0
        }
        #[doc = "desc VSEL27."]
        #[inline(always)]
        pub const fn set_vsel27(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
        }
        #[doc = "desc VSEL28."]
        #[must_use]
        #[inline(always)]
        pub const fn vsel28(&self) -> bool {
            let val = (self.0 >> 28usize) & 0x01;
            val != 0
        }
        #[doc = "desc VSEL28."]
        #[inline(always)]
        pub const fn set_vsel28(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
        }
        #[doc = "desc VSEL29."]
        #[must_use]
        #[inline(always)]
        pub const fn vsel29(&self) -> bool {
            let val = (self.0 >> 29usize) & 0x01;
            val != 0
        }
        #[doc = "desc VSEL29."]
        #[inline(always)]
        pub const fn set_vsel29(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
        }
        #[doc = "desc VSEL30."]
        #[must_use]
        #[inline(always)]
        pub const fn vsel30(&self) -> bool {
            let val = (self.0 >> 30usize) & 0x01;
            val != 0
        }
        #[doc = "desc VSEL30."]
        #[inline(always)]
        pub const fn set_vsel30(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
        }
        #[doc = "desc VSEL31."]
        #[must_use]
        #[inline(always)]
        pub const fn vsel31(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[doc = "desc VSEL31."]
        #[inline(always)]
        pub const fn set_vsel31(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for Vssel140 {
        #[inline(always)]
        fn default() -> Vssel140 {
            Vssel140(0)
        }
    }
    impl core::fmt::Debug for Vssel140 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Vssel140")
                .field("vsel0", &self.vsel0())
                .field("vsel1", &self.vsel1())
                .field("vsel2", &self.vsel2())
                .field("vsel3", &self.vsel3())
                .field("vsel4", &self.vsel4())
                .field("vsel5", &self.vsel5())
                .field("vsel6", &self.vsel6())
                .field("vsel7", &self.vsel7())
                .field("vsel8", &self.vsel8())
                .field("vsel9", &self.vsel9())
                .field("vsel10", &self.vsel10())
                .field("vsel11", &self.vsel11())
                .field("vsel12", &self.vsel12())
                .field("vsel13", &self.vsel13())
                .field("vsel14", &self.vsel14())
                .field("vsel15", &self.vsel15())
                .field("vsel16", &self.vsel16())
                .field("vsel17", &self.vsel17())
                .field("vsel18", &self.vsel18())
                .field("vsel19", &self.vsel19())
                .field("vsel20", &self.vsel20())
                .field("vsel21", &self.vsel21())
                .field("vsel22", &self.vsel22())
                .field("vsel23", &self.vsel23())
                .field("vsel24", &self.vsel24())
                .field("vsel25", &self.vsel25())
                .field("vsel26", &self.vsel26())
                .field("vsel27", &self.vsel27())
                .field("vsel28", &self.vsel28())
                .field("vsel29", &self.vsel29())
                .field("vsel30", &self.vsel30())
                .field("vsel31", &self.vsel31())
                .finish()
        }
    }
    #[doc = "desc VSSEL141."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Vssel141(pub u32);
    impl Vssel141 {
        #[doc = "desc VSEL0."]
        #[must_use]
        #[inline(always)]
        pub const fn vsel0(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "desc VSEL0."]
        #[inline(always)]
        pub const fn set_vsel0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "desc VSEL1."]
        #[must_use]
        #[inline(always)]
        pub const fn vsel1(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "desc VSEL1."]
        #[inline(always)]
        pub const fn set_vsel1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "desc VSEL2."]
        #[must_use]
        #[inline(always)]
        pub const fn vsel2(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "desc VSEL2."]
        #[inline(always)]
        pub const fn set_vsel2(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "desc VSEL3."]
        #[must_use]
        #[inline(always)]
        pub const fn vsel3(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "desc VSEL3."]
        #[inline(always)]
        pub const fn set_vsel3(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "desc VSEL4."]
        #[must_use]
        #[inline(always)]
        pub const fn vsel4(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "desc VSEL4."]
        #[inline(always)]
        pub const fn set_vsel4(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "desc VSEL5."]
        #[must_use]
        #[inline(always)]
        pub const fn vsel5(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "desc VSEL5."]
        #[inline(always)]
        pub const fn set_vsel5(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "desc VSEL6."]
        #[must_use]
        #[inline(always)]
        pub const fn vsel6(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "desc VSEL6."]
        #[inline(always)]
        pub const fn set_vsel6(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "desc VSEL7."]
        #[must_use]
        #[inline(always)]
        pub const fn vsel7(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "desc VSEL7."]
        #[inline(always)]
        pub const fn set_vsel7(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "desc VSEL8."]
        #[must_use]
        #[inline(always)]
        pub const fn vsel8(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "desc VSEL8."]
        #[inline(always)]
        pub const fn set_vsel8(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "desc VSEL9."]
        #[must_use]
        #[inline(always)]
        pub const fn vsel9(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "desc VSEL9."]
        #[inline(always)]
        pub const fn set_vsel9(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "desc VSEL10."]
        #[must_use]
        #[inline(always)]
        pub const fn vsel10(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "desc VSEL10."]
        #[inline(always)]
        pub const fn set_vsel10(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[doc = "desc VSEL11."]
        #[must_use]
        #[inline(always)]
        pub const fn vsel11(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "desc VSEL11."]
        #[inline(always)]
        pub const fn set_vsel11(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[doc = "desc VSEL12."]
        #[must_use]
        #[inline(always)]
        pub const fn vsel12(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "desc VSEL12."]
        #[inline(always)]
        pub const fn set_vsel12(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "desc VSEL13."]
        #[must_use]
        #[inline(always)]
        pub const fn vsel13(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[doc = "desc VSEL13."]
        #[inline(always)]
        pub const fn set_vsel13(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[doc = "desc VSEL14."]
        #[must_use]
        #[inline(always)]
        pub const fn vsel14(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "desc VSEL14."]
        #[inline(always)]
        pub const fn set_vsel14(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[doc = "desc VSEL15."]
        #[must_use]
        #[inline(always)]
        pub const fn vsel15(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "desc VSEL15."]
        #[inline(always)]
        pub const fn set_vsel15(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
        #[doc = "desc VSEL16."]
        #[must_use]
        #[inline(always)]
        pub const fn vsel16(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "desc VSEL16."]
        #[inline(always)]
        pub const fn set_vsel16(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "desc VSEL17."]
        #[must_use]
        #[inline(always)]
        pub const fn vsel17(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "desc VSEL17."]
        #[inline(always)]
        pub const fn set_vsel17(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[doc = "desc VSEL18."]
        #[must_use]
        #[inline(always)]
        pub const fn vsel18(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[doc = "desc VSEL18."]
        #[inline(always)]
        pub const fn set_vsel18(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
        #[doc = "desc VSEL19."]
        #[must_use]
        #[inline(always)]
        pub const fn vsel19(&self) -> bool {
            let val = (self.0 >> 19usize) & 0x01;
            val != 0
        }
        #[doc = "desc VSEL19."]
        #[inline(always)]
        pub const fn set_vsel19(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
        }
        #[doc = "desc VSEL20."]
        #[must_use]
        #[inline(always)]
        pub const fn vsel20(&self) -> bool {
            let val = (self.0 >> 20usize) & 0x01;
            val != 0
        }
        #[doc = "desc VSEL20."]
        #[inline(always)]
        pub const fn set_vsel20(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
        }
        #[doc = "desc VSEL21."]
        #[must_use]
        #[inline(always)]
        pub const fn vsel21(&self) -> bool {
            let val = (self.0 >> 21usize) & 0x01;
            val != 0
        }
        #[doc = "desc VSEL21."]
        #[inline(always)]
        pub const fn set_vsel21(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
        }
        #[doc = "desc VSEL22."]
        #[must_use]
        #[inline(always)]
        pub const fn vsel22(&self) -> bool {
            let val = (self.0 >> 22usize) & 0x01;
            val != 0
        }
        #[doc = "desc VSEL22."]
        #[inline(always)]
        pub const fn set_vsel22(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
        }
        #[doc = "desc VSEL23."]
        #[must_use]
        #[inline(always)]
        pub const fn vsel23(&self) -> bool {
            let val = (self.0 >> 23usize) & 0x01;
            val != 0
        }
        #[doc = "desc VSEL23."]
        #[inline(always)]
        pub const fn set_vsel23(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
        }
        #[doc = "desc VSEL24."]
        #[must_use]
        #[inline(always)]
        pub const fn vsel24(&self) -> bool {
            let val = (self.0 >> 24usize) & 0x01;
            val != 0
        }
        #[doc = "desc VSEL24."]
        #[inline(always)]
        pub const fn set_vsel24(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
        }
        #[doc = "desc VSEL25."]
        #[must_use]
        #[inline(always)]
        pub const fn vsel25(&self) -> bool {
            let val = (self.0 >> 25usize) & 0x01;
            val != 0
        }
        #[doc = "desc VSEL25."]
        #[inline(always)]
        pub const fn set_vsel25(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
        }
        #[doc = "desc VSEL26."]
        #[must_use]
        #[inline(always)]
        pub const fn vsel26(&self) -> bool {
            let val = (self.0 >> 26usize) & 0x01;
            val != 0
        }
        #[doc = "desc VSEL26."]
        #[inline(always)]
        pub const fn set_vsel26(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
        }
        #[doc = "desc VSEL27."]
        #[must_use]
        #[inline(always)]
        pub const fn vsel27(&self) -> bool {
            let val = (self.0 >> 27usize) & 0x01;
            val != 0
        }
        #[doc = "desc VSEL27."]
        #[inline(always)]
        pub const fn set_vsel27(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
        }
        #[doc = "desc VSEL28."]
        #[must_use]
        #[inline(always)]
        pub const fn vsel28(&self) -> bool {
            let val = (self.0 >> 28usize) & 0x01;
            val != 0
        }
        #[doc = "desc VSEL28."]
        #[inline(always)]
        pub const fn set_vsel28(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
        }
        #[doc = "desc VSEL29."]
        #[must_use]
        #[inline(always)]
        pub const fn vsel29(&self) -> bool {
            let val = (self.0 >> 29usize) & 0x01;
            val != 0
        }
        #[doc = "desc VSEL29."]
        #[inline(always)]
        pub const fn set_vsel29(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
        }
        #[doc = "desc VSEL30."]
        #[must_use]
        #[inline(always)]
        pub const fn vsel30(&self) -> bool {
            let val = (self.0 >> 30usize) & 0x01;
            val != 0
        }
        #[doc = "desc VSEL30."]
        #[inline(always)]
        pub const fn set_vsel30(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
        }
        #[doc = "desc VSEL31."]
        #[must_use]
        #[inline(always)]
        pub const fn vsel31(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[doc = "desc VSEL31."]
        #[inline(always)]
        pub const fn set_vsel31(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for Vssel141 {
        #[inline(always)]
        fn default() -> Vssel141 {
            Vssel141(0)
        }
    }
    impl core::fmt::Debug for Vssel141 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Vssel141")
                .field("vsel0", &self.vsel0())
                .field("vsel1", &self.vsel1())
                .field("vsel2", &self.vsel2())
                .field("vsel3", &self.vsel3())
                .field("vsel4", &self.vsel4())
                .field("vsel5", &self.vsel5())
                .field("vsel6", &self.vsel6())
                .field("vsel7", &self.vsel7())
                .field("vsel8", &self.vsel8())
                .field("vsel9", &self.vsel9())
                .field("vsel10", &self.vsel10())
                .field("vsel11", &self.vsel11())
                .field("vsel12", &self.vsel12())
                .field("vsel13", &self.vsel13())
                .field("vsel14", &self.vsel14())
                .field("vsel15", &self.vsel15())
                .field("vsel16", &self.vsel16())
                .field("vsel17", &self.vsel17())
                .field("vsel18", &self.vsel18())
                .field("vsel19", &self.vsel19())
                .field("vsel20", &self.vsel20())
                .field("vsel21", &self.vsel21())
                .field("vsel22", &self.vsel22())
                .field("vsel23", &self.vsel23())
                .field("vsel24", &self.vsel24())
                .field("vsel25", &self.vsel25())
                .field("vsel26", &self.vsel26())
                .field("vsel27", &self.vsel27())
                .field("vsel28", &self.vsel28())
                .field("vsel29", &self.vsel29())
                .field("vsel30", &self.vsel30())
                .field("vsel31", &self.vsel31())
                .finish()
        }
    }
    #[doc = "desc VSSEL142."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Vssel142(pub u32);
    impl Vssel142 {
        #[doc = "desc VSEL0."]
        #[must_use]
        #[inline(always)]
        pub const fn vsel0(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "desc VSEL0."]
        #[inline(always)]
        pub const fn set_vsel0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "desc VSEL1."]
        #[must_use]
        #[inline(always)]
        pub const fn vsel1(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "desc VSEL1."]
        #[inline(always)]
        pub const fn set_vsel1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "desc VSEL2."]
        #[must_use]
        #[inline(always)]
        pub const fn vsel2(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "desc VSEL2."]
        #[inline(always)]
        pub const fn set_vsel2(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "desc VSEL3."]
        #[must_use]
        #[inline(always)]
        pub const fn vsel3(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "desc VSEL3."]
        #[inline(always)]
        pub const fn set_vsel3(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "desc VSEL4."]
        #[must_use]
        #[inline(always)]
        pub const fn vsel4(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "desc VSEL4."]
        #[inline(always)]
        pub const fn set_vsel4(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "desc VSEL5."]
        #[must_use]
        #[inline(always)]
        pub const fn vsel5(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "desc VSEL5."]
        #[inline(always)]
        pub const fn set_vsel5(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "desc VSEL6."]
        #[must_use]
        #[inline(always)]
        pub const fn vsel6(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "desc VSEL6."]
        #[inline(always)]
        pub const fn set_vsel6(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "desc VSEL7."]
        #[must_use]
        #[inline(always)]
        pub const fn vsel7(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "desc VSEL7."]
        #[inline(always)]
        pub const fn set_vsel7(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "desc VSEL8."]
        #[must_use]
        #[inline(always)]
        pub const fn vsel8(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "desc VSEL8."]
        #[inline(always)]
        pub const fn set_vsel8(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "desc VSEL9."]
        #[must_use]
        #[inline(always)]
        pub const fn vsel9(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "desc VSEL9."]
        #[inline(always)]
        pub const fn set_vsel9(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "desc VSEL10."]
        #[must_use]
        #[inline(always)]
        pub const fn vsel10(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "desc VSEL10."]
        #[inline(always)]
        pub const fn set_vsel10(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[doc = "desc VSEL11."]
        #[must_use]
        #[inline(always)]
        pub const fn vsel11(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "desc VSEL11."]
        #[inline(always)]
        pub const fn set_vsel11(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[doc = "desc VSEL12."]
        #[must_use]
        #[inline(always)]
        pub const fn vsel12(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "desc VSEL12."]
        #[inline(always)]
        pub const fn set_vsel12(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "desc VSEL13."]
        #[must_use]
        #[inline(always)]
        pub const fn vsel13(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[doc = "desc VSEL13."]
        #[inline(always)]
        pub const fn set_vsel13(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[doc = "desc VSEL14."]
        #[must_use]
        #[inline(always)]
        pub const fn vsel14(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "desc VSEL14."]
        #[inline(always)]
        pub const fn set_vsel14(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[doc = "desc VSEL15."]
        #[must_use]
        #[inline(always)]
        pub const fn vsel15(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "desc VSEL15."]
        #[inline(always)]
        pub const fn set_vsel15(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
        #[doc = "desc VSEL16."]
        #[must_use]
        #[inline(always)]
        pub const fn vsel16(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "desc VSEL16."]
        #[inline(always)]
        pub const fn set_vsel16(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "desc VSEL17."]
        #[must_use]
        #[inline(always)]
        pub const fn vsel17(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "desc VSEL17."]
        #[inline(always)]
        pub const fn set_vsel17(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[doc = "desc VSEL18."]
        #[must_use]
        #[inline(always)]
        pub const fn vsel18(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[doc = "desc VSEL18."]
        #[inline(always)]
        pub const fn set_vsel18(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
        #[doc = "desc VSEL19."]
        #[must_use]
        #[inline(always)]
        pub const fn vsel19(&self) -> bool {
            let val = (self.0 >> 19usize) & 0x01;
            val != 0
        }
        #[doc = "desc VSEL19."]
        #[inline(always)]
        pub const fn set_vsel19(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
        }
        #[doc = "desc VSEL20."]
        #[must_use]
        #[inline(always)]
        pub const fn vsel20(&self) -> bool {
            let val = (self.0 >> 20usize) & 0x01;
            val != 0
        }
        #[doc = "desc VSEL20."]
        #[inline(always)]
        pub const fn set_vsel20(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
        }
        #[doc = "desc VSEL21."]
        #[must_use]
        #[inline(always)]
        pub const fn vsel21(&self) -> bool {
            let val = (self.0 >> 21usize) & 0x01;
            val != 0
        }
        #[doc = "desc VSEL21."]
        #[inline(always)]
        pub const fn set_vsel21(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
        }
        #[doc = "desc VSEL22."]
        #[must_use]
        #[inline(always)]
        pub const fn vsel22(&self) -> bool {
            let val = (self.0 >> 22usize) & 0x01;
            val != 0
        }
        #[doc = "desc VSEL22."]
        #[inline(always)]
        pub const fn set_vsel22(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
        }
        #[doc = "desc VSEL23."]
        #[must_use]
        #[inline(always)]
        pub const fn vsel23(&self) -> bool {
            let val = (self.0 >> 23usize) & 0x01;
            val != 0
        }
        #[doc = "desc VSEL23."]
        #[inline(always)]
        pub const fn set_vsel23(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
        }
        #[doc = "desc VSEL24."]
        #[must_use]
        #[inline(always)]
        pub const fn vsel24(&self) -> bool {
            let val = (self.0 >> 24usize) & 0x01;
            val != 0
        }
        #[doc = "desc VSEL24."]
        #[inline(always)]
        pub const fn set_vsel24(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
        }
        #[doc = "desc VSEL25."]
        #[must_use]
        #[inline(always)]
        pub const fn vsel25(&self) -> bool {
            let val = (self.0 >> 25usize) & 0x01;
            val != 0
        }
        #[doc = "desc VSEL25."]
        #[inline(always)]
        pub const fn set_vsel25(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
        }
        #[doc = "desc VSEL26."]
        #[must_use]
        #[inline(always)]
        pub const fn vsel26(&self) -> bool {
            let val = (self.0 >> 26usize) & 0x01;
            val != 0
        }
        #[doc = "desc VSEL26."]
        #[inline(always)]
        pub const fn set_vsel26(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
        }
        #[doc = "desc VSEL27."]
        #[must_use]
        #[inline(always)]
        pub const fn vsel27(&self) -> bool {
            let val = (self.0 >> 27usize) & 0x01;
            val != 0
        }
        #[doc = "desc VSEL27."]
        #[inline(always)]
        pub const fn set_vsel27(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
        }
        #[doc = "desc VSEL28."]
        #[must_use]
        #[inline(always)]
        pub const fn vsel28(&self) -> bool {
            let val = (self.0 >> 28usize) & 0x01;
            val != 0
        }
        #[doc = "desc VSEL28."]
        #[inline(always)]
        pub const fn set_vsel28(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
        }
        #[doc = "desc VSEL29."]
        #[must_use]
        #[inline(always)]
        pub const fn vsel29(&self) -> bool {
            let val = (self.0 >> 29usize) & 0x01;
            val != 0
        }
        #[doc = "desc VSEL29."]
        #[inline(always)]
        pub const fn set_vsel29(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
        }
        #[doc = "desc VSEL30."]
        #[must_use]
        #[inline(always)]
        pub const fn vsel30(&self) -> bool {
            let val = (self.0 >> 30usize) & 0x01;
            val != 0
        }
        #[doc = "desc VSEL30."]
        #[inline(always)]
        pub const fn set_vsel30(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
        }
        #[doc = "desc VSEL31."]
        #[must_use]
        #[inline(always)]
        pub const fn vsel31(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[doc = "desc VSEL31."]
        #[inline(always)]
        pub const fn set_vsel31(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for Vssel142 {
        #[inline(always)]
        fn default() -> Vssel142 {
            Vssel142(0)
        }
    }
    impl core::fmt::Debug for Vssel142 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Vssel142")
                .field("vsel0", &self.vsel0())
                .field("vsel1", &self.vsel1())
                .field("vsel2", &self.vsel2())
                .field("vsel3", &self.vsel3())
                .field("vsel4", &self.vsel4())
                .field("vsel5", &self.vsel5())
                .field("vsel6", &self.vsel6())
                .field("vsel7", &self.vsel7())
                .field("vsel8", &self.vsel8())
                .field("vsel9", &self.vsel9())
                .field("vsel10", &self.vsel10())
                .field("vsel11", &self.vsel11())
                .field("vsel12", &self.vsel12())
                .field("vsel13", &self.vsel13())
                .field("vsel14", &self.vsel14())
                .field("vsel15", &self.vsel15())
                .field("vsel16", &self.vsel16())
                .field("vsel17", &self.vsel17())
                .field("vsel18", &self.vsel18())
                .field("vsel19", &self.vsel19())
                .field("vsel20", &self.vsel20())
                .field("vsel21", &self.vsel21())
                .field("vsel22", &self.vsel22())
                .field("vsel23", &self.vsel23())
                .field("vsel24", &self.vsel24())
                .field("vsel25", &self.vsel25())
                .field("vsel26", &self.vsel26())
                .field("vsel27", &self.vsel27())
                .field("vsel28", &self.vsel28())
                .field("vsel29", &self.vsel29())
                .field("vsel30", &self.vsel30())
                .field("vsel31", &self.vsel31())
                .finish()
        }
    }
    #[doc = "desc VSSEL143."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Vssel143(pub u32);
    impl Vssel143 {
        #[doc = "desc VSEL0."]
        #[must_use]
        #[inline(always)]
        pub const fn vsel0(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "desc VSEL0."]
        #[inline(always)]
        pub const fn set_vsel0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "desc VSEL1."]
        #[must_use]
        #[inline(always)]
        pub const fn vsel1(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "desc VSEL1."]
        #[inline(always)]
        pub const fn set_vsel1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "desc VSEL2."]
        #[must_use]
        #[inline(always)]
        pub const fn vsel2(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "desc VSEL2."]
        #[inline(always)]
        pub const fn set_vsel2(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "desc VSEL3."]
        #[must_use]
        #[inline(always)]
        pub const fn vsel3(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "desc VSEL3."]
        #[inline(always)]
        pub const fn set_vsel3(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "desc VSEL4."]
        #[must_use]
        #[inline(always)]
        pub const fn vsel4(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "desc VSEL4."]
        #[inline(always)]
        pub const fn set_vsel4(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "desc VSEL5."]
        #[must_use]
        #[inline(always)]
        pub const fn vsel5(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "desc VSEL5."]
        #[inline(always)]
        pub const fn set_vsel5(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "desc VSEL6."]
        #[must_use]
        #[inline(always)]
        pub const fn vsel6(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "desc VSEL6."]
        #[inline(always)]
        pub const fn set_vsel6(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "desc VSEL7."]
        #[must_use]
        #[inline(always)]
        pub const fn vsel7(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "desc VSEL7."]
        #[inline(always)]
        pub const fn set_vsel7(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "desc VSEL8."]
        #[must_use]
        #[inline(always)]
        pub const fn vsel8(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "desc VSEL8."]
        #[inline(always)]
        pub const fn set_vsel8(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "desc VSEL9."]
        #[must_use]
        #[inline(always)]
        pub const fn vsel9(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "desc VSEL9."]
        #[inline(always)]
        pub const fn set_vsel9(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "desc VSEL10."]
        #[must_use]
        #[inline(always)]
        pub const fn vsel10(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "desc VSEL10."]
        #[inline(always)]
        pub const fn set_vsel10(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[doc = "desc VSEL11."]
        #[must_use]
        #[inline(always)]
        pub const fn vsel11(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "desc VSEL11."]
        #[inline(always)]
        pub const fn set_vsel11(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[doc = "desc VSEL12."]
        #[must_use]
        #[inline(always)]
        pub const fn vsel12(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "desc VSEL12."]
        #[inline(always)]
        pub const fn set_vsel12(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "desc VSEL13."]
        #[must_use]
        #[inline(always)]
        pub const fn vsel13(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[doc = "desc VSEL13."]
        #[inline(always)]
        pub const fn set_vsel13(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[doc = "desc VSEL14."]
        #[must_use]
        #[inline(always)]
        pub const fn vsel14(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "desc VSEL14."]
        #[inline(always)]
        pub const fn set_vsel14(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[doc = "desc VSEL15."]
        #[must_use]
        #[inline(always)]
        pub const fn vsel15(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "desc VSEL15."]
        #[inline(always)]
        pub const fn set_vsel15(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
        #[doc = "desc VSEL16."]
        #[must_use]
        #[inline(always)]
        pub const fn vsel16(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "desc VSEL16."]
        #[inline(always)]
        pub const fn set_vsel16(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "desc VSEL17."]
        #[must_use]
        #[inline(always)]
        pub const fn vsel17(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "desc VSEL17."]
        #[inline(always)]
        pub const fn set_vsel17(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[doc = "desc VSEL18."]
        #[must_use]
        #[inline(always)]
        pub const fn vsel18(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[doc = "desc VSEL18."]
        #[inline(always)]
        pub const fn set_vsel18(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
        #[doc = "desc VSEL19."]
        #[must_use]
        #[inline(always)]
        pub const fn vsel19(&self) -> bool {
            let val = (self.0 >> 19usize) & 0x01;
            val != 0
        }
        #[doc = "desc VSEL19."]
        #[inline(always)]
        pub const fn set_vsel19(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
        }
        #[doc = "desc VSEL20."]
        #[must_use]
        #[inline(always)]
        pub const fn vsel20(&self) -> bool {
            let val = (self.0 >> 20usize) & 0x01;
            val != 0
        }
        #[doc = "desc VSEL20."]
        #[inline(always)]
        pub const fn set_vsel20(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
        }
        #[doc = "desc VSEL21."]
        #[must_use]
        #[inline(always)]
        pub const fn vsel21(&self) -> bool {
            let val = (self.0 >> 21usize) & 0x01;
            val != 0
        }
        #[doc = "desc VSEL21."]
        #[inline(always)]
        pub const fn set_vsel21(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
        }
        #[doc = "desc VSEL22."]
        #[must_use]
        #[inline(always)]
        pub const fn vsel22(&self) -> bool {
            let val = (self.0 >> 22usize) & 0x01;
            val != 0
        }
        #[doc = "desc VSEL22."]
        #[inline(always)]
        pub const fn set_vsel22(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
        }
        #[doc = "desc VSEL23."]
        #[must_use]
        #[inline(always)]
        pub const fn vsel23(&self) -> bool {
            let val = (self.0 >> 23usize) & 0x01;
            val != 0
        }
        #[doc = "desc VSEL23."]
        #[inline(always)]
        pub const fn set_vsel23(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
        }
        #[doc = "desc VSEL24."]
        #[must_use]
        #[inline(always)]
        pub const fn vsel24(&self) -> bool {
            let val = (self.0 >> 24usize) & 0x01;
            val != 0
        }
        #[doc = "desc VSEL24."]
        #[inline(always)]
        pub const fn set_vsel24(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
        }
        #[doc = "desc VSEL25."]
        #[must_use]
        #[inline(always)]
        pub const fn vsel25(&self) -> bool {
            let val = (self.0 >> 25usize) & 0x01;
            val != 0
        }
        #[doc = "desc VSEL25."]
        #[inline(always)]
        pub const fn set_vsel25(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
        }
        #[doc = "desc VSEL26."]
        #[must_use]
        #[inline(always)]
        pub const fn vsel26(&self) -> bool {
            let val = (self.0 >> 26usize) & 0x01;
            val != 0
        }
        #[doc = "desc VSEL26."]
        #[inline(always)]
        pub const fn set_vsel26(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
        }
        #[doc = "desc VSEL27."]
        #[must_use]
        #[inline(always)]
        pub const fn vsel27(&self) -> bool {
            let val = (self.0 >> 27usize) & 0x01;
            val != 0
        }
        #[doc = "desc VSEL27."]
        #[inline(always)]
        pub const fn set_vsel27(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
        }
        #[doc = "desc VSEL28."]
        #[must_use]
        #[inline(always)]
        pub const fn vsel28(&self) -> bool {
            let val = (self.0 >> 28usize) & 0x01;
            val != 0
        }
        #[doc = "desc VSEL28."]
        #[inline(always)]
        pub const fn set_vsel28(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
        }
        #[doc = "desc VSEL29."]
        #[must_use]
        #[inline(always)]
        pub const fn vsel29(&self) -> bool {
            let val = (self.0 >> 29usize) & 0x01;
            val != 0
        }
        #[doc = "desc VSEL29."]
        #[inline(always)]
        pub const fn set_vsel29(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
        }
        #[doc = "desc VSEL30."]
        #[must_use]
        #[inline(always)]
        pub const fn vsel30(&self) -> bool {
            let val = (self.0 >> 30usize) & 0x01;
            val != 0
        }
        #[doc = "desc VSEL30."]
        #[inline(always)]
        pub const fn set_vsel30(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
        }
        #[doc = "desc VSEL31."]
        #[must_use]
        #[inline(always)]
        pub const fn vsel31(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[doc = "desc VSEL31."]
        #[inline(always)]
        pub const fn set_vsel31(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for Vssel143 {
        #[inline(always)]
        fn default() -> Vssel143 {
            Vssel143(0)
        }
    }
    impl core::fmt::Debug for Vssel143 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Vssel143")
                .field("vsel0", &self.vsel0())
                .field("vsel1", &self.vsel1())
                .field("vsel2", &self.vsel2())
                .field("vsel3", &self.vsel3())
                .field("vsel4", &self.vsel4())
                .field("vsel5", &self.vsel5())
                .field("vsel6", &self.vsel6())
                .field("vsel7", &self.vsel7())
                .field("vsel8", &self.vsel8())
                .field("vsel9", &self.vsel9())
                .field("vsel10", &self.vsel10())
                .field("vsel11", &self.vsel11())
                .field("vsel12", &self.vsel12())
                .field("vsel13", &self.vsel13())
                .field("vsel14", &self.vsel14())
                .field("vsel15", &self.vsel15())
                .field("vsel16", &self.vsel16())
                .field("vsel17", &self.vsel17())
                .field("vsel18", &self.vsel18())
                .field("vsel19", &self.vsel19())
                .field("vsel20", &self.vsel20())
                .field("vsel21", &self.vsel21())
                .field("vsel22", &self.vsel22())
                .field("vsel23", &self.vsel23())
                .field("vsel24", &self.vsel24())
                .field("vsel25", &self.vsel25())
                .field("vsel26", &self.vsel26())
                .field("vsel27", &self.vsel27())
                .field("vsel28", &self.vsel28())
                .field("vsel29", &self.vsel29())
                .field("vsel30", &self.vsel30())
                .field("vsel31", &self.vsel31())
                .finish()
        }
    }
    #[doc = "desc WKEN."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Wken(pub u32);
    impl Wken {
        #[doc = "desc EIRQWKEN."]
        #[must_use]
        #[inline(always)]
        pub const fn eirqwken(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[doc = "desc EIRQWKEN."]
        #[inline(always)]
        pub const fn set_eirqwken(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
        }
        #[doc = "desc SWDTWKEN."]
        #[must_use]
        #[inline(always)]
        pub const fn swdtwken(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "desc SWDTWKEN."]
        #[inline(always)]
        pub const fn set_swdtwken(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "desc PVD1WKEN."]
        #[must_use]
        #[inline(always)]
        pub const fn pvd1wken(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "desc PVD1WKEN."]
        #[inline(always)]
        pub const fn set_pvd1wken(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[doc = "desc PVD2WKEN."]
        #[must_use]
        #[inline(always)]
        pub const fn pvd2wken(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[doc = "desc PVD2WKEN."]
        #[inline(always)]
        pub const fn set_pvd2wken(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
        #[doc = "desc CMPWKEN."]
        #[must_use]
        #[inline(always)]
        pub const fn cmpwken(&self) -> bool {
            let val = (self.0 >> 19usize) & 0x01;
            val != 0
        }
        #[doc = "desc CMPWKEN."]
        #[inline(always)]
        pub const fn set_cmpwken(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
        }
        #[doc = "desc WKTMWKEN."]
        #[must_use]
        #[inline(always)]
        pub const fn wktmwken(&self) -> bool {
            let val = (self.0 >> 20usize) & 0x01;
            val != 0
        }
        #[doc = "desc WKTMWKEN."]
        #[inline(always)]
        pub const fn set_wktmwken(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
        }
        #[doc = "desc RTCALMWKEN."]
        #[must_use]
        #[inline(always)]
        pub const fn rtcalmwken(&self) -> bool {
            let val = (self.0 >> 21usize) & 0x01;
            val != 0
        }
        #[doc = "desc RTCALMWKEN."]
        #[inline(always)]
        pub const fn set_rtcalmwken(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
        }
        #[doc = "desc RTCPRDWKEN."]
        #[must_use]
        #[inline(always)]
        pub const fn rtcprdwken(&self) -> bool {
            let val = (self.0 >> 22usize) & 0x01;
            val != 0
        }
        #[doc = "desc RTCPRDWKEN."]
        #[inline(always)]
        pub const fn set_rtcprdwken(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
        }
        #[doc = "desc TMR0CMPWKEN."]
        #[must_use]
        #[inline(always)]
        pub const fn tmr0cmpwken(&self) -> bool {
            let val = (self.0 >> 23usize) & 0x01;
            val != 0
        }
        #[doc = "desc TMR0CMPWKEN."]
        #[inline(always)]
        pub const fn set_tmr0cmpwken(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
        }
        #[doc = "desc TMR2GCMWKEN."]
        #[must_use]
        #[inline(always)]
        pub const fn tmr2gcmwken(&self) -> bool {
            let val = (self.0 >> 24usize) & 0x01;
            val != 0
        }
        #[doc = "desc TMR2GCMWKEN."]
        #[inline(always)]
        pub const fn set_tmr2gcmwken(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
        }
        #[doc = "desc TMR2OVFWKEN."]
        #[must_use]
        #[inline(always)]
        pub const fn tmr2ovfwken(&self) -> bool {
            let val = (self.0 >> 25usize) & 0x01;
            val != 0
        }
        #[doc = "desc TMR2OVFWKEN."]
        #[inline(always)]
        pub const fn set_tmr2ovfwken(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
        }
        #[doc = "desc RXWKEN."]
        #[must_use]
        #[inline(always)]
        pub const fn rxwken(&self) -> bool {
            let val = (self.0 >> 26usize) & 0x01;
            val != 0
        }
        #[doc = "desc RXWKEN."]
        #[inline(always)]
        pub const fn set_rxwken(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
        }
        #[doc = "desc USHWKEN."]
        #[must_use]
        #[inline(always)]
        pub const fn ushwken(&self) -> bool {
            let val = (self.0 >> 27usize) & 0x01;
            val != 0
        }
        #[doc = "desc USHWKEN."]
        #[inline(always)]
        pub const fn set_ushwken(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
        }
        #[doc = "desc USFWKEN."]
        #[must_use]
        #[inline(always)]
        pub const fn usfwken(&self) -> bool {
            let val = (self.0 >> 28usize) & 0x01;
            val != 0
        }
        #[doc = "desc USFWKEN."]
        #[inline(always)]
        pub const fn set_usfwken(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
        }
        #[doc = "desc ETHWKEN."]
        #[must_use]
        #[inline(always)]
        pub const fn ethwken(&self) -> bool {
            let val = (self.0 >> 29usize) & 0x01;
            val != 0
        }
        #[doc = "desc ETHWKEN."]
        #[inline(always)]
        pub const fn set_ethwken(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
        }
    }
    impl Default for Wken {
        #[inline(always)]
        fn default() -> Wken {
            Wken(0)
        }
    }
    impl core::fmt::Debug for Wken {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Wken")
                .field("eirqwken", &self.eirqwken())
                .field("swdtwken", &self.swdtwken())
                .field("pvd1wken", &self.pvd1wken())
                .field("pvd2wken", &self.pvd2wken())
                .field("cmpwken", &self.cmpwken())
                .field("wktmwken", &self.wktmwken())
                .field("rtcalmwken", &self.rtcalmwken())
                .field("rtcprdwken", &self.rtcprdwken())
                .field("tmr0cmpwken", &self.tmr0cmpwken())
                .field("tmr2gcmwken", &self.tmr2gcmwken())
                .field("tmr2ovfwken", &self.tmr2ovfwken())
                .field("rxwken", &self.rxwken())
                .field("ushwken", &self.ushwken())
                .field("usfwken", &self.usfwken())
                .field("ethwken", &self.ethwken())
                .finish()
        }
    }
}
