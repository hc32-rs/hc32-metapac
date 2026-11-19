#![allow(clippy::missing_safety_doc)]
#![allow(clippy::identity_op)]
#![allow(clippy::unnecessary_cast)]
#![allow(clippy::erasing_op)]

#[doc = "desc DMA."]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dma {
    ptr: *mut u8,
}
unsafe impl Send for Dma {}
unsafe impl Sync for Dma {}
impl Dma {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "desc EN."]
    #[inline(always)]
    pub const fn en(self) -> crate::common::Reg<regs::En, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0usize) as _) }
    }
    #[doc = "desc INTSTAT0."]
    #[inline(always)]
    pub const fn intstat0(self) -> crate::common::Reg<regs::Intstat0, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x04usize) as _) }
    }
    #[doc = "desc INTSTAT1."]
    #[inline(always)]
    pub const fn intstat1(self) -> crate::common::Reg<regs::Intstat1, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x08usize) as _) }
    }
    #[doc = "desc INTMASK0."]
    #[inline(always)]
    pub const fn intmask0(self) -> crate::common::Reg<regs::Intmask0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0cusize) as _) }
    }
    #[doc = "desc INTMASK1."]
    #[inline(always)]
    pub const fn intmask1(self) -> crate::common::Reg<regs::Intmask1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x10usize) as _) }
    }
    #[doc = "desc INTCLR0."]
    #[inline(always)]
    pub const fn intclr0(self) -> crate::common::Reg<regs::Intclr0, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x14usize) as _) }
    }
    #[doc = "desc INTCLR1."]
    #[inline(always)]
    pub const fn intclr1(self) -> crate::common::Reg<regs::Intclr1, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x18usize) as _) }
    }
    #[doc = "desc CHEN."]
    #[inline(always)]
    pub const fn chen(self) -> crate::common::Reg<regs::Chen, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1cusize) as _) }
    }
    #[doc = "desc REQSTAT."]
    #[inline(always)]
    pub const fn reqstat(self) -> crate::common::Reg<regs::Reqstat, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x20usize) as _) }
    }
    #[doc = "desc CHSTAT."]
    #[inline(always)]
    pub const fn chstat(self) -> crate::common::Reg<regs::Chstat, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x24usize) as _) }
    }
    #[doc = "desc RCFGCTL."]
    #[inline(always)]
    pub const fn rcfgctl(self) -> crate::common::Reg<regs::Rcfgctl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x2cusize) as _) }
    }
    #[doc = "desc SWREQ."]
    #[inline(always)]
    pub const fn swreq(self) -> crate::common::Reg<regs::Swreq, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x30usize) as _) }
    }
    #[doc = "desc CHENCLR."]
    #[inline(always)]
    pub const fn chenclr(self) -> crate::common::Reg<regs::Chenclr, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x34usize) as _) }
    }
    #[doc = "desc SAR0."]
    #[inline(always)]
    pub const fn sar0(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x40usize) as _) }
    }
    #[doc = "desc DAR0."]
    #[inline(always)]
    pub const fn dar0(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x44usize) as _) }
    }
    #[doc = "desc DTCTL0."]
    #[inline(always)]
    pub const fn dtctl0(self) -> crate::common::Reg<regs::Dtctl0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x48usize) as _) }
    }
    #[doc = "desc RPT0."]
    #[inline(always)]
    pub const fn rpt0(self) -> crate::common::Reg<regs::Rpt0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x4cusize) as _) }
    }
    #[doc = "desc RPTB0."]
    #[inline(always)]
    pub const fn rptb0(self) -> crate::common::Reg<regs::Rptb0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x4cusize) as _) }
    }
    #[doc = "desc SNSEQCTL0."]
    #[inline(always)]
    pub const fn snseqctl0(self) -> crate::common::Reg<regs::Snseqctl0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x50usize) as _) }
    }
    #[doc = "desc SNSEQCTLB0."]
    #[inline(always)]
    pub const fn snseqctlb0(self) -> crate::common::Reg<regs::Snseqctlb0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x50usize) as _) }
    }
    #[doc = "desc DNSEQCTL0."]
    #[inline(always)]
    pub const fn dnseqctl0(self) -> crate::common::Reg<regs::Dnseqctl0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x54usize) as _) }
    }
    #[doc = "desc DNSEQCTLB0."]
    #[inline(always)]
    pub const fn dnseqctlb0(self) -> crate::common::Reg<regs::Dnseqctlb0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x54usize) as _) }
    }
    #[doc = "desc LLP0."]
    #[inline(always)]
    pub const fn llp0(self) -> crate::common::Reg<regs::Llp0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x58usize) as _) }
    }
    #[doc = "desc CHCTL0."]
    #[inline(always)]
    pub const fn chctl0(self) -> crate::common::Reg<regs::Chctl0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x5cusize) as _) }
    }
    #[doc = "desc MONSAR0."]
    #[inline(always)]
    pub const fn monsar0(self) -> crate::common::Reg<u32, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x60usize) as _) }
    }
    #[doc = "desc MONDAR0."]
    #[inline(always)]
    pub const fn mondar0(self) -> crate::common::Reg<u32, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x64usize) as _) }
    }
    #[doc = "desc MONDTCTL0."]
    #[inline(always)]
    pub const fn mondtctl0(self) -> crate::common::Reg<regs::Mondtctl0, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x68usize) as _) }
    }
    #[doc = "desc MONRPT0."]
    #[inline(always)]
    pub const fn monrpt0(self) -> crate::common::Reg<regs::Monrpt0, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x6cusize) as _) }
    }
    #[doc = "desc MONSNSEQCTL0."]
    #[inline(always)]
    pub const fn monsnseqctl0(self) -> crate::common::Reg<regs::Monsnseqctl0, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x70usize) as _) }
    }
    #[doc = "desc MONDNSEQCTL0."]
    #[inline(always)]
    pub const fn mondnseqctl0(self) -> crate::common::Reg<regs::Mondnseqctl0, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x74usize) as _) }
    }
    #[doc = "desc SAR1."]
    #[inline(always)]
    pub const fn sar1(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x80usize) as _) }
    }
    #[doc = "desc DAR1."]
    #[inline(always)]
    pub const fn dar1(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x84usize) as _) }
    }
    #[doc = "desc DTCTL1."]
    #[inline(always)]
    pub const fn dtctl1(self) -> crate::common::Reg<regs::Dtctl1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x88usize) as _) }
    }
    #[doc = "desc RPT1."]
    #[inline(always)]
    pub const fn rpt1(self) -> crate::common::Reg<regs::Rpt1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x8cusize) as _) }
    }
    #[doc = "desc RPTB1."]
    #[inline(always)]
    pub const fn rptb1(self) -> crate::common::Reg<regs::Rptb1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x8cusize) as _) }
    }
    #[doc = "desc SNSEQCTL1."]
    #[inline(always)]
    pub const fn snseqctl1(self) -> crate::common::Reg<regs::Snseqctl1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x90usize) as _) }
    }
    #[doc = "desc SNSEQCTLB1."]
    #[inline(always)]
    pub const fn snseqctlb1(self) -> crate::common::Reg<regs::Snseqctlb1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x90usize) as _) }
    }
    #[doc = "desc DNSEQCTL1."]
    #[inline(always)]
    pub const fn dnseqctl1(self) -> crate::common::Reg<regs::Dnseqctl1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x94usize) as _) }
    }
    #[doc = "desc DNSEQCTLB1."]
    #[inline(always)]
    pub const fn dnseqctlb1(self) -> crate::common::Reg<regs::Dnseqctlb1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x94usize) as _) }
    }
    #[doc = "desc LLP1."]
    #[inline(always)]
    pub const fn llp1(self) -> crate::common::Reg<regs::Llp1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x98usize) as _) }
    }
    #[doc = "desc CHCTL1."]
    #[inline(always)]
    pub const fn chctl1(self) -> crate::common::Reg<regs::Chctl1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x9cusize) as _) }
    }
    #[doc = "desc MONSAR1."]
    #[inline(always)]
    pub const fn monsar1(self) -> crate::common::Reg<u32, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xa0usize) as _) }
    }
    #[doc = "desc MONDAR1."]
    #[inline(always)]
    pub const fn mondar1(self) -> crate::common::Reg<u32, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xa4usize) as _) }
    }
    #[doc = "desc MONDTCTL1."]
    #[inline(always)]
    pub const fn mondtctl1(self) -> crate::common::Reg<regs::Mondtctl1, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xa8usize) as _) }
    }
    #[doc = "desc MONRPT1."]
    #[inline(always)]
    pub const fn monrpt1(self) -> crate::common::Reg<regs::Monrpt1, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xacusize) as _) }
    }
    #[doc = "desc MONSNSEQCTL1."]
    #[inline(always)]
    pub const fn monsnseqctl1(self) -> crate::common::Reg<regs::Monsnseqctl1, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xb0usize) as _) }
    }
    #[doc = "desc MONDNSEQCTL1."]
    #[inline(always)]
    pub const fn mondnseqctl1(self) -> crate::common::Reg<regs::Mondnseqctl1, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xb4usize) as _) }
    }
    #[doc = "desc SAR2."]
    #[inline(always)]
    pub const fn sar2(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xc0usize) as _) }
    }
    #[doc = "desc DAR2."]
    #[inline(always)]
    pub const fn dar2(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xc4usize) as _) }
    }
    #[doc = "desc DTCTL2."]
    #[inline(always)]
    pub const fn dtctl2(self) -> crate::common::Reg<regs::Dtctl2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xc8usize) as _) }
    }
    #[doc = "desc RPT2."]
    #[inline(always)]
    pub const fn rpt2(self) -> crate::common::Reg<regs::Rpt2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xccusize) as _) }
    }
    #[doc = "desc RPTB2."]
    #[inline(always)]
    pub const fn rptb2(self) -> crate::common::Reg<regs::Rptb2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xccusize) as _) }
    }
    #[doc = "desc SNSEQCTL2."]
    #[inline(always)]
    pub const fn snseqctl2(self) -> crate::common::Reg<regs::Snseqctl2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xd0usize) as _) }
    }
    #[doc = "desc SNSEQCTLB2."]
    #[inline(always)]
    pub const fn snseqctlb2(self) -> crate::common::Reg<regs::Snseqctlb2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xd0usize) as _) }
    }
    #[doc = "desc DNSEQCTL2."]
    #[inline(always)]
    pub const fn dnseqctl2(self) -> crate::common::Reg<regs::Dnseqctl2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xd4usize) as _) }
    }
    #[doc = "desc DNSEQCTLB2."]
    #[inline(always)]
    pub const fn dnseqctlb2(self) -> crate::common::Reg<regs::Dnseqctlb2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xd4usize) as _) }
    }
    #[doc = "desc LLP2."]
    #[inline(always)]
    pub const fn llp2(self) -> crate::common::Reg<regs::Llp2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xd8usize) as _) }
    }
    #[doc = "desc CHCTL2."]
    #[inline(always)]
    pub const fn chctl2(self) -> crate::common::Reg<regs::Chctl2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xdcusize) as _) }
    }
    #[doc = "desc MONSAR2."]
    #[inline(always)]
    pub const fn monsar2(self) -> crate::common::Reg<u32, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xe0usize) as _) }
    }
    #[doc = "desc MONDAR2."]
    #[inline(always)]
    pub const fn mondar2(self) -> crate::common::Reg<u32, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xe4usize) as _) }
    }
    #[doc = "desc MONDTCTL2."]
    #[inline(always)]
    pub const fn mondtctl2(self) -> crate::common::Reg<regs::Mondtctl2, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xe8usize) as _) }
    }
    #[doc = "desc MONRPT2."]
    #[inline(always)]
    pub const fn monrpt2(self) -> crate::common::Reg<regs::Monrpt2, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xecusize) as _) }
    }
    #[doc = "desc MONSNSEQCTL2."]
    #[inline(always)]
    pub const fn monsnseqctl2(self) -> crate::common::Reg<regs::Monsnseqctl2, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xf0usize) as _) }
    }
    #[doc = "desc MONDNSEQCTL2."]
    #[inline(always)]
    pub const fn mondnseqctl2(self) -> crate::common::Reg<regs::Mondnseqctl2, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xf4usize) as _) }
    }
    #[doc = "desc SAR3."]
    #[inline(always)]
    pub const fn sar3(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0100usize) as _) }
    }
    #[doc = "desc DAR3."]
    #[inline(always)]
    pub const fn dar3(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0104usize) as _) }
    }
    #[doc = "desc DTCTL3."]
    #[inline(always)]
    pub const fn dtctl3(self) -> crate::common::Reg<regs::Dtctl3, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0108usize) as _) }
    }
    #[doc = "desc RPT3."]
    #[inline(always)]
    pub const fn rpt3(self) -> crate::common::Reg<regs::Rpt3, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x010cusize) as _) }
    }
    #[doc = "desc RPTB3."]
    #[inline(always)]
    pub const fn rptb3(self) -> crate::common::Reg<regs::Rptb3, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x010cusize) as _) }
    }
    #[doc = "desc SNSEQCTL3."]
    #[inline(always)]
    pub const fn snseqctl3(self) -> crate::common::Reg<regs::Snseqctl3, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0110usize) as _) }
    }
    #[doc = "desc SNSEQCTLB3."]
    #[inline(always)]
    pub const fn snseqctlb3(self) -> crate::common::Reg<regs::Snseqctlb3, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0110usize) as _) }
    }
    #[doc = "desc DNSEQCTL3."]
    #[inline(always)]
    pub const fn dnseqctl3(self) -> crate::common::Reg<regs::Dnseqctl3, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0114usize) as _) }
    }
    #[doc = "desc DNSEQCTLB3."]
    #[inline(always)]
    pub const fn dnseqctlb3(self) -> crate::common::Reg<regs::Dnseqctlb3, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0114usize) as _) }
    }
    #[doc = "desc LLP3."]
    #[inline(always)]
    pub const fn llp3(self) -> crate::common::Reg<regs::Llp3, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0118usize) as _) }
    }
    #[doc = "desc CHCTL3."]
    #[inline(always)]
    pub const fn chctl3(self) -> crate::common::Reg<regs::Chctl3, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x011cusize) as _) }
    }
    #[doc = "desc MONSAR3."]
    #[inline(always)]
    pub const fn monsar3(self) -> crate::common::Reg<u32, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0120usize) as _) }
    }
    #[doc = "desc MONDAR3."]
    #[inline(always)]
    pub const fn mondar3(self) -> crate::common::Reg<u32, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0124usize) as _) }
    }
    #[doc = "desc MONDTCTL3."]
    #[inline(always)]
    pub const fn mondtctl3(self) -> crate::common::Reg<regs::Mondtctl3, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0128usize) as _) }
    }
    #[doc = "desc MONRPT3."]
    #[inline(always)]
    pub const fn monrpt3(self) -> crate::common::Reg<regs::Monrpt3, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x012cusize) as _) }
    }
    #[doc = "desc MONSNSEQCTL3."]
    #[inline(always)]
    pub const fn monsnseqctl3(self) -> crate::common::Reg<regs::Monsnseqctl3, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0130usize) as _) }
    }
    #[doc = "desc MONDNSEQCTL3."]
    #[inline(always)]
    pub const fn mondnseqctl3(self) -> crate::common::Reg<regs::Mondnseqctl3, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0134usize) as _) }
    }
    #[doc = "desc SAR4."]
    #[inline(always)]
    pub const fn sar4(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0140usize) as _) }
    }
    #[doc = "desc DAR4."]
    #[inline(always)]
    pub const fn dar4(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0144usize) as _) }
    }
    #[doc = "desc DTCTL4."]
    #[inline(always)]
    pub const fn dtctl4(self) -> crate::common::Reg<regs::Dtctl4, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0148usize) as _) }
    }
    #[doc = "desc RPT4."]
    #[inline(always)]
    pub const fn rpt4(self) -> crate::common::Reg<regs::Rpt4, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x014cusize) as _) }
    }
    #[doc = "desc RPTB4."]
    #[inline(always)]
    pub const fn rptb4(self) -> crate::common::Reg<regs::Rptb4, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x014cusize) as _) }
    }
    #[doc = "desc SNSEQCTL4."]
    #[inline(always)]
    pub const fn snseqctl4(self) -> crate::common::Reg<regs::Snseqctl4, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0150usize) as _) }
    }
    #[doc = "desc SNSEQCTLB4."]
    #[inline(always)]
    pub const fn snseqctlb4(self) -> crate::common::Reg<regs::Snseqctlb4, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0150usize) as _) }
    }
    #[doc = "desc DNSEQCTL4."]
    #[inline(always)]
    pub const fn dnseqctl4(self) -> crate::common::Reg<regs::Dnseqctl4, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0154usize) as _) }
    }
    #[doc = "desc DNSEQCTLB4."]
    #[inline(always)]
    pub const fn dnseqctlb4(self) -> crate::common::Reg<regs::Dnseqctlb4, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0154usize) as _) }
    }
    #[doc = "desc LLP4."]
    #[inline(always)]
    pub const fn llp4(self) -> crate::common::Reg<regs::Llp4, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0158usize) as _) }
    }
    #[doc = "desc CHCTL4."]
    #[inline(always)]
    pub const fn chctl4(self) -> crate::common::Reg<regs::Chctl4, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x015cusize) as _) }
    }
    #[doc = "desc MONSAR4."]
    #[inline(always)]
    pub const fn monsar4(self) -> crate::common::Reg<u32, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0160usize) as _) }
    }
    #[doc = "desc MONDAR4."]
    #[inline(always)]
    pub const fn mondar4(self) -> crate::common::Reg<u32, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0164usize) as _) }
    }
    #[doc = "desc MONDTCTL4."]
    #[inline(always)]
    pub const fn mondtctl4(self) -> crate::common::Reg<regs::Mondtctl4, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0168usize) as _) }
    }
    #[doc = "desc MONRPT4."]
    #[inline(always)]
    pub const fn monrpt4(self) -> crate::common::Reg<regs::Monrpt4, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x016cusize) as _) }
    }
    #[doc = "desc MONSNSEQCTL4."]
    #[inline(always)]
    pub const fn monsnseqctl4(self) -> crate::common::Reg<regs::Monsnseqctl4, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0170usize) as _) }
    }
    #[doc = "desc MONDNSEQCTL4."]
    #[inline(always)]
    pub const fn mondnseqctl4(self) -> crate::common::Reg<regs::Mondnseqctl4, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0174usize) as _) }
    }
    #[doc = "desc SAR5."]
    #[inline(always)]
    pub const fn sar5(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0180usize) as _) }
    }
    #[doc = "desc DAR5."]
    #[inline(always)]
    pub const fn dar5(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0184usize) as _) }
    }
    #[doc = "desc DTCTL5."]
    #[inline(always)]
    pub const fn dtctl5(self) -> crate::common::Reg<regs::Dtctl5, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0188usize) as _) }
    }
    #[doc = "desc RPT5."]
    #[inline(always)]
    pub const fn rpt5(self) -> crate::common::Reg<regs::Rpt5, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x018cusize) as _) }
    }
    #[doc = "desc RPTB5."]
    #[inline(always)]
    pub const fn rptb5(self) -> crate::common::Reg<regs::Rptb5, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x018cusize) as _) }
    }
    #[doc = "desc SNSEQCTL5."]
    #[inline(always)]
    pub const fn snseqctl5(self) -> crate::common::Reg<regs::Snseqctl5, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0190usize) as _) }
    }
    #[doc = "desc SNSEQCTLB5."]
    #[inline(always)]
    pub const fn snseqctlb5(self) -> crate::common::Reg<regs::Snseqctlb5, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0190usize) as _) }
    }
    #[doc = "desc DNSEQCTL5."]
    #[inline(always)]
    pub const fn dnseqctl5(self) -> crate::common::Reg<regs::Dnseqctl5, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0194usize) as _) }
    }
    #[doc = "desc DNSEQCTLB5."]
    #[inline(always)]
    pub const fn dnseqctlb5(self) -> crate::common::Reg<regs::Dnseqctlb5, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0194usize) as _) }
    }
    #[doc = "desc LLP5."]
    #[inline(always)]
    pub const fn llp5(self) -> crate::common::Reg<regs::Llp5, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0198usize) as _) }
    }
    #[doc = "desc CHCTL5."]
    #[inline(always)]
    pub const fn chctl5(self) -> crate::common::Reg<regs::Chctl5, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x019cusize) as _) }
    }
    #[doc = "desc MONSAR5."]
    #[inline(always)]
    pub const fn monsar5(self) -> crate::common::Reg<u32, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x01a0usize) as _) }
    }
    #[doc = "desc MONDAR5."]
    #[inline(always)]
    pub const fn mondar5(self) -> crate::common::Reg<u32, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x01a4usize) as _) }
    }
    #[doc = "desc MONDTCTL5."]
    #[inline(always)]
    pub const fn mondtctl5(self) -> crate::common::Reg<regs::Mondtctl5, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x01a8usize) as _) }
    }
    #[doc = "desc MONRPT5."]
    #[inline(always)]
    pub const fn monrpt5(self) -> crate::common::Reg<regs::Monrpt5, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x01acusize) as _) }
    }
    #[doc = "desc MONSNSEQCTL5."]
    #[inline(always)]
    pub const fn monsnseqctl5(self) -> crate::common::Reg<regs::Monsnseqctl5, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x01b0usize) as _) }
    }
    #[doc = "desc MONDNSEQCTL5."]
    #[inline(always)]
    pub const fn mondnseqctl5(self) -> crate::common::Reg<regs::Mondnseqctl5, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x01b4usize) as _) }
    }
}
pub mod regs {
    #[doc = "desc CHCTL0."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Chctl0(pub u32);
    impl Chctl0 {
        #[doc = "desc SINC."]
        #[must_use]
        #[inline(always)]
        pub const fn sinc(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x03;
            val as u8
        }
        #[doc = "desc SINC."]
        #[inline(always)]
        pub const fn set_sinc(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 0usize)) | (((val as u32) & 0x03) << 0usize);
        }
        #[doc = "desc DINC."]
        #[must_use]
        #[inline(always)]
        pub const fn dinc(&self) -> u8 {
            let val = (self.0 >> 2usize) & 0x03;
            val as u8
        }
        #[doc = "desc DINC."]
        #[inline(always)]
        pub const fn set_dinc(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 2usize)) | (((val as u32) & 0x03) << 2usize);
        }
        #[doc = "desc SRPTEN."]
        #[must_use]
        #[inline(always)]
        pub const fn srpten(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "desc SRPTEN."]
        #[inline(always)]
        pub const fn set_srpten(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "desc DRPTEN."]
        #[must_use]
        #[inline(always)]
        pub const fn drpten(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "desc DRPTEN."]
        #[inline(always)]
        pub const fn set_drpten(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "desc SNSEQEN."]
        #[must_use]
        #[inline(always)]
        pub const fn snseqen(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "desc SNSEQEN."]
        #[inline(always)]
        pub const fn set_snseqen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "desc DNSEQEN."]
        #[must_use]
        #[inline(always)]
        pub const fn dnseqen(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "desc DNSEQEN."]
        #[inline(always)]
        pub const fn set_dnseqen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "desc HSIZE."]
        #[must_use]
        #[inline(always)]
        pub const fn hsize(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0x03;
            val as u8
        }
        #[doc = "desc HSIZE."]
        #[inline(always)]
        pub const fn set_hsize(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 8usize)) | (((val as u32) & 0x03) << 8usize);
        }
        #[doc = "desc LLPEN."]
        #[must_use]
        #[inline(always)]
        pub const fn llpen(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "desc LLPEN."]
        #[inline(always)]
        pub const fn set_llpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[doc = "desc LLPRUN."]
        #[must_use]
        #[inline(always)]
        pub const fn llprun(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "desc LLPRUN."]
        #[inline(always)]
        pub const fn set_llprun(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[doc = "desc IE."]
        #[must_use]
        #[inline(always)]
        pub const fn ie(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "desc IE."]
        #[inline(always)]
        pub const fn set_ie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "desc HPROT."]
        #[must_use]
        #[inline(always)]
        pub const fn hprot(&self) -> u8 {
            let val = (self.0 >> 14usize) & 0x03;
            val as u8
        }
        #[doc = "desc HPROT."]
        #[inline(always)]
        pub const fn set_hprot(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 14usize)) | (((val as u32) & 0x03) << 14usize);
        }
    }
    impl Default for Chctl0 {
        #[inline(always)]
        fn default() -> Chctl0 {
            Chctl0(0)
        }
    }
    impl core::fmt::Debug for Chctl0 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Chctl0")
                .field("sinc", &self.sinc())
                .field("dinc", &self.dinc())
                .field("srpten", &self.srpten())
                .field("drpten", &self.drpten())
                .field("snseqen", &self.snseqen())
                .field("dnseqen", &self.dnseqen())
                .field("hsize", &self.hsize())
                .field("llpen", &self.llpen())
                .field("llprun", &self.llprun())
                .field("ie", &self.ie())
                .field("hprot", &self.hprot())
                .finish()
        }
    }
    #[doc = "desc CHCTL1."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Chctl1(pub u32);
    impl Chctl1 {
        #[doc = "desc SINC."]
        #[must_use]
        #[inline(always)]
        pub const fn sinc(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x03;
            val as u8
        }
        #[doc = "desc SINC."]
        #[inline(always)]
        pub const fn set_sinc(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 0usize)) | (((val as u32) & 0x03) << 0usize);
        }
        #[doc = "desc DINC."]
        #[must_use]
        #[inline(always)]
        pub const fn dinc(&self) -> u8 {
            let val = (self.0 >> 2usize) & 0x03;
            val as u8
        }
        #[doc = "desc DINC."]
        #[inline(always)]
        pub const fn set_dinc(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 2usize)) | (((val as u32) & 0x03) << 2usize);
        }
        #[doc = "desc SRPTEN."]
        #[must_use]
        #[inline(always)]
        pub const fn srpten(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "desc SRPTEN."]
        #[inline(always)]
        pub const fn set_srpten(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "desc DRPTEN."]
        #[must_use]
        #[inline(always)]
        pub const fn drpten(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "desc DRPTEN."]
        #[inline(always)]
        pub const fn set_drpten(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "desc SNSEQEN."]
        #[must_use]
        #[inline(always)]
        pub const fn snseqen(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "desc SNSEQEN."]
        #[inline(always)]
        pub const fn set_snseqen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "desc DNSEQEN."]
        #[must_use]
        #[inline(always)]
        pub const fn dnseqen(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "desc DNSEQEN."]
        #[inline(always)]
        pub const fn set_dnseqen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "desc HSIZE."]
        #[must_use]
        #[inline(always)]
        pub const fn hsize(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0x03;
            val as u8
        }
        #[doc = "desc HSIZE."]
        #[inline(always)]
        pub const fn set_hsize(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 8usize)) | (((val as u32) & 0x03) << 8usize);
        }
        #[doc = "desc LLPEN."]
        #[must_use]
        #[inline(always)]
        pub const fn llpen(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "desc LLPEN."]
        #[inline(always)]
        pub const fn set_llpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[doc = "desc LLPRUN."]
        #[must_use]
        #[inline(always)]
        pub const fn llprun(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "desc LLPRUN."]
        #[inline(always)]
        pub const fn set_llprun(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[doc = "desc IE."]
        #[must_use]
        #[inline(always)]
        pub const fn ie(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "desc IE."]
        #[inline(always)]
        pub const fn set_ie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "desc HPROT."]
        #[must_use]
        #[inline(always)]
        pub const fn hprot(&self) -> u8 {
            let val = (self.0 >> 14usize) & 0x03;
            val as u8
        }
        #[doc = "desc HPROT."]
        #[inline(always)]
        pub const fn set_hprot(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 14usize)) | (((val as u32) & 0x03) << 14usize);
        }
    }
    impl Default for Chctl1 {
        #[inline(always)]
        fn default() -> Chctl1 {
            Chctl1(0)
        }
    }
    impl core::fmt::Debug for Chctl1 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Chctl1")
                .field("sinc", &self.sinc())
                .field("dinc", &self.dinc())
                .field("srpten", &self.srpten())
                .field("drpten", &self.drpten())
                .field("snseqen", &self.snseqen())
                .field("dnseqen", &self.dnseqen())
                .field("hsize", &self.hsize())
                .field("llpen", &self.llpen())
                .field("llprun", &self.llprun())
                .field("ie", &self.ie())
                .field("hprot", &self.hprot())
                .finish()
        }
    }
    #[doc = "desc CHCTL2."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Chctl2(pub u32);
    impl Chctl2 {
        #[doc = "desc SINC."]
        #[must_use]
        #[inline(always)]
        pub const fn sinc(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x03;
            val as u8
        }
        #[doc = "desc SINC."]
        #[inline(always)]
        pub const fn set_sinc(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 0usize)) | (((val as u32) & 0x03) << 0usize);
        }
        #[doc = "desc DINC."]
        #[must_use]
        #[inline(always)]
        pub const fn dinc(&self) -> u8 {
            let val = (self.0 >> 2usize) & 0x03;
            val as u8
        }
        #[doc = "desc DINC."]
        #[inline(always)]
        pub const fn set_dinc(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 2usize)) | (((val as u32) & 0x03) << 2usize);
        }
        #[doc = "desc SRPTEN."]
        #[must_use]
        #[inline(always)]
        pub const fn srpten(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "desc SRPTEN."]
        #[inline(always)]
        pub const fn set_srpten(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "desc DRPTEN."]
        #[must_use]
        #[inline(always)]
        pub const fn drpten(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "desc DRPTEN."]
        #[inline(always)]
        pub const fn set_drpten(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "desc SNSEQEN."]
        #[must_use]
        #[inline(always)]
        pub const fn snseqen(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "desc SNSEQEN."]
        #[inline(always)]
        pub const fn set_snseqen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "desc DNSEQEN."]
        #[must_use]
        #[inline(always)]
        pub const fn dnseqen(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "desc DNSEQEN."]
        #[inline(always)]
        pub const fn set_dnseqen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "desc HSIZE."]
        #[must_use]
        #[inline(always)]
        pub const fn hsize(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0x03;
            val as u8
        }
        #[doc = "desc HSIZE."]
        #[inline(always)]
        pub const fn set_hsize(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 8usize)) | (((val as u32) & 0x03) << 8usize);
        }
        #[doc = "desc LLPEN."]
        #[must_use]
        #[inline(always)]
        pub const fn llpen(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "desc LLPEN."]
        #[inline(always)]
        pub const fn set_llpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[doc = "desc LLPRUN."]
        #[must_use]
        #[inline(always)]
        pub const fn llprun(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "desc LLPRUN."]
        #[inline(always)]
        pub const fn set_llprun(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[doc = "desc IE."]
        #[must_use]
        #[inline(always)]
        pub const fn ie(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "desc IE."]
        #[inline(always)]
        pub const fn set_ie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "desc HPROT."]
        #[must_use]
        #[inline(always)]
        pub const fn hprot(&self) -> u8 {
            let val = (self.0 >> 14usize) & 0x03;
            val as u8
        }
        #[doc = "desc HPROT."]
        #[inline(always)]
        pub const fn set_hprot(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 14usize)) | (((val as u32) & 0x03) << 14usize);
        }
    }
    impl Default for Chctl2 {
        #[inline(always)]
        fn default() -> Chctl2 {
            Chctl2(0)
        }
    }
    impl core::fmt::Debug for Chctl2 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Chctl2")
                .field("sinc", &self.sinc())
                .field("dinc", &self.dinc())
                .field("srpten", &self.srpten())
                .field("drpten", &self.drpten())
                .field("snseqen", &self.snseqen())
                .field("dnseqen", &self.dnseqen())
                .field("hsize", &self.hsize())
                .field("llpen", &self.llpen())
                .field("llprun", &self.llprun())
                .field("ie", &self.ie())
                .field("hprot", &self.hprot())
                .finish()
        }
    }
    #[doc = "desc CHCTL3."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Chctl3(pub u32);
    impl Chctl3 {
        #[doc = "desc SINC."]
        #[must_use]
        #[inline(always)]
        pub const fn sinc(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x03;
            val as u8
        }
        #[doc = "desc SINC."]
        #[inline(always)]
        pub const fn set_sinc(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 0usize)) | (((val as u32) & 0x03) << 0usize);
        }
        #[doc = "desc DINC."]
        #[must_use]
        #[inline(always)]
        pub const fn dinc(&self) -> u8 {
            let val = (self.0 >> 2usize) & 0x03;
            val as u8
        }
        #[doc = "desc DINC."]
        #[inline(always)]
        pub const fn set_dinc(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 2usize)) | (((val as u32) & 0x03) << 2usize);
        }
        #[doc = "desc SRPTEN."]
        #[must_use]
        #[inline(always)]
        pub const fn srpten(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "desc SRPTEN."]
        #[inline(always)]
        pub const fn set_srpten(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "desc DRPTEN."]
        #[must_use]
        #[inline(always)]
        pub const fn drpten(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "desc DRPTEN."]
        #[inline(always)]
        pub const fn set_drpten(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "desc SNSEQEN."]
        #[must_use]
        #[inline(always)]
        pub const fn snseqen(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "desc SNSEQEN."]
        #[inline(always)]
        pub const fn set_snseqen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "desc DNSEQEN."]
        #[must_use]
        #[inline(always)]
        pub const fn dnseqen(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "desc DNSEQEN."]
        #[inline(always)]
        pub const fn set_dnseqen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "desc HSIZE."]
        #[must_use]
        #[inline(always)]
        pub const fn hsize(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0x03;
            val as u8
        }
        #[doc = "desc HSIZE."]
        #[inline(always)]
        pub const fn set_hsize(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 8usize)) | (((val as u32) & 0x03) << 8usize);
        }
        #[doc = "desc LLPEN."]
        #[must_use]
        #[inline(always)]
        pub const fn llpen(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "desc LLPEN."]
        #[inline(always)]
        pub const fn set_llpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[doc = "desc LLPRUN."]
        #[must_use]
        #[inline(always)]
        pub const fn llprun(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "desc LLPRUN."]
        #[inline(always)]
        pub const fn set_llprun(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[doc = "desc IE."]
        #[must_use]
        #[inline(always)]
        pub const fn ie(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "desc IE."]
        #[inline(always)]
        pub const fn set_ie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "desc HPROT."]
        #[must_use]
        #[inline(always)]
        pub const fn hprot(&self) -> u8 {
            let val = (self.0 >> 14usize) & 0x03;
            val as u8
        }
        #[doc = "desc HPROT."]
        #[inline(always)]
        pub const fn set_hprot(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 14usize)) | (((val as u32) & 0x03) << 14usize);
        }
    }
    impl Default for Chctl3 {
        #[inline(always)]
        fn default() -> Chctl3 {
            Chctl3(0)
        }
    }
    impl core::fmt::Debug for Chctl3 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Chctl3")
                .field("sinc", &self.sinc())
                .field("dinc", &self.dinc())
                .field("srpten", &self.srpten())
                .field("drpten", &self.drpten())
                .field("snseqen", &self.snseqen())
                .field("dnseqen", &self.dnseqen())
                .field("hsize", &self.hsize())
                .field("llpen", &self.llpen())
                .field("llprun", &self.llprun())
                .field("ie", &self.ie())
                .field("hprot", &self.hprot())
                .finish()
        }
    }
    #[doc = "desc CHCTL4."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Chctl4(pub u32);
    impl Chctl4 {
        #[doc = "desc SINC."]
        #[must_use]
        #[inline(always)]
        pub const fn sinc(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x03;
            val as u8
        }
        #[doc = "desc SINC."]
        #[inline(always)]
        pub const fn set_sinc(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 0usize)) | (((val as u32) & 0x03) << 0usize);
        }
        #[doc = "desc DINC."]
        #[must_use]
        #[inline(always)]
        pub const fn dinc(&self) -> u8 {
            let val = (self.0 >> 2usize) & 0x03;
            val as u8
        }
        #[doc = "desc DINC."]
        #[inline(always)]
        pub const fn set_dinc(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 2usize)) | (((val as u32) & 0x03) << 2usize);
        }
        #[doc = "desc SRPTEN."]
        #[must_use]
        #[inline(always)]
        pub const fn srpten(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "desc SRPTEN."]
        #[inline(always)]
        pub const fn set_srpten(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "desc DRPTEN."]
        #[must_use]
        #[inline(always)]
        pub const fn drpten(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "desc DRPTEN."]
        #[inline(always)]
        pub const fn set_drpten(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "desc SNSEQEN."]
        #[must_use]
        #[inline(always)]
        pub const fn snseqen(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "desc SNSEQEN."]
        #[inline(always)]
        pub const fn set_snseqen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "desc DNSEQEN."]
        #[must_use]
        #[inline(always)]
        pub const fn dnseqen(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "desc DNSEQEN."]
        #[inline(always)]
        pub const fn set_dnseqen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "desc HSIZE."]
        #[must_use]
        #[inline(always)]
        pub const fn hsize(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0x03;
            val as u8
        }
        #[doc = "desc HSIZE."]
        #[inline(always)]
        pub const fn set_hsize(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 8usize)) | (((val as u32) & 0x03) << 8usize);
        }
        #[doc = "desc LLPEN."]
        #[must_use]
        #[inline(always)]
        pub const fn llpen(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "desc LLPEN."]
        #[inline(always)]
        pub const fn set_llpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[doc = "desc LLPRUN."]
        #[must_use]
        #[inline(always)]
        pub const fn llprun(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "desc LLPRUN."]
        #[inline(always)]
        pub const fn set_llprun(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[doc = "desc IE."]
        #[must_use]
        #[inline(always)]
        pub const fn ie(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "desc IE."]
        #[inline(always)]
        pub const fn set_ie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "desc HPROT."]
        #[must_use]
        #[inline(always)]
        pub const fn hprot(&self) -> u8 {
            let val = (self.0 >> 14usize) & 0x03;
            val as u8
        }
        #[doc = "desc HPROT."]
        #[inline(always)]
        pub const fn set_hprot(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 14usize)) | (((val as u32) & 0x03) << 14usize);
        }
    }
    impl Default for Chctl4 {
        #[inline(always)]
        fn default() -> Chctl4 {
            Chctl4(0)
        }
    }
    impl core::fmt::Debug for Chctl4 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Chctl4")
                .field("sinc", &self.sinc())
                .field("dinc", &self.dinc())
                .field("srpten", &self.srpten())
                .field("drpten", &self.drpten())
                .field("snseqen", &self.snseqen())
                .field("dnseqen", &self.dnseqen())
                .field("hsize", &self.hsize())
                .field("llpen", &self.llpen())
                .field("llprun", &self.llprun())
                .field("ie", &self.ie())
                .field("hprot", &self.hprot())
                .finish()
        }
    }
    #[doc = "desc CHCTL5."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Chctl5(pub u32);
    impl Chctl5 {
        #[doc = "desc SINC."]
        #[must_use]
        #[inline(always)]
        pub const fn sinc(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x03;
            val as u8
        }
        #[doc = "desc SINC."]
        #[inline(always)]
        pub const fn set_sinc(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 0usize)) | (((val as u32) & 0x03) << 0usize);
        }
        #[doc = "desc DINC."]
        #[must_use]
        #[inline(always)]
        pub const fn dinc(&self) -> u8 {
            let val = (self.0 >> 2usize) & 0x03;
            val as u8
        }
        #[doc = "desc DINC."]
        #[inline(always)]
        pub const fn set_dinc(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 2usize)) | (((val as u32) & 0x03) << 2usize);
        }
        #[doc = "desc SRPTEN."]
        #[must_use]
        #[inline(always)]
        pub const fn srpten(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "desc SRPTEN."]
        #[inline(always)]
        pub const fn set_srpten(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "desc DRPTEN."]
        #[must_use]
        #[inline(always)]
        pub const fn drpten(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "desc DRPTEN."]
        #[inline(always)]
        pub const fn set_drpten(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "desc SNSEQEN."]
        #[must_use]
        #[inline(always)]
        pub const fn snseqen(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "desc SNSEQEN."]
        #[inline(always)]
        pub const fn set_snseqen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "desc DNSEQEN."]
        #[must_use]
        #[inline(always)]
        pub const fn dnseqen(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "desc DNSEQEN."]
        #[inline(always)]
        pub const fn set_dnseqen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "desc HSIZE."]
        #[must_use]
        #[inline(always)]
        pub const fn hsize(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0x03;
            val as u8
        }
        #[doc = "desc HSIZE."]
        #[inline(always)]
        pub const fn set_hsize(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 8usize)) | (((val as u32) & 0x03) << 8usize);
        }
        #[doc = "desc LLPEN."]
        #[must_use]
        #[inline(always)]
        pub const fn llpen(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "desc LLPEN."]
        #[inline(always)]
        pub const fn set_llpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[doc = "desc LLPRUN."]
        #[must_use]
        #[inline(always)]
        pub const fn llprun(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "desc LLPRUN."]
        #[inline(always)]
        pub const fn set_llprun(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[doc = "desc IE."]
        #[must_use]
        #[inline(always)]
        pub const fn ie(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "desc IE."]
        #[inline(always)]
        pub const fn set_ie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "desc HPROT."]
        #[must_use]
        #[inline(always)]
        pub const fn hprot(&self) -> u8 {
            let val = (self.0 >> 14usize) & 0x03;
            val as u8
        }
        #[doc = "desc HPROT."]
        #[inline(always)]
        pub const fn set_hprot(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 14usize)) | (((val as u32) & 0x03) << 14usize);
        }
    }
    impl Default for Chctl5 {
        #[inline(always)]
        fn default() -> Chctl5 {
            Chctl5(0)
        }
    }
    impl core::fmt::Debug for Chctl5 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Chctl5")
                .field("sinc", &self.sinc())
                .field("dinc", &self.dinc())
                .field("srpten", &self.srpten())
                .field("drpten", &self.drpten())
                .field("snseqen", &self.snseqen())
                .field("dnseqen", &self.dnseqen())
                .field("hsize", &self.hsize())
                .field("llpen", &self.llpen())
                .field("llprun", &self.llprun())
                .field("ie", &self.ie())
                .field("hprot", &self.hprot())
                .finish()
        }
    }
    #[doc = "desc CHEN."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Chen(pub u32);
    impl Chen {
        #[doc = "desc CHEN."]
        #[must_use]
        #[inline(always)]
        pub const fn chen(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x3f;
            val as u8
        }
        #[doc = "desc CHEN."]
        #[inline(always)]
        pub const fn set_chen(&mut self, val: u8) {
            self.0 = (self.0 & !(0x3f << 0usize)) | (((val as u32) & 0x3f) << 0usize);
        }
    }
    impl Default for Chen {
        #[inline(always)]
        fn default() -> Chen {
            Chen(0)
        }
    }
    impl core::fmt::Debug for Chen {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Chen").field("chen", &self.chen()).finish()
        }
    }
    #[doc = "desc CHENCLR."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Chenclr(pub u32);
    impl Chenclr {
        #[doc = "desc CHENCLR."]
        #[must_use]
        #[inline(always)]
        pub const fn chenclr(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x3f;
            val as u8
        }
        #[doc = "desc CHENCLR."]
        #[inline(always)]
        pub const fn set_chenclr(&mut self, val: u8) {
            self.0 = (self.0 & !(0x3f << 0usize)) | (((val as u32) & 0x3f) << 0usize);
        }
    }
    impl Default for Chenclr {
        #[inline(always)]
        fn default() -> Chenclr {
            Chenclr(0)
        }
    }
    impl core::fmt::Debug for Chenclr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Chenclr")
                .field("chenclr", &self.chenclr())
                .finish()
        }
    }
    #[doc = "desc CHSTAT."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Chstat(pub u32);
    impl Chstat {
        #[doc = "desc DMAACT."]
        #[must_use]
        #[inline(always)]
        pub const fn dmaact(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "desc DMAACT."]
        #[inline(always)]
        pub const fn set_dmaact(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "desc RCFGACT."]
        #[must_use]
        #[inline(always)]
        pub const fn rcfgact(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "desc RCFGACT."]
        #[inline(always)]
        pub const fn set_rcfgact(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "desc CHACT."]
        #[must_use]
        #[inline(always)]
        pub const fn chact(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x3f;
            val as u8
        }
        #[doc = "desc CHACT."]
        #[inline(always)]
        pub const fn set_chact(&mut self, val: u8) {
            self.0 = (self.0 & !(0x3f << 16usize)) | (((val as u32) & 0x3f) << 16usize);
        }
    }
    impl Default for Chstat {
        #[inline(always)]
        fn default() -> Chstat {
            Chstat(0)
        }
    }
    impl core::fmt::Debug for Chstat {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Chstat")
                .field("dmaact", &self.dmaact())
                .field("rcfgact", &self.rcfgact())
                .field("chact", &self.chact())
                .finish()
        }
    }
    #[doc = "desc DNSEQCTL0."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Dnseqctl0(pub u32);
    impl Dnseqctl0 {
        #[doc = "desc DOFFSET."]
        #[must_use]
        #[inline(always)]
        pub const fn doffset(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0x000f_ffff;
            val as u32
        }
        #[doc = "desc DOFFSET."]
        #[inline(always)]
        pub const fn set_doffset(&mut self, val: u32) {
            self.0 = (self.0 & !(0x000f_ffff << 0usize)) | (((val as u32) & 0x000f_ffff) << 0usize);
        }
        #[doc = "desc DNSCNT."]
        #[must_use]
        #[inline(always)]
        pub const fn dnscnt(&self) -> u16 {
            let val = (self.0 >> 20usize) & 0x0fff;
            val as u16
        }
        #[doc = "desc DNSCNT."]
        #[inline(always)]
        pub const fn set_dnscnt(&mut self, val: u16) {
            self.0 = (self.0 & !(0x0fff << 20usize)) | (((val as u32) & 0x0fff) << 20usize);
        }
    }
    impl Default for Dnseqctl0 {
        #[inline(always)]
        fn default() -> Dnseqctl0 {
            Dnseqctl0(0)
        }
    }
    impl core::fmt::Debug for Dnseqctl0 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Dnseqctl0")
                .field("doffset", &self.doffset())
                .field("dnscnt", &self.dnscnt())
                .finish()
        }
    }
    #[doc = "desc DNSEQCTL1."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Dnseqctl1(pub u32);
    impl Dnseqctl1 {
        #[doc = "desc DOFFSET."]
        #[must_use]
        #[inline(always)]
        pub const fn doffset(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0x000f_ffff;
            val as u32
        }
        #[doc = "desc DOFFSET."]
        #[inline(always)]
        pub const fn set_doffset(&mut self, val: u32) {
            self.0 = (self.0 & !(0x000f_ffff << 0usize)) | (((val as u32) & 0x000f_ffff) << 0usize);
        }
        #[doc = "desc DNSCNT."]
        #[must_use]
        #[inline(always)]
        pub const fn dnscnt(&self) -> u16 {
            let val = (self.0 >> 20usize) & 0x0fff;
            val as u16
        }
        #[doc = "desc DNSCNT."]
        #[inline(always)]
        pub const fn set_dnscnt(&mut self, val: u16) {
            self.0 = (self.0 & !(0x0fff << 20usize)) | (((val as u32) & 0x0fff) << 20usize);
        }
    }
    impl Default for Dnseqctl1 {
        #[inline(always)]
        fn default() -> Dnseqctl1 {
            Dnseqctl1(0)
        }
    }
    impl core::fmt::Debug for Dnseqctl1 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Dnseqctl1")
                .field("doffset", &self.doffset())
                .field("dnscnt", &self.dnscnt())
                .finish()
        }
    }
    #[doc = "desc DNSEQCTL2."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Dnseqctl2(pub u32);
    impl Dnseqctl2 {
        #[doc = "desc DOFFSET."]
        #[must_use]
        #[inline(always)]
        pub const fn doffset(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0x000f_ffff;
            val as u32
        }
        #[doc = "desc DOFFSET."]
        #[inline(always)]
        pub const fn set_doffset(&mut self, val: u32) {
            self.0 = (self.0 & !(0x000f_ffff << 0usize)) | (((val as u32) & 0x000f_ffff) << 0usize);
        }
        #[doc = "desc DNSCNT."]
        #[must_use]
        #[inline(always)]
        pub const fn dnscnt(&self) -> u16 {
            let val = (self.0 >> 20usize) & 0x0fff;
            val as u16
        }
        #[doc = "desc DNSCNT."]
        #[inline(always)]
        pub const fn set_dnscnt(&mut self, val: u16) {
            self.0 = (self.0 & !(0x0fff << 20usize)) | (((val as u32) & 0x0fff) << 20usize);
        }
    }
    impl Default for Dnseqctl2 {
        #[inline(always)]
        fn default() -> Dnseqctl2 {
            Dnseqctl2(0)
        }
    }
    impl core::fmt::Debug for Dnseqctl2 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Dnseqctl2")
                .field("doffset", &self.doffset())
                .field("dnscnt", &self.dnscnt())
                .finish()
        }
    }
    #[doc = "desc DNSEQCTL3."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Dnseqctl3(pub u32);
    impl Dnseqctl3 {
        #[doc = "desc DOFFSET."]
        #[must_use]
        #[inline(always)]
        pub const fn doffset(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0x000f_ffff;
            val as u32
        }
        #[doc = "desc DOFFSET."]
        #[inline(always)]
        pub const fn set_doffset(&mut self, val: u32) {
            self.0 = (self.0 & !(0x000f_ffff << 0usize)) | (((val as u32) & 0x000f_ffff) << 0usize);
        }
        #[doc = "desc DNSCNT."]
        #[must_use]
        #[inline(always)]
        pub const fn dnscnt(&self) -> u16 {
            let val = (self.0 >> 20usize) & 0x0fff;
            val as u16
        }
        #[doc = "desc DNSCNT."]
        #[inline(always)]
        pub const fn set_dnscnt(&mut self, val: u16) {
            self.0 = (self.0 & !(0x0fff << 20usize)) | (((val as u32) & 0x0fff) << 20usize);
        }
    }
    impl Default for Dnseqctl3 {
        #[inline(always)]
        fn default() -> Dnseqctl3 {
            Dnseqctl3(0)
        }
    }
    impl core::fmt::Debug for Dnseqctl3 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Dnseqctl3")
                .field("doffset", &self.doffset())
                .field("dnscnt", &self.dnscnt())
                .finish()
        }
    }
    #[doc = "desc DNSEQCTL4."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Dnseqctl4(pub u32);
    impl Dnseqctl4 {
        #[doc = "desc DOFFSET."]
        #[must_use]
        #[inline(always)]
        pub const fn doffset(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0x000f_ffff;
            val as u32
        }
        #[doc = "desc DOFFSET."]
        #[inline(always)]
        pub const fn set_doffset(&mut self, val: u32) {
            self.0 = (self.0 & !(0x000f_ffff << 0usize)) | (((val as u32) & 0x000f_ffff) << 0usize);
        }
        #[doc = "desc DNSCNT."]
        #[must_use]
        #[inline(always)]
        pub const fn dnscnt(&self) -> u16 {
            let val = (self.0 >> 20usize) & 0x0fff;
            val as u16
        }
        #[doc = "desc DNSCNT."]
        #[inline(always)]
        pub const fn set_dnscnt(&mut self, val: u16) {
            self.0 = (self.0 & !(0x0fff << 20usize)) | (((val as u32) & 0x0fff) << 20usize);
        }
    }
    impl Default for Dnseqctl4 {
        #[inline(always)]
        fn default() -> Dnseqctl4 {
            Dnseqctl4(0)
        }
    }
    impl core::fmt::Debug for Dnseqctl4 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Dnseqctl4")
                .field("doffset", &self.doffset())
                .field("dnscnt", &self.dnscnt())
                .finish()
        }
    }
    #[doc = "desc DNSEQCTL5."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Dnseqctl5(pub u32);
    impl Dnseqctl5 {
        #[doc = "desc DOFFSET."]
        #[must_use]
        #[inline(always)]
        pub const fn doffset(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0x000f_ffff;
            val as u32
        }
        #[doc = "desc DOFFSET."]
        #[inline(always)]
        pub const fn set_doffset(&mut self, val: u32) {
            self.0 = (self.0 & !(0x000f_ffff << 0usize)) | (((val as u32) & 0x000f_ffff) << 0usize);
        }
        #[doc = "desc DNSCNT."]
        #[must_use]
        #[inline(always)]
        pub const fn dnscnt(&self) -> u16 {
            let val = (self.0 >> 20usize) & 0x0fff;
            val as u16
        }
        #[doc = "desc DNSCNT."]
        #[inline(always)]
        pub const fn set_dnscnt(&mut self, val: u16) {
            self.0 = (self.0 & !(0x0fff << 20usize)) | (((val as u32) & 0x0fff) << 20usize);
        }
    }
    impl Default for Dnseqctl5 {
        #[inline(always)]
        fn default() -> Dnseqctl5 {
            Dnseqctl5(0)
        }
    }
    impl core::fmt::Debug for Dnseqctl5 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Dnseqctl5")
                .field("doffset", &self.doffset())
                .field("dnscnt", &self.dnscnt())
                .finish()
        }
    }
    #[doc = "desc DNSEQCTLB0."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Dnseqctlb0(pub u32);
    impl Dnseqctlb0 {
        #[doc = "desc DNSDIST."]
        #[must_use]
        #[inline(always)]
        pub const fn dnsdist(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0x000f_ffff;
            val as u32
        }
        #[doc = "desc DNSDIST."]
        #[inline(always)]
        pub const fn set_dnsdist(&mut self, val: u32) {
            self.0 = (self.0 & !(0x000f_ffff << 0usize)) | (((val as u32) & 0x000f_ffff) << 0usize);
        }
        #[doc = "desc DNSCNTB."]
        #[must_use]
        #[inline(always)]
        pub const fn dnscntb(&self) -> u16 {
            let val = (self.0 >> 20usize) & 0x0fff;
            val as u16
        }
        #[doc = "desc DNSCNTB."]
        #[inline(always)]
        pub const fn set_dnscntb(&mut self, val: u16) {
            self.0 = (self.0 & !(0x0fff << 20usize)) | (((val as u32) & 0x0fff) << 20usize);
        }
    }
    impl Default for Dnseqctlb0 {
        #[inline(always)]
        fn default() -> Dnseqctlb0 {
            Dnseqctlb0(0)
        }
    }
    impl core::fmt::Debug for Dnseqctlb0 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Dnseqctlb0")
                .field("dnsdist", &self.dnsdist())
                .field("dnscntb", &self.dnscntb())
                .finish()
        }
    }
    #[doc = "desc DNSEQCTLB1."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Dnseqctlb1(pub u32);
    impl Dnseqctlb1 {
        #[doc = "desc DNSDIST."]
        #[must_use]
        #[inline(always)]
        pub const fn dnsdist(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0x000f_ffff;
            val as u32
        }
        #[doc = "desc DNSDIST."]
        #[inline(always)]
        pub const fn set_dnsdist(&mut self, val: u32) {
            self.0 = (self.0 & !(0x000f_ffff << 0usize)) | (((val as u32) & 0x000f_ffff) << 0usize);
        }
        #[doc = "desc DNSCNTB."]
        #[must_use]
        #[inline(always)]
        pub const fn dnscntb(&self) -> u16 {
            let val = (self.0 >> 20usize) & 0x0fff;
            val as u16
        }
        #[doc = "desc DNSCNTB."]
        #[inline(always)]
        pub const fn set_dnscntb(&mut self, val: u16) {
            self.0 = (self.0 & !(0x0fff << 20usize)) | (((val as u32) & 0x0fff) << 20usize);
        }
    }
    impl Default for Dnseqctlb1 {
        #[inline(always)]
        fn default() -> Dnseqctlb1 {
            Dnseqctlb1(0)
        }
    }
    impl core::fmt::Debug for Dnseqctlb1 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Dnseqctlb1")
                .field("dnsdist", &self.dnsdist())
                .field("dnscntb", &self.dnscntb())
                .finish()
        }
    }
    #[doc = "desc DNSEQCTLB2."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Dnseqctlb2(pub u32);
    impl Dnseqctlb2 {
        #[doc = "desc DNSDIST."]
        #[must_use]
        #[inline(always)]
        pub const fn dnsdist(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0x000f_ffff;
            val as u32
        }
        #[doc = "desc DNSDIST."]
        #[inline(always)]
        pub const fn set_dnsdist(&mut self, val: u32) {
            self.0 = (self.0 & !(0x000f_ffff << 0usize)) | (((val as u32) & 0x000f_ffff) << 0usize);
        }
        #[doc = "desc DNSCNTB."]
        #[must_use]
        #[inline(always)]
        pub const fn dnscntb(&self) -> u16 {
            let val = (self.0 >> 20usize) & 0x0fff;
            val as u16
        }
        #[doc = "desc DNSCNTB."]
        #[inline(always)]
        pub const fn set_dnscntb(&mut self, val: u16) {
            self.0 = (self.0 & !(0x0fff << 20usize)) | (((val as u32) & 0x0fff) << 20usize);
        }
    }
    impl Default for Dnseqctlb2 {
        #[inline(always)]
        fn default() -> Dnseqctlb2 {
            Dnseqctlb2(0)
        }
    }
    impl core::fmt::Debug for Dnseqctlb2 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Dnseqctlb2")
                .field("dnsdist", &self.dnsdist())
                .field("dnscntb", &self.dnscntb())
                .finish()
        }
    }
    #[doc = "desc DNSEQCTLB3."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Dnseqctlb3(pub u32);
    impl Dnseqctlb3 {
        #[doc = "desc DNSDIST."]
        #[must_use]
        #[inline(always)]
        pub const fn dnsdist(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0x000f_ffff;
            val as u32
        }
        #[doc = "desc DNSDIST."]
        #[inline(always)]
        pub const fn set_dnsdist(&mut self, val: u32) {
            self.0 = (self.0 & !(0x000f_ffff << 0usize)) | (((val as u32) & 0x000f_ffff) << 0usize);
        }
        #[doc = "desc DNSCNTB."]
        #[must_use]
        #[inline(always)]
        pub const fn dnscntb(&self) -> u16 {
            let val = (self.0 >> 20usize) & 0x0fff;
            val as u16
        }
        #[doc = "desc DNSCNTB."]
        #[inline(always)]
        pub const fn set_dnscntb(&mut self, val: u16) {
            self.0 = (self.0 & !(0x0fff << 20usize)) | (((val as u32) & 0x0fff) << 20usize);
        }
    }
    impl Default for Dnseqctlb3 {
        #[inline(always)]
        fn default() -> Dnseqctlb3 {
            Dnseqctlb3(0)
        }
    }
    impl core::fmt::Debug for Dnseqctlb3 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Dnseqctlb3")
                .field("dnsdist", &self.dnsdist())
                .field("dnscntb", &self.dnscntb())
                .finish()
        }
    }
    #[doc = "desc DNSEQCTLB4."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Dnseqctlb4(pub u32);
    impl Dnseqctlb4 {
        #[doc = "desc DNSDIST."]
        #[must_use]
        #[inline(always)]
        pub const fn dnsdist(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0x000f_ffff;
            val as u32
        }
        #[doc = "desc DNSDIST."]
        #[inline(always)]
        pub const fn set_dnsdist(&mut self, val: u32) {
            self.0 = (self.0 & !(0x000f_ffff << 0usize)) | (((val as u32) & 0x000f_ffff) << 0usize);
        }
        #[doc = "desc DNSCNTB."]
        #[must_use]
        #[inline(always)]
        pub const fn dnscntb(&self) -> u16 {
            let val = (self.0 >> 20usize) & 0x0fff;
            val as u16
        }
        #[doc = "desc DNSCNTB."]
        #[inline(always)]
        pub const fn set_dnscntb(&mut self, val: u16) {
            self.0 = (self.0 & !(0x0fff << 20usize)) | (((val as u32) & 0x0fff) << 20usize);
        }
    }
    impl Default for Dnseqctlb4 {
        #[inline(always)]
        fn default() -> Dnseqctlb4 {
            Dnseqctlb4(0)
        }
    }
    impl core::fmt::Debug for Dnseqctlb4 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Dnseqctlb4")
                .field("dnsdist", &self.dnsdist())
                .field("dnscntb", &self.dnscntb())
                .finish()
        }
    }
    #[doc = "desc DNSEQCTLB5."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Dnseqctlb5(pub u32);
    impl Dnseqctlb5 {
        #[doc = "desc DNSDIST."]
        #[must_use]
        #[inline(always)]
        pub const fn dnsdist(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0x000f_ffff;
            val as u32
        }
        #[doc = "desc DNSDIST."]
        #[inline(always)]
        pub const fn set_dnsdist(&mut self, val: u32) {
            self.0 = (self.0 & !(0x000f_ffff << 0usize)) | (((val as u32) & 0x000f_ffff) << 0usize);
        }
        #[doc = "desc DNSCNTB."]
        #[must_use]
        #[inline(always)]
        pub const fn dnscntb(&self) -> u16 {
            let val = (self.0 >> 20usize) & 0x0fff;
            val as u16
        }
        #[doc = "desc DNSCNTB."]
        #[inline(always)]
        pub const fn set_dnscntb(&mut self, val: u16) {
            self.0 = (self.0 & !(0x0fff << 20usize)) | (((val as u32) & 0x0fff) << 20usize);
        }
    }
    impl Default for Dnseqctlb5 {
        #[inline(always)]
        fn default() -> Dnseqctlb5 {
            Dnseqctlb5(0)
        }
    }
    impl core::fmt::Debug for Dnseqctlb5 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Dnseqctlb5")
                .field("dnsdist", &self.dnsdist())
                .field("dnscntb", &self.dnscntb())
                .finish()
        }
    }
    #[doc = "desc DTCTL0."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Dtctl0(pub u32);
    impl Dtctl0 {
        #[doc = "desc BLKSIZE."]
        #[must_use]
        #[inline(always)]
        pub const fn blksize(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x03ff;
            val as u16
        }
        #[doc = "desc BLKSIZE."]
        #[inline(always)]
        pub const fn set_blksize(&mut self, val: u16) {
            self.0 = (self.0 & !(0x03ff << 0usize)) | (((val as u32) & 0x03ff) << 0usize);
        }
        #[doc = "desc CNT."]
        #[must_use]
        #[inline(always)]
        pub const fn cnt(&self) -> u16 {
            let val = (self.0 >> 16usize) & 0xffff;
            val as u16
        }
        #[doc = "desc CNT."]
        #[inline(always)]
        pub const fn set_cnt(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 16usize)) | (((val as u32) & 0xffff) << 16usize);
        }
    }
    impl Default for Dtctl0 {
        #[inline(always)]
        fn default() -> Dtctl0 {
            Dtctl0(0)
        }
    }
    impl core::fmt::Debug for Dtctl0 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Dtctl0")
                .field("blksize", &self.blksize())
                .field("cnt", &self.cnt())
                .finish()
        }
    }
    #[doc = "desc DTCTL1."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Dtctl1(pub u32);
    impl Dtctl1 {
        #[doc = "desc BLKSIZE."]
        #[must_use]
        #[inline(always)]
        pub const fn blksize(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x03ff;
            val as u16
        }
        #[doc = "desc BLKSIZE."]
        #[inline(always)]
        pub const fn set_blksize(&mut self, val: u16) {
            self.0 = (self.0 & !(0x03ff << 0usize)) | (((val as u32) & 0x03ff) << 0usize);
        }
        #[doc = "desc CNT."]
        #[must_use]
        #[inline(always)]
        pub const fn cnt(&self) -> u16 {
            let val = (self.0 >> 16usize) & 0xffff;
            val as u16
        }
        #[doc = "desc CNT."]
        #[inline(always)]
        pub const fn set_cnt(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 16usize)) | (((val as u32) & 0xffff) << 16usize);
        }
    }
    impl Default for Dtctl1 {
        #[inline(always)]
        fn default() -> Dtctl1 {
            Dtctl1(0)
        }
    }
    impl core::fmt::Debug for Dtctl1 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Dtctl1")
                .field("blksize", &self.blksize())
                .field("cnt", &self.cnt())
                .finish()
        }
    }
    #[doc = "desc DTCTL2."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Dtctl2(pub u32);
    impl Dtctl2 {
        #[doc = "desc BLKSIZE."]
        #[must_use]
        #[inline(always)]
        pub const fn blksize(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x03ff;
            val as u16
        }
        #[doc = "desc BLKSIZE."]
        #[inline(always)]
        pub const fn set_blksize(&mut self, val: u16) {
            self.0 = (self.0 & !(0x03ff << 0usize)) | (((val as u32) & 0x03ff) << 0usize);
        }
        #[doc = "desc CNT."]
        #[must_use]
        #[inline(always)]
        pub const fn cnt(&self) -> u16 {
            let val = (self.0 >> 16usize) & 0xffff;
            val as u16
        }
        #[doc = "desc CNT."]
        #[inline(always)]
        pub const fn set_cnt(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 16usize)) | (((val as u32) & 0xffff) << 16usize);
        }
    }
    impl Default for Dtctl2 {
        #[inline(always)]
        fn default() -> Dtctl2 {
            Dtctl2(0)
        }
    }
    impl core::fmt::Debug for Dtctl2 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Dtctl2")
                .field("blksize", &self.blksize())
                .field("cnt", &self.cnt())
                .finish()
        }
    }
    #[doc = "desc DTCTL3."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Dtctl3(pub u32);
    impl Dtctl3 {
        #[doc = "desc BLKSIZE."]
        #[must_use]
        #[inline(always)]
        pub const fn blksize(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x03ff;
            val as u16
        }
        #[doc = "desc BLKSIZE."]
        #[inline(always)]
        pub const fn set_blksize(&mut self, val: u16) {
            self.0 = (self.0 & !(0x03ff << 0usize)) | (((val as u32) & 0x03ff) << 0usize);
        }
        #[doc = "desc CNT."]
        #[must_use]
        #[inline(always)]
        pub const fn cnt(&self) -> u16 {
            let val = (self.0 >> 16usize) & 0xffff;
            val as u16
        }
        #[doc = "desc CNT."]
        #[inline(always)]
        pub const fn set_cnt(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 16usize)) | (((val as u32) & 0xffff) << 16usize);
        }
    }
    impl Default for Dtctl3 {
        #[inline(always)]
        fn default() -> Dtctl3 {
            Dtctl3(0)
        }
    }
    impl core::fmt::Debug for Dtctl3 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Dtctl3")
                .field("blksize", &self.blksize())
                .field("cnt", &self.cnt())
                .finish()
        }
    }
    #[doc = "desc DTCTL4."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Dtctl4(pub u32);
    impl Dtctl4 {
        #[doc = "desc BLKSIZE."]
        #[must_use]
        #[inline(always)]
        pub const fn blksize(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x03ff;
            val as u16
        }
        #[doc = "desc BLKSIZE."]
        #[inline(always)]
        pub const fn set_blksize(&mut self, val: u16) {
            self.0 = (self.0 & !(0x03ff << 0usize)) | (((val as u32) & 0x03ff) << 0usize);
        }
        #[doc = "desc CNT."]
        #[must_use]
        #[inline(always)]
        pub const fn cnt(&self) -> u16 {
            let val = (self.0 >> 16usize) & 0xffff;
            val as u16
        }
        #[doc = "desc CNT."]
        #[inline(always)]
        pub const fn set_cnt(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 16usize)) | (((val as u32) & 0xffff) << 16usize);
        }
    }
    impl Default for Dtctl4 {
        #[inline(always)]
        fn default() -> Dtctl4 {
            Dtctl4(0)
        }
    }
    impl core::fmt::Debug for Dtctl4 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Dtctl4")
                .field("blksize", &self.blksize())
                .field("cnt", &self.cnt())
                .finish()
        }
    }
    #[doc = "desc DTCTL5."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Dtctl5(pub u32);
    impl Dtctl5 {
        #[doc = "desc BLKSIZE."]
        #[must_use]
        #[inline(always)]
        pub const fn blksize(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x03ff;
            val as u16
        }
        #[doc = "desc BLKSIZE."]
        #[inline(always)]
        pub const fn set_blksize(&mut self, val: u16) {
            self.0 = (self.0 & !(0x03ff << 0usize)) | (((val as u32) & 0x03ff) << 0usize);
        }
        #[doc = "desc CNT."]
        #[must_use]
        #[inline(always)]
        pub const fn cnt(&self) -> u16 {
            let val = (self.0 >> 16usize) & 0xffff;
            val as u16
        }
        #[doc = "desc CNT."]
        #[inline(always)]
        pub const fn set_cnt(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 16usize)) | (((val as u32) & 0xffff) << 16usize);
        }
    }
    impl Default for Dtctl5 {
        #[inline(always)]
        fn default() -> Dtctl5 {
            Dtctl5(0)
        }
    }
    impl core::fmt::Debug for Dtctl5 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Dtctl5")
                .field("blksize", &self.blksize())
                .field("cnt", &self.cnt())
                .finish()
        }
    }
    #[doc = "desc EN."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct En(pub u32);
    impl En {
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
    }
    impl Default for En {
        #[inline(always)]
        fn default() -> En {
            En(0)
        }
    }
    impl core::fmt::Debug for En {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("En").field("en", &self.en()).finish()
        }
    }
    #[doc = "desc INTCLR0."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Intclr0(pub u32);
    impl Intclr0 {
        #[doc = "desc CLRTRNERR."]
        #[must_use]
        #[inline(always)]
        pub const fn clrtrnerr(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x3f;
            val as u8
        }
        #[doc = "desc CLRTRNERR."]
        #[inline(always)]
        pub const fn set_clrtrnerr(&mut self, val: u8) {
            self.0 = (self.0 & !(0x3f << 0usize)) | (((val as u32) & 0x3f) << 0usize);
        }
        #[doc = "desc CLRREQERR."]
        #[must_use]
        #[inline(always)]
        pub const fn clrreqerr(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x3f;
            val as u8
        }
        #[doc = "desc CLRREQERR."]
        #[inline(always)]
        pub const fn set_clrreqerr(&mut self, val: u8) {
            self.0 = (self.0 & !(0x3f << 16usize)) | (((val as u32) & 0x3f) << 16usize);
        }
    }
    impl Default for Intclr0 {
        #[inline(always)]
        fn default() -> Intclr0 {
            Intclr0(0)
        }
    }
    impl core::fmt::Debug for Intclr0 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Intclr0")
                .field("clrtrnerr", &self.clrtrnerr())
                .field("clrreqerr", &self.clrreqerr())
                .finish()
        }
    }
    #[doc = "desc INTCLR1."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Intclr1(pub u32);
    impl Intclr1 {
        #[doc = "desc CLRTC."]
        #[must_use]
        #[inline(always)]
        pub const fn clrtc(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x3f;
            val as u8
        }
        #[doc = "desc CLRTC."]
        #[inline(always)]
        pub const fn set_clrtc(&mut self, val: u8) {
            self.0 = (self.0 & !(0x3f << 0usize)) | (((val as u32) & 0x3f) << 0usize);
        }
        #[doc = "desc CLRBTC."]
        #[must_use]
        #[inline(always)]
        pub const fn clrbtc(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x3f;
            val as u8
        }
        #[doc = "desc CLRBTC."]
        #[inline(always)]
        pub const fn set_clrbtc(&mut self, val: u8) {
            self.0 = (self.0 & !(0x3f << 16usize)) | (((val as u32) & 0x3f) << 16usize);
        }
    }
    impl Default for Intclr1 {
        #[inline(always)]
        fn default() -> Intclr1 {
            Intclr1(0)
        }
    }
    impl core::fmt::Debug for Intclr1 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Intclr1")
                .field("clrtc", &self.clrtc())
                .field("clrbtc", &self.clrbtc())
                .finish()
        }
    }
    #[doc = "desc INTMASK0."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Intmask0(pub u32);
    impl Intmask0 {
        #[doc = "desc MSKTRNERR."]
        #[must_use]
        #[inline(always)]
        pub const fn msktrnerr(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x3f;
            val as u8
        }
        #[doc = "desc MSKTRNERR."]
        #[inline(always)]
        pub const fn set_msktrnerr(&mut self, val: u8) {
            self.0 = (self.0 & !(0x3f << 0usize)) | (((val as u32) & 0x3f) << 0usize);
        }
        #[doc = "desc MSKREQERR."]
        #[must_use]
        #[inline(always)]
        pub const fn mskreqerr(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x3f;
            val as u8
        }
        #[doc = "desc MSKREQERR."]
        #[inline(always)]
        pub const fn set_mskreqerr(&mut self, val: u8) {
            self.0 = (self.0 & !(0x3f << 16usize)) | (((val as u32) & 0x3f) << 16usize);
        }
    }
    impl Default for Intmask0 {
        #[inline(always)]
        fn default() -> Intmask0 {
            Intmask0(0)
        }
    }
    impl core::fmt::Debug for Intmask0 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Intmask0")
                .field("msktrnerr", &self.msktrnerr())
                .field("mskreqerr", &self.mskreqerr())
                .finish()
        }
    }
    #[doc = "desc INTMASK1."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Intmask1(pub u32);
    impl Intmask1 {
        #[doc = "desc MSKTC."]
        #[must_use]
        #[inline(always)]
        pub const fn msktc(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x3f;
            val as u8
        }
        #[doc = "desc MSKTC."]
        #[inline(always)]
        pub const fn set_msktc(&mut self, val: u8) {
            self.0 = (self.0 & !(0x3f << 0usize)) | (((val as u32) & 0x3f) << 0usize);
        }
        #[doc = "desc MSKBTC."]
        #[must_use]
        #[inline(always)]
        pub const fn mskbtc(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x3f;
            val as u8
        }
        #[doc = "desc MSKBTC."]
        #[inline(always)]
        pub const fn set_mskbtc(&mut self, val: u8) {
            self.0 = (self.0 & !(0x3f << 16usize)) | (((val as u32) & 0x3f) << 16usize);
        }
    }
    impl Default for Intmask1 {
        #[inline(always)]
        fn default() -> Intmask1 {
            Intmask1(0)
        }
    }
    impl core::fmt::Debug for Intmask1 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Intmask1")
                .field("msktc", &self.msktc())
                .field("mskbtc", &self.mskbtc())
                .finish()
        }
    }
    #[doc = "desc INTSTAT0."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Intstat0(pub u32);
    impl Intstat0 {
        #[doc = "desc TRNERR."]
        #[must_use]
        #[inline(always)]
        pub const fn trnerr(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x3f;
            val as u8
        }
        #[doc = "desc TRNERR."]
        #[inline(always)]
        pub const fn set_trnerr(&mut self, val: u8) {
            self.0 = (self.0 & !(0x3f << 0usize)) | (((val as u32) & 0x3f) << 0usize);
        }
        #[doc = "desc REQERR."]
        #[must_use]
        #[inline(always)]
        pub const fn reqerr(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x3f;
            val as u8
        }
        #[doc = "desc REQERR."]
        #[inline(always)]
        pub const fn set_reqerr(&mut self, val: u8) {
            self.0 = (self.0 & !(0x3f << 16usize)) | (((val as u32) & 0x3f) << 16usize);
        }
    }
    impl Default for Intstat0 {
        #[inline(always)]
        fn default() -> Intstat0 {
            Intstat0(0)
        }
    }
    impl core::fmt::Debug for Intstat0 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Intstat0")
                .field("trnerr", &self.trnerr())
                .field("reqerr", &self.reqerr())
                .finish()
        }
    }
    #[doc = "desc INTSTAT1."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Intstat1(pub u32);
    impl Intstat1 {
        #[doc = "desc TC."]
        #[must_use]
        #[inline(always)]
        pub const fn tc(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x3f;
            val as u8
        }
        #[doc = "desc TC."]
        #[inline(always)]
        pub const fn set_tc(&mut self, val: u8) {
            self.0 = (self.0 & !(0x3f << 0usize)) | (((val as u32) & 0x3f) << 0usize);
        }
        #[doc = "desc BTC."]
        #[must_use]
        #[inline(always)]
        pub const fn btc(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x3f;
            val as u8
        }
        #[doc = "desc BTC."]
        #[inline(always)]
        pub const fn set_btc(&mut self, val: u8) {
            self.0 = (self.0 & !(0x3f << 16usize)) | (((val as u32) & 0x3f) << 16usize);
        }
    }
    impl Default for Intstat1 {
        #[inline(always)]
        fn default() -> Intstat1 {
            Intstat1(0)
        }
    }
    impl core::fmt::Debug for Intstat1 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Intstat1")
                .field("tc", &self.tc())
                .field("btc", &self.btc())
                .finish()
        }
    }
    #[doc = "desc LLP0."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Llp0(pub u32);
    impl Llp0 {
        #[doc = "desc LLP."]
        #[must_use]
        #[inline(always)]
        pub const fn llp(&self) -> u32 {
            let val = (self.0 >> 2usize) & 0x3fff_ffff;
            val as u32
        }
        #[doc = "desc LLP."]
        #[inline(always)]
        pub const fn set_llp(&mut self, val: u32) {
            self.0 = (self.0 & !(0x3fff_ffff << 2usize)) | (((val as u32) & 0x3fff_ffff) << 2usize);
        }
    }
    impl Default for Llp0 {
        #[inline(always)]
        fn default() -> Llp0 {
            Llp0(0)
        }
    }
    impl core::fmt::Debug for Llp0 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Llp0").field("llp", &self.llp()).finish()
        }
    }
    #[doc = "desc LLP1."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Llp1(pub u32);
    impl Llp1 {
        #[doc = "desc LLP."]
        #[must_use]
        #[inline(always)]
        pub const fn llp(&self) -> u32 {
            let val = (self.0 >> 2usize) & 0x3fff_ffff;
            val as u32
        }
        #[doc = "desc LLP."]
        #[inline(always)]
        pub const fn set_llp(&mut self, val: u32) {
            self.0 = (self.0 & !(0x3fff_ffff << 2usize)) | (((val as u32) & 0x3fff_ffff) << 2usize);
        }
    }
    impl Default for Llp1 {
        #[inline(always)]
        fn default() -> Llp1 {
            Llp1(0)
        }
    }
    impl core::fmt::Debug for Llp1 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Llp1").field("llp", &self.llp()).finish()
        }
    }
    #[doc = "desc LLP2."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Llp2(pub u32);
    impl Llp2 {
        #[doc = "desc LLP."]
        #[must_use]
        #[inline(always)]
        pub const fn llp(&self) -> u32 {
            let val = (self.0 >> 2usize) & 0x3fff_ffff;
            val as u32
        }
        #[doc = "desc LLP."]
        #[inline(always)]
        pub const fn set_llp(&mut self, val: u32) {
            self.0 = (self.0 & !(0x3fff_ffff << 2usize)) | (((val as u32) & 0x3fff_ffff) << 2usize);
        }
    }
    impl Default for Llp2 {
        #[inline(always)]
        fn default() -> Llp2 {
            Llp2(0)
        }
    }
    impl core::fmt::Debug for Llp2 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Llp2").field("llp", &self.llp()).finish()
        }
    }
    #[doc = "desc LLP3."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Llp3(pub u32);
    impl Llp3 {
        #[doc = "desc LLP."]
        #[must_use]
        #[inline(always)]
        pub const fn llp(&self) -> u32 {
            let val = (self.0 >> 2usize) & 0x3fff_ffff;
            val as u32
        }
        #[doc = "desc LLP."]
        #[inline(always)]
        pub const fn set_llp(&mut self, val: u32) {
            self.0 = (self.0 & !(0x3fff_ffff << 2usize)) | (((val as u32) & 0x3fff_ffff) << 2usize);
        }
    }
    impl Default for Llp3 {
        #[inline(always)]
        fn default() -> Llp3 {
            Llp3(0)
        }
    }
    impl core::fmt::Debug for Llp3 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Llp3").field("llp", &self.llp()).finish()
        }
    }
    #[doc = "desc LLP4."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Llp4(pub u32);
    impl Llp4 {
        #[doc = "desc LLP."]
        #[must_use]
        #[inline(always)]
        pub const fn llp(&self) -> u32 {
            let val = (self.0 >> 2usize) & 0x3fff_ffff;
            val as u32
        }
        #[doc = "desc LLP."]
        #[inline(always)]
        pub const fn set_llp(&mut self, val: u32) {
            self.0 = (self.0 & !(0x3fff_ffff << 2usize)) | (((val as u32) & 0x3fff_ffff) << 2usize);
        }
    }
    impl Default for Llp4 {
        #[inline(always)]
        fn default() -> Llp4 {
            Llp4(0)
        }
    }
    impl core::fmt::Debug for Llp4 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Llp4").field("llp", &self.llp()).finish()
        }
    }
    #[doc = "desc LLP5."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Llp5(pub u32);
    impl Llp5 {
        #[doc = "desc LLP."]
        #[must_use]
        #[inline(always)]
        pub const fn llp(&self) -> u32 {
            let val = (self.0 >> 2usize) & 0x3fff_ffff;
            val as u32
        }
        #[doc = "desc LLP."]
        #[inline(always)]
        pub const fn set_llp(&mut self, val: u32) {
            self.0 = (self.0 & !(0x3fff_ffff << 2usize)) | (((val as u32) & 0x3fff_ffff) << 2usize);
        }
    }
    impl Default for Llp5 {
        #[inline(always)]
        fn default() -> Llp5 {
            Llp5(0)
        }
    }
    impl core::fmt::Debug for Llp5 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Llp5").field("llp", &self.llp()).finish()
        }
    }
    #[doc = "desc MONDNSEQCTL0."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Mondnseqctl0(pub u32);
    impl Mondnseqctl0 {
        #[doc = "desc DOFFSET."]
        #[must_use]
        #[inline(always)]
        pub const fn doffset(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0x000f_ffff;
            val as u32
        }
        #[doc = "desc DOFFSET."]
        #[inline(always)]
        pub const fn set_doffset(&mut self, val: u32) {
            self.0 = (self.0 & !(0x000f_ffff << 0usize)) | (((val as u32) & 0x000f_ffff) << 0usize);
        }
        #[doc = "desc DNSCNT."]
        #[must_use]
        #[inline(always)]
        pub const fn dnscnt(&self) -> u16 {
            let val = (self.0 >> 20usize) & 0x0fff;
            val as u16
        }
        #[doc = "desc DNSCNT."]
        #[inline(always)]
        pub const fn set_dnscnt(&mut self, val: u16) {
            self.0 = (self.0 & !(0x0fff << 20usize)) | (((val as u32) & 0x0fff) << 20usize);
        }
    }
    impl Default for Mondnseqctl0 {
        #[inline(always)]
        fn default() -> Mondnseqctl0 {
            Mondnseqctl0(0)
        }
    }
    impl core::fmt::Debug for Mondnseqctl0 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Mondnseqctl0")
                .field("doffset", &self.doffset())
                .field("dnscnt", &self.dnscnt())
                .finish()
        }
    }
    #[doc = "desc MONDNSEQCTL1."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Mondnseqctl1(pub u32);
    impl Mondnseqctl1 {
        #[doc = "desc DOFFSET."]
        #[must_use]
        #[inline(always)]
        pub const fn doffset(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0x000f_ffff;
            val as u32
        }
        #[doc = "desc DOFFSET."]
        #[inline(always)]
        pub const fn set_doffset(&mut self, val: u32) {
            self.0 = (self.0 & !(0x000f_ffff << 0usize)) | (((val as u32) & 0x000f_ffff) << 0usize);
        }
        #[doc = "desc DNSCNT."]
        #[must_use]
        #[inline(always)]
        pub const fn dnscnt(&self) -> u16 {
            let val = (self.0 >> 20usize) & 0x0fff;
            val as u16
        }
        #[doc = "desc DNSCNT."]
        #[inline(always)]
        pub const fn set_dnscnt(&mut self, val: u16) {
            self.0 = (self.0 & !(0x0fff << 20usize)) | (((val as u32) & 0x0fff) << 20usize);
        }
    }
    impl Default for Mondnseqctl1 {
        #[inline(always)]
        fn default() -> Mondnseqctl1 {
            Mondnseqctl1(0)
        }
    }
    impl core::fmt::Debug for Mondnseqctl1 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Mondnseqctl1")
                .field("doffset", &self.doffset())
                .field("dnscnt", &self.dnscnt())
                .finish()
        }
    }
    #[doc = "desc MONDNSEQCTL2."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Mondnseqctl2(pub u32);
    impl Mondnseqctl2 {
        #[doc = "desc DOFFSET."]
        #[must_use]
        #[inline(always)]
        pub const fn doffset(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0x000f_ffff;
            val as u32
        }
        #[doc = "desc DOFFSET."]
        #[inline(always)]
        pub const fn set_doffset(&mut self, val: u32) {
            self.0 = (self.0 & !(0x000f_ffff << 0usize)) | (((val as u32) & 0x000f_ffff) << 0usize);
        }
        #[doc = "desc DNSCNT."]
        #[must_use]
        #[inline(always)]
        pub const fn dnscnt(&self) -> u16 {
            let val = (self.0 >> 20usize) & 0x0fff;
            val as u16
        }
        #[doc = "desc DNSCNT."]
        #[inline(always)]
        pub const fn set_dnscnt(&mut self, val: u16) {
            self.0 = (self.0 & !(0x0fff << 20usize)) | (((val as u32) & 0x0fff) << 20usize);
        }
    }
    impl Default for Mondnseqctl2 {
        #[inline(always)]
        fn default() -> Mondnseqctl2 {
            Mondnseqctl2(0)
        }
    }
    impl core::fmt::Debug for Mondnseqctl2 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Mondnseqctl2")
                .field("doffset", &self.doffset())
                .field("dnscnt", &self.dnscnt())
                .finish()
        }
    }
    #[doc = "desc MONDNSEQCTL3."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Mondnseqctl3(pub u32);
    impl Mondnseqctl3 {
        #[doc = "desc DOFFSET."]
        #[must_use]
        #[inline(always)]
        pub const fn doffset(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0x000f_ffff;
            val as u32
        }
        #[doc = "desc DOFFSET."]
        #[inline(always)]
        pub const fn set_doffset(&mut self, val: u32) {
            self.0 = (self.0 & !(0x000f_ffff << 0usize)) | (((val as u32) & 0x000f_ffff) << 0usize);
        }
        #[doc = "desc DNSCNT."]
        #[must_use]
        #[inline(always)]
        pub const fn dnscnt(&self) -> u16 {
            let val = (self.0 >> 20usize) & 0x0fff;
            val as u16
        }
        #[doc = "desc DNSCNT."]
        #[inline(always)]
        pub const fn set_dnscnt(&mut self, val: u16) {
            self.0 = (self.0 & !(0x0fff << 20usize)) | (((val as u32) & 0x0fff) << 20usize);
        }
    }
    impl Default for Mondnseqctl3 {
        #[inline(always)]
        fn default() -> Mondnseqctl3 {
            Mondnseqctl3(0)
        }
    }
    impl core::fmt::Debug for Mondnseqctl3 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Mondnseqctl3")
                .field("doffset", &self.doffset())
                .field("dnscnt", &self.dnscnt())
                .finish()
        }
    }
    #[doc = "desc MONDNSEQCTL4."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Mondnseqctl4(pub u32);
    impl Mondnseqctl4 {
        #[doc = "desc DOFFSET."]
        #[must_use]
        #[inline(always)]
        pub const fn doffset(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0x000f_ffff;
            val as u32
        }
        #[doc = "desc DOFFSET."]
        #[inline(always)]
        pub const fn set_doffset(&mut self, val: u32) {
            self.0 = (self.0 & !(0x000f_ffff << 0usize)) | (((val as u32) & 0x000f_ffff) << 0usize);
        }
        #[doc = "desc DNSCNT."]
        #[must_use]
        #[inline(always)]
        pub const fn dnscnt(&self) -> u16 {
            let val = (self.0 >> 20usize) & 0x0fff;
            val as u16
        }
        #[doc = "desc DNSCNT."]
        #[inline(always)]
        pub const fn set_dnscnt(&mut self, val: u16) {
            self.0 = (self.0 & !(0x0fff << 20usize)) | (((val as u32) & 0x0fff) << 20usize);
        }
    }
    impl Default for Mondnseqctl4 {
        #[inline(always)]
        fn default() -> Mondnseqctl4 {
            Mondnseqctl4(0)
        }
    }
    impl core::fmt::Debug for Mondnseqctl4 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Mondnseqctl4")
                .field("doffset", &self.doffset())
                .field("dnscnt", &self.dnscnt())
                .finish()
        }
    }
    #[doc = "desc MONDNSEQCTL5."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Mondnseqctl5(pub u32);
    impl Mondnseqctl5 {
        #[doc = "desc DOFFSET."]
        #[must_use]
        #[inline(always)]
        pub const fn doffset(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0x000f_ffff;
            val as u32
        }
        #[doc = "desc DOFFSET."]
        #[inline(always)]
        pub const fn set_doffset(&mut self, val: u32) {
            self.0 = (self.0 & !(0x000f_ffff << 0usize)) | (((val as u32) & 0x000f_ffff) << 0usize);
        }
        #[doc = "desc DNSCNT."]
        #[must_use]
        #[inline(always)]
        pub const fn dnscnt(&self) -> u16 {
            let val = (self.0 >> 20usize) & 0x0fff;
            val as u16
        }
        #[doc = "desc DNSCNT."]
        #[inline(always)]
        pub const fn set_dnscnt(&mut self, val: u16) {
            self.0 = (self.0 & !(0x0fff << 20usize)) | (((val as u32) & 0x0fff) << 20usize);
        }
    }
    impl Default for Mondnseqctl5 {
        #[inline(always)]
        fn default() -> Mondnseqctl5 {
            Mondnseqctl5(0)
        }
    }
    impl core::fmt::Debug for Mondnseqctl5 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Mondnseqctl5")
                .field("doffset", &self.doffset())
                .field("dnscnt", &self.dnscnt())
                .finish()
        }
    }
    #[doc = "desc MONDTCTL0."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Mondtctl0(pub u32);
    impl Mondtctl0 {
        #[doc = "desc BLKSIZE."]
        #[must_use]
        #[inline(always)]
        pub const fn blksize(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x03ff;
            val as u16
        }
        #[doc = "desc BLKSIZE."]
        #[inline(always)]
        pub const fn set_blksize(&mut self, val: u16) {
            self.0 = (self.0 & !(0x03ff << 0usize)) | (((val as u32) & 0x03ff) << 0usize);
        }
        #[doc = "desc CNT."]
        #[must_use]
        #[inline(always)]
        pub const fn cnt(&self) -> u16 {
            let val = (self.0 >> 16usize) & 0xffff;
            val as u16
        }
        #[doc = "desc CNT."]
        #[inline(always)]
        pub const fn set_cnt(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 16usize)) | (((val as u32) & 0xffff) << 16usize);
        }
    }
    impl Default for Mondtctl0 {
        #[inline(always)]
        fn default() -> Mondtctl0 {
            Mondtctl0(0)
        }
    }
    impl core::fmt::Debug for Mondtctl0 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Mondtctl0")
                .field("blksize", &self.blksize())
                .field("cnt", &self.cnt())
                .finish()
        }
    }
    #[doc = "desc MONDTCTL1."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Mondtctl1(pub u32);
    impl Mondtctl1 {
        #[doc = "desc BLKSIZE."]
        #[must_use]
        #[inline(always)]
        pub const fn blksize(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x03ff;
            val as u16
        }
        #[doc = "desc BLKSIZE."]
        #[inline(always)]
        pub const fn set_blksize(&mut self, val: u16) {
            self.0 = (self.0 & !(0x03ff << 0usize)) | (((val as u32) & 0x03ff) << 0usize);
        }
        #[doc = "desc CNT."]
        #[must_use]
        #[inline(always)]
        pub const fn cnt(&self) -> u16 {
            let val = (self.0 >> 16usize) & 0xffff;
            val as u16
        }
        #[doc = "desc CNT."]
        #[inline(always)]
        pub const fn set_cnt(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 16usize)) | (((val as u32) & 0xffff) << 16usize);
        }
    }
    impl Default for Mondtctl1 {
        #[inline(always)]
        fn default() -> Mondtctl1 {
            Mondtctl1(0)
        }
    }
    impl core::fmt::Debug for Mondtctl1 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Mondtctl1")
                .field("blksize", &self.blksize())
                .field("cnt", &self.cnt())
                .finish()
        }
    }
    #[doc = "desc MONDTCTL2."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Mondtctl2(pub u32);
    impl Mondtctl2 {
        #[doc = "desc BLKSIZE."]
        #[must_use]
        #[inline(always)]
        pub const fn blksize(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x03ff;
            val as u16
        }
        #[doc = "desc BLKSIZE."]
        #[inline(always)]
        pub const fn set_blksize(&mut self, val: u16) {
            self.0 = (self.0 & !(0x03ff << 0usize)) | (((val as u32) & 0x03ff) << 0usize);
        }
        #[doc = "desc CNT."]
        #[must_use]
        #[inline(always)]
        pub const fn cnt(&self) -> u16 {
            let val = (self.0 >> 16usize) & 0xffff;
            val as u16
        }
        #[doc = "desc CNT."]
        #[inline(always)]
        pub const fn set_cnt(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 16usize)) | (((val as u32) & 0xffff) << 16usize);
        }
    }
    impl Default for Mondtctl2 {
        #[inline(always)]
        fn default() -> Mondtctl2 {
            Mondtctl2(0)
        }
    }
    impl core::fmt::Debug for Mondtctl2 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Mondtctl2")
                .field("blksize", &self.blksize())
                .field("cnt", &self.cnt())
                .finish()
        }
    }
    #[doc = "desc MONDTCTL3."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Mondtctl3(pub u32);
    impl Mondtctl3 {
        #[doc = "desc BLKSIZE."]
        #[must_use]
        #[inline(always)]
        pub const fn blksize(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x03ff;
            val as u16
        }
        #[doc = "desc BLKSIZE."]
        #[inline(always)]
        pub const fn set_blksize(&mut self, val: u16) {
            self.0 = (self.0 & !(0x03ff << 0usize)) | (((val as u32) & 0x03ff) << 0usize);
        }
        #[doc = "desc CNT."]
        #[must_use]
        #[inline(always)]
        pub const fn cnt(&self) -> u16 {
            let val = (self.0 >> 16usize) & 0xffff;
            val as u16
        }
        #[doc = "desc CNT."]
        #[inline(always)]
        pub const fn set_cnt(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 16usize)) | (((val as u32) & 0xffff) << 16usize);
        }
    }
    impl Default for Mondtctl3 {
        #[inline(always)]
        fn default() -> Mondtctl3 {
            Mondtctl3(0)
        }
    }
    impl core::fmt::Debug for Mondtctl3 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Mondtctl3")
                .field("blksize", &self.blksize())
                .field("cnt", &self.cnt())
                .finish()
        }
    }
    #[doc = "desc MONDTCTL4."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Mondtctl4(pub u32);
    impl Mondtctl4 {
        #[doc = "desc BLKSIZE."]
        #[must_use]
        #[inline(always)]
        pub const fn blksize(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x03ff;
            val as u16
        }
        #[doc = "desc BLKSIZE."]
        #[inline(always)]
        pub const fn set_blksize(&mut self, val: u16) {
            self.0 = (self.0 & !(0x03ff << 0usize)) | (((val as u32) & 0x03ff) << 0usize);
        }
        #[doc = "desc CNT."]
        #[must_use]
        #[inline(always)]
        pub const fn cnt(&self) -> u16 {
            let val = (self.0 >> 16usize) & 0xffff;
            val as u16
        }
        #[doc = "desc CNT."]
        #[inline(always)]
        pub const fn set_cnt(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 16usize)) | (((val as u32) & 0xffff) << 16usize);
        }
    }
    impl Default for Mondtctl4 {
        #[inline(always)]
        fn default() -> Mondtctl4 {
            Mondtctl4(0)
        }
    }
    impl core::fmt::Debug for Mondtctl4 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Mondtctl4")
                .field("blksize", &self.blksize())
                .field("cnt", &self.cnt())
                .finish()
        }
    }
    #[doc = "desc MONDTCTL5."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Mondtctl5(pub u32);
    impl Mondtctl5 {
        #[doc = "desc BLKSIZE."]
        #[must_use]
        #[inline(always)]
        pub const fn blksize(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x03ff;
            val as u16
        }
        #[doc = "desc BLKSIZE."]
        #[inline(always)]
        pub const fn set_blksize(&mut self, val: u16) {
            self.0 = (self.0 & !(0x03ff << 0usize)) | (((val as u32) & 0x03ff) << 0usize);
        }
        #[doc = "desc CNT."]
        #[must_use]
        #[inline(always)]
        pub const fn cnt(&self) -> u16 {
            let val = (self.0 >> 16usize) & 0xffff;
            val as u16
        }
        #[doc = "desc CNT."]
        #[inline(always)]
        pub const fn set_cnt(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 16usize)) | (((val as u32) & 0xffff) << 16usize);
        }
    }
    impl Default for Mondtctl5 {
        #[inline(always)]
        fn default() -> Mondtctl5 {
            Mondtctl5(0)
        }
    }
    impl core::fmt::Debug for Mondtctl5 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Mondtctl5")
                .field("blksize", &self.blksize())
                .field("cnt", &self.cnt())
                .finish()
        }
    }
    #[doc = "desc MONRPT0."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Monrpt0(pub u32);
    impl Monrpt0 {
        #[doc = "desc SRPT."]
        #[must_use]
        #[inline(always)]
        pub const fn srpt(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x03ff;
            val as u16
        }
        #[doc = "desc SRPT."]
        #[inline(always)]
        pub const fn set_srpt(&mut self, val: u16) {
            self.0 = (self.0 & !(0x03ff << 0usize)) | (((val as u32) & 0x03ff) << 0usize);
        }
        #[doc = "desc DRPT."]
        #[must_use]
        #[inline(always)]
        pub const fn drpt(&self) -> u16 {
            let val = (self.0 >> 16usize) & 0x03ff;
            val as u16
        }
        #[doc = "desc DRPT."]
        #[inline(always)]
        pub const fn set_drpt(&mut self, val: u16) {
            self.0 = (self.0 & !(0x03ff << 16usize)) | (((val as u32) & 0x03ff) << 16usize);
        }
    }
    impl Default for Monrpt0 {
        #[inline(always)]
        fn default() -> Monrpt0 {
            Monrpt0(0)
        }
    }
    impl core::fmt::Debug for Monrpt0 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Monrpt0")
                .field("srpt", &self.srpt())
                .field("drpt", &self.drpt())
                .finish()
        }
    }
    #[doc = "desc MONRPT1."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Monrpt1(pub u32);
    impl Monrpt1 {
        #[doc = "desc SRPT."]
        #[must_use]
        #[inline(always)]
        pub const fn srpt(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x03ff;
            val as u16
        }
        #[doc = "desc SRPT."]
        #[inline(always)]
        pub const fn set_srpt(&mut self, val: u16) {
            self.0 = (self.0 & !(0x03ff << 0usize)) | (((val as u32) & 0x03ff) << 0usize);
        }
        #[doc = "desc DRPT."]
        #[must_use]
        #[inline(always)]
        pub const fn drpt(&self) -> u16 {
            let val = (self.0 >> 16usize) & 0x03ff;
            val as u16
        }
        #[doc = "desc DRPT."]
        #[inline(always)]
        pub const fn set_drpt(&mut self, val: u16) {
            self.0 = (self.0 & !(0x03ff << 16usize)) | (((val as u32) & 0x03ff) << 16usize);
        }
    }
    impl Default for Monrpt1 {
        #[inline(always)]
        fn default() -> Monrpt1 {
            Monrpt1(0)
        }
    }
    impl core::fmt::Debug for Monrpt1 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Monrpt1")
                .field("srpt", &self.srpt())
                .field("drpt", &self.drpt())
                .finish()
        }
    }
    #[doc = "desc MONRPT2."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Monrpt2(pub u32);
    impl Monrpt2 {
        #[doc = "desc SRPT."]
        #[must_use]
        #[inline(always)]
        pub const fn srpt(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x03ff;
            val as u16
        }
        #[doc = "desc SRPT."]
        #[inline(always)]
        pub const fn set_srpt(&mut self, val: u16) {
            self.0 = (self.0 & !(0x03ff << 0usize)) | (((val as u32) & 0x03ff) << 0usize);
        }
        #[doc = "desc DRPT."]
        #[must_use]
        #[inline(always)]
        pub const fn drpt(&self) -> u16 {
            let val = (self.0 >> 16usize) & 0x03ff;
            val as u16
        }
        #[doc = "desc DRPT."]
        #[inline(always)]
        pub const fn set_drpt(&mut self, val: u16) {
            self.0 = (self.0 & !(0x03ff << 16usize)) | (((val as u32) & 0x03ff) << 16usize);
        }
    }
    impl Default for Monrpt2 {
        #[inline(always)]
        fn default() -> Monrpt2 {
            Monrpt2(0)
        }
    }
    impl core::fmt::Debug for Monrpt2 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Monrpt2")
                .field("srpt", &self.srpt())
                .field("drpt", &self.drpt())
                .finish()
        }
    }
    #[doc = "desc MONRPT3."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Monrpt3(pub u32);
    impl Monrpt3 {
        #[doc = "desc SRPT."]
        #[must_use]
        #[inline(always)]
        pub const fn srpt(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x03ff;
            val as u16
        }
        #[doc = "desc SRPT."]
        #[inline(always)]
        pub const fn set_srpt(&mut self, val: u16) {
            self.0 = (self.0 & !(0x03ff << 0usize)) | (((val as u32) & 0x03ff) << 0usize);
        }
        #[doc = "desc DRPT."]
        #[must_use]
        #[inline(always)]
        pub const fn drpt(&self) -> u16 {
            let val = (self.0 >> 16usize) & 0x03ff;
            val as u16
        }
        #[doc = "desc DRPT."]
        #[inline(always)]
        pub const fn set_drpt(&mut self, val: u16) {
            self.0 = (self.0 & !(0x03ff << 16usize)) | (((val as u32) & 0x03ff) << 16usize);
        }
    }
    impl Default for Monrpt3 {
        #[inline(always)]
        fn default() -> Monrpt3 {
            Monrpt3(0)
        }
    }
    impl core::fmt::Debug for Monrpt3 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Monrpt3")
                .field("srpt", &self.srpt())
                .field("drpt", &self.drpt())
                .finish()
        }
    }
    #[doc = "desc MONRPT4."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Monrpt4(pub u32);
    impl Monrpt4 {
        #[doc = "desc SRPT."]
        #[must_use]
        #[inline(always)]
        pub const fn srpt(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x03ff;
            val as u16
        }
        #[doc = "desc SRPT."]
        #[inline(always)]
        pub const fn set_srpt(&mut self, val: u16) {
            self.0 = (self.0 & !(0x03ff << 0usize)) | (((val as u32) & 0x03ff) << 0usize);
        }
        #[doc = "desc DRPT."]
        #[must_use]
        #[inline(always)]
        pub const fn drpt(&self) -> u16 {
            let val = (self.0 >> 16usize) & 0x03ff;
            val as u16
        }
        #[doc = "desc DRPT."]
        #[inline(always)]
        pub const fn set_drpt(&mut self, val: u16) {
            self.0 = (self.0 & !(0x03ff << 16usize)) | (((val as u32) & 0x03ff) << 16usize);
        }
    }
    impl Default for Monrpt4 {
        #[inline(always)]
        fn default() -> Monrpt4 {
            Monrpt4(0)
        }
    }
    impl core::fmt::Debug for Monrpt4 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Monrpt4")
                .field("srpt", &self.srpt())
                .field("drpt", &self.drpt())
                .finish()
        }
    }
    #[doc = "desc MONRPT5."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Monrpt5(pub u32);
    impl Monrpt5 {
        #[doc = "desc SRPT."]
        #[must_use]
        #[inline(always)]
        pub const fn srpt(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x03ff;
            val as u16
        }
        #[doc = "desc SRPT."]
        #[inline(always)]
        pub const fn set_srpt(&mut self, val: u16) {
            self.0 = (self.0 & !(0x03ff << 0usize)) | (((val as u32) & 0x03ff) << 0usize);
        }
        #[doc = "desc DRPT."]
        #[must_use]
        #[inline(always)]
        pub const fn drpt(&self) -> u16 {
            let val = (self.0 >> 16usize) & 0x03ff;
            val as u16
        }
        #[doc = "desc DRPT."]
        #[inline(always)]
        pub const fn set_drpt(&mut self, val: u16) {
            self.0 = (self.0 & !(0x03ff << 16usize)) | (((val as u32) & 0x03ff) << 16usize);
        }
    }
    impl Default for Monrpt5 {
        #[inline(always)]
        fn default() -> Monrpt5 {
            Monrpt5(0)
        }
    }
    impl core::fmt::Debug for Monrpt5 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Monrpt5")
                .field("srpt", &self.srpt())
                .field("drpt", &self.drpt())
                .finish()
        }
    }
    #[doc = "desc MONSNSEQCTL0."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Monsnseqctl0(pub u32);
    impl Monsnseqctl0 {
        #[doc = "desc SOFFSET."]
        #[must_use]
        #[inline(always)]
        pub const fn soffset(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0x000f_ffff;
            val as u32
        }
        #[doc = "desc SOFFSET."]
        #[inline(always)]
        pub const fn set_soffset(&mut self, val: u32) {
            self.0 = (self.0 & !(0x000f_ffff << 0usize)) | (((val as u32) & 0x000f_ffff) << 0usize);
        }
        #[doc = "desc SNSCNT."]
        #[must_use]
        #[inline(always)]
        pub const fn snscnt(&self) -> u16 {
            let val = (self.0 >> 20usize) & 0x0fff;
            val as u16
        }
        #[doc = "desc SNSCNT."]
        #[inline(always)]
        pub const fn set_snscnt(&mut self, val: u16) {
            self.0 = (self.0 & !(0x0fff << 20usize)) | (((val as u32) & 0x0fff) << 20usize);
        }
    }
    impl Default for Monsnseqctl0 {
        #[inline(always)]
        fn default() -> Monsnseqctl0 {
            Monsnseqctl0(0)
        }
    }
    impl core::fmt::Debug for Monsnseqctl0 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Monsnseqctl0")
                .field("soffset", &self.soffset())
                .field("snscnt", &self.snscnt())
                .finish()
        }
    }
    #[doc = "desc MONSNSEQCTL1."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Monsnseqctl1(pub u32);
    impl Monsnseqctl1 {
        #[doc = "desc SOFFSET."]
        #[must_use]
        #[inline(always)]
        pub const fn soffset(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0x000f_ffff;
            val as u32
        }
        #[doc = "desc SOFFSET."]
        #[inline(always)]
        pub const fn set_soffset(&mut self, val: u32) {
            self.0 = (self.0 & !(0x000f_ffff << 0usize)) | (((val as u32) & 0x000f_ffff) << 0usize);
        }
        #[doc = "desc SNSCNT."]
        #[must_use]
        #[inline(always)]
        pub const fn snscnt(&self) -> u16 {
            let val = (self.0 >> 20usize) & 0x0fff;
            val as u16
        }
        #[doc = "desc SNSCNT."]
        #[inline(always)]
        pub const fn set_snscnt(&mut self, val: u16) {
            self.0 = (self.0 & !(0x0fff << 20usize)) | (((val as u32) & 0x0fff) << 20usize);
        }
    }
    impl Default for Monsnseqctl1 {
        #[inline(always)]
        fn default() -> Monsnseqctl1 {
            Monsnseqctl1(0)
        }
    }
    impl core::fmt::Debug for Monsnseqctl1 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Monsnseqctl1")
                .field("soffset", &self.soffset())
                .field("snscnt", &self.snscnt())
                .finish()
        }
    }
    #[doc = "desc MONSNSEQCTL2."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Monsnseqctl2(pub u32);
    impl Monsnseqctl2 {
        #[doc = "desc SOFFSET."]
        #[must_use]
        #[inline(always)]
        pub const fn soffset(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0x000f_ffff;
            val as u32
        }
        #[doc = "desc SOFFSET."]
        #[inline(always)]
        pub const fn set_soffset(&mut self, val: u32) {
            self.0 = (self.0 & !(0x000f_ffff << 0usize)) | (((val as u32) & 0x000f_ffff) << 0usize);
        }
        #[doc = "desc SNSCNT."]
        #[must_use]
        #[inline(always)]
        pub const fn snscnt(&self) -> u16 {
            let val = (self.0 >> 20usize) & 0x0fff;
            val as u16
        }
        #[doc = "desc SNSCNT."]
        #[inline(always)]
        pub const fn set_snscnt(&mut self, val: u16) {
            self.0 = (self.0 & !(0x0fff << 20usize)) | (((val as u32) & 0x0fff) << 20usize);
        }
    }
    impl Default for Monsnseqctl2 {
        #[inline(always)]
        fn default() -> Monsnseqctl2 {
            Monsnseqctl2(0)
        }
    }
    impl core::fmt::Debug for Monsnseqctl2 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Monsnseqctl2")
                .field("soffset", &self.soffset())
                .field("snscnt", &self.snscnt())
                .finish()
        }
    }
    #[doc = "desc MONSNSEQCTL3."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Monsnseqctl3(pub u32);
    impl Monsnseqctl3 {
        #[doc = "desc SOFFSET."]
        #[must_use]
        #[inline(always)]
        pub const fn soffset(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0x000f_ffff;
            val as u32
        }
        #[doc = "desc SOFFSET."]
        #[inline(always)]
        pub const fn set_soffset(&mut self, val: u32) {
            self.0 = (self.0 & !(0x000f_ffff << 0usize)) | (((val as u32) & 0x000f_ffff) << 0usize);
        }
        #[doc = "desc SNSCNT."]
        #[must_use]
        #[inline(always)]
        pub const fn snscnt(&self) -> u16 {
            let val = (self.0 >> 20usize) & 0x0fff;
            val as u16
        }
        #[doc = "desc SNSCNT."]
        #[inline(always)]
        pub const fn set_snscnt(&mut self, val: u16) {
            self.0 = (self.0 & !(0x0fff << 20usize)) | (((val as u32) & 0x0fff) << 20usize);
        }
    }
    impl Default for Monsnseqctl3 {
        #[inline(always)]
        fn default() -> Monsnseqctl3 {
            Monsnseqctl3(0)
        }
    }
    impl core::fmt::Debug for Monsnseqctl3 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Monsnseqctl3")
                .field("soffset", &self.soffset())
                .field("snscnt", &self.snscnt())
                .finish()
        }
    }
    #[doc = "desc MONSNSEQCTL4."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Monsnseqctl4(pub u32);
    impl Monsnseqctl4 {
        #[doc = "desc SOFFSET."]
        #[must_use]
        #[inline(always)]
        pub const fn soffset(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0x000f_ffff;
            val as u32
        }
        #[doc = "desc SOFFSET."]
        #[inline(always)]
        pub const fn set_soffset(&mut self, val: u32) {
            self.0 = (self.0 & !(0x000f_ffff << 0usize)) | (((val as u32) & 0x000f_ffff) << 0usize);
        }
        #[doc = "desc SNSCNT."]
        #[must_use]
        #[inline(always)]
        pub const fn snscnt(&self) -> u16 {
            let val = (self.0 >> 20usize) & 0x0fff;
            val as u16
        }
        #[doc = "desc SNSCNT."]
        #[inline(always)]
        pub const fn set_snscnt(&mut self, val: u16) {
            self.0 = (self.0 & !(0x0fff << 20usize)) | (((val as u32) & 0x0fff) << 20usize);
        }
    }
    impl Default for Monsnseqctl4 {
        #[inline(always)]
        fn default() -> Monsnseqctl4 {
            Monsnseqctl4(0)
        }
    }
    impl core::fmt::Debug for Monsnseqctl4 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Monsnseqctl4")
                .field("soffset", &self.soffset())
                .field("snscnt", &self.snscnt())
                .finish()
        }
    }
    #[doc = "desc MONSNSEQCTL5."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Monsnseqctl5(pub u32);
    impl Monsnseqctl5 {
        #[doc = "desc SOFFSET."]
        #[must_use]
        #[inline(always)]
        pub const fn soffset(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0x000f_ffff;
            val as u32
        }
        #[doc = "desc SOFFSET."]
        #[inline(always)]
        pub const fn set_soffset(&mut self, val: u32) {
            self.0 = (self.0 & !(0x000f_ffff << 0usize)) | (((val as u32) & 0x000f_ffff) << 0usize);
        }
        #[doc = "desc SNSCNT."]
        #[must_use]
        #[inline(always)]
        pub const fn snscnt(&self) -> u16 {
            let val = (self.0 >> 20usize) & 0x0fff;
            val as u16
        }
        #[doc = "desc SNSCNT."]
        #[inline(always)]
        pub const fn set_snscnt(&mut self, val: u16) {
            self.0 = (self.0 & !(0x0fff << 20usize)) | (((val as u32) & 0x0fff) << 20usize);
        }
    }
    impl Default for Monsnseqctl5 {
        #[inline(always)]
        fn default() -> Monsnseqctl5 {
            Monsnseqctl5(0)
        }
    }
    impl core::fmt::Debug for Monsnseqctl5 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Monsnseqctl5")
                .field("soffset", &self.soffset())
                .field("snscnt", &self.snscnt())
                .finish()
        }
    }
    #[doc = "desc RCFGCTL."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Rcfgctl(pub u32);
    impl Rcfgctl {
        #[doc = "desc RCFGEN."]
        #[must_use]
        #[inline(always)]
        pub const fn rcfgen(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "desc RCFGEN."]
        #[inline(always)]
        pub const fn set_rcfgen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "desc RCFGLLP."]
        #[must_use]
        #[inline(always)]
        pub const fn rcfgllp(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "desc RCFGLLP."]
        #[inline(always)]
        pub const fn set_rcfgllp(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "desc RCFGCHS."]
        #[must_use]
        #[inline(always)]
        pub const fn rcfgchs(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0x0f;
            val as u8
        }
        #[doc = "desc RCFGCHS."]
        #[inline(always)]
        pub const fn set_rcfgchs(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 8usize)) | (((val as u32) & 0x0f) << 8usize);
        }
        #[doc = "desc SARMD."]
        #[must_use]
        #[inline(always)]
        pub const fn sarmd(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x03;
            val as u8
        }
        #[doc = "desc SARMD."]
        #[inline(always)]
        pub const fn set_sarmd(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 16usize)) | (((val as u32) & 0x03) << 16usize);
        }
        #[doc = "desc DARMD."]
        #[must_use]
        #[inline(always)]
        pub const fn darmd(&self) -> u8 {
            let val = (self.0 >> 18usize) & 0x03;
            val as u8
        }
        #[doc = "desc DARMD."]
        #[inline(always)]
        pub const fn set_darmd(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 18usize)) | (((val as u32) & 0x03) << 18usize);
        }
        #[doc = "desc CNTMD."]
        #[must_use]
        #[inline(always)]
        pub const fn cntmd(&self) -> u8 {
            let val = (self.0 >> 20usize) & 0x03;
            val as u8
        }
        #[doc = "desc CNTMD."]
        #[inline(always)]
        pub const fn set_cntmd(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 20usize)) | (((val as u32) & 0x03) << 20usize);
        }
    }
    impl Default for Rcfgctl {
        #[inline(always)]
        fn default() -> Rcfgctl {
            Rcfgctl(0)
        }
    }
    impl core::fmt::Debug for Rcfgctl {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Rcfgctl")
                .field("rcfgen", &self.rcfgen())
                .field("rcfgllp", &self.rcfgllp())
                .field("rcfgchs", &self.rcfgchs())
                .field("sarmd", &self.sarmd())
                .field("darmd", &self.darmd())
                .field("cntmd", &self.cntmd())
                .finish()
        }
    }
    #[doc = "desc REQSTAT."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Reqstat(pub u32);
    impl Reqstat {
        #[doc = "desc CHREQ."]
        #[must_use]
        #[inline(always)]
        pub const fn chreq(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x3f;
            val as u8
        }
        #[doc = "desc CHREQ."]
        #[inline(always)]
        pub const fn set_chreq(&mut self, val: u8) {
            self.0 = (self.0 & !(0x3f << 0usize)) | (((val as u32) & 0x3f) << 0usize);
        }
        #[doc = "desc RCFGREQ."]
        #[must_use]
        #[inline(always)]
        pub const fn rcfgreq(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "desc RCFGREQ."]
        #[inline(always)]
        pub const fn set_rcfgreq(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
    }
    impl Default for Reqstat {
        #[inline(always)]
        fn default() -> Reqstat {
            Reqstat(0)
        }
    }
    impl core::fmt::Debug for Reqstat {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Reqstat")
                .field("chreq", &self.chreq())
                .field("rcfgreq", &self.rcfgreq())
                .finish()
        }
    }
    #[doc = "desc RPT0."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Rpt0(pub u32);
    impl Rpt0 {
        #[doc = "desc SRPT."]
        #[must_use]
        #[inline(always)]
        pub const fn srpt(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x03ff;
            val as u16
        }
        #[doc = "desc SRPT."]
        #[inline(always)]
        pub const fn set_srpt(&mut self, val: u16) {
            self.0 = (self.0 & !(0x03ff << 0usize)) | (((val as u32) & 0x03ff) << 0usize);
        }
        #[doc = "desc DRPT."]
        #[must_use]
        #[inline(always)]
        pub const fn drpt(&self) -> u16 {
            let val = (self.0 >> 16usize) & 0x03ff;
            val as u16
        }
        #[doc = "desc DRPT."]
        #[inline(always)]
        pub const fn set_drpt(&mut self, val: u16) {
            self.0 = (self.0 & !(0x03ff << 16usize)) | (((val as u32) & 0x03ff) << 16usize);
        }
    }
    impl Default for Rpt0 {
        #[inline(always)]
        fn default() -> Rpt0 {
            Rpt0(0)
        }
    }
    impl core::fmt::Debug for Rpt0 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Rpt0")
                .field("srpt", &self.srpt())
                .field("drpt", &self.drpt())
                .finish()
        }
    }
    #[doc = "desc RPT1."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Rpt1(pub u32);
    impl Rpt1 {
        #[doc = "desc SRPT."]
        #[must_use]
        #[inline(always)]
        pub const fn srpt(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x03ff;
            val as u16
        }
        #[doc = "desc SRPT."]
        #[inline(always)]
        pub const fn set_srpt(&mut self, val: u16) {
            self.0 = (self.0 & !(0x03ff << 0usize)) | (((val as u32) & 0x03ff) << 0usize);
        }
        #[doc = "desc DRPT."]
        #[must_use]
        #[inline(always)]
        pub const fn drpt(&self) -> u16 {
            let val = (self.0 >> 16usize) & 0x03ff;
            val as u16
        }
        #[doc = "desc DRPT."]
        #[inline(always)]
        pub const fn set_drpt(&mut self, val: u16) {
            self.0 = (self.0 & !(0x03ff << 16usize)) | (((val as u32) & 0x03ff) << 16usize);
        }
    }
    impl Default for Rpt1 {
        #[inline(always)]
        fn default() -> Rpt1 {
            Rpt1(0)
        }
    }
    impl core::fmt::Debug for Rpt1 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Rpt1")
                .field("srpt", &self.srpt())
                .field("drpt", &self.drpt())
                .finish()
        }
    }
    #[doc = "desc RPT2."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Rpt2(pub u32);
    impl Rpt2 {
        #[doc = "desc SRPT."]
        #[must_use]
        #[inline(always)]
        pub const fn srpt(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x03ff;
            val as u16
        }
        #[doc = "desc SRPT."]
        #[inline(always)]
        pub const fn set_srpt(&mut self, val: u16) {
            self.0 = (self.0 & !(0x03ff << 0usize)) | (((val as u32) & 0x03ff) << 0usize);
        }
        #[doc = "desc DRPT."]
        #[must_use]
        #[inline(always)]
        pub const fn drpt(&self) -> u16 {
            let val = (self.0 >> 16usize) & 0x03ff;
            val as u16
        }
        #[doc = "desc DRPT."]
        #[inline(always)]
        pub const fn set_drpt(&mut self, val: u16) {
            self.0 = (self.0 & !(0x03ff << 16usize)) | (((val as u32) & 0x03ff) << 16usize);
        }
    }
    impl Default for Rpt2 {
        #[inline(always)]
        fn default() -> Rpt2 {
            Rpt2(0)
        }
    }
    impl core::fmt::Debug for Rpt2 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Rpt2")
                .field("srpt", &self.srpt())
                .field("drpt", &self.drpt())
                .finish()
        }
    }
    #[doc = "desc RPT3."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Rpt3(pub u32);
    impl Rpt3 {
        #[doc = "desc SRPT."]
        #[must_use]
        #[inline(always)]
        pub const fn srpt(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x03ff;
            val as u16
        }
        #[doc = "desc SRPT."]
        #[inline(always)]
        pub const fn set_srpt(&mut self, val: u16) {
            self.0 = (self.0 & !(0x03ff << 0usize)) | (((val as u32) & 0x03ff) << 0usize);
        }
        #[doc = "desc DRPT."]
        #[must_use]
        #[inline(always)]
        pub const fn drpt(&self) -> u16 {
            let val = (self.0 >> 16usize) & 0x03ff;
            val as u16
        }
        #[doc = "desc DRPT."]
        #[inline(always)]
        pub const fn set_drpt(&mut self, val: u16) {
            self.0 = (self.0 & !(0x03ff << 16usize)) | (((val as u32) & 0x03ff) << 16usize);
        }
    }
    impl Default for Rpt3 {
        #[inline(always)]
        fn default() -> Rpt3 {
            Rpt3(0)
        }
    }
    impl core::fmt::Debug for Rpt3 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Rpt3")
                .field("srpt", &self.srpt())
                .field("drpt", &self.drpt())
                .finish()
        }
    }
    #[doc = "desc RPT4."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Rpt4(pub u32);
    impl Rpt4 {
        #[doc = "desc SRPT."]
        #[must_use]
        #[inline(always)]
        pub const fn srpt(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x03ff;
            val as u16
        }
        #[doc = "desc SRPT."]
        #[inline(always)]
        pub const fn set_srpt(&mut self, val: u16) {
            self.0 = (self.0 & !(0x03ff << 0usize)) | (((val as u32) & 0x03ff) << 0usize);
        }
        #[doc = "desc DRPT."]
        #[must_use]
        #[inline(always)]
        pub const fn drpt(&self) -> u16 {
            let val = (self.0 >> 16usize) & 0x03ff;
            val as u16
        }
        #[doc = "desc DRPT."]
        #[inline(always)]
        pub const fn set_drpt(&mut self, val: u16) {
            self.0 = (self.0 & !(0x03ff << 16usize)) | (((val as u32) & 0x03ff) << 16usize);
        }
    }
    impl Default for Rpt4 {
        #[inline(always)]
        fn default() -> Rpt4 {
            Rpt4(0)
        }
    }
    impl core::fmt::Debug for Rpt4 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Rpt4")
                .field("srpt", &self.srpt())
                .field("drpt", &self.drpt())
                .finish()
        }
    }
    #[doc = "desc RPT5."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Rpt5(pub u32);
    impl Rpt5 {
        #[doc = "desc SRPT."]
        #[must_use]
        #[inline(always)]
        pub const fn srpt(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x03ff;
            val as u16
        }
        #[doc = "desc SRPT."]
        #[inline(always)]
        pub const fn set_srpt(&mut self, val: u16) {
            self.0 = (self.0 & !(0x03ff << 0usize)) | (((val as u32) & 0x03ff) << 0usize);
        }
        #[doc = "desc DRPT."]
        #[must_use]
        #[inline(always)]
        pub const fn drpt(&self) -> u16 {
            let val = (self.0 >> 16usize) & 0x03ff;
            val as u16
        }
        #[doc = "desc DRPT."]
        #[inline(always)]
        pub const fn set_drpt(&mut self, val: u16) {
            self.0 = (self.0 & !(0x03ff << 16usize)) | (((val as u32) & 0x03ff) << 16usize);
        }
    }
    impl Default for Rpt5 {
        #[inline(always)]
        fn default() -> Rpt5 {
            Rpt5(0)
        }
    }
    impl core::fmt::Debug for Rpt5 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Rpt5")
                .field("srpt", &self.srpt())
                .field("drpt", &self.drpt())
                .finish()
        }
    }
    #[doc = "desc RPTB0."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Rptb0(pub u32);
    impl Rptb0 {
        #[doc = "desc SRPTB."]
        #[must_use]
        #[inline(always)]
        pub const fn srptb(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x03ff;
            val as u16
        }
        #[doc = "desc SRPTB."]
        #[inline(always)]
        pub const fn set_srptb(&mut self, val: u16) {
            self.0 = (self.0 & !(0x03ff << 0usize)) | (((val as u32) & 0x03ff) << 0usize);
        }
        #[doc = "desc DRPTB."]
        #[must_use]
        #[inline(always)]
        pub const fn drptb(&self) -> u16 {
            let val = (self.0 >> 16usize) & 0x03ff;
            val as u16
        }
        #[doc = "desc DRPTB."]
        #[inline(always)]
        pub const fn set_drptb(&mut self, val: u16) {
            self.0 = (self.0 & !(0x03ff << 16usize)) | (((val as u32) & 0x03ff) << 16usize);
        }
    }
    impl Default for Rptb0 {
        #[inline(always)]
        fn default() -> Rptb0 {
            Rptb0(0)
        }
    }
    impl core::fmt::Debug for Rptb0 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Rptb0")
                .field("srptb", &self.srptb())
                .field("drptb", &self.drptb())
                .finish()
        }
    }
    #[doc = "desc RPTB1."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Rptb1(pub u32);
    impl Rptb1 {
        #[doc = "desc SRPTB."]
        #[must_use]
        #[inline(always)]
        pub const fn srptb(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x03ff;
            val as u16
        }
        #[doc = "desc SRPTB."]
        #[inline(always)]
        pub const fn set_srptb(&mut self, val: u16) {
            self.0 = (self.0 & !(0x03ff << 0usize)) | (((val as u32) & 0x03ff) << 0usize);
        }
        #[doc = "desc DRPTB."]
        #[must_use]
        #[inline(always)]
        pub const fn drptb(&self) -> u16 {
            let val = (self.0 >> 16usize) & 0x03ff;
            val as u16
        }
        #[doc = "desc DRPTB."]
        #[inline(always)]
        pub const fn set_drptb(&mut self, val: u16) {
            self.0 = (self.0 & !(0x03ff << 16usize)) | (((val as u32) & 0x03ff) << 16usize);
        }
    }
    impl Default for Rptb1 {
        #[inline(always)]
        fn default() -> Rptb1 {
            Rptb1(0)
        }
    }
    impl core::fmt::Debug for Rptb1 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Rptb1")
                .field("srptb", &self.srptb())
                .field("drptb", &self.drptb())
                .finish()
        }
    }
    #[doc = "desc RPTB2."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Rptb2(pub u32);
    impl Rptb2 {
        #[doc = "desc SRPTB."]
        #[must_use]
        #[inline(always)]
        pub const fn srptb(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x03ff;
            val as u16
        }
        #[doc = "desc SRPTB."]
        #[inline(always)]
        pub const fn set_srptb(&mut self, val: u16) {
            self.0 = (self.0 & !(0x03ff << 0usize)) | (((val as u32) & 0x03ff) << 0usize);
        }
        #[doc = "desc DRPTB."]
        #[must_use]
        #[inline(always)]
        pub const fn drptb(&self) -> u16 {
            let val = (self.0 >> 16usize) & 0x03ff;
            val as u16
        }
        #[doc = "desc DRPTB."]
        #[inline(always)]
        pub const fn set_drptb(&mut self, val: u16) {
            self.0 = (self.0 & !(0x03ff << 16usize)) | (((val as u32) & 0x03ff) << 16usize);
        }
    }
    impl Default for Rptb2 {
        #[inline(always)]
        fn default() -> Rptb2 {
            Rptb2(0)
        }
    }
    impl core::fmt::Debug for Rptb2 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Rptb2")
                .field("srptb", &self.srptb())
                .field("drptb", &self.drptb())
                .finish()
        }
    }
    #[doc = "desc RPTB3."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Rptb3(pub u32);
    impl Rptb3 {
        #[doc = "desc SRPTB."]
        #[must_use]
        #[inline(always)]
        pub const fn srptb(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x03ff;
            val as u16
        }
        #[doc = "desc SRPTB."]
        #[inline(always)]
        pub const fn set_srptb(&mut self, val: u16) {
            self.0 = (self.0 & !(0x03ff << 0usize)) | (((val as u32) & 0x03ff) << 0usize);
        }
        #[doc = "desc DRPTB."]
        #[must_use]
        #[inline(always)]
        pub const fn drptb(&self) -> u16 {
            let val = (self.0 >> 16usize) & 0x03ff;
            val as u16
        }
        #[doc = "desc DRPTB."]
        #[inline(always)]
        pub const fn set_drptb(&mut self, val: u16) {
            self.0 = (self.0 & !(0x03ff << 16usize)) | (((val as u32) & 0x03ff) << 16usize);
        }
    }
    impl Default for Rptb3 {
        #[inline(always)]
        fn default() -> Rptb3 {
            Rptb3(0)
        }
    }
    impl core::fmt::Debug for Rptb3 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Rptb3")
                .field("srptb", &self.srptb())
                .field("drptb", &self.drptb())
                .finish()
        }
    }
    #[doc = "desc RPTB4."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Rptb4(pub u32);
    impl Rptb4 {
        #[doc = "desc SRPTB."]
        #[must_use]
        #[inline(always)]
        pub const fn srptb(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x03ff;
            val as u16
        }
        #[doc = "desc SRPTB."]
        #[inline(always)]
        pub const fn set_srptb(&mut self, val: u16) {
            self.0 = (self.0 & !(0x03ff << 0usize)) | (((val as u32) & 0x03ff) << 0usize);
        }
        #[doc = "desc DRPTB."]
        #[must_use]
        #[inline(always)]
        pub const fn drptb(&self) -> u16 {
            let val = (self.0 >> 16usize) & 0x03ff;
            val as u16
        }
        #[doc = "desc DRPTB."]
        #[inline(always)]
        pub const fn set_drptb(&mut self, val: u16) {
            self.0 = (self.0 & !(0x03ff << 16usize)) | (((val as u32) & 0x03ff) << 16usize);
        }
    }
    impl Default for Rptb4 {
        #[inline(always)]
        fn default() -> Rptb4 {
            Rptb4(0)
        }
    }
    impl core::fmt::Debug for Rptb4 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Rptb4")
                .field("srptb", &self.srptb())
                .field("drptb", &self.drptb())
                .finish()
        }
    }
    #[doc = "desc RPTB5."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Rptb5(pub u32);
    impl Rptb5 {
        #[doc = "desc SRPTB."]
        #[must_use]
        #[inline(always)]
        pub const fn srptb(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x03ff;
            val as u16
        }
        #[doc = "desc SRPTB."]
        #[inline(always)]
        pub const fn set_srptb(&mut self, val: u16) {
            self.0 = (self.0 & !(0x03ff << 0usize)) | (((val as u32) & 0x03ff) << 0usize);
        }
        #[doc = "desc DRPTB."]
        #[must_use]
        #[inline(always)]
        pub const fn drptb(&self) -> u16 {
            let val = (self.0 >> 16usize) & 0x03ff;
            val as u16
        }
        #[doc = "desc DRPTB."]
        #[inline(always)]
        pub const fn set_drptb(&mut self, val: u16) {
            self.0 = (self.0 & !(0x03ff << 16usize)) | (((val as u32) & 0x03ff) << 16usize);
        }
    }
    impl Default for Rptb5 {
        #[inline(always)]
        fn default() -> Rptb5 {
            Rptb5(0)
        }
    }
    impl core::fmt::Debug for Rptb5 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Rptb5")
                .field("srptb", &self.srptb())
                .field("drptb", &self.drptb())
                .finish()
        }
    }
    #[doc = "desc SNSEQCTL0."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Snseqctl0(pub u32);
    impl Snseqctl0 {
        #[doc = "desc SOFFSET."]
        #[must_use]
        #[inline(always)]
        pub const fn soffset(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0x000f_ffff;
            val as u32
        }
        #[doc = "desc SOFFSET."]
        #[inline(always)]
        pub const fn set_soffset(&mut self, val: u32) {
            self.0 = (self.0 & !(0x000f_ffff << 0usize)) | (((val as u32) & 0x000f_ffff) << 0usize);
        }
        #[doc = "desc SNSCNT."]
        #[must_use]
        #[inline(always)]
        pub const fn snscnt(&self) -> u16 {
            let val = (self.0 >> 20usize) & 0x0fff;
            val as u16
        }
        #[doc = "desc SNSCNT."]
        #[inline(always)]
        pub const fn set_snscnt(&mut self, val: u16) {
            self.0 = (self.0 & !(0x0fff << 20usize)) | (((val as u32) & 0x0fff) << 20usize);
        }
    }
    impl Default for Snseqctl0 {
        #[inline(always)]
        fn default() -> Snseqctl0 {
            Snseqctl0(0)
        }
    }
    impl core::fmt::Debug for Snseqctl0 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Snseqctl0")
                .field("soffset", &self.soffset())
                .field("snscnt", &self.snscnt())
                .finish()
        }
    }
    #[doc = "desc SNSEQCTL1."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Snseqctl1(pub u32);
    impl Snseqctl1 {
        #[doc = "desc SOFFSET."]
        #[must_use]
        #[inline(always)]
        pub const fn soffset(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0x000f_ffff;
            val as u32
        }
        #[doc = "desc SOFFSET."]
        #[inline(always)]
        pub const fn set_soffset(&mut self, val: u32) {
            self.0 = (self.0 & !(0x000f_ffff << 0usize)) | (((val as u32) & 0x000f_ffff) << 0usize);
        }
        #[doc = "desc SNSCNT."]
        #[must_use]
        #[inline(always)]
        pub const fn snscnt(&self) -> u16 {
            let val = (self.0 >> 20usize) & 0x0fff;
            val as u16
        }
        #[doc = "desc SNSCNT."]
        #[inline(always)]
        pub const fn set_snscnt(&mut self, val: u16) {
            self.0 = (self.0 & !(0x0fff << 20usize)) | (((val as u32) & 0x0fff) << 20usize);
        }
    }
    impl Default for Snseqctl1 {
        #[inline(always)]
        fn default() -> Snseqctl1 {
            Snseqctl1(0)
        }
    }
    impl core::fmt::Debug for Snseqctl1 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Snseqctl1")
                .field("soffset", &self.soffset())
                .field("snscnt", &self.snscnt())
                .finish()
        }
    }
    #[doc = "desc SNSEQCTL2."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Snseqctl2(pub u32);
    impl Snseqctl2 {
        #[doc = "desc SOFFSET."]
        #[must_use]
        #[inline(always)]
        pub const fn soffset(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0x000f_ffff;
            val as u32
        }
        #[doc = "desc SOFFSET."]
        #[inline(always)]
        pub const fn set_soffset(&mut self, val: u32) {
            self.0 = (self.0 & !(0x000f_ffff << 0usize)) | (((val as u32) & 0x000f_ffff) << 0usize);
        }
        #[doc = "desc SNSCNT."]
        #[must_use]
        #[inline(always)]
        pub const fn snscnt(&self) -> u16 {
            let val = (self.0 >> 20usize) & 0x0fff;
            val as u16
        }
        #[doc = "desc SNSCNT."]
        #[inline(always)]
        pub const fn set_snscnt(&mut self, val: u16) {
            self.0 = (self.0 & !(0x0fff << 20usize)) | (((val as u32) & 0x0fff) << 20usize);
        }
    }
    impl Default for Snseqctl2 {
        #[inline(always)]
        fn default() -> Snseqctl2 {
            Snseqctl2(0)
        }
    }
    impl core::fmt::Debug for Snseqctl2 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Snseqctl2")
                .field("soffset", &self.soffset())
                .field("snscnt", &self.snscnt())
                .finish()
        }
    }
    #[doc = "desc SNSEQCTL3."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Snseqctl3(pub u32);
    impl Snseqctl3 {
        #[doc = "desc SOFFSET."]
        #[must_use]
        #[inline(always)]
        pub const fn soffset(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0x000f_ffff;
            val as u32
        }
        #[doc = "desc SOFFSET."]
        #[inline(always)]
        pub const fn set_soffset(&mut self, val: u32) {
            self.0 = (self.0 & !(0x000f_ffff << 0usize)) | (((val as u32) & 0x000f_ffff) << 0usize);
        }
        #[doc = "desc SNSCNT."]
        #[must_use]
        #[inline(always)]
        pub const fn snscnt(&self) -> u16 {
            let val = (self.0 >> 20usize) & 0x0fff;
            val as u16
        }
        #[doc = "desc SNSCNT."]
        #[inline(always)]
        pub const fn set_snscnt(&mut self, val: u16) {
            self.0 = (self.0 & !(0x0fff << 20usize)) | (((val as u32) & 0x0fff) << 20usize);
        }
    }
    impl Default for Snseqctl3 {
        #[inline(always)]
        fn default() -> Snseqctl3 {
            Snseqctl3(0)
        }
    }
    impl core::fmt::Debug for Snseqctl3 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Snseqctl3")
                .field("soffset", &self.soffset())
                .field("snscnt", &self.snscnt())
                .finish()
        }
    }
    #[doc = "desc SNSEQCTL4."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Snseqctl4(pub u32);
    impl Snseqctl4 {
        #[doc = "desc SOFFSET."]
        #[must_use]
        #[inline(always)]
        pub const fn soffset(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0x000f_ffff;
            val as u32
        }
        #[doc = "desc SOFFSET."]
        #[inline(always)]
        pub const fn set_soffset(&mut self, val: u32) {
            self.0 = (self.0 & !(0x000f_ffff << 0usize)) | (((val as u32) & 0x000f_ffff) << 0usize);
        }
        #[doc = "desc SNSCNT."]
        #[must_use]
        #[inline(always)]
        pub const fn snscnt(&self) -> u16 {
            let val = (self.0 >> 20usize) & 0x0fff;
            val as u16
        }
        #[doc = "desc SNSCNT."]
        #[inline(always)]
        pub const fn set_snscnt(&mut self, val: u16) {
            self.0 = (self.0 & !(0x0fff << 20usize)) | (((val as u32) & 0x0fff) << 20usize);
        }
    }
    impl Default for Snseqctl4 {
        #[inline(always)]
        fn default() -> Snseqctl4 {
            Snseqctl4(0)
        }
    }
    impl core::fmt::Debug for Snseqctl4 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Snseqctl4")
                .field("soffset", &self.soffset())
                .field("snscnt", &self.snscnt())
                .finish()
        }
    }
    #[doc = "desc SNSEQCTL5."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Snseqctl5(pub u32);
    impl Snseqctl5 {
        #[doc = "desc SOFFSET."]
        #[must_use]
        #[inline(always)]
        pub const fn soffset(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0x000f_ffff;
            val as u32
        }
        #[doc = "desc SOFFSET."]
        #[inline(always)]
        pub const fn set_soffset(&mut self, val: u32) {
            self.0 = (self.0 & !(0x000f_ffff << 0usize)) | (((val as u32) & 0x000f_ffff) << 0usize);
        }
        #[doc = "desc SNSCNT."]
        #[must_use]
        #[inline(always)]
        pub const fn snscnt(&self) -> u16 {
            let val = (self.0 >> 20usize) & 0x0fff;
            val as u16
        }
        #[doc = "desc SNSCNT."]
        #[inline(always)]
        pub const fn set_snscnt(&mut self, val: u16) {
            self.0 = (self.0 & !(0x0fff << 20usize)) | (((val as u32) & 0x0fff) << 20usize);
        }
    }
    impl Default for Snseqctl5 {
        #[inline(always)]
        fn default() -> Snseqctl5 {
            Snseqctl5(0)
        }
    }
    impl core::fmt::Debug for Snseqctl5 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Snseqctl5")
                .field("soffset", &self.soffset())
                .field("snscnt", &self.snscnt())
                .finish()
        }
    }
    #[doc = "desc SNSEQCTLB0."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Snseqctlb0(pub u32);
    impl Snseqctlb0 {
        #[doc = "desc SNSDIST."]
        #[must_use]
        #[inline(always)]
        pub const fn snsdist(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0x000f_ffff;
            val as u32
        }
        #[doc = "desc SNSDIST."]
        #[inline(always)]
        pub const fn set_snsdist(&mut self, val: u32) {
            self.0 = (self.0 & !(0x000f_ffff << 0usize)) | (((val as u32) & 0x000f_ffff) << 0usize);
        }
        #[doc = "desc SNSCNTB."]
        #[must_use]
        #[inline(always)]
        pub const fn snscntb(&self) -> u16 {
            let val = (self.0 >> 20usize) & 0x0fff;
            val as u16
        }
        #[doc = "desc SNSCNTB."]
        #[inline(always)]
        pub const fn set_snscntb(&mut self, val: u16) {
            self.0 = (self.0 & !(0x0fff << 20usize)) | (((val as u32) & 0x0fff) << 20usize);
        }
    }
    impl Default for Snseqctlb0 {
        #[inline(always)]
        fn default() -> Snseqctlb0 {
            Snseqctlb0(0)
        }
    }
    impl core::fmt::Debug for Snseqctlb0 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Snseqctlb0")
                .field("snsdist", &self.snsdist())
                .field("snscntb", &self.snscntb())
                .finish()
        }
    }
    #[doc = "desc SNSEQCTLB1."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Snseqctlb1(pub u32);
    impl Snseqctlb1 {
        #[doc = "desc SNSDIST."]
        #[must_use]
        #[inline(always)]
        pub const fn snsdist(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0x000f_ffff;
            val as u32
        }
        #[doc = "desc SNSDIST."]
        #[inline(always)]
        pub const fn set_snsdist(&mut self, val: u32) {
            self.0 = (self.0 & !(0x000f_ffff << 0usize)) | (((val as u32) & 0x000f_ffff) << 0usize);
        }
        #[doc = "desc SNSCNTB."]
        #[must_use]
        #[inline(always)]
        pub const fn snscntb(&self) -> u16 {
            let val = (self.0 >> 20usize) & 0x0fff;
            val as u16
        }
        #[doc = "desc SNSCNTB."]
        #[inline(always)]
        pub const fn set_snscntb(&mut self, val: u16) {
            self.0 = (self.0 & !(0x0fff << 20usize)) | (((val as u32) & 0x0fff) << 20usize);
        }
    }
    impl Default for Snseqctlb1 {
        #[inline(always)]
        fn default() -> Snseqctlb1 {
            Snseqctlb1(0)
        }
    }
    impl core::fmt::Debug for Snseqctlb1 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Snseqctlb1")
                .field("snsdist", &self.snsdist())
                .field("snscntb", &self.snscntb())
                .finish()
        }
    }
    #[doc = "desc SNSEQCTLB2."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Snseqctlb2(pub u32);
    impl Snseqctlb2 {
        #[doc = "desc SNSDIST."]
        #[must_use]
        #[inline(always)]
        pub const fn snsdist(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0x000f_ffff;
            val as u32
        }
        #[doc = "desc SNSDIST."]
        #[inline(always)]
        pub const fn set_snsdist(&mut self, val: u32) {
            self.0 = (self.0 & !(0x000f_ffff << 0usize)) | (((val as u32) & 0x000f_ffff) << 0usize);
        }
        #[doc = "desc SNSCNTB."]
        #[must_use]
        #[inline(always)]
        pub const fn snscntb(&self) -> u16 {
            let val = (self.0 >> 20usize) & 0x0fff;
            val as u16
        }
        #[doc = "desc SNSCNTB."]
        #[inline(always)]
        pub const fn set_snscntb(&mut self, val: u16) {
            self.0 = (self.0 & !(0x0fff << 20usize)) | (((val as u32) & 0x0fff) << 20usize);
        }
    }
    impl Default for Snseqctlb2 {
        #[inline(always)]
        fn default() -> Snseqctlb2 {
            Snseqctlb2(0)
        }
    }
    impl core::fmt::Debug for Snseqctlb2 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Snseqctlb2")
                .field("snsdist", &self.snsdist())
                .field("snscntb", &self.snscntb())
                .finish()
        }
    }
    #[doc = "desc SNSEQCTLB3."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Snseqctlb3(pub u32);
    impl Snseqctlb3 {
        #[doc = "desc SNSDIST."]
        #[must_use]
        #[inline(always)]
        pub const fn snsdist(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0x000f_ffff;
            val as u32
        }
        #[doc = "desc SNSDIST."]
        #[inline(always)]
        pub const fn set_snsdist(&mut self, val: u32) {
            self.0 = (self.0 & !(0x000f_ffff << 0usize)) | (((val as u32) & 0x000f_ffff) << 0usize);
        }
        #[doc = "desc SNSCNTB."]
        #[must_use]
        #[inline(always)]
        pub const fn snscntb(&self) -> u16 {
            let val = (self.0 >> 20usize) & 0x0fff;
            val as u16
        }
        #[doc = "desc SNSCNTB."]
        #[inline(always)]
        pub const fn set_snscntb(&mut self, val: u16) {
            self.0 = (self.0 & !(0x0fff << 20usize)) | (((val as u32) & 0x0fff) << 20usize);
        }
    }
    impl Default for Snseqctlb3 {
        #[inline(always)]
        fn default() -> Snseqctlb3 {
            Snseqctlb3(0)
        }
    }
    impl core::fmt::Debug for Snseqctlb3 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Snseqctlb3")
                .field("snsdist", &self.snsdist())
                .field("snscntb", &self.snscntb())
                .finish()
        }
    }
    #[doc = "desc SNSEQCTLB4."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Snseqctlb4(pub u32);
    impl Snseqctlb4 {
        #[doc = "desc SNSDIST."]
        #[must_use]
        #[inline(always)]
        pub const fn snsdist(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0x000f_ffff;
            val as u32
        }
        #[doc = "desc SNSDIST."]
        #[inline(always)]
        pub const fn set_snsdist(&mut self, val: u32) {
            self.0 = (self.0 & !(0x000f_ffff << 0usize)) | (((val as u32) & 0x000f_ffff) << 0usize);
        }
        #[doc = "desc SNSCNTB."]
        #[must_use]
        #[inline(always)]
        pub const fn snscntb(&self) -> u16 {
            let val = (self.0 >> 20usize) & 0x0fff;
            val as u16
        }
        #[doc = "desc SNSCNTB."]
        #[inline(always)]
        pub const fn set_snscntb(&mut self, val: u16) {
            self.0 = (self.0 & !(0x0fff << 20usize)) | (((val as u32) & 0x0fff) << 20usize);
        }
    }
    impl Default for Snseqctlb4 {
        #[inline(always)]
        fn default() -> Snseqctlb4 {
            Snseqctlb4(0)
        }
    }
    impl core::fmt::Debug for Snseqctlb4 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Snseqctlb4")
                .field("snsdist", &self.snsdist())
                .field("snscntb", &self.snscntb())
                .finish()
        }
    }
    #[doc = "desc SNSEQCTLB5."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Snseqctlb5(pub u32);
    impl Snseqctlb5 {
        #[doc = "desc SNSDIST."]
        #[must_use]
        #[inline(always)]
        pub const fn snsdist(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0x000f_ffff;
            val as u32
        }
        #[doc = "desc SNSDIST."]
        #[inline(always)]
        pub const fn set_snsdist(&mut self, val: u32) {
            self.0 = (self.0 & !(0x000f_ffff << 0usize)) | (((val as u32) & 0x000f_ffff) << 0usize);
        }
        #[doc = "desc SNSCNTB."]
        #[must_use]
        #[inline(always)]
        pub const fn snscntb(&self) -> u16 {
            let val = (self.0 >> 20usize) & 0x0fff;
            val as u16
        }
        #[doc = "desc SNSCNTB."]
        #[inline(always)]
        pub const fn set_snscntb(&mut self, val: u16) {
            self.0 = (self.0 & !(0x0fff << 20usize)) | (((val as u32) & 0x0fff) << 20usize);
        }
    }
    impl Default for Snseqctlb5 {
        #[inline(always)]
        fn default() -> Snseqctlb5 {
            Snseqctlb5(0)
        }
    }
    impl core::fmt::Debug for Snseqctlb5 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Snseqctlb5")
                .field("snsdist", &self.snsdist())
                .field("snscntb", &self.snscntb())
                .finish()
        }
    }
    #[doc = "desc SWREQ."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Swreq(pub u32);
    impl Swreq {
        #[doc = "desc SWREQ."]
        #[must_use]
        #[inline(always)]
        pub const fn swreq(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[doc = "desc SWREQ."]
        #[inline(always)]
        pub const fn set_swreq(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
        }
        #[doc = "desc SWRCFGREQ."]
        #[must_use]
        #[inline(always)]
        pub const fn swrcfgreq(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "desc SWRCFGREQ."]
        #[inline(always)]
        pub const fn set_swrcfgreq(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
        #[doc = "desc SWREQWP."]
        #[must_use]
        #[inline(always)]
        pub const fn swreqwp(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0xff;
            val as u8
        }
        #[doc = "desc SWREQWP."]
        #[inline(always)]
        pub const fn set_swreqwp(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 16usize)) | (((val as u32) & 0xff) << 16usize);
        }
        #[doc = "desc SWRCFGWP."]
        #[must_use]
        #[inline(always)]
        pub const fn swrcfgwp(&self) -> u8 {
            let val = (self.0 >> 24usize) & 0xff;
            val as u8
        }
        #[doc = "desc SWRCFGWP."]
        #[inline(always)]
        pub const fn set_swrcfgwp(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 24usize)) | (((val as u32) & 0xff) << 24usize);
        }
    }
    impl Default for Swreq {
        #[inline(always)]
        fn default() -> Swreq {
            Swreq(0)
        }
    }
    impl core::fmt::Debug for Swreq {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Swreq")
                .field("swreq", &self.swreq())
                .field("swrcfgreq", &self.swrcfgreq())
                .field("swreqwp", &self.swreqwp())
                .field("swrcfgwp", &self.swrcfgwp())
                .finish()
        }
    }
}
