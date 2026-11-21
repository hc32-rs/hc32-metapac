#![allow(clippy::missing_safety_doc)]
#![allow(clippy::identity_op)]
#![allow(clippy::unnecessary_cast)]
#![allow(clippy::erasing_op)]

#[doc = "desc GPIO."]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Gpio {
    ptr: *mut u8,
}
unsafe impl Send for Gpio {}
unsafe impl Sync for Gpio {}
impl Gpio {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "desc PA00_SEL."]
    #[inline(always)]
    pub const fn pa00_sel(self) -> crate::common::Reg<regs::Pa00Sel, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0usize) as _) }
    }
    #[doc = "desc PA01_SEL."]
    #[inline(always)]
    pub const fn pa01_sel(self) -> crate::common::Reg<regs::Pa01Sel, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x04usize) as _) }
    }
    #[doc = "desc PA02_SEL."]
    #[inline(always)]
    pub const fn pa02_sel(self) -> crate::common::Reg<regs::Pa02Sel, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x08usize) as _) }
    }
    #[doc = "desc PA03_SEL."]
    #[inline(always)]
    pub const fn pa03_sel(self) -> crate::common::Reg<regs::Pa03Sel, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0cusize) as _) }
    }
    #[doc = "desc PA04_SEL."]
    #[inline(always)]
    pub const fn pa04_sel(self) -> crate::common::Reg<regs::Pa04Sel, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x10usize) as _) }
    }
    #[doc = "desc PA05_SEL."]
    #[inline(always)]
    pub const fn pa05_sel(self) -> crate::common::Reg<regs::Pa05Sel, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x14usize) as _) }
    }
    #[doc = "desc PA06_SEL."]
    #[inline(always)]
    pub const fn pa06_sel(self) -> crate::common::Reg<regs::Pa06Sel, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x18usize) as _) }
    }
    #[doc = "desc PA07_SEL."]
    #[inline(always)]
    pub const fn pa07_sel(self) -> crate::common::Reg<regs::Pa07Sel, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1cusize) as _) }
    }
    #[doc = "desc PA08_SEL."]
    #[inline(always)]
    pub const fn pa08_sel(self) -> crate::common::Reg<regs::Pa08Sel, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x20usize) as _) }
    }
    #[doc = "desc PA09_SEL."]
    #[inline(always)]
    pub const fn pa09_sel(self) -> crate::common::Reg<regs::Pa09Sel, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x24usize) as _) }
    }
    #[doc = "desc PA10_SEL."]
    #[inline(always)]
    pub const fn pa10_sel(self) -> crate::common::Reg<regs::Pa10Sel, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x28usize) as _) }
    }
    #[doc = "desc PA11_SEL."]
    #[inline(always)]
    pub const fn pa11_sel(self) -> crate::common::Reg<regs::Pa11Sel, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x2cusize) as _) }
    }
    #[doc = "desc PA12_SEL."]
    #[inline(always)]
    pub const fn pa12_sel(self) -> crate::common::Reg<regs::Pa12Sel, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x30usize) as _) }
    }
    #[doc = "desc PA13_SEL."]
    #[inline(always)]
    pub const fn pa13_sel(self) -> crate::common::Reg<regs::Pa13Sel, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x34usize) as _) }
    }
    #[doc = "desc PA14_SEL."]
    #[inline(always)]
    pub const fn pa14_sel(self) -> crate::common::Reg<regs::Pa14Sel, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x38usize) as _) }
    }
    #[doc = "desc PA15_SEL."]
    #[inline(always)]
    pub const fn pa15_sel(self) -> crate::common::Reg<regs::Pa15Sel, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x3cusize) as _) }
    }
    #[doc = "desc PB00_SEL."]
    #[inline(always)]
    pub const fn pb00_sel(self) -> crate::common::Reg<regs::Pb00Sel, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x40usize) as _) }
    }
    #[doc = "desc PB01_SEL."]
    #[inline(always)]
    pub const fn pb01_sel(self) -> crate::common::Reg<regs::Pb01Sel, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x44usize) as _) }
    }
    #[doc = "desc PB02_SEL."]
    #[inline(always)]
    pub const fn pb02_sel(self) -> crate::common::Reg<regs::Pb02Sel, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x48usize) as _) }
    }
    #[doc = "desc PB03_SEL."]
    #[inline(always)]
    pub const fn pb03_sel(self) -> crate::common::Reg<regs::Pb03Sel, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x4cusize) as _) }
    }
    #[doc = "desc PB04_SEL."]
    #[inline(always)]
    pub const fn pb04_sel(self) -> crate::common::Reg<regs::Pb04Sel, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x50usize) as _) }
    }
    #[doc = "desc PB05_SEL."]
    #[inline(always)]
    pub const fn pb05_sel(self) -> crate::common::Reg<regs::Pb05Sel, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x54usize) as _) }
    }
    #[doc = "desc PB06_SEL."]
    #[inline(always)]
    pub const fn pb06_sel(self) -> crate::common::Reg<regs::Pb06Sel, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x58usize) as _) }
    }
    #[doc = "desc PB07_SEL."]
    #[inline(always)]
    pub const fn pb07_sel(self) -> crate::common::Reg<regs::Pb07Sel, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x5cusize) as _) }
    }
    #[doc = "desc PB08_SEL."]
    #[inline(always)]
    pub const fn pb08_sel(self) -> crate::common::Reg<regs::Pb08Sel, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x60usize) as _) }
    }
    #[doc = "desc PB09_SEL."]
    #[inline(always)]
    pub const fn pb09_sel(self) -> crate::common::Reg<regs::Pb09Sel, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x64usize) as _) }
    }
    #[doc = "desc PB10_SEL."]
    #[inline(always)]
    pub const fn pb10_sel(self) -> crate::common::Reg<regs::Pb10Sel, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x68usize) as _) }
    }
    #[doc = "desc PB11_SEL."]
    #[inline(always)]
    pub const fn pb11_sel(self) -> crate::common::Reg<regs::Pb11Sel, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x6cusize) as _) }
    }
    #[doc = "desc PB12_SEL."]
    #[inline(always)]
    pub const fn pb12_sel(self) -> crate::common::Reg<regs::Pb12Sel, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x70usize) as _) }
    }
    #[doc = "desc PB13_SEL."]
    #[inline(always)]
    pub const fn pb13_sel(self) -> crate::common::Reg<regs::Pb13Sel, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x74usize) as _) }
    }
    #[doc = "desc PB14_SEL."]
    #[inline(always)]
    pub const fn pb14_sel(self) -> crate::common::Reg<regs::Pb14Sel, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x78usize) as _) }
    }
    #[doc = "desc PB15_SEL."]
    #[inline(always)]
    pub const fn pb15_sel(self) -> crate::common::Reg<regs::Pb15Sel, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x7cusize) as _) }
    }
    #[doc = "desc PC00_SEL."]
    #[inline(always)]
    pub const fn pc00_sel(self) -> crate::common::Reg<regs::Pc00Sel, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x80usize) as _) }
    }
    #[doc = "desc PC01_SEL."]
    #[inline(always)]
    pub const fn pc01_sel(self) -> crate::common::Reg<regs::Pc01Sel, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x84usize) as _) }
    }
    #[doc = "desc PC02_SEL."]
    #[inline(always)]
    pub const fn pc02_sel(self) -> crate::common::Reg<regs::Pc02Sel, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x88usize) as _) }
    }
    #[doc = "desc PC03_SEL."]
    #[inline(always)]
    pub const fn pc03_sel(self) -> crate::common::Reg<regs::Pc03Sel, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x8cusize) as _) }
    }
    #[doc = "desc PC04_SEL."]
    #[inline(always)]
    pub const fn pc04_sel(self) -> crate::common::Reg<regs::Pc04Sel, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x90usize) as _) }
    }
    #[doc = "desc PC05_SEL."]
    #[inline(always)]
    pub const fn pc05_sel(self) -> crate::common::Reg<regs::Pc05Sel, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x94usize) as _) }
    }
    #[doc = "desc PC06_SEL."]
    #[inline(always)]
    pub const fn pc06_sel(self) -> crate::common::Reg<regs::Pc06Sel, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x98usize) as _) }
    }
    #[doc = "desc PC07_SEL."]
    #[inline(always)]
    pub const fn pc07_sel(self) -> crate::common::Reg<regs::Pc07Sel, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x9cusize) as _) }
    }
    #[doc = "desc PC08_SEL."]
    #[inline(always)]
    pub const fn pc08_sel(self) -> crate::common::Reg<regs::Pc08Sel, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xa0usize) as _) }
    }
    #[doc = "desc PC09_SEL."]
    #[inline(always)]
    pub const fn pc09_sel(self) -> crate::common::Reg<regs::Pc09Sel, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xa4usize) as _) }
    }
    #[doc = "desc PC10_SEL."]
    #[inline(always)]
    pub const fn pc10_sel(self) -> crate::common::Reg<regs::Pc10Sel, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xa8usize) as _) }
    }
    #[doc = "desc PC11_SEL."]
    #[inline(always)]
    pub const fn pc11_sel(self) -> crate::common::Reg<regs::Pc11Sel, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xacusize) as _) }
    }
    #[doc = "desc PC12_SEL."]
    #[inline(always)]
    pub const fn pc12_sel(self) -> crate::common::Reg<regs::Pc12Sel, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xb0usize) as _) }
    }
    #[doc = "desc PC13_SEL."]
    #[inline(always)]
    pub const fn pc13_sel(self) -> crate::common::Reg<regs::Pc13Sel, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xb4usize) as _) }
    }
    #[doc = "desc PC14_SEL."]
    #[inline(always)]
    pub const fn pc14_sel(self) -> crate::common::Reg<regs::Pc14Sel, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xb8usize) as _) }
    }
    #[doc = "desc PC15_SEL."]
    #[inline(always)]
    pub const fn pc15_sel(self) -> crate::common::Reg<regs::Pc15Sel, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xbcusize) as _) }
    }
    #[doc = "desc PD00_SEL."]
    #[inline(always)]
    pub const fn pd00_sel(self) -> crate::common::Reg<regs::Pd00Sel, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xc0usize) as _) }
    }
    #[doc = "desc PD01_SEL."]
    #[inline(always)]
    pub const fn pd01_sel(self) -> crate::common::Reg<regs::Pd01Sel, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xc4usize) as _) }
    }
    #[doc = "desc PD02_SEL."]
    #[inline(always)]
    pub const fn pd02_sel(self) -> crate::common::Reg<regs::Pd02Sel, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xc8usize) as _) }
    }
    #[doc = "desc PD03_SEL."]
    #[inline(always)]
    pub const fn pd03_sel(self) -> crate::common::Reg<regs::Pd03Sel, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xccusize) as _) }
    }
    #[doc = "desc PD04_SEL."]
    #[inline(always)]
    pub const fn pd04_sel(self) -> crate::common::Reg<regs::Pd04Sel, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xd0usize) as _) }
    }
    #[doc = "desc PD05_SEL."]
    #[inline(always)]
    pub const fn pd05_sel(self) -> crate::common::Reg<regs::Pd05Sel, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xd4usize) as _) }
    }
    #[doc = "desc PD06_SEL."]
    #[inline(always)]
    pub const fn pd06_sel(self) -> crate::common::Reg<regs::Pd06Sel, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xd8usize) as _) }
    }
    #[doc = "desc PD07_SEL."]
    #[inline(always)]
    pub const fn pd07_sel(self) -> crate::common::Reg<regs::Pd07Sel, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xdcusize) as _) }
    }
    #[doc = "desc PADIR."]
    #[inline(always)]
    pub const fn padir(self) -> crate::common::Reg<regs::Padir, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0100usize) as _) }
    }
    #[doc = "desc PAIN."]
    #[inline(always)]
    pub const fn pain(self) -> crate::common::Reg<regs::Pain, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0104usize) as _) }
    }
    #[doc = "desc PAOUT."]
    #[inline(always)]
    pub const fn paout(self) -> crate::common::Reg<regs::Paout, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0108usize) as _) }
    }
    #[doc = "desc PAADS."]
    #[inline(always)]
    pub const fn paads(self) -> crate::common::Reg<regs::Paads, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x010cusize) as _) }
    }
    #[doc = "desc PABSET."]
    #[inline(always)]
    pub const fn pabset(self) -> crate::common::Reg<regs::Pabset, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0110usize) as _) }
    }
    #[doc = "desc PABCLR."]
    #[inline(always)]
    pub const fn pabclr(self) -> crate::common::Reg<regs::Pabclr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0114usize) as _) }
    }
    #[doc = "desc PABSETCLR."]
    #[inline(always)]
    pub const fn pabsetclr(self) -> crate::common::Reg<regs::Pabsetclr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0118usize) as _) }
    }
    #[doc = "desc PADR."]
    #[inline(always)]
    pub const fn padr(self) -> crate::common::Reg<regs::Padr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x011cusize) as _) }
    }
    #[doc = "desc PAPU."]
    #[inline(always)]
    pub const fn papu(self) -> crate::common::Reg<regs::Papu, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0120usize) as _) }
    }
    #[doc = "desc PAPD."]
    #[inline(always)]
    pub const fn papd(self) -> crate::common::Reg<regs::Papd, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0124usize) as _) }
    }
    #[doc = "desc PAOD."]
    #[inline(always)]
    pub const fn paod(self) -> crate::common::Reg<regs::Paod, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x012cusize) as _) }
    }
    #[doc = "desc PAHIE."]
    #[inline(always)]
    pub const fn pahie(self) -> crate::common::Reg<regs::Pahie, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0130usize) as _) }
    }
    #[doc = "desc PALIE."]
    #[inline(always)]
    pub const fn palie(self) -> crate::common::Reg<regs::Palie, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0134usize) as _) }
    }
    #[doc = "desc PARIE."]
    #[inline(always)]
    pub const fn parie(self) -> crate::common::Reg<regs::Parie, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0138usize) as _) }
    }
    #[doc = "desc PAFIE."]
    #[inline(always)]
    pub const fn pafie(self) -> crate::common::Reg<regs::Pafie, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x013cusize) as _) }
    }
    #[doc = "desc PBDIR."]
    #[inline(always)]
    pub const fn pbdir(self) -> crate::common::Reg<regs::Pbdir, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0140usize) as _) }
    }
    #[doc = "desc PBIN."]
    #[inline(always)]
    pub const fn pbin(self) -> crate::common::Reg<regs::Pbin, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0144usize) as _) }
    }
    #[doc = "desc PBOUT."]
    #[inline(always)]
    pub const fn pbout(self) -> crate::common::Reg<regs::Pbout, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0148usize) as _) }
    }
    #[doc = "desc PBADS."]
    #[inline(always)]
    pub const fn pbads(self) -> crate::common::Reg<regs::Pbads, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x014cusize) as _) }
    }
    #[doc = "desc PBBSET."]
    #[inline(always)]
    pub const fn pbbset(self) -> crate::common::Reg<regs::Pbbset, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0150usize) as _) }
    }
    #[doc = "desc PBBCLR."]
    #[inline(always)]
    pub const fn pbbclr(self) -> crate::common::Reg<regs::Pbbclr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0154usize) as _) }
    }
    #[doc = "desc PBBSETCLR."]
    #[inline(always)]
    pub const fn pbbsetclr(self) -> crate::common::Reg<regs::Pbbsetclr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0158usize) as _) }
    }
    #[doc = "desc PBDR."]
    #[inline(always)]
    pub const fn pbdr(self) -> crate::common::Reg<regs::Pbdr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x015cusize) as _) }
    }
    #[doc = "desc PBPU."]
    #[inline(always)]
    pub const fn pbpu(self) -> crate::common::Reg<regs::Pbpu, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0160usize) as _) }
    }
    #[doc = "desc PBPD."]
    #[inline(always)]
    pub const fn pbpd(self) -> crate::common::Reg<regs::Pbpd, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0164usize) as _) }
    }
    #[doc = "desc PBOD."]
    #[inline(always)]
    pub const fn pbod(self) -> crate::common::Reg<regs::Pbod, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x016cusize) as _) }
    }
    #[doc = "desc PBHIE."]
    #[inline(always)]
    pub const fn pbhie(self) -> crate::common::Reg<regs::Pbhie, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0170usize) as _) }
    }
    #[doc = "desc PBLIE."]
    #[inline(always)]
    pub const fn pblie(self) -> crate::common::Reg<regs::Pblie, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0174usize) as _) }
    }
    #[doc = "desc PBRIE."]
    #[inline(always)]
    pub const fn pbrie(self) -> crate::common::Reg<regs::Pbrie, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0178usize) as _) }
    }
    #[doc = "desc PBFIE."]
    #[inline(always)]
    pub const fn pbfie(self) -> crate::common::Reg<regs::Pbfie, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x017cusize) as _) }
    }
    #[doc = "desc PCDIR."]
    #[inline(always)]
    pub const fn pcdir(self) -> crate::common::Reg<regs::Pcdir, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0180usize) as _) }
    }
    #[doc = "desc PCIN."]
    #[inline(always)]
    pub const fn pcin(self) -> crate::common::Reg<regs::Pcin, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0184usize) as _) }
    }
    #[doc = "desc PCOUT."]
    #[inline(always)]
    pub const fn pcout(self) -> crate::common::Reg<regs::Pcout, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0188usize) as _) }
    }
    #[doc = "desc PCADS."]
    #[inline(always)]
    pub const fn pcads(self) -> crate::common::Reg<regs::Pcads, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x018cusize) as _) }
    }
    #[doc = "desc PCBSET."]
    #[inline(always)]
    pub const fn pcbset(self) -> crate::common::Reg<regs::Pcbset, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0190usize) as _) }
    }
    #[doc = "desc PCBCLR."]
    #[inline(always)]
    pub const fn pcbclr(self) -> crate::common::Reg<regs::Pcbclr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0194usize) as _) }
    }
    #[doc = "desc PCBSETCLR."]
    #[inline(always)]
    pub const fn pcbsetclr(self) -> crate::common::Reg<regs::Pcbsetclr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0198usize) as _) }
    }
    #[doc = "desc PCDR."]
    #[inline(always)]
    pub const fn pcdr(self) -> crate::common::Reg<regs::Pcdr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x019cusize) as _) }
    }
    #[doc = "desc PCPU."]
    #[inline(always)]
    pub const fn pcpu(self) -> crate::common::Reg<regs::Pcpu, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x01a0usize) as _) }
    }
    #[doc = "desc PCPD."]
    #[inline(always)]
    pub const fn pcpd(self) -> crate::common::Reg<regs::Pcpd, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x01a4usize) as _) }
    }
    #[doc = "desc PCOD."]
    #[inline(always)]
    pub const fn pcod(self) -> crate::common::Reg<regs::Pcod, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x01acusize) as _) }
    }
    #[doc = "desc PCHIE."]
    #[inline(always)]
    pub const fn pchie(self) -> crate::common::Reg<regs::Pchie, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x01b0usize) as _) }
    }
    #[doc = "desc PCLIE."]
    #[inline(always)]
    pub const fn pclie(self) -> crate::common::Reg<regs::Pclie, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x01b4usize) as _) }
    }
    #[doc = "desc PCRIE."]
    #[inline(always)]
    pub const fn pcrie(self) -> crate::common::Reg<regs::Pcrie, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x01b8usize) as _) }
    }
    #[doc = "desc PCFIE."]
    #[inline(always)]
    pub const fn pcfie(self) -> crate::common::Reg<regs::Pcfie, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x01bcusize) as _) }
    }
    #[doc = "desc PDDIR."]
    #[inline(always)]
    pub const fn pddir(self) -> crate::common::Reg<regs::Pddir, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x01c0usize) as _) }
    }
    #[doc = "desc PDIN."]
    #[inline(always)]
    pub const fn pdin(self) -> crate::common::Reg<regs::Pdin, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x01c4usize) as _) }
    }
    #[doc = "desc PDOUT."]
    #[inline(always)]
    pub const fn pdout(self) -> crate::common::Reg<regs::Pdout, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x01c8usize) as _) }
    }
    #[doc = "desc PDADS."]
    #[inline(always)]
    pub const fn pdads(self) -> crate::common::Reg<regs::Pdads, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x01ccusize) as _) }
    }
    #[doc = "desc PDBSET."]
    #[inline(always)]
    pub const fn pdbset(self) -> crate::common::Reg<regs::Pdbset, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x01d0usize) as _) }
    }
    #[doc = "desc PDBCLR."]
    #[inline(always)]
    pub const fn pdbclr(self) -> crate::common::Reg<regs::Pdbclr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x01d4usize) as _) }
    }
    #[doc = "desc PDBSETCLR."]
    #[inline(always)]
    pub const fn pdbsetclr(self) -> crate::common::Reg<regs::Pdbsetclr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x01d8usize) as _) }
    }
    #[doc = "desc PDDR."]
    #[inline(always)]
    pub const fn pddr(self) -> crate::common::Reg<regs::Pddr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x01dcusize) as _) }
    }
    #[doc = "desc PDPU."]
    #[inline(always)]
    pub const fn pdpu(self) -> crate::common::Reg<regs::Pdpu, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x01e0usize) as _) }
    }
    #[doc = "desc PDPD."]
    #[inline(always)]
    pub const fn pdpd(self) -> crate::common::Reg<regs::Pdpd, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x01e4usize) as _) }
    }
    #[doc = "desc PDOD."]
    #[inline(always)]
    pub const fn pdod(self) -> crate::common::Reg<regs::Pdod, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x01ecusize) as _) }
    }
    #[doc = "desc PDHIE."]
    #[inline(always)]
    pub const fn pdhie(self) -> crate::common::Reg<regs::Pdhie, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x01f0usize) as _) }
    }
    #[doc = "desc PDLIE."]
    #[inline(always)]
    pub const fn pdlie(self) -> crate::common::Reg<regs::Pdlie, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x01f4usize) as _) }
    }
    #[doc = "desc PDRIE."]
    #[inline(always)]
    pub const fn pdrie(self) -> crate::common::Reg<regs::Pdrie, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x01f8usize) as _) }
    }
    #[doc = "desc PDFIE."]
    #[inline(always)]
    pub const fn pdfie(self) -> crate::common::Reg<regs::Pdfie, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x01fcusize) as _) }
    }
    #[doc = "desc PA_STAT."]
    #[inline(always)]
    pub const fn pa_stat(self) -> crate::common::Reg<regs::PaStat, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0200usize) as _) }
    }
    #[doc = "desc PA_ICLR."]
    #[inline(always)]
    pub const fn pa_iclr(self) -> crate::common::Reg<regs::PaIclr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0210usize) as _) }
    }
    #[doc = "desc PB_STAT."]
    #[inline(always)]
    pub const fn pb_stat(self) -> crate::common::Reg<regs::PbStat, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0240usize) as _) }
    }
    #[doc = "desc PB_ICLR."]
    #[inline(always)]
    pub const fn pb_iclr(self) -> crate::common::Reg<regs::PbIclr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0250usize) as _) }
    }
    #[doc = "desc PC_STAT."]
    #[inline(always)]
    pub const fn pc_stat(self) -> crate::common::Reg<regs::PcStat, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0280usize) as _) }
    }
    #[doc = "desc PC_ICLR."]
    #[inline(always)]
    pub const fn pc_iclr(self) -> crate::common::Reg<regs::PcIclr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0290usize) as _) }
    }
    #[doc = "desc PD_STAT."]
    #[inline(always)]
    pub const fn pd_stat(self) -> crate::common::Reg<regs::PdStat, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x02c0usize) as _) }
    }
    #[doc = "desc PD_ICLR."]
    #[inline(always)]
    pub const fn pd_iclr(self) -> crate::common::Reg<regs::PdIclr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x02d0usize) as _) }
    }
    #[doc = "desc CTRL0."]
    #[inline(always)]
    pub const fn ctrl0(self) -> crate::common::Reg<regs::Ctrl0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0300usize) as _) }
    }
    #[doc = "desc CTRL1."]
    #[inline(always)]
    pub const fn ctrl1(self) -> crate::common::Reg<regs::Ctrl1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0304usize) as _) }
    }
    #[doc = "desc CTRL2."]
    #[inline(always)]
    pub const fn ctrl2(self) -> crate::common::Reg<regs::Ctrl2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0308usize) as _) }
    }
    #[doc = "desc TIMGS."]
    #[inline(always)]
    pub const fn timgs(self) -> crate::common::Reg<regs::Timgs, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x030cusize) as _) }
    }
    #[doc = "desc TIMES."]
    #[inline(always)]
    pub const fn times(self) -> crate::common::Reg<regs::Times, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0310usize) as _) }
    }
    #[doc = "desc TIMCPS."]
    #[inline(always)]
    pub const fn timcps(self) -> crate::common::Reg<regs::Timcps, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0314usize) as _) }
    }
    #[doc = "desc PCAS."]
    #[inline(always)]
    pub const fn pcas(self) -> crate::common::Reg<regs::Pcas, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0318usize) as _) }
    }
}
pub mod regs {
    #[doc = "desc CTRL0."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ctrl0(pub u32);
    impl Ctrl0 {
        #[doc = "desc IESEL."]
        #[must_use]
        #[inline(always)]
        pub const fn iesel(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "desc IESEL."]
        #[inline(always)]
        pub const fn set_iesel(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
    }
    impl Default for Ctrl0 {
        #[inline(always)]
        fn default() -> Ctrl0 {
            Ctrl0(0)
        }
    }
    impl core::fmt::Debug for Ctrl0 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Ctrl0")
                .field("iesel", &self.iesel())
                .finish()
        }
    }
    #[doc = "desc CTRL1."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ctrl1(pub u32);
    impl Ctrl1 {
        #[doc = "desc EXT_CLK_SEL."]
        #[must_use]
        #[inline(always)]
        pub const fn ext_clk_sel(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x0f;
            val as u8
        }
        #[doc = "desc EXT_CLK_SEL."]
        #[inline(always)]
        pub const fn set_ext_clk_sel(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
        }
        #[doc = "desc SSN0_SEL."]
        #[must_use]
        #[inline(always)]
        pub const fn ssn0_sel(&self) -> u8 {
            let val = (self.0 >> 4usize) & 0x0f;
            val as u8
        }
        #[doc = "desc SSN0_SEL."]
        #[inline(always)]
        pub const fn set_ssn0_sel(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 4usize)) | (((val as u32) & 0x0f) << 4usize);
        }
        #[doc = "desc PCLK_SEL."]
        #[must_use]
        #[inline(always)]
        pub const fn pclk_sel(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0x03;
            val as u8
        }
        #[doc = "desc PCLK_SEL."]
        #[inline(always)]
        pub const fn set_pclk_sel(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 8usize)) | (((val as u32) & 0x03) << 8usize);
        }
        #[doc = "desc HCLK_SEL."]
        #[must_use]
        #[inline(always)]
        pub const fn hclk_sel(&self) -> u8 {
            let val = (self.0 >> 10usize) & 0x03;
            val as u8
        }
        #[doc = "desc HCLK_SEL."]
        #[inline(always)]
        pub const fn set_hclk_sel(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 10usize)) | (((val as u32) & 0x03) << 10usize);
        }
        #[doc = "desc PCLK_EN."]
        #[must_use]
        #[inline(always)]
        pub const fn pclk_en(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "desc PCLK_EN."]
        #[inline(always)]
        pub const fn set_pclk_en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "desc HCLK_EN."]
        #[must_use]
        #[inline(always)]
        pub const fn hclk_en(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[doc = "desc HCLK_EN."]
        #[inline(always)]
        pub const fn set_hclk_en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[doc = "desc IR_POL."]
        #[must_use]
        #[inline(always)]
        pub const fn ir_pol(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "desc IR_POL."]
        #[inline(always)]
        pub const fn set_ir_pol(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
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
    impl Default for Ctrl1 {
        #[inline(always)]
        fn default() -> Ctrl1 {
            Ctrl1(0)
        }
    }
    impl core::fmt::Debug for Ctrl1 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Ctrl1")
                .field("ext_clk_sel", &self.ext_clk_sel())
                .field("ssn0_sel", &self.ssn0_sel())
                .field("pclk_sel", &self.pclk_sel())
                .field("hclk_sel", &self.hclk_sel())
                .field("pclk_en", &self.pclk_en())
                .field("hclk_en", &self.hclk_en())
                .field("ir_pol", &self.ir_pol())
                .field("rsv", &self.rsv())
                .finish()
        }
    }
    #[doc = "desc CTRL2."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ctrl2(pub u32);
    impl Ctrl2 {
        #[doc = "desc SSN1_SEL."]
        #[must_use]
        #[inline(always)]
        pub const fn ssn1_sel(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x0f;
            val as u8
        }
        #[doc = "desc SSN1_SEL."]
        #[inline(always)]
        pub const fn set_ssn1_sel(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
        }
        #[doc = "desc AHB_SEL."]
        #[must_use]
        #[inline(always)]
        pub const fn ahb_sel(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "desc AHB_SEL."]
        #[inline(always)]
        pub const fn set_ahb_sel(&mut self, val: bool) {
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
    impl Default for Ctrl2 {
        #[inline(always)]
        fn default() -> Ctrl2 {
            Ctrl2(0)
        }
    }
    impl core::fmt::Debug for Ctrl2 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Ctrl2")
                .field("ssn1_sel", &self.ssn1_sel())
                .field("ahb_sel", &self.ahb_sel())
                .field("rsv", &self.rsv())
                .finish()
        }
    }
    #[doc = "desc PA00_SEL."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Pa00Sel(pub u32);
    impl Pa00Sel {
        #[doc = "desc SEL."]
        #[must_use]
        #[inline(always)]
        pub const fn sel(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x07;
            val as u8
        }
        #[doc = "desc SEL."]
        #[inline(always)]
        pub const fn set_sel(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 0usize)) | (((val as u32) & 0x07) << 0usize);
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
    impl Default for Pa00Sel {
        #[inline(always)]
        fn default() -> Pa00Sel {
            Pa00Sel(0)
        }
    }
    impl core::fmt::Debug for Pa00Sel {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Pa00Sel")
                .field("sel", &self.sel())
                .field("rsv", &self.rsv())
                .finish()
        }
    }
    #[doc = "desc PA01_SEL."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Pa01Sel(pub u32);
    impl Pa01Sel {
        #[doc = "desc SEL."]
        #[must_use]
        #[inline(always)]
        pub const fn sel(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x07;
            val as u8
        }
        #[doc = "desc SEL."]
        #[inline(always)]
        pub const fn set_sel(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 0usize)) | (((val as u32) & 0x07) << 0usize);
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
    impl Default for Pa01Sel {
        #[inline(always)]
        fn default() -> Pa01Sel {
            Pa01Sel(0)
        }
    }
    impl core::fmt::Debug for Pa01Sel {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Pa01Sel")
                .field("sel", &self.sel())
                .field("rsv", &self.rsv())
                .finish()
        }
    }
    #[doc = "desc PA02_SEL."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Pa02Sel(pub u32);
    impl Pa02Sel {
        #[doc = "desc SEL."]
        #[must_use]
        #[inline(always)]
        pub const fn sel(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x07;
            val as u8
        }
        #[doc = "desc SEL."]
        #[inline(always)]
        pub const fn set_sel(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 0usize)) | (((val as u32) & 0x07) << 0usize);
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
    impl Default for Pa02Sel {
        #[inline(always)]
        fn default() -> Pa02Sel {
            Pa02Sel(0)
        }
    }
    impl core::fmt::Debug for Pa02Sel {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Pa02Sel")
                .field("sel", &self.sel())
                .field("rsv", &self.rsv())
                .finish()
        }
    }
    #[doc = "desc PA03_SEL."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Pa03Sel(pub u32);
    impl Pa03Sel {
        #[doc = "desc SEL."]
        #[must_use]
        #[inline(always)]
        pub const fn sel(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x07;
            val as u8
        }
        #[doc = "desc SEL."]
        #[inline(always)]
        pub const fn set_sel(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 0usize)) | (((val as u32) & 0x07) << 0usize);
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
    impl Default for Pa03Sel {
        #[inline(always)]
        fn default() -> Pa03Sel {
            Pa03Sel(0)
        }
    }
    impl core::fmt::Debug for Pa03Sel {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Pa03Sel")
                .field("sel", &self.sel())
                .field("rsv", &self.rsv())
                .finish()
        }
    }
    #[doc = "desc PA04_SEL."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Pa04Sel(pub u32);
    impl Pa04Sel {
        #[doc = "desc SEL."]
        #[must_use]
        #[inline(always)]
        pub const fn sel(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x07;
            val as u8
        }
        #[doc = "desc SEL."]
        #[inline(always)]
        pub const fn set_sel(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 0usize)) | (((val as u32) & 0x07) << 0usize);
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
    impl Default for Pa04Sel {
        #[inline(always)]
        fn default() -> Pa04Sel {
            Pa04Sel(0)
        }
    }
    impl core::fmt::Debug for Pa04Sel {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Pa04Sel")
                .field("sel", &self.sel())
                .field("rsv", &self.rsv())
                .finish()
        }
    }
    #[doc = "desc PA05_SEL."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Pa05Sel(pub u32);
    impl Pa05Sel {
        #[doc = "desc SEL."]
        #[must_use]
        #[inline(always)]
        pub const fn sel(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x07;
            val as u8
        }
        #[doc = "desc SEL."]
        #[inline(always)]
        pub const fn set_sel(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 0usize)) | (((val as u32) & 0x07) << 0usize);
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
    impl Default for Pa05Sel {
        #[inline(always)]
        fn default() -> Pa05Sel {
            Pa05Sel(0)
        }
    }
    impl core::fmt::Debug for Pa05Sel {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Pa05Sel")
                .field("sel", &self.sel())
                .field("rsv", &self.rsv())
                .finish()
        }
    }
    #[doc = "desc PA06_SEL."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Pa06Sel(pub u32);
    impl Pa06Sel {
        #[doc = "desc SEL."]
        #[must_use]
        #[inline(always)]
        pub const fn sel(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x07;
            val as u8
        }
        #[doc = "desc SEL."]
        #[inline(always)]
        pub const fn set_sel(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 0usize)) | (((val as u32) & 0x07) << 0usize);
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
    impl Default for Pa06Sel {
        #[inline(always)]
        fn default() -> Pa06Sel {
            Pa06Sel(0)
        }
    }
    impl core::fmt::Debug for Pa06Sel {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Pa06Sel")
                .field("sel", &self.sel())
                .field("rsv", &self.rsv())
                .finish()
        }
    }
    #[doc = "desc PA07_SEL."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Pa07Sel(pub u32);
    impl Pa07Sel {
        #[doc = "desc SEL."]
        #[must_use]
        #[inline(always)]
        pub const fn sel(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x07;
            val as u8
        }
        #[doc = "desc SEL."]
        #[inline(always)]
        pub const fn set_sel(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 0usize)) | (((val as u32) & 0x07) << 0usize);
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
    impl Default for Pa07Sel {
        #[inline(always)]
        fn default() -> Pa07Sel {
            Pa07Sel(0)
        }
    }
    impl core::fmt::Debug for Pa07Sel {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Pa07Sel")
                .field("sel", &self.sel())
                .field("rsv", &self.rsv())
                .finish()
        }
    }
    #[doc = "desc PA08_SEL."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Pa08Sel(pub u32);
    impl Pa08Sel {
        #[doc = "desc SEL."]
        #[must_use]
        #[inline(always)]
        pub const fn sel(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x07;
            val as u8
        }
        #[doc = "desc SEL."]
        #[inline(always)]
        pub const fn set_sel(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 0usize)) | (((val as u32) & 0x07) << 0usize);
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
    impl Default for Pa08Sel {
        #[inline(always)]
        fn default() -> Pa08Sel {
            Pa08Sel(0)
        }
    }
    impl core::fmt::Debug for Pa08Sel {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Pa08Sel")
                .field("sel", &self.sel())
                .field("rsv", &self.rsv())
                .finish()
        }
    }
    #[doc = "desc PA09_SEL."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Pa09Sel(pub u32);
    impl Pa09Sel {
        #[doc = "desc SEL."]
        #[must_use]
        #[inline(always)]
        pub const fn sel(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x07;
            val as u8
        }
        #[doc = "desc SEL."]
        #[inline(always)]
        pub const fn set_sel(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 0usize)) | (((val as u32) & 0x07) << 0usize);
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
    impl Default for Pa09Sel {
        #[inline(always)]
        fn default() -> Pa09Sel {
            Pa09Sel(0)
        }
    }
    impl core::fmt::Debug for Pa09Sel {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Pa09Sel")
                .field("sel", &self.sel())
                .field("rsv", &self.rsv())
                .finish()
        }
    }
    #[doc = "desc PA10_SEL."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Pa10Sel(pub u32);
    impl Pa10Sel {
        #[doc = "desc SEL."]
        #[must_use]
        #[inline(always)]
        pub const fn sel(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x07;
            val as u8
        }
        #[doc = "desc SEL."]
        #[inline(always)]
        pub const fn set_sel(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 0usize)) | (((val as u32) & 0x07) << 0usize);
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
    impl Default for Pa10Sel {
        #[inline(always)]
        fn default() -> Pa10Sel {
            Pa10Sel(0)
        }
    }
    impl core::fmt::Debug for Pa10Sel {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Pa10Sel")
                .field("sel", &self.sel())
                .field("rsv", &self.rsv())
                .finish()
        }
    }
    #[doc = "desc PA11_SEL."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Pa11Sel(pub u32);
    impl Pa11Sel {
        #[doc = "desc SEL."]
        #[must_use]
        #[inline(always)]
        pub const fn sel(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x07;
            val as u8
        }
        #[doc = "desc SEL."]
        #[inline(always)]
        pub const fn set_sel(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 0usize)) | (((val as u32) & 0x07) << 0usize);
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
    impl Default for Pa11Sel {
        #[inline(always)]
        fn default() -> Pa11Sel {
            Pa11Sel(0)
        }
    }
    impl core::fmt::Debug for Pa11Sel {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Pa11Sel")
                .field("sel", &self.sel())
                .field("rsv", &self.rsv())
                .finish()
        }
    }
    #[doc = "desc PA12_SEL."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Pa12Sel(pub u32);
    impl Pa12Sel {
        #[doc = "desc SEL."]
        #[must_use]
        #[inline(always)]
        pub const fn sel(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x07;
            val as u8
        }
        #[doc = "desc SEL."]
        #[inline(always)]
        pub const fn set_sel(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 0usize)) | (((val as u32) & 0x07) << 0usize);
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
    impl Default for Pa12Sel {
        #[inline(always)]
        fn default() -> Pa12Sel {
            Pa12Sel(0)
        }
    }
    impl core::fmt::Debug for Pa12Sel {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Pa12Sel")
                .field("sel", &self.sel())
                .field("rsv", &self.rsv())
                .finish()
        }
    }
    #[doc = "desc PA13_SEL."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Pa13Sel(pub u32);
    impl Pa13Sel {
        #[doc = "desc SEL."]
        #[must_use]
        #[inline(always)]
        pub const fn sel(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x07;
            val as u8
        }
        #[doc = "desc SEL."]
        #[inline(always)]
        pub const fn set_sel(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 0usize)) | (((val as u32) & 0x07) << 0usize);
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
    impl Default for Pa13Sel {
        #[inline(always)]
        fn default() -> Pa13Sel {
            Pa13Sel(0)
        }
    }
    impl core::fmt::Debug for Pa13Sel {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Pa13Sel")
                .field("sel", &self.sel())
                .field("rsv", &self.rsv())
                .finish()
        }
    }
    #[doc = "desc PA14_SEL."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Pa14Sel(pub u32);
    impl Pa14Sel {
        #[doc = "desc SEL."]
        #[must_use]
        #[inline(always)]
        pub const fn sel(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x07;
            val as u8
        }
        #[doc = "desc SEL."]
        #[inline(always)]
        pub const fn set_sel(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 0usize)) | (((val as u32) & 0x07) << 0usize);
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
    impl Default for Pa14Sel {
        #[inline(always)]
        fn default() -> Pa14Sel {
            Pa14Sel(0)
        }
    }
    impl core::fmt::Debug for Pa14Sel {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Pa14Sel")
                .field("sel", &self.sel())
                .field("rsv", &self.rsv())
                .finish()
        }
    }
    #[doc = "desc PA15_SEL."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Pa15Sel(pub u32);
    impl Pa15Sel {
        #[doc = "desc SEL."]
        #[must_use]
        #[inline(always)]
        pub const fn sel(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x07;
            val as u8
        }
        #[doc = "desc SEL."]
        #[inline(always)]
        pub const fn set_sel(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 0usize)) | (((val as u32) & 0x07) << 0usize);
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
    impl Default for Pa15Sel {
        #[inline(always)]
        fn default() -> Pa15Sel {
            Pa15Sel(0)
        }
    }
    impl core::fmt::Debug for Pa15Sel {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Pa15Sel")
                .field("sel", &self.sel())
                .field("rsv", &self.rsv())
                .finish()
        }
    }
    #[doc = "desc PA_ICLR."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct PaIclr(pub u32);
    impl PaIclr {
        #[doc = "desc PA00."]
        #[must_use]
        #[inline(always)]
        pub const fn pa00(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "desc PA00."]
        #[inline(always)]
        pub const fn set_pa00(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "desc PA01."]
        #[must_use]
        #[inline(always)]
        pub const fn pa01(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "desc PA01."]
        #[inline(always)]
        pub const fn set_pa01(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "desc PA02."]
        #[must_use]
        #[inline(always)]
        pub const fn pa02(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "desc PA02."]
        #[inline(always)]
        pub const fn set_pa02(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "desc PA03."]
        #[must_use]
        #[inline(always)]
        pub const fn pa03(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "desc PA03."]
        #[inline(always)]
        pub const fn set_pa03(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "desc PA04."]
        #[must_use]
        #[inline(always)]
        pub const fn pa04(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "desc PA04."]
        #[inline(always)]
        pub const fn set_pa04(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "desc PA05."]
        #[must_use]
        #[inline(always)]
        pub const fn pa05(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "desc PA05."]
        #[inline(always)]
        pub const fn set_pa05(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "desc PA06."]
        #[must_use]
        #[inline(always)]
        pub const fn pa06(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "desc PA06."]
        #[inline(always)]
        pub const fn set_pa06(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "desc PA07."]
        #[must_use]
        #[inline(always)]
        pub const fn pa07(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "desc PA07."]
        #[inline(always)]
        pub const fn set_pa07(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "desc PA08."]
        #[must_use]
        #[inline(always)]
        pub const fn pa08(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "desc PA08."]
        #[inline(always)]
        pub const fn set_pa08(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "desc PA09."]
        #[must_use]
        #[inline(always)]
        pub const fn pa09(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "desc PA09."]
        #[inline(always)]
        pub const fn set_pa09(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "desc PA10."]
        #[must_use]
        #[inline(always)]
        pub const fn pa10(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "desc PA10."]
        #[inline(always)]
        pub const fn set_pa10(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[doc = "desc PA11."]
        #[must_use]
        #[inline(always)]
        pub const fn pa11(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "desc PA11."]
        #[inline(always)]
        pub const fn set_pa11(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[doc = "desc PA12."]
        #[must_use]
        #[inline(always)]
        pub const fn pa12(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "desc PA12."]
        #[inline(always)]
        pub const fn set_pa12(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "desc PA13."]
        #[must_use]
        #[inline(always)]
        pub const fn pa13(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[doc = "desc PA13."]
        #[inline(always)]
        pub const fn set_pa13(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[doc = "desc PA14."]
        #[must_use]
        #[inline(always)]
        pub const fn pa14(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "desc PA14."]
        #[inline(always)]
        pub const fn set_pa14(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[doc = "desc PA15."]
        #[must_use]
        #[inline(always)]
        pub const fn pa15(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "desc PA15."]
        #[inline(always)]
        pub const fn set_pa15(&mut self, val: bool) {
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
    impl Default for PaIclr {
        #[inline(always)]
        fn default() -> PaIclr {
            PaIclr(0)
        }
    }
    impl core::fmt::Debug for PaIclr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("PaIclr")
                .field("pa00", &self.pa00())
                .field("pa01", &self.pa01())
                .field("pa02", &self.pa02())
                .field("pa03", &self.pa03())
                .field("pa04", &self.pa04())
                .field("pa05", &self.pa05())
                .field("pa06", &self.pa06())
                .field("pa07", &self.pa07())
                .field("pa08", &self.pa08())
                .field("pa09", &self.pa09())
                .field("pa10", &self.pa10())
                .field("pa11", &self.pa11())
                .field("pa12", &self.pa12())
                .field("pa13", &self.pa13())
                .field("pa14", &self.pa14())
                .field("pa15", &self.pa15())
                .field("rsv", &self.rsv())
                .finish()
        }
    }
    #[doc = "desc PA_STAT."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct PaStat(pub u32);
    impl PaStat {
        #[doc = "desc PA00."]
        #[must_use]
        #[inline(always)]
        pub const fn pa00(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "desc PA00."]
        #[inline(always)]
        pub const fn set_pa00(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "desc PA01."]
        #[must_use]
        #[inline(always)]
        pub const fn pa01(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "desc PA01."]
        #[inline(always)]
        pub const fn set_pa01(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "desc PA02."]
        #[must_use]
        #[inline(always)]
        pub const fn pa02(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "desc PA02."]
        #[inline(always)]
        pub const fn set_pa02(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "desc PA03."]
        #[must_use]
        #[inline(always)]
        pub const fn pa03(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "desc PA03."]
        #[inline(always)]
        pub const fn set_pa03(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "desc PA04."]
        #[must_use]
        #[inline(always)]
        pub const fn pa04(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "desc PA04."]
        #[inline(always)]
        pub const fn set_pa04(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "desc PA05."]
        #[must_use]
        #[inline(always)]
        pub const fn pa05(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "desc PA05."]
        #[inline(always)]
        pub const fn set_pa05(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "desc PA06."]
        #[must_use]
        #[inline(always)]
        pub const fn pa06(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "desc PA06."]
        #[inline(always)]
        pub const fn set_pa06(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "desc PA07."]
        #[must_use]
        #[inline(always)]
        pub const fn pa07(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "desc PA07."]
        #[inline(always)]
        pub const fn set_pa07(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "desc PA08."]
        #[must_use]
        #[inline(always)]
        pub const fn pa08(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "desc PA08."]
        #[inline(always)]
        pub const fn set_pa08(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "desc PA09."]
        #[must_use]
        #[inline(always)]
        pub const fn pa09(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "desc PA09."]
        #[inline(always)]
        pub const fn set_pa09(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "desc PA10."]
        #[must_use]
        #[inline(always)]
        pub const fn pa10(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "desc PA10."]
        #[inline(always)]
        pub const fn set_pa10(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[doc = "desc PA11."]
        #[must_use]
        #[inline(always)]
        pub const fn pa11(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "desc PA11."]
        #[inline(always)]
        pub const fn set_pa11(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[doc = "desc PA12."]
        #[must_use]
        #[inline(always)]
        pub const fn pa12(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "desc PA12."]
        #[inline(always)]
        pub const fn set_pa12(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "desc PA13."]
        #[must_use]
        #[inline(always)]
        pub const fn pa13(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[doc = "desc PA13."]
        #[inline(always)]
        pub const fn set_pa13(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[doc = "desc PA14."]
        #[must_use]
        #[inline(always)]
        pub const fn pa14(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "desc PA14."]
        #[inline(always)]
        pub const fn set_pa14(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[doc = "desc PA15."]
        #[must_use]
        #[inline(always)]
        pub const fn pa15(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "desc PA15."]
        #[inline(always)]
        pub const fn set_pa15(&mut self, val: bool) {
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
    impl Default for PaStat {
        #[inline(always)]
        fn default() -> PaStat {
            PaStat(0)
        }
    }
    impl core::fmt::Debug for PaStat {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("PaStat")
                .field("pa00", &self.pa00())
                .field("pa01", &self.pa01())
                .field("pa02", &self.pa02())
                .field("pa03", &self.pa03())
                .field("pa04", &self.pa04())
                .field("pa05", &self.pa05())
                .field("pa06", &self.pa06())
                .field("pa07", &self.pa07())
                .field("pa08", &self.pa08())
                .field("pa09", &self.pa09())
                .field("pa10", &self.pa10())
                .field("pa11", &self.pa11())
                .field("pa12", &self.pa12())
                .field("pa13", &self.pa13())
                .field("pa14", &self.pa14())
                .field("pa15", &self.pa15())
                .field("rsv", &self.rsv())
                .finish()
        }
    }
    #[doc = "desc PAADS."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Paads(pub u32);
    impl Paads {
        #[doc = "desc PA00."]
        #[must_use]
        #[inline(always)]
        pub const fn pa00(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "desc PA00."]
        #[inline(always)]
        pub const fn set_pa00(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "desc PA01."]
        #[must_use]
        #[inline(always)]
        pub const fn pa01(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "desc PA01."]
        #[inline(always)]
        pub const fn set_pa01(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "desc PA02."]
        #[must_use]
        #[inline(always)]
        pub const fn pa02(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "desc PA02."]
        #[inline(always)]
        pub const fn set_pa02(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "desc PA03."]
        #[must_use]
        #[inline(always)]
        pub const fn pa03(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "desc PA03."]
        #[inline(always)]
        pub const fn set_pa03(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "desc PA04."]
        #[must_use]
        #[inline(always)]
        pub const fn pa04(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "desc PA04."]
        #[inline(always)]
        pub const fn set_pa04(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "desc PA05."]
        #[must_use]
        #[inline(always)]
        pub const fn pa05(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "desc PA05."]
        #[inline(always)]
        pub const fn set_pa05(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "desc PA06."]
        #[must_use]
        #[inline(always)]
        pub const fn pa06(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "desc PA06."]
        #[inline(always)]
        pub const fn set_pa06(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "desc PA07."]
        #[must_use]
        #[inline(always)]
        pub const fn pa07(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "desc PA07."]
        #[inline(always)]
        pub const fn set_pa07(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "desc PA08."]
        #[must_use]
        #[inline(always)]
        pub const fn pa08(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "desc PA08."]
        #[inline(always)]
        pub const fn set_pa08(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "desc PA09."]
        #[must_use]
        #[inline(always)]
        pub const fn pa09(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "desc PA09."]
        #[inline(always)]
        pub const fn set_pa09(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "desc PA10."]
        #[must_use]
        #[inline(always)]
        pub const fn pa10(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "desc PA10."]
        #[inline(always)]
        pub const fn set_pa10(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[doc = "desc PA11."]
        #[must_use]
        #[inline(always)]
        pub const fn pa11(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "desc PA11."]
        #[inline(always)]
        pub const fn set_pa11(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[doc = "desc PA12."]
        #[must_use]
        #[inline(always)]
        pub const fn pa12(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "desc PA12."]
        #[inline(always)]
        pub const fn set_pa12(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "desc PA13."]
        #[must_use]
        #[inline(always)]
        pub const fn pa13(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[doc = "desc PA13."]
        #[inline(always)]
        pub const fn set_pa13(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[doc = "desc PA14."]
        #[must_use]
        #[inline(always)]
        pub const fn pa14(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "desc PA14."]
        #[inline(always)]
        pub const fn set_pa14(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[doc = "desc PA15."]
        #[must_use]
        #[inline(always)]
        pub const fn pa15(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "desc PA15."]
        #[inline(always)]
        pub const fn set_pa15(&mut self, val: bool) {
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
    impl Default for Paads {
        #[inline(always)]
        fn default() -> Paads {
            Paads(0)
        }
    }
    impl core::fmt::Debug for Paads {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Paads")
                .field("pa00", &self.pa00())
                .field("pa01", &self.pa01())
                .field("pa02", &self.pa02())
                .field("pa03", &self.pa03())
                .field("pa04", &self.pa04())
                .field("pa05", &self.pa05())
                .field("pa06", &self.pa06())
                .field("pa07", &self.pa07())
                .field("pa08", &self.pa08())
                .field("pa09", &self.pa09())
                .field("pa10", &self.pa10())
                .field("pa11", &self.pa11())
                .field("pa12", &self.pa12())
                .field("pa13", &self.pa13())
                .field("pa14", &self.pa14())
                .field("pa15", &self.pa15())
                .field("rsv", &self.rsv())
                .finish()
        }
    }
    #[doc = "desc PABCLR."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Pabclr(pub u32);
    impl Pabclr {
        #[doc = "desc PA00."]
        #[must_use]
        #[inline(always)]
        pub const fn pa00(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "desc PA00."]
        #[inline(always)]
        pub const fn set_pa00(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "desc PA01."]
        #[must_use]
        #[inline(always)]
        pub const fn pa01(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "desc PA01."]
        #[inline(always)]
        pub const fn set_pa01(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "desc PA02."]
        #[must_use]
        #[inline(always)]
        pub const fn pa02(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "desc PA02."]
        #[inline(always)]
        pub const fn set_pa02(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "desc PA03."]
        #[must_use]
        #[inline(always)]
        pub const fn pa03(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "desc PA03."]
        #[inline(always)]
        pub const fn set_pa03(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "desc PA04."]
        #[must_use]
        #[inline(always)]
        pub const fn pa04(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "desc PA04."]
        #[inline(always)]
        pub const fn set_pa04(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "desc PA05."]
        #[must_use]
        #[inline(always)]
        pub const fn pa05(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "desc PA05."]
        #[inline(always)]
        pub const fn set_pa05(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "desc PA06."]
        #[must_use]
        #[inline(always)]
        pub const fn pa06(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "desc PA06."]
        #[inline(always)]
        pub const fn set_pa06(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "desc PA07."]
        #[must_use]
        #[inline(always)]
        pub const fn pa07(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "desc PA07."]
        #[inline(always)]
        pub const fn set_pa07(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "desc PA08."]
        #[must_use]
        #[inline(always)]
        pub const fn pa08(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "desc PA08."]
        #[inline(always)]
        pub const fn set_pa08(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "desc PA09."]
        #[must_use]
        #[inline(always)]
        pub const fn pa09(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "desc PA09."]
        #[inline(always)]
        pub const fn set_pa09(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "desc PA10."]
        #[must_use]
        #[inline(always)]
        pub const fn pa10(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "desc PA10."]
        #[inline(always)]
        pub const fn set_pa10(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[doc = "desc PA11."]
        #[must_use]
        #[inline(always)]
        pub const fn pa11(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "desc PA11."]
        #[inline(always)]
        pub const fn set_pa11(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[doc = "desc PA12."]
        #[must_use]
        #[inline(always)]
        pub const fn pa12(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "desc PA12."]
        #[inline(always)]
        pub const fn set_pa12(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "desc PA13."]
        #[must_use]
        #[inline(always)]
        pub const fn pa13(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[doc = "desc PA13."]
        #[inline(always)]
        pub const fn set_pa13(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[doc = "desc PA14."]
        #[must_use]
        #[inline(always)]
        pub const fn pa14(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "desc PA14."]
        #[inline(always)]
        pub const fn set_pa14(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[doc = "desc PA15."]
        #[must_use]
        #[inline(always)]
        pub const fn pa15(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "desc PA15."]
        #[inline(always)]
        pub const fn set_pa15(&mut self, val: bool) {
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
    impl Default for Pabclr {
        #[inline(always)]
        fn default() -> Pabclr {
            Pabclr(0)
        }
    }
    impl core::fmt::Debug for Pabclr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Pabclr")
                .field("pa00", &self.pa00())
                .field("pa01", &self.pa01())
                .field("pa02", &self.pa02())
                .field("pa03", &self.pa03())
                .field("pa04", &self.pa04())
                .field("pa05", &self.pa05())
                .field("pa06", &self.pa06())
                .field("pa07", &self.pa07())
                .field("pa08", &self.pa08())
                .field("pa09", &self.pa09())
                .field("pa10", &self.pa10())
                .field("pa11", &self.pa11())
                .field("pa12", &self.pa12())
                .field("pa13", &self.pa13())
                .field("pa14", &self.pa14())
                .field("pa15", &self.pa15())
                .field("rsv", &self.rsv())
                .finish()
        }
    }
    #[doc = "desc PABSET."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Pabset(pub u32);
    impl Pabset {
        #[doc = "desc PA00."]
        #[must_use]
        #[inline(always)]
        pub const fn pa00(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "desc PA00."]
        #[inline(always)]
        pub const fn set_pa00(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "desc PA01."]
        #[must_use]
        #[inline(always)]
        pub const fn pa01(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "desc PA01."]
        #[inline(always)]
        pub const fn set_pa01(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "desc PA02."]
        #[must_use]
        #[inline(always)]
        pub const fn pa02(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "desc PA02."]
        #[inline(always)]
        pub const fn set_pa02(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "desc PA03."]
        #[must_use]
        #[inline(always)]
        pub const fn pa03(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "desc PA03."]
        #[inline(always)]
        pub const fn set_pa03(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "desc PA04."]
        #[must_use]
        #[inline(always)]
        pub const fn pa04(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "desc PA04."]
        #[inline(always)]
        pub const fn set_pa04(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "desc PA05."]
        #[must_use]
        #[inline(always)]
        pub const fn pa05(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "desc PA05."]
        #[inline(always)]
        pub const fn set_pa05(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "desc PA06."]
        #[must_use]
        #[inline(always)]
        pub const fn pa06(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "desc PA06."]
        #[inline(always)]
        pub const fn set_pa06(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "desc PA07."]
        #[must_use]
        #[inline(always)]
        pub const fn pa07(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "desc PA07."]
        #[inline(always)]
        pub const fn set_pa07(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "desc PA08."]
        #[must_use]
        #[inline(always)]
        pub const fn pa08(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "desc PA08."]
        #[inline(always)]
        pub const fn set_pa08(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "desc PA09."]
        #[must_use]
        #[inline(always)]
        pub const fn pa09(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "desc PA09."]
        #[inline(always)]
        pub const fn set_pa09(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "desc PA10."]
        #[must_use]
        #[inline(always)]
        pub const fn pa10(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "desc PA10."]
        #[inline(always)]
        pub const fn set_pa10(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[doc = "desc PA11."]
        #[must_use]
        #[inline(always)]
        pub const fn pa11(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "desc PA11."]
        #[inline(always)]
        pub const fn set_pa11(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[doc = "desc PA12."]
        #[must_use]
        #[inline(always)]
        pub const fn pa12(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "desc PA12."]
        #[inline(always)]
        pub const fn set_pa12(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "desc PA13."]
        #[must_use]
        #[inline(always)]
        pub const fn pa13(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[doc = "desc PA13."]
        #[inline(always)]
        pub const fn set_pa13(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[doc = "desc PA14."]
        #[must_use]
        #[inline(always)]
        pub const fn pa14(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "desc PA14."]
        #[inline(always)]
        pub const fn set_pa14(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[doc = "desc PA15."]
        #[must_use]
        #[inline(always)]
        pub const fn pa15(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "desc PA15."]
        #[inline(always)]
        pub const fn set_pa15(&mut self, val: bool) {
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
    impl Default for Pabset {
        #[inline(always)]
        fn default() -> Pabset {
            Pabset(0)
        }
    }
    impl core::fmt::Debug for Pabset {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Pabset")
                .field("pa00", &self.pa00())
                .field("pa01", &self.pa01())
                .field("pa02", &self.pa02())
                .field("pa03", &self.pa03())
                .field("pa04", &self.pa04())
                .field("pa05", &self.pa05())
                .field("pa06", &self.pa06())
                .field("pa07", &self.pa07())
                .field("pa08", &self.pa08())
                .field("pa09", &self.pa09())
                .field("pa10", &self.pa10())
                .field("pa11", &self.pa11())
                .field("pa12", &self.pa12())
                .field("pa13", &self.pa13())
                .field("pa14", &self.pa14())
                .field("pa15", &self.pa15())
                .field("rsv", &self.rsv())
                .finish()
        }
    }
    #[doc = "desc PABSETCLR."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Pabsetclr(pub u32);
    impl Pabsetclr {
        #[doc = "desc PABCLR."]
        #[must_use]
        #[inline(always)]
        pub const fn pabclr(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[doc = "desc PABCLR."]
        #[inline(always)]
        pub const fn set_pabclr(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
        }
        #[doc = "desc PABSET."]
        #[must_use]
        #[inline(always)]
        pub const fn pabset(&self) -> u16 {
            let val = (self.0 >> 16usize) & 0xffff;
            val as u16
        }
        #[doc = "desc PABSET."]
        #[inline(always)]
        pub const fn set_pabset(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 16usize)) | (((val as u32) & 0xffff) << 16usize);
        }
    }
    impl Default for Pabsetclr {
        #[inline(always)]
        fn default() -> Pabsetclr {
            Pabsetclr(0)
        }
    }
    impl core::fmt::Debug for Pabsetclr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Pabsetclr")
                .field("pabclr", &self.pabclr())
                .field("pabset", &self.pabset())
                .finish()
        }
    }
    #[doc = "desc PADIR."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Padir(pub u32);
    impl Padir {
        #[doc = "desc PA00."]
        #[must_use]
        #[inline(always)]
        pub const fn pa00(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "desc PA00."]
        #[inline(always)]
        pub const fn set_pa00(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "desc PA01."]
        #[must_use]
        #[inline(always)]
        pub const fn pa01(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "desc PA01."]
        #[inline(always)]
        pub const fn set_pa01(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "desc PA02."]
        #[must_use]
        #[inline(always)]
        pub const fn pa02(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "desc PA02."]
        #[inline(always)]
        pub const fn set_pa02(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "desc PA03."]
        #[must_use]
        #[inline(always)]
        pub const fn pa03(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "desc PA03."]
        #[inline(always)]
        pub const fn set_pa03(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "desc PA04."]
        #[must_use]
        #[inline(always)]
        pub const fn pa04(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "desc PA04."]
        #[inline(always)]
        pub const fn set_pa04(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "desc PA05."]
        #[must_use]
        #[inline(always)]
        pub const fn pa05(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "desc PA05."]
        #[inline(always)]
        pub const fn set_pa05(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "desc PA06."]
        #[must_use]
        #[inline(always)]
        pub const fn pa06(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "desc PA06."]
        #[inline(always)]
        pub const fn set_pa06(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "desc PA07."]
        #[must_use]
        #[inline(always)]
        pub const fn pa07(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "desc PA07."]
        #[inline(always)]
        pub const fn set_pa07(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "desc PA08."]
        #[must_use]
        #[inline(always)]
        pub const fn pa08(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "desc PA08."]
        #[inline(always)]
        pub const fn set_pa08(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "desc PA09."]
        #[must_use]
        #[inline(always)]
        pub const fn pa09(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "desc PA09."]
        #[inline(always)]
        pub const fn set_pa09(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "desc PA10."]
        #[must_use]
        #[inline(always)]
        pub const fn pa10(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "desc PA10."]
        #[inline(always)]
        pub const fn set_pa10(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[doc = "desc PA11."]
        #[must_use]
        #[inline(always)]
        pub const fn pa11(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "desc PA11."]
        #[inline(always)]
        pub const fn set_pa11(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[doc = "desc PA12."]
        #[must_use]
        #[inline(always)]
        pub const fn pa12(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "desc PA12."]
        #[inline(always)]
        pub const fn set_pa12(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "desc PA13."]
        #[must_use]
        #[inline(always)]
        pub const fn pa13(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[doc = "desc PA13."]
        #[inline(always)]
        pub const fn set_pa13(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[doc = "desc PA14."]
        #[must_use]
        #[inline(always)]
        pub const fn pa14(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "desc PA14."]
        #[inline(always)]
        pub const fn set_pa14(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[doc = "desc PA15."]
        #[must_use]
        #[inline(always)]
        pub const fn pa15(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "desc PA15."]
        #[inline(always)]
        pub const fn set_pa15(&mut self, val: bool) {
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
    impl Default for Padir {
        #[inline(always)]
        fn default() -> Padir {
            Padir(0)
        }
    }
    impl core::fmt::Debug for Padir {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Padir")
                .field("pa00", &self.pa00())
                .field("pa01", &self.pa01())
                .field("pa02", &self.pa02())
                .field("pa03", &self.pa03())
                .field("pa04", &self.pa04())
                .field("pa05", &self.pa05())
                .field("pa06", &self.pa06())
                .field("pa07", &self.pa07())
                .field("pa08", &self.pa08())
                .field("pa09", &self.pa09())
                .field("pa10", &self.pa10())
                .field("pa11", &self.pa11())
                .field("pa12", &self.pa12())
                .field("pa13", &self.pa13())
                .field("pa14", &self.pa14())
                .field("pa15", &self.pa15())
                .field("rsv", &self.rsv())
                .finish()
        }
    }
    #[doc = "desc PADR."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Padr(pub u32);
    impl Padr {
        #[doc = "desc PA00."]
        #[must_use]
        #[inline(always)]
        pub const fn pa00(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "desc PA00."]
        #[inline(always)]
        pub const fn set_pa00(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "desc PA01."]
        #[must_use]
        #[inline(always)]
        pub const fn pa01(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "desc PA01."]
        #[inline(always)]
        pub const fn set_pa01(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "desc PA02."]
        #[must_use]
        #[inline(always)]
        pub const fn pa02(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "desc PA02."]
        #[inline(always)]
        pub const fn set_pa02(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "desc PA03."]
        #[must_use]
        #[inline(always)]
        pub const fn pa03(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "desc PA03."]
        #[inline(always)]
        pub const fn set_pa03(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "desc PA04."]
        #[must_use]
        #[inline(always)]
        pub const fn pa04(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "desc PA04."]
        #[inline(always)]
        pub const fn set_pa04(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "desc PA05."]
        #[must_use]
        #[inline(always)]
        pub const fn pa05(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "desc PA05."]
        #[inline(always)]
        pub const fn set_pa05(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "desc PA06."]
        #[must_use]
        #[inline(always)]
        pub const fn pa06(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "desc PA06."]
        #[inline(always)]
        pub const fn set_pa06(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "desc PA07."]
        #[must_use]
        #[inline(always)]
        pub const fn pa07(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "desc PA07."]
        #[inline(always)]
        pub const fn set_pa07(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "desc PA08."]
        #[must_use]
        #[inline(always)]
        pub const fn pa08(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "desc PA08."]
        #[inline(always)]
        pub const fn set_pa08(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "desc PA09."]
        #[must_use]
        #[inline(always)]
        pub const fn pa09(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "desc PA09."]
        #[inline(always)]
        pub const fn set_pa09(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "desc PA10."]
        #[must_use]
        #[inline(always)]
        pub const fn pa10(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "desc PA10."]
        #[inline(always)]
        pub const fn set_pa10(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[doc = "desc PA11."]
        #[must_use]
        #[inline(always)]
        pub const fn pa11(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "desc PA11."]
        #[inline(always)]
        pub const fn set_pa11(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[doc = "desc PA12."]
        #[must_use]
        #[inline(always)]
        pub const fn pa12(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "desc PA12."]
        #[inline(always)]
        pub const fn set_pa12(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "desc PA13."]
        #[must_use]
        #[inline(always)]
        pub const fn pa13(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[doc = "desc PA13."]
        #[inline(always)]
        pub const fn set_pa13(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[doc = "desc PA14."]
        #[must_use]
        #[inline(always)]
        pub const fn pa14(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "desc PA14."]
        #[inline(always)]
        pub const fn set_pa14(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[doc = "desc PA15."]
        #[must_use]
        #[inline(always)]
        pub const fn pa15(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "desc PA15."]
        #[inline(always)]
        pub const fn set_pa15(&mut self, val: bool) {
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
    impl Default for Padr {
        #[inline(always)]
        fn default() -> Padr {
            Padr(0)
        }
    }
    impl core::fmt::Debug for Padr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Padr")
                .field("pa00", &self.pa00())
                .field("pa01", &self.pa01())
                .field("pa02", &self.pa02())
                .field("pa03", &self.pa03())
                .field("pa04", &self.pa04())
                .field("pa05", &self.pa05())
                .field("pa06", &self.pa06())
                .field("pa07", &self.pa07())
                .field("pa08", &self.pa08())
                .field("pa09", &self.pa09())
                .field("pa10", &self.pa10())
                .field("pa11", &self.pa11())
                .field("pa12", &self.pa12())
                .field("pa13", &self.pa13())
                .field("pa14", &self.pa14())
                .field("pa15", &self.pa15())
                .field("rsv", &self.rsv())
                .finish()
        }
    }
    #[doc = "desc PAFIE."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Pafie(pub u32);
    impl Pafie {
        #[doc = "desc PA00."]
        #[must_use]
        #[inline(always)]
        pub const fn pa00(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "desc PA00."]
        #[inline(always)]
        pub const fn set_pa00(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "desc PA01."]
        #[must_use]
        #[inline(always)]
        pub const fn pa01(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "desc PA01."]
        #[inline(always)]
        pub const fn set_pa01(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "desc PA02."]
        #[must_use]
        #[inline(always)]
        pub const fn pa02(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "desc PA02."]
        #[inline(always)]
        pub const fn set_pa02(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "desc PA03."]
        #[must_use]
        #[inline(always)]
        pub const fn pa03(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "desc PA03."]
        #[inline(always)]
        pub const fn set_pa03(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "desc PA04."]
        #[must_use]
        #[inline(always)]
        pub const fn pa04(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "desc PA04."]
        #[inline(always)]
        pub const fn set_pa04(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "desc PA05."]
        #[must_use]
        #[inline(always)]
        pub const fn pa05(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "desc PA05."]
        #[inline(always)]
        pub const fn set_pa05(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "desc PA06."]
        #[must_use]
        #[inline(always)]
        pub const fn pa06(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "desc PA06."]
        #[inline(always)]
        pub const fn set_pa06(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "desc PA07."]
        #[must_use]
        #[inline(always)]
        pub const fn pa07(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "desc PA07."]
        #[inline(always)]
        pub const fn set_pa07(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "desc PA08."]
        #[must_use]
        #[inline(always)]
        pub const fn pa08(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "desc PA08."]
        #[inline(always)]
        pub const fn set_pa08(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "desc PA09."]
        #[must_use]
        #[inline(always)]
        pub const fn pa09(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "desc PA09."]
        #[inline(always)]
        pub const fn set_pa09(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "desc PA10."]
        #[must_use]
        #[inline(always)]
        pub const fn pa10(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "desc PA10."]
        #[inline(always)]
        pub const fn set_pa10(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[doc = "desc PA11."]
        #[must_use]
        #[inline(always)]
        pub const fn pa11(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "desc PA11."]
        #[inline(always)]
        pub const fn set_pa11(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[doc = "desc PA12."]
        #[must_use]
        #[inline(always)]
        pub const fn pa12(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "desc PA12."]
        #[inline(always)]
        pub const fn set_pa12(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "desc PA13."]
        #[must_use]
        #[inline(always)]
        pub const fn pa13(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[doc = "desc PA13."]
        #[inline(always)]
        pub const fn set_pa13(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[doc = "desc PA14."]
        #[must_use]
        #[inline(always)]
        pub const fn pa14(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "desc PA14."]
        #[inline(always)]
        pub const fn set_pa14(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[doc = "desc PA15."]
        #[must_use]
        #[inline(always)]
        pub const fn pa15(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "desc PA15."]
        #[inline(always)]
        pub const fn set_pa15(&mut self, val: bool) {
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
    impl Default for Pafie {
        #[inline(always)]
        fn default() -> Pafie {
            Pafie(0)
        }
    }
    impl core::fmt::Debug for Pafie {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Pafie")
                .field("pa00", &self.pa00())
                .field("pa01", &self.pa01())
                .field("pa02", &self.pa02())
                .field("pa03", &self.pa03())
                .field("pa04", &self.pa04())
                .field("pa05", &self.pa05())
                .field("pa06", &self.pa06())
                .field("pa07", &self.pa07())
                .field("pa08", &self.pa08())
                .field("pa09", &self.pa09())
                .field("pa10", &self.pa10())
                .field("pa11", &self.pa11())
                .field("pa12", &self.pa12())
                .field("pa13", &self.pa13())
                .field("pa14", &self.pa14())
                .field("pa15", &self.pa15())
                .field("rsv", &self.rsv())
                .finish()
        }
    }
    #[doc = "desc PAHIE."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Pahie(pub u32);
    impl Pahie {
        #[doc = "desc PA00."]
        #[must_use]
        #[inline(always)]
        pub const fn pa00(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "desc PA00."]
        #[inline(always)]
        pub const fn set_pa00(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "desc PA01."]
        #[must_use]
        #[inline(always)]
        pub const fn pa01(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "desc PA01."]
        #[inline(always)]
        pub const fn set_pa01(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "desc PA02."]
        #[must_use]
        #[inline(always)]
        pub const fn pa02(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "desc PA02."]
        #[inline(always)]
        pub const fn set_pa02(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "desc PA03."]
        #[must_use]
        #[inline(always)]
        pub const fn pa03(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "desc PA03."]
        #[inline(always)]
        pub const fn set_pa03(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "desc PA04."]
        #[must_use]
        #[inline(always)]
        pub const fn pa04(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "desc PA04."]
        #[inline(always)]
        pub const fn set_pa04(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "desc PA05."]
        #[must_use]
        #[inline(always)]
        pub const fn pa05(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "desc PA05."]
        #[inline(always)]
        pub const fn set_pa05(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "desc PA06."]
        #[must_use]
        #[inline(always)]
        pub const fn pa06(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "desc PA06."]
        #[inline(always)]
        pub const fn set_pa06(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "desc PA07."]
        #[must_use]
        #[inline(always)]
        pub const fn pa07(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "desc PA07."]
        #[inline(always)]
        pub const fn set_pa07(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "desc PA08."]
        #[must_use]
        #[inline(always)]
        pub const fn pa08(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "desc PA08."]
        #[inline(always)]
        pub const fn set_pa08(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "desc PA09."]
        #[must_use]
        #[inline(always)]
        pub const fn pa09(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "desc PA09."]
        #[inline(always)]
        pub const fn set_pa09(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "desc PA10."]
        #[must_use]
        #[inline(always)]
        pub const fn pa10(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "desc PA10."]
        #[inline(always)]
        pub const fn set_pa10(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[doc = "desc PA11."]
        #[must_use]
        #[inline(always)]
        pub const fn pa11(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "desc PA11."]
        #[inline(always)]
        pub const fn set_pa11(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[doc = "desc PA12."]
        #[must_use]
        #[inline(always)]
        pub const fn pa12(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "desc PA12."]
        #[inline(always)]
        pub const fn set_pa12(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "desc PA13."]
        #[must_use]
        #[inline(always)]
        pub const fn pa13(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[doc = "desc PA13."]
        #[inline(always)]
        pub const fn set_pa13(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[doc = "desc PA14."]
        #[must_use]
        #[inline(always)]
        pub const fn pa14(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "desc PA14."]
        #[inline(always)]
        pub const fn set_pa14(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[doc = "desc PA15."]
        #[must_use]
        #[inline(always)]
        pub const fn pa15(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "desc PA15."]
        #[inline(always)]
        pub const fn set_pa15(&mut self, val: bool) {
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
    impl Default for Pahie {
        #[inline(always)]
        fn default() -> Pahie {
            Pahie(0)
        }
    }
    impl core::fmt::Debug for Pahie {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Pahie")
                .field("pa00", &self.pa00())
                .field("pa01", &self.pa01())
                .field("pa02", &self.pa02())
                .field("pa03", &self.pa03())
                .field("pa04", &self.pa04())
                .field("pa05", &self.pa05())
                .field("pa06", &self.pa06())
                .field("pa07", &self.pa07())
                .field("pa08", &self.pa08())
                .field("pa09", &self.pa09())
                .field("pa10", &self.pa10())
                .field("pa11", &self.pa11())
                .field("pa12", &self.pa12())
                .field("pa13", &self.pa13())
                .field("pa14", &self.pa14())
                .field("pa15", &self.pa15())
                .field("rsv", &self.rsv())
                .finish()
        }
    }
    #[doc = "desc PAIN."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Pain(pub u32);
    impl Pain {
        #[doc = "desc PA00."]
        #[must_use]
        #[inline(always)]
        pub const fn pa00(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "desc PA00."]
        #[inline(always)]
        pub const fn set_pa00(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "desc PA01."]
        #[must_use]
        #[inline(always)]
        pub const fn pa01(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "desc PA01."]
        #[inline(always)]
        pub const fn set_pa01(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "desc PA02."]
        #[must_use]
        #[inline(always)]
        pub const fn pa02(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "desc PA02."]
        #[inline(always)]
        pub const fn set_pa02(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "desc PA03."]
        #[must_use]
        #[inline(always)]
        pub const fn pa03(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "desc PA03."]
        #[inline(always)]
        pub const fn set_pa03(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "desc PA04."]
        #[must_use]
        #[inline(always)]
        pub const fn pa04(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "desc PA04."]
        #[inline(always)]
        pub const fn set_pa04(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "desc PA05."]
        #[must_use]
        #[inline(always)]
        pub const fn pa05(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "desc PA05."]
        #[inline(always)]
        pub const fn set_pa05(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "desc PA06."]
        #[must_use]
        #[inline(always)]
        pub const fn pa06(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "desc PA06."]
        #[inline(always)]
        pub const fn set_pa06(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "desc PA07."]
        #[must_use]
        #[inline(always)]
        pub const fn pa07(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "desc PA07."]
        #[inline(always)]
        pub const fn set_pa07(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "desc PA08."]
        #[must_use]
        #[inline(always)]
        pub const fn pa08(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "desc PA08."]
        #[inline(always)]
        pub const fn set_pa08(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "desc PA09."]
        #[must_use]
        #[inline(always)]
        pub const fn pa09(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "desc PA09."]
        #[inline(always)]
        pub const fn set_pa09(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "desc PA10."]
        #[must_use]
        #[inline(always)]
        pub const fn pa10(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "desc PA10."]
        #[inline(always)]
        pub const fn set_pa10(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[doc = "desc PA11."]
        #[must_use]
        #[inline(always)]
        pub const fn pa11(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "desc PA11."]
        #[inline(always)]
        pub const fn set_pa11(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[doc = "desc PA12."]
        #[must_use]
        #[inline(always)]
        pub const fn pa12(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "desc PA12."]
        #[inline(always)]
        pub const fn set_pa12(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "desc PA13."]
        #[must_use]
        #[inline(always)]
        pub const fn pa13(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[doc = "desc PA13."]
        #[inline(always)]
        pub const fn set_pa13(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[doc = "desc PA14."]
        #[must_use]
        #[inline(always)]
        pub const fn pa14(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "desc PA14."]
        #[inline(always)]
        pub const fn set_pa14(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[doc = "desc PA15."]
        #[must_use]
        #[inline(always)]
        pub const fn pa15(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "desc PA15."]
        #[inline(always)]
        pub const fn set_pa15(&mut self, val: bool) {
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
    impl Default for Pain {
        #[inline(always)]
        fn default() -> Pain {
            Pain(0)
        }
    }
    impl core::fmt::Debug for Pain {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Pain")
                .field("pa00", &self.pa00())
                .field("pa01", &self.pa01())
                .field("pa02", &self.pa02())
                .field("pa03", &self.pa03())
                .field("pa04", &self.pa04())
                .field("pa05", &self.pa05())
                .field("pa06", &self.pa06())
                .field("pa07", &self.pa07())
                .field("pa08", &self.pa08())
                .field("pa09", &self.pa09())
                .field("pa10", &self.pa10())
                .field("pa11", &self.pa11())
                .field("pa12", &self.pa12())
                .field("pa13", &self.pa13())
                .field("pa14", &self.pa14())
                .field("pa15", &self.pa15())
                .field("rsv", &self.rsv())
                .finish()
        }
    }
    #[doc = "desc PALIE."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Palie(pub u32);
    impl Palie {
        #[doc = "desc PA00."]
        #[must_use]
        #[inline(always)]
        pub const fn pa00(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "desc PA00."]
        #[inline(always)]
        pub const fn set_pa00(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "desc PA01."]
        #[must_use]
        #[inline(always)]
        pub const fn pa01(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "desc PA01."]
        #[inline(always)]
        pub const fn set_pa01(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "desc PA02."]
        #[must_use]
        #[inline(always)]
        pub const fn pa02(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "desc PA02."]
        #[inline(always)]
        pub const fn set_pa02(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "desc PA03."]
        #[must_use]
        #[inline(always)]
        pub const fn pa03(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "desc PA03."]
        #[inline(always)]
        pub const fn set_pa03(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "desc PA04."]
        #[must_use]
        #[inline(always)]
        pub const fn pa04(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "desc PA04."]
        #[inline(always)]
        pub const fn set_pa04(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "desc PA05."]
        #[must_use]
        #[inline(always)]
        pub const fn pa05(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "desc PA05."]
        #[inline(always)]
        pub const fn set_pa05(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "desc PA06."]
        #[must_use]
        #[inline(always)]
        pub const fn pa06(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "desc PA06."]
        #[inline(always)]
        pub const fn set_pa06(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "desc PA07."]
        #[must_use]
        #[inline(always)]
        pub const fn pa07(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "desc PA07."]
        #[inline(always)]
        pub const fn set_pa07(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "desc PA08."]
        #[must_use]
        #[inline(always)]
        pub const fn pa08(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "desc PA08."]
        #[inline(always)]
        pub const fn set_pa08(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "desc PA09."]
        #[must_use]
        #[inline(always)]
        pub const fn pa09(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "desc PA09."]
        #[inline(always)]
        pub const fn set_pa09(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "desc PA10."]
        #[must_use]
        #[inline(always)]
        pub const fn pa10(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "desc PA10."]
        #[inline(always)]
        pub const fn set_pa10(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[doc = "desc PA11."]
        #[must_use]
        #[inline(always)]
        pub const fn pa11(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "desc PA11."]
        #[inline(always)]
        pub const fn set_pa11(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[doc = "desc PA12."]
        #[must_use]
        #[inline(always)]
        pub const fn pa12(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "desc PA12."]
        #[inline(always)]
        pub const fn set_pa12(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "desc PA13."]
        #[must_use]
        #[inline(always)]
        pub const fn pa13(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[doc = "desc PA13."]
        #[inline(always)]
        pub const fn set_pa13(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[doc = "desc PA14."]
        #[must_use]
        #[inline(always)]
        pub const fn pa14(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "desc PA14."]
        #[inline(always)]
        pub const fn set_pa14(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[doc = "desc PA15."]
        #[must_use]
        #[inline(always)]
        pub const fn pa15(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "desc PA15."]
        #[inline(always)]
        pub const fn set_pa15(&mut self, val: bool) {
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
    impl Default for Palie {
        #[inline(always)]
        fn default() -> Palie {
            Palie(0)
        }
    }
    impl core::fmt::Debug for Palie {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Palie")
                .field("pa00", &self.pa00())
                .field("pa01", &self.pa01())
                .field("pa02", &self.pa02())
                .field("pa03", &self.pa03())
                .field("pa04", &self.pa04())
                .field("pa05", &self.pa05())
                .field("pa06", &self.pa06())
                .field("pa07", &self.pa07())
                .field("pa08", &self.pa08())
                .field("pa09", &self.pa09())
                .field("pa10", &self.pa10())
                .field("pa11", &self.pa11())
                .field("pa12", &self.pa12())
                .field("pa13", &self.pa13())
                .field("pa14", &self.pa14())
                .field("pa15", &self.pa15())
                .field("rsv", &self.rsv())
                .finish()
        }
    }
    #[doc = "desc PAOD."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Paod(pub u32);
    impl Paod {
        #[doc = "desc PA00."]
        #[must_use]
        #[inline(always)]
        pub const fn pa00(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "desc PA00."]
        #[inline(always)]
        pub const fn set_pa00(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "desc PA01."]
        #[must_use]
        #[inline(always)]
        pub const fn pa01(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "desc PA01."]
        #[inline(always)]
        pub const fn set_pa01(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "desc PA02."]
        #[must_use]
        #[inline(always)]
        pub const fn pa02(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "desc PA02."]
        #[inline(always)]
        pub const fn set_pa02(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "desc PA03."]
        #[must_use]
        #[inline(always)]
        pub const fn pa03(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "desc PA03."]
        #[inline(always)]
        pub const fn set_pa03(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "desc PA04."]
        #[must_use]
        #[inline(always)]
        pub const fn pa04(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "desc PA04."]
        #[inline(always)]
        pub const fn set_pa04(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "desc PA05."]
        #[must_use]
        #[inline(always)]
        pub const fn pa05(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "desc PA05."]
        #[inline(always)]
        pub const fn set_pa05(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "desc PA06."]
        #[must_use]
        #[inline(always)]
        pub const fn pa06(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "desc PA06."]
        #[inline(always)]
        pub const fn set_pa06(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "desc PA07."]
        #[must_use]
        #[inline(always)]
        pub const fn pa07(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "desc PA07."]
        #[inline(always)]
        pub const fn set_pa07(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "desc PA08."]
        #[must_use]
        #[inline(always)]
        pub const fn pa08(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "desc PA08."]
        #[inline(always)]
        pub const fn set_pa08(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "desc PA09."]
        #[must_use]
        #[inline(always)]
        pub const fn pa09(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "desc PA09."]
        #[inline(always)]
        pub const fn set_pa09(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "desc PA10."]
        #[must_use]
        #[inline(always)]
        pub const fn pa10(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "desc PA10."]
        #[inline(always)]
        pub const fn set_pa10(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[doc = "desc PA11."]
        #[must_use]
        #[inline(always)]
        pub const fn pa11(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "desc PA11."]
        #[inline(always)]
        pub const fn set_pa11(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[doc = "desc PA12."]
        #[must_use]
        #[inline(always)]
        pub const fn pa12(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "desc PA12."]
        #[inline(always)]
        pub const fn set_pa12(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "desc PA13."]
        #[must_use]
        #[inline(always)]
        pub const fn pa13(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[doc = "desc PA13."]
        #[inline(always)]
        pub const fn set_pa13(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[doc = "desc PA14."]
        #[must_use]
        #[inline(always)]
        pub const fn pa14(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "desc PA14."]
        #[inline(always)]
        pub const fn set_pa14(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[doc = "desc PA15."]
        #[must_use]
        #[inline(always)]
        pub const fn pa15(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "desc PA15."]
        #[inline(always)]
        pub const fn set_pa15(&mut self, val: bool) {
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
    impl Default for Paod {
        #[inline(always)]
        fn default() -> Paod {
            Paod(0)
        }
    }
    impl core::fmt::Debug for Paod {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Paod")
                .field("pa00", &self.pa00())
                .field("pa01", &self.pa01())
                .field("pa02", &self.pa02())
                .field("pa03", &self.pa03())
                .field("pa04", &self.pa04())
                .field("pa05", &self.pa05())
                .field("pa06", &self.pa06())
                .field("pa07", &self.pa07())
                .field("pa08", &self.pa08())
                .field("pa09", &self.pa09())
                .field("pa10", &self.pa10())
                .field("pa11", &self.pa11())
                .field("pa12", &self.pa12())
                .field("pa13", &self.pa13())
                .field("pa14", &self.pa14())
                .field("pa15", &self.pa15())
                .field("rsv", &self.rsv())
                .finish()
        }
    }
    #[doc = "desc PAOUT."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Paout(pub u32);
    impl Paout {
        #[doc = "desc PA00."]
        #[must_use]
        #[inline(always)]
        pub const fn pa00(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "desc PA00."]
        #[inline(always)]
        pub const fn set_pa00(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "desc PA01."]
        #[must_use]
        #[inline(always)]
        pub const fn pa01(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "desc PA01."]
        #[inline(always)]
        pub const fn set_pa01(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "desc PA02."]
        #[must_use]
        #[inline(always)]
        pub const fn pa02(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "desc PA02."]
        #[inline(always)]
        pub const fn set_pa02(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "desc PA03."]
        #[must_use]
        #[inline(always)]
        pub const fn pa03(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "desc PA03."]
        #[inline(always)]
        pub const fn set_pa03(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "desc PA04."]
        #[must_use]
        #[inline(always)]
        pub const fn pa04(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "desc PA04."]
        #[inline(always)]
        pub const fn set_pa04(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "desc PA05."]
        #[must_use]
        #[inline(always)]
        pub const fn pa05(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "desc PA05."]
        #[inline(always)]
        pub const fn set_pa05(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "desc PA06."]
        #[must_use]
        #[inline(always)]
        pub const fn pa06(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "desc PA06."]
        #[inline(always)]
        pub const fn set_pa06(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "desc PA07."]
        #[must_use]
        #[inline(always)]
        pub const fn pa07(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "desc PA07."]
        #[inline(always)]
        pub const fn set_pa07(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "desc PA08."]
        #[must_use]
        #[inline(always)]
        pub const fn pa08(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "desc PA08."]
        #[inline(always)]
        pub const fn set_pa08(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "desc PA09."]
        #[must_use]
        #[inline(always)]
        pub const fn pa09(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "desc PA09."]
        #[inline(always)]
        pub const fn set_pa09(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "desc PA10."]
        #[must_use]
        #[inline(always)]
        pub const fn pa10(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "desc PA10."]
        #[inline(always)]
        pub const fn set_pa10(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[doc = "desc PA11."]
        #[must_use]
        #[inline(always)]
        pub const fn pa11(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "desc PA11."]
        #[inline(always)]
        pub const fn set_pa11(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[doc = "desc PA12."]
        #[must_use]
        #[inline(always)]
        pub const fn pa12(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "desc PA12."]
        #[inline(always)]
        pub const fn set_pa12(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "desc PA13."]
        #[must_use]
        #[inline(always)]
        pub const fn pa13(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[doc = "desc PA13."]
        #[inline(always)]
        pub const fn set_pa13(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[doc = "desc PA14."]
        #[must_use]
        #[inline(always)]
        pub const fn pa14(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "desc PA14."]
        #[inline(always)]
        pub const fn set_pa14(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[doc = "desc PA15."]
        #[must_use]
        #[inline(always)]
        pub const fn pa15(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "desc PA15."]
        #[inline(always)]
        pub const fn set_pa15(&mut self, val: bool) {
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
    impl Default for Paout {
        #[inline(always)]
        fn default() -> Paout {
            Paout(0)
        }
    }
    impl core::fmt::Debug for Paout {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Paout")
                .field("pa00", &self.pa00())
                .field("pa01", &self.pa01())
                .field("pa02", &self.pa02())
                .field("pa03", &self.pa03())
                .field("pa04", &self.pa04())
                .field("pa05", &self.pa05())
                .field("pa06", &self.pa06())
                .field("pa07", &self.pa07())
                .field("pa08", &self.pa08())
                .field("pa09", &self.pa09())
                .field("pa10", &self.pa10())
                .field("pa11", &self.pa11())
                .field("pa12", &self.pa12())
                .field("pa13", &self.pa13())
                .field("pa14", &self.pa14())
                .field("pa15", &self.pa15())
                .field("rsv", &self.rsv())
                .finish()
        }
    }
    #[doc = "desc PAPD."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Papd(pub u32);
    impl Papd {
        #[doc = "desc PA00."]
        #[must_use]
        #[inline(always)]
        pub const fn pa00(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "desc PA00."]
        #[inline(always)]
        pub const fn set_pa00(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "desc PA01."]
        #[must_use]
        #[inline(always)]
        pub const fn pa01(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "desc PA01."]
        #[inline(always)]
        pub const fn set_pa01(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "desc PA02."]
        #[must_use]
        #[inline(always)]
        pub const fn pa02(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "desc PA02."]
        #[inline(always)]
        pub const fn set_pa02(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "desc PA03."]
        #[must_use]
        #[inline(always)]
        pub const fn pa03(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "desc PA03."]
        #[inline(always)]
        pub const fn set_pa03(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "desc PA04."]
        #[must_use]
        #[inline(always)]
        pub const fn pa04(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "desc PA04."]
        #[inline(always)]
        pub const fn set_pa04(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "desc PA05."]
        #[must_use]
        #[inline(always)]
        pub const fn pa05(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "desc PA05."]
        #[inline(always)]
        pub const fn set_pa05(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "desc PA06."]
        #[must_use]
        #[inline(always)]
        pub const fn pa06(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "desc PA06."]
        #[inline(always)]
        pub const fn set_pa06(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "desc PA07."]
        #[must_use]
        #[inline(always)]
        pub const fn pa07(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "desc PA07."]
        #[inline(always)]
        pub const fn set_pa07(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "desc PA08."]
        #[must_use]
        #[inline(always)]
        pub const fn pa08(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "desc PA08."]
        #[inline(always)]
        pub const fn set_pa08(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "desc PA09."]
        #[must_use]
        #[inline(always)]
        pub const fn pa09(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "desc PA09."]
        #[inline(always)]
        pub const fn set_pa09(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "desc PA10."]
        #[must_use]
        #[inline(always)]
        pub const fn pa10(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "desc PA10."]
        #[inline(always)]
        pub const fn set_pa10(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[doc = "desc PA11."]
        #[must_use]
        #[inline(always)]
        pub const fn pa11(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "desc PA11."]
        #[inline(always)]
        pub const fn set_pa11(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[doc = "desc PA12."]
        #[must_use]
        #[inline(always)]
        pub const fn pa12(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "desc PA12."]
        #[inline(always)]
        pub const fn set_pa12(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "desc PA13."]
        #[must_use]
        #[inline(always)]
        pub const fn pa13(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[doc = "desc PA13."]
        #[inline(always)]
        pub const fn set_pa13(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[doc = "desc PA14."]
        #[must_use]
        #[inline(always)]
        pub const fn pa14(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "desc PA14."]
        #[inline(always)]
        pub const fn set_pa14(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[doc = "desc PA15."]
        #[must_use]
        #[inline(always)]
        pub const fn pa15(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "desc PA15."]
        #[inline(always)]
        pub const fn set_pa15(&mut self, val: bool) {
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
    impl Default for Papd {
        #[inline(always)]
        fn default() -> Papd {
            Papd(0)
        }
    }
    impl core::fmt::Debug for Papd {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Papd")
                .field("pa00", &self.pa00())
                .field("pa01", &self.pa01())
                .field("pa02", &self.pa02())
                .field("pa03", &self.pa03())
                .field("pa04", &self.pa04())
                .field("pa05", &self.pa05())
                .field("pa06", &self.pa06())
                .field("pa07", &self.pa07())
                .field("pa08", &self.pa08())
                .field("pa09", &self.pa09())
                .field("pa10", &self.pa10())
                .field("pa11", &self.pa11())
                .field("pa12", &self.pa12())
                .field("pa13", &self.pa13())
                .field("pa14", &self.pa14())
                .field("pa15", &self.pa15())
                .field("rsv", &self.rsv())
                .finish()
        }
    }
    #[doc = "desc PAPU."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Papu(pub u32);
    impl Papu {
        #[doc = "desc PA00."]
        #[must_use]
        #[inline(always)]
        pub const fn pa00(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "desc PA00."]
        #[inline(always)]
        pub const fn set_pa00(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "desc PA01."]
        #[must_use]
        #[inline(always)]
        pub const fn pa01(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "desc PA01."]
        #[inline(always)]
        pub const fn set_pa01(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "desc PA02."]
        #[must_use]
        #[inline(always)]
        pub const fn pa02(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "desc PA02."]
        #[inline(always)]
        pub const fn set_pa02(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "desc PA03."]
        #[must_use]
        #[inline(always)]
        pub const fn pa03(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "desc PA03."]
        #[inline(always)]
        pub const fn set_pa03(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "desc PA04."]
        #[must_use]
        #[inline(always)]
        pub const fn pa04(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "desc PA04."]
        #[inline(always)]
        pub const fn set_pa04(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "desc PA05."]
        #[must_use]
        #[inline(always)]
        pub const fn pa05(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "desc PA05."]
        #[inline(always)]
        pub const fn set_pa05(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "desc PA06."]
        #[must_use]
        #[inline(always)]
        pub const fn pa06(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "desc PA06."]
        #[inline(always)]
        pub const fn set_pa06(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "desc PA07."]
        #[must_use]
        #[inline(always)]
        pub const fn pa07(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "desc PA07."]
        #[inline(always)]
        pub const fn set_pa07(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "desc PA08."]
        #[must_use]
        #[inline(always)]
        pub const fn pa08(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "desc PA08."]
        #[inline(always)]
        pub const fn set_pa08(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "desc PA09."]
        #[must_use]
        #[inline(always)]
        pub const fn pa09(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "desc PA09."]
        #[inline(always)]
        pub const fn set_pa09(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "desc PA10."]
        #[must_use]
        #[inline(always)]
        pub const fn pa10(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "desc PA10."]
        #[inline(always)]
        pub const fn set_pa10(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[doc = "desc PA11."]
        #[must_use]
        #[inline(always)]
        pub const fn pa11(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "desc PA11."]
        #[inline(always)]
        pub const fn set_pa11(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[doc = "desc PA12."]
        #[must_use]
        #[inline(always)]
        pub const fn pa12(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "desc PA12."]
        #[inline(always)]
        pub const fn set_pa12(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "desc PA13."]
        #[must_use]
        #[inline(always)]
        pub const fn pa13(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[doc = "desc PA13."]
        #[inline(always)]
        pub const fn set_pa13(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[doc = "desc PA14."]
        #[must_use]
        #[inline(always)]
        pub const fn pa14(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "desc PA14."]
        #[inline(always)]
        pub const fn set_pa14(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[doc = "desc PA15."]
        #[must_use]
        #[inline(always)]
        pub const fn pa15(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "desc PA15."]
        #[inline(always)]
        pub const fn set_pa15(&mut self, val: bool) {
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
    impl Default for Papu {
        #[inline(always)]
        fn default() -> Papu {
            Papu(0)
        }
    }
    impl core::fmt::Debug for Papu {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Papu")
                .field("pa00", &self.pa00())
                .field("pa01", &self.pa01())
                .field("pa02", &self.pa02())
                .field("pa03", &self.pa03())
                .field("pa04", &self.pa04())
                .field("pa05", &self.pa05())
                .field("pa06", &self.pa06())
                .field("pa07", &self.pa07())
                .field("pa08", &self.pa08())
                .field("pa09", &self.pa09())
                .field("pa10", &self.pa10())
                .field("pa11", &self.pa11())
                .field("pa12", &self.pa12())
                .field("pa13", &self.pa13())
                .field("pa14", &self.pa14())
                .field("pa15", &self.pa15())
                .field("rsv", &self.rsv())
                .finish()
        }
    }
    #[doc = "desc PARIE."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Parie(pub u32);
    impl Parie {
        #[doc = "desc PA00."]
        #[must_use]
        #[inline(always)]
        pub const fn pa00(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "desc PA00."]
        #[inline(always)]
        pub const fn set_pa00(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "desc PA01."]
        #[must_use]
        #[inline(always)]
        pub const fn pa01(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "desc PA01."]
        #[inline(always)]
        pub const fn set_pa01(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "desc PA02."]
        #[must_use]
        #[inline(always)]
        pub const fn pa02(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "desc PA02."]
        #[inline(always)]
        pub const fn set_pa02(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "desc PA03."]
        #[must_use]
        #[inline(always)]
        pub const fn pa03(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "desc PA03."]
        #[inline(always)]
        pub const fn set_pa03(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "desc PA04."]
        #[must_use]
        #[inline(always)]
        pub const fn pa04(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "desc PA04."]
        #[inline(always)]
        pub const fn set_pa04(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "desc PA05."]
        #[must_use]
        #[inline(always)]
        pub const fn pa05(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "desc PA05."]
        #[inline(always)]
        pub const fn set_pa05(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "desc PA06."]
        #[must_use]
        #[inline(always)]
        pub const fn pa06(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "desc PA06."]
        #[inline(always)]
        pub const fn set_pa06(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "desc PA07."]
        #[must_use]
        #[inline(always)]
        pub const fn pa07(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "desc PA07."]
        #[inline(always)]
        pub const fn set_pa07(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "desc PA08."]
        #[must_use]
        #[inline(always)]
        pub const fn pa08(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "desc PA08."]
        #[inline(always)]
        pub const fn set_pa08(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "desc PA09."]
        #[must_use]
        #[inline(always)]
        pub const fn pa09(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "desc PA09."]
        #[inline(always)]
        pub const fn set_pa09(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "desc PA10."]
        #[must_use]
        #[inline(always)]
        pub const fn pa10(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "desc PA10."]
        #[inline(always)]
        pub const fn set_pa10(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[doc = "desc PA11."]
        #[must_use]
        #[inline(always)]
        pub const fn pa11(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "desc PA11."]
        #[inline(always)]
        pub const fn set_pa11(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[doc = "desc PA12."]
        #[must_use]
        #[inline(always)]
        pub const fn pa12(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "desc PA12."]
        #[inline(always)]
        pub const fn set_pa12(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "desc PA13."]
        #[must_use]
        #[inline(always)]
        pub const fn pa13(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[doc = "desc PA13."]
        #[inline(always)]
        pub const fn set_pa13(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[doc = "desc PA14."]
        #[must_use]
        #[inline(always)]
        pub const fn pa14(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "desc PA14."]
        #[inline(always)]
        pub const fn set_pa14(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[doc = "desc PA15."]
        #[must_use]
        #[inline(always)]
        pub const fn pa15(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "desc PA15."]
        #[inline(always)]
        pub const fn set_pa15(&mut self, val: bool) {
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
    impl Default for Parie {
        #[inline(always)]
        fn default() -> Parie {
            Parie(0)
        }
    }
    impl core::fmt::Debug for Parie {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Parie")
                .field("pa00", &self.pa00())
                .field("pa01", &self.pa01())
                .field("pa02", &self.pa02())
                .field("pa03", &self.pa03())
                .field("pa04", &self.pa04())
                .field("pa05", &self.pa05())
                .field("pa06", &self.pa06())
                .field("pa07", &self.pa07())
                .field("pa08", &self.pa08())
                .field("pa09", &self.pa09())
                .field("pa10", &self.pa10())
                .field("pa11", &self.pa11())
                .field("pa12", &self.pa12())
                .field("pa13", &self.pa13())
                .field("pa14", &self.pa14())
                .field("pa15", &self.pa15())
                .field("rsv", &self.rsv())
                .finish()
        }
    }
    #[doc = "desc PB00_SEL."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Pb00Sel(pub u32);
    impl Pb00Sel {
        #[doc = "desc SEL."]
        #[must_use]
        #[inline(always)]
        pub const fn sel(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x07;
            val as u8
        }
        #[doc = "desc SEL."]
        #[inline(always)]
        pub const fn set_sel(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 0usize)) | (((val as u32) & 0x07) << 0usize);
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
    impl Default for Pb00Sel {
        #[inline(always)]
        fn default() -> Pb00Sel {
            Pb00Sel(0)
        }
    }
    impl core::fmt::Debug for Pb00Sel {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Pb00Sel")
                .field("sel", &self.sel())
                .field("rsv", &self.rsv())
                .finish()
        }
    }
    #[doc = "desc PB01_SEL."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Pb01Sel(pub u32);
    impl Pb01Sel {
        #[doc = "desc SEL."]
        #[must_use]
        #[inline(always)]
        pub const fn sel(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x07;
            val as u8
        }
        #[doc = "desc SEL."]
        #[inline(always)]
        pub const fn set_sel(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 0usize)) | (((val as u32) & 0x07) << 0usize);
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
    impl Default for Pb01Sel {
        #[inline(always)]
        fn default() -> Pb01Sel {
            Pb01Sel(0)
        }
    }
    impl core::fmt::Debug for Pb01Sel {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Pb01Sel")
                .field("sel", &self.sel())
                .field("rsv", &self.rsv())
                .finish()
        }
    }
    #[doc = "desc PB02_SEL."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Pb02Sel(pub u32);
    impl Pb02Sel {
        #[doc = "desc SEL."]
        #[must_use]
        #[inline(always)]
        pub const fn sel(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x07;
            val as u8
        }
        #[doc = "desc SEL."]
        #[inline(always)]
        pub const fn set_sel(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 0usize)) | (((val as u32) & 0x07) << 0usize);
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
    impl Default for Pb02Sel {
        #[inline(always)]
        fn default() -> Pb02Sel {
            Pb02Sel(0)
        }
    }
    impl core::fmt::Debug for Pb02Sel {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Pb02Sel")
                .field("sel", &self.sel())
                .field("rsv", &self.rsv())
                .finish()
        }
    }
    #[doc = "desc PB03_SEL."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Pb03Sel(pub u32);
    impl Pb03Sel {
        #[doc = "desc SEL."]
        #[must_use]
        #[inline(always)]
        pub const fn sel(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x07;
            val as u8
        }
        #[doc = "desc SEL."]
        #[inline(always)]
        pub const fn set_sel(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 0usize)) | (((val as u32) & 0x07) << 0usize);
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
    impl Default for Pb03Sel {
        #[inline(always)]
        fn default() -> Pb03Sel {
            Pb03Sel(0)
        }
    }
    impl core::fmt::Debug for Pb03Sel {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Pb03Sel")
                .field("sel", &self.sel())
                .field("rsv", &self.rsv())
                .finish()
        }
    }
    #[doc = "desc PB04_SEL."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Pb04Sel(pub u32);
    impl Pb04Sel {
        #[doc = "desc SEL."]
        #[must_use]
        #[inline(always)]
        pub const fn sel(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x07;
            val as u8
        }
        #[doc = "desc SEL."]
        #[inline(always)]
        pub const fn set_sel(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 0usize)) | (((val as u32) & 0x07) << 0usize);
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
    impl Default for Pb04Sel {
        #[inline(always)]
        fn default() -> Pb04Sel {
            Pb04Sel(0)
        }
    }
    impl core::fmt::Debug for Pb04Sel {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Pb04Sel")
                .field("sel", &self.sel())
                .field("rsv", &self.rsv())
                .finish()
        }
    }
    #[doc = "desc PB05_SEL."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Pb05Sel(pub u32);
    impl Pb05Sel {
        #[doc = "desc SEL."]
        #[must_use]
        #[inline(always)]
        pub const fn sel(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x07;
            val as u8
        }
        #[doc = "desc SEL."]
        #[inline(always)]
        pub const fn set_sel(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 0usize)) | (((val as u32) & 0x07) << 0usize);
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
    impl Default for Pb05Sel {
        #[inline(always)]
        fn default() -> Pb05Sel {
            Pb05Sel(0)
        }
    }
    impl core::fmt::Debug for Pb05Sel {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Pb05Sel")
                .field("sel", &self.sel())
                .field("rsv", &self.rsv())
                .finish()
        }
    }
    #[doc = "desc PB06_SEL."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Pb06Sel(pub u32);
    impl Pb06Sel {
        #[doc = "desc SEL."]
        #[must_use]
        #[inline(always)]
        pub const fn sel(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x07;
            val as u8
        }
        #[doc = "desc SEL."]
        #[inline(always)]
        pub const fn set_sel(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 0usize)) | (((val as u32) & 0x07) << 0usize);
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
    impl Default for Pb06Sel {
        #[inline(always)]
        fn default() -> Pb06Sel {
            Pb06Sel(0)
        }
    }
    impl core::fmt::Debug for Pb06Sel {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Pb06Sel")
                .field("sel", &self.sel())
                .field("rsv", &self.rsv())
                .finish()
        }
    }
    #[doc = "desc PB07_SEL."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Pb07Sel(pub u32);
    impl Pb07Sel {
        #[doc = "desc SEL."]
        #[must_use]
        #[inline(always)]
        pub const fn sel(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x07;
            val as u8
        }
        #[doc = "desc SEL."]
        #[inline(always)]
        pub const fn set_sel(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 0usize)) | (((val as u32) & 0x07) << 0usize);
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
    impl Default for Pb07Sel {
        #[inline(always)]
        fn default() -> Pb07Sel {
            Pb07Sel(0)
        }
    }
    impl core::fmt::Debug for Pb07Sel {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Pb07Sel")
                .field("sel", &self.sel())
                .field("rsv", &self.rsv())
                .finish()
        }
    }
    #[doc = "desc PB08_SEL."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Pb08Sel(pub u32);
    impl Pb08Sel {
        #[doc = "desc SEL."]
        #[must_use]
        #[inline(always)]
        pub const fn sel(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x07;
            val as u8
        }
        #[doc = "desc SEL."]
        #[inline(always)]
        pub const fn set_sel(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 0usize)) | (((val as u32) & 0x07) << 0usize);
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
    impl Default for Pb08Sel {
        #[inline(always)]
        fn default() -> Pb08Sel {
            Pb08Sel(0)
        }
    }
    impl core::fmt::Debug for Pb08Sel {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Pb08Sel")
                .field("sel", &self.sel())
                .field("rsv", &self.rsv())
                .finish()
        }
    }
    #[doc = "desc PB09_SEL."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Pb09Sel(pub u32);
    impl Pb09Sel {
        #[doc = "desc SEL."]
        #[must_use]
        #[inline(always)]
        pub const fn sel(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x07;
            val as u8
        }
        #[doc = "desc SEL."]
        #[inline(always)]
        pub const fn set_sel(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 0usize)) | (((val as u32) & 0x07) << 0usize);
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
    impl Default for Pb09Sel {
        #[inline(always)]
        fn default() -> Pb09Sel {
            Pb09Sel(0)
        }
    }
    impl core::fmt::Debug for Pb09Sel {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Pb09Sel")
                .field("sel", &self.sel())
                .field("rsv", &self.rsv())
                .finish()
        }
    }
    #[doc = "desc PB10_SEL."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Pb10Sel(pub u32);
    impl Pb10Sel {
        #[doc = "desc SEL."]
        #[must_use]
        #[inline(always)]
        pub const fn sel(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x07;
            val as u8
        }
        #[doc = "desc SEL."]
        #[inline(always)]
        pub const fn set_sel(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 0usize)) | (((val as u32) & 0x07) << 0usize);
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
    impl Default for Pb10Sel {
        #[inline(always)]
        fn default() -> Pb10Sel {
            Pb10Sel(0)
        }
    }
    impl core::fmt::Debug for Pb10Sel {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Pb10Sel")
                .field("sel", &self.sel())
                .field("rsv", &self.rsv())
                .finish()
        }
    }
    #[doc = "desc PB11_SEL."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Pb11Sel(pub u32);
    impl Pb11Sel {
        #[doc = "desc SEL."]
        #[must_use]
        #[inline(always)]
        pub const fn sel(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x07;
            val as u8
        }
        #[doc = "desc SEL."]
        #[inline(always)]
        pub const fn set_sel(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 0usize)) | (((val as u32) & 0x07) << 0usize);
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
    impl Default for Pb11Sel {
        #[inline(always)]
        fn default() -> Pb11Sel {
            Pb11Sel(0)
        }
    }
    impl core::fmt::Debug for Pb11Sel {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Pb11Sel")
                .field("sel", &self.sel())
                .field("rsv", &self.rsv())
                .finish()
        }
    }
    #[doc = "desc PB12_SEL."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Pb12Sel(pub u32);
    impl Pb12Sel {
        #[doc = "desc SEL."]
        #[must_use]
        #[inline(always)]
        pub const fn sel(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x07;
            val as u8
        }
        #[doc = "desc SEL."]
        #[inline(always)]
        pub const fn set_sel(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 0usize)) | (((val as u32) & 0x07) << 0usize);
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
    impl Default for Pb12Sel {
        #[inline(always)]
        fn default() -> Pb12Sel {
            Pb12Sel(0)
        }
    }
    impl core::fmt::Debug for Pb12Sel {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Pb12Sel")
                .field("sel", &self.sel())
                .field("rsv", &self.rsv())
                .finish()
        }
    }
    #[doc = "desc PB13_SEL."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Pb13Sel(pub u32);
    impl Pb13Sel {
        #[doc = "desc SEL."]
        #[must_use]
        #[inline(always)]
        pub const fn sel(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x07;
            val as u8
        }
        #[doc = "desc SEL."]
        #[inline(always)]
        pub const fn set_sel(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 0usize)) | (((val as u32) & 0x07) << 0usize);
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
    impl Default for Pb13Sel {
        #[inline(always)]
        fn default() -> Pb13Sel {
            Pb13Sel(0)
        }
    }
    impl core::fmt::Debug for Pb13Sel {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Pb13Sel")
                .field("sel", &self.sel())
                .field("rsv", &self.rsv())
                .finish()
        }
    }
    #[doc = "desc PB14_SEL."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Pb14Sel(pub u32);
    impl Pb14Sel {
        #[doc = "desc SEL."]
        #[must_use]
        #[inline(always)]
        pub const fn sel(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x07;
            val as u8
        }
        #[doc = "desc SEL."]
        #[inline(always)]
        pub const fn set_sel(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 0usize)) | (((val as u32) & 0x07) << 0usize);
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
    impl Default for Pb14Sel {
        #[inline(always)]
        fn default() -> Pb14Sel {
            Pb14Sel(0)
        }
    }
    impl core::fmt::Debug for Pb14Sel {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Pb14Sel")
                .field("sel", &self.sel())
                .field("rsv", &self.rsv())
                .finish()
        }
    }
    #[doc = "desc PB15_SEL."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Pb15Sel(pub u32);
    impl Pb15Sel {
        #[doc = "desc SEL."]
        #[must_use]
        #[inline(always)]
        pub const fn sel(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x07;
            val as u8
        }
        #[doc = "desc SEL."]
        #[inline(always)]
        pub const fn set_sel(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 0usize)) | (((val as u32) & 0x07) << 0usize);
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
    impl Default for Pb15Sel {
        #[inline(always)]
        fn default() -> Pb15Sel {
            Pb15Sel(0)
        }
    }
    impl core::fmt::Debug for Pb15Sel {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Pb15Sel")
                .field("sel", &self.sel())
                .field("rsv", &self.rsv())
                .finish()
        }
    }
    #[doc = "desc PB_ICLR."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct PbIclr(pub u32);
    impl PbIclr {
        #[doc = "desc PB00."]
        #[must_use]
        #[inline(always)]
        pub const fn pb00(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "desc PB00."]
        #[inline(always)]
        pub const fn set_pb00(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "desc PB01."]
        #[must_use]
        #[inline(always)]
        pub const fn pb01(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "desc PB01."]
        #[inline(always)]
        pub const fn set_pb01(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "desc PB02."]
        #[must_use]
        #[inline(always)]
        pub const fn pb02(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "desc PB02."]
        #[inline(always)]
        pub const fn set_pb02(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "desc PB03."]
        #[must_use]
        #[inline(always)]
        pub const fn pb03(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "desc PB03."]
        #[inline(always)]
        pub const fn set_pb03(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "desc PB04."]
        #[must_use]
        #[inline(always)]
        pub const fn pb04(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "desc PB04."]
        #[inline(always)]
        pub const fn set_pb04(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "desc PB05."]
        #[must_use]
        #[inline(always)]
        pub const fn pb05(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "desc PB05."]
        #[inline(always)]
        pub const fn set_pb05(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "desc PB06."]
        #[must_use]
        #[inline(always)]
        pub const fn pb06(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "desc PB06."]
        #[inline(always)]
        pub const fn set_pb06(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "desc PB07."]
        #[must_use]
        #[inline(always)]
        pub const fn pb07(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "desc PB07."]
        #[inline(always)]
        pub const fn set_pb07(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "desc PB08."]
        #[must_use]
        #[inline(always)]
        pub const fn pb08(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "desc PB08."]
        #[inline(always)]
        pub const fn set_pb08(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "desc PB09."]
        #[must_use]
        #[inline(always)]
        pub const fn pb09(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "desc PB09."]
        #[inline(always)]
        pub const fn set_pb09(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "desc PB10."]
        #[must_use]
        #[inline(always)]
        pub const fn pb10(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "desc PB10."]
        #[inline(always)]
        pub const fn set_pb10(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[doc = "desc PB11."]
        #[must_use]
        #[inline(always)]
        pub const fn pb11(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "desc PB11."]
        #[inline(always)]
        pub const fn set_pb11(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[doc = "desc PB12."]
        #[must_use]
        #[inline(always)]
        pub const fn pb12(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "desc PB12."]
        #[inline(always)]
        pub const fn set_pb12(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "desc PB13."]
        #[must_use]
        #[inline(always)]
        pub const fn pb13(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[doc = "desc PB13."]
        #[inline(always)]
        pub const fn set_pb13(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[doc = "desc PB14."]
        #[must_use]
        #[inline(always)]
        pub const fn pb14(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "desc PB14."]
        #[inline(always)]
        pub const fn set_pb14(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[doc = "desc PB15."]
        #[must_use]
        #[inline(always)]
        pub const fn pb15(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "desc PB15."]
        #[inline(always)]
        pub const fn set_pb15(&mut self, val: bool) {
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
    impl Default for PbIclr {
        #[inline(always)]
        fn default() -> PbIclr {
            PbIclr(0)
        }
    }
    impl core::fmt::Debug for PbIclr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("PbIclr")
                .field("pb00", &self.pb00())
                .field("pb01", &self.pb01())
                .field("pb02", &self.pb02())
                .field("pb03", &self.pb03())
                .field("pb04", &self.pb04())
                .field("pb05", &self.pb05())
                .field("pb06", &self.pb06())
                .field("pb07", &self.pb07())
                .field("pb08", &self.pb08())
                .field("pb09", &self.pb09())
                .field("pb10", &self.pb10())
                .field("pb11", &self.pb11())
                .field("pb12", &self.pb12())
                .field("pb13", &self.pb13())
                .field("pb14", &self.pb14())
                .field("pb15", &self.pb15())
                .field("rsv", &self.rsv())
                .finish()
        }
    }
    #[doc = "desc PB_STAT."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct PbStat(pub u32);
    impl PbStat {
        #[doc = "desc PB00."]
        #[must_use]
        #[inline(always)]
        pub const fn pb00(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "desc PB00."]
        #[inline(always)]
        pub const fn set_pb00(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "desc PB01."]
        #[must_use]
        #[inline(always)]
        pub const fn pb01(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "desc PB01."]
        #[inline(always)]
        pub const fn set_pb01(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "desc PB02."]
        #[must_use]
        #[inline(always)]
        pub const fn pb02(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "desc PB02."]
        #[inline(always)]
        pub const fn set_pb02(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "desc PB03."]
        #[must_use]
        #[inline(always)]
        pub const fn pb03(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "desc PB03."]
        #[inline(always)]
        pub const fn set_pb03(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "desc PB04."]
        #[must_use]
        #[inline(always)]
        pub const fn pb04(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "desc PB04."]
        #[inline(always)]
        pub const fn set_pb04(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "desc PB05."]
        #[must_use]
        #[inline(always)]
        pub const fn pb05(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "desc PB05."]
        #[inline(always)]
        pub const fn set_pb05(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "desc PB06."]
        #[must_use]
        #[inline(always)]
        pub const fn pb06(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "desc PB06."]
        #[inline(always)]
        pub const fn set_pb06(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "desc PB07."]
        #[must_use]
        #[inline(always)]
        pub const fn pb07(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "desc PB07."]
        #[inline(always)]
        pub const fn set_pb07(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "desc PB08."]
        #[must_use]
        #[inline(always)]
        pub const fn pb08(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "desc PB08."]
        #[inline(always)]
        pub const fn set_pb08(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "desc PB09."]
        #[must_use]
        #[inline(always)]
        pub const fn pb09(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "desc PB09."]
        #[inline(always)]
        pub const fn set_pb09(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "desc PB10."]
        #[must_use]
        #[inline(always)]
        pub const fn pb10(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "desc PB10."]
        #[inline(always)]
        pub const fn set_pb10(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[doc = "desc PB11."]
        #[must_use]
        #[inline(always)]
        pub const fn pb11(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "desc PB11."]
        #[inline(always)]
        pub const fn set_pb11(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[doc = "desc PB12."]
        #[must_use]
        #[inline(always)]
        pub const fn pb12(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "desc PB12."]
        #[inline(always)]
        pub const fn set_pb12(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "desc PB13."]
        #[must_use]
        #[inline(always)]
        pub const fn pb13(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[doc = "desc PB13."]
        #[inline(always)]
        pub const fn set_pb13(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[doc = "desc PB14."]
        #[must_use]
        #[inline(always)]
        pub const fn pb14(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "desc PB14."]
        #[inline(always)]
        pub const fn set_pb14(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[doc = "desc PB15."]
        #[must_use]
        #[inline(always)]
        pub const fn pb15(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "desc PB15."]
        #[inline(always)]
        pub const fn set_pb15(&mut self, val: bool) {
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
    impl Default for PbStat {
        #[inline(always)]
        fn default() -> PbStat {
            PbStat(0)
        }
    }
    impl core::fmt::Debug for PbStat {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("PbStat")
                .field("pb00", &self.pb00())
                .field("pb01", &self.pb01())
                .field("pb02", &self.pb02())
                .field("pb03", &self.pb03())
                .field("pb04", &self.pb04())
                .field("pb05", &self.pb05())
                .field("pb06", &self.pb06())
                .field("pb07", &self.pb07())
                .field("pb08", &self.pb08())
                .field("pb09", &self.pb09())
                .field("pb10", &self.pb10())
                .field("pb11", &self.pb11())
                .field("pb12", &self.pb12())
                .field("pb13", &self.pb13())
                .field("pb14", &self.pb14())
                .field("pb15", &self.pb15())
                .field("rsv", &self.rsv())
                .finish()
        }
    }
    #[doc = "desc PBADS."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Pbads(pub u32);
    impl Pbads {
        #[doc = "desc PB00."]
        #[must_use]
        #[inline(always)]
        pub const fn pb00(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "desc PB00."]
        #[inline(always)]
        pub const fn set_pb00(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "desc PB01."]
        #[must_use]
        #[inline(always)]
        pub const fn pb01(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "desc PB01."]
        #[inline(always)]
        pub const fn set_pb01(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "desc PB02."]
        #[must_use]
        #[inline(always)]
        pub const fn pb02(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "desc PB02."]
        #[inline(always)]
        pub const fn set_pb02(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "desc PB03."]
        #[must_use]
        #[inline(always)]
        pub const fn pb03(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "desc PB03."]
        #[inline(always)]
        pub const fn set_pb03(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "desc PB04."]
        #[must_use]
        #[inline(always)]
        pub const fn pb04(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "desc PB04."]
        #[inline(always)]
        pub const fn set_pb04(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "desc PB05."]
        #[must_use]
        #[inline(always)]
        pub const fn pb05(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "desc PB05."]
        #[inline(always)]
        pub const fn set_pb05(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "desc PB06."]
        #[must_use]
        #[inline(always)]
        pub const fn pb06(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "desc PB06."]
        #[inline(always)]
        pub const fn set_pb06(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "desc PB07."]
        #[must_use]
        #[inline(always)]
        pub const fn pb07(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "desc PB07."]
        #[inline(always)]
        pub const fn set_pb07(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "desc PB08."]
        #[must_use]
        #[inline(always)]
        pub const fn pb08(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "desc PB08."]
        #[inline(always)]
        pub const fn set_pb08(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "desc PB09."]
        #[must_use]
        #[inline(always)]
        pub const fn pb09(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "desc PB09."]
        #[inline(always)]
        pub const fn set_pb09(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "desc PB10."]
        #[must_use]
        #[inline(always)]
        pub const fn pb10(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "desc PB10."]
        #[inline(always)]
        pub const fn set_pb10(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[doc = "desc PB11."]
        #[must_use]
        #[inline(always)]
        pub const fn pb11(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "desc PB11."]
        #[inline(always)]
        pub const fn set_pb11(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[doc = "desc PB12."]
        #[must_use]
        #[inline(always)]
        pub const fn pb12(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "desc PB12."]
        #[inline(always)]
        pub const fn set_pb12(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "desc PB13."]
        #[must_use]
        #[inline(always)]
        pub const fn pb13(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[doc = "desc PB13."]
        #[inline(always)]
        pub const fn set_pb13(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[doc = "desc PB14."]
        #[must_use]
        #[inline(always)]
        pub const fn pb14(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "desc PB14."]
        #[inline(always)]
        pub const fn set_pb14(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[doc = "desc PB15."]
        #[must_use]
        #[inline(always)]
        pub const fn pb15(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "desc PB15."]
        #[inline(always)]
        pub const fn set_pb15(&mut self, val: bool) {
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
    impl Default for Pbads {
        #[inline(always)]
        fn default() -> Pbads {
            Pbads(0)
        }
    }
    impl core::fmt::Debug for Pbads {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Pbads")
                .field("pb00", &self.pb00())
                .field("pb01", &self.pb01())
                .field("pb02", &self.pb02())
                .field("pb03", &self.pb03())
                .field("pb04", &self.pb04())
                .field("pb05", &self.pb05())
                .field("pb06", &self.pb06())
                .field("pb07", &self.pb07())
                .field("pb08", &self.pb08())
                .field("pb09", &self.pb09())
                .field("pb10", &self.pb10())
                .field("pb11", &self.pb11())
                .field("pb12", &self.pb12())
                .field("pb13", &self.pb13())
                .field("pb14", &self.pb14())
                .field("pb15", &self.pb15())
                .field("rsv", &self.rsv())
                .finish()
        }
    }
    #[doc = "desc PBBCLR."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Pbbclr(pub u32);
    impl Pbbclr {
        #[doc = "desc PB00."]
        #[must_use]
        #[inline(always)]
        pub const fn pb00(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "desc PB00."]
        #[inline(always)]
        pub const fn set_pb00(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "desc PB01."]
        #[must_use]
        #[inline(always)]
        pub const fn pb01(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "desc PB01."]
        #[inline(always)]
        pub const fn set_pb01(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "desc PB02."]
        #[must_use]
        #[inline(always)]
        pub const fn pb02(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "desc PB02."]
        #[inline(always)]
        pub const fn set_pb02(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "desc PB03."]
        #[must_use]
        #[inline(always)]
        pub const fn pb03(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "desc PB03."]
        #[inline(always)]
        pub const fn set_pb03(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "desc PB04."]
        #[must_use]
        #[inline(always)]
        pub const fn pb04(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "desc PB04."]
        #[inline(always)]
        pub const fn set_pb04(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "desc PB05."]
        #[must_use]
        #[inline(always)]
        pub const fn pb05(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "desc PB05."]
        #[inline(always)]
        pub const fn set_pb05(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "desc PB06."]
        #[must_use]
        #[inline(always)]
        pub const fn pb06(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "desc PB06."]
        #[inline(always)]
        pub const fn set_pb06(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "desc PB07."]
        #[must_use]
        #[inline(always)]
        pub const fn pb07(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "desc PB07."]
        #[inline(always)]
        pub const fn set_pb07(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "desc PB08."]
        #[must_use]
        #[inline(always)]
        pub const fn pb08(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "desc PB08."]
        #[inline(always)]
        pub const fn set_pb08(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "desc PB09."]
        #[must_use]
        #[inline(always)]
        pub const fn pb09(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "desc PB09."]
        #[inline(always)]
        pub const fn set_pb09(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "desc PB10."]
        #[must_use]
        #[inline(always)]
        pub const fn pb10(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "desc PB10."]
        #[inline(always)]
        pub const fn set_pb10(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[doc = "desc PB11."]
        #[must_use]
        #[inline(always)]
        pub const fn pb11(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "desc PB11."]
        #[inline(always)]
        pub const fn set_pb11(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[doc = "desc PB12."]
        #[must_use]
        #[inline(always)]
        pub const fn pb12(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "desc PB12."]
        #[inline(always)]
        pub const fn set_pb12(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "desc PB13."]
        #[must_use]
        #[inline(always)]
        pub const fn pb13(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[doc = "desc PB13."]
        #[inline(always)]
        pub const fn set_pb13(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[doc = "desc PB14."]
        #[must_use]
        #[inline(always)]
        pub const fn pb14(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "desc PB14."]
        #[inline(always)]
        pub const fn set_pb14(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[doc = "desc PB15."]
        #[must_use]
        #[inline(always)]
        pub const fn pb15(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "desc PB15."]
        #[inline(always)]
        pub const fn set_pb15(&mut self, val: bool) {
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
    impl Default for Pbbclr {
        #[inline(always)]
        fn default() -> Pbbclr {
            Pbbclr(0)
        }
    }
    impl core::fmt::Debug for Pbbclr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Pbbclr")
                .field("pb00", &self.pb00())
                .field("pb01", &self.pb01())
                .field("pb02", &self.pb02())
                .field("pb03", &self.pb03())
                .field("pb04", &self.pb04())
                .field("pb05", &self.pb05())
                .field("pb06", &self.pb06())
                .field("pb07", &self.pb07())
                .field("pb08", &self.pb08())
                .field("pb09", &self.pb09())
                .field("pb10", &self.pb10())
                .field("pb11", &self.pb11())
                .field("pb12", &self.pb12())
                .field("pb13", &self.pb13())
                .field("pb14", &self.pb14())
                .field("pb15", &self.pb15())
                .field("rsv", &self.rsv())
                .finish()
        }
    }
    #[doc = "desc PBBSET."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Pbbset(pub u32);
    impl Pbbset {
        #[doc = "desc PB00."]
        #[must_use]
        #[inline(always)]
        pub const fn pb00(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "desc PB00."]
        #[inline(always)]
        pub const fn set_pb00(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "desc PB01."]
        #[must_use]
        #[inline(always)]
        pub const fn pb01(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "desc PB01."]
        #[inline(always)]
        pub const fn set_pb01(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "desc PB02."]
        #[must_use]
        #[inline(always)]
        pub const fn pb02(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "desc PB02."]
        #[inline(always)]
        pub const fn set_pb02(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "desc PB03."]
        #[must_use]
        #[inline(always)]
        pub const fn pb03(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "desc PB03."]
        #[inline(always)]
        pub const fn set_pb03(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "desc PB04."]
        #[must_use]
        #[inline(always)]
        pub const fn pb04(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "desc PB04."]
        #[inline(always)]
        pub const fn set_pb04(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "desc PB05."]
        #[must_use]
        #[inline(always)]
        pub const fn pb05(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "desc PB05."]
        #[inline(always)]
        pub const fn set_pb05(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "desc PB06."]
        #[must_use]
        #[inline(always)]
        pub const fn pb06(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "desc PB06."]
        #[inline(always)]
        pub const fn set_pb06(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "desc PB07."]
        #[must_use]
        #[inline(always)]
        pub const fn pb07(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "desc PB07."]
        #[inline(always)]
        pub const fn set_pb07(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "desc PB08."]
        #[must_use]
        #[inline(always)]
        pub const fn pb08(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "desc PB08."]
        #[inline(always)]
        pub const fn set_pb08(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "desc PB09."]
        #[must_use]
        #[inline(always)]
        pub const fn pb09(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "desc PB09."]
        #[inline(always)]
        pub const fn set_pb09(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "desc PB10."]
        #[must_use]
        #[inline(always)]
        pub const fn pb10(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "desc PB10."]
        #[inline(always)]
        pub const fn set_pb10(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[doc = "desc PB11."]
        #[must_use]
        #[inline(always)]
        pub const fn pb11(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "desc PB11."]
        #[inline(always)]
        pub const fn set_pb11(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[doc = "desc PB12."]
        #[must_use]
        #[inline(always)]
        pub const fn pb12(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "desc PB12."]
        #[inline(always)]
        pub const fn set_pb12(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "desc PB13."]
        #[must_use]
        #[inline(always)]
        pub const fn pb13(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[doc = "desc PB13."]
        #[inline(always)]
        pub const fn set_pb13(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[doc = "desc PB14."]
        #[must_use]
        #[inline(always)]
        pub const fn pb14(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "desc PB14."]
        #[inline(always)]
        pub const fn set_pb14(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[doc = "desc PB15."]
        #[must_use]
        #[inline(always)]
        pub const fn pb15(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "desc PB15."]
        #[inline(always)]
        pub const fn set_pb15(&mut self, val: bool) {
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
    impl Default for Pbbset {
        #[inline(always)]
        fn default() -> Pbbset {
            Pbbset(0)
        }
    }
    impl core::fmt::Debug for Pbbset {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Pbbset")
                .field("pb00", &self.pb00())
                .field("pb01", &self.pb01())
                .field("pb02", &self.pb02())
                .field("pb03", &self.pb03())
                .field("pb04", &self.pb04())
                .field("pb05", &self.pb05())
                .field("pb06", &self.pb06())
                .field("pb07", &self.pb07())
                .field("pb08", &self.pb08())
                .field("pb09", &self.pb09())
                .field("pb10", &self.pb10())
                .field("pb11", &self.pb11())
                .field("pb12", &self.pb12())
                .field("pb13", &self.pb13())
                .field("pb14", &self.pb14())
                .field("pb15", &self.pb15())
                .field("rsv", &self.rsv())
                .finish()
        }
    }
    #[doc = "desc PBBSETCLR."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Pbbsetclr(pub u32);
    impl Pbbsetclr {
        #[doc = "desc PBBCLR."]
        #[must_use]
        #[inline(always)]
        pub const fn pbbclr(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[doc = "desc PBBCLR."]
        #[inline(always)]
        pub const fn set_pbbclr(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
        }
        #[doc = "desc PBBSET."]
        #[must_use]
        #[inline(always)]
        pub const fn pbbset(&self) -> u16 {
            let val = (self.0 >> 16usize) & 0xffff;
            val as u16
        }
        #[doc = "desc PBBSET."]
        #[inline(always)]
        pub const fn set_pbbset(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 16usize)) | (((val as u32) & 0xffff) << 16usize);
        }
    }
    impl Default for Pbbsetclr {
        #[inline(always)]
        fn default() -> Pbbsetclr {
            Pbbsetclr(0)
        }
    }
    impl core::fmt::Debug for Pbbsetclr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Pbbsetclr")
                .field("pbbclr", &self.pbbclr())
                .field("pbbset", &self.pbbset())
                .finish()
        }
    }
    #[doc = "desc PBDIR."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Pbdir(pub u32);
    impl Pbdir {
        #[doc = "desc PB00."]
        #[must_use]
        #[inline(always)]
        pub const fn pb00(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "desc PB00."]
        #[inline(always)]
        pub const fn set_pb00(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "desc PB01."]
        #[must_use]
        #[inline(always)]
        pub const fn pb01(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "desc PB01."]
        #[inline(always)]
        pub const fn set_pb01(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "desc PB02."]
        #[must_use]
        #[inline(always)]
        pub const fn pb02(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "desc PB02."]
        #[inline(always)]
        pub const fn set_pb02(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "desc PB03."]
        #[must_use]
        #[inline(always)]
        pub const fn pb03(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "desc PB03."]
        #[inline(always)]
        pub const fn set_pb03(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "desc PB04."]
        #[must_use]
        #[inline(always)]
        pub const fn pb04(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "desc PB04."]
        #[inline(always)]
        pub const fn set_pb04(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "desc PB05."]
        #[must_use]
        #[inline(always)]
        pub const fn pb05(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "desc PB05."]
        #[inline(always)]
        pub const fn set_pb05(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "desc PB06."]
        #[must_use]
        #[inline(always)]
        pub const fn pb06(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "desc PB06."]
        #[inline(always)]
        pub const fn set_pb06(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "desc PB07."]
        #[must_use]
        #[inline(always)]
        pub const fn pb07(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "desc PB07."]
        #[inline(always)]
        pub const fn set_pb07(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "desc PB08."]
        #[must_use]
        #[inline(always)]
        pub const fn pb08(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "desc PB08."]
        #[inline(always)]
        pub const fn set_pb08(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "desc PB09."]
        #[must_use]
        #[inline(always)]
        pub const fn pb09(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "desc PB09."]
        #[inline(always)]
        pub const fn set_pb09(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "desc PB10."]
        #[must_use]
        #[inline(always)]
        pub const fn pb10(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "desc PB10."]
        #[inline(always)]
        pub const fn set_pb10(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[doc = "desc PB11."]
        #[must_use]
        #[inline(always)]
        pub const fn pb11(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "desc PB11."]
        #[inline(always)]
        pub const fn set_pb11(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[doc = "desc PB12."]
        #[must_use]
        #[inline(always)]
        pub const fn pb12(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "desc PB12."]
        #[inline(always)]
        pub const fn set_pb12(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "desc PB13."]
        #[must_use]
        #[inline(always)]
        pub const fn pb13(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[doc = "desc PB13."]
        #[inline(always)]
        pub const fn set_pb13(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[doc = "desc PB14."]
        #[must_use]
        #[inline(always)]
        pub const fn pb14(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "desc PB14."]
        #[inline(always)]
        pub const fn set_pb14(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[doc = "desc PB15."]
        #[must_use]
        #[inline(always)]
        pub const fn pb15(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "desc PB15."]
        #[inline(always)]
        pub const fn set_pb15(&mut self, val: bool) {
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
    impl Default for Pbdir {
        #[inline(always)]
        fn default() -> Pbdir {
            Pbdir(0)
        }
    }
    impl core::fmt::Debug for Pbdir {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Pbdir")
                .field("pb00", &self.pb00())
                .field("pb01", &self.pb01())
                .field("pb02", &self.pb02())
                .field("pb03", &self.pb03())
                .field("pb04", &self.pb04())
                .field("pb05", &self.pb05())
                .field("pb06", &self.pb06())
                .field("pb07", &self.pb07())
                .field("pb08", &self.pb08())
                .field("pb09", &self.pb09())
                .field("pb10", &self.pb10())
                .field("pb11", &self.pb11())
                .field("pb12", &self.pb12())
                .field("pb13", &self.pb13())
                .field("pb14", &self.pb14())
                .field("pb15", &self.pb15())
                .field("rsv", &self.rsv())
                .finish()
        }
    }
    #[doc = "desc PBDR."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Pbdr(pub u32);
    impl Pbdr {
        #[doc = "desc PB00."]
        #[must_use]
        #[inline(always)]
        pub const fn pb00(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "desc PB00."]
        #[inline(always)]
        pub const fn set_pb00(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "desc PB01."]
        #[must_use]
        #[inline(always)]
        pub const fn pb01(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "desc PB01."]
        #[inline(always)]
        pub const fn set_pb01(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "desc PB02."]
        #[must_use]
        #[inline(always)]
        pub const fn pb02(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "desc PB02."]
        #[inline(always)]
        pub const fn set_pb02(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "desc PB03."]
        #[must_use]
        #[inline(always)]
        pub const fn pb03(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "desc PB03."]
        #[inline(always)]
        pub const fn set_pb03(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "desc PB04."]
        #[must_use]
        #[inline(always)]
        pub const fn pb04(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "desc PB04."]
        #[inline(always)]
        pub const fn set_pb04(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "desc PB05."]
        #[must_use]
        #[inline(always)]
        pub const fn pb05(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "desc PB05."]
        #[inline(always)]
        pub const fn set_pb05(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "desc PB06."]
        #[must_use]
        #[inline(always)]
        pub const fn pb06(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "desc PB06."]
        #[inline(always)]
        pub const fn set_pb06(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "desc PB07."]
        #[must_use]
        #[inline(always)]
        pub const fn pb07(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "desc PB07."]
        #[inline(always)]
        pub const fn set_pb07(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "desc PB08."]
        #[must_use]
        #[inline(always)]
        pub const fn pb08(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "desc PB08."]
        #[inline(always)]
        pub const fn set_pb08(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "desc PB09."]
        #[must_use]
        #[inline(always)]
        pub const fn pb09(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "desc PB09."]
        #[inline(always)]
        pub const fn set_pb09(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "desc PB10."]
        #[must_use]
        #[inline(always)]
        pub const fn pb10(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "desc PB10."]
        #[inline(always)]
        pub const fn set_pb10(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[doc = "desc PB11."]
        #[must_use]
        #[inline(always)]
        pub const fn pb11(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "desc PB11."]
        #[inline(always)]
        pub const fn set_pb11(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[doc = "desc PB12."]
        #[must_use]
        #[inline(always)]
        pub const fn pb12(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "desc PB12."]
        #[inline(always)]
        pub const fn set_pb12(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "desc PB13."]
        #[must_use]
        #[inline(always)]
        pub const fn pb13(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[doc = "desc PB13."]
        #[inline(always)]
        pub const fn set_pb13(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[doc = "desc PB14."]
        #[must_use]
        #[inline(always)]
        pub const fn pb14(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "desc PB14."]
        #[inline(always)]
        pub const fn set_pb14(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[doc = "desc PB15."]
        #[must_use]
        #[inline(always)]
        pub const fn pb15(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "desc PB15."]
        #[inline(always)]
        pub const fn set_pb15(&mut self, val: bool) {
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
    impl Default for Pbdr {
        #[inline(always)]
        fn default() -> Pbdr {
            Pbdr(0)
        }
    }
    impl core::fmt::Debug for Pbdr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Pbdr")
                .field("pb00", &self.pb00())
                .field("pb01", &self.pb01())
                .field("pb02", &self.pb02())
                .field("pb03", &self.pb03())
                .field("pb04", &self.pb04())
                .field("pb05", &self.pb05())
                .field("pb06", &self.pb06())
                .field("pb07", &self.pb07())
                .field("pb08", &self.pb08())
                .field("pb09", &self.pb09())
                .field("pb10", &self.pb10())
                .field("pb11", &self.pb11())
                .field("pb12", &self.pb12())
                .field("pb13", &self.pb13())
                .field("pb14", &self.pb14())
                .field("pb15", &self.pb15())
                .field("rsv", &self.rsv())
                .finish()
        }
    }
    #[doc = "desc PBFIE."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Pbfie(pub u32);
    impl Pbfie {
        #[doc = "desc PB00."]
        #[must_use]
        #[inline(always)]
        pub const fn pb00(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "desc PB00."]
        #[inline(always)]
        pub const fn set_pb00(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "desc PB01."]
        #[must_use]
        #[inline(always)]
        pub const fn pb01(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "desc PB01."]
        #[inline(always)]
        pub const fn set_pb01(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "desc PB02."]
        #[must_use]
        #[inline(always)]
        pub const fn pb02(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "desc PB02."]
        #[inline(always)]
        pub const fn set_pb02(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "desc PB03."]
        #[must_use]
        #[inline(always)]
        pub const fn pb03(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "desc PB03."]
        #[inline(always)]
        pub const fn set_pb03(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "desc PB04."]
        #[must_use]
        #[inline(always)]
        pub const fn pb04(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "desc PB04."]
        #[inline(always)]
        pub const fn set_pb04(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "desc PB05."]
        #[must_use]
        #[inline(always)]
        pub const fn pb05(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "desc PB05."]
        #[inline(always)]
        pub const fn set_pb05(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "desc PB06."]
        #[must_use]
        #[inline(always)]
        pub const fn pb06(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "desc PB06."]
        #[inline(always)]
        pub const fn set_pb06(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "desc PB07."]
        #[must_use]
        #[inline(always)]
        pub const fn pb07(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "desc PB07."]
        #[inline(always)]
        pub const fn set_pb07(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "desc PB08."]
        #[must_use]
        #[inline(always)]
        pub const fn pb08(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "desc PB08."]
        #[inline(always)]
        pub const fn set_pb08(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "desc PB09."]
        #[must_use]
        #[inline(always)]
        pub const fn pb09(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "desc PB09."]
        #[inline(always)]
        pub const fn set_pb09(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "desc PB10."]
        #[must_use]
        #[inline(always)]
        pub const fn pb10(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "desc PB10."]
        #[inline(always)]
        pub const fn set_pb10(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[doc = "desc PB11."]
        #[must_use]
        #[inline(always)]
        pub const fn pb11(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "desc PB11."]
        #[inline(always)]
        pub const fn set_pb11(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[doc = "desc PB12."]
        #[must_use]
        #[inline(always)]
        pub const fn pb12(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "desc PB12."]
        #[inline(always)]
        pub const fn set_pb12(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "desc PB13."]
        #[must_use]
        #[inline(always)]
        pub const fn pb13(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[doc = "desc PB13."]
        #[inline(always)]
        pub const fn set_pb13(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[doc = "desc PB14."]
        #[must_use]
        #[inline(always)]
        pub const fn pb14(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "desc PB14."]
        #[inline(always)]
        pub const fn set_pb14(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[doc = "desc PB15."]
        #[must_use]
        #[inline(always)]
        pub const fn pb15(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "desc PB15."]
        #[inline(always)]
        pub const fn set_pb15(&mut self, val: bool) {
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
    impl Default for Pbfie {
        #[inline(always)]
        fn default() -> Pbfie {
            Pbfie(0)
        }
    }
    impl core::fmt::Debug for Pbfie {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Pbfie")
                .field("pb00", &self.pb00())
                .field("pb01", &self.pb01())
                .field("pb02", &self.pb02())
                .field("pb03", &self.pb03())
                .field("pb04", &self.pb04())
                .field("pb05", &self.pb05())
                .field("pb06", &self.pb06())
                .field("pb07", &self.pb07())
                .field("pb08", &self.pb08())
                .field("pb09", &self.pb09())
                .field("pb10", &self.pb10())
                .field("pb11", &self.pb11())
                .field("pb12", &self.pb12())
                .field("pb13", &self.pb13())
                .field("pb14", &self.pb14())
                .field("pb15", &self.pb15())
                .field("rsv", &self.rsv())
                .finish()
        }
    }
    #[doc = "desc PBHIE."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Pbhie(pub u32);
    impl Pbhie {
        #[doc = "desc PB00."]
        #[must_use]
        #[inline(always)]
        pub const fn pb00(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "desc PB00."]
        #[inline(always)]
        pub const fn set_pb00(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "desc PB01."]
        #[must_use]
        #[inline(always)]
        pub const fn pb01(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "desc PB01."]
        #[inline(always)]
        pub const fn set_pb01(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "desc PB02."]
        #[must_use]
        #[inline(always)]
        pub const fn pb02(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "desc PB02."]
        #[inline(always)]
        pub const fn set_pb02(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "desc PB03."]
        #[must_use]
        #[inline(always)]
        pub const fn pb03(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "desc PB03."]
        #[inline(always)]
        pub const fn set_pb03(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "desc PB04."]
        #[must_use]
        #[inline(always)]
        pub const fn pb04(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "desc PB04."]
        #[inline(always)]
        pub const fn set_pb04(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "desc PB05."]
        #[must_use]
        #[inline(always)]
        pub const fn pb05(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "desc PB05."]
        #[inline(always)]
        pub const fn set_pb05(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "desc PB06."]
        #[must_use]
        #[inline(always)]
        pub const fn pb06(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "desc PB06."]
        #[inline(always)]
        pub const fn set_pb06(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "desc PB07."]
        #[must_use]
        #[inline(always)]
        pub const fn pb07(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "desc PB07."]
        #[inline(always)]
        pub const fn set_pb07(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "desc PB08."]
        #[must_use]
        #[inline(always)]
        pub const fn pb08(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "desc PB08."]
        #[inline(always)]
        pub const fn set_pb08(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "desc PB09."]
        #[must_use]
        #[inline(always)]
        pub const fn pb09(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "desc PB09."]
        #[inline(always)]
        pub const fn set_pb09(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "desc PB10."]
        #[must_use]
        #[inline(always)]
        pub const fn pb10(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "desc PB10."]
        #[inline(always)]
        pub const fn set_pb10(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[doc = "desc PB11."]
        #[must_use]
        #[inline(always)]
        pub const fn pb11(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "desc PB11."]
        #[inline(always)]
        pub const fn set_pb11(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[doc = "desc PB12."]
        #[must_use]
        #[inline(always)]
        pub const fn pb12(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "desc PB12."]
        #[inline(always)]
        pub const fn set_pb12(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "desc PB13."]
        #[must_use]
        #[inline(always)]
        pub const fn pb13(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[doc = "desc PB13."]
        #[inline(always)]
        pub const fn set_pb13(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[doc = "desc PB14."]
        #[must_use]
        #[inline(always)]
        pub const fn pb14(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "desc PB14."]
        #[inline(always)]
        pub const fn set_pb14(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[doc = "desc PB15."]
        #[must_use]
        #[inline(always)]
        pub const fn pb15(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "desc PB15."]
        #[inline(always)]
        pub const fn set_pb15(&mut self, val: bool) {
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
    impl Default for Pbhie {
        #[inline(always)]
        fn default() -> Pbhie {
            Pbhie(0)
        }
    }
    impl core::fmt::Debug for Pbhie {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Pbhie")
                .field("pb00", &self.pb00())
                .field("pb01", &self.pb01())
                .field("pb02", &self.pb02())
                .field("pb03", &self.pb03())
                .field("pb04", &self.pb04())
                .field("pb05", &self.pb05())
                .field("pb06", &self.pb06())
                .field("pb07", &self.pb07())
                .field("pb08", &self.pb08())
                .field("pb09", &self.pb09())
                .field("pb10", &self.pb10())
                .field("pb11", &self.pb11())
                .field("pb12", &self.pb12())
                .field("pb13", &self.pb13())
                .field("pb14", &self.pb14())
                .field("pb15", &self.pb15())
                .field("rsv", &self.rsv())
                .finish()
        }
    }
    #[doc = "desc PBIN."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Pbin(pub u32);
    impl Pbin {
        #[doc = "desc PB00."]
        #[must_use]
        #[inline(always)]
        pub const fn pb00(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "desc PB00."]
        #[inline(always)]
        pub const fn set_pb00(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "desc PB01."]
        #[must_use]
        #[inline(always)]
        pub const fn pb01(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "desc PB01."]
        #[inline(always)]
        pub const fn set_pb01(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "desc PB02."]
        #[must_use]
        #[inline(always)]
        pub const fn pb02(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "desc PB02."]
        #[inline(always)]
        pub const fn set_pb02(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "desc PB03."]
        #[must_use]
        #[inline(always)]
        pub const fn pb03(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "desc PB03."]
        #[inline(always)]
        pub const fn set_pb03(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "desc PB04."]
        #[must_use]
        #[inline(always)]
        pub const fn pb04(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "desc PB04."]
        #[inline(always)]
        pub const fn set_pb04(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "desc PB05."]
        #[must_use]
        #[inline(always)]
        pub const fn pb05(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "desc PB05."]
        #[inline(always)]
        pub const fn set_pb05(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "desc PB06."]
        #[must_use]
        #[inline(always)]
        pub const fn pb06(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "desc PB06."]
        #[inline(always)]
        pub const fn set_pb06(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "desc PB07."]
        #[must_use]
        #[inline(always)]
        pub const fn pb07(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "desc PB07."]
        #[inline(always)]
        pub const fn set_pb07(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "desc PB08."]
        #[must_use]
        #[inline(always)]
        pub const fn pb08(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "desc PB08."]
        #[inline(always)]
        pub const fn set_pb08(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "desc PB09."]
        #[must_use]
        #[inline(always)]
        pub const fn pb09(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "desc PB09."]
        #[inline(always)]
        pub const fn set_pb09(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "desc PB10."]
        #[must_use]
        #[inline(always)]
        pub const fn pb10(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "desc PB10."]
        #[inline(always)]
        pub const fn set_pb10(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[doc = "desc PB11."]
        #[must_use]
        #[inline(always)]
        pub const fn pb11(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "desc PB11."]
        #[inline(always)]
        pub const fn set_pb11(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[doc = "desc PB12."]
        #[must_use]
        #[inline(always)]
        pub const fn pb12(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "desc PB12."]
        #[inline(always)]
        pub const fn set_pb12(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "desc PB13."]
        #[must_use]
        #[inline(always)]
        pub const fn pb13(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[doc = "desc PB13."]
        #[inline(always)]
        pub const fn set_pb13(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[doc = "desc PB14."]
        #[must_use]
        #[inline(always)]
        pub const fn pb14(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "desc PB14."]
        #[inline(always)]
        pub const fn set_pb14(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[doc = "desc PB15."]
        #[must_use]
        #[inline(always)]
        pub const fn pb15(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "desc PB15."]
        #[inline(always)]
        pub const fn set_pb15(&mut self, val: bool) {
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
    impl Default for Pbin {
        #[inline(always)]
        fn default() -> Pbin {
            Pbin(0)
        }
    }
    impl core::fmt::Debug for Pbin {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Pbin")
                .field("pb00", &self.pb00())
                .field("pb01", &self.pb01())
                .field("pb02", &self.pb02())
                .field("pb03", &self.pb03())
                .field("pb04", &self.pb04())
                .field("pb05", &self.pb05())
                .field("pb06", &self.pb06())
                .field("pb07", &self.pb07())
                .field("pb08", &self.pb08())
                .field("pb09", &self.pb09())
                .field("pb10", &self.pb10())
                .field("pb11", &self.pb11())
                .field("pb12", &self.pb12())
                .field("pb13", &self.pb13())
                .field("pb14", &self.pb14())
                .field("pb15", &self.pb15())
                .field("rsv", &self.rsv())
                .finish()
        }
    }
    #[doc = "desc PBLIE."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Pblie(pub u32);
    impl Pblie {
        #[doc = "desc PB00."]
        #[must_use]
        #[inline(always)]
        pub const fn pb00(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "desc PB00."]
        #[inline(always)]
        pub const fn set_pb00(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "desc PB01."]
        #[must_use]
        #[inline(always)]
        pub const fn pb01(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "desc PB01."]
        #[inline(always)]
        pub const fn set_pb01(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "desc PB02."]
        #[must_use]
        #[inline(always)]
        pub const fn pb02(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "desc PB02."]
        #[inline(always)]
        pub const fn set_pb02(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "desc PB03."]
        #[must_use]
        #[inline(always)]
        pub const fn pb03(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "desc PB03."]
        #[inline(always)]
        pub const fn set_pb03(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "desc PB04."]
        #[must_use]
        #[inline(always)]
        pub const fn pb04(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "desc PB04."]
        #[inline(always)]
        pub const fn set_pb04(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "desc PB05."]
        #[must_use]
        #[inline(always)]
        pub const fn pb05(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "desc PB05."]
        #[inline(always)]
        pub const fn set_pb05(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "desc PB06."]
        #[must_use]
        #[inline(always)]
        pub const fn pb06(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "desc PB06."]
        #[inline(always)]
        pub const fn set_pb06(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "desc PB07."]
        #[must_use]
        #[inline(always)]
        pub const fn pb07(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "desc PB07."]
        #[inline(always)]
        pub const fn set_pb07(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "desc PB08."]
        #[must_use]
        #[inline(always)]
        pub const fn pb08(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "desc PB08."]
        #[inline(always)]
        pub const fn set_pb08(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "desc PB09."]
        #[must_use]
        #[inline(always)]
        pub const fn pb09(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "desc PB09."]
        #[inline(always)]
        pub const fn set_pb09(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "desc PB10."]
        #[must_use]
        #[inline(always)]
        pub const fn pb10(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "desc PB10."]
        #[inline(always)]
        pub const fn set_pb10(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[doc = "desc PB11."]
        #[must_use]
        #[inline(always)]
        pub const fn pb11(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "desc PB11."]
        #[inline(always)]
        pub const fn set_pb11(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[doc = "desc PB12."]
        #[must_use]
        #[inline(always)]
        pub const fn pb12(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "desc PB12."]
        #[inline(always)]
        pub const fn set_pb12(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "desc PB13."]
        #[must_use]
        #[inline(always)]
        pub const fn pb13(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[doc = "desc PB13."]
        #[inline(always)]
        pub const fn set_pb13(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[doc = "desc PB14."]
        #[must_use]
        #[inline(always)]
        pub const fn pb14(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "desc PB14."]
        #[inline(always)]
        pub const fn set_pb14(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[doc = "desc PB15."]
        #[must_use]
        #[inline(always)]
        pub const fn pb15(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "desc PB15."]
        #[inline(always)]
        pub const fn set_pb15(&mut self, val: bool) {
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
    impl Default for Pblie {
        #[inline(always)]
        fn default() -> Pblie {
            Pblie(0)
        }
    }
    impl core::fmt::Debug for Pblie {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Pblie")
                .field("pb00", &self.pb00())
                .field("pb01", &self.pb01())
                .field("pb02", &self.pb02())
                .field("pb03", &self.pb03())
                .field("pb04", &self.pb04())
                .field("pb05", &self.pb05())
                .field("pb06", &self.pb06())
                .field("pb07", &self.pb07())
                .field("pb08", &self.pb08())
                .field("pb09", &self.pb09())
                .field("pb10", &self.pb10())
                .field("pb11", &self.pb11())
                .field("pb12", &self.pb12())
                .field("pb13", &self.pb13())
                .field("pb14", &self.pb14())
                .field("pb15", &self.pb15())
                .field("rsv", &self.rsv())
                .finish()
        }
    }
    #[doc = "desc PBOD."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Pbod(pub u32);
    impl Pbod {
        #[doc = "desc PB00."]
        #[must_use]
        #[inline(always)]
        pub const fn pb00(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "desc PB00."]
        #[inline(always)]
        pub const fn set_pb00(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "desc PB01."]
        #[must_use]
        #[inline(always)]
        pub const fn pb01(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "desc PB01."]
        #[inline(always)]
        pub const fn set_pb01(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "desc PB02."]
        #[must_use]
        #[inline(always)]
        pub const fn pb02(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "desc PB02."]
        #[inline(always)]
        pub const fn set_pb02(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "desc PB03."]
        #[must_use]
        #[inline(always)]
        pub const fn pb03(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "desc PB03."]
        #[inline(always)]
        pub const fn set_pb03(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "desc PB04."]
        #[must_use]
        #[inline(always)]
        pub const fn pb04(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "desc PB04."]
        #[inline(always)]
        pub const fn set_pb04(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "desc PB05."]
        #[must_use]
        #[inline(always)]
        pub const fn pb05(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "desc PB05."]
        #[inline(always)]
        pub const fn set_pb05(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "desc PB06."]
        #[must_use]
        #[inline(always)]
        pub const fn pb06(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "desc PB06."]
        #[inline(always)]
        pub const fn set_pb06(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "desc PB07."]
        #[must_use]
        #[inline(always)]
        pub const fn pb07(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "desc PB07."]
        #[inline(always)]
        pub const fn set_pb07(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "desc PB08."]
        #[must_use]
        #[inline(always)]
        pub const fn pb08(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "desc PB08."]
        #[inline(always)]
        pub const fn set_pb08(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "desc PB09."]
        #[must_use]
        #[inline(always)]
        pub const fn pb09(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "desc PB09."]
        #[inline(always)]
        pub const fn set_pb09(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "desc PB10."]
        #[must_use]
        #[inline(always)]
        pub const fn pb10(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "desc PB10."]
        #[inline(always)]
        pub const fn set_pb10(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[doc = "desc PB11."]
        #[must_use]
        #[inline(always)]
        pub const fn pb11(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "desc PB11."]
        #[inline(always)]
        pub const fn set_pb11(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[doc = "desc PB12."]
        #[must_use]
        #[inline(always)]
        pub const fn pb12(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "desc PB12."]
        #[inline(always)]
        pub const fn set_pb12(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "desc PB13."]
        #[must_use]
        #[inline(always)]
        pub const fn pb13(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[doc = "desc PB13."]
        #[inline(always)]
        pub const fn set_pb13(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[doc = "desc PB14."]
        #[must_use]
        #[inline(always)]
        pub const fn pb14(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "desc PB14."]
        #[inline(always)]
        pub const fn set_pb14(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[doc = "desc PB15."]
        #[must_use]
        #[inline(always)]
        pub const fn pb15(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "desc PB15."]
        #[inline(always)]
        pub const fn set_pb15(&mut self, val: bool) {
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
    impl Default for Pbod {
        #[inline(always)]
        fn default() -> Pbod {
            Pbod(0)
        }
    }
    impl core::fmt::Debug for Pbod {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Pbod")
                .field("pb00", &self.pb00())
                .field("pb01", &self.pb01())
                .field("pb02", &self.pb02())
                .field("pb03", &self.pb03())
                .field("pb04", &self.pb04())
                .field("pb05", &self.pb05())
                .field("pb06", &self.pb06())
                .field("pb07", &self.pb07())
                .field("pb08", &self.pb08())
                .field("pb09", &self.pb09())
                .field("pb10", &self.pb10())
                .field("pb11", &self.pb11())
                .field("pb12", &self.pb12())
                .field("pb13", &self.pb13())
                .field("pb14", &self.pb14())
                .field("pb15", &self.pb15())
                .field("rsv", &self.rsv())
                .finish()
        }
    }
    #[doc = "desc PBOUT."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Pbout(pub u32);
    impl Pbout {
        #[doc = "desc PB00."]
        #[must_use]
        #[inline(always)]
        pub const fn pb00(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "desc PB00."]
        #[inline(always)]
        pub const fn set_pb00(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "desc PB01."]
        #[must_use]
        #[inline(always)]
        pub const fn pb01(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "desc PB01."]
        #[inline(always)]
        pub const fn set_pb01(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "desc PB02."]
        #[must_use]
        #[inline(always)]
        pub const fn pb02(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "desc PB02."]
        #[inline(always)]
        pub const fn set_pb02(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "desc PB03."]
        #[must_use]
        #[inline(always)]
        pub const fn pb03(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "desc PB03."]
        #[inline(always)]
        pub const fn set_pb03(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "desc PB04."]
        #[must_use]
        #[inline(always)]
        pub const fn pb04(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "desc PB04."]
        #[inline(always)]
        pub const fn set_pb04(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "desc PB05."]
        #[must_use]
        #[inline(always)]
        pub const fn pb05(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "desc PB05."]
        #[inline(always)]
        pub const fn set_pb05(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "desc PB06."]
        #[must_use]
        #[inline(always)]
        pub const fn pb06(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "desc PB06."]
        #[inline(always)]
        pub const fn set_pb06(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "desc PB07."]
        #[must_use]
        #[inline(always)]
        pub const fn pb07(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "desc PB07."]
        #[inline(always)]
        pub const fn set_pb07(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "desc PB08."]
        #[must_use]
        #[inline(always)]
        pub const fn pb08(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "desc PB08."]
        #[inline(always)]
        pub const fn set_pb08(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "desc PB09."]
        #[must_use]
        #[inline(always)]
        pub const fn pb09(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "desc PB09."]
        #[inline(always)]
        pub const fn set_pb09(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "desc PB10."]
        #[must_use]
        #[inline(always)]
        pub const fn pb10(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "desc PB10."]
        #[inline(always)]
        pub const fn set_pb10(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[doc = "desc PB11."]
        #[must_use]
        #[inline(always)]
        pub const fn pb11(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "desc PB11."]
        #[inline(always)]
        pub const fn set_pb11(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[doc = "desc PB12."]
        #[must_use]
        #[inline(always)]
        pub const fn pb12(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "desc PB12."]
        #[inline(always)]
        pub const fn set_pb12(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "desc PB13."]
        #[must_use]
        #[inline(always)]
        pub const fn pb13(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[doc = "desc PB13."]
        #[inline(always)]
        pub const fn set_pb13(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[doc = "desc PB14."]
        #[must_use]
        #[inline(always)]
        pub const fn pb14(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "desc PB14."]
        #[inline(always)]
        pub const fn set_pb14(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[doc = "desc PB15."]
        #[must_use]
        #[inline(always)]
        pub const fn pb15(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "desc PB15."]
        #[inline(always)]
        pub const fn set_pb15(&mut self, val: bool) {
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
    impl Default for Pbout {
        #[inline(always)]
        fn default() -> Pbout {
            Pbout(0)
        }
    }
    impl core::fmt::Debug for Pbout {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Pbout")
                .field("pb00", &self.pb00())
                .field("pb01", &self.pb01())
                .field("pb02", &self.pb02())
                .field("pb03", &self.pb03())
                .field("pb04", &self.pb04())
                .field("pb05", &self.pb05())
                .field("pb06", &self.pb06())
                .field("pb07", &self.pb07())
                .field("pb08", &self.pb08())
                .field("pb09", &self.pb09())
                .field("pb10", &self.pb10())
                .field("pb11", &self.pb11())
                .field("pb12", &self.pb12())
                .field("pb13", &self.pb13())
                .field("pb14", &self.pb14())
                .field("pb15", &self.pb15())
                .field("rsv", &self.rsv())
                .finish()
        }
    }
    #[doc = "desc PBPD."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Pbpd(pub u32);
    impl Pbpd {
        #[doc = "desc PB00."]
        #[must_use]
        #[inline(always)]
        pub const fn pb00(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "desc PB00."]
        #[inline(always)]
        pub const fn set_pb00(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "desc PB01."]
        #[must_use]
        #[inline(always)]
        pub const fn pb01(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "desc PB01."]
        #[inline(always)]
        pub const fn set_pb01(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "desc PB02."]
        #[must_use]
        #[inline(always)]
        pub const fn pb02(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "desc PB02."]
        #[inline(always)]
        pub const fn set_pb02(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "desc PB03."]
        #[must_use]
        #[inline(always)]
        pub const fn pb03(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "desc PB03."]
        #[inline(always)]
        pub const fn set_pb03(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "desc PB04."]
        #[must_use]
        #[inline(always)]
        pub const fn pb04(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "desc PB04."]
        #[inline(always)]
        pub const fn set_pb04(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "desc PB05."]
        #[must_use]
        #[inline(always)]
        pub const fn pb05(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "desc PB05."]
        #[inline(always)]
        pub const fn set_pb05(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "desc PB06."]
        #[must_use]
        #[inline(always)]
        pub const fn pb06(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "desc PB06."]
        #[inline(always)]
        pub const fn set_pb06(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "desc PB07."]
        #[must_use]
        #[inline(always)]
        pub const fn pb07(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "desc PB07."]
        #[inline(always)]
        pub const fn set_pb07(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "desc PB08."]
        #[must_use]
        #[inline(always)]
        pub const fn pb08(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "desc PB08."]
        #[inline(always)]
        pub const fn set_pb08(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "desc PB09."]
        #[must_use]
        #[inline(always)]
        pub const fn pb09(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "desc PB09."]
        #[inline(always)]
        pub const fn set_pb09(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "desc PB10."]
        #[must_use]
        #[inline(always)]
        pub const fn pb10(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "desc PB10."]
        #[inline(always)]
        pub const fn set_pb10(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[doc = "desc PB11."]
        #[must_use]
        #[inline(always)]
        pub const fn pb11(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "desc PB11."]
        #[inline(always)]
        pub const fn set_pb11(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[doc = "desc PB12."]
        #[must_use]
        #[inline(always)]
        pub const fn pb12(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "desc PB12."]
        #[inline(always)]
        pub const fn set_pb12(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "desc PB13."]
        #[must_use]
        #[inline(always)]
        pub const fn pb13(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[doc = "desc PB13."]
        #[inline(always)]
        pub const fn set_pb13(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[doc = "desc PB14."]
        #[must_use]
        #[inline(always)]
        pub const fn pb14(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "desc PB14."]
        #[inline(always)]
        pub const fn set_pb14(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[doc = "desc PB15."]
        #[must_use]
        #[inline(always)]
        pub const fn pb15(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "desc PB15."]
        #[inline(always)]
        pub const fn set_pb15(&mut self, val: bool) {
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
    impl Default for Pbpd {
        #[inline(always)]
        fn default() -> Pbpd {
            Pbpd(0)
        }
    }
    impl core::fmt::Debug for Pbpd {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Pbpd")
                .field("pb00", &self.pb00())
                .field("pb01", &self.pb01())
                .field("pb02", &self.pb02())
                .field("pb03", &self.pb03())
                .field("pb04", &self.pb04())
                .field("pb05", &self.pb05())
                .field("pb06", &self.pb06())
                .field("pb07", &self.pb07())
                .field("pb08", &self.pb08())
                .field("pb09", &self.pb09())
                .field("pb10", &self.pb10())
                .field("pb11", &self.pb11())
                .field("pb12", &self.pb12())
                .field("pb13", &self.pb13())
                .field("pb14", &self.pb14())
                .field("pb15", &self.pb15())
                .field("rsv", &self.rsv())
                .finish()
        }
    }
    #[doc = "desc PBPU."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Pbpu(pub u32);
    impl Pbpu {
        #[doc = "desc PB00."]
        #[must_use]
        #[inline(always)]
        pub const fn pb00(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "desc PB00."]
        #[inline(always)]
        pub const fn set_pb00(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "desc PB01."]
        #[must_use]
        #[inline(always)]
        pub const fn pb01(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "desc PB01."]
        #[inline(always)]
        pub const fn set_pb01(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "desc PB02."]
        #[must_use]
        #[inline(always)]
        pub const fn pb02(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "desc PB02."]
        #[inline(always)]
        pub const fn set_pb02(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "desc PB03."]
        #[must_use]
        #[inline(always)]
        pub const fn pb03(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "desc PB03."]
        #[inline(always)]
        pub const fn set_pb03(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "desc PB04."]
        #[must_use]
        #[inline(always)]
        pub const fn pb04(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "desc PB04."]
        #[inline(always)]
        pub const fn set_pb04(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "desc PB05."]
        #[must_use]
        #[inline(always)]
        pub const fn pb05(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "desc PB05."]
        #[inline(always)]
        pub const fn set_pb05(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "desc PB06."]
        #[must_use]
        #[inline(always)]
        pub const fn pb06(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "desc PB06."]
        #[inline(always)]
        pub const fn set_pb06(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "desc PB07."]
        #[must_use]
        #[inline(always)]
        pub const fn pb07(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "desc PB07."]
        #[inline(always)]
        pub const fn set_pb07(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "desc PB08."]
        #[must_use]
        #[inline(always)]
        pub const fn pb08(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "desc PB08."]
        #[inline(always)]
        pub const fn set_pb08(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "desc PB09."]
        #[must_use]
        #[inline(always)]
        pub const fn pb09(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "desc PB09."]
        #[inline(always)]
        pub const fn set_pb09(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "desc PB10."]
        #[must_use]
        #[inline(always)]
        pub const fn pb10(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "desc PB10."]
        #[inline(always)]
        pub const fn set_pb10(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[doc = "desc PB11."]
        #[must_use]
        #[inline(always)]
        pub const fn pb11(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "desc PB11."]
        #[inline(always)]
        pub const fn set_pb11(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[doc = "desc PB12."]
        #[must_use]
        #[inline(always)]
        pub const fn pb12(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "desc PB12."]
        #[inline(always)]
        pub const fn set_pb12(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "desc PB13."]
        #[must_use]
        #[inline(always)]
        pub const fn pb13(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[doc = "desc PB13."]
        #[inline(always)]
        pub const fn set_pb13(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[doc = "desc PB14."]
        #[must_use]
        #[inline(always)]
        pub const fn pb14(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "desc PB14."]
        #[inline(always)]
        pub const fn set_pb14(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[doc = "desc PB15."]
        #[must_use]
        #[inline(always)]
        pub const fn pb15(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "desc PB15."]
        #[inline(always)]
        pub const fn set_pb15(&mut self, val: bool) {
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
    impl Default for Pbpu {
        #[inline(always)]
        fn default() -> Pbpu {
            Pbpu(0)
        }
    }
    impl core::fmt::Debug for Pbpu {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Pbpu")
                .field("pb00", &self.pb00())
                .field("pb01", &self.pb01())
                .field("pb02", &self.pb02())
                .field("pb03", &self.pb03())
                .field("pb04", &self.pb04())
                .field("pb05", &self.pb05())
                .field("pb06", &self.pb06())
                .field("pb07", &self.pb07())
                .field("pb08", &self.pb08())
                .field("pb09", &self.pb09())
                .field("pb10", &self.pb10())
                .field("pb11", &self.pb11())
                .field("pb12", &self.pb12())
                .field("pb13", &self.pb13())
                .field("pb14", &self.pb14())
                .field("pb15", &self.pb15())
                .field("rsv", &self.rsv())
                .finish()
        }
    }
    #[doc = "desc PBRIE."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Pbrie(pub u32);
    impl Pbrie {
        #[doc = "desc PB00."]
        #[must_use]
        #[inline(always)]
        pub const fn pb00(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "desc PB00."]
        #[inline(always)]
        pub const fn set_pb00(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "desc PB01."]
        #[must_use]
        #[inline(always)]
        pub const fn pb01(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "desc PB01."]
        #[inline(always)]
        pub const fn set_pb01(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "desc PB02."]
        #[must_use]
        #[inline(always)]
        pub const fn pb02(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "desc PB02."]
        #[inline(always)]
        pub const fn set_pb02(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "desc PB03."]
        #[must_use]
        #[inline(always)]
        pub const fn pb03(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "desc PB03."]
        #[inline(always)]
        pub const fn set_pb03(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "desc PB04."]
        #[must_use]
        #[inline(always)]
        pub const fn pb04(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "desc PB04."]
        #[inline(always)]
        pub const fn set_pb04(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "desc PB05."]
        #[must_use]
        #[inline(always)]
        pub const fn pb05(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "desc PB05."]
        #[inline(always)]
        pub const fn set_pb05(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "desc PB06."]
        #[must_use]
        #[inline(always)]
        pub const fn pb06(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "desc PB06."]
        #[inline(always)]
        pub const fn set_pb06(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "desc PB07."]
        #[must_use]
        #[inline(always)]
        pub const fn pb07(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "desc PB07."]
        #[inline(always)]
        pub const fn set_pb07(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "desc PB08."]
        #[must_use]
        #[inline(always)]
        pub const fn pb08(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "desc PB08."]
        #[inline(always)]
        pub const fn set_pb08(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "desc PB09."]
        #[must_use]
        #[inline(always)]
        pub const fn pb09(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "desc PB09."]
        #[inline(always)]
        pub const fn set_pb09(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "desc PB10."]
        #[must_use]
        #[inline(always)]
        pub const fn pb10(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "desc PB10."]
        #[inline(always)]
        pub const fn set_pb10(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[doc = "desc PB11."]
        #[must_use]
        #[inline(always)]
        pub const fn pb11(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "desc PB11."]
        #[inline(always)]
        pub const fn set_pb11(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[doc = "desc PB12."]
        #[must_use]
        #[inline(always)]
        pub const fn pb12(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "desc PB12."]
        #[inline(always)]
        pub const fn set_pb12(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "desc PB13."]
        #[must_use]
        #[inline(always)]
        pub const fn pb13(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[doc = "desc PB13."]
        #[inline(always)]
        pub const fn set_pb13(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[doc = "desc PB14."]
        #[must_use]
        #[inline(always)]
        pub const fn pb14(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "desc PB14."]
        #[inline(always)]
        pub const fn set_pb14(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[doc = "desc PB15."]
        #[must_use]
        #[inline(always)]
        pub const fn pb15(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "desc PB15."]
        #[inline(always)]
        pub const fn set_pb15(&mut self, val: bool) {
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
    impl Default for Pbrie {
        #[inline(always)]
        fn default() -> Pbrie {
            Pbrie(0)
        }
    }
    impl core::fmt::Debug for Pbrie {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Pbrie")
                .field("pb00", &self.pb00())
                .field("pb01", &self.pb01())
                .field("pb02", &self.pb02())
                .field("pb03", &self.pb03())
                .field("pb04", &self.pb04())
                .field("pb05", &self.pb05())
                .field("pb06", &self.pb06())
                .field("pb07", &self.pb07())
                .field("pb08", &self.pb08())
                .field("pb09", &self.pb09())
                .field("pb10", &self.pb10())
                .field("pb11", &self.pb11())
                .field("pb12", &self.pb12())
                .field("pb13", &self.pb13())
                .field("pb14", &self.pb14())
                .field("pb15", &self.pb15())
                .field("rsv", &self.rsv())
                .finish()
        }
    }
    #[doc = "desc PC00_SEL."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Pc00Sel(pub u32);
    impl Pc00Sel {
        #[doc = "desc SEL."]
        #[must_use]
        #[inline(always)]
        pub const fn sel(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x07;
            val as u8
        }
        #[doc = "desc SEL."]
        #[inline(always)]
        pub const fn set_sel(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 0usize)) | (((val as u32) & 0x07) << 0usize);
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
    impl Default for Pc00Sel {
        #[inline(always)]
        fn default() -> Pc00Sel {
            Pc00Sel(0)
        }
    }
    impl core::fmt::Debug for Pc00Sel {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Pc00Sel")
                .field("sel", &self.sel())
                .field("rsv", &self.rsv())
                .finish()
        }
    }
    #[doc = "desc PC01_SEL."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Pc01Sel(pub u32);
    impl Pc01Sel {
        #[doc = "desc SEL."]
        #[must_use]
        #[inline(always)]
        pub const fn sel(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x07;
            val as u8
        }
        #[doc = "desc SEL."]
        #[inline(always)]
        pub const fn set_sel(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 0usize)) | (((val as u32) & 0x07) << 0usize);
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
    impl Default for Pc01Sel {
        #[inline(always)]
        fn default() -> Pc01Sel {
            Pc01Sel(0)
        }
    }
    impl core::fmt::Debug for Pc01Sel {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Pc01Sel")
                .field("sel", &self.sel())
                .field("rsv", &self.rsv())
                .finish()
        }
    }
    #[doc = "desc PC02_SEL."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Pc02Sel(pub u32);
    impl Pc02Sel {
        #[doc = "desc SEL."]
        #[must_use]
        #[inline(always)]
        pub const fn sel(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x07;
            val as u8
        }
        #[doc = "desc SEL."]
        #[inline(always)]
        pub const fn set_sel(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 0usize)) | (((val as u32) & 0x07) << 0usize);
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
    impl Default for Pc02Sel {
        #[inline(always)]
        fn default() -> Pc02Sel {
            Pc02Sel(0)
        }
    }
    impl core::fmt::Debug for Pc02Sel {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Pc02Sel")
                .field("sel", &self.sel())
                .field("rsv", &self.rsv())
                .finish()
        }
    }
    #[doc = "desc PC03_SEL."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Pc03Sel(pub u32);
    impl Pc03Sel {
        #[doc = "desc SEL."]
        #[must_use]
        #[inline(always)]
        pub const fn sel(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x07;
            val as u8
        }
        #[doc = "desc SEL."]
        #[inline(always)]
        pub const fn set_sel(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 0usize)) | (((val as u32) & 0x07) << 0usize);
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
    impl Default for Pc03Sel {
        #[inline(always)]
        fn default() -> Pc03Sel {
            Pc03Sel(0)
        }
    }
    impl core::fmt::Debug for Pc03Sel {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Pc03Sel")
                .field("sel", &self.sel())
                .field("rsv", &self.rsv())
                .finish()
        }
    }
    #[doc = "desc PC04_SEL."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Pc04Sel(pub u32);
    impl Pc04Sel {
        #[doc = "desc SEL."]
        #[must_use]
        #[inline(always)]
        pub const fn sel(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x07;
            val as u8
        }
        #[doc = "desc SEL."]
        #[inline(always)]
        pub const fn set_sel(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 0usize)) | (((val as u32) & 0x07) << 0usize);
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
    impl Default for Pc04Sel {
        #[inline(always)]
        fn default() -> Pc04Sel {
            Pc04Sel(0)
        }
    }
    impl core::fmt::Debug for Pc04Sel {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Pc04Sel")
                .field("sel", &self.sel())
                .field("rsv", &self.rsv())
                .finish()
        }
    }
    #[doc = "desc PC05_SEL."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Pc05Sel(pub u32);
    impl Pc05Sel {
        #[doc = "desc SEL."]
        #[must_use]
        #[inline(always)]
        pub const fn sel(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x07;
            val as u8
        }
        #[doc = "desc SEL."]
        #[inline(always)]
        pub const fn set_sel(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 0usize)) | (((val as u32) & 0x07) << 0usize);
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
    impl Default for Pc05Sel {
        #[inline(always)]
        fn default() -> Pc05Sel {
            Pc05Sel(0)
        }
    }
    impl core::fmt::Debug for Pc05Sel {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Pc05Sel")
                .field("sel", &self.sel())
                .field("rsv", &self.rsv())
                .finish()
        }
    }
    #[doc = "desc PC06_SEL."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Pc06Sel(pub u32);
    impl Pc06Sel {
        #[doc = "desc SEL."]
        #[must_use]
        #[inline(always)]
        pub const fn sel(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x07;
            val as u8
        }
        #[doc = "desc SEL."]
        #[inline(always)]
        pub const fn set_sel(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 0usize)) | (((val as u32) & 0x07) << 0usize);
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
    impl Default for Pc06Sel {
        #[inline(always)]
        fn default() -> Pc06Sel {
            Pc06Sel(0)
        }
    }
    impl core::fmt::Debug for Pc06Sel {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Pc06Sel")
                .field("sel", &self.sel())
                .field("rsv", &self.rsv())
                .finish()
        }
    }
    #[doc = "desc PC07_SEL."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Pc07Sel(pub u32);
    impl Pc07Sel {
        #[doc = "desc SEL."]
        #[must_use]
        #[inline(always)]
        pub const fn sel(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x07;
            val as u8
        }
        #[doc = "desc SEL."]
        #[inline(always)]
        pub const fn set_sel(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 0usize)) | (((val as u32) & 0x07) << 0usize);
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
    impl Default for Pc07Sel {
        #[inline(always)]
        fn default() -> Pc07Sel {
            Pc07Sel(0)
        }
    }
    impl core::fmt::Debug for Pc07Sel {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Pc07Sel")
                .field("sel", &self.sel())
                .field("rsv", &self.rsv())
                .finish()
        }
    }
    #[doc = "desc PC08_SEL."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Pc08Sel(pub u32);
    impl Pc08Sel {
        #[doc = "desc SEL."]
        #[must_use]
        #[inline(always)]
        pub const fn sel(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x07;
            val as u8
        }
        #[doc = "desc SEL."]
        #[inline(always)]
        pub const fn set_sel(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 0usize)) | (((val as u32) & 0x07) << 0usize);
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
    impl Default for Pc08Sel {
        #[inline(always)]
        fn default() -> Pc08Sel {
            Pc08Sel(0)
        }
    }
    impl core::fmt::Debug for Pc08Sel {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Pc08Sel")
                .field("sel", &self.sel())
                .field("rsv", &self.rsv())
                .finish()
        }
    }
    #[doc = "desc PC09_SEL."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Pc09Sel(pub u32);
    impl Pc09Sel {
        #[doc = "desc SEL."]
        #[must_use]
        #[inline(always)]
        pub const fn sel(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x07;
            val as u8
        }
        #[doc = "desc SEL."]
        #[inline(always)]
        pub const fn set_sel(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 0usize)) | (((val as u32) & 0x07) << 0usize);
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
    impl Default for Pc09Sel {
        #[inline(always)]
        fn default() -> Pc09Sel {
            Pc09Sel(0)
        }
    }
    impl core::fmt::Debug for Pc09Sel {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Pc09Sel")
                .field("sel", &self.sel())
                .field("rsv", &self.rsv())
                .finish()
        }
    }
    #[doc = "desc PC10_SEL."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Pc10Sel(pub u32);
    impl Pc10Sel {
        #[doc = "desc SEL."]
        #[must_use]
        #[inline(always)]
        pub const fn sel(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x07;
            val as u8
        }
        #[doc = "desc SEL."]
        #[inline(always)]
        pub const fn set_sel(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 0usize)) | (((val as u32) & 0x07) << 0usize);
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
    impl Default for Pc10Sel {
        #[inline(always)]
        fn default() -> Pc10Sel {
            Pc10Sel(0)
        }
    }
    impl core::fmt::Debug for Pc10Sel {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Pc10Sel")
                .field("sel", &self.sel())
                .field("rsv", &self.rsv())
                .finish()
        }
    }
    #[doc = "desc PC11_SEL."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Pc11Sel(pub u32);
    impl Pc11Sel {
        #[doc = "desc SEL."]
        #[must_use]
        #[inline(always)]
        pub const fn sel(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x07;
            val as u8
        }
        #[doc = "desc SEL."]
        #[inline(always)]
        pub const fn set_sel(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 0usize)) | (((val as u32) & 0x07) << 0usize);
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
    impl Default for Pc11Sel {
        #[inline(always)]
        fn default() -> Pc11Sel {
            Pc11Sel(0)
        }
    }
    impl core::fmt::Debug for Pc11Sel {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Pc11Sel")
                .field("sel", &self.sel())
                .field("rsv", &self.rsv())
                .finish()
        }
    }
    #[doc = "desc PC12_SEL."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Pc12Sel(pub u32);
    impl Pc12Sel {
        #[doc = "desc SEL."]
        #[must_use]
        #[inline(always)]
        pub const fn sel(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x07;
            val as u8
        }
        #[doc = "desc SEL."]
        #[inline(always)]
        pub const fn set_sel(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 0usize)) | (((val as u32) & 0x07) << 0usize);
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
    impl Default for Pc12Sel {
        #[inline(always)]
        fn default() -> Pc12Sel {
            Pc12Sel(0)
        }
    }
    impl core::fmt::Debug for Pc12Sel {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Pc12Sel")
                .field("sel", &self.sel())
                .field("rsv", &self.rsv())
                .finish()
        }
    }
    #[doc = "desc PC13_SEL."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Pc13Sel(pub u32);
    impl Pc13Sel {
        #[doc = "desc SEL."]
        #[must_use]
        #[inline(always)]
        pub const fn sel(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x07;
            val as u8
        }
        #[doc = "desc SEL."]
        #[inline(always)]
        pub const fn set_sel(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 0usize)) | (((val as u32) & 0x07) << 0usize);
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
    impl Default for Pc13Sel {
        #[inline(always)]
        fn default() -> Pc13Sel {
            Pc13Sel(0)
        }
    }
    impl core::fmt::Debug for Pc13Sel {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Pc13Sel")
                .field("sel", &self.sel())
                .field("rsv", &self.rsv())
                .finish()
        }
    }
    #[doc = "desc PC14_SEL."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Pc14Sel(pub u32);
    impl Pc14Sel {
        #[doc = "desc SEL."]
        #[must_use]
        #[inline(always)]
        pub const fn sel(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x07;
            val as u8
        }
        #[doc = "desc SEL."]
        #[inline(always)]
        pub const fn set_sel(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 0usize)) | (((val as u32) & 0x07) << 0usize);
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
    impl Default for Pc14Sel {
        #[inline(always)]
        fn default() -> Pc14Sel {
            Pc14Sel(0)
        }
    }
    impl core::fmt::Debug for Pc14Sel {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Pc14Sel")
                .field("sel", &self.sel())
                .field("rsv", &self.rsv())
                .finish()
        }
    }
    #[doc = "desc PC15_SEL."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Pc15Sel(pub u32);
    impl Pc15Sel {
        #[doc = "desc SEL."]
        #[must_use]
        #[inline(always)]
        pub const fn sel(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x07;
            val as u8
        }
        #[doc = "desc SEL."]
        #[inline(always)]
        pub const fn set_sel(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 0usize)) | (((val as u32) & 0x07) << 0usize);
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
    impl Default for Pc15Sel {
        #[inline(always)]
        fn default() -> Pc15Sel {
            Pc15Sel(0)
        }
    }
    impl core::fmt::Debug for Pc15Sel {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Pc15Sel")
                .field("sel", &self.sel())
                .field("rsv", &self.rsv())
                .finish()
        }
    }
    #[doc = "desc PC_ICLR."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct PcIclr(pub u32);
    impl PcIclr {
        #[doc = "desc PC00."]
        #[must_use]
        #[inline(always)]
        pub const fn pc00(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "desc PC00."]
        #[inline(always)]
        pub const fn set_pc00(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "desc PC01."]
        #[must_use]
        #[inline(always)]
        pub const fn pc01(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "desc PC01."]
        #[inline(always)]
        pub const fn set_pc01(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "desc PC02."]
        #[must_use]
        #[inline(always)]
        pub const fn pc02(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "desc PC02."]
        #[inline(always)]
        pub const fn set_pc02(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "desc PC03."]
        #[must_use]
        #[inline(always)]
        pub const fn pc03(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "desc PC03."]
        #[inline(always)]
        pub const fn set_pc03(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "desc PC04."]
        #[must_use]
        #[inline(always)]
        pub const fn pc04(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "desc PC04."]
        #[inline(always)]
        pub const fn set_pc04(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "desc PC05."]
        #[must_use]
        #[inline(always)]
        pub const fn pc05(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "desc PC05."]
        #[inline(always)]
        pub const fn set_pc05(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "desc PC06."]
        #[must_use]
        #[inline(always)]
        pub const fn pc06(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "desc PC06."]
        #[inline(always)]
        pub const fn set_pc06(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "desc PC07."]
        #[must_use]
        #[inline(always)]
        pub const fn pc07(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "desc PC07."]
        #[inline(always)]
        pub const fn set_pc07(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "desc PC08."]
        #[must_use]
        #[inline(always)]
        pub const fn pc08(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "desc PC08."]
        #[inline(always)]
        pub const fn set_pc08(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "desc PC09."]
        #[must_use]
        #[inline(always)]
        pub const fn pc09(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "desc PC09."]
        #[inline(always)]
        pub const fn set_pc09(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "desc PC10."]
        #[must_use]
        #[inline(always)]
        pub const fn pc10(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "desc PC10."]
        #[inline(always)]
        pub const fn set_pc10(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[doc = "desc PC11."]
        #[must_use]
        #[inline(always)]
        pub const fn pc11(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "desc PC11."]
        #[inline(always)]
        pub const fn set_pc11(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[doc = "desc PC12."]
        #[must_use]
        #[inline(always)]
        pub const fn pc12(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "desc PC12."]
        #[inline(always)]
        pub const fn set_pc12(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "desc PC13."]
        #[must_use]
        #[inline(always)]
        pub const fn pc13(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[doc = "desc PC13."]
        #[inline(always)]
        pub const fn set_pc13(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[doc = "desc PC14."]
        #[must_use]
        #[inline(always)]
        pub const fn pc14(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "desc PC14."]
        #[inline(always)]
        pub const fn set_pc14(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[doc = "desc PC15."]
        #[must_use]
        #[inline(always)]
        pub const fn pc15(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "desc PC15."]
        #[inline(always)]
        pub const fn set_pc15(&mut self, val: bool) {
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
    impl Default for PcIclr {
        #[inline(always)]
        fn default() -> PcIclr {
            PcIclr(0)
        }
    }
    impl core::fmt::Debug for PcIclr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("PcIclr")
                .field("pc00", &self.pc00())
                .field("pc01", &self.pc01())
                .field("pc02", &self.pc02())
                .field("pc03", &self.pc03())
                .field("pc04", &self.pc04())
                .field("pc05", &self.pc05())
                .field("pc06", &self.pc06())
                .field("pc07", &self.pc07())
                .field("pc08", &self.pc08())
                .field("pc09", &self.pc09())
                .field("pc10", &self.pc10())
                .field("pc11", &self.pc11())
                .field("pc12", &self.pc12())
                .field("pc13", &self.pc13())
                .field("pc14", &self.pc14())
                .field("pc15", &self.pc15())
                .field("rsv", &self.rsv())
                .finish()
        }
    }
    #[doc = "desc PC_STAT."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct PcStat(pub u32);
    impl PcStat {
        #[doc = "desc PC00."]
        #[must_use]
        #[inline(always)]
        pub const fn pc00(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "desc PC00."]
        #[inline(always)]
        pub const fn set_pc00(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "desc PC01."]
        #[must_use]
        #[inline(always)]
        pub const fn pc01(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "desc PC01."]
        #[inline(always)]
        pub const fn set_pc01(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "desc PC02."]
        #[must_use]
        #[inline(always)]
        pub const fn pc02(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "desc PC02."]
        #[inline(always)]
        pub const fn set_pc02(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "desc PC03."]
        #[must_use]
        #[inline(always)]
        pub const fn pc03(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "desc PC03."]
        #[inline(always)]
        pub const fn set_pc03(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "desc PC04."]
        #[must_use]
        #[inline(always)]
        pub const fn pc04(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "desc PC04."]
        #[inline(always)]
        pub const fn set_pc04(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "desc PC05."]
        #[must_use]
        #[inline(always)]
        pub const fn pc05(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "desc PC05."]
        #[inline(always)]
        pub const fn set_pc05(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "desc PC06."]
        #[must_use]
        #[inline(always)]
        pub const fn pc06(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "desc PC06."]
        #[inline(always)]
        pub const fn set_pc06(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "desc PC07."]
        #[must_use]
        #[inline(always)]
        pub const fn pc07(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "desc PC07."]
        #[inline(always)]
        pub const fn set_pc07(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "desc PC08."]
        #[must_use]
        #[inline(always)]
        pub const fn pc08(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "desc PC08."]
        #[inline(always)]
        pub const fn set_pc08(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "desc PC09."]
        #[must_use]
        #[inline(always)]
        pub const fn pc09(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "desc PC09."]
        #[inline(always)]
        pub const fn set_pc09(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "desc PC10."]
        #[must_use]
        #[inline(always)]
        pub const fn pc10(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "desc PC10."]
        #[inline(always)]
        pub const fn set_pc10(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[doc = "desc PC11."]
        #[must_use]
        #[inline(always)]
        pub const fn pc11(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "desc PC11."]
        #[inline(always)]
        pub const fn set_pc11(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[doc = "desc PC12."]
        #[must_use]
        #[inline(always)]
        pub const fn pc12(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "desc PC12."]
        #[inline(always)]
        pub const fn set_pc12(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "desc PC13."]
        #[must_use]
        #[inline(always)]
        pub const fn pc13(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[doc = "desc PC13."]
        #[inline(always)]
        pub const fn set_pc13(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[doc = "desc PC14."]
        #[must_use]
        #[inline(always)]
        pub const fn pc14(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "desc PC14."]
        #[inline(always)]
        pub const fn set_pc14(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[doc = "desc PC15."]
        #[must_use]
        #[inline(always)]
        pub const fn pc15(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "desc PC15."]
        #[inline(always)]
        pub const fn set_pc15(&mut self, val: bool) {
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
    impl Default for PcStat {
        #[inline(always)]
        fn default() -> PcStat {
            PcStat(0)
        }
    }
    impl core::fmt::Debug for PcStat {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("PcStat")
                .field("pc00", &self.pc00())
                .field("pc01", &self.pc01())
                .field("pc02", &self.pc02())
                .field("pc03", &self.pc03())
                .field("pc04", &self.pc04())
                .field("pc05", &self.pc05())
                .field("pc06", &self.pc06())
                .field("pc07", &self.pc07())
                .field("pc08", &self.pc08())
                .field("pc09", &self.pc09())
                .field("pc10", &self.pc10())
                .field("pc11", &self.pc11())
                .field("pc12", &self.pc12())
                .field("pc13", &self.pc13())
                .field("pc14", &self.pc14())
                .field("pc15", &self.pc15())
                .field("rsv", &self.rsv())
                .finish()
        }
    }
    #[doc = "desc PCADS."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Pcads(pub u32);
    impl Pcads {
        #[doc = "desc PC00."]
        #[must_use]
        #[inline(always)]
        pub const fn pc00(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "desc PC00."]
        #[inline(always)]
        pub const fn set_pc00(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "desc PC01."]
        #[must_use]
        #[inline(always)]
        pub const fn pc01(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "desc PC01."]
        #[inline(always)]
        pub const fn set_pc01(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "desc PC02."]
        #[must_use]
        #[inline(always)]
        pub const fn pc02(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "desc PC02."]
        #[inline(always)]
        pub const fn set_pc02(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "desc PC03."]
        #[must_use]
        #[inline(always)]
        pub const fn pc03(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "desc PC03."]
        #[inline(always)]
        pub const fn set_pc03(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "desc PC04."]
        #[must_use]
        #[inline(always)]
        pub const fn pc04(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "desc PC04."]
        #[inline(always)]
        pub const fn set_pc04(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "desc PC05."]
        #[must_use]
        #[inline(always)]
        pub const fn pc05(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "desc PC05."]
        #[inline(always)]
        pub const fn set_pc05(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "desc PC06."]
        #[must_use]
        #[inline(always)]
        pub const fn pc06(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "desc PC06."]
        #[inline(always)]
        pub const fn set_pc06(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "desc PC07."]
        #[must_use]
        #[inline(always)]
        pub const fn pc07(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "desc PC07."]
        #[inline(always)]
        pub const fn set_pc07(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "desc PC08."]
        #[must_use]
        #[inline(always)]
        pub const fn pc08(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "desc PC08."]
        #[inline(always)]
        pub const fn set_pc08(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "desc PC09."]
        #[must_use]
        #[inline(always)]
        pub const fn pc09(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "desc PC09."]
        #[inline(always)]
        pub const fn set_pc09(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "desc PC10."]
        #[must_use]
        #[inline(always)]
        pub const fn pc10(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "desc PC10."]
        #[inline(always)]
        pub const fn set_pc10(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[doc = "desc PC11."]
        #[must_use]
        #[inline(always)]
        pub const fn pc11(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "desc PC11."]
        #[inline(always)]
        pub const fn set_pc11(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[doc = "desc PC12."]
        #[must_use]
        #[inline(always)]
        pub const fn pc12(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "desc PC12."]
        #[inline(always)]
        pub const fn set_pc12(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "desc PC13."]
        #[must_use]
        #[inline(always)]
        pub const fn pc13(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[doc = "desc PC13."]
        #[inline(always)]
        pub const fn set_pc13(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[doc = "desc PC14."]
        #[must_use]
        #[inline(always)]
        pub const fn pc14(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "desc PC14."]
        #[inline(always)]
        pub const fn set_pc14(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[doc = "desc PC15."]
        #[must_use]
        #[inline(always)]
        pub const fn pc15(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "desc PC15."]
        #[inline(always)]
        pub const fn set_pc15(&mut self, val: bool) {
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
    impl Default for Pcads {
        #[inline(always)]
        fn default() -> Pcads {
            Pcads(0)
        }
    }
    impl core::fmt::Debug for Pcads {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Pcads")
                .field("pc00", &self.pc00())
                .field("pc01", &self.pc01())
                .field("pc02", &self.pc02())
                .field("pc03", &self.pc03())
                .field("pc04", &self.pc04())
                .field("pc05", &self.pc05())
                .field("pc06", &self.pc06())
                .field("pc07", &self.pc07())
                .field("pc08", &self.pc08())
                .field("pc09", &self.pc09())
                .field("pc10", &self.pc10())
                .field("pc11", &self.pc11())
                .field("pc12", &self.pc12())
                .field("pc13", &self.pc13())
                .field("pc14", &self.pc14())
                .field("pc15", &self.pc15())
                .field("rsv", &self.rsv())
                .finish()
        }
    }
    #[doc = "desc PCAS."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Pcas(pub u32);
    impl Pcas {
        #[doc = "desc PCA_CH0."]
        #[must_use]
        #[inline(always)]
        pub const fn pca_ch0(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x07;
            val as u8
        }
        #[doc = "desc PCA_CH0."]
        #[inline(always)]
        pub const fn set_pca_ch0(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 0usize)) | (((val as u32) & 0x07) << 0usize);
        }
        #[doc = "desc PCA_ECI."]
        #[must_use]
        #[inline(always)]
        pub const fn pca_eci(&self) -> u8 {
            let val = (self.0 >> 3usize) & 0x07;
            val as u8
        }
        #[doc = "desc PCA_ECI."]
        #[inline(always)]
        pub const fn set_pca_eci(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 3usize)) | (((val as u32) & 0x07) << 3usize);
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
    impl Default for Pcas {
        #[inline(always)]
        fn default() -> Pcas {
            Pcas(0)
        }
    }
    impl core::fmt::Debug for Pcas {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Pcas")
                .field("pca_ch0", &self.pca_ch0())
                .field("pca_eci", &self.pca_eci())
                .field("rsv", &self.rsv())
                .finish()
        }
    }
    #[doc = "desc PCBCLR."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Pcbclr(pub u32);
    impl Pcbclr {
        #[doc = "desc PC00."]
        #[must_use]
        #[inline(always)]
        pub const fn pc00(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "desc PC00."]
        #[inline(always)]
        pub const fn set_pc00(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "desc PC01."]
        #[must_use]
        #[inline(always)]
        pub const fn pc01(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "desc PC01."]
        #[inline(always)]
        pub const fn set_pc01(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "desc PC02."]
        #[must_use]
        #[inline(always)]
        pub const fn pc02(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "desc PC02."]
        #[inline(always)]
        pub const fn set_pc02(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "desc PC03."]
        #[must_use]
        #[inline(always)]
        pub const fn pc03(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "desc PC03."]
        #[inline(always)]
        pub const fn set_pc03(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "desc PC04."]
        #[must_use]
        #[inline(always)]
        pub const fn pc04(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "desc PC04."]
        #[inline(always)]
        pub const fn set_pc04(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "desc PC05."]
        #[must_use]
        #[inline(always)]
        pub const fn pc05(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "desc PC05."]
        #[inline(always)]
        pub const fn set_pc05(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "desc PC06."]
        #[must_use]
        #[inline(always)]
        pub const fn pc06(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "desc PC06."]
        #[inline(always)]
        pub const fn set_pc06(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "desc PC07."]
        #[must_use]
        #[inline(always)]
        pub const fn pc07(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "desc PC07."]
        #[inline(always)]
        pub const fn set_pc07(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "desc PC08."]
        #[must_use]
        #[inline(always)]
        pub const fn pc08(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "desc PC08."]
        #[inline(always)]
        pub const fn set_pc08(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "desc PC09."]
        #[must_use]
        #[inline(always)]
        pub const fn pc09(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "desc PC09."]
        #[inline(always)]
        pub const fn set_pc09(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "desc PC10."]
        #[must_use]
        #[inline(always)]
        pub const fn pc10(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "desc PC10."]
        #[inline(always)]
        pub const fn set_pc10(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[doc = "desc PC11."]
        #[must_use]
        #[inline(always)]
        pub const fn pc11(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "desc PC11."]
        #[inline(always)]
        pub const fn set_pc11(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[doc = "desc PC12."]
        #[must_use]
        #[inline(always)]
        pub const fn pc12(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "desc PC12."]
        #[inline(always)]
        pub const fn set_pc12(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "desc PC13."]
        #[must_use]
        #[inline(always)]
        pub const fn pc13(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[doc = "desc PC13."]
        #[inline(always)]
        pub const fn set_pc13(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[doc = "desc PC14."]
        #[must_use]
        #[inline(always)]
        pub const fn pc14(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "desc PC14."]
        #[inline(always)]
        pub const fn set_pc14(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[doc = "desc PC15."]
        #[must_use]
        #[inline(always)]
        pub const fn pc15(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "desc PC15."]
        #[inline(always)]
        pub const fn set_pc15(&mut self, val: bool) {
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
    impl Default for Pcbclr {
        #[inline(always)]
        fn default() -> Pcbclr {
            Pcbclr(0)
        }
    }
    impl core::fmt::Debug for Pcbclr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Pcbclr")
                .field("pc00", &self.pc00())
                .field("pc01", &self.pc01())
                .field("pc02", &self.pc02())
                .field("pc03", &self.pc03())
                .field("pc04", &self.pc04())
                .field("pc05", &self.pc05())
                .field("pc06", &self.pc06())
                .field("pc07", &self.pc07())
                .field("pc08", &self.pc08())
                .field("pc09", &self.pc09())
                .field("pc10", &self.pc10())
                .field("pc11", &self.pc11())
                .field("pc12", &self.pc12())
                .field("pc13", &self.pc13())
                .field("pc14", &self.pc14())
                .field("pc15", &self.pc15())
                .field("rsv", &self.rsv())
                .finish()
        }
    }
    #[doc = "desc PCBSET."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Pcbset(pub u32);
    impl Pcbset {
        #[doc = "desc PC00."]
        #[must_use]
        #[inline(always)]
        pub const fn pc00(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "desc PC00."]
        #[inline(always)]
        pub const fn set_pc00(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "desc PC01."]
        #[must_use]
        #[inline(always)]
        pub const fn pc01(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "desc PC01."]
        #[inline(always)]
        pub const fn set_pc01(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "desc PC02."]
        #[must_use]
        #[inline(always)]
        pub const fn pc02(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "desc PC02."]
        #[inline(always)]
        pub const fn set_pc02(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "desc PC03."]
        #[must_use]
        #[inline(always)]
        pub const fn pc03(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "desc PC03."]
        #[inline(always)]
        pub const fn set_pc03(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "desc PC04."]
        #[must_use]
        #[inline(always)]
        pub const fn pc04(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "desc PC04."]
        #[inline(always)]
        pub const fn set_pc04(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "desc PC05."]
        #[must_use]
        #[inline(always)]
        pub const fn pc05(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "desc PC05."]
        #[inline(always)]
        pub const fn set_pc05(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "desc PC06."]
        #[must_use]
        #[inline(always)]
        pub const fn pc06(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "desc PC06."]
        #[inline(always)]
        pub const fn set_pc06(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "desc PC07."]
        #[must_use]
        #[inline(always)]
        pub const fn pc07(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "desc PC07."]
        #[inline(always)]
        pub const fn set_pc07(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "desc PC08."]
        #[must_use]
        #[inline(always)]
        pub const fn pc08(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "desc PC08."]
        #[inline(always)]
        pub const fn set_pc08(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "desc PC09."]
        #[must_use]
        #[inline(always)]
        pub const fn pc09(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "desc PC09."]
        #[inline(always)]
        pub const fn set_pc09(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "desc PC10."]
        #[must_use]
        #[inline(always)]
        pub const fn pc10(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "desc PC10."]
        #[inline(always)]
        pub const fn set_pc10(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[doc = "desc PC11."]
        #[must_use]
        #[inline(always)]
        pub const fn pc11(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "desc PC11."]
        #[inline(always)]
        pub const fn set_pc11(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[doc = "desc PC12."]
        #[must_use]
        #[inline(always)]
        pub const fn pc12(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "desc PC12."]
        #[inline(always)]
        pub const fn set_pc12(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "desc PC13."]
        #[must_use]
        #[inline(always)]
        pub const fn pc13(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[doc = "desc PC13."]
        #[inline(always)]
        pub const fn set_pc13(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[doc = "desc PC14."]
        #[must_use]
        #[inline(always)]
        pub const fn pc14(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "desc PC14."]
        #[inline(always)]
        pub const fn set_pc14(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[doc = "desc PC15."]
        #[must_use]
        #[inline(always)]
        pub const fn pc15(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "desc PC15."]
        #[inline(always)]
        pub const fn set_pc15(&mut self, val: bool) {
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
    impl Default for Pcbset {
        #[inline(always)]
        fn default() -> Pcbset {
            Pcbset(0)
        }
    }
    impl core::fmt::Debug for Pcbset {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Pcbset")
                .field("pc00", &self.pc00())
                .field("pc01", &self.pc01())
                .field("pc02", &self.pc02())
                .field("pc03", &self.pc03())
                .field("pc04", &self.pc04())
                .field("pc05", &self.pc05())
                .field("pc06", &self.pc06())
                .field("pc07", &self.pc07())
                .field("pc08", &self.pc08())
                .field("pc09", &self.pc09())
                .field("pc10", &self.pc10())
                .field("pc11", &self.pc11())
                .field("pc12", &self.pc12())
                .field("pc13", &self.pc13())
                .field("pc14", &self.pc14())
                .field("pc15", &self.pc15())
                .field("rsv", &self.rsv())
                .finish()
        }
    }
    #[doc = "desc PCBSETCLR."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Pcbsetclr(pub u32);
    impl Pcbsetclr {
        #[doc = "desc PCBCLR."]
        #[must_use]
        #[inline(always)]
        pub const fn pcbclr(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[doc = "desc PCBCLR."]
        #[inline(always)]
        pub const fn set_pcbclr(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
        }
        #[doc = "desc PCBSET."]
        #[must_use]
        #[inline(always)]
        pub const fn pcbset(&self) -> u16 {
            let val = (self.0 >> 16usize) & 0xffff;
            val as u16
        }
        #[doc = "desc PCBSET."]
        #[inline(always)]
        pub const fn set_pcbset(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 16usize)) | (((val as u32) & 0xffff) << 16usize);
        }
    }
    impl Default for Pcbsetclr {
        #[inline(always)]
        fn default() -> Pcbsetclr {
            Pcbsetclr(0)
        }
    }
    impl core::fmt::Debug for Pcbsetclr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Pcbsetclr")
                .field("pcbclr", &self.pcbclr())
                .field("pcbset", &self.pcbset())
                .finish()
        }
    }
    #[doc = "desc PCDIR."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Pcdir(pub u32);
    impl Pcdir {
        #[doc = "desc PC00."]
        #[must_use]
        #[inline(always)]
        pub const fn pc00(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "desc PC00."]
        #[inline(always)]
        pub const fn set_pc00(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "desc PC01."]
        #[must_use]
        #[inline(always)]
        pub const fn pc01(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "desc PC01."]
        #[inline(always)]
        pub const fn set_pc01(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "desc PC02."]
        #[must_use]
        #[inline(always)]
        pub const fn pc02(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "desc PC02."]
        #[inline(always)]
        pub const fn set_pc02(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "desc PC03."]
        #[must_use]
        #[inline(always)]
        pub const fn pc03(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "desc PC03."]
        #[inline(always)]
        pub const fn set_pc03(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "desc PC04."]
        #[must_use]
        #[inline(always)]
        pub const fn pc04(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "desc PC04."]
        #[inline(always)]
        pub const fn set_pc04(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "desc PC05."]
        #[must_use]
        #[inline(always)]
        pub const fn pc05(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "desc PC05."]
        #[inline(always)]
        pub const fn set_pc05(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "desc PC06."]
        #[must_use]
        #[inline(always)]
        pub const fn pc06(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "desc PC06."]
        #[inline(always)]
        pub const fn set_pc06(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "desc PC07."]
        #[must_use]
        #[inline(always)]
        pub const fn pc07(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "desc PC07."]
        #[inline(always)]
        pub const fn set_pc07(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "desc PC08."]
        #[must_use]
        #[inline(always)]
        pub const fn pc08(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "desc PC08."]
        #[inline(always)]
        pub const fn set_pc08(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "desc PC09."]
        #[must_use]
        #[inline(always)]
        pub const fn pc09(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "desc PC09."]
        #[inline(always)]
        pub const fn set_pc09(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "desc PC10."]
        #[must_use]
        #[inline(always)]
        pub const fn pc10(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "desc PC10."]
        #[inline(always)]
        pub const fn set_pc10(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[doc = "desc PC11."]
        #[must_use]
        #[inline(always)]
        pub const fn pc11(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "desc PC11."]
        #[inline(always)]
        pub const fn set_pc11(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[doc = "desc PC12."]
        #[must_use]
        #[inline(always)]
        pub const fn pc12(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "desc PC12."]
        #[inline(always)]
        pub const fn set_pc12(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "desc PC13."]
        #[must_use]
        #[inline(always)]
        pub const fn pc13(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[doc = "desc PC13."]
        #[inline(always)]
        pub const fn set_pc13(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[doc = "desc PC14."]
        #[must_use]
        #[inline(always)]
        pub const fn pc14(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "desc PC14."]
        #[inline(always)]
        pub const fn set_pc14(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[doc = "desc PC15."]
        #[must_use]
        #[inline(always)]
        pub const fn pc15(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "desc PC15."]
        #[inline(always)]
        pub const fn set_pc15(&mut self, val: bool) {
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
    impl Default for Pcdir {
        #[inline(always)]
        fn default() -> Pcdir {
            Pcdir(0)
        }
    }
    impl core::fmt::Debug for Pcdir {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Pcdir")
                .field("pc00", &self.pc00())
                .field("pc01", &self.pc01())
                .field("pc02", &self.pc02())
                .field("pc03", &self.pc03())
                .field("pc04", &self.pc04())
                .field("pc05", &self.pc05())
                .field("pc06", &self.pc06())
                .field("pc07", &self.pc07())
                .field("pc08", &self.pc08())
                .field("pc09", &self.pc09())
                .field("pc10", &self.pc10())
                .field("pc11", &self.pc11())
                .field("pc12", &self.pc12())
                .field("pc13", &self.pc13())
                .field("pc14", &self.pc14())
                .field("pc15", &self.pc15())
                .field("rsv", &self.rsv())
                .finish()
        }
    }
    #[doc = "desc PCDR."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Pcdr(pub u32);
    impl Pcdr {
        #[doc = "desc PC00."]
        #[must_use]
        #[inline(always)]
        pub const fn pc00(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "desc PC00."]
        #[inline(always)]
        pub const fn set_pc00(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "desc PC01."]
        #[must_use]
        #[inline(always)]
        pub const fn pc01(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "desc PC01."]
        #[inline(always)]
        pub const fn set_pc01(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "desc PC02."]
        #[must_use]
        #[inline(always)]
        pub const fn pc02(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "desc PC02."]
        #[inline(always)]
        pub const fn set_pc02(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "desc PC03."]
        #[must_use]
        #[inline(always)]
        pub const fn pc03(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "desc PC03."]
        #[inline(always)]
        pub const fn set_pc03(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "desc PC04."]
        #[must_use]
        #[inline(always)]
        pub const fn pc04(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "desc PC04."]
        #[inline(always)]
        pub const fn set_pc04(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "desc PC05."]
        #[must_use]
        #[inline(always)]
        pub const fn pc05(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "desc PC05."]
        #[inline(always)]
        pub const fn set_pc05(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "desc PC06."]
        #[must_use]
        #[inline(always)]
        pub const fn pc06(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "desc PC06."]
        #[inline(always)]
        pub const fn set_pc06(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "desc PC07."]
        #[must_use]
        #[inline(always)]
        pub const fn pc07(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "desc PC07."]
        #[inline(always)]
        pub const fn set_pc07(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "desc PC08."]
        #[must_use]
        #[inline(always)]
        pub const fn pc08(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "desc PC08."]
        #[inline(always)]
        pub const fn set_pc08(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "desc PC09."]
        #[must_use]
        #[inline(always)]
        pub const fn pc09(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "desc PC09."]
        #[inline(always)]
        pub const fn set_pc09(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "desc PC10."]
        #[must_use]
        #[inline(always)]
        pub const fn pc10(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "desc PC10."]
        #[inline(always)]
        pub const fn set_pc10(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[doc = "desc PC11."]
        #[must_use]
        #[inline(always)]
        pub const fn pc11(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "desc PC11."]
        #[inline(always)]
        pub const fn set_pc11(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[doc = "desc PC12."]
        #[must_use]
        #[inline(always)]
        pub const fn pc12(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "desc PC12."]
        #[inline(always)]
        pub const fn set_pc12(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "desc PC13."]
        #[must_use]
        #[inline(always)]
        pub const fn pc13(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[doc = "desc PC13."]
        #[inline(always)]
        pub const fn set_pc13(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[doc = "desc PC14."]
        #[must_use]
        #[inline(always)]
        pub const fn pc14(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "desc PC14."]
        #[inline(always)]
        pub const fn set_pc14(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[doc = "desc PC15."]
        #[must_use]
        #[inline(always)]
        pub const fn pc15(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "desc PC15."]
        #[inline(always)]
        pub const fn set_pc15(&mut self, val: bool) {
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
    impl Default for Pcdr {
        #[inline(always)]
        fn default() -> Pcdr {
            Pcdr(0)
        }
    }
    impl core::fmt::Debug for Pcdr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Pcdr")
                .field("pc00", &self.pc00())
                .field("pc01", &self.pc01())
                .field("pc02", &self.pc02())
                .field("pc03", &self.pc03())
                .field("pc04", &self.pc04())
                .field("pc05", &self.pc05())
                .field("pc06", &self.pc06())
                .field("pc07", &self.pc07())
                .field("pc08", &self.pc08())
                .field("pc09", &self.pc09())
                .field("pc10", &self.pc10())
                .field("pc11", &self.pc11())
                .field("pc12", &self.pc12())
                .field("pc13", &self.pc13())
                .field("pc14", &self.pc14())
                .field("pc15", &self.pc15())
                .field("rsv", &self.rsv())
                .finish()
        }
    }
    #[doc = "desc PCFIE."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Pcfie(pub u32);
    impl Pcfie {
        #[doc = "desc PC00."]
        #[must_use]
        #[inline(always)]
        pub const fn pc00(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "desc PC00."]
        #[inline(always)]
        pub const fn set_pc00(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "desc PC01."]
        #[must_use]
        #[inline(always)]
        pub const fn pc01(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "desc PC01."]
        #[inline(always)]
        pub const fn set_pc01(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "desc PC02."]
        #[must_use]
        #[inline(always)]
        pub const fn pc02(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "desc PC02."]
        #[inline(always)]
        pub const fn set_pc02(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "desc PC03."]
        #[must_use]
        #[inline(always)]
        pub const fn pc03(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "desc PC03."]
        #[inline(always)]
        pub const fn set_pc03(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "desc PC04."]
        #[must_use]
        #[inline(always)]
        pub const fn pc04(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "desc PC04."]
        #[inline(always)]
        pub const fn set_pc04(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "desc PC05."]
        #[must_use]
        #[inline(always)]
        pub const fn pc05(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "desc PC05."]
        #[inline(always)]
        pub const fn set_pc05(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "desc PC06."]
        #[must_use]
        #[inline(always)]
        pub const fn pc06(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "desc PC06."]
        #[inline(always)]
        pub const fn set_pc06(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "desc PC07."]
        #[must_use]
        #[inline(always)]
        pub const fn pc07(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "desc PC07."]
        #[inline(always)]
        pub const fn set_pc07(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "desc PC08."]
        #[must_use]
        #[inline(always)]
        pub const fn pc08(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "desc PC08."]
        #[inline(always)]
        pub const fn set_pc08(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "desc PC09."]
        #[must_use]
        #[inline(always)]
        pub const fn pc09(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "desc PC09."]
        #[inline(always)]
        pub const fn set_pc09(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "desc PC10."]
        #[must_use]
        #[inline(always)]
        pub const fn pc10(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "desc PC10."]
        #[inline(always)]
        pub const fn set_pc10(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[doc = "desc PC11."]
        #[must_use]
        #[inline(always)]
        pub const fn pc11(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "desc PC11."]
        #[inline(always)]
        pub const fn set_pc11(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[doc = "desc PC12."]
        #[must_use]
        #[inline(always)]
        pub const fn pc12(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "desc PC12."]
        #[inline(always)]
        pub const fn set_pc12(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "desc PC13."]
        #[must_use]
        #[inline(always)]
        pub const fn pc13(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[doc = "desc PC13."]
        #[inline(always)]
        pub const fn set_pc13(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[doc = "desc PC14."]
        #[must_use]
        #[inline(always)]
        pub const fn pc14(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "desc PC14."]
        #[inline(always)]
        pub const fn set_pc14(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[doc = "desc PC15."]
        #[must_use]
        #[inline(always)]
        pub const fn pc15(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "desc PC15."]
        #[inline(always)]
        pub const fn set_pc15(&mut self, val: bool) {
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
    impl Default for Pcfie {
        #[inline(always)]
        fn default() -> Pcfie {
            Pcfie(0)
        }
    }
    impl core::fmt::Debug for Pcfie {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Pcfie")
                .field("pc00", &self.pc00())
                .field("pc01", &self.pc01())
                .field("pc02", &self.pc02())
                .field("pc03", &self.pc03())
                .field("pc04", &self.pc04())
                .field("pc05", &self.pc05())
                .field("pc06", &self.pc06())
                .field("pc07", &self.pc07())
                .field("pc08", &self.pc08())
                .field("pc09", &self.pc09())
                .field("pc10", &self.pc10())
                .field("pc11", &self.pc11())
                .field("pc12", &self.pc12())
                .field("pc13", &self.pc13())
                .field("pc14", &self.pc14())
                .field("pc15", &self.pc15())
                .field("rsv", &self.rsv())
                .finish()
        }
    }
    #[doc = "desc PCHIE."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Pchie(pub u32);
    impl Pchie {
        #[doc = "desc PC00."]
        #[must_use]
        #[inline(always)]
        pub const fn pc00(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "desc PC00."]
        #[inline(always)]
        pub const fn set_pc00(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "desc PC01."]
        #[must_use]
        #[inline(always)]
        pub const fn pc01(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "desc PC01."]
        #[inline(always)]
        pub const fn set_pc01(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "desc PC02."]
        #[must_use]
        #[inline(always)]
        pub const fn pc02(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "desc PC02."]
        #[inline(always)]
        pub const fn set_pc02(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "desc PC03."]
        #[must_use]
        #[inline(always)]
        pub const fn pc03(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "desc PC03."]
        #[inline(always)]
        pub const fn set_pc03(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "desc PC04."]
        #[must_use]
        #[inline(always)]
        pub const fn pc04(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "desc PC04."]
        #[inline(always)]
        pub const fn set_pc04(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "desc PC05."]
        #[must_use]
        #[inline(always)]
        pub const fn pc05(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "desc PC05."]
        #[inline(always)]
        pub const fn set_pc05(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "desc PC06."]
        #[must_use]
        #[inline(always)]
        pub const fn pc06(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "desc PC06."]
        #[inline(always)]
        pub const fn set_pc06(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "desc PC07."]
        #[must_use]
        #[inline(always)]
        pub const fn pc07(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "desc PC07."]
        #[inline(always)]
        pub const fn set_pc07(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "desc PC08."]
        #[must_use]
        #[inline(always)]
        pub const fn pc08(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "desc PC08."]
        #[inline(always)]
        pub const fn set_pc08(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "desc PC09."]
        #[must_use]
        #[inline(always)]
        pub const fn pc09(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "desc PC09."]
        #[inline(always)]
        pub const fn set_pc09(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "desc PC10."]
        #[must_use]
        #[inline(always)]
        pub const fn pc10(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "desc PC10."]
        #[inline(always)]
        pub const fn set_pc10(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[doc = "desc PC11."]
        #[must_use]
        #[inline(always)]
        pub const fn pc11(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "desc PC11."]
        #[inline(always)]
        pub const fn set_pc11(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[doc = "desc PC12."]
        #[must_use]
        #[inline(always)]
        pub const fn pc12(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "desc PC12."]
        #[inline(always)]
        pub const fn set_pc12(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "desc PC13."]
        #[must_use]
        #[inline(always)]
        pub const fn pc13(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[doc = "desc PC13."]
        #[inline(always)]
        pub const fn set_pc13(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[doc = "desc PC14."]
        #[must_use]
        #[inline(always)]
        pub const fn pc14(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "desc PC14."]
        #[inline(always)]
        pub const fn set_pc14(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[doc = "desc PC15."]
        #[must_use]
        #[inline(always)]
        pub const fn pc15(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "desc PC15."]
        #[inline(always)]
        pub const fn set_pc15(&mut self, val: bool) {
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
    impl Default for Pchie {
        #[inline(always)]
        fn default() -> Pchie {
            Pchie(0)
        }
    }
    impl core::fmt::Debug for Pchie {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Pchie")
                .field("pc00", &self.pc00())
                .field("pc01", &self.pc01())
                .field("pc02", &self.pc02())
                .field("pc03", &self.pc03())
                .field("pc04", &self.pc04())
                .field("pc05", &self.pc05())
                .field("pc06", &self.pc06())
                .field("pc07", &self.pc07())
                .field("pc08", &self.pc08())
                .field("pc09", &self.pc09())
                .field("pc10", &self.pc10())
                .field("pc11", &self.pc11())
                .field("pc12", &self.pc12())
                .field("pc13", &self.pc13())
                .field("pc14", &self.pc14())
                .field("pc15", &self.pc15())
                .field("rsv", &self.rsv())
                .finish()
        }
    }
    #[doc = "desc PCIN."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Pcin(pub u32);
    impl Pcin {
        #[doc = "desc PC00."]
        #[must_use]
        #[inline(always)]
        pub const fn pc00(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "desc PC00."]
        #[inline(always)]
        pub const fn set_pc00(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "desc PC01."]
        #[must_use]
        #[inline(always)]
        pub const fn pc01(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "desc PC01."]
        #[inline(always)]
        pub const fn set_pc01(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "desc PC02."]
        #[must_use]
        #[inline(always)]
        pub const fn pc02(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "desc PC02."]
        #[inline(always)]
        pub const fn set_pc02(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "desc PC03."]
        #[must_use]
        #[inline(always)]
        pub const fn pc03(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "desc PC03."]
        #[inline(always)]
        pub const fn set_pc03(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "desc PC04."]
        #[must_use]
        #[inline(always)]
        pub const fn pc04(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "desc PC04."]
        #[inline(always)]
        pub const fn set_pc04(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "desc PC05."]
        #[must_use]
        #[inline(always)]
        pub const fn pc05(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "desc PC05."]
        #[inline(always)]
        pub const fn set_pc05(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "desc PC06."]
        #[must_use]
        #[inline(always)]
        pub const fn pc06(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "desc PC06."]
        #[inline(always)]
        pub const fn set_pc06(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "desc PC07."]
        #[must_use]
        #[inline(always)]
        pub const fn pc07(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "desc PC07."]
        #[inline(always)]
        pub const fn set_pc07(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "desc PC08."]
        #[must_use]
        #[inline(always)]
        pub const fn pc08(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "desc PC08."]
        #[inline(always)]
        pub const fn set_pc08(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "desc PC09."]
        #[must_use]
        #[inline(always)]
        pub const fn pc09(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "desc PC09."]
        #[inline(always)]
        pub const fn set_pc09(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "desc PC10."]
        #[must_use]
        #[inline(always)]
        pub const fn pc10(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "desc PC10."]
        #[inline(always)]
        pub const fn set_pc10(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[doc = "desc PC11."]
        #[must_use]
        #[inline(always)]
        pub const fn pc11(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "desc PC11."]
        #[inline(always)]
        pub const fn set_pc11(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[doc = "desc PC12."]
        #[must_use]
        #[inline(always)]
        pub const fn pc12(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "desc PC12."]
        #[inline(always)]
        pub const fn set_pc12(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "desc PC13."]
        #[must_use]
        #[inline(always)]
        pub const fn pc13(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[doc = "desc PC13."]
        #[inline(always)]
        pub const fn set_pc13(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[doc = "desc PC14."]
        #[must_use]
        #[inline(always)]
        pub const fn pc14(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "desc PC14."]
        #[inline(always)]
        pub const fn set_pc14(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[doc = "desc PC15."]
        #[must_use]
        #[inline(always)]
        pub const fn pc15(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "desc PC15."]
        #[inline(always)]
        pub const fn set_pc15(&mut self, val: bool) {
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
    impl Default for Pcin {
        #[inline(always)]
        fn default() -> Pcin {
            Pcin(0)
        }
    }
    impl core::fmt::Debug for Pcin {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Pcin")
                .field("pc00", &self.pc00())
                .field("pc01", &self.pc01())
                .field("pc02", &self.pc02())
                .field("pc03", &self.pc03())
                .field("pc04", &self.pc04())
                .field("pc05", &self.pc05())
                .field("pc06", &self.pc06())
                .field("pc07", &self.pc07())
                .field("pc08", &self.pc08())
                .field("pc09", &self.pc09())
                .field("pc10", &self.pc10())
                .field("pc11", &self.pc11())
                .field("pc12", &self.pc12())
                .field("pc13", &self.pc13())
                .field("pc14", &self.pc14())
                .field("pc15", &self.pc15())
                .field("rsv", &self.rsv())
                .finish()
        }
    }
    #[doc = "desc PCLIE."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Pclie(pub u32);
    impl Pclie {
        #[doc = "desc PC00."]
        #[must_use]
        #[inline(always)]
        pub const fn pc00(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "desc PC00."]
        #[inline(always)]
        pub const fn set_pc00(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "desc PC01."]
        #[must_use]
        #[inline(always)]
        pub const fn pc01(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "desc PC01."]
        #[inline(always)]
        pub const fn set_pc01(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "desc PC02."]
        #[must_use]
        #[inline(always)]
        pub const fn pc02(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "desc PC02."]
        #[inline(always)]
        pub const fn set_pc02(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "desc PC03."]
        #[must_use]
        #[inline(always)]
        pub const fn pc03(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "desc PC03."]
        #[inline(always)]
        pub const fn set_pc03(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "desc PC04."]
        #[must_use]
        #[inline(always)]
        pub const fn pc04(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "desc PC04."]
        #[inline(always)]
        pub const fn set_pc04(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "desc PC05."]
        #[must_use]
        #[inline(always)]
        pub const fn pc05(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "desc PC05."]
        #[inline(always)]
        pub const fn set_pc05(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "desc PC06."]
        #[must_use]
        #[inline(always)]
        pub const fn pc06(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "desc PC06."]
        #[inline(always)]
        pub const fn set_pc06(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "desc PC07."]
        #[must_use]
        #[inline(always)]
        pub const fn pc07(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "desc PC07."]
        #[inline(always)]
        pub const fn set_pc07(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "desc PC08."]
        #[must_use]
        #[inline(always)]
        pub const fn pc08(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "desc PC08."]
        #[inline(always)]
        pub const fn set_pc08(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "desc PC09."]
        #[must_use]
        #[inline(always)]
        pub const fn pc09(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "desc PC09."]
        #[inline(always)]
        pub const fn set_pc09(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "desc PC10."]
        #[must_use]
        #[inline(always)]
        pub const fn pc10(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "desc PC10."]
        #[inline(always)]
        pub const fn set_pc10(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[doc = "desc PC11."]
        #[must_use]
        #[inline(always)]
        pub const fn pc11(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "desc PC11."]
        #[inline(always)]
        pub const fn set_pc11(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[doc = "desc PC12."]
        #[must_use]
        #[inline(always)]
        pub const fn pc12(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "desc PC12."]
        #[inline(always)]
        pub const fn set_pc12(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "desc PC13."]
        #[must_use]
        #[inline(always)]
        pub const fn pc13(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[doc = "desc PC13."]
        #[inline(always)]
        pub const fn set_pc13(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[doc = "desc PC14."]
        #[must_use]
        #[inline(always)]
        pub const fn pc14(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "desc PC14."]
        #[inline(always)]
        pub const fn set_pc14(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[doc = "desc PC15."]
        #[must_use]
        #[inline(always)]
        pub const fn pc15(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "desc PC15."]
        #[inline(always)]
        pub const fn set_pc15(&mut self, val: bool) {
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
    impl Default for Pclie {
        #[inline(always)]
        fn default() -> Pclie {
            Pclie(0)
        }
    }
    impl core::fmt::Debug for Pclie {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Pclie")
                .field("pc00", &self.pc00())
                .field("pc01", &self.pc01())
                .field("pc02", &self.pc02())
                .field("pc03", &self.pc03())
                .field("pc04", &self.pc04())
                .field("pc05", &self.pc05())
                .field("pc06", &self.pc06())
                .field("pc07", &self.pc07())
                .field("pc08", &self.pc08())
                .field("pc09", &self.pc09())
                .field("pc10", &self.pc10())
                .field("pc11", &self.pc11())
                .field("pc12", &self.pc12())
                .field("pc13", &self.pc13())
                .field("pc14", &self.pc14())
                .field("pc15", &self.pc15())
                .field("rsv", &self.rsv())
                .finish()
        }
    }
    #[doc = "desc PCOD."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Pcod(pub u32);
    impl Pcod {
        #[doc = "desc PC00."]
        #[must_use]
        #[inline(always)]
        pub const fn pc00(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "desc PC00."]
        #[inline(always)]
        pub const fn set_pc00(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "desc PC01."]
        #[must_use]
        #[inline(always)]
        pub const fn pc01(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "desc PC01."]
        #[inline(always)]
        pub const fn set_pc01(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "desc PC02."]
        #[must_use]
        #[inline(always)]
        pub const fn pc02(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "desc PC02."]
        #[inline(always)]
        pub const fn set_pc02(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "desc PC03."]
        #[must_use]
        #[inline(always)]
        pub const fn pc03(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "desc PC03."]
        #[inline(always)]
        pub const fn set_pc03(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "desc PC04."]
        #[must_use]
        #[inline(always)]
        pub const fn pc04(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "desc PC04."]
        #[inline(always)]
        pub const fn set_pc04(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "desc PC05."]
        #[must_use]
        #[inline(always)]
        pub const fn pc05(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "desc PC05."]
        #[inline(always)]
        pub const fn set_pc05(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "desc PC06."]
        #[must_use]
        #[inline(always)]
        pub const fn pc06(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "desc PC06."]
        #[inline(always)]
        pub const fn set_pc06(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "desc PC07."]
        #[must_use]
        #[inline(always)]
        pub const fn pc07(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "desc PC07."]
        #[inline(always)]
        pub const fn set_pc07(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "desc PC08."]
        #[must_use]
        #[inline(always)]
        pub const fn pc08(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "desc PC08."]
        #[inline(always)]
        pub const fn set_pc08(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "desc PC09."]
        #[must_use]
        #[inline(always)]
        pub const fn pc09(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "desc PC09."]
        #[inline(always)]
        pub const fn set_pc09(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "desc PC10."]
        #[must_use]
        #[inline(always)]
        pub const fn pc10(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "desc PC10."]
        #[inline(always)]
        pub const fn set_pc10(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[doc = "desc PC11."]
        #[must_use]
        #[inline(always)]
        pub const fn pc11(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "desc PC11."]
        #[inline(always)]
        pub const fn set_pc11(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[doc = "desc PC12."]
        #[must_use]
        #[inline(always)]
        pub const fn pc12(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "desc PC12."]
        #[inline(always)]
        pub const fn set_pc12(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "desc PC13."]
        #[must_use]
        #[inline(always)]
        pub const fn pc13(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[doc = "desc PC13."]
        #[inline(always)]
        pub const fn set_pc13(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[doc = "desc PC14."]
        #[must_use]
        #[inline(always)]
        pub const fn pc14(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "desc PC14."]
        #[inline(always)]
        pub const fn set_pc14(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[doc = "desc PC15."]
        #[must_use]
        #[inline(always)]
        pub const fn pc15(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "desc PC15."]
        #[inline(always)]
        pub const fn set_pc15(&mut self, val: bool) {
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
    impl Default for Pcod {
        #[inline(always)]
        fn default() -> Pcod {
            Pcod(0)
        }
    }
    impl core::fmt::Debug for Pcod {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Pcod")
                .field("pc00", &self.pc00())
                .field("pc01", &self.pc01())
                .field("pc02", &self.pc02())
                .field("pc03", &self.pc03())
                .field("pc04", &self.pc04())
                .field("pc05", &self.pc05())
                .field("pc06", &self.pc06())
                .field("pc07", &self.pc07())
                .field("pc08", &self.pc08())
                .field("pc09", &self.pc09())
                .field("pc10", &self.pc10())
                .field("pc11", &self.pc11())
                .field("pc12", &self.pc12())
                .field("pc13", &self.pc13())
                .field("pc14", &self.pc14())
                .field("pc15", &self.pc15())
                .field("rsv", &self.rsv())
                .finish()
        }
    }
    #[doc = "desc PCOUT."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Pcout(pub u32);
    impl Pcout {
        #[doc = "desc PC00."]
        #[must_use]
        #[inline(always)]
        pub const fn pc00(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "desc PC00."]
        #[inline(always)]
        pub const fn set_pc00(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "desc PC01."]
        #[must_use]
        #[inline(always)]
        pub const fn pc01(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "desc PC01."]
        #[inline(always)]
        pub const fn set_pc01(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "desc PC02."]
        #[must_use]
        #[inline(always)]
        pub const fn pc02(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "desc PC02."]
        #[inline(always)]
        pub const fn set_pc02(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "desc PC03."]
        #[must_use]
        #[inline(always)]
        pub const fn pc03(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "desc PC03."]
        #[inline(always)]
        pub const fn set_pc03(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "desc PC04."]
        #[must_use]
        #[inline(always)]
        pub const fn pc04(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "desc PC04."]
        #[inline(always)]
        pub const fn set_pc04(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "desc PC05."]
        #[must_use]
        #[inline(always)]
        pub const fn pc05(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "desc PC05."]
        #[inline(always)]
        pub const fn set_pc05(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "desc PC06."]
        #[must_use]
        #[inline(always)]
        pub const fn pc06(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "desc PC06."]
        #[inline(always)]
        pub const fn set_pc06(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "desc PC07."]
        #[must_use]
        #[inline(always)]
        pub const fn pc07(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "desc PC07."]
        #[inline(always)]
        pub const fn set_pc07(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "desc PC08."]
        #[must_use]
        #[inline(always)]
        pub const fn pc08(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "desc PC08."]
        #[inline(always)]
        pub const fn set_pc08(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "desc PC09."]
        #[must_use]
        #[inline(always)]
        pub const fn pc09(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "desc PC09."]
        #[inline(always)]
        pub const fn set_pc09(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "desc PC10."]
        #[must_use]
        #[inline(always)]
        pub const fn pc10(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "desc PC10."]
        #[inline(always)]
        pub const fn set_pc10(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[doc = "desc PC11."]
        #[must_use]
        #[inline(always)]
        pub const fn pc11(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "desc PC11."]
        #[inline(always)]
        pub const fn set_pc11(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[doc = "desc PC12."]
        #[must_use]
        #[inline(always)]
        pub const fn pc12(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "desc PC12."]
        #[inline(always)]
        pub const fn set_pc12(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "desc PC13."]
        #[must_use]
        #[inline(always)]
        pub const fn pc13(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[doc = "desc PC13."]
        #[inline(always)]
        pub const fn set_pc13(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[doc = "desc PC14."]
        #[must_use]
        #[inline(always)]
        pub const fn pc14(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "desc PC14."]
        #[inline(always)]
        pub const fn set_pc14(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[doc = "desc PC15."]
        #[must_use]
        #[inline(always)]
        pub const fn pc15(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "desc PC15."]
        #[inline(always)]
        pub const fn set_pc15(&mut self, val: bool) {
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
    impl Default for Pcout {
        #[inline(always)]
        fn default() -> Pcout {
            Pcout(0)
        }
    }
    impl core::fmt::Debug for Pcout {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Pcout")
                .field("pc00", &self.pc00())
                .field("pc01", &self.pc01())
                .field("pc02", &self.pc02())
                .field("pc03", &self.pc03())
                .field("pc04", &self.pc04())
                .field("pc05", &self.pc05())
                .field("pc06", &self.pc06())
                .field("pc07", &self.pc07())
                .field("pc08", &self.pc08())
                .field("pc09", &self.pc09())
                .field("pc10", &self.pc10())
                .field("pc11", &self.pc11())
                .field("pc12", &self.pc12())
                .field("pc13", &self.pc13())
                .field("pc14", &self.pc14())
                .field("pc15", &self.pc15())
                .field("rsv", &self.rsv())
                .finish()
        }
    }
    #[doc = "desc PCPD."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Pcpd(pub u32);
    impl Pcpd {
        #[doc = "desc PC00."]
        #[must_use]
        #[inline(always)]
        pub const fn pc00(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "desc PC00."]
        #[inline(always)]
        pub const fn set_pc00(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "desc PC01."]
        #[must_use]
        #[inline(always)]
        pub const fn pc01(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "desc PC01."]
        #[inline(always)]
        pub const fn set_pc01(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "desc PC02."]
        #[must_use]
        #[inline(always)]
        pub const fn pc02(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "desc PC02."]
        #[inline(always)]
        pub const fn set_pc02(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "desc PC03."]
        #[must_use]
        #[inline(always)]
        pub const fn pc03(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "desc PC03."]
        #[inline(always)]
        pub const fn set_pc03(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "desc PC04."]
        #[must_use]
        #[inline(always)]
        pub const fn pc04(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "desc PC04."]
        #[inline(always)]
        pub const fn set_pc04(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "desc PC05."]
        #[must_use]
        #[inline(always)]
        pub const fn pc05(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "desc PC05."]
        #[inline(always)]
        pub const fn set_pc05(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "desc PC06."]
        #[must_use]
        #[inline(always)]
        pub const fn pc06(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "desc PC06."]
        #[inline(always)]
        pub const fn set_pc06(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "desc PC07."]
        #[must_use]
        #[inline(always)]
        pub const fn pc07(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "desc PC07."]
        #[inline(always)]
        pub const fn set_pc07(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "desc PC08."]
        #[must_use]
        #[inline(always)]
        pub const fn pc08(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "desc PC08."]
        #[inline(always)]
        pub const fn set_pc08(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "desc PC09."]
        #[must_use]
        #[inline(always)]
        pub const fn pc09(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "desc PC09."]
        #[inline(always)]
        pub const fn set_pc09(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "desc PC10."]
        #[must_use]
        #[inline(always)]
        pub const fn pc10(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "desc PC10."]
        #[inline(always)]
        pub const fn set_pc10(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[doc = "desc PC11."]
        #[must_use]
        #[inline(always)]
        pub const fn pc11(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "desc PC11."]
        #[inline(always)]
        pub const fn set_pc11(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[doc = "desc PC12."]
        #[must_use]
        #[inline(always)]
        pub const fn pc12(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "desc PC12."]
        #[inline(always)]
        pub const fn set_pc12(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "desc PC13."]
        #[must_use]
        #[inline(always)]
        pub const fn pc13(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[doc = "desc PC13."]
        #[inline(always)]
        pub const fn set_pc13(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[doc = "desc PC14."]
        #[must_use]
        #[inline(always)]
        pub const fn pc14(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "desc PC14."]
        #[inline(always)]
        pub const fn set_pc14(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[doc = "desc PC15."]
        #[must_use]
        #[inline(always)]
        pub const fn pc15(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "desc PC15."]
        #[inline(always)]
        pub const fn set_pc15(&mut self, val: bool) {
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
    impl Default for Pcpd {
        #[inline(always)]
        fn default() -> Pcpd {
            Pcpd(0)
        }
    }
    impl core::fmt::Debug for Pcpd {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Pcpd")
                .field("pc00", &self.pc00())
                .field("pc01", &self.pc01())
                .field("pc02", &self.pc02())
                .field("pc03", &self.pc03())
                .field("pc04", &self.pc04())
                .field("pc05", &self.pc05())
                .field("pc06", &self.pc06())
                .field("pc07", &self.pc07())
                .field("pc08", &self.pc08())
                .field("pc09", &self.pc09())
                .field("pc10", &self.pc10())
                .field("pc11", &self.pc11())
                .field("pc12", &self.pc12())
                .field("pc13", &self.pc13())
                .field("pc14", &self.pc14())
                .field("pc15", &self.pc15())
                .field("rsv", &self.rsv())
                .finish()
        }
    }
    #[doc = "desc PCPU."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Pcpu(pub u32);
    impl Pcpu {
        #[doc = "desc PC00."]
        #[must_use]
        #[inline(always)]
        pub const fn pc00(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "desc PC00."]
        #[inline(always)]
        pub const fn set_pc00(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "desc PC01."]
        #[must_use]
        #[inline(always)]
        pub const fn pc01(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "desc PC01."]
        #[inline(always)]
        pub const fn set_pc01(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "desc PC02."]
        #[must_use]
        #[inline(always)]
        pub const fn pc02(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "desc PC02."]
        #[inline(always)]
        pub const fn set_pc02(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "desc PC03."]
        #[must_use]
        #[inline(always)]
        pub const fn pc03(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "desc PC03."]
        #[inline(always)]
        pub const fn set_pc03(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "desc PC04."]
        #[must_use]
        #[inline(always)]
        pub const fn pc04(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "desc PC04."]
        #[inline(always)]
        pub const fn set_pc04(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "desc PC05."]
        #[must_use]
        #[inline(always)]
        pub const fn pc05(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "desc PC05."]
        #[inline(always)]
        pub const fn set_pc05(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "desc PC06."]
        #[must_use]
        #[inline(always)]
        pub const fn pc06(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "desc PC06."]
        #[inline(always)]
        pub const fn set_pc06(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "desc PC07."]
        #[must_use]
        #[inline(always)]
        pub const fn pc07(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "desc PC07."]
        #[inline(always)]
        pub const fn set_pc07(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "desc PC08."]
        #[must_use]
        #[inline(always)]
        pub const fn pc08(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "desc PC08."]
        #[inline(always)]
        pub const fn set_pc08(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "desc PC09."]
        #[must_use]
        #[inline(always)]
        pub const fn pc09(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "desc PC09."]
        #[inline(always)]
        pub const fn set_pc09(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "desc PC10."]
        #[must_use]
        #[inline(always)]
        pub const fn pc10(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "desc PC10."]
        #[inline(always)]
        pub const fn set_pc10(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[doc = "desc PC11."]
        #[must_use]
        #[inline(always)]
        pub const fn pc11(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "desc PC11."]
        #[inline(always)]
        pub const fn set_pc11(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[doc = "desc PC12."]
        #[must_use]
        #[inline(always)]
        pub const fn pc12(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "desc PC12."]
        #[inline(always)]
        pub const fn set_pc12(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "desc PC13."]
        #[must_use]
        #[inline(always)]
        pub const fn pc13(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[doc = "desc PC13."]
        #[inline(always)]
        pub const fn set_pc13(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[doc = "desc PC14."]
        #[must_use]
        #[inline(always)]
        pub const fn pc14(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "desc PC14."]
        #[inline(always)]
        pub const fn set_pc14(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[doc = "desc PC15."]
        #[must_use]
        #[inline(always)]
        pub const fn pc15(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "desc PC15."]
        #[inline(always)]
        pub const fn set_pc15(&mut self, val: bool) {
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
    impl Default for Pcpu {
        #[inline(always)]
        fn default() -> Pcpu {
            Pcpu(0)
        }
    }
    impl core::fmt::Debug for Pcpu {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Pcpu")
                .field("pc00", &self.pc00())
                .field("pc01", &self.pc01())
                .field("pc02", &self.pc02())
                .field("pc03", &self.pc03())
                .field("pc04", &self.pc04())
                .field("pc05", &self.pc05())
                .field("pc06", &self.pc06())
                .field("pc07", &self.pc07())
                .field("pc08", &self.pc08())
                .field("pc09", &self.pc09())
                .field("pc10", &self.pc10())
                .field("pc11", &self.pc11())
                .field("pc12", &self.pc12())
                .field("pc13", &self.pc13())
                .field("pc14", &self.pc14())
                .field("pc15", &self.pc15())
                .field("rsv", &self.rsv())
                .finish()
        }
    }
    #[doc = "desc PCRIE."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Pcrie(pub u32);
    impl Pcrie {
        #[doc = "desc PC00."]
        #[must_use]
        #[inline(always)]
        pub const fn pc00(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "desc PC00."]
        #[inline(always)]
        pub const fn set_pc00(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "desc PC01."]
        #[must_use]
        #[inline(always)]
        pub const fn pc01(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "desc PC01."]
        #[inline(always)]
        pub const fn set_pc01(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "desc PC02."]
        #[must_use]
        #[inline(always)]
        pub const fn pc02(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "desc PC02."]
        #[inline(always)]
        pub const fn set_pc02(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "desc PC03."]
        #[must_use]
        #[inline(always)]
        pub const fn pc03(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "desc PC03."]
        #[inline(always)]
        pub const fn set_pc03(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "desc PC04."]
        #[must_use]
        #[inline(always)]
        pub const fn pc04(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "desc PC04."]
        #[inline(always)]
        pub const fn set_pc04(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "desc PC05."]
        #[must_use]
        #[inline(always)]
        pub const fn pc05(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "desc PC05."]
        #[inline(always)]
        pub const fn set_pc05(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "desc PC06."]
        #[must_use]
        #[inline(always)]
        pub const fn pc06(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "desc PC06."]
        #[inline(always)]
        pub const fn set_pc06(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "desc PC07."]
        #[must_use]
        #[inline(always)]
        pub const fn pc07(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "desc PC07."]
        #[inline(always)]
        pub const fn set_pc07(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "desc PC08."]
        #[must_use]
        #[inline(always)]
        pub const fn pc08(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "desc PC08."]
        #[inline(always)]
        pub const fn set_pc08(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "desc PC09."]
        #[must_use]
        #[inline(always)]
        pub const fn pc09(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "desc PC09."]
        #[inline(always)]
        pub const fn set_pc09(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "desc PC10."]
        #[must_use]
        #[inline(always)]
        pub const fn pc10(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "desc PC10."]
        #[inline(always)]
        pub const fn set_pc10(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[doc = "desc PC11."]
        #[must_use]
        #[inline(always)]
        pub const fn pc11(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "desc PC11."]
        #[inline(always)]
        pub const fn set_pc11(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[doc = "desc PC12."]
        #[must_use]
        #[inline(always)]
        pub const fn pc12(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "desc PC12."]
        #[inline(always)]
        pub const fn set_pc12(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "desc PC13."]
        #[must_use]
        #[inline(always)]
        pub const fn pc13(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[doc = "desc PC13."]
        #[inline(always)]
        pub const fn set_pc13(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[doc = "desc PC14."]
        #[must_use]
        #[inline(always)]
        pub const fn pc14(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "desc PC14."]
        #[inline(always)]
        pub const fn set_pc14(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[doc = "desc PC15."]
        #[must_use]
        #[inline(always)]
        pub const fn pc15(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "desc PC15."]
        #[inline(always)]
        pub const fn set_pc15(&mut self, val: bool) {
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
    impl Default for Pcrie {
        #[inline(always)]
        fn default() -> Pcrie {
            Pcrie(0)
        }
    }
    impl core::fmt::Debug for Pcrie {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Pcrie")
                .field("pc00", &self.pc00())
                .field("pc01", &self.pc01())
                .field("pc02", &self.pc02())
                .field("pc03", &self.pc03())
                .field("pc04", &self.pc04())
                .field("pc05", &self.pc05())
                .field("pc06", &self.pc06())
                .field("pc07", &self.pc07())
                .field("pc08", &self.pc08())
                .field("pc09", &self.pc09())
                .field("pc10", &self.pc10())
                .field("pc11", &self.pc11())
                .field("pc12", &self.pc12())
                .field("pc13", &self.pc13())
                .field("pc14", &self.pc14())
                .field("pc15", &self.pc15())
                .field("rsv", &self.rsv())
                .finish()
        }
    }
    #[doc = "desc PD00_SEL."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Pd00Sel(pub u32);
    impl Pd00Sel {
        #[doc = "desc SEL."]
        #[must_use]
        #[inline(always)]
        pub const fn sel(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x07;
            val as u8
        }
        #[doc = "desc SEL."]
        #[inline(always)]
        pub const fn set_sel(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 0usize)) | (((val as u32) & 0x07) << 0usize);
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
    impl Default for Pd00Sel {
        #[inline(always)]
        fn default() -> Pd00Sel {
            Pd00Sel(0)
        }
    }
    impl core::fmt::Debug for Pd00Sel {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Pd00Sel")
                .field("sel", &self.sel())
                .field("rsv", &self.rsv())
                .finish()
        }
    }
    #[doc = "desc PD01_SEL."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Pd01Sel(pub u32);
    impl Pd01Sel {
        #[doc = "desc SEL."]
        #[must_use]
        #[inline(always)]
        pub const fn sel(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x07;
            val as u8
        }
        #[doc = "desc SEL."]
        #[inline(always)]
        pub const fn set_sel(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 0usize)) | (((val as u32) & 0x07) << 0usize);
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
    impl Default for Pd01Sel {
        #[inline(always)]
        fn default() -> Pd01Sel {
            Pd01Sel(0)
        }
    }
    impl core::fmt::Debug for Pd01Sel {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Pd01Sel")
                .field("sel", &self.sel())
                .field("rsv", &self.rsv())
                .finish()
        }
    }
    #[doc = "desc PD02_SEL."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Pd02Sel(pub u32);
    impl Pd02Sel {
        #[doc = "desc SEL."]
        #[must_use]
        #[inline(always)]
        pub const fn sel(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x07;
            val as u8
        }
        #[doc = "desc SEL."]
        #[inline(always)]
        pub const fn set_sel(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 0usize)) | (((val as u32) & 0x07) << 0usize);
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
    impl Default for Pd02Sel {
        #[inline(always)]
        fn default() -> Pd02Sel {
            Pd02Sel(0)
        }
    }
    impl core::fmt::Debug for Pd02Sel {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Pd02Sel")
                .field("sel", &self.sel())
                .field("rsv", &self.rsv())
                .finish()
        }
    }
    #[doc = "desc PD03_SEL."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Pd03Sel(pub u32);
    impl Pd03Sel {
        #[doc = "desc SEL."]
        #[must_use]
        #[inline(always)]
        pub const fn sel(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x07;
            val as u8
        }
        #[doc = "desc SEL."]
        #[inline(always)]
        pub const fn set_sel(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 0usize)) | (((val as u32) & 0x07) << 0usize);
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
    impl Default for Pd03Sel {
        #[inline(always)]
        fn default() -> Pd03Sel {
            Pd03Sel(0)
        }
    }
    impl core::fmt::Debug for Pd03Sel {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Pd03Sel")
                .field("sel", &self.sel())
                .field("rsv", &self.rsv())
                .finish()
        }
    }
    #[doc = "desc PD04_SEL."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Pd04Sel(pub u32);
    impl Pd04Sel {
        #[doc = "desc SEL."]
        #[must_use]
        #[inline(always)]
        pub const fn sel(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x07;
            val as u8
        }
        #[doc = "desc SEL."]
        #[inline(always)]
        pub const fn set_sel(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 0usize)) | (((val as u32) & 0x07) << 0usize);
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
    impl Default for Pd04Sel {
        #[inline(always)]
        fn default() -> Pd04Sel {
            Pd04Sel(0)
        }
    }
    impl core::fmt::Debug for Pd04Sel {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Pd04Sel")
                .field("sel", &self.sel())
                .field("rsv", &self.rsv())
                .finish()
        }
    }
    #[doc = "desc PD05_SEL."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Pd05Sel(pub u32);
    impl Pd05Sel {
        #[doc = "desc SEL."]
        #[must_use]
        #[inline(always)]
        pub const fn sel(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x07;
            val as u8
        }
        #[doc = "desc SEL."]
        #[inline(always)]
        pub const fn set_sel(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 0usize)) | (((val as u32) & 0x07) << 0usize);
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
    impl Default for Pd05Sel {
        #[inline(always)]
        fn default() -> Pd05Sel {
            Pd05Sel(0)
        }
    }
    impl core::fmt::Debug for Pd05Sel {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Pd05Sel")
                .field("sel", &self.sel())
                .field("rsv", &self.rsv())
                .finish()
        }
    }
    #[doc = "desc PD06_SEL."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Pd06Sel(pub u32);
    impl Pd06Sel {
        #[doc = "desc SEL."]
        #[must_use]
        #[inline(always)]
        pub const fn sel(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x07;
            val as u8
        }
        #[doc = "desc SEL."]
        #[inline(always)]
        pub const fn set_sel(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 0usize)) | (((val as u32) & 0x07) << 0usize);
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
    impl Default for Pd06Sel {
        #[inline(always)]
        fn default() -> Pd06Sel {
            Pd06Sel(0)
        }
    }
    impl core::fmt::Debug for Pd06Sel {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Pd06Sel")
                .field("sel", &self.sel())
                .field("rsv", &self.rsv())
                .finish()
        }
    }
    #[doc = "desc PD07_SEL."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Pd07Sel(pub u32);
    impl Pd07Sel {
        #[doc = "desc SEL."]
        #[must_use]
        #[inline(always)]
        pub const fn sel(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x07;
            val as u8
        }
        #[doc = "desc SEL."]
        #[inline(always)]
        pub const fn set_sel(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 0usize)) | (((val as u32) & 0x07) << 0usize);
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
    impl Default for Pd07Sel {
        #[inline(always)]
        fn default() -> Pd07Sel {
            Pd07Sel(0)
        }
    }
    impl core::fmt::Debug for Pd07Sel {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Pd07Sel")
                .field("sel", &self.sel())
                .field("rsv", &self.rsv())
                .finish()
        }
    }
    #[doc = "desc PD_ICLR."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct PdIclr(pub u32);
    impl PdIclr {
        #[doc = "desc PD00."]
        #[must_use]
        #[inline(always)]
        pub const fn pd00(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "desc PD00."]
        #[inline(always)]
        pub const fn set_pd00(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "desc PD01."]
        #[must_use]
        #[inline(always)]
        pub const fn pd01(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "desc PD01."]
        #[inline(always)]
        pub const fn set_pd01(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "desc PD02."]
        #[must_use]
        #[inline(always)]
        pub const fn pd02(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "desc PD02."]
        #[inline(always)]
        pub const fn set_pd02(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "desc PD03."]
        #[must_use]
        #[inline(always)]
        pub const fn pd03(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "desc PD03."]
        #[inline(always)]
        pub const fn set_pd03(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "desc PD04."]
        #[must_use]
        #[inline(always)]
        pub const fn pd04(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "desc PD04."]
        #[inline(always)]
        pub const fn set_pd04(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "desc PD05."]
        #[must_use]
        #[inline(always)]
        pub const fn pd05(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "desc PD05."]
        #[inline(always)]
        pub const fn set_pd05(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "desc PD06."]
        #[must_use]
        #[inline(always)]
        pub const fn pd06(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "desc PD06."]
        #[inline(always)]
        pub const fn set_pd06(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "desc PD07."]
        #[must_use]
        #[inline(always)]
        pub const fn pd07(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "desc PD07."]
        #[inline(always)]
        pub const fn set_pd07(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
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
    impl Default for PdIclr {
        #[inline(always)]
        fn default() -> PdIclr {
            PdIclr(0)
        }
    }
    impl core::fmt::Debug for PdIclr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("PdIclr")
                .field("pd00", &self.pd00())
                .field("pd01", &self.pd01())
                .field("pd02", &self.pd02())
                .field("pd03", &self.pd03())
                .field("pd04", &self.pd04())
                .field("pd05", &self.pd05())
                .field("pd06", &self.pd06())
                .field("pd07", &self.pd07())
                .field("rsv", &self.rsv())
                .finish()
        }
    }
    #[doc = "desc PD_STAT."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct PdStat(pub u32);
    impl PdStat {
        #[doc = "desc PD00."]
        #[must_use]
        #[inline(always)]
        pub const fn pd00(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "desc PD00."]
        #[inline(always)]
        pub const fn set_pd00(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "desc PD01."]
        #[must_use]
        #[inline(always)]
        pub const fn pd01(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "desc PD01."]
        #[inline(always)]
        pub const fn set_pd01(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "desc PD02."]
        #[must_use]
        #[inline(always)]
        pub const fn pd02(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "desc PD02."]
        #[inline(always)]
        pub const fn set_pd02(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "desc PD03."]
        #[must_use]
        #[inline(always)]
        pub const fn pd03(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "desc PD03."]
        #[inline(always)]
        pub const fn set_pd03(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "desc PD04."]
        #[must_use]
        #[inline(always)]
        pub const fn pd04(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "desc PD04."]
        #[inline(always)]
        pub const fn set_pd04(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "desc PD05."]
        #[must_use]
        #[inline(always)]
        pub const fn pd05(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "desc PD05."]
        #[inline(always)]
        pub const fn set_pd05(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "desc PD06."]
        #[must_use]
        #[inline(always)]
        pub const fn pd06(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "desc PD06."]
        #[inline(always)]
        pub const fn set_pd06(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "desc PD07."]
        #[must_use]
        #[inline(always)]
        pub const fn pd07(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "desc PD07."]
        #[inline(always)]
        pub const fn set_pd07(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
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
    impl Default for PdStat {
        #[inline(always)]
        fn default() -> PdStat {
            PdStat(0)
        }
    }
    impl core::fmt::Debug for PdStat {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("PdStat")
                .field("pd00", &self.pd00())
                .field("pd01", &self.pd01())
                .field("pd02", &self.pd02())
                .field("pd03", &self.pd03())
                .field("pd04", &self.pd04())
                .field("pd05", &self.pd05())
                .field("pd06", &self.pd06())
                .field("pd07", &self.pd07())
                .field("rsv", &self.rsv())
                .finish()
        }
    }
    #[doc = "desc PDADS."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Pdads(pub u32);
    impl Pdads {
        #[doc = "desc PD00."]
        #[must_use]
        #[inline(always)]
        pub const fn pd00(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "desc PD00."]
        #[inline(always)]
        pub const fn set_pd00(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "desc PD01."]
        #[must_use]
        #[inline(always)]
        pub const fn pd01(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "desc PD01."]
        #[inline(always)]
        pub const fn set_pd01(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "desc PD02."]
        #[must_use]
        #[inline(always)]
        pub const fn pd02(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "desc PD02."]
        #[inline(always)]
        pub const fn set_pd02(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "desc PD03."]
        #[must_use]
        #[inline(always)]
        pub const fn pd03(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "desc PD03."]
        #[inline(always)]
        pub const fn set_pd03(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "desc PD04."]
        #[must_use]
        #[inline(always)]
        pub const fn pd04(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "desc PD04."]
        #[inline(always)]
        pub const fn set_pd04(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "desc PD05."]
        #[must_use]
        #[inline(always)]
        pub const fn pd05(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "desc PD05."]
        #[inline(always)]
        pub const fn set_pd05(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "desc PD06."]
        #[must_use]
        #[inline(always)]
        pub const fn pd06(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "desc PD06."]
        #[inline(always)]
        pub const fn set_pd06(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "desc PD07."]
        #[must_use]
        #[inline(always)]
        pub const fn pd07(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "desc PD07."]
        #[inline(always)]
        pub const fn set_pd07(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
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
    impl Default for Pdads {
        #[inline(always)]
        fn default() -> Pdads {
            Pdads(0)
        }
    }
    impl core::fmt::Debug for Pdads {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Pdads")
                .field("pd00", &self.pd00())
                .field("pd01", &self.pd01())
                .field("pd02", &self.pd02())
                .field("pd03", &self.pd03())
                .field("pd04", &self.pd04())
                .field("pd05", &self.pd05())
                .field("pd06", &self.pd06())
                .field("pd07", &self.pd07())
                .field("rsv", &self.rsv())
                .finish()
        }
    }
    #[doc = "desc PDBCLR."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Pdbclr(pub u32);
    impl Pdbclr {
        #[doc = "desc PD00."]
        #[must_use]
        #[inline(always)]
        pub const fn pd00(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "desc PD00."]
        #[inline(always)]
        pub const fn set_pd00(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "desc PD01."]
        #[must_use]
        #[inline(always)]
        pub const fn pd01(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "desc PD01."]
        #[inline(always)]
        pub const fn set_pd01(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "desc PD02."]
        #[must_use]
        #[inline(always)]
        pub const fn pd02(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "desc PD02."]
        #[inline(always)]
        pub const fn set_pd02(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "desc PD03."]
        #[must_use]
        #[inline(always)]
        pub const fn pd03(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "desc PD03."]
        #[inline(always)]
        pub const fn set_pd03(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "desc PD04."]
        #[must_use]
        #[inline(always)]
        pub const fn pd04(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "desc PD04."]
        #[inline(always)]
        pub const fn set_pd04(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "desc PD05."]
        #[must_use]
        #[inline(always)]
        pub const fn pd05(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "desc PD05."]
        #[inline(always)]
        pub const fn set_pd05(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "desc PD06."]
        #[must_use]
        #[inline(always)]
        pub const fn pd06(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "desc PD06."]
        #[inline(always)]
        pub const fn set_pd06(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "desc PD07."]
        #[must_use]
        #[inline(always)]
        pub const fn pd07(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "desc PD07."]
        #[inline(always)]
        pub const fn set_pd07(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
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
    impl Default for Pdbclr {
        #[inline(always)]
        fn default() -> Pdbclr {
            Pdbclr(0)
        }
    }
    impl core::fmt::Debug for Pdbclr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Pdbclr")
                .field("pd00", &self.pd00())
                .field("pd01", &self.pd01())
                .field("pd02", &self.pd02())
                .field("pd03", &self.pd03())
                .field("pd04", &self.pd04())
                .field("pd05", &self.pd05())
                .field("pd06", &self.pd06())
                .field("pd07", &self.pd07())
                .field("rsv", &self.rsv())
                .finish()
        }
    }
    #[doc = "desc PDBSET."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Pdbset(pub u32);
    impl Pdbset {
        #[doc = "desc PD00."]
        #[must_use]
        #[inline(always)]
        pub const fn pd00(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "desc PD00."]
        #[inline(always)]
        pub const fn set_pd00(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "desc PD01."]
        #[must_use]
        #[inline(always)]
        pub const fn pd01(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "desc PD01."]
        #[inline(always)]
        pub const fn set_pd01(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "desc PD02."]
        #[must_use]
        #[inline(always)]
        pub const fn pd02(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "desc PD02."]
        #[inline(always)]
        pub const fn set_pd02(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "desc PD03."]
        #[must_use]
        #[inline(always)]
        pub const fn pd03(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "desc PD03."]
        #[inline(always)]
        pub const fn set_pd03(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "desc PD04."]
        #[must_use]
        #[inline(always)]
        pub const fn pd04(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "desc PD04."]
        #[inline(always)]
        pub const fn set_pd04(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "desc PD05."]
        #[must_use]
        #[inline(always)]
        pub const fn pd05(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "desc PD05."]
        #[inline(always)]
        pub const fn set_pd05(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "desc PD06."]
        #[must_use]
        #[inline(always)]
        pub const fn pd06(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "desc PD06."]
        #[inline(always)]
        pub const fn set_pd06(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "desc PD07."]
        #[must_use]
        #[inline(always)]
        pub const fn pd07(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "desc PD07."]
        #[inline(always)]
        pub const fn set_pd07(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
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
    impl Default for Pdbset {
        #[inline(always)]
        fn default() -> Pdbset {
            Pdbset(0)
        }
    }
    impl core::fmt::Debug for Pdbset {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Pdbset")
                .field("pd00", &self.pd00())
                .field("pd01", &self.pd01())
                .field("pd02", &self.pd02())
                .field("pd03", &self.pd03())
                .field("pd04", &self.pd04())
                .field("pd05", &self.pd05())
                .field("pd06", &self.pd06())
                .field("pd07", &self.pd07())
                .field("rsv", &self.rsv())
                .finish()
        }
    }
    #[doc = "desc PDBSETCLR."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Pdbsetclr(pub u32);
    impl Pdbsetclr {
        #[doc = "desc PDBCLR."]
        #[must_use]
        #[inline(always)]
        pub const fn pdbclr(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[doc = "desc PDBCLR."]
        #[inline(always)]
        pub const fn set_pdbclr(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
        }
        #[doc = "desc PDBSET."]
        #[must_use]
        #[inline(always)]
        pub const fn pdbset(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0xff;
            val as u8
        }
        #[doc = "desc PDBSET."]
        #[inline(always)]
        pub const fn set_pdbset(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 16usize)) | (((val as u32) & 0xff) << 16usize);
        }
    }
    impl Default for Pdbsetclr {
        #[inline(always)]
        fn default() -> Pdbsetclr {
            Pdbsetclr(0)
        }
    }
    impl core::fmt::Debug for Pdbsetclr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Pdbsetclr")
                .field("pdbclr", &self.pdbclr())
                .field("pdbset", &self.pdbset())
                .finish()
        }
    }
    #[doc = "desc PDDIR."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Pddir(pub u32);
    impl Pddir {
        #[doc = "desc PD00."]
        #[must_use]
        #[inline(always)]
        pub const fn pd00(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "desc PD00."]
        #[inline(always)]
        pub const fn set_pd00(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "desc PD01."]
        #[must_use]
        #[inline(always)]
        pub const fn pd01(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "desc PD01."]
        #[inline(always)]
        pub const fn set_pd01(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "desc PD02."]
        #[must_use]
        #[inline(always)]
        pub const fn pd02(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "desc PD02."]
        #[inline(always)]
        pub const fn set_pd02(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "desc PD03."]
        #[must_use]
        #[inline(always)]
        pub const fn pd03(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "desc PD03."]
        #[inline(always)]
        pub const fn set_pd03(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "desc PD04."]
        #[must_use]
        #[inline(always)]
        pub const fn pd04(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "desc PD04."]
        #[inline(always)]
        pub const fn set_pd04(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "desc PD05."]
        #[must_use]
        #[inline(always)]
        pub const fn pd05(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "desc PD05."]
        #[inline(always)]
        pub const fn set_pd05(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "desc PD06."]
        #[must_use]
        #[inline(always)]
        pub const fn pd06(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "desc PD06."]
        #[inline(always)]
        pub const fn set_pd06(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "desc PD07."]
        #[must_use]
        #[inline(always)]
        pub const fn pd07(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "desc PD07."]
        #[inline(always)]
        pub const fn set_pd07(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
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
    impl Default for Pddir {
        #[inline(always)]
        fn default() -> Pddir {
            Pddir(0)
        }
    }
    impl core::fmt::Debug for Pddir {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Pddir")
                .field("pd00", &self.pd00())
                .field("pd01", &self.pd01())
                .field("pd02", &self.pd02())
                .field("pd03", &self.pd03())
                .field("pd04", &self.pd04())
                .field("pd05", &self.pd05())
                .field("pd06", &self.pd06())
                .field("pd07", &self.pd07())
                .field("rsv", &self.rsv())
                .finish()
        }
    }
    #[doc = "desc PDDR."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Pddr(pub u32);
    impl Pddr {
        #[doc = "desc PD00."]
        #[must_use]
        #[inline(always)]
        pub const fn pd00(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "desc PD00."]
        #[inline(always)]
        pub const fn set_pd00(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "desc PD01."]
        #[must_use]
        #[inline(always)]
        pub const fn pd01(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "desc PD01."]
        #[inline(always)]
        pub const fn set_pd01(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "desc PD02."]
        #[must_use]
        #[inline(always)]
        pub const fn pd02(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "desc PD02."]
        #[inline(always)]
        pub const fn set_pd02(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "desc PD03."]
        #[must_use]
        #[inline(always)]
        pub const fn pd03(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "desc PD03."]
        #[inline(always)]
        pub const fn set_pd03(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "desc PD04."]
        #[must_use]
        #[inline(always)]
        pub const fn pd04(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "desc PD04."]
        #[inline(always)]
        pub const fn set_pd04(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "desc PD05."]
        #[must_use]
        #[inline(always)]
        pub const fn pd05(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "desc PD05."]
        #[inline(always)]
        pub const fn set_pd05(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "desc PD06."]
        #[must_use]
        #[inline(always)]
        pub const fn pd06(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "desc PD06."]
        #[inline(always)]
        pub const fn set_pd06(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "desc PD07."]
        #[must_use]
        #[inline(always)]
        pub const fn pd07(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "desc PD07."]
        #[inline(always)]
        pub const fn set_pd07(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
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
    impl Default for Pddr {
        #[inline(always)]
        fn default() -> Pddr {
            Pddr(0)
        }
    }
    impl core::fmt::Debug for Pddr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Pddr")
                .field("pd00", &self.pd00())
                .field("pd01", &self.pd01())
                .field("pd02", &self.pd02())
                .field("pd03", &self.pd03())
                .field("pd04", &self.pd04())
                .field("pd05", &self.pd05())
                .field("pd06", &self.pd06())
                .field("pd07", &self.pd07())
                .field("rsv", &self.rsv())
                .finish()
        }
    }
    #[doc = "desc PDFIE."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Pdfie(pub u32);
    impl Pdfie {
        #[doc = "desc PD00."]
        #[must_use]
        #[inline(always)]
        pub const fn pd00(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "desc PD00."]
        #[inline(always)]
        pub const fn set_pd00(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "desc PD01."]
        #[must_use]
        #[inline(always)]
        pub const fn pd01(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "desc PD01."]
        #[inline(always)]
        pub const fn set_pd01(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "desc PD02."]
        #[must_use]
        #[inline(always)]
        pub const fn pd02(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "desc PD02."]
        #[inline(always)]
        pub const fn set_pd02(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "desc PD03."]
        #[must_use]
        #[inline(always)]
        pub const fn pd03(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "desc PD03."]
        #[inline(always)]
        pub const fn set_pd03(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "desc PD04."]
        #[must_use]
        #[inline(always)]
        pub const fn pd04(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "desc PD04."]
        #[inline(always)]
        pub const fn set_pd04(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "desc PD05."]
        #[must_use]
        #[inline(always)]
        pub const fn pd05(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "desc PD05."]
        #[inline(always)]
        pub const fn set_pd05(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "desc PD06."]
        #[must_use]
        #[inline(always)]
        pub const fn pd06(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "desc PD06."]
        #[inline(always)]
        pub const fn set_pd06(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "desc PD07."]
        #[must_use]
        #[inline(always)]
        pub const fn pd07(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "desc PD07."]
        #[inline(always)]
        pub const fn set_pd07(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
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
    impl Default for Pdfie {
        #[inline(always)]
        fn default() -> Pdfie {
            Pdfie(0)
        }
    }
    impl core::fmt::Debug for Pdfie {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Pdfie")
                .field("pd00", &self.pd00())
                .field("pd01", &self.pd01())
                .field("pd02", &self.pd02())
                .field("pd03", &self.pd03())
                .field("pd04", &self.pd04())
                .field("pd05", &self.pd05())
                .field("pd06", &self.pd06())
                .field("pd07", &self.pd07())
                .field("rsv", &self.rsv())
                .finish()
        }
    }
    #[doc = "desc PDHIE."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Pdhie(pub u32);
    impl Pdhie {
        #[doc = "desc PD00."]
        #[must_use]
        #[inline(always)]
        pub const fn pd00(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "desc PD00."]
        #[inline(always)]
        pub const fn set_pd00(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "desc PD01."]
        #[must_use]
        #[inline(always)]
        pub const fn pd01(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "desc PD01."]
        #[inline(always)]
        pub const fn set_pd01(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "desc PD02."]
        #[must_use]
        #[inline(always)]
        pub const fn pd02(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "desc PD02."]
        #[inline(always)]
        pub const fn set_pd02(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "desc PD03."]
        #[must_use]
        #[inline(always)]
        pub const fn pd03(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "desc PD03."]
        #[inline(always)]
        pub const fn set_pd03(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "desc PD04."]
        #[must_use]
        #[inline(always)]
        pub const fn pd04(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "desc PD04."]
        #[inline(always)]
        pub const fn set_pd04(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "desc PD05."]
        #[must_use]
        #[inline(always)]
        pub const fn pd05(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "desc PD05."]
        #[inline(always)]
        pub const fn set_pd05(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "desc PD06."]
        #[must_use]
        #[inline(always)]
        pub const fn pd06(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "desc PD06."]
        #[inline(always)]
        pub const fn set_pd06(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "desc PD07."]
        #[must_use]
        #[inline(always)]
        pub const fn pd07(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "desc PD07."]
        #[inline(always)]
        pub const fn set_pd07(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
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
    impl Default for Pdhie {
        #[inline(always)]
        fn default() -> Pdhie {
            Pdhie(0)
        }
    }
    impl core::fmt::Debug for Pdhie {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Pdhie")
                .field("pd00", &self.pd00())
                .field("pd01", &self.pd01())
                .field("pd02", &self.pd02())
                .field("pd03", &self.pd03())
                .field("pd04", &self.pd04())
                .field("pd05", &self.pd05())
                .field("pd06", &self.pd06())
                .field("pd07", &self.pd07())
                .field("rsv", &self.rsv())
                .finish()
        }
    }
    #[doc = "desc PDIN."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Pdin(pub u32);
    impl Pdin {
        #[doc = "desc PD00."]
        #[must_use]
        #[inline(always)]
        pub const fn pd00(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "desc PD00."]
        #[inline(always)]
        pub const fn set_pd00(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "desc PD01."]
        #[must_use]
        #[inline(always)]
        pub const fn pd01(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "desc PD01."]
        #[inline(always)]
        pub const fn set_pd01(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "desc PD02."]
        #[must_use]
        #[inline(always)]
        pub const fn pd02(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "desc PD02."]
        #[inline(always)]
        pub const fn set_pd02(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "desc PD03."]
        #[must_use]
        #[inline(always)]
        pub const fn pd03(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "desc PD03."]
        #[inline(always)]
        pub const fn set_pd03(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "desc PD04."]
        #[must_use]
        #[inline(always)]
        pub const fn pd04(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "desc PD04."]
        #[inline(always)]
        pub const fn set_pd04(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "desc PD05."]
        #[must_use]
        #[inline(always)]
        pub const fn pd05(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "desc PD05."]
        #[inline(always)]
        pub const fn set_pd05(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "desc PD06."]
        #[must_use]
        #[inline(always)]
        pub const fn pd06(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "desc PD06."]
        #[inline(always)]
        pub const fn set_pd06(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "desc PD07."]
        #[must_use]
        #[inline(always)]
        pub const fn pd07(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "desc PD07."]
        #[inline(always)]
        pub const fn set_pd07(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
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
    impl Default for Pdin {
        #[inline(always)]
        fn default() -> Pdin {
            Pdin(0)
        }
    }
    impl core::fmt::Debug for Pdin {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Pdin")
                .field("pd00", &self.pd00())
                .field("pd01", &self.pd01())
                .field("pd02", &self.pd02())
                .field("pd03", &self.pd03())
                .field("pd04", &self.pd04())
                .field("pd05", &self.pd05())
                .field("pd06", &self.pd06())
                .field("pd07", &self.pd07())
                .field("rsv", &self.rsv())
                .finish()
        }
    }
    #[doc = "desc PDLIE."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Pdlie(pub u32);
    impl Pdlie {
        #[doc = "desc PD00."]
        #[must_use]
        #[inline(always)]
        pub const fn pd00(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "desc PD00."]
        #[inline(always)]
        pub const fn set_pd00(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "desc PD01."]
        #[must_use]
        #[inline(always)]
        pub const fn pd01(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "desc PD01."]
        #[inline(always)]
        pub const fn set_pd01(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "desc PD02."]
        #[must_use]
        #[inline(always)]
        pub const fn pd02(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "desc PD02."]
        #[inline(always)]
        pub const fn set_pd02(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "desc PD03."]
        #[must_use]
        #[inline(always)]
        pub const fn pd03(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "desc PD03."]
        #[inline(always)]
        pub const fn set_pd03(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "desc PD04."]
        #[must_use]
        #[inline(always)]
        pub const fn pd04(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "desc PD04."]
        #[inline(always)]
        pub const fn set_pd04(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "desc PD05."]
        #[must_use]
        #[inline(always)]
        pub const fn pd05(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "desc PD05."]
        #[inline(always)]
        pub const fn set_pd05(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "desc PD06."]
        #[must_use]
        #[inline(always)]
        pub const fn pd06(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "desc PD06."]
        #[inline(always)]
        pub const fn set_pd06(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "desc PD07."]
        #[must_use]
        #[inline(always)]
        pub const fn pd07(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "desc PD07."]
        #[inline(always)]
        pub const fn set_pd07(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
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
    impl Default for Pdlie {
        #[inline(always)]
        fn default() -> Pdlie {
            Pdlie(0)
        }
    }
    impl core::fmt::Debug for Pdlie {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Pdlie")
                .field("pd00", &self.pd00())
                .field("pd01", &self.pd01())
                .field("pd02", &self.pd02())
                .field("pd03", &self.pd03())
                .field("pd04", &self.pd04())
                .field("pd05", &self.pd05())
                .field("pd06", &self.pd06())
                .field("pd07", &self.pd07())
                .field("rsv", &self.rsv())
                .finish()
        }
    }
    #[doc = "desc PDOD."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Pdod(pub u32);
    impl Pdod {
        #[doc = "desc PD00."]
        #[must_use]
        #[inline(always)]
        pub const fn pd00(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "desc PD00."]
        #[inline(always)]
        pub const fn set_pd00(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "desc PD01."]
        #[must_use]
        #[inline(always)]
        pub const fn pd01(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "desc PD01."]
        #[inline(always)]
        pub const fn set_pd01(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "desc PD02."]
        #[must_use]
        #[inline(always)]
        pub const fn pd02(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "desc PD02."]
        #[inline(always)]
        pub const fn set_pd02(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "desc PD03."]
        #[must_use]
        #[inline(always)]
        pub const fn pd03(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "desc PD03."]
        #[inline(always)]
        pub const fn set_pd03(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "desc PD04."]
        #[must_use]
        #[inline(always)]
        pub const fn pd04(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "desc PD04."]
        #[inline(always)]
        pub const fn set_pd04(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "desc PD05."]
        #[must_use]
        #[inline(always)]
        pub const fn pd05(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "desc PD05."]
        #[inline(always)]
        pub const fn set_pd05(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "desc PD06."]
        #[must_use]
        #[inline(always)]
        pub const fn pd06(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "desc PD06."]
        #[inline(always)]
        pub const fn set_pd06(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "desc PD07."]
        #[must_use]
        #[inline(always)]
        pub const fn pd07(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "desc PD07."]
        #[inline(always)]
        pub const fn set_pd07(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
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
    impl Default for Pdod {
        #[inline(always)]
        fn default() -> Pdod {
            Pdod(0)
        }
    }
    impl core::fmt::Debug for Pdod {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Pdod")
                .field("pd00", &self.pd00())
                .field("pd01", &self.pd01())
                .field("pd02", &self.pd02())
                .field("pd03", &self.pd03())
                .field("pd04", &self.pd04())
                .field("pd05", &self.pd05())
                .field("pd06", &self.pd06())
                .field("pd07", &self.pd07())
                .field("rsv", &self.rsv())
                .finish()
        }
    }
    #[doc = "desc PDOUT."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Pdout(pub u32);
    impl Pdout {
        #[doc = "desc PD00."]
        #[must_use]
        #[inline(always)]
        pub const fn pd00(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "desc PD00."]
        #[inline(always)]
        pub const fn set_pd00(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "desc PD01."]
        #[must_use]
        #[inline(always)]
        pub const fn pd01(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "desc PD01."]
        #[inline(always)]
        pub const fn set_pd01(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "desc PD02."]
        #[must_use]
        #[inline(always)]
        pub const fn pd02(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "desc PD02."]
        #[inline(always)]
        pub const fn set_pd02(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "desc PD03."]
        #[must_use]
        #[inline(always)]
        pub const fn pd03(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "desc PD03."]
        #[inline(always)]
        pub const fn set_pd03(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "desc PD04."]
        #[must_use]
        #[inline(always)]
        pub const fn pd04(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "desc PD04."]
        #[inline(always)]
        pub const fn set_pd04(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "desc PD05."]
        #[must_use]
        #[inline(always)]
        pub const fn pd05(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "desc PD05."]
        #[inline(always)]
        pub const fn set_pd05(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "desc PD06."]
        #[must_use]
        #[inline(always)]
        pub const fn pd06(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "desc PD06."]
        #[inline(always)]
        pub const fn set_pd06(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "desc PD07."]
        #[must_use]
        #[inline(always)]
        pub const fn pd07(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "desc PD07."]
        #[inline(always)]
        pub const fn set_pd07(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
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
    impl Default for Pdout {
        #[inline(always)]
        fn default() -> Pdout {
            Pdout(0)
        }
    }
    impl core::fmt::Debug for Pdout {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Pdout")
                .field("pd00", &self.pd00())
                .field("pd01", &self.pd01())
                .field("pd02", &self.pd02())
                .field("pd03", &self.pd03())
                .field("pd04", &self.pd04())
                .field("pd05", &self.pd05())
                .field("pd06", &self.pd06())
                .field("pd07", &self.pd07())
                .field("rsv", &self.rsv())
                .finish()
        }
    }
    #[doc = "desc PDPD."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Pdpd(pub u32);
    impl Pdpd {
        #[doc = "desc PD00."]
        #[must_use]
        #[inline(always)]
        pub const fn pd00(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "desc PD00."]
        #[inline(always)]
        pub const fn set_pd00(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "desc PD01."]
        #[must_use]
        #[inline(always)]
        pub const fn pd01(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "desc PD01."]
        #[inline(always)]
        pub const fn set_pd01(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "desc PD02."]
        #[must_use]
        #[inline(always)]
        pub const fn pd02(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "desc PD02."]
        #[inline(always)]
        pub const fn set_pd02(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "desc PD03."]
        #[must_use]
        #[inline(always)]
        pub const fn pd03(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "desc PD03."]
        #[inline(always)]
        pub const fn set_pd03(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "desc PD04."]
        #[must_use]
        #[inline(always)]
        pub const fn pd04(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "desc PD04."]
        #[inline(always)]
        pub const fn set_pd04(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "desc PD05."]
        #[must_use]
        #[inline(always)]
        pub const fn pd05(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "desc PD05."]
        #[inline(always)]
        pub const fn set_pd05(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "desc PD06."]
        #[must_use]
        #[inline(always)]
        pub const fn pd06(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "desc PD06."]
        #[inline(always)]
        pub const fn set_pd06(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "desc PD07."]
        #[must_use]
        #[inline(always)]
        pub const fn pd07(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "desc PD07."]
        #[inline(always)]
        pub const fn set_pd07(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
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
    impl Default for Pdpd {
        #[inline(always)]
        fn default() -> Pdpd {
            Pdpd(0)
        }
    }
    impl core::fmt::Debug for Pdpd {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Pdpd")
                .field("pd00", &self.pd00())
                .field("pd01", &self.pd01())
                .field("pd02", &self.pd02())
                .field("pd03", &self.pd03())
                .field("pd04", &self.pd04())
                .field("pd05", &self.pd05())
                .field("pd06", &self.pd06())
                .field("pd07", &self.pd07())
                .field("rsv", &self.rsv())
                .finish()
        }
    }
    #[doc = "desc PDPU."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Pdpu(pub u32);
    impl Pdpu {
        #[doc = "desc PD00."]
        #[must_use]
        #[inline(always)]
        pub const fn pd00(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "desc PD00."]
        #[inline(always)]
        pub const fn set_pd00(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "desc PD01."]
        #[must_use]
        #[inline(always)]
        pub const fn pd01(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "desc PD01."]
        #[inline(always)]
        pub const fn set_pd01(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "desc PD02."]
        #[must_use]
        #[inline(always)]
        pub const fn pd02(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "desc PD02."]
        #[inline(always)]
        pub const fn set_pd02(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "desc PD03."]
        #[must_use]
        #[inline(always)]
        pub const fn pd03(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "desc PD03."]
        #[inline(always)]
        pub const fn set_pd03(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "desc PD04."]
        #[must_use]
        #[inline(always)]
        pub const fn pd04(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "desc PD04."]
        #[inline(always)]
        pub const fn set_pd04(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "desc PD05."]
        #[must_use]
        #[inline(always)]
        pub const fn pd05(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "desc PD05."]
        #[inline(always)]
        pub const fn set_pd05(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "desc PD06."]
        #[must_use]
        #[inline(always)]
        pub const fn pd06(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "desc PD06."]
        #[inline(always)]
        pub const fn set_pd06(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "desc PD07."]
        #[must_use]
        #[inline(always)]
        pub const fn pd07(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "desc PD07."]
        #[inline(always)]
        pub const fn set_pd07(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
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
    impl Default for Pdpu {
        #[inline(always)]
        fn default() -> Pdpu {
            Pdpu(0)
        }
    }
    impl core::fmt::Debug for Pdpu {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Pdpu")
                .field("pd00", &self.pd00())
                .field("pd01", &self.pd01())
                .field("pd02", &self.pd02())
                .field("pd03", &self.pd03())
                .field("pd04", &self.pd04())
                .field("pd05", &self.pd05())
                .field("pd06", &self.pd06())
                .field("pd07", &self.pd07())
                .field("rsv", &self.rsv())
                .finish()
        }
    }
    #[doc = "desc PDRIE."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Pdrie(pub u32);
    impl Pdrie {
        #[doc = "desc PD00."]
        #[must_use]
        #[inline(always)]
        pub const fn pd00(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "desc PD00."]
        #[inline(always)]
        pub const fn set_pd00(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "desc PD01."]
        #[must_use]
        #[inline(always)]
        pub const fn pd01(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "desc PD01."]
        #[inline(always)]
        pub const fn set_pd01(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "desc PD02."]
        #[must_use]
        #[inline(always)]
        pub const fn pd02(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "desc PD02."]
        #[inline(always)]
        pub const fn set_pd02(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "desc PD03."]
        #[must_use]
        #[inline(always)]
        pub const fn pd03(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "desc PD03."]
        #[inline(always)]
        pub const fn set_pd03(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "desc PD04."]
        #[must_use]
        #[inline(always)]
        pub const fn pd04(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "desc PD04."]
        #[inline(always)]
        pub const fn set_pd04(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "desc PD05."]
        #[must_use]
        #[inline(always)]
        pub const fn pd05(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "desc PD05."]
        #[inline(always)]
        pub const fn set_pd05(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "desc PD06."]
        #[must_use]
        #[inline(always)]
        pub const fn pd06(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "desc PD06."]
        #[inline(always)]
        pub const fn set_pd06(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "desc PD07."]
        #[must_use]
        #[inline(always)]
        pub const fn pd07(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "desc PD07."]
        #[inline(always)]
        pub const fn set_pd07(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
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
    impl Default for Pdrie {
        #[inline(always)]
        fn default() -> Pdrie {
            Pdrie(0)
        }
    }
    impl core::fmt::Debug for Pdrie {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Pdrie")
                .field("pd00", &self.pd00())
                .field("pd01", &self.pd01())
                .field("pd02", &self.pd02())
                .field("pd03", &self.pd03())
                .field("pd04", &self.pd04())
                .field("pd05", &self.pd05())
                .field("pd06", &self.pd06())
                .field("pd07", &self.pd07())
                .field("rsv", &self.rsv())
                .finish()
        }
    }
    #[doc = "desc TIMCPS."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Timcps(pub u32);
    impl Timcps {
        #[doc = "desc TIM0_CA."]
        #[must_use]
        #[inline(always)]
        pub const fn tim0_ca(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x07;
            val as u8
        }
        #[doc = "desc TIM0_CA."]
        #[inline(always)]
        pub const fn set_tim0_ca(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 0usize)) | (((val as u32) & 0x07) << 0usize);
        }
        #[doc = "desc TIM1_CA."]
        #[must_use]
        #[inline(always)]
        pub const fn tim1_ca(&self) -> u8 {
            let val = (self.0 >> 3usize) & 0x07;
            val as u8
        }
        #[doc = "desc TIM1_CA."]
        #[inline(always)]
        pub const fn set_tim1_ca(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 3usize)) | (((val as u32) & 0x07) << 3usize);
        }
        #[doc = "desc TIM2_CA."]
        #[must_use]
        #[inline(always)]
        pub const fn tim2_ca(&self) -> u8 {
            let val = (self.0 >> 6usize) & 0x07;
            val as u8
        }
        #[doc = "desc TIM2_CA."]
        #[inline(always)]
        pub const fn set_tim2_ca(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 6usize)) | (((val as u32) & 0x07) << 6usize);
        }
        #[doc = "desc TIM3_CA."]
        #[must_use]
        #[inline(always)]
        pub const fn tim3_ca(&self) -> u8 {
            let val = (self.0 >> 9usize) & 0x07;
            val as u8
        }
        #[doc = "desc TIM3_CA."]
        #[inline(always)]
        pub const fn set_tim3_ca(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 9usize)) | (((val as u32) & 0x07) << 9usize);
        }
        #[doc = "desc TIM3_CB."]
        #[must_use]
        #[inline(always)]
        pub const fn tim3_cb(&self) -> u8 {
            let val = (self.0 >> 12usize) & 0x07;
            val as u8
        }
        #[doc = "desc TIM3_CB."]
        #[inline(always)]
        pub const fn set_tim3_cb(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 12usize)) | (((val as u32) & 0x07) << 12usize);
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
    impl Default for Timcps {
        #[inline(always)]
        fn default() -> Timcps {
            Timcps(0)
        }
    }
    impl core::fmt::Debug for Timcps {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Timcps")
                .field("tim0_ca", &self.tim0_ca())
                .field("tim1_ca", &self.tim1_ca())
                .field("tim2_ca", &self.tim2_ca())
                .field("tim3_ca", &self.tim3_ca())
                .field("tim3_cb", &self.tim3_cb())
                .field("rsv", &self.rsv())
                .finish()
        }
    }
    #[doc = "desc TIMES."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Times(pub u32);
    impl Times {
        #[doc = "desc TIM0_E."]
        #[must_use]
        #[inline(always)]
        pub const fn tim0_e(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x07;
            val as u8
        }
        #[doc = "desc TIM0_E."]
        #[inline(always)]
        pub const fn set_tim0_e(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 0usize)) | (((val as u32) & 0x07) << 0usize);
        }
        #[doc = "desc TIM1_E."]
        #[must_use]
        #[inline(always)]
        pub const fn tim1_e(&self) -> u8 {
            let val = (self.0 >> 3usize) & 0x07;
            val as u8
        }
        #[doc = "desc TIM1_E."]
        #[inline(always)]
        pub const fn set_tim1_e(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 3usize)) | (((val as u32) & 0x07) << 3usize);
        }
        #[doc = "desc TIM2_E."]
        #[must_use]
        #[inline(always)]
        pub const fn tim2_e(&self) -> u8 {
            let val = (self.0 >> 6usize) & 0x07;
            val as u8
        }
        #[doc = "desc TIM2_E."]
        #[inline(always)]
        pub const fn set_tim2_e(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 6usize)) | (((val as u32) & 0x07) << 6usize);
        }
        #[doc = "desc TIM3_E."]
        #[must_use]
        #[inline(always)]
        pub const fn tim3_e(&self) -> u8 {
            let val = (self.0 >> 9usize) & 0x07;
            val as u8
        }
        #[doc = "desc TIM3_E."]
        #[inline(always)]
        pub const fn set_tim3_e(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 9usize)) | (((val as u32) & 0x07) << 9usize);
        }
        #[doc = "desc LPTIM_E."]
        #[must_use]
        #[inline(always)]
        pub const fn lptim_e(&self) -> u8 {
            let val = (self.0 >> 12usize) & 0x07;
            val as u8
        }
        #[doc = "desc LPTIM_E."]
        #[inline(always)]
        pub const fn set_lptim_e(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 12usize)) | (((val as u32) & 0x07) << 12usize);
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
    impl Default for Times {
        #[inline(always)]
        fn default() -> Times {
            Times(0)
        }
    }
    impl core::fmt::Debug for Times {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Times")
                .field("tim0_e", &self.tim0_e())
                .field("tim1_e", &self.tim1_e())
                .field("tim2_e", &self.tim2_e())
                .field("tim3_e", &self.tim3_e())
                .field("lptim_e", &self.lptim_e())
                .field("rsv", &self.rsv())
                .finish()
        }
    }
    #[doc = "desc TIMGS."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Timgs(pub u32);
    impl Timgs {
        #[doc = "desc TIM0_G."]
        #[must_use]
        #[inline(always)]
        pub const fn tim0_g(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x07;
            val as u8
        }
        #[doc = "desc TIM0_G."]
        #[inline(always)]
        pub const fn set_tim0_g(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 0usize)) | (((val as u32) & 0x07) << 0usize);
        }
        #[doc = "desc TIM1_G."]
        #[must_use]
        #[inline(always)]
        pub const fn tim1_g(&self) -> u8 {
            let val = (self.0 >> 3usize) & 0x07;
            val as u8
        }
        #[doc = "desc TIM1_G."]
        #[inline(always)]
        pub const fn set_tim1_g(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 3usize)) | (((val as u32) & 0x07) << 3usize);
        }
        #[doc = "desc TIM2_G."]
        #[must_use]
        #[inline(always)]
        pub const fn tim2_g(&self) -> u8 {
            let val = (self.0 >> 6usize) & 0x07;
            val as u8
        }
        #[doc = "desc TIM2_G."]
        #[inline(always)]
        pub const fn set_tim2_g(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 6usize)) | (((val as u32) & 0x07) << 6usize);
        }
        #[doc = "desc TIM3_G."]
        #[must_use]
        #[inline(always)]
        pub const fn tim3_g(&self) -> u8 {
            let val = (self.0 >> 9usize) & 0x07;
            val as u8
        }
        #[doc = "desc TIM3_G."]
        #[inline(always)]
        pub const fn set_tim3_g(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 9usize)) | (((val as u32) & 0x07) << 9usize);
        }
        #[doc = "desc LPTIM_G."]
        #[must_use]
        #[inline(always)]
        pub const fn lptim_g(&self) -> u8 {
            let val = (self.0 >> 12usize) & 0x07;
            val as u8
        }
        #[doc = "desc LPTIM_G."]
        #[inline(always)]
        pub const fn set_lptim_g(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 12usize)) | (((val as u32) & 0x07) << 12usize);
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
    impl Default for Timgs {
        #[inline(always)]
        fn default() -> Timgs {
            Timgs(0)
        }
    }
    impl core::fmt::Debug for Timgs {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Timgs")
                .field("tim0_g", &self.tim0_g())
                .field("tim1_g", &self.tim1_g())
                .field("tim2_g", &self.tim2_g())
                .field("tim3_g", &self.tim3_g())
                .field("lptim_g", &self.lptim_g())
                .field("rsv", &self.rsv())
                .finish()
        }
    }
}
