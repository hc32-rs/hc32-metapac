#![allow(clippy::missing_safety_doc)]
#![allow(clippy::identity_op)]
#![allow(clippy::unnecessary_cast)]
#![allow(clippy::erasing_op)]

#[doc = "desc ETH."]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Eth {
    ptr: *mut u8,
}
unsafe impl Send for Eth {}
unsafe impl Sync for Eth {}
impl Eth {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "desc MAC_IFCONFR."]
    #[inline(always)]
    pub const fn mac_ifconfr(self) -> crate::common::Reg<regs::MacIfconfr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x5410usize) as _) }
    }
    #[doc = "desc MAC_CONFIGR."]
    #[inline(always)]
    pub const fn mac_configr(self) -> crate::common::Reg<regs::MacConfigr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0001_0000usize) as _) }
    }
    #[doc = "desc MAC_FLTCTLR."]
    #[inline(always)]
    pub const fn mac_fltctlr(self) -> crate::common::Reg<regs::MacFltctlr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0001_0004usize) as _) }
    }
    #[doc = "desc MAC_HASHTHR."]
    #[inline(always)]
    pub const fn mac_hashthr(self) -> crate::common::Reg<regs::MacHashthr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0001_0008usize) as _) }
    }
    #[doc = "desc MAC_HASHTLR."]
    #[inline(always)]
    pub const fn mac_hashtlr(self) -> crate::common::Reg<regs::MacHashtlr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0001_000cusize) as _) }
    }
    #[doc = "desc MAC_SMIADDR."]
    #[inline(always)]
    pub const fn mac_smiaddr(self) -> crate::common::Reg<regs::MacSmiaddr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0001_0010usize) as _) }
    }
    #[doc = "desc MAC_SMIDATR."]
    #[inline(always)]
    pub const fn mac_smidatr(self) -> crate::common::Reg<regs::MacSmidatr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0001_0014usize) as _) }
    }
    #[doc = "desc MAC_FLOCTLR."]
    #[inline(always)]
    pub const fn mac_floctlr(self) -> crate::common::Reg<regs::MacFloctlr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0001_0018usize) as _) }
    }
    #[doc = "desc MAC_VTAFLTR."]
    #[inline(always)]
    pub const fn mac_vtafltr(self) -> crate::common::Reg<regs::MacVtafltr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0001_001cusize) as _) }
    }
    #[doc = "desc MAC_MACSTSR."]
    #[inline(always)]
    pub const fn mac_macstsr(self) -> crate::common::Reg<regs::MacMacstsr, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0001_0024usize) as _) }
    }
    #[doc = "desc MAC_RTWKFFR."]
    #[inline(always)]
    pub const fn mac_rtwkffr(self) -> crate::common::Reg<regs::MacRtwkffr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0001_0028usize) as _) }
    }
    #[doc = "desc MAC_PMTCTLR."]
    #[inline(always)]
    pub const fn mac_pmtctlr(self) -> crate::common::Reg<regs::MacPmtctlr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0001_002cusize) as _) }
    }
    #[doc = "desc MAC_INTSTSR."]
    #[inline(always)]
    pub const fn mac_intstsr(self) -> crate::common::Reg<regs::MacIntstsr, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0001_0038usize) as _) }
    }
    #[doc = "desc MAC_INTMSKR."]
    #[inline(always)]
    pub const fn mac_intmskr(self) -> crate::common::Reg<regs::MacIntmskr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0001_003cusize) as _) }
    }
    #[doc = "desc MAC_MACADHR0."]
    #[inline(always)]
    pub const fn mac_macadhr0(self) -> crate::common::Reg<regs::MacMacadhr0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0001_0040usize) as _) }
    }
    #[doc = "desc MAC_MACADLR0."]
    #[inline(always)]
    pub const fn mac_macadlr0(self) -> crate::common::Reg<regs::MacMacadlr0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0001_0044usize) as _) }
    }
    #[doc = "desc MAC_MACADHR1."]
    #[inline(always)]
    pub const fn mac_macadhr1(self) -> crate::common::Reg<regs::MacMacadhr1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0001_0048usize) as _) }
    }
    #[doc = "desc MAC_MACADLR1."]
    #[inline(always)]
    pub const fn mac_macadlr1(self) -> crate::common::Reg<regs::MacMacadlr1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0001_004cusize) as _) }
    }
    #[doc = "desc MAC_MACADHR2."]
    #[inline(always)]
    pub const fn mac_macadhr2(self) -> crate::common::Reg<regs::MacMacadhr2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0001_0050usize) as _) }
    }
    #[doc = "desc MAC_MACADLR2."]
    #[inline(always)]
    pub const fn mac_macadlr2(self) -> crate::common::Reg<regs::MacMacadlr2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0001_0054usize) as _) }
    }
    #[doc = "desc MAC_MACADHR3."]
    #[inline(always)]
    pub const fn mac_macadhr3(self) -> crate::common::Reg<regs::MacMacadhr3, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0001_0058usize) as _) }
    }
    #[doc = "desc MAC_MACADLR3."]
    #[inline(always)]
    pub const fn mac_macadlr3(self) -> crate::common::Reg<regs::MacMacadlr3, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0001_005cusize) as _) }
    }
    #[doc = "desc MAC_MACADHR4."]
    #[inline(always)]
    pub const fn mac_macadhr4(self) -> crate::common::Reg<regs::MacMacadhr4, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0001_0060usize) as _) }
    }
    #[doc = "desc MAC_MACADLR4."]
    #[inline(always)]
    pub const fn mac_macadlr4(self) -> crate::common::Reg<regs::MacMacadlr4, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0001_0064usize) as _) }
    }
    #[doc = "desc MMC_MMCCTLR."]
    #[inline(always)]
    pub const fn mmc_mmcctlr(self) -> crate::common::Reg<regs::MmcMmcctlr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0001_0100usize) as _) }
    }
    #[doc = "desc MMC_REVSTSR."]
    #[inline(always)]
    pub const fn mmc_revstsr(self) -> crate::common::Reg<regs::MmcRevstsr, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0001_0104usize) as _) }
    }
    #[doc = "desc MMC_TRSSTSR."]
    #[inline(always)]
    pub const fn mmc_trsstsr(self) -> crate::common::Reg<regs::MmcTrsstsr, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0001_0108usize) as _) }
    }
    #[doc = "desc MMC_RITCTLR."]
    #[inline(always)]
    pub const fn mmc_ritctlr(self) -> crate::common::Reg<regs::MmcRitctlr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0001_010cusize) as _) }
    }
    #[doc = "desc MMC_TITCTLR."]
    #[inline(always)]
    pub const fn mmc_titctlr(self) -> crate::common::Reg<regs::MmcTitctlr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0001_0110usize) as _) }
    }
    #[doc = "desc MMC_TXBRGFR."]
    #[inline(always)]
    pub const fn mmc_txbrgfr(self) -> crate::common::Reg<regs::MmcTxbrgfr, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0001_011cusize) as _) }
    }
    #[doc = "desc MMC_TXMUGFR."]
    #[inline(always)]
    pub const fn mmc_txmugfr(self) -> crate::common::Reg<regs::MmcTxmugfr, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0001_0120usize) as _) }
    }
    #[doc = "desc MMC_TXDEEFR."]
    #[inline(always)]
    pub const fn mmc_txdeefr(self) -> crate::common::Reg<regs::MmcTxdeefr, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0001_0154usize) as _) }
    }
    #[doc = "desc MMC_TXLCEFR."]
    #[inline(always)]
    pub const fn mmc_txlcefr(self) -> crate::common::Reg<regs::MmcTxlcefr, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0001_0158usize) as _) }
    }
    #[doc = "desc MMC_TXECEFR."]
    #[inline(always)]
    pub const fn mmc_txecefr(self) -> crate::common::Reg<regs::MmcTxecefr, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0001_015cusize) as _) }
    }
    #[doc = "desc MMC_TXCAEFR."]
    #[inline(always)]
    pub const fn mmc_txcaefr(self) -> crate::common::Reg<regs::MmcTxcaefr, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0001_0160usize) as _) }
    }
    #[doc = "desc MMC_TXUNGFR."]
    #[inline(always)]
    pub const fn mmc_txungfr(self) -> crate::common::Reg<regs::MmcTxungfr, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0001_0168usize) as _) }
    }
    #[doc = "desc MMC_TXEDEFR."]
    #[inline(always)]
    pub const fn mmc_txedefr(self) -> crate::common::Reg<regs::MmcTxedefr, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0001_016cusize) as _) }
    }
    #[doc = "desc MMC_RXBRGFR."]
    #[inline(always)]
    pub const fn mmc_rxbrgfr(self) -> crate::common::Reg<regs::MmcRxbrgfr, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0001_018cusize) as _) }
    }
    #[doc = "desc MMC_RXMUGFR."]
    #[inline(always)]
    pub const fn mmc_rxmugfr(self) -> crate::common::Reg<regs::MmcRxmugfr, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0001_0190usize) as _) }
    }
    #[doc = "desc MMC_RXCREFR."]
    #[inline(always)]
    pub const fn mmc_rxcrefr(self) -> crate::common::Reg<regs::MmcRxcrefr, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0001_0194usize) as _) }
    }
    #[doc = "desc MMC_RXALEFR."]
    #[inline(always)]
    pub const fn mmc_rxalefr(self) -> crate::common::Reg<regs::MmcRxalefr, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0001_0198usize) as _) }
    }
    #[doc = "desc MMC_RXRUEFR."]
    #[inline(always)]
    pub const fn mmc_rxruefr(self) -> crate::common::Reg<regs::MmcRxruefr, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0001_019cusize) as _) }
    }
    #[doc = "desc MMC_RXUNGFR."]
    #[inline(always)]
    pub const fn mmc_rxungfr(self) -> crate::common::Reg<regs::MmcRxungfr, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0001_01c4usize) as _) }
    }
    #[doc = "desc MMC_RXLEEFR."]
    #[inline(always)]
    pub const fn mmc_rxleefr(self) -> crate::common::Reg<regs::MmcRxleefr, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0001_01c8usize) as _) }
    }
    #[doc = "desc MMC_RXOREFR."]
    #[inline(always)]
    pub const fn mmc_rxorefr(self) -> crate::common::Reg<regs::MmcRxorefr, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0001_01ccusize) as _) }
    }
    #[doc = "desc MAC_L34CTLR."]
    #[inline(always)]
    pub const fn mac_l34ctlr(self) -> crate::common::Reg<regs::MacL34ctlr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0001_0400usize) as _) }
    }
    #[doc = "desc MAC_L4PORTR."]
    #[inline(always)]
    pub const fn mac_l4portr(self) -> crate::common::Reg<regs::MacL4portr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0001_0404usize) as _) }
    }
    #[doc = "desc MAC_L3ADDRR0."]
    #[inline(always)]
    pub const fn mac_l3addrr0(self) -> crate::common::Reg<regs::MacL3addrr0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0001_0410usize) as _) }
    }
    #[doc = "desc MAC_L3ADDRR1."]
    #[inline(always)]
    pub const fn mac_l3addrr1(self) -> crate::common::Reg<regs::MacL3addrr1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0001_0414usize) as _) }
    }
    #[doc = "desc MAC_L3ADDRR2."]
    #[inline(always)]
    pub const fn mac_l3addrr2(self) -> crate::common::Reg<regs::MacL3addrr2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0001_0418usize) as _) }
    }
    #[doc = "desc MAC_L3ADDRR3."]
    #[inline(always)]
    pub const fn mac_l3addrr3(self) -> crate::common::Reg<regs::MacL3addrr3, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0001_041cusize) as _) }
    }
    #[doc = "desc MAC_VTACTLR."]
    #[inline(always)]
    pub const fn mac_vtactlr(self) -> crate::common::Reg<regs::MacVtactlr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0001_0584usize) as _) }
    }
    #[doc = "desc MAC_VLAHTBR."]
    #[inline(always)]
    pub const fn mac_vlahtbr(self) -> crate::common::Reg<regs::MacVlahtbr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0001_0588usize) as _) }
    }
    #[doc = "desc PTP_TSPCTLR."]
    #[inline(always)]
    pub const fn ptp_tspctlr(self) -> crate::common::Reg<regs::PtpTspctlr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0001_0700usize) as _) }
    }
    #[doc = "desc PTP_TSPNSAR."]
    #[inline(always)]
    pub const fn ptp_tspnsar(self) -> crate::common::Reg<regs::PtpTspnsar, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0001_0704usize) as _) }
    }
    #[doc = "desc PTP_TMSSECR."]
    #[inline(always)]
    pub const fn ptp_tmssecr(self) -> crate::common::Reg<regs::PtpTmssecr, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0001_0708usize) as _) }
    }
    #[doc = "desc PTP_TMSNSER."]
    #[inline(always)]
    pub const fn ptp_tmsnser(self) -> crate::common::Reg<regs::PtpTmsnser, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0001_070cusize) as _) }
    }
    #[doc = "desc PTP_TMUSECR."]
    #[inline(always)]
    pub const fn ptp_tmusecr(self) -> crate::common::Reg<regs::PtpTmusecr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0001_0710usize) as _) }
    }
    #[doc = "desc PTP_TMUNSER."]
    #[inline(always)]
    pub const fn ptp_tmunser(self) -> crate::common::Reg<regs::PtpTmunser, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0001_0714usize) as _) }
    }
    #[doc = "desc PTP_TSPADDR."]
    #[inline(always)]
    pub const fn ptp_tspaddr(self) -> crate::common::Reg<regs::PtpTspaddr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0001_0718usize) as _) }
    }
    #[doc = "desc PTP_TMTSECR0."]
    #[inline(always)]
    pub const fn ptp_tmtsecr0(self) -> crate::common::Reg<regs::PtpTmtsecr0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0001_071cusize) as _) }
    }
    #[doc = "desc PTP_TMTNSER0."]
    #[inline(always)]
    pub const fn ptp_tmtnser0(self) -> crate::common::Reg<regs::PtpTmtnser0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0001_0720usize) as _) }
    }
    #[doc = "desc PTP_TSPSTSR."]
    #[inline(always)]
    pub const fn ptp_tspstsr(self) -> crate::common::Reg<regs::PtpTspstsr, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0001_0728usize) as _) }
    }
    #[doc = "desc PTP_PPSCTLR."]
    #[inline(always)]
    pub const fn ptp_ppsctlr(self) -> crate::common::Reg<regs::PtpPpsctlr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0001_072cusize) as _) }
    }
    #[doc = "desc PTP_TMTSECR1."]
    #[inline(always)]
    pub const fn ptp_tmtsecr1(self) -> crate::common::Reg<regs::PtpTmtsecr1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0001_0780usize) as _) }
    }
    #[doc = "desc PTP_TMTNSER1."]
    #[inline(always)]
    pub const fn ptp_tmtnser1(self) -> crate::common::Reg<regs::PtpTmtnser1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0001_0784usize) as _) }
    }
    #[doc = "desc DMA_BUSMODR."]
    #[inline(always)]
    pub const fn dma_busmodr(self) -> crate::common::Reg<regs::DmaBusmodr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0001_1000usize) as _) }
    }
    #[doc = "desc DMA_TXPOLLR."]
    #[inline(always)]
    pub const fn dma_txpollr(self) -> crate::common::Reg<regs::DmaTxpollr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0001_1004usize) as _) }
    }
    #[doc = "desc DMA_RXPOLLR."]
    #[inline(always)]
    pub const fn dma_rxpollr(self) -> crate::common::Reg<regs::DmaRxpollr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0001_1008usize) as _) }
    }
    #[doc = "desc DMA_RXDLADR."]
    #[inline(always)]
    pub const fn dma_rxdladr(self) -> crate::common::Reg<regs::DmaRxdladr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0001_100cusize) as _) }
    }
    #[doc = "desc DMA_TXDLADR."]
    #[inline(always)]
    pub const fn dma_txdladr(self) -> crate::common::Reg<regs::DmaTxdladr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0001_1010usize) as _) }
    }
    #[doc = "desc DMA_DMASTSR."]
    #[inline(always)]
    pub const fn dma_dmastsr(self) -> crate::common::Reg<regs::DmaDmastsr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0001_1014usize) as _) }
    }
    #[doc = "desc DMA_OPRMODR."]
    #[inline(always)]
    pub const fn dma_oprmodr(self) -> crate::common::Reg<regs::DmaOprmodr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0001_1018usize) as _) }
    }
    #[doc = "desc DMA_INTENAR."]
    #[inline(always)]
    pub const fn dma_intenar(self) -> crate::common::Reg<regs::DmaIntenar, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0001_101cusize) as _) }
    }
    #[doc = "desc DMA_RFRCNTR."]
    #[inline(always)]
    pub const fn dma_rfrcntr(self) -> crate::common::Reg<regs::DmaRfrcntr, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0001_1020usize) as _) }
    }
    #[doc = "desc DMA_REVWDTR."]
    #[inline(always)]
    pub const fn dma_revwdtr(self) -> crate::common::Reg<regs::DmaRevwdtr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0001_1024usize) as _) }
    }
    #[doc = "desc DMA_CHTXDER."]
    #[inline(always)]
    pub const fn dma_chtxder(self) -> crate::common::Reg<regs::DmaChtxder, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0001_1048usize) as _) }
    }
    #[doc = "desc DMA_CHRXDER."]
    #[inline(always)]
    pub const fn dma_chrxder(self) -> crate::common::Reg<regs::DmaChrxder, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0001_104cusize) as _) }
    }
    #[doc = "desc DMA_CHTXBFR."]
    #[inline(always)]
    pub const fn dma_chtxbfr(self) -> crate::common::Reg<regs::DmaChtxbfr, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0001_1050usize) as _) }
    }
    #[doc = "desc DMA_CHRXBFR."]
    #[inline(always)]
    pub const fn dma_chrxbfr(self) -> crate::common::Reg<regs::DmaChrxbfr, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0001_1054usize) as _) }
    }
}
pub mod regs {
    #[doc = "desc DMA_BUSMODR."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct DmaBusmodr(pub u32);
    impl DmaBusmodr {
        #[doc = "desc SWR."]
        #[must_use]
        #[inline(always)]
        pub const fn swr(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "desc SWR."]
        #[inline(always)]
        pub const fn set_swr(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "desc DMAA."]
        #[must_use]
        #[inline(always)]
        pub const fn dmaa(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "desc DMAA."]
        #[inline(always)]
        pub const fn set_dmaa(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "desc DSL."]
        #[must_use]
        #[inline(always)]
        pub const fn dsl(&self) -> u8 {
            let val = (self.0 >> 2usize) & 0x1f;
            val as u8
        }
        #[doc = "desc DSL."]
        #[inline(always)]
        pub const fn set_dsl(&mut self, val: u8) {
            self.0 = (self.0 & !(0x1f << 2usize)) | (((val as u32) & 0x1f) << 2usize);
        }
        #[doc = "desc DSEN."]
        #[must_use]
        #[inline(always)]
        pub const fn dsen(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "desc DSEN."]
        #[inline(always)]
        pub const fn set_dsen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "desc TPBL."]
        #[must_use]
        #[inline(always)]
        pub const fn tpbl(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0x3f;
            val as u8
        }
        #[doc = "desc TPBL."]
        #[inline(always)]
        pub const fn set_tpbl(&mut self, val: u8) {
            self.0 = (self.0 & !(0x3f << 8usize)) | (((val as u32) & 0x3f) << 8usize);
        }
        #[doc = "desc PRAT."]
        #[must_use]
        #[inline(always)]
        pub const fn prat(&self) -> u8 {
            let val = (self.0 >> 14usize) & 0x03;
            val as u8
        }
        #[doc = "desc PRAT."]
        #[inline(always)]
        pub const fn set_prat(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 14usize)) | (((val as u32) & 0x03) << 14usize);
        }
        #[doc = "desc FBST."]
        #[must_use]
        #[inline(always)]
        pub const fn fbst(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "desc FBST."]
        #[inline(always)]
        pub const fn set_fbst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "desc RPBL."]
        #[must_use]
        #[inline(always)]
        pub const fn rpbl(&self) -> u8 {
            let val = (self.0 >> 17usize) & 0x3f;
            val as u8
        }
        #[doc = "desc RPBL."]
        #[inline(always)]
        pub const fn set_rpbl(&mut self, val: u8) {
            self.0 = (self.0 & !(0x3f << 17usize)) | (((val as u32) & 0x3f) << 17usize);
        }
        #[doc = "desc SPBL."]
        #[must_use]
        #[inline(always)]
        pub const fn spbl(&self) -> bool {
            let val = (self.0 >> 23usize) & 0x01;
            val != 0
        }
        #[doc = "desc SPBL."]
        #[inline(always)]
        pub const fn set_spbl(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
        }
        #[doc = "desc M8PBL."]
        #[must_use]
        #[inline(always)]
        pub const fn m8pbl(&self) -> bool {
            let val = (self.0 >> 24usize) & 0x01;
            val != 0
        }
        #[doc = "desc M8PBL."]
        #[inline(always)]
        pub const fn set_m8pbl(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
        }
        #[doc = "desc AAL."]
        #[must_use]
        #[inline(always)]
        pub const fn aal(&self) -> bool {
            let val = (self.0 >> 25usize) & 0x01;
            val != 0
        }
        #[doc = "desc AAL."]
        #[inline(always)]
        pub const fn set_aal(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
        }
        #[doc = "desc MBST."]
        #[must_use]
        #[inline(always)]
        pub const fn mbst(&self) -> bool {
            let val = (self.0 >> 26usize) & 0x01;
            val != 0
        }
        #[doc = "desc MBST."]
        #[inline(always)]
        pub const fn set_mbst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
        }
        #[doc = "desc TXPR."]
        #[must_use]
        #[inline(always)]
        pub const fn txpr(&self) -> bool {
            let val = (self.0 >> 27usize) & 0x01;
            val != 0
        }
        #[doc = "desc TXPR."]
        #[inline(always)]
        pub const fn set_txpr(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
        }
    }
    impl Default for DmaBusmodr {
        #[inline(always)]
        fn default() -> DmaBusmodr {
            DmaBusmodr(0)
        }
    }
    impl core::fmt::Debug for DmaBusmodr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("DmaBusmodr")
                .field("swr", &self.swr())
                .field("dmaa", &self.dmaa())
                .field("dsl", &self.dsl())
                .field("dsen", &self.dsen())
                .field("tpbl", &self.tpbl())
                .field("prat", &self.prat())
                .field("fbst", &self.fbst())
                .field("rpbl", &self.rpbl())
                .field("spbl", &self.spbl())
                .field("m8pbl", &self.m8pbl())
                .field("aal", &self.aal())
                .field("mbst", &self.mbst())
                .field("txpr", &self.txpr())
                .finish()
        }
    }
    #[doc = "desc DMA_CHRXBFR."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct DmaChrxbfr(pub u32);
    impl DmaChrxbfr {
        #[doc = "desc CHRXBF."]
        #[must_use]
        #[inline(always)]
        pub const fn chrxbf(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "desc CHRXBF."]
        #[inline(always)]
        pub const fn set_chrxbf(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for DmaChrxbfr {
        #[inline(always)]
        fn default() -> DmaChrxbfr {
            DmaChrxbfr(0)
        }
    }
    impl core::fmt::Debug for DmaChrxbfr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("DmaChrxbfr")
                .field("chrxbf", &self.chrxbf())
                .finish()
        }
    }
    #[doc = "desc DMA_CHRXDER."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct DmaChrxder(pub u32);
    impl DmaChrxder {
        #[doc = "desc CHRXDE."]
        #[must_use]
        #[inline(always)]
        pub const fn chrxde(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "desc CHRXDE."]
        #[inline(always)]
        pub const fn set_chrxde(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for DmaChrxder {
        #[inline(always)]
        fn default() -> DmaChrxder {
            DmaChrxder(0)
        }
    }
    impl core::fmt::Debug for DmaChrxder {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("DmaChrxder")
                .field("chrxde", &self.chrxde())
                .finish()
        }
    }
    #[doc = "desc DMA_CHTXBFR."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct DmaChtxbfr(pub u32);
    impl DmaChtxbfr {
        #[doc = "desc CHTXBF."]
        #[must_use]
        #[inline(always)]
        pub const fn chtxbf(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "desc CHTXBF."]
        #[inline(always)]
        pub const fn set_chtxbf(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for DmaChtxbfr {
        #[inline(always)]
        fn default() -> DmaChtxbfr {
            DmaChtxbfr(0)
        }
    }
    impl core::fmt::Debug for DmaChtxbfr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("DmaChtxbfr")
                .field("chtxbf", &self.chtxbf())
                .finish()
        }
    }
    #[doc = "desc DMA_CHTXDER."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct DmaChtxder(pub u32);
    impl DmaChtxder {
        #[doc = "desc CHTXDE."]
        #[must_use]
        #[inline(always)]
        pub const fn chtxde(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "desc CHTXDE."]
        #[inline(always)]
        pub const fn set_chtxde(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for DmaChtxder {
        #[inline(always)]
        fn default() -> DmaChtxder {
            DmaChtxder(0)
        }
    }
    impl core::fmt::Debug for DmaChtxder {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("DmaChtxder")
                .field("chtxde", &self.chtxde())
                .finish()
        }
    }
    #[doc = "desc DMA_DMASTSR."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct DmaDmastsr(pub u32);
    impl DmaDmastsr {
        #[doc = "desc TIS."]
        #[must_use]
        #[inline(always)]
        pub const fn tis(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "desc TIS."]
        #[inline(always)]
        pub const fn set_tis(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "desc TSS."]
        #[must_use]
        #[inline(always)]
        pub const fn tss(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "desc TSS."]
        #[inline(always)]
        pub const fn set_tss(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "desc TUS."]
        #[must_use]
        #[inline(always)]
        pub const fn tus(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "desc TUS."]
        #[inline(always)]
        pub const fn set_tus(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "desc TJS."]
        #[must_use]
        #[inline(always)]
        pub const fn tjs(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "desc TJS."]
        #[inline(always)]
        pub const fn set_tjs(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "desc OVS."]
        #[must_use]
        #[inline(always)]
        pub const fn ovs(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "desc OVS."]
        #[inline(always)]
        pub const fn set_ovs(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "desc UNS."]
        #[must_use]
        #[inline(always)]
        pub const fn uns(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "desc UNS."]
        #[inline(always)]
        pub const fn set_uns(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "desc RIS."]
        #[must_use]
        #[inline(always)]
        pub const fn ris(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "desc RIS."]
        #[inline(always)]
        pub const fn set_ris(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "desc RUS."]
        #[must_use]
        #[inline(always)]
        pub const fn rus(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "desc RUS."]
        #[inline(always)]
        pub const fn set_rus(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "desc RSS."]
        #[must_use]
        #[inline(always)]
        pub const fn rss(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "desc RSS."]
        #[inline(always)]
        pub const fn set_rss(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "desc RWS."]
        #[must_use]
        #[inline(always)]
        pub const fn rws(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "desc RWS."]
        #[inline(always)]
        pub const fn set_rws(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "desc ETS."]
        #[must_use]
        #[inline(always)]
        pub const fn ets(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "desc ETS."]
        #[inline(always)]
        pub const fn set_ets(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[doc = "desc FBS."]
        #[must_use]
        #[inline(always)]
        pub const fn fbs(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[doc = "desc FBS."]
        #[inline(always)]
        pub const fn set_fbs(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[doc = "desc ERS."]
        #[must_use]
        #[inline(always)]
        pub const fn ers(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "desc ERS."]
        #[inline(always)]
        pub const fn set_ers(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[doc = "desc AIS."]
        #[must_use]
        #[inline(always)]
        pub const fn ais(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "desc AIS."]
        #[inline(always)]
        pub const fn set_ais(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
        #[doc = "desc NIS."]
        #[must_use]
        #[inline(always)]
        pub const fn nis(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "desc NIS."]
        #[inline(always)]
        pub const fn set_nis(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "desc RSTS."]
        #[must_use]
        #[inline(always)]
        pub const fn rsts(&self) -> u8 {
            let val = (self.0 >> 17usize) & 0x07;
            val as u8
        }
        #[doc = "desc RSTS."]
        #[inline(always)]
        pub const fn set_rsts(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 17usize)) | (((val as u32) & 0x07) << 17usize);
        }
        #[doc = "desc TSTS."]
        #[must_use]
        #[inline(always)]
        pub const fn tsts(&self) -> u8 {
            let val = (self.0 >> 20usize) & 0x07;
            val as u8
        }
        #[doc = "desc TSTS."]
        #[inline(always)]
        pub const fn set_tsts(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 20usize)) | (((val as u32) & 0x07) << 20usize);
        }
        #[doc = "desc EBUS."]
        #[must_use]
        #[inline(always)]
        pub const fn ebus(&self) -> u8 {
            let val = (self.0 >> 23usize) & 0x07;
            val as u8
        }
        #[doc = "desc EBUS."]
        #[inline(always)]
        pub const fn set_ebus(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 23usize)) | (((val as u32) & 0x07) << 23usize);
        }
        #[doc = "desc MMCS."]
        #[must_use]
        #[inline(always)]
        pub const fn mmcs(&self) -> bool {
            let val = (self.0 >> 27usize) & 0x01;
            val != 0
        }
        #[doc = "desc MMCS."]
        #[inline(always)]
        pub const fn set_mmcs(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
        }
        #[doc = "desc PMTS."]
        #[must_use]
        #[inline(always)]
        pub const fn pmts(&self) -> bool {
            let val = (self.0 >> 28usize) & 0x01;
            val != 0
        }
        #[doc = "desc PMTS."]
        #[inline(always)]
        pub const fn set_pmts(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
        }
        #[doc = "desc PTPS."]
        #[must_use]
        #[inline(always)]
        pub const fn ptps(&self) -> bool {
            let val = (self.0 >> 29usize) & 0x01;
            val != 0
        }
        #[doc = "desc PTPS."]
        #[inline(always)]
        pub const fn set_ptps(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
        }
    }
    impl Default for DmaDmastsr {
        #[inline(always)]
        fn default() -> DmaDmastsr {
            DmaDmastsr(0)
        }
    }
    impl core::fmt::Debug for DmaDmastsr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("DmaDmastsr")
                .field("tis", &self.tis())
                .field("tss", &self.tss())
                .field("tus", &self.tus())
                .field("tjs", &self.tjs())
                .field("ovs", &self.ovs())
                .field("uns", &self.uns())
                .field("ris", &self.ris())
                .field("rus", &self.rus())
                .field("rss", &self.rss())
                .field("rws", &self.rws())
                .field("ets", &self.ets())
                .field("fbs", &self.fbs())
                .field("ers", &self.ers())
                .field("ais", &self.ais())
                .field("nis", &self.nis())
                .field("rsts", &self.rsts())
                .field("tsts", &self.tsts())
                .field("ebus", &self.ebus())
                .field("mmcs", &self.mmcs())
                .field("pmts", &self.pmts())
                .field("ptps", &self.ptps())
                .finish()
        }
    }
    #[doc = "desc DMA_INTENAR."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct DmaIntenar(pub u32);
    impl DmaIntenar {
        #[doc = "desc TIE."]
        #[must_use]
        #[inline(always)]
        pub const fn tie(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "desc TIE."]
        #[inline(always)]
        pub const fn set_tie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "desc TSE."]
        #[must_use]
        #[inline(always)]
        pub const fn tse(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "desc TSE."]
        #[inline(always)]
        pub const fn set_tse(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "desc TUE."]
        #[must_use]
        #[inline(always)]
        pub const fn tue(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "desc TUE."]
        #[inline(always)]
        pub const fn set_tue(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "desc TJE."]
        #[must_use]
        #[inline(always)]
        pub const fn tje(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "desc TJE."]
        #[inline(always)]
        pub const fn set_tje(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "desc OVE."]
        #[must_use]
        #[inline(always)]
        pub const fn ove(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "desc OVE."]
        #[inline(always)]
        pub const fn set_ove(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "desc UNE."]
        #[must_use]
        #[inline(always)]
        pub const fn une(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "desc UNE."]
        #[inline(always)]
        pub const fn set_une(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "desc RIE."]
        #[must_use]
        #[inline(always)]
        pub const fn rie(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "desc RIE."]
        #[inline(always)]
        pub const fn set_rie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "desc RUE."]
        #[must_use]
        #[inline(always)]
        pub const fn rue(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "desc RUE."]
        #[inline(always)]
        pub const fn set_rue(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "desc RSE."]
        #[must_use]
        #[inline(always)]
        pub const fn rse(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "desc RSE."]
        #[inline(always)]
        pub const fn set_rse(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "desc RWE."]
        #[must_use]
        #[inline(always)]
        pub const fn rwe(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "desc RWE."]
        #[inline(always)]
        pub const fn set_rwe(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "desc ETE."]
        #[must_use]
        #[inline(always)]
        pub const fn ete(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "desc ETE."]
        #[inline(always)]
        pub const fn set_ete(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[doc = "desc FBE."]
        #[must_use]
        #[inline(always)]
        pub const fn fbe(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[doc = "desc FBE."]
        #[inline(always)]
        pub const fn set_fbe(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[doc = "desc ERE."]
        #[must_use]
        #[inline(always)]
        pub const fn ere(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "desc ERE."]
        #[inline(always)]
        pub const fn set_ere(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[doc = "desc AIE."]
        #[must_use]
        #[inline(always)]
        pub const fn aie(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "desc AIE."]
        #[inline(always)]
        pub const fn set_aie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
        #[doc = "desc NIE."]
        #[must_use]
        #[inline(always)]
        pub const fn nie(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "desc NIE."]
        #[inline(always)]
        pub const fn set_nie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
    }
    impl Default for DmaIntenar {
        #[inline(always)]
        fn default() -> DmaIntenar {
            DmaIntenar(0)
        }
    }
    impl core::fmt::Debug for DmaIntenar {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("DmaIntenar")
                .field("tie", &self.tie())
                .field("tse", &self.tse())
                .field("tue", &self.tue())
                .field("tje", &self.tje())
                .field("ove", &self.ove())
                .field("une", &self.une())
                .field("rie", &self.rie())
                .field("rue", &self.rue())
                .field("rse", &self.rse())
                .field("rwe", &self.rwe())
                .field("ete", &self.ete())
                .field("fbe", &self.fbe())
                .field("ere", &self.ere())
                .field("aie", &self.aie())
                .field("nie", &self.nie())
                .finish()
        }
    }
    #[doc = "desc DMA_OPRMODR."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct DmaOprmodr(pub u32);
    impl DmaOprmodr {
        #[doc = "desc STR."]
        #[must_use]
        #[inline(always)]
        pub const fn str(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "desc STR."]
        #[inline(always)]
        pub const fn set_str(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "desc OSF."]
        #[must_use]
        #[inline(always)]
        pub const fn osf(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "desc OSF."]
        #[inline(always)]
        pub const fn set_osf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "desc RTC."]
        #[must_use]
        #[inline(always)]
        pub const fn rtc(&self) -> u8 {
            let val = (self.0 >> 3usize) & 0x03;
            val as u8
        }
        #[doc = "desc RTC."]
        #[inline(always)]
        pub const fn set_rtc(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 3usize)) | (((val as u32) & 0x03) << 3usize);
        }
        #[doc = "desc DGF."]
        #[must_use]
        #[inline(always)]
        pub const fn dgf(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "desc DGF."]
        #[inline(always)]
        pub const fn set_dgf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "desc FUF."]
        #[must_use]
        #[inline(always)]
        pub const fn fuf(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "desc FUF."]
        #[inline(always)]
        pub const fn set_fuf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "desc FEF."]
        #[must_use]
        #[inline(always)]
        pub const fn fef(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "desc FEF."]
        #[inline(always)]
        pub const fn set_fef(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "desc STT."]
        #[must_use]
        #[inline(always)]
        pub const fn stt(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[doc = "desc STT."]
        #[inline(always)]
        pub const fn set_stt(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[doc = "desc TTC."]
        #[must_use]
        #[inline(always)]
        pub const fn ttc(&self) -> u8 {
            let val = (self.0 >> 14usize) & 0x07;
            val as u8
        }
        #[doc = "desc TTC."]
        #[inline(always)]
        pub const fn set_ttc(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 14usize)) | (((val as u32) & 0x07) << 14usize);
        }
        #[doc = "desc FTF."]
        #[must_use]
        #[inline(always)]
        pub const fn ftf(&self) -> bool {
            let val = (self.0 >> 20usize) & 0x01;
            val != 0
        }
        #[doc = "desc FTF."]
        #[inline(always)]
        pub const fn set_ftf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
        }
        #[doc = "desc TSF."]
        #[must_use]
        #[inline(always)]
        pub const fn tsf(&self) -> bool {
            let val = (self.0 >> 21usize) & 0x01;
            val != 0
        }
        #[doc = "desc TSF."]
        #[inline(always)]
        pub const fn set_tsf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
        }
        #[doc = "desc DFRF."]
        #[must_use]
        #[inline(always)]
        pub const fn dfrf(&self) -> bool {
            let val = (self.0 >> 24usize) & 0x01;
            val != 0
        }
        #[doc = "desc DFRF."]
        #[inline(always)]
        pub const fn set_dfrf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
        }
        #[doc = "desc RSF."]
        #[must_use]
        #[inline(always)]
        pub const fn rsf(&self) -> bool {
            let val = (self.0 >> 25usize) & 0x01;
            val != 0
        }
        #[doc = "desc RSF."]
        #[inline(always)]
        pub const fn set_rsf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
        }
        #[doc = "desc DTCOE."]
        #[must_use]
        #[inline(always)]
        pub const fn dtcoe(&self) -> bool {
            let val = (self.0 >> 26usize) & 0x01;
            val != 0
        }
        #[doc = "desc DTCOE."]
        #[inline(always)]
        pub const fn set_dtcoe(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
        }
    }
    impl Default for DmaOprmodr {
        #[inline(always)]
        fn default() -> DmaOprmodr {
            DmaOprmodr(0)
        }
    }
    impl core::fmt::Debug for DmaOprmodr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("DmaOprmodr")
                .field("str", &self.str())
                .field("osf", &self.osf())
                .field("rtc", &self.rtc())
                .field("dgf", &self.dgf())
                .field("fuf", &self.fuf())
                .field("fef", &self.fef())
                .field("stt", &self.stt())
                .field("ttc", &self.ttc())
                .field("ftf", &self.ftf())
                .field("tsf", &self.tsf())
                .field("dfrf", &self.dfrf())
                .field("rsf", &self.rsf())
                .field("dtcoe", &self.dtcoe())
                .finish()
        }
    }
    #[doc = "desc DMA_REVWDTR."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct DmaRevwdtr(pub u32);
    impl DmaRevwdtr {
        #[doc = "desc RIWT."]
        #[must_use]
        #[inline(always)]
        pub const fn riwt(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[doc = "desc RIWT."]
        #[inline(always)]
        pub const fn set_riwt(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
        }
    }
    impl Default for DmaRevwdtr {
        #[inline(always)]
        fn default() -> DmaRevwdtr {
            DmaRevwdtr(0)
        }
    }
    impl core::fmt::Debug for DmaRevwdtr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("DmaRevwdtr")
                .field("riwt", &self.riwt())
                .finish()
        }
    }
    #[doc = "desc DMA_RFRCNTR."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct DmaRfrcntr(pub u32);
    impl DmaRfrcntr {
        #[doc = "desc UNACNT."]
        #[must_use]
        #[inline(always)]
        pub const fn unacnt(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[doc = "desc UNACNT."]
        #[inline(always)]
        pub const fn set_unacnt(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
        }
        #[doc = "desc UNAOVF."]
        #[must_use]
        #[inline(always)]
        pub const fn unaovf(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "desc UNAOVF."]
        #[inline(always)]
        pub const fn set_unaovf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "desc OVFCNT."]
        #[must_use]
        #[inline(always)]
        pub const fn ovfcnt(&self) -> u16 {
            let val = (self.0 >> 17usize) & 0x07ff;
            val as u16
        }
        #[doc = "desc OVFCNT."]
        #[inline(always)]
        pub const fn set_ovfcnt(&mut self, val: u16) {
            self.0 = (self.0 & !(0x07ff << 17usize)) | (((val as u32) & 0x07ff) << 17usize);
        }
        #[doc = "desc OVFOVF."]
        #[must_use]
        #[inline(always)]
        pub const fn ovfovf(&self) -> bool {
            let val = (self.0 >> 28usize) & 0x01;
            val != 0
        }
        #[doc = "desc OVFOVF."]
        #[inline(always)]
        pub const fn set_ovfovf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
        }
    }
    impl Default for DmaRfrcntr {
        #[inline(always)]
        fn default() -> DmaRfrcntr {
            DmaRfrcntr(0)
        }
    }
    impl core::fmt::Debug for DmaRfrcntr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("DmaRfrcntr")
                .field("unacnt", &self.unacnt())
                .field("unaovf", &self.unaovf())
                .field("ovfcnt", &self.ovfcnt())
                .field("ovfovf", &self.ovfovf())
                .finish()
        }
    }
    #[doc = "desc DMA_RXDLADR."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct DmaRxdladr(pub u32);
    impl DmaRxdladr {
        #[doc = "desc RXDLAD."]
        #[must_use]
        #[inline(always)]
        pub const fn rxdlad(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "desc RXDLAD."]
        #[inline(always)]
        pub const fn set_rxdlad(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for DmaRxdladr {
        #[inline(always)]
        fn default() -> DmaRxdladr {
            DmaRxdladr(0)
        }
    }
    impl core::fmt::Debug for DmaRxdladr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("DmaRxdladr")
                .field("rxdlad", &self.rxdlad())
                .finish()
        }
    }
    #[doc = "desc DMA_RXPOLLR."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct DmaRxpollr(pub u32);
    impl DmaRxpollr {
        #[doc = "desc RXPOLL."]
        #[must_use]
        #[inline(always)]
        pub const fn rxpoll(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "desc RXPOLL."]
        #[inline(always)]
        pub const fn set_rxpoll(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for DmaRxpollr {
        #[inline(always)]
        fn default() -> DmaRxpollr {
            DmaRxpollr(0)
        }
    }
    impl core::fmt::Debug for DmaRxpollr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("DmaRxpollr")
                .field("rxpoll", &self.rxpoll())
                .finish()
        }
    }
    #[doc = "desc DMA_TXDLADR."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct DmaTxdladr(pub u32);
    impl DmaTxdladr {
        #[doc = "desc TXDLAD."]
        #[must_use]
        #[inline(always)]
        pub const fn txdlad(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "desc TXDLAD."]
        #[inline(always)]
        pub const fn set_txdlad(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for DmaTxdladr {
        #[inline(always)]
        fn default() -> DmaTxdladr {
            DmaTxdladr(0)
        }
    }
    impl core::fmt::Debug for DmaTxdladr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("DmaTxdladr")
                .field("txdlad", &self.txdlad())
                .finish()
        }
    }
    #[doc = "desc DMA_TXPOLLR."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct DmaTxpollr(pub u32);
    impl DmaTxpollr {
        #[doc = "desc TXPOLL."]
        #[must_use]
        #[inline(always)]
        pub const fn txpoll(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "desc TXPOLL."]
        #[inline(always)]
        pub const fn set_txpoll(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for DmaTxpollr {
        #[inline(always)]
        fn default() -> DmaTxpollr {
            DmaTxpollr(0)
        }
    }
    impl core::fmt::Debug for DmaTxpollr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("DmaTxpollr")
                .field("txpoll", &self.txpoll())
                .finish()
        }
    }
    #[doc = "desc MAC_CONFIGR."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct MacConfigr(pub u32);
    impl MacConfigr {
        #[doc = "desc RE."]
        #[must_use]
        #[inline(always)]
        pub const fn re(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "desc RE."]
        #[inline(always)]
        pub const fn set_re(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "desc TE."]
        #[must_use]
        #[inline(always)]
        pub const fn te(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "desc TE."]
        #[inline(always)]
        pub const fn set_te(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "desc DC."]
        #[must_use]
        #[inline(always)]
        pub const fn dc(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "desc DC."]
        #[inline(always)]
        pub const fn set_dc(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "desc BL."]
        #[must_use]
        #[inline(always)]
        pub const fn bl(&self) -> u8 {
            let val = (self.0 >> 5usize) & 0x03;
            val as u8
        }
        #[doc = "desc BL."]
        #[inline(always)]
        pub const fn set_bl(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 5usize)) | (((val as u32) & 0x03) << 5usize);
        }
        #[doc = "desc ACS."]
        #[must_use]
        #[inline(always)]
        pub const fn acs(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "desc ACS."]
        #[inline(always)]
        pub const fn set_acs(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "desc DRTY."]
        #[must_use]
        #[inline(always)]
        pub const fn drty(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "desc DRTY."]
        #[inline(always)]
        pub const fn set_drty(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "desc IPCO."]
        #[must_use]
        #[inline(always)]
        pub const fn ipco(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "desc IPCO."]
        #[inline(always)]
        pub const fn set_ipco(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[doc = "desc DM."]
        #[must_use]
        #[inline(always)]
        pub const fn dm(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "desc DM."]
        #[inline(always)]
        pub const fn set_dm(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[doc = "desc LM."]
        #[must_use]
        #[inline(always)]
        pub const fn lm(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "desc LM."]
        #[inline(always)]
        pub const fn set_lm(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "desc DO."]
        #[must_use]
        #[inline(always)]
        pub const fn do_(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[doc = "desc DO."]
        #[inline(always)]
        pub const fn set_do_(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[doc = "desc FES."]
        #[must_use]
        #[inline(always)]
        pub const fn fes(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "desc FES."]
        #[inline(always)]
        pub const fn set_fes(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[doc = "desc DCRS."]
        #[must_use]
        #[inline(always)]
        pub const fn dcrs(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "desc DCRS."]
        #[inline(always)]
        pub const fn set_dcrs(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "desc IFG."]
        #[must_use]
        #[inline(always)]
        pub const fn ifg(&self) -> u8 {
            let val = (self.0 >> 17usize) & 0x07;
            val as u8
        }
        #[doc = "desc IFG."]
        #[inline(always)]
        pub const fn set_ifg(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 17usize)) | (((val as u32) & 0x07) << 17usize);
        }
        #[doc = "desc MJB."]
        #[must_use]
        #[inline(always)]
        pub const fn mjb(&self) -> bool {
            let val = (self.0 >> 22usize) & 0x01;
            val != 0
        }
        #[doc = "desc MJB."]
        #[inline(always)]
        pub const fn set_mjb(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
        }
        #[doc = "desc MWD."]
        #[must_use]
        #[inline(always)]
        pub const fn mwd(&self) -> bool {
            let val = (self.0 >> 23usize) & 0x01;
            val != 0
        }
        #[doc = "desc MWD."]
        #[inline(always)]
        pub const fn set_mwd(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
        }
        #[doc = "desc CST."]
        #[must_use]
        #[inline(always)]
        pub const fn cst(&self) -> bool {
            let val = (self.0 >> 25usize) & 0x01;
            val != 0
        }
        #[doc = "desc CST."]
        #[inline(always)]
        pub const fn set_cst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
        }
        #[doc = "desc SAIRC."]
        #[must_use]
        #[inline(always)]
        pub const fn sairc(&self) -> u8 {
            let val = (self.0 >> 28usize) & 0x07;
            val as u8
        }
        #[doc = "desc SAIRC."]
        #[inline(always)]
        pub const fn set_sairc(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 28usize)) | (((val as u32) & 0x07) << 28usize);
        }
    }
    impl Default for MacConfigr {
        #[inline(always)]
        fn default() -> MacConfigr {
            MacConfigr(0)
        }
    }
    impl core::fmt::Debug for MacConfigr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("MacConfigr")
                .field("re", &self.re())
                .field("te", &self.te())
                .field("dc", &self.dc())
                .field("bl", &self.bl())
                .field("acs", &self.acs())
                .field("drty", &self.drty())
                .field("ipco", &self.ipco())
                .field("dm", &self.dm())
                .field("lm", &self.lm())
                .field("do_", &self.do_())
                .field("fes", &self.fes())
                .field("dcrs", &self.dcrs())
                .field("ifg", &self.ifg())
                .field("mjb", &self.mjb())
                .field("mwd", &self.mwd())
                .field("cst", &self.cst())
                .field("sairc", &self.sairc())
                .finish()
        }
    }
    #[doc = "desc MAC_FLOCTLR."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct MacFloctlr(pub u32);
    impl MacFloctlr {
        #[doc = "desc FCA_BPA."]
        #[must_use]
        #[inline(always)]
        pub const fn fca_bpa(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "desc FCA_BPA."]
        #[inline(always)]
        pub const fn set_fca_bpa(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "desc TFE."]
        #[must_use]
        #[inline(always)]
        pub const fn tfe(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "desc TFE."]
        #[inline(always)]
        pub const fn set_tfe(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "desc RFE."]
        #[must_use]
        #[inline(always)]
        pub const fn rfe(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "desc RFE."]
        #[inline(always)]
        pub const fn set_rfe(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "desc UNP."]
        #[must_use]
        #[inline(always)]
        pub const fn unp(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "desc UNP."]
        #[inline(always)]
        pub const fn set_unp(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "desc PLT."]
        #[must_use]
        #[inline(always)]
        pub const fn plt(&self) -> u8 {
            let val = (self.0 >> 4usize) & 0x03;
            val as u8
        }
        #[doc = "desc PLT."]
        #[inline(always)]
        pub const fn set_plt(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 4usize)) | (((val as u32) & 0x03) << 4usize);
        }
        #[doc = "desc DZPQ."]
        #[must_use]
        #[inline(always)]
        pub const fn dzpq(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "desc DZPQ."]
        #[inline(always)]
        pub const fn set_dzpq(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "desc PAUSET."]
        #[must_use]
        #[inline(always)]
        pub const fn pauset(&self) -> u16 {
            let val = (self.0 >> 16usize) & 0xffff;
            val as u16
        }
        #[doc = "desc PAUSET."]
        #[inline(always)]
        pub const fn set_pauset(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 16usize)) | (((val as u32) & 0xffff) << 16usize);
        }
    }
    impl Default for MacFloctlr {
        #[inline(always)]
        fn default() -> MacFloctlr {
            MacFloctlr(0)
        }
    }
    impl core::fmt::Debug for MacFloctlr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("MacFloctlr")
                .field("fca_bpa", &self.fca_bpa())
                .field("tfe", &self.tfe())
                .field("rfe", &self.rfe())
                .field("unp", &self.unp())
                .field("plt", &self.plt())
                .field("dzpq", &self.dzpq())
                .field("pauset", &self.pauset())
                .finish()
        }
    }
    #[doc = "desc MAC_FLTCTLR."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct MacFltctlr(pub u32);
    impl MacFltctlr {
        #[doc = "desc PR."]
        #[must_use]
        #[inline(always)]
        pub const fn pr(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "desc PR."]
        #[inline(always)]
        pub const fn set_pr(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "desc HUC."]
        #[must_use]
        #[inline(always)]
        pub const fn huc(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "desc HUC."]
        #[inline(always)]
        pub const fn set_huc(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "desc HMC."]
        #[must_use]
        #[inline(always)]
        pub const fn hmc(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "desc HMC."]
        #[inline(always)]
        pub const fn set_hmc(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "desc DAIF."]
        #[must_use]
        #[inline(always)]
        pub const fn daif(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "desc DAIF."]
        #[inline(always)]
        pub const fn set_daif(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "desc PMF."]
        #[must_use]
        #[inline(always)]
        pub const fn pmf(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "desc PMF."]
        #[inline(always)]
        pub const fn set_pmf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "desc DBF."]
        #[must_use]
        #[inline(always)]
        pub const fn dbf(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "desc DBF."]
        #[inline(always)]
        pub const fn set_dbf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "desc PCF."]
        #[must_use]
        #[inline(always)]
        pub const fn pcf(&self) -> u8 {
            let val = (self.0 >> 6usize) & 0x03;
            val as u8
        }
        #[doc = "desc PCF."]
        #[inline(always)]
        pub const fn set_pcf(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 6usize)) | (((val as u32) & 0x03) << 6usize);
        }
        #[doc = "desc SAIF."]
        #[must_use]
        #[inline(always)]
        pub const fn saif(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "desc SAIF."]
        #[inline(always)]
        pub const fn set_saif(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "desc SAF."]
        #[must_use]
        #[inline(always)]
        pub const fn saf(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "desc SAF."]
        #[inline(always)]
        pub const fn set_saf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "desc HPF."]
        #[must_use]
        #[inline(always)]
        pub const fn hpf(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "desc HPF."]
        #[inline(always)]
        pub const fn set_hpf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[doc = "desc VTFE."]
        #[must_use]
        #[inline(always)]
        pub const fn vtfe(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "desc VTFE."]
        #[inline(always)]
        pub const fn set_vtfe(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "desc IPFE."]
        #[must_use]
        #[inline(always)]
        pub const fn ipfe(&self) -> bool {
            let val = (self.0 >> 20usize) & 0x01;
            val != 0
        }
        #[doc = "desc IPFE."]
        #[inline(always)]
        pub const fn set_ipfe(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
        }
        #[doc = "desc DNTU."]
        #[must_use]
        #[inline(always)]
        pub const fn dntu(&self) -> bool {
            let val = (self.0 >> 21usize) & 0x01;
            val != 0
        }
        #[doc = "desc DNTU."]
        #[inline(always)]
        pub const fn set_dntu(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
        }
        #[doc = "desc RA."]
        #[must_use]
        #[inline(always)]
        pub const fn ra(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[doc = "desc RA."]
        #[inline(always)]
        pub const fn set_ra(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for MacFltctlr {
        #[inline(always)]
        fn default() -> MacFltctlr {
            MacFltctlr(0)
        }
    }
    impl core::fmt::Debug for MacFltctlr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("MacFltctlr")
                .field("pr", &self.pr())
                .field("huc", &self.huc())
                .field("hmc", &self.hmc())
                .field("daif", &self.daif())
                .field("pmf", &self.pmf())
                .field("dbf", &self.dbf())
                .field("pcf", &self.pcf())
                .field("saif", &self.saif())
                .field("saf", &self.saf())
                .field("hpf", &self.hpf())
                .field("vtfe", &self.vtfe())
                .field("ipfe", &self.ipfe())
                .field("dntu", &self.dntu())
                .field("ra", &self.ra())
                .finish()
        }
    }
    #[doc = "desc MAC_HASHTHR."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct MacHashthr(pub u32);
    impl MacHashthr {
        #[doc = "desc HTH."]
        #[must_use]
        #[inline(always)]
        pub const fn hth(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "desc HTH."]
        #[inline(always)]
        pub const fn set_hth(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for MacHashthr {
        #[inline(always)]
        fn default() -> MacHashthr {
            MacHashthr(0)
        }
    }
    impl core::fmt::Debug for MacHashthr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("MacHashthr")
                .field("hth", &self.hth())
                .finish()
        }
    }
    #[doc = "desc MAC_HASHTLR."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct MacHashtlr(pub u32);
    impl MacHashtlr {
        #[doc = "desc HTL."]
        #[must_use]
        #[inline(always)]
        pub const fn htl(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "desc HTL."]
        #[inline(always)]
        pub const fn set_htl(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for MacHashtlr {
        #[inline(always)]
        fn default() -> MacHashtlr {
            MacHashtlr(0)
        }
    }
    impl core::fmt::Debug for MacHashtlr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("MacHashtlr")
                .field("htl", &self.htl())
                .finish()
        }
    }
    #[doc = "desc MAC_IFCONFR."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct MacIfconfr(pub u32);
    impl MacIfconfr {
        #[doc = "desc IFSEL."]
        #[must_use]
        #[inline(always)]
        pub const fn ifsel(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "desc IFSEL."]
        #[inline(always)]
        pub const fn set_ifsel(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "desc RCKINV."]
        #[must_use]
        #[inline(always)]
        pub const fn rckinv(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "desc RCKINV."]
        #[inline(always)]
        pub const fn set_rckinv(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "desc TCKINV."]
        #[must_use]
        #[inline(always)]
        pub const fn tckinv(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "desc TCKINV."]
        #[inline(always)]
        pub const fn set_tckinv(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
    }
    impl Default for MacIfconfr {
        #[inline(always)]
        fn default() -> MacIfconfr {
            MacIfconfr(0)
        }
    }
    impl core::fmt::Debug for MacIfconfr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("MacIfconfr")
                .field("ifsel", &self.ifsel())
                .field("rckinv", &self.rckinv())
                .field("tckinv", &self.tckinv())
                .finish()
        }
    }
    #[doc = "desc MAC_INTMSKR."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct MacIntmskr(pub u32);
    impl MacIntmskr {
        #[doc = "desc PMTIM."]
        #[must_use]
        #[inline(always)]
        pub const fn pmtim(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "desc PMTIM."]
        #[inline(always)]
        pub const fn set_pmtim(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "desc TSPIM."]
        #[must_use]
        #[inline(always)]
        pub const fn tspim(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "desc TSPIM."]
        #[inline(always)]
        pub const fn set_tspim(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
    }
    impl Default for MacIntmskr {
        #[inline(always)]
        fn default() -> MacIntmskr {
            MacIntmskr(0)
        }
    }
    impl core::fmt::Debug for MacIntmskr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("MacIntmskr")
                .field("pmtim", &self.pmtim())
                .field("tspim", &self.tspim())
                .finish()
        }
    }
    #[doc = "desc MAC_INTSTSR."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct MacIntstsr(pub u32);
    impl MacIntstsr {
        #[doc = "desc PMTIS."]
        #[must_use]
        #[inline(always)]
        pub const fn pmtis(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "desc PMTIS."]
        #[inline(always)]
        pub const fn set_pmtis(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "desc MMCIS."]
        #[must_use]
        #[inline(always)]
        pub const fn mmcis(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "desc MMCIS."]
        #[inline(always)]
        pub const fn set_mmcis(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "desc MMCRXIS."]
        #[must_use]
        #[inline(always)]
        pub const fn mmcrxis(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "desc MMCRXIS."]
        #[inline(always)]
        pub const fn set_mmcrxis(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "desc MMCTXIS."]
        #[must_use]
        #[inline(always)]
        pub const fn mmctxis(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "desc MMCTXIS."]
        #[inline(always)]
        pub const fn set_mmctxis(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "desc TSPIS."]
        #[must_use]
        #[inline(always)]
        pub const fn tspis(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "desc TSPIS."]
        #[inline(always)]
        pub const fn set_tspis(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
    }
    impl Default for MacIntstsr {
        #[inline(always)]
        fn default() -> MacIntstsr {
            MacIntstsr(0)
        }
    }
    impl core::fmt::Debug for MacIntstsr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("MacIntstsr")
                .field("pmtis", &self.pmtis())
                .field("mmcis", &self.mmcis())
                .field("mmcrxis", &self.mmcrxis())
                .field("mmctxis", &self.mmctxis())
                .field("tspis", &self.tspis())
                .finish()
        }
    }
    #[doc = "desc MAC_L34CTLR."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct MacL34ctlr(pub u32);
    impl MacL34ctlr {
        #[doc = "desc L3PEN."]
        #[must_use]
        #[inline(always)]
        pub const fn l3pen(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "desc L3PEN."]
        #[inline(always)]
        pub const fn set_l3pen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "desc L3SAM."]
        #[must_use]
        #[inline(always)]
        pub const fn l3sam(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "desc L3SAM."]
        #[inline(always)]
        pub const fn set_l3sam(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "desc L3SAIM."]
        #[must_use]
        #[inline(always)]
        pub const fn l3saim(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "desc L3SAIM."]
        #[inline(always)]
        pub const fn set_l3saim(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "desc L3DAM."]
        #[must_use]
        #[inline(always)]
        pub const fn l3dam(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "desc L3DAM."]
        #[inline(always)]
        pub const fn set_l3dam(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "desc L3DAIM."]
        #[must_use]
        #[inline(always)]
        pub const fn l3daim(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "desc L3DAIM."]
        #[inline(always)]
        pub const fn set_l3daim(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "desc L3HSBM."]
        #[must_use]
        #[inline(always)]
        pub const fn l3hsbm(&self) -> u8 {
            let val = (self.0 >> 6usize) & 0x1f;
            val as u8
        }
        #[doc = "desc L3HSBM."]
        #[inline(always)]
        pub const fn set_l3hsbm(&mut self, val: u8) {
            self.0 = (self.0 & !(0x1f << 6usize)) | (((val as u32) & 0x1f) << 6usize);
        }
        #[doc = "desc L3HDBM."]
        #[must_use]
        #[inline(always)]
        pub const fn l3hdbm(&self) -> u8 {
            let val = (self.0 >> 11usize) & 0x1f;
            val as u8
        }
        #[doc = "desc L3HDBM."]
        #[inline(always)]
        pub const fn set_l3hdbm(&mut self, val: u8) {
            self.0 = (self.0 & !(0x1f << 11usize)) | (((val as u32) & 0x1f) << 11usize);
        }
        #[doc = "desc L4PEN."]
        #[must_use]
        #[inline(always)]
        pub const fn l4pen(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "desc L4PEN."]
        #[inline(always)]
        pub const fn set_l4pen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "desc L4SPM."]
        #[must_use]
        #[inline(always)]
        pub const fn l4spm(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[doc = "desc L4SPM."]
        #[inline(always)]
        pub const fn set_l4spm(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
        #[doc = "desc L4SPIM."]
        #[must_use]
        #[inline(always)]
        pub const fn l4spim(&self) -> bool {
            let val = (self.0 >> 19usize) & 0x01;
            val != 0
        }
        #[doc = "desc L4SPIM."]
        #[inline(always)]
        pub const fn set_l4spim(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
        }
        #[doc = "desc L4DPM."]
        #[must_use]
        #[inline(always)]
        pub const fn l4dpm(&self) -> bool {
            let val = (self.0 >> 20usize) & 0x01;
            val != 0
        }
        #[doc = "desc L4DPM."]
        #[inline(always)]
        pub const fn set_l4dpm(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
        }
        #[doc = "desc L4DPIM."]
        #[must_use]
        #[inline(always)]
        pub const fn l4dpim(&self) -> bool {
            let val = (self.0 >> 21usize) & 0x01;
            val != 0
        }
        #[doc = "desc L4DPIM."]
        #[inline(always)]
        pub const fn set_l4dpim(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
        }
    }
    impl Default for MacL34ctlr {
        #[inline(always)]
        fn default() -> MacL34ctlr {
            MacL34ctlr(0)
        }
    }
    impl core::fmt::Debug for MacL34ctlr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("MacL34ctlr")
                .field("l3pen", &self.l3pen())
                .field("l3sam", &self.l3sam())
                .field("l3saim", &self.l3saim())
                .field("l3dam", &self.l3dam())
                .field("l3daim", &self.l3daim())
                .field("l3hsbm", &self.l3hsbm())
                .field("l3hdbm", &self.l3hdbm())
                .field("l4pen", &self.l4pen())
                .field("l4spm", &self.l4spm())
                .field("l4spim", &self.l4spim())
                .field("l4dpm", &self.l4dpm())
                .field("l4dpim", &self.l4dpim())
                .finish()
        }
    }
    #[doc = "desc MAC_L3ADDRR0."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct MacL3addrr0(pub u32);
    impl MacL3addrr0 {
        #[doc = "desc L3ADDR0."]
        #[must_use]
        #[inline(always)]
        pub const fn l3addr0(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "desc L3ADDR0."]
        #[inline(always)]
        pub const fn set_l3addr0(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for MacL3addrr0 {
        #[inline(always)]
        fn default() -> MacL3addrr0 {
            MacL3addrr0(0)
        }
    }
    impl core::fmt::Debug for MacL3addrr0 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("MacL3addrr0")
                .field("l3addr0", &self.l3addr0())
                .finish()
        }
    }
    #[doc = "desc MAC_L3ADDRR1."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct MacL3addrr1(pub u32);
    impl MacL3addrr1 {
        #[doc = "desc L3ADDR2."]
        #[must_use]
        #[inline(always)]
        pub const fn l3addr2(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "desc L3ADDR2."]
        #[inline(always)]
        pub const fn set_l3addr2(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for MacL3addrr1 {
        #[inline(always)]
        fn default() -> MacL3addrr1 {
            MacL3addrr1(0)
        }
    }
    impl core::fmt::Debug for MacL3addrr1 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("MacL3addrr1")
                .field("l3addr2", &self.l3addr2())
                .finish()
        }
    }
    #[doc = "desc MAC_L3ADDRR2."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct MacL3addrr2(pub u32);
    impl MacL3addrr2 {
        #[doc = "desc L3ADDR2."]
        #[must_use]
        #[inline(always)]
        pub const fn l3addr2(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "desc L3ADDR2."]
        #[inline(always)]
        pub const fn set_l3addr2(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for MacL3addrr2 {
        #[inline(always)]
        fn default() -> MacL3addrr2 {
            MacL3addrr2(0)
        }
    }
    impl core::fmt::Debug for MacL3addrr2 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("MacL3addrr2")
                .field("l3addr2", &self.l3addr2())
                .finish()
        }
    }
    #[doc = "desc MAC_L3ADDRR3."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct MacL3addrr3(pub u32);
    impl MacL3addrr3 {
        #[doc = "desc L3ADDR3."]
        #[must_use]
        #[inline(always)]
        pub const fn l3addr3(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "desc L3ADDR3."]
        #[inline(always)]
        pub const fn set_l3addr3(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for MacL3addrr3 {
        #[inline(always)]
        fn default() -> MacL3addrr3 {
            MacL3addrr3(0)
        }
    }
    impl core::fmt::Debug for MacL3addrr3 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("MacL3addrr3")
                .field("l3addr3", &self.l3addr3())
                .finish()
        }
    }
    #[doc = "desc MAC_L4PORTR."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct MacL4portr(pub u32);
    impl MacL4portr {
        #[doc = "desc L4SPVAL."]
        #[must_use]
        #[inline(always)]
        pub const fn l4spval(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[doc = "desc L4SPVAL."]
        #[inline(always)]
        pub const fn set_l4spval(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
        }
        #[doc = "desc L4DPVAL."]
        #[must_use]
        #[inline(always)]
        pub const fn l4dpval(&self) -> u16 {
            let val = (self.0 >> 16usize) & 0xffff;
            val as u16
        }
        #[doc = "desc L4DPVAL."]
        #[inline(always)]
        pub const fn set_l4dpval(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 16usize)) | (((val as u32) & 0xffff) << 16usize);
        }
    }
    impl Default for MacL4portr {
        #[inline(always)]
        fn default() -> MacL4portr {
            MacL4portr(0)
        }
    }
    impl core::fmt::Debug for MacL4portr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("MacL4portr")
                .field("l4spval", &self.l4spval())
                .field("l4dpval", &self.l4dpval())
                .finish()
        }
    }
    #[doc = "desc MAC_MACADHR0."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct MacMacadhr0(pub u32);
    impl MacMacadhr0 {
        #[doc = "desc ADDRH0."]
        #[must_use]
        #[inline(always)]
        pub const fn addrh0(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[doc = "desc ADDRH0."]
        #[inline(always)]
        pub const fn set_addrh0(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
        }
        #[doc = "desc AE0."]
        #[must_use]
        #[inline(always)]
        pub const fn ae0(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[doc = "desc AE0."]
        #[inline(always)]
        pub const fn set_ae0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for MacMacadhr0 {
        #[inline(always)]
        fn default() -> MacMacadhr0 {
            MacMacadhr0(0)
        }
    }
    impl core::fmt::Debug for MacMacadhr0 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("MacMacadhr0")
                .field("addrh0", &self.addrh0())
                .field("ae0", &self.ae0())
                .finish()
        }
    }
    #[doc = "desc MAC_MACADHR1."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct MacMacadhr1(pub u32);
    impl MacMacadhr1 {
        #[doc = "desc ADDRH1."]
        #[must_use]
        #[inline(always)]
        pub const fn addrh1(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[doc = "desc ADDRH1."]
        #[inline(always)]
        pub const fn set_addrh1(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
        }
        #[doc = "desc MBC1."]
        #[must_use]
        #[inline(always)]
        pub const fn mbc1(&self) -> u8 {
            let val = (self.0 >> 24usize) & 0x3f;
            val as u8
        }
        #[doc = "desc MBC1."]
        #[inline(always)]
        pub const fn set_mbc1(&mut self, val: u8) {
            self.0 = (self.0 & !(0x3f << 24usize)) | (((val as u32) & 0x3f) << 24usize);
        }
        #[doc = "desc SA1."]
        #[must_use]
        #[inline(always)]
        pub const fn sa1(&self) -> bool {
            let val = (self.0 >> 30usize) & 0x01;
            val != 0
        }
        #[doc = "desc SA1."]
        #[inline(always)]
        pub const fn set_sa1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
        }
        #[doc = "desc AE1."]
        #[must_use]
        #[inline(always)]
        pub const fn ae1(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[doc = "desc AE1."]
        #[inline(always)]
        pub const fn set_ae1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for MacMacadhr1 {
        #[inline(always)]
        fn default() -> MacMacadhr1 {
            MacMacadhr1(0)
        }
    }
    impl core::fmt::Debug for MacMacadhr1 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("MacMacadhr1")
                .field("addrh1", &self.addrh1())
                .field("mbc1", &self.mbc1())
                .field("sa1", &self.sa1())
                .field("ae1", &self.ae1())
                .finish()
        }
    }
    #[doc = "desc MAC_MACADHR2."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct MacMacadhr2(pub u32);
    impl MacMacadhr2 {
        #[doc = "desc ADDRH2."]
        #[must_use]
        #[inline(always)]
        pub const fn addrh2(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[doc = "desc ADDRH2."]
        #[inline(always)]
        pub const fn set_addrh2(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
        }
        #[doc = "desc MBC2."]
        #[must_use]
        #[inline(always)]
        pub const fn mbc2(&self) -> u8 {
            let val = (self.0 >> 24usize) & 0x3f;
            val as u8
        }
        #[doc = "desc MBC2."]
        #[inline(always)]
        pub const fn set_mbc2(&mut self, val: u8) {
            self.0 = (self.0 & !(0x3f << 24usize)) | (((val as u32) & 0x3f) << 24usize);
        }
        #[doc = "desc SA2."]
        #[must_use]
        #[inline(always)]
        pub const fn sa2(&self) -> bool {
            let val = (self.0 >> 30usize) & 0x01;
            val != 0
        }
        #[doc = "desc SA2."]
        #[inline(always)]
        pub const fn set_sa2(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
        }
        #[doc = "desc AE2."]
        #[must_use]
        #[inline(always)]
        pub const fn ae2(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[doc = "desc AE2."]
        #[inline(always)]
        pub const fn set_ae2(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for MacMacadhr2 {
        #[inline(always)]
        fn default() -> MacMacadhr2 {
            MacMacadhr2(0)
        }
    }
    impl core::fmt::Debug for MacMacadhr2 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("MacMacadhr2")
                .field("addrh2", &self.addrh2())
                .field("mbc2", &self.mbc2())
                .field("sa2", &self.sa2())
                .field("ae2", &self.ae2())
                .finish()
        }
    }
    #[doc = "desc MAC_MACADHR3."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct MacMacadhr3(pub u32);
    impl MacMacadhr3 {
        #[doc = "desc ADDRH3."]
        #[must_use]
        #[inline(always)]
        pub const fn addrh3(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[doc = "desc ADDRH3."]
        #[inline(always)]
        pub const fn set_addrh3(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
        }
        #[doc = "desc MBC3."]
        #[must_use]
        #[inline(always)]
        pub const fn mbc3(&self) -> u8 {
            let val = (self.0 >> 24usize) & 0x3f;
            val as u8
        }
        #[doc = "desc MBC3."]
        #[inline(always)]
        pub const fn set_mbc3(&mut self, val: u8) {
            self.0 = (self.0 & !(0x3f << 24usize)) | (((val as u32) & 0x3f) << 24usize);
        }
        #[doc = "desc SA3."]
        #[must_use]
        #[inline(always)]
        pub const fn sa3(&self) -> bool {
            let val = (self.0 >> 30usize) & 0x01;
            val != 0
        }
        #[doc = "desc SA3."]
        #[inline(always)]
        pub const fn set_sa3(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
        }
        #[doc = "desc AE3."]
        #[must_use]
        #[inline(always)]
        pub const fn ae3(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[doc = "desc AE3."]
        #[inline(always)]
        pub const fn set_ae3(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for MacMacadhr3 {
        #[inline(always)]
        fn default() -> MacMacadhr3 {
            MacMacadhr3(0)
        }
    }
    impl core::fmt::Debug for MacMacadhr3 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("MacMacadhr3")
                .field("addrh3", &self.addrh3())
                .field("mbc3", &self.mbc3())
                .field("sa3", &self.sa3())
                .field("ae3", &self.ae3())
                .finish()
        }
    }
    #[doc = "desc MAC_MACADHR4."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct MacMacadhr4(pub u32);
    impl MacMacadhr4 {
        #[doc = "desc ADDRH4."]
        #[must_use]
        #[inline(always)]
        pub const fn addrh4(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[doc = "desc ADDRH4."]
        #[inline(always)]
        pub const fn set_addrh4(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
        }
        #[doc = "desc MBC4."]
        #[must_use]
        #[inline(always)]
        pub const fn mbc4(&self) -> u8 {
            let val = (self.0 >> 24usize) & 0x3f;
            val as u8
        }
        #[doc = "desc MBC4."]
        #[inline(always)]
        pub const fn set_mbc4(&mut self, val: u8) {
            self.0 = (self.0 & !(0x3f << 24usize)) | (((val as u32) & 0x3f) << 24usize);
        }
        #[doc = "desc SA4."]
        #[must_use]
        #[inline(always)]
        pub const fn sa4(&self) -> bool {
            let val = (self.0 >> 30usize) & 0x01;
            val != 0
        }
        #[doc = "desc SA4."]
        #[inline(always)]
        pub const fn set_sa4(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
        }
        #[doc = "desc AE4."]
        #[must_use]
        #[inline(always)]
        pub const fn ae4(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[doc = "desc AE4."]
        #[inline(always)]
        pub const fn set_ae4(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for MacMacadhr4 {
        #[inline(always)]
        fn default() -> MacMacadhr4 {
            MacMacadhr4(0)
        }
    }
    impl core::fmt::Debug for MacMacadhr4 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("MacMacadhr4")
                .field("addrh4", &self.addrh4())
                .field("mbc4", &self.mbc4())
                .field("sa4", &self.sa4())
                .field("ae4", &self.ae4())
                .finish()
        }
    }
    #[doc = "desc MAC_MACADLR0."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct MacMacadlr0(pub u32);
    impl MacMacadlr0 {
        #[doc = "desc ADDRL0."]
        #[must_use]
        #[inline(always)]
        pub const fn addrl0(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "desc ADDRL0."]
        #[inline(always)]
        pub const fn set_addrl0(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for MacMacadlr0 {
        #[inline(always)]
        fn default() -> MacMacadlr0 {
            MacMacadlr0(0)
        }
    }
    impl core::fmt::Debug for MacMacadlr0 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("MacMacadlr0")
                .field("addrl0", &self.addrl0())
                .finish()
        }
    }
    #[doc = "desc MAC_MACADLR1."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct MacMacadlr1(pub u32);
    impl MacMacadlr1 {
        #[doc = "desc ADDRL1."]
        #[must_use]
        #[inline(always)]
        pub const fn addrl1(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "desc ADDRL1."]
        #[inline(always)]
        pub const fn set_addrl1(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for MacMacadlr1 {
        #[inline(always)]
        fn default() -> MacMacadlr1 {
            MacMacadlr1(0)
        }
    }
    impl core::fmt::Debug for MacMacadlr1 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("MacMacadlr1")
                .field("addrl1", &self.addrl1())
                .finish()
        }
    }
    #[doc = "desc MAC_MACADLR2."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct MacMacadlr2(pub u32);
    impl MacMacadlr2 {
        #[doc = "desc ADDRL2."]
        #[must_use]
        #[inline(always)]
        pub const fn addrl2(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "desc ADDRL2."]
        #[inline(always)]
        pub const fn set_addrl2(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for MacMacadlr2 {
        #[inline(always)]
        fn default() -> MacMacadlr2 {
            MacMacadlr2(0)
        }
    }
    impl core::fmt::Debug for MacMacadlr2 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("MacMacadlr2")
                .field("addrl2", &self.addrl2())
                .finish()
        }
    }
    #[doc = "desc MAC_MACADLR3."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct MacMacadlr3(pub u32);
    impl MacMacadlr3 {
        #[doc = "desc ADDRL3."]
        #[must_use]
        #[inline(always)]
        pub const fn addrl3(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "desc ADDRL3."]
        #[inline(always)]
        pub const fn set_addrl3(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for MacMacadlr3 {
        #[inline(always)]
        fn default() -> MacMacadlr3 {
            MacMacadlr3(0)
        }
    }
    impl core::fmt::Debug for MacMacadlr3 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("MacMacadlr3")
                .field("addrl3", &self.addrl3())
                .finish()
        }
    }
    #[doc = "desc MAC_MACADLR4."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct MacMacadlr4(pub u32);
    impl MacMacadlr4 {
        #[doc = "desc ADDRL4."]
        #[must_use]
        #[inline(always)]
        pub const fn addrl4(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "desc ADDRL4."]
        #[inline(always)]
        pub const fn set_addrl4(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for MacMacadlr4 {
        #[inline(always)]
        fn default() -> MacMacadlr4 {
            MacMacadlr4(0)
        }
    }
    impl core::fmt::Debug for MacMacadlr4 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("MacMacadlr4")
                .field("addrl4", &self.addrl4())
                .finish()
        }
    }
    #[doc = "desc MAC_MACSTSR."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct MacMacstsr(pub u32);
    impl MacMacstsr {
        #[doc = "desc MREA."]
        #[must_use]
        #[inline(always)]
        pub const fn mrea(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "desc MREA."]
        #[inline(always)]
        pub const fn set_mrea(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "desc MRS."]
        #[must_use]
        #[inline(always)]
        pub const fn mrs(&self) -> u8 {
            let val = (self.0 >> 1usize) & 0x03;
            val as u8
        }
        #[doc = "desc MRS."]
        #[inline(always)]
        pub const fn set_mrs(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 1usize)) | (((val as u32) & 0x03) << 1usize);
        }
        #[doc = "desc RFWA."]
        #[must_use]
        #[inline(always)]
        pub const fn rfwa(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "desc RFWA."]
        #[inline(always)]
        pub const fn set_rfwa(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "desc RFRS."]
        #[must_use]
        #[inline(always)]
        pub const fn rfrs(&self) -> u8 {
            let val = (self.0 >> 5usize) & 0x03;
            val as u8
        }
        #[doc = "desc RFRS."]
        #[inline(always)]
        pub const fn set_rfrs(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 5usize)) | (((val as u32) & 0x03) << 5usize);
        }
        #[doc = "desc RFFL."]
        #[must_use]
        #[inline(always)]
        pub const fn rffl(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0x03;
            val as u8
        }
        #[doc = "desc RFFL."]
        #[inline(always)]
        pub const fn set_rffl(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 8usize)) | (((val as u32) & 0x03) << 8usize);
        }
        #[doc = "desc MTEA."]
        #[must_use]
        #[inline(always)]
        pub const fn mtea(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "desc MTEA."]
        #[inline(always)]
        pub const fn set_mtea(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "desc MTS."]
        #[must_use]
        #[inline(always)]
        pub const fn mts(&self) -> u8 {
            let val = (self.0 >> 17usize) & 0x03;
            val as u8
        }
        #[doc = "desc MTS."]
        #[inline(always)]
        pub const fn set_mts(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 17usize)) | (((val as u32) & 0x03) << 17usize);
        }
        #[doc = "desc MTP."]
        #[must_use]
        #[inline(always)]
        pub const fn mtp(&self) -> bool {
            let val = (self.0 >> 19usize) & 0x01;
            val != 0
        }
        #[doc = "desc MTP."]
        #[inline(always)]
        pub const fn set_mtp(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
        }
        #[doc = "desc TFRS."]
        #[must_use]
        #[inline(always)]
        pub const fn tfrs(&self) -> u8 {
            let val = (self.0 >> 20usize) & 0x03;
            val as u8
        }
        #[doc = "desc TFRS."]
        #[inline(always)]
        pub const fn set_tfrs(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 20usize)) | (((val as u32) & 0x03) << 20usize);
        }
        #[doc = "desc TFWA."]
        #[must_use]
        #[inline(always)]
        pub const fn tfwa(&self) -> bool {
            let val = (self.0 >> 22usize) & 0x01;
            val != 0
        }
        #[doc = "desc TFWA."]
        #[inline(always)]
        pub const fn set_tfwa(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
        }
        #[doc = "desc TFNE."]
        #[must_use]
        #[inline(always)]
        pub const fn tfne(&self) -> bool {
            let val = (self.0 >> 24usize) & 0x01;
            val != 0
        }
        #[doc = "desc TFNE."]
        #[inline(always)]
        pub const fn set_tfne(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
        }
        #[doc = "desc TFF."]
        #[must_use]
        #[inline(always)]
        pub const fn tff(&self) -> bool {
            let val = (self.0 >> 25usize) & 0x01;
            val != 0
        }
        #[doc = "desc TFF."]
        #[inline(always)]
        pub const fn set_tff(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
        }
    }
    impl Default for MacMacstsr {
        #[inline(always)]
        fn default() -> MacMacstsr {
            MacMacstsr(0)
        }
    }
    impl core::fmt::Debug for MacMacstsr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("MacMacstsr")
                .field("mrea", &self.mrea())
                .field("mrs", &self.mrs())
                .field("rfwa", &self.rfwa())
                .field("rfrs", &self.rfrs())
                .field("rffl", &self.rffl())
                .field("mtea", &self.mtea())
                .field("mts", &self.mts())
                .field("mtp", &self.mtp())
                .field("tfrs", &self.tfrs())
                .field("tfwa", &self.tfwa())
                .field("tfne", &self.tfne())
                .field("tff", &self.tff())
                .finish()
        }
    }
    #[doc = "desc MAC_PMTCTLR."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct MacPmtctlr(pub u32);
    impl MacPmtctlr {
        #[doc = "desc PWDN."]
        #[must_use]
        #[inline(always)]
        pub const fn pwdn(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "desc PWDN."]
        #[inline(always)]
        pub const fn set_pwdn(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "desc MPEN."]
        #[must_use]
        #[inline(always)]
        pub const fn mpen(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "desc MPEN."]
        #[inline(always)]
        pub const fn set_mpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "desc WKEN."]
        #[must_use]
        #[inline(always)]
        pub const fn wken(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "desc WKEN."]
        #[inline(always)]
        pub const fn set_wken(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "desc MPFR."]
        #[must_use]
        #[inline(always)]
        pub const fn mpfr(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "desc MPFR."]
        #[inline(always)]
        pub const fn set_mpfr(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "desc WKFR."]
        #[must_use]
        #[inline(always)]
        pub const fn wkfr(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "desc WKFR."]
        #[inline(always)]
        pub const fn set_wkfr(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "desc GLUB."]
        #[must_use]
        #[inline(always)]
        pub const fn glub(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "desc GLUB."]
        #[inline(always)]
        pub const fn set_glub(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "desc RTWKTR."]
        #[must_use]
        #[inline(always)]
        pub const fn rtwktr(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "desc RTWKTR."]
        #[inline(always)]
        pub const fn set_rtwktr(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[doc = "desc RTWKPT."]
        #[must_use]
        #[inline(always)]
        pub const fn rtwkpt(&self) -> u8 {
            let val = (self.0 >> 24usize) & 0x07;
            val as u8
        }
        #[doc = "desc RTWKPT."]
        #[inline(always)]
        pub const fn set_rtwkpt(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 24usize)) | (((val as u32) & 0x07) << 24usize);
        }
        #[doc = "desc RTWKFR."]
        #[must_use]
        #[inline(always)]
        pub const fn rtwkfr(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[doc = "desc RTWKFR."]
        #[inline(always)]
        pub const fn set_rtwkfr(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for MacPmtctlr {
        #[inline(always)]
        fn default() -> MacPmtctlr {
            MacPmtctlr(0)
        }
    }
    impl core::fmt::Debug for MacPmtctlr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("MacPmtctlr")
                .field("pwdn", &self.pwdn())
                .field("mpen", &self.mpen())
                .field("wken", &self.wken())
                .field("mpfr", &self.mpfr())
                .field("wkfr", &self.wkfr())
                .field("glub", &self.glub())
                .field("rtwktr", &self.rtwktr())
                .field("rtwkpt", &self.rtwkpt())
                .field("rtwkfr", &self.rtwkfr())
                .finish()
        }
    }
    #[doc = "desc MAC_RTWKFFR."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct MacRtwkffr(pub u32);
    impl MacRtwkffr {
        #[doc = "desc WKUPFRMFT."]
        #[must_use]
        #[inline(always)]
        pub const fn wkupfrmft(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "desc WKUPFRMFT."]
        #[inline(always)]
        pub const fn set_wkupfrmft(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for MacRtwkffr {
        #[inline(always)]
        fn default() -> MacRtwkffr {
            MacRtwkffr(0)
        }
    }
    impl core::fmt::Debug for MacRtwkffr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("MacRtwkffr")
                .field("wkupfrmft", &self.wkupfrmft())
                .finish()
        }
    }
    #[doc = "desc MAC_SMIADDR."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct MacSmiaddr(pub u32);
    impl MacSmiaddr {
        #[doc = "desc SMIB."]
        #[must_use]
        #[inline(always)]
        pub const fn smib(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "desc SMIB."]
        #[inline(always)]
        pub const fn set_smib(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "desc SMIW."]
        #[must_use]
        #[inline(always)]
        pub const fn smiw(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "desc SMIW."]
        #[inline(always)]
        pub const fn set_smiw(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "desc SMIC."]
        #[must_use]
        #[inline(always)]
        pub const fn smic(&self) -> u8 {
            let val = (self.0 >> 2usize) & 0x0f;
            val as u8
        }
        #[doc = "desc SMIC."]
        #[inline(always)]
        pub const fn set_smic(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 2usize)) | (((val as u32) & 0x0f) << 2usize);
        }
        #[doc = "desc SMIR."]
        #[must_use]
        #[inline(always)]
        pub const fn smir(&self) -> u8 {
            let val = (self.0 >> 6usize) & 0x1f;
            val as u8
        }
        #[doc = "desc SMIR."]
        #[inline(always)]
        pub const fn set_smir(&mut self, val: u8) {
            self.0 = (self.0 & !(0x1f << 6usize)) | (((val as u32) & 0x1f) << 6usize);
        }
        #[doc = "desc SMIA."]
        #[must_use]
        #[inline(always)]
        pub const fn smia(&self) -> u8 {
            let val = (self.0 >> 11usize) & 0x1f;
            val as u8
        }
        #[doc = "desc SMIA."]
        #[inline(always)]
        pub const fn set_smia(&mut self, val: u8) {
            self.0 = (self.0 & !(0x1f << 11usize)) | (((val as u32) & 0x1f) << 11usize);
        }
    }
    impl Default for MacSmiaddr {
        #[inline(always)]
        fn default() -> MacSmiaddr {
            MacSmiaddr(0)
        }
    }
    impl core::fmt::Debug for MacSmiaddr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("MacSmiaddr")
                .field("smib", &self.smib())
                .field("smiw", &self.smiw())
                .field("smic", &self.smic())
                .field("smir", &self.smir())
                .field("smia", &self.smia())
                .finish()
        }
    }
    #[doc = "desc MAC_SMIDATR."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct MacSmidatr(pub u32);
    impl MacSmidatr {
        #[doc = "desc SMID."]
        #[must_use]
        #[inline(always)]
        pub const fn smid(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[doc = "desc SMID."]
        #[inline(always)]
        pub const fn set_smid(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
        }
    }
    impl Default for MacSmidatr {
        #[inline(always)]
        fn default() -> MacSmidatr {
            MacSmidatr(0)
        }
    }
    impl core::fmt::Debug for MacSmidatr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("MacSmidatr")
                .field("smid", &self.smid())
                .finish()
        }
    }
    #[doc = "desc MAC_VLAHTBR."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct MacVlahtbr(pub u32);
    impl MacVlahtbr {
        #[doc = "desc VLHT."]
        #[must_use]
        #[inline(always)]
        pub const fn vlht(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[doc = "desc VLHT."]
        #[inline(always)]
        pub const fn set_vlht(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
        }
    }
    impl Default for MacVlahtbr {
        #[inline(always)]
        fn default() -> MacVlahtbr {
            MacVlahtbr(0)
        }
    }
    impl core::fmt::Debug for MacVlahtbr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("MacVlahtbr")
                .field("vlht", &self.vlht())
                .finish()
        }
    }
    #[doc = "desc MAC_VTACTLR."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct MacVtactlr(pub u32);
    impl MacVtactlr {
        #[doc = "desc VLANV."]
        #[must_use]
        #[inline(always)]
        pub const fn vlanv(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[doc = "desc VLANV."]
        #[inline(always)]
        pub const fn set_vlanv(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
        }
        #[doc = "desc VLANC."]
        #[must_use]
        #[inline(always)]
        pub const fn vlanc(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x03;
            val as u8
        }
        #[doc = "desc VLANC."]
        #[inline(always)]
        pub const fn set_vlanc(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 16usize)) | (((val as u32) & 0x03) << 16usize);
        }
        #[doc = "desc VLANS."]
        #[must_use]
        #[inline(always)]
        pub const fn vlans(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[doc = "desc VLANS."]
        #[inline(always)]
        pub const fn set_vlans(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
    }
    impl Default for MacVtactlr {
        #[inline(always)]
        fn default() -> MacVtactlr {
            MacVtactlr(0)
        }
    }
    impl core::fmt::Debug for MacVtactlr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("MacVtactlr")
                .field("vlanv", &self.vlanv())
                .field("vlanc", &self.vlanc())
                .field("vlans", &self.vlans())
                .finish()
        }
    }
    #[doc = "desc MAC_VTAFLTR."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct MacVtafltr(pub u32);
    impl MacVtafltr {
        #[doc = "desc VLFLT."]
        #[must_use]
        #[inline(always)]
        pub const fn vlflt(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[doc = "desc VLFLT."]
        #[inline(always)]
        pub const fn set_vlflt(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
        }
        #[doc = "desc VTAL."]
        #[must_use]
        #[inline(always)]
        pub const fn vtal(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "desc VTAL."]
        #[inline(always)]
        pub const fn set_vtal(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "desc VTIM."]
        #[must_use]
        #[inline(always)]
        pub const fn vtim(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "desc VTIM."]
        #[inline(always)]
        pub const fn set_vtim(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[doc = "desc VTHM."]
        #[must_use]
        #[inline(always)]
        pub const fn vthm(&self) -> bool {
            let val = (self.0 >> 19usize) & 0x01;
            val != 0
        }
        #[doc = "desc VTHM."]
        #[inline(always)]
        pub const fn set_vthm(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
        }
    }
    impl Default for MacVtafltr {
        #[inline(always)]
        fn default() -> MacVtafltr {
            MacVtafltr(0)
        }
    }
    impl core::fmt::Debug for MacVtafltr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("MacVtafltr")
                .field("vlflt", &self.vlflt())
                .field("vtal", &self.vtal())
                .field("vtim", &self.vtim())
                .field("vthm", &self.vthm())
                .finish()
        }
    }
    #[doc = "desc MMC_MMCCTLR."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct MmcMmcctlr(pub u32);
    impl MmcMmcctlr {
        #[doc = "desc CRST."]
        #[must_use]
        #[inline(always)]
        pub const fn crst(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "desc CRST."]
        #[inline(always)]
        pub const fn set_crst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "desc COS."]
        #[must_use]
        #[inline(always)]
        pub const fn cos(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "desc COS."]
        #[inline(always)]
        pub const fn set_cos(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "desc ROR."]
        #[must_use]
        #[inline(always)]
        pub const fn ror(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "desc ROR."]
        #[inline(always)]
        pub const fn set_ror(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "desc MCF."]
        #[must_use]
        #[inline(always)]
        pub const fn mcf(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "desc MCF."]
        #[inline(always)]
        pub const fn set_mcf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "desc MCPSET."]
        #[must_use]
        #[inline(always)]
        pub const fn mcpset(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "desc MCPSET."]
        #[inline(always)]
        pub const fn set_mcpset(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "desc MCPSEL."]
        #[must_use]
        #[inline(always)]
        pub const fn mcpsel(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "desc MCPSEL."]
        #[inline(always)]
        pub const fn set_mcpsel(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
    }
    impl Default for MmcMmcctlr {
        #[inline(always)]
        fn default() -> MmcMmcctlr {
            MmcMmcctlr(0)
        }
    }
    impl core::fmt::Debug for MmcMmcctlr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("MmcMmcctlr")
                .field("crst", &self.crst())
                .field("cos", &self.cos())
                .field("ror", &self.ror())
                .field("mcf", &self.mcf())
                .field("mcpset", &self.mcpset())
                .field("mcpsel", &self.mcpsel())
                .finish()
        }
    }
    #[doc = "desc MMC_REVSTSR."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct MmcRevstsr(pub u32);
    impl MmcRevstsr {
        #[doc = "desc RXBGIS."]
        #[must_use]
        #[inline(always)]
        pub const fn rxbgis(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "desc RXBGIS."]
        #[inline(always)]
        pub const fn set_rxbgis(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "desc RXMGIS."]
        #[must_use]
        #[inline(always)]
        pub const fn rxmgis(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "desc RXMGIS."]
        #[inline(always)]
        pub const fn set_rxmgis(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "desc RXCEIS."]
        #[must_use]
        #[inline(always)]
        pub const fn rxceis(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "desc RXCEIS."]
        #[inline(always)]
        pub const fn set_rxceis(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "desc RXAEIS."]
        #[must_use]
        #[inline(always)]
        pub const fn rxaeis(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "desc RXAEIS."]
        #[inline(always)]
        pub const fn set_rxaeis(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "desc RXREIS."]
        #[must_use]
        #[inline(always)]
        pub const fn rxreis(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "desc RXREIS."]
        #[inline(always)]
        pub const fn set_rxreis(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "desc RXUGIS."]
        #[must_use]
        #[inline(always)]
        pub const fn rxugis(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "desc RXUGIS."]
        #[inline(always)]
        pub const fn set_rxugis(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[doc = "desc RXLEIS."]
        #[must_use]
        #[inline(always)]
        pub const fn rxleis(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[doc = "desc RXLEIS."]
        #[inline(always)]
        pub const fn set_rxleis(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
        #[doc = "desc RXOEIS."]
        #[must_use]
        #[inline(always)]
        pub const fn rxoeis(&self) -> bool {
            let val = (self.0 >> 19usize) & 0x01;
            val != 0
        }
        #[doc = "desc RXOEIS."]
        #[inline(always)]
        pub const fn set_rxoeis(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
        }
    }
    impl Default for MmcRevstsr {
        #[inline(always)]
        fn default() -> MmcRevstsr {
            MmcRevstsr(0)
        }
    }
    impl core::fmt::Debug for MmcRevstsr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("MmcRevstsr")
                .field("rxbgis", &self.rxbgis())
                .field("rxmgis", &self.rxmgis())
                .field("rxceis", &self.rxceis())
                .field("rxaeis", &self.rxaeis())
                .field("rxreis", &self.rxreis())
                .field("rxugis", &self.rxugis())
                .field("rxleis", &self.rxleis())
                .field("rxoeis", &self.rxoeis())
                .finish()
        }
    }
    #[doc = "desc MMC_RITCTLR."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct MmcRitctlr(pub u32);
    impl MmcRitctlr {
        #[doc = "desc RXBGIM."]
        #[must_use]
        #[inline(always)]
        pub const fn rxbgim(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "desc RXBGIM."]
        #[inline(always)]
        pub const fn set_rxbgim(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "desc RXMGIM."]
        #[must_use]
        #[inline(always)]
        pub const fn rxmgim(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "desc RXMGIM."]
        #[inline(always)]
        pub const fn set_rxmgim(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "desc RXCEIM."]
        #[must_use]
        #[inline(always)]
        pub const fn rxceim(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "desc RXCEIM."]
        #[inline(always)]
        pub const fn set_rxceim(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "desc RXAEIM."]
        #[must_use]
        #[inline(always)]
        pub const fn rxaeim(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "desc RXAEIM."]
        #[inline(always)]
        pub const fn set_rxaeim(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "desc RXREIM."]
        #[must_use]
        #[inline(always)]
        pub const fn rxreim(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "desc RXREIM."]
        #[inline(always)]
        pub const fn set_rxreim(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "desc RXUGIM."]
        #[must_use]
        #[inline(always)]
        pub const fn rxugim(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "desc RXUGIM."]
        #[inline(always)]
        pub const fn set_rxugim(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[doc = "desc RXLEIM."]
        #[must_use]
        #[inline(always)]
        pub const fn rxleim(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[doc = "desc RXLEIM."]
        #[inline(always)]
        pub const fn set_rxleim(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
        #[doc = "desc RXOEIM."]
        #[must_use]
        #[inline(always)]
        pub const fn rxoeim(&self) -> bool {
            let val = (self.0 >> 19usize) & 0x01;
            val != 0
        }
        #[doc = "desc RXOEIM."]
        #[inline(always)]
        pub const fn set_rxoeim(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
        }
    }
    impl Default for MmcRitctlr {
        #[inline(always)]
        fn default() -> MmcRitctlr {
            MmcRitctlr(0)
        }
    }
    impl core::fmt::Debug for MmcRitctlr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("MmcRitctlr")
                .field("rxbgim", &self.rxbgim())
                .field("rxmgim", &self.rxmgim())
                .field("rxceim", &self.rxceim())
                .field("rxaeim", &self.rxaeim())
                .field("rxreim", &self.rxreim())
                .field("rxugim", &self.rxugim())
                .field("rxleim", &self.rxleim())
                .field("rxoeim", &self.rxoeim())
                .finish()
        }
    }
    #[doc = "desc MMC_RXALEFR."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct MmcRxalefr(pub u32);
    impl MmcRxalefr {
        #[doc = "desc RXALECNT."]
        #[must_use]
        #[inline(always)]
        pub const fn rxalecnt(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[doc = "desc RXALECNT."]
        #[inline(always)]
        pub const fn set_rxalecnt(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
        }
    }
    impl Default for MmcRxalefr {
        #[inline(always)]
        fn default() -> MmcRxalefr {
            MmcRxalefr(0)
        }
    }
    impl core::fmt::Debug for MmcRxalefr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("MmcRxalefr")
                .field("rxalecnt", &self.rxalecnt())
                .finish()
        }
    }
    #[doc = "desc MMC_RXBRGFR."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct MmcRxbrgfr(pub u32);
    impl MmcRxbrgfr {
        #[doc = "desc RXBRGCNT."]
        #[must_use]
        #[inline(always)]
        pub const fn rxbrgcnt(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[doc = "desc RXBRGCNT."]
        #[inline(always)]
        pub const fn set_rxbrgcnt(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
        }
    }
    impl Default for MmcRxbrgfr {
        #[inline(always)]
        fn default() -> MmcRxbrgfr {
            MmcRxbrgfr(0)
        }
    }
    impl core::fmt::Debug for MmcRxbrgfr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("MmcRxbrgfr")
                .field("rxbrgcnt", &self.rxbrgcnt())
                .finish()
        }
    }
    #[doc = "desc MMC_RXCREFR."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct MmcRxcrefr(pub u32);
    impl MmcRxcrefr {
        #[doc = "desc RXCRECNT."]
        #[must_use]
        #[inline(always)]
        pub const fn rxcrecnt(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[doc = "desc RXCRECNT."]
        #[inline(always)]
        pub const fn set_rxcrecnt(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
        }
    }
    impl Default for MmcRxcrefr {
        #[inline(always)]
        fn default() -> MmcRxcrefr {
            MmcRxcrefr(0)
        }
    }
    impl core::fmt::Debug for MmcRxcrefr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("MmcRxcrefr")
                .field("rxcrecnt", &self.rxcrecnt())
                .finish()
        }
    }
    #[doc = "desc MMC_RXLEEFR."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct MmcRxleefr(pub u32);
    impl MmcRxleefr {
        #[doc = "desc RXLEECNT."]
        #[must_use]
        #[inline(always)]
        pub const fn rxleecnt(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[doc = "desc RXLEECNT."]
        #[inline(always)]
        pub const fn set_rxleecnt(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
        }
    }
    impl Default for MmcRxleefr {
        #[inline(always)]
        fn default() -> MmcRxleefr {
            MmcRxleefr(0)
        }
    }
    impl core::fmt::Debug for MmcRxleefr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("MmcRxleefr")
                .field("rxleecnt", &self.rxleecnt())
                .finish()
        }
    }
    #[doc = "desc MMC_RXMUGFR."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct MmcRxmugfr(pub u32);
    impl MmcRxmugfr {
        #[doc = "desc RXMUGCNT."]
        #[must_use]
        #[inline(always)]
        pub const fn rxmugcnt(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[doc = "desc RXMUGCNT."]
        #[inline(always)]
        pub const fn set_rxmugcnt(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
        }
    }
    impl Default for MmcRxmugfr {
        #[inline(always)]
        fn default() -> MmcRxmugfr {
            MmcRxmugfr(0)
        }
    }
    impl core::fmt::Debug for MmcRxmugfr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("MmcRxmugfr")
                .field("rxmugcnt", &self.rxmugcnt())
                .finish()
        }
    }
    #[doc = "desc MMC_RXOREFR."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct MmcRxorefr(pub u32);
    impl MmcRxorefr {
        #[doc = "desc RXORECNT."]
        #[must_use]
        #[inline(always)]
        pub const fn rxorecnt(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[doc = "desc RXORECNT."]
        #[inline(always)]
        pub const fn set_rxorecnt(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
        }
    }
    impl Default for MmcRxorefr {
        #[inline(always)]
        fn default() -> MmcRxorefr {
            MmcRxorefr(0)
        }
    }
    impl core::fmt::Debug for MmcRxorefr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("MmcRxorefr")
                .field("rxorecnt", &self.rxorecnt())
                .finish()
        }
    }
    #[doc = "desc MMC_RXRUEFR."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct MmcRxruefr(pub u32);
    impl MmcRxruefr {
        #[doc = "desc RXRUECNT."]
        #[must_use]
        #[inline(always)]
        pub const fn rxruecnt(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[doc = "desc RXRUECNT."]
        #[inline(always)]
        pub const fn set_rxruecnt(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
        }
    }
    impl Default for MmcRxruefr {
        #[inline(always)]
        fn default() -> MmcRxruefr {
            MmcRxruefr(0)
        }
    }
    impl core::fmt::Debug for MmcRxruefr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("MmcRxruefr")
                .field("rxruecnt", &self.rxruecnt())
                .finish()
        }
    }
    #[doc = "desc MMC_RXUNGFR."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct MmcRxungfr(pub u32);
    impl MmcRxungfr {
        #[doc = "desc RXUNGCNT."]
        #[must_use]
        #[inline(always)]
        pub const fn rxungcnt(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[doc = "desc RXUNGCNT."]
        #[inline(always)]
        pub const fn set_rxungcnt(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
        }
    }
    impl Default for MmcRxungfr {
        #[inline(always)]
        fn default() -> MmcRxungfr {
            MmcRxungfr(0)
        }
    }
    impl core::fmt::Debug for MmcRxungfr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("MmcRxungfr")
                .field("rxungcnt", &self.rxungcnt())
                .finish()
        }
    }
    #[doc = "desc MMC_TITCTLR."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct MmcTitctlr(pub u32);
    impl MmcTitctlr {
        #[doc = "desc TXBGIM."]
        #[must_use]
        #[inline(always)]
        pub const fn txbgim(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "desc TXBGIM."]
        #[inline(always)]
        pub const fn set_txbgim(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "desc TXMGIM."]
        #[must_use]
        #[inline(always)]
        pub const fn txmgim(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "desc TXMGIM."]
        #[inline(always)]
        pub const fn set_txmgim(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "desc TXDEEIM."]
        #[must_use]
        #[inline(always)]
        pub const fn txdeeim(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "desc TXDEEIM."]
        #[inline(always)]
        pub const fn set_txdeeim(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "desc TXLCEIM."]
        #[must_use]
        #[inline(always)]
        pub const fn txlceim(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "desc TXLCEIM."]
        #[inline(always)]
        pub const fn set_txlceim(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[doc = "desc TXECEIM."]
        #[must_use]
        #[inline(always)]
        pub const fn txeceim(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[doc = "desc TXECEIM."]
        #[inline(always)]
        pub const fn set_txeceim(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
        #[doc = "desc TXCAEIM."]
        #[must_use]
        #[inline(always)]
        pub const fn txcaeim(&self) -> bool {
            let val = (self.0 >> 19usize) & 0x01;
            val != 0
        }
        #[doc = "desc TXCAEIM."]
        #[inline(always)]
        pub const fn set_txcaeim(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
        }
        #[doc = "desc TXUGIM."]
        #[must_use]
        #[inline(always)]
        pub const fn txugim(&self) -> bool {
            let val = (self.0 >> 21usize) & 0x01;
            val != 0
        }
        #[doc = "desc TXUGIM."]
        #[inline(always)]
        pub const fn set_txugim(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
        }
        #[doc = "desc TXEDEIM."]
        #[must_use]
        #[inline(always)]
        pub const fn txedeim(&self) -> bool {
            let val = (self.0 >> 22usize) & 0x01;
            val != 0
        }
        #[doc = "desc TXEDEIM."]
        #[inline(always)]
        pub const fn set_txedeim(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
        }
    }
    impl Default for MmcTitctlr {
        #[inline(always)]
        fn default() -> MmcTitctlr {
            MmcTitctlr(0)
        }
    }
    impl core::fmt::Debug for MmcTitctlr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("MmcTitctlr")
                .field("txbgim", &self.txbgim())
                .field("txmgim", &self.txmgim())
                .field("txdeeim", &self.txdeeim())
                .field("txlceim", &self.txlceim())
                .field("txeceim", &self.txeceim())
                .field("txcaeim", &self.txcaeim())
                .field("txugim", &self.txugim())
                .field("txedeim", &self.txedeim())
                .finish()
        }
    }
    #[doc = "desc MMC_TRSSTSR."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct MmcTrsstsr(pub u32);
    impl MmcTrsstsr {
        #[doc = "desc TXBGIS."]
        #[must_use]
        #[inline(always)]
        pub const fn txbgis(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "desc TXBGIS."]
        #[inline(always)]
        pub const fn set_txbgis(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "desc TXMGIS."]
        #[must_use]
        #[inline(always)]
        pub const fn txmgis(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "desc TXMGIS."]
        #[inline(always)]
        pub const fn set_txmgis(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "desc TXDEEIS."]
        #[must_use]
        #[inline(always)]
        pub const fn txdeeis(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "desc TXDEEIS."]
        #[inline(always)]
        pub const fn set_txdeeis(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "desc TXLCEIS."]
        #[must_use]
        #[inline(always)]
        pub const fn txlceis(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "desc TXLCEIS."]
        #[inline(always)]
        pub const fn set_txlceis(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[doc = "desc TXECEIS."]
        #[must_use]
        #[inline(always)]
        pub const fn txeceis(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[doc = "desc TXECEIS."]
        #[inline(always)]
        pub const fn set_txeceis(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
        #[doc = "desc TXCAEIS."]
        #[must_use]
        #[inline(always)]
        pub const fn txcaeis(&self) -> bool {
            let val = (self.0 >> 19usize) & 0x01;
            val != 0
        }
        #[doc = "desc TXCAEIS."]
        #[inline(always)]
        pub const fn set_txcaeis(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
        }
        #[doc = "desc TXUGIS."]
        #[must_use]
        #[inline(always)]
        pub const fn txugis(&self) -> bool {
            let val = (self.0 >> 21usize) & 0x01;
            val != 0
        }
        #[doc = "desc TXUGIS."]
        #[inline(always)]
        pub const fn set_txugis(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
        }
        #[doc = "desc TXEDEIS."]
        #[must_use]
        #[inline(always)]
        pub const fn txedeis(&self) -> bool {
            let val = (self.0 >> 22usize) & 0x01;
            val != 0
        }
        #[doc = "desc TXEDEIS."]
        #[inline(always)]
        pub const fn set_txedeis(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
        }
    }
    impl Default for MmcTrsstsr {
        #[inline(always)]
        fn default() -> MmcTrsstsr {
            MmcTrsstsr(0)
        }
    }
    impl core::fmt::Debug for MmcTrsstsr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("MmcTrsstsr")
                .field("txbgis", &self.txbgis())
                .field("txmgis", &self.txmgis())
                .field("txdeeis", &self.txdeeis())
                .field("txlceis", &self.txlceis())
                .field("txeceis", &self.txeceis())
                .field("txcaeis", &self.txcaeis())
                .field("txugis", &self.txugis())
                .field("txedeis", &self.txedeis())
                .finish()
        }
    }
    #[doc = "desc MMC_TXBRGFR."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct MmcTxbrgfr(pub u32);
    impl MmcTxbrgfr {
        #[doc = "desc TXBRGCNT."]
        #[must_use]
        #[inline(always)]
        pub const fn txbrgcnt(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[doc = "desc TXBRGCNT."]
        #[inline(always)]
        pub const fn set_txbrgcnt(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
        }
    }
    impl Default for MmcTxbrgfr {
        #[inline(always)]
        fn default() -> MmcTxbrgfr {
            MmcTxbrgfr(0)
        }
    }
    impl core::fmt::Debug for MmcTxbrgfr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("MmcTxbrgfr")
                .field("txbrgcnt", &self.txbrgcnt())
                .finish()
        }
    }
    #[doc = "desc MMC_TXCAEFR."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct MmcTxcaefr(pub u32);
    impl MmcTxcaefr {
        #[doc = "desc TXCAECNT."]
        #[must_use]
        #[inline(always)]
        pub const fn txcaecnt(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[doc = "desc TXCAECNT."]
        #[inline(always)]
        pub const fn set_txcaecnt(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
        }
    }
    impl Default for MmcTxcaefr {
        #[inline(always)]
        fn default() -> MmcTxcaefr {
            MmcTxcaefr(0)
        }
    }
    impl core::fmt::Debug for MmcTxcaefr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("MmcTxcaefr")
                .field("txcaecnt", &self.txcaecnt())
                .finish()
        }
    }
    #[doc = "desc MMC_TXDEEFR."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct MmcTxdeefr(pub u32);
    impl MmcTxdeefr {
        #[doc = "desc TXDEECNT."]
        #[must_use]
        #[inline(always)]
        pub const fn txdeecnt(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[doc = "desc TXDEECNT."]
        #[inline(always)]
        pub const fn set_txdeecnt(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
        }
    }
    impl Default for MmcTxdeefr {
        #[inline(always)]
        fn default() -> MmcTxdeefr {
            MmcTxdeefr(0)
        }
    }
    impl core::fmt::Debug for MmcTxdeefr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("MmcTxdeefr")
                .field("txdeecnt", &self.txdeecnt())
                .finish()
        }
    }
    #[doc = "desc MMC_TXECEFR."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct MmcTxecefr(pub u32);
    impl MmcTxecefr {
        #[doc = "desc TXECECNT."]
        #[must_use]
        #[inline(always)]
        pub const fn txececnt(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[doc = "desc TXECECNT."]
        #[inline(always)]
        pub const fn set_txececnt(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
        }
    }
    impl Default for MmcTxecefr {
        #[inline(always)]
        fn default() -> MmcTxecefr {
            MmcTxecefr(0)
        }
    }
    impl core::fmt::Debug for MmcTxecefr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("MmcTxecefr")
                .field("txececnt", &self.txececnt())
                .finish()
        }
    }
    #[doc = "desc MMC_TXEDEFR."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct MmcTxedefr(pub u32);
    impl MmcTxedefr {
        #[doc = "desc TXEDECNT."]
        #[must_use]
        #[inline(always)]
        pub const fn txedecnt(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[doc = "desc TXEDECNT."]
        #[inline(always)]
        pub const fn set_txedecnt(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
        }
    }
    impl Default for MmcTxedefr {
        #[inline(always)]
        fn default() -> MmcTxedefr {
            MmcTxedefr(0)
        }
    }
    impl core::fmt::Debug for MmcTxedefr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("MmcTxedefr")
                .field("txedecnt", &self.txedecnt())
                .finish()
        }
    }
    #[doc = "desc MMC_TXLCEFR."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct MmcTxlcefr(pub u32);
    impl MmcTxlcefr {
        #[doc = "desc TXLCECNT."]
        #[must_use]
        #[inline(always)]
        pub const fn txlcecnt(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[doc = "desc TXLCECNT."]
        #[inline(always)]
        pub const fn set_txlcecnt(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
        }
    }
    impl Default for MmcTxlcefr {
        #[inline(always)]
        fn default() -> MmcTxlcefr {
            MmcTxlcefr(0)
        }
    }
    impl core::fmt::Debug for MmcTxlcefr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("MmcTxlcefr")
                .field("txlcecnt", &self.txlcecnt())
                .finish()
        }
    }
    #[doc = "desc MMC_TXMUGFR."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct MmcTxmugfr(pub u32);
    impl MmcTxmugfr {
        #[doc = "desc TXMUGCNT."]
        #[must_use]
        #[inline(always)]
        pub const fn txmugcnt(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[doc = "desc TXMUGCNT."]
        #[inline(always)]
        pub const fn set_txmugcnt(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
        }
    }
    impl Default for MmcTxmugfr {
        #[inline(always)]
        fn default() -> MmcTxmugfr {
            MmcTxmugfr(0)
        }
    }
    impl core::fmt::Debug for MmcTxmugfr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("MmcTxmugfr")
                .field("txmugcnt", &self.txmugcnt())
                .finish()
        }
    }
    #[doc = "desc MMC_TXUNGFR."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct MmcTxungfr(pub u32);
    impl MmcTxungfr {
        #[doc = "desc TXUNGCNT."]
        #[must_use]
        #[inline(always)]
        pub const fn txungcnt(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[doc = "desc TXUNGCNT."]
        #[inline(always)]
        pub const fn set_txungcnt(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
        }
    }
    impl Default for MmcTxungfr {
        #[inline(always)]
        fn default() -> MmcTxungfr {
            MmcTxungfr(0)
        }
    }
    impl core::fmt::Debug for MmcTxungfr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("MmcTxungfr")
                .field("txungcnt", &self.txungcnt())
                .finish()
        }
    }
    #[doc = "desc PTP_PPSCTLR."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct PtpPpsctlr(pub u32);
    impl PtpPpsctlr {
        #[doc = "desc PPSFRE0."]
        #[must_use]
        #[inline(always)]
        pub const fn ppsfre0(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x0f;
            val as u8
        }
        #[doc = "desc PPSFRE0."]
        #[inline(always)]
        pub const fn set_ppsfre0(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
        }
        #[doc = "desc PPSOMD."]
        #[must_use]
        #[inline(always)]
        pub const fn ppsomd(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "desc PPSOMD."]
        #[inline(always)]
        pub const fn set_ppsomd(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "desc TT0SEL."]
        #[must_use]
        #[inline(always)]
        pub const fn tt0sel(&self) -> u8 {
            let val = (self.0 >> 5usize) & 0x03;
            val as u8
        }
        #[doc = "desc TT0SEL."]
        #[inline(always)]
        pub const fn set_tt0sel(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 5usize)) | (((val as u32) & 0x03) << 5usize);
        }
        #[doc = "desc PPSFRE1."]
        #[must_use]
        #[inline(always)]
        pub const fn ppsfre1(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0x07;
            val as u8
        }
        #[doc = "desc PPSFRE1."]
        #[inline(always)]
        pub const fn set_ppsfre1(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 8usize)) | (((val as u32) & 0x07) << 8usize);
        }
        #[doc = "desc TT1SEL."]
        #[must_use]
        #[inline(always)]
        pub const fn tt1sel(&self) -> u8 {
            let val = (self.0 >> 13usize) & 0x03;
            val as u8
        }
        #[doc = "desc TT1SEL."]
        #[inline(always)]
        pub const fn set_tt1sel(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 13usize)) | (((val as u32) & 0x03) << 13usize);
        }
    }
    impl Default for PtpPpsctlr {
        #[inline(always)]
        fn default() -> PtpPpsctlr {
            PtpPpsctlr(0)
        }
    }
    impl core::fmt::Debug for PtpPpsctlr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("PtpPpsctlr")
                .field("ppsfre0", &self.ppsfre0())
                .field("ppsomd", &self.ppsomd())
                .field("tt0sel", &self.tt0sel())
                .field("ppsfre1", &self.ppsfre1())
                .field("tt1sel", &self.tt1sel())
                .finish()
        }
    }
    #[doc = "desc PTP_TMSNSER."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct PtpTmsnser(pub u32);
    impl PtpTmsnser {
        #[doc = "desc TSPSYSNSEC."]
        #[must_use]
        #[inline(always)]
        pub const fn tspsysnsec(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0x7fff_ffff;
            val as u32
        }
        #[doc = "desc TSPSYSNSEC."]
        #[inline(always)]
        pub const fn set_tspsysnsec(&mut self, val: u32) {
            self.0 = (self.0 & !(0x7fff_ffff << 0usize)) | (((val as u32) & 0x7fff_ffff) << 0usize);
        }
    }
    impl Default for PtpTmsnser {
        #[inline(always)]
        fn default() -> PtpTmsnser {
            PtpTmsnser(0)
        }
    }
    impl core::fmt::Debug for PtpTmsnser {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("PtpTmsnser")
                .field("tspsysnsec", &self.tspsysnsec())
                .finish()
        }
    }
    #[doc = "desc PTP_TMSSECR."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct PtpTmssecr(pub u32);
    impl PtpTmssecr {
        #[doc = "desc TSPSYSSEC."]
        #[must_use]
        #[inline(always)]
        pub const fn tspsyssec(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "desc TSPSYSSEC."]
        #[inline(always)]
        pub const fn set_tspsyssec(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for PtpTmssecr {
        #[inline(always)]
        fn default() -> PtpTmssecr {
            PtpTmssecr(0)
        }
    }
    impl core::fmt::Debug for PtpTmssecr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("PtpTmssecr")
                .field("tspsyssec", &self.tspsyssec())
                .finish()
        }
    }
    #[doc = "desc PTP_TMTNSER0."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct PtpTmtnser0(pub u32);
    impl PtpTmtnser0 {
        #[doc = "desc TSPTAGNSEC0."]
        #[must_use]
        #[inline(always)]
        pub const fn tsptagnsec0(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0x7fff_ffff;
            val as u32
        }
        #[doc = "desc TSPTAGNSEC0."]
        #[inline(always)]
        pub const fn set_tsptagnsec0(&mut self, val: u32) {
            self.0 = (self.0 & !(0x7fff_ffff << 0usize)) | (((val as u32) & 0x7fff_ffff) << 0usize);
        }
    }
    impl Default for PtpTmtnser0 {
        #[inline(always)]
        fn default() -> PtpTmtnser0 {
            PtpTmtnser0(0)
        }
    }
    impl core::fmt::Debug for PtpTmtnser0 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("PtpTmtnser0")
                .field("tsptagnsec0", &self.tsptagnsec0())
                .finish()
        }
    }
    #[doc = "desc PTP_TMTNSER1."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct PtpTmtnser1(pub u32);
    impl PtpTmtnser1 {
        #[doc = "desc TSPTAGNSEC1."]
        #[must_use]
        #[inline(always)]
        pub const fn tsptagnsec1(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0x7fff_ffff;
            val as u32
        }
        #[doc = "desc TSPTAGNSEC1."]
        #[inline(always)]
        pub const fn set_tsptagnsec1(&mut self, val: u32) {
            self.0 = (self.0 & !(0x7fff_ffff << 0usize)) | (((val as u32) & 0x7fff_ffff) << 0usize);
        }
    }
    impl Default for PtpTmtnser1 {
        #[inline(always)]
        fn default() -> PtpTmtnser1 {
            PtpTmtnser1(0)
        }
    }
    impl core::fmt::Debug for PtpTmtnser1 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("PtpTmtnser1")
                .field("tsptagnsec1", &self.tsptagnsec1())
                .finish()
        }
    }
    #[doc = "desc PTP_TMTSECR0."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct PtpTmtsecr0(pub u32);
    impl PtpTmtsecr0 {
        #[doc = "desc TSPTAGSEC0."]
        #[must_use]
        #[inline(always)]
        pub const fn tsptagsec0(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "desc TSPTAGSEC0."]
        #[inline(always)]
        pub const fn set_tsptagsec0(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for PtpTmtsecr0 {
        #[inline(always)]
        fn default() -> PtpTmtsecr0 {
            PtpTmtsecr0(0)
        }
    }
    impl core::fmt::Debug for PtpTmtsecr0 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("PtpTmtsecr0")
                .field("tsptagsec0", &self.tsptagsec0())
                .finish()
        }
    }
    #[doc = "desc PTP_TMTSECR1."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct PtpTmtsecr1(pub u32);
    impl PtpTmtsecr1 {
        #[doc = "desc TSPTAGSEC1."]
        #[must_use]
        #[inline(always)]
        pub const fn tsptagsec1(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "desc TSPTAGSEC1."]
        #[inline(always)]
        pub const fn set_tsptagsec1(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for PtpTmtsecr1 {
        #[inline(always)]
        fn default() -> PtpTmtsecr1 {
            PtpTmtsecr1(0)
        }
    }
    impl core::fmt::Debug for PtpTmtsecr1 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("PtpTmtsecr1")
                .field("tsptagsec1", &self.tsptagsec1())
                .finish()
        }
    }
    #[doc = "desc PTP_TMUNSER."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct PtpTmunser(pub u32);
    impl PtpTmunser {
        #[doc = "desc TSPUPNSEC."]
        #[must_use]
        #[inline(always)]
        pub const fn tspupnsec(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0x7fff_ffff;
            val as u32
        }
        #[doc = "desc TSPUPNSEC."]
        #[inline(always)]
        pub const fn set_tspupnsec(&mut self, val: u32) {
            self.0 = (self.0 & !(0x7fff_ffff << 0usize)) | (((val as u32) & 0x7fff_ffff) << 0usize);
        }
        #[doc = "desc TSPUPNS."]
        #[must_use]
        #[inline(always)]
        pub const fn tspupns(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[doc = "desc TSPUPNS."]
        #[inline(always)]
        pub const fn set_tspupns(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for PtpTmunser {
        #[inline(always)]
        fn default() -> PtpTmunser {
            PtpTmunser(0)
        }
    }
    impl core::fmt::Debug for PtpTmunser {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("PtpTmunser")
                .field("tspupnsec", &self.tspupnsec())
                .field("tspupns", &self.tspupns())
                .finish()
        }
    }
    #[doc = "desc PTP_TMUSECR."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct PtpTmusecr(pub u32);
    impl PtpTmusecr {
        #[doc = "desc TSPUPSEC."]
        #[must_use]
        #[inline(always)]
        pub const fn tspupsec(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "desc TSPUPSEC."]
        #[inline(always)]
        pub const fn set_tspupsec(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for PtpTmusecr {
        #[inline(always)]
        fn default() -> PtpTmusecr {
            PtpTmusecr(0)
        }
    }
    impl core::fmt::Debug for PtpTmusecr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("PtpTmusecr")
                .field("tspupsec", &self.tspupsec())
                .finish()
        }
    }
    #[doc = "desc PTP_TSPADDR."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct PtpTspaddr(pub u32);
    impl PtpTspaddr {
        #[doc = "desc TSPADD."]
        #[must_use]
        #[inline(always)]
        pub const fn tspadd(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "desc TSPADD."]
        #[inline(always)]
        pub const fn set_tspadd(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for PtpTspaddr {
        #[inline(always)]
        fn default() -> PtpTspaddr {
            PtpTspaddr(0)
        }
    }
    impl core::fmt::Debug for PtpTspaddr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("PtpTspaddr")
                .field("tspadd", &self.tspadd())
                .finish()
        }
    }
    #[doc = "desc PTP_TSPCTLR."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct PtpTspctlr(pub u32);
    impl PtpTspctlr {
        #[doc = "desc TSPEN."]
        #[must_use]
        #[inline(always)]
        pub const fn tspen(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "desc TSPEN."]
        #[inline(always)]
        pub const fn set_tspen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "desc TSPUPSEL."]
        #[must_use]
        #[inline(always)]
        pub const fn tspupsel(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "desc TSPUPSEL."]
        #[inline(always)]
        pub const fn set_tspupsel(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "desc TSPINI."]
        #[must_use]
        #[inline(always)]
        pub const fn tspini(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "desc TSPINI."]
        #[inline(always)]
        pub const fn set_tspini(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "desc TSPUP."]
        #[must_use]
        #[inline(always)]
        pub const fn tspup(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "desc TSPUP."]
        #[inline(always)]
        pub const fn set_tspup(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "desc TSPINT."]
        #[must_use]
        #[inline(always)]
        pub const fn tspint(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "desc TSPINT."]
        #[inline(always)]
        pub const fn set_tspint(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "desc TSPADUP."]
        #[must_use]
        #[inline(always)]
        pub const fn tspadup(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "desc TSPADUP."]
        #[inline(always)]
        pub const fn set_tspadup(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "desc TSPEALL."]
        #[must_use]
        #[inline(always)]
        pub const fn tspeall(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "desc TSPEALL."]
        #[inline(always)]
        pub const fn set_tspeall(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "desc TSPSSR."]
        #[must_use]
        #[inline(always)]
        pub const fn tspssr(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "desc TSPSSR."]
        #[inline(always)]
        pub const fn set_tspssr(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "desc TSPVER."]
        #[must_use]
        #[inline(always)]
        pub const fn tspver(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "desc TSPVER."]
        #[inline(always)]
        pub const fn set_tspver(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[doc = "desc TSPOVETH."]
        #[must_use]
        #[inline(always)]
        pub const fn tspoveth(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "desc TSPOVETH."]
        #[inline(always)]
        pub const fn set_tspoveth(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[doc = "desc TSPOVIPV6."]
        #[must_use]
        #[inline(always)]
        pub const fn tspovipv6(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "desc TSPOVIPV6."]
        #[inline(always)]
        pub const fn set_tspovipv6(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "desc TSPOVIPV4."]
        #[must_use]
        #[inline(always)]
        pub const fn tspovipv4(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[doc = "desc TSPOVIPV4."]
        #[inline(always)]
        pub const fn set_tspovipv4(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[doc = "desc TSPMTSEL."]
        #[must_use]
        #[inline(always)]
        pub const fn tspmtsel(&self) -> u8 {
            let val = (self.0 >> 14usize) & 0x0f;
            val as u8
        }
        #[doc = "desc TSPMTSEL."]
        #[inline(always)]
        pub const fn set_tspmtsel(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 14usize)) | (((val as u32) & 0x0f) << 14usize);
        }
        #[doc = "desc TSPADF."]
        #[must_use]
        #[inline(always)]
        pub const fn tspadf(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[doc = "desc TSPADF."]
        #[inline(always)]
        pub const fn set_tspadf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
    }
    impl Default for PtpTspctlr {
        #[inline(always)]
        fn default() -> PtpTspctlr {
            PtpTspctlr(0)
        }
    }
    impl core::fmt::Debug for PtpTspctlr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("PtpTspctlr")
                .field("tspen", &self.tspen())
                .field("tspupsel", &self.tspupsel())
                .field("tspini", &self.tspini())
                .field("tspup", &self.tspup())
                .field("tspint", &self.tspint())
                .field("tspadup", &self.tspadup())
                .field("tspeall", &self.tspeall())
                .field("tspssr", &self.tspssr())
                .field("tspver", &self.tspver())
                .field("tspoveth", &self.tspoveth())
                .field("tspovipv6", &self.tspovipv6())
                .field("tspovipv4", &self.tspovipv4())
                .field("tspmtsel", &self.tspmtsel())
                .field("tspadf", &self.tspadf())
                .finish()
        }
    }
    #[doc = "desc PTP_TSPNSAR."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct PtpTspnsar(pub u32);
    impl PtpTspnsar {
        #[doc = "desc TSPNSEADD."]
        #[must_use]
        #[inline(always)]
        pub const fn tspnseadd(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[doc = "desc TSPNSEADD."]
        #[inline(always)]
        pub const fn set_tspnseadd(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
        }
    }
    impl Default for PtpTspnsar {
        #[inline(always)]
        fn default() -> PtpTspnsar {
            PtpTspnsar(0)
        }
    }
    impl core::fmt::Debug for PtpTspnsar {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("PtpTspnsar")
                .field("tspnseadd", &self.tspnseadd())
                .finish()
        }
    }
    #[doc = "desc PTP_TSPSTSR."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct PtpTspstsr(pub u32);
    impl PtpTspstsr {
        #[doc = "desc TSOVF."]
        #[must_use]
        #[inline(always)]
        pub const fn tsovf(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "desc TSOVF."]
        #[inline(always)]
        pub const fn set_tsovf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "desc TSTAR0."]
        #[must_use]
        #[inline(always)]
        pub const fn tstar0(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "desc TSTAR0."]
        #[inline(always)]
        pub const fn set_tstar0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "desc TSERR0."]
        #[must_use]
        #[inline(always)]
        pub const fn tserr0(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "desc TSERR0."]
        #[inline(always)]
        pub const fn set_tserr0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "desc TSTAR1."]
        #[must_use]
        #[inline(always)]
        pub const fn tstar1(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "desc TSTAR1."]
        #[inline(always)]
        pub const fn set_tstar1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "desc TSERR1."]
        #[must_use]
        #[inline(always)]
        pub const fn tserr1(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "desc TSERR1."]
        #[inline(always)]
        pub const fn set_tserr1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
    }
    impl Default for PtpTspstsr {
        #[inline(always)]
        fn default() -> PtpTspstsr {
            PtpTspstsr(0)
        }
    }
    impl core::fmt::Debug for PtpTspstsr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("PtpTspstsr")
                .field("tsovf", &self.tsovf())
                .field("tstar0", &self.tstar0())
                .field("tserr0", &self.tserr0())
                .field("tstar1", &self.tstar1())
                .field("tserr1", &self.tserr1())
                .finish()
        }
    }
}
