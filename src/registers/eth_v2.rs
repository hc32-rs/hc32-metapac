
                    use crate::metadata::ir::*;
                    pub(crate) static REGISTERS: IR = IR {
    blocks: &[
        Block {
            name: "Eth",
            extends: None,
            description: Some(
                "desc ETH.",
            ),
            items: &[
                BlockItem {
                    name: "mac_configr",
                    description: Some(
                        "desc MAC_CONFIGR.",
                    ),
                    array: None,
                    byte_offset: 0x0,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "MacConfigr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "mac_fltctlr",
                    description: Some(
                        "desc MAC_FLTCTLR.",
                    ),
                    array: None,
                    byte_offset: 0x4,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "MacFltctlr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "mac_hashthr",
                    description: Some(
                        "desc MAC_HASHTHR.",
                    ),
                    array: None,
                    byte_offset: 0x8,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: None,
                        },
                    ),
                },
                BlockItem {
                    name: "mac_hashtlr",
                    description: Some(
                        "desc MAC_HASHTLR.",
                    ),
                    array: None,
                    byte_offset: 0xc,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: None,
                        },
                    ),
                },
                BlockItem {
                    name: "mac_smiaddr",
                    description: Some(
                        "desc MAC_SMIADDR.",
                    ),
                    array: None,
                    byte_offset: 0x10,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "MacSmiaddr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "mac_smidatr",
                    description: Some(
                        "desc MAC_SMIDATR.",
                    ),
                    array: None,
                    byte_offset: 0x14,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "MacSmidatr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "mac_floctlr",
                    description: Some(
                        "desc MAC_FLOCTLR.",
                    ),
                    array: None,
                    byte_offset: 0x18,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "MacFloctlr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "mac_vtafltr",
                    description: Some(
                        "desc MAC_VTAFLTR.",
                    ),
                    array: None,
                    byte_offset: 0x1c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "MacVtafltr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "mac_macstsr",
                    description: Some(
                        "desc MAC_MACSTSR.",
                    ),
                    array: None,
                    byte_offset: 0x24,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::Read,
                            bit_size: 32,
                            fieldset: Some(
                                "MacMacstsr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "mac_rtwkffr",
                    description: Some(
                        "desc MAC_RTWKFFR.",
                    ),
                    array: None,
                    byte_offset: 0x28,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: None,
                        },
                    ),
                },
                BlockItem {
                    name: "mac_pmtctlr",
                    description: Some(
                        "desc MAC_PMTCTLR.",
                    ),
                    array: None,
                    byte_offset: 0x2c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "MacPmtctlr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "mac_intstsr",
                    description: Some(
                        "desc MAC_INTSTSR.",
                    ),
                    array: None,
                    byte_offset: 0x38,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::Read,
                            bit_size: 32,
                            fieldset: Some(
                                "MacIntstsr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "mac_intmskr",
                    description: Some(
                        "desc MAC_INTMSKR.",
                    ),
                    array: None,
                    byte_offset: 0x3c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "MacIntmskr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "mac_macadhr0",
                    description: Some(
                        "desc MAC_MACADHR0.",
                    ),
                    array: None,
                    byte_offset: 0x40,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "MacMacadhr0",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "mac_macadlr0",
                    description: Some(
                        "desc MAC_MACADLR0.",
                    ),
                    array: None,
                    byte_offset: 0x44,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: None,
                        },
                    ),
                },
                BlockItem {
                    name: "mac_macadhr1",
                    description: Some(
                        "desc MAC_MACADHR1.",
                    ),
                    array: None,
                    byte_offset: 0x48,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "MacMacadhr1",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "mac_macadlr1",
                    description: Some(
                        "desc MAC_MACADLR1.",
                    ),
                    array: None,
                    byte_offset: 0x4c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: None,
                        },
                    ),
                },
                BlockItem {
                    name: "mac_macadhr2",
                    description: Some(
                        "desc MAC_MACADHR2.",
                    ),
                    array: None,
                    byte_offset: 0x50,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "MacMacadhr2",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "mac_macadlr2",
                    description: Some(
                        "desc MAC_MACADLR2.",
                    ),
                    array: None,
                    byte_offset: 0x54,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: None,
                        },
                    ),
                },
                BlockItem {
                    name: "mac_macadhr3",
                    description: Some(
                        "desc MAC_MACADHR3.",
                    ),
                    array: None,
                    byte_offset: 0x58,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "MacMacadhr3",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "mac_macadlr3",
                    description: Some(
                        "desc MAC_MACADLR3.",
                    ),
                    array: None,
                    byte_offset: 0x5c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: None,
                        },
                    ),
                },
                BlockItem {
                    name: "mac_macadhr4",
                    description: Some(
                        "desc MAC_MACADHR4.",
                    ),
                    array: None,
                    byte_offset: 0x60,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "MacMacadhr4",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "mac_macadlr4",
                    description: Some(
                        "desc MAC_MACADLR4.",
                    ),
                    array: None,
                    byte_offset: 0x64,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: None,
                        },
                    ),
                },
                BlockItem {
                    name: "mmc_mmcctlr",
                    description: Some(
                        "desc MMC_MMCCTLR.",
                    ),
                    array: None,
                    byte_offset: 0x100,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "MmcMmcctlr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "mmc_revstsr",
                    description: Some(
                        "desc MMC_REVSTSR.",
                    ),
                    array: None,
                    byte_offset: 0x104,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::Read,
                            bit_size: 32,
                            fieldset: Some(
                                "MmcRevstsr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "mmc_trsstsr",
                    description: Some(
                        "desc MMC_TRSSTSR.",
                    ),
                    array: None,
                    byte_offset: 0x108,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::Read,
                            bit_size: 32,
                            fieldset: Some(
                                "MmcTrsstsr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "mmc_ritctlr",
                    description: Some(
                        "desc MMC_RITCTLR.",
                    ),
                    array: None,
                    byte_offset: 0x10c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "MmcRitctlr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "mmc_titctlr",
                    description: Some(
                        "desc MMC_TITCTLR.",
                    ),
                    array: None,
                    byte_offset: 0x110,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "MmcTitctlr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "mmc_txbrgfr",
                    description: Some(
                        "desc MMC_TXBRGFR.",
                    ),
                    array: None,
                    byte_offset: 0x11c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::Read,
                            bit_size: 32,
                            fieldset: Some(
                                "MmcTxbrgfr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "mmc_txmugfr",
                    description: Some(
                        "desc MMC_TXMUGFR.",
                    ),
                    array: None,
                    byte_offset: 0x120,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::Read,
                            bit_size: 32,
                            fieldset: Some(
                                "MmcTxmugfr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "mmc_txdeefr",
                    description: Some(
                        "desc MMC_TXDEEFR.",
                    ),
                    array: None,
                    byte_offset: 0x154,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::Read,
                            bit_size: 32,
                            fieldset: Some(
                                "MmcTxdeefr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "mmc_txlcefr",
                    description: Some(
                        "desc MMC_TXLCEFR.",
                    ),
                    array: None,
                    byte_offset: 0x158,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::Read,
                            bit_size: 32,
                            fieldset: Some(
                                "MmcTxlcefr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "mmc_txecefr",
                    description: Some(
                        "desc MMC_TXECEFR.",
                    ),
                    array: None,
                    byte_offset: 0x15c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::Read,
                            bit_size: 32,
                            fieldset: Some(
                                "MmcTxecefr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "mmc_txcaefr",
                    description: Some(
                        "desc MMC_TXCAEFR.",
                    ),
                    array: None,
                    byte_offset: 0x160,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::Read,
                            bit_size: 32,
                            fieldset: Some(
                                "MmcTxcaefr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "mmc_txungfr",
                    description: Some(
                        "desc MMC_TXUNGFR.",
                    ),
                    array: None,
                    byte_offset: 0x168,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::Read,
                            bit_size: 32,
                            fieldset: Some(
                                "MmcTxungfr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "mmc_txedefr",
                    description: Some(
                        "desc MMC_TXEDEFR.",
                    ),
                    array: None,
                    byte_offset: 0x16c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::Read,
                            bit_size: 32,
                            fieldset: Some(
                                "MmcTxedefr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "mmc_rxbrgfr",
                    description: Some(
                        "desc MMC_RXBRGFR.",
                    ),
                    array: None,
                    byte_offset: 0x18c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::Read,
                            bit_size: 32,
                            fieldset: Some(
                                "MmcRxbrgfr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "mmc_rxmugfr",
                    description: Some(
                        "desc MMC_RXMUGFR.",
                    ),
                    array: None,
                    byte_offset: 0x190,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::Read,
                            bit_size: 32,
                            fieldset: Some(
                                "MmcRxmugfr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "mmc_rxcrefr",
                    description: Some(
                        "desc MMC_RXCREFR.",
                    ),
                    array: None,
                    byte_offset: 0x194,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::Read,
                            bit_size: 32,
                            fieldset: Some(
                                "MmcRxcrefr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "mmc_rxalefr",
                    description: Some(
                        "desc MMC_RXALEFR.",
                    ),
                    array: None,
                    byte_offset: 0x198,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::Read,
                            bit_size: 32,
                            fieldset: Some(
                                "MmcRxalefr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "mmc_rxruefr",
                    description: Some(
                        "desc MMC_RXRUEFR.",
                    ),
                    array: None,
                    byte_offset: 0x19c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::Read,
                            bit_size: 32,
                            fieldset: Some(
                                "MmcRxruefr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "mmc_rxungfr",
                    description: Some(
                        "desc MMC_RXUNGFR.",
                    ),
                    array: None,
                    byte_offset: 0x1c4,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::Read,
                            bit_size: 32,
                            fieldset: Some(
                                "MmcRxungfr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "mmc_rxleefr",
                    description: Some(
                        "desc MMC_RXLEEFR.",
                    ),
                    array: None,
                    byte_offset: 0x1c8,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::Read,
                            bit_size: 32,
                            fieldset: Some(
                                "MmcRxleefr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "mmc_rxorefr",
                    description: Some(
                        "desc MMC_RXOREFR.",
                    ),
                    array: None,
                    byte_offset: 0x1cc,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::Read,
                            bit_size: 32,
                            fieldset: Some(
                                "MmcRxorefr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "mac_l34ctlr",
                    description: Some(
                        "desc MAC_L34CTLR.",
                    ),
                    array: None,
                    byte_offset: 0x400,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "MacL34ctlr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "mac_l4portr",
                    description: Some(
                        "desc MAC_L4PORTR.",
                    ),
                    array: None,
                    byte_offset: 0x404,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "MacL4portr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "mac_l3addrr0",
                    description: Some(
                        "desc MAC_L3ADDRR0.",
                    ),
                    array: None,
                    byte_offset: 0x410,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: None,
                        },
                    ),
                },
                BlockItem {
                    name: "mac_l3addrr1",
                    description: Some(
                        "desc MAC_L3ADDRR1.",
                    ),
                    array: None,
                    byte_offset: 0x414,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: None,
                        },
                    ),
                },
                BlockItem {
                    name: "mac_l3addrr2",
                    description: Some(
                        "desc MAC_L3ADDRR2.",
                    ),
                    array: None,
                    byte_offset: 0x418,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: None,
                        },
                    ),
                },
                BlockItem {
                    name: "mac_l3addrr3",
                    description: Some(
                        "desc MAC_L3ADDRR3.",
                    ),
                    array: None,
                    byte_offset: 0x41c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: None,
                        },
                    ),
                },
                BlockItem {
                    name: "mac_vtactlr",
                    description: Some(
                        "desc MAC_VTACTLR.",
                    ),
                    array: None,
                    byte_offset: 0x584,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "MacVtactlr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "mac_vlahtbr",
                    description: Some(
                        "desc MAC_VLAHTBR.",
                    ),
                    array: None,
                    byte_offset: 0x588,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "MacVlahtbr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "ptp_tspctlr",
                    description: Some(
                        "desc PTP_TSPCTLR.",
                    ),
                    array: None,
                    byte_offset: 0x700,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "PtpTspctlr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "ptp_tspnsar",
                    description: Some(
                        "desc PTP_TSPNSAR.",
                    ),
                    array: None,
                    byte_offset: 0x704,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "PtpTspnsar",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "ptp_tmssecr",
                    description: Some(
                        "desc PTP_TMSSECR.",
                    ),
                    array: None,
                    byte_offset: 0x708,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::Read,
                            bit_size: 32,
                            fieldset: None,
                        },
                    ),
                },
                BlockItem {
                    name: "ptp_tmsnser",
                    description: Some(
                        "desc PTP_TMSNSER.",
                    ),
                    array: None,
                    byte_offset: 0x70c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::Read,
                            bit_size: 32,
                            fieldset: Some(
                                "PtpTmsnser",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "ptp_tmusecr",
                    description: Some(
                        "desc PTP_TMUSECR.",
                    ),
                    array: None,
                    byte_offset: 0x710,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: None,
                        },
                    ),
                },
                BlockItem {
                    name: "ptp_tmunser",
                    description: Some(
                        "desc PTP_TMUNSER.",
                    ),
                    array: None,
                    byte_offset: 0x714,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "PtpTmunser",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "ptp_tspaddr",
                    description: Some(
                        "desc PTP_TSPADDR.",
                    ),
                    array: None,
                    byte_offset: 0x718,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: None,
                        },
                    ),
                },
                BlockItem {
                    name: "ptp_tmtsecr0",
                    description: Some(
                        "desc PTP_TMTSECR0.",
                    ),
                    array: None,
                    byte_offset: 0x71c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: None,
                        },
                    ),
                },
                BlockItem {
                    name: "ptp_tmtnser0",
                    description: Some(
                        "desc PTP_TMTNSER0.",
                    ),
                    array: None,
                    byte_offset: 0x720,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "PtpTmtnser0",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "ptp_tspstsr",
                    description: Some(
                        "desc PTP_TSPSTSR.",
                    ),
                    array: None,
                    byte_offset: 0x728,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::Read,
                            bit_size: 32,
                            fieldset: Some(
                                "PtpTspstsr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "ptp_ppsctlr",
                    description: Some(
                        "desc PTP_PPSCTLR.",
                    ),
                    array: None,
                    byte_offset: 0x72c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "PtpPpsctlr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "ptp_tmtsecr1",
                    description: Some(
                        "desc PTP_TMTSECR1.",
                    ),
                    array: None,
                    byte_offset: 0x780,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: None,
                        },
                    ),
                },
                BlockItem {
                    name: "ptp_tmtnser1",
                    description: Some(
                        "desc PTP_TMTNSER1.",
                    ),
                    array: None,
                    byte_offset: 0x784,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "PtpTmtnser1",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "dma_busmodr",
                    description: Some(
                        "desc DMA_BUSMODR.",
                    ),
                    array: None,
                    byte_offset: 0x1000,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "DmaBusmodr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "dma_txpollr",
                    description: Some(
                        "desc DMA_TXPOLLR.",
                    ),
                    array: None,
                    byte_offset: 0x1004,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: None,
                        },
                    ),
                },
                BlockItem {
                    name: "dma_rxpollr",
                    description: Some(
                        "desc DMA_RXPOLLR.",
                    ),
                    array: None,
                    byte_offset: 0x1008,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: None,
                        },
                    ),
                },
                BlockItem {
                    name: "dma_rxdladr",
                    description: Some(
                        "desc DMA_RXDLADR.",
                    ),
                    array: None,
                    byte_offset: 0x100c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: None,
                        },
                    ),
                },
                BlockItem {
                    name: "dma_txdladr",
                    description: Some(
                        "desc DMA_TXDLADR.",
                    ),
                    array: None,
                    byte_offset: 0x1010,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: None,
                        },
                    ),
                },
                BlockItem {
                    name: "dma_dmastsr",
                    description: Some(
                        "desc DMA_DMASTSR.",
                    ),
                    array: None,
                    byte_offset: 0x1014,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "DmaDmastsr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "dma_oprmodr",
                    description: Some(
                        "desc DMA_OPRMODR.",
                    ),
                    array: None,
                    byte_offset: 0x1018,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "DmaOprmodr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "dma_intenar",
                    description: Some(
                        "desc DMA_INTENAR.",
                    ),
                    array: None,
                    byte_offset: 0x101c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "DmaIntenar",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "dma_rfrcntr",
                    description: Some(
                        "desc DMA_RFRCNTR.",
                    ),
                    array: None,
                    byte_offset: 0x1020,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::Read,
                            bit_size: 32,
                            fieldset: Some(
                                "DmaRfrcntr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "dma_revwdtr",
                    description: Some(
                        "desc DMA_REVWDTR.",
                    ),
                    array: None,
                    byte_offset: 0x1024,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "DmaRevwdtr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "dma_chtxder",
                    description: Some(
                        "desc DMA_CHTXDER.",
                    ),
                    array: None,
                    byte_offset: 0x1048,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::Read,
                            bit_size: 32,
                            fieldset: None,
                        },
                    ),
                },
                BlockItem {
                    name: "dma_chrxder",
                    description: Some(
                        "desc DMA_CHRXDER.",
                    ),
                    array: None,
                    byte_offset: 0x104c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::Read,
                            bit_size: 32,
                            fieldset: None,
                        },
                    ),
                },
                BlockItem {
                    name: "dma_chtxbfr",
                    description: Some(
                        "desc DMA_CHTXBFR.",
                    ),
                    array: None,
                    byte_offset: 0x1050,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::Read,
                            bit_size: 32,
                            fieldset: None,
                        },
                    ),
                },
                BlockItem {
                    name: "dma_chrxbfr",
                    description: Some(
                        "desc DMA_CHRXBFR.",
                    ),
                    array: None,
                    byte_offset: 0x1054,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::Read,
                            bit_size: 32,
                            fieldset: None,
                        },
                    ),
                },
            ],
        },
    ],
    fieldsets: &[
        FieldSet {
            name: "DmaBusmodr",
            extends: None,
            description: Some(
                "desc DMA_BUSMODR.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "swr",
                    description: Some(
                        "desc SWR.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "dmaa",
                    description: Some(
                        "desc DMAA.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 1,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "dsl",
                    description: Some(
                        "desc DSL.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 2,
                        },
                    ),
                    bit_size: 5,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "dsen",
                    description: Some(
                        "desc DSEN.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 7,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "tpbl",
                    description: Some(
                        "desc TPBL.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 8,
                        },
                    ),
                    bit_size: 6,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "prat",
                    description: Some(
                        "desc PRAT.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 14,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "fbst",
                    description: Some(
                        "desc FBST.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 16,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "rpbl",
                    description: Some(
                        "desc RPBL.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 17,
                        },
                    ),
                    bit_size: 6,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "spbl",
                    description: Some(
                        "desc SPBL.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 23,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "m8pbl",
                    description: Some(
                        "desc M8PBL.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 24,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "aal",
                    description: Some(
                        "desc AAL.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 25,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "mbst",
                    description: Some(
                        "desc MBST.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 26,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "txpr",
                    description: Some(
                        "desc TXPR.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 27,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "DmaDmastsr",
            extends: None,
            description: Some(
                "desc DMA_DMASTSR.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "tis",
                    description: Some(
                        "desc TIS.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "tss",
                    description: Some(
                        "desc TSS.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 1,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "tus",
                    description: Some(
                        "desc TUS.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 2,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "tjs",
                    description: Some(
                        "desc TJS.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 3,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "ovs",
                    description: Some(
                        "desc OVS.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 4,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "uns",
                    description: Some(
                        "desc UNS.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 5,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "ris",
                    description: Some(
                        "desc RIS.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 6,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "rus",
                    description: Some(
                        "desc RUS.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 7,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "rss",
                    description: Some(
                        "desc RSS.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 8,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "rws",
                    description: Some(
                        "desc RWS.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 9,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "ets",
                    description: Some(
                        "desc ETS.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 10,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "fbs",
                    description: Some(
                        "desc FBS.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 13,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "ers",
                    description: Some(
                        "desc ERS.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 14,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "ais",
                    description: Some(
                        "desc AIS.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 15,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "nis",
                    description: Some(
                        "desc NIS.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 16,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "rsts",
                    description: Some(
                        "desc RSTS.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 17,
                        },
                    ),
                    bit_size: 3,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "tsts",
                    description: Some(
                        "desc TSTS.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 20,
                        },
                    ),
                    bit_size: 3,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "ebus",
                    description: Some(
                        "desc EBUS.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 23,
                        },
                    ),
                    bit_size: 3,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "mmcs",
                    description: Some(
                        "desc MMCS.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 27,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "pmts",
                    description: Some(
                        "desc PMTS.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 28,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "ptps",
                    description: Some(
                        "desc PTPS.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 29,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "DmaIntenar",
            extends: None,
            description: Some(
                "desc DMA_INTENAR.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "tie",
                    description: Some(
                        "desc TIE.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "tse",
                    description: Some(
                        "desc TSE.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 1,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "tue",
                    description: Some(
                        "desc TUE.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 2,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "tje",
                    description: Some(
                        "desc TJE.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 3,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "ove",
                    description: Some(
                        "desc OVE.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 4,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "une",
                    description: Some(
                        "desc UNE.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 5,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "rie",
                    description: Some(
                        "desc RIE.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 6,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "rue",
                    description: Some(
                        "desc RUE.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 7,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "rse",
                    description: Some(
                        "desc RSE.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 8,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "rwe",
                    description: Some(
                        "desc RWE.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 9,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "ete",
                    description: Some(
                        "desc ETE.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 10,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "fbe",
                    description: Some(
                        "desc FBE.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 13,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "ere",
                    description: Some(
                        "desc ERE.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 14,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "aie",
                    description: Some(
                        "desc AIE.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 15,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "nie",
                    description: Some(
                        "desc NIE.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 16,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "DmaOprmodr",
            extends: None,
            description: Some(
                "desc DMA_OPRMODR.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "str",
                    description: Some(
                        "desc STR.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 1,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "osf",
                    description: Some(
                        "desc OSF.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 2,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "rtc",
                    description: Some(
                        "desc RTC.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 3,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "dgf",
                    description: Some(
                        "desc DGF.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 5,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "fuf",
                    description: Some(
                        "desc FUF.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 6,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "fef",
                    description: Some(
                        "desc FEF.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 7,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "stt",
                    description: Some(
                        "desc STT.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 13,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "ttc",
                    description: Some(
                        "desc TTC.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 14,
                        },
                    ),
                    bit_size: 3,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "ftf",
                    description: Some(
                        "desc FTF.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 20,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "tsf",
                    description: Some(
                        "desc TSF.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 21,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "dfrf",
                    description: Some(
                        "desc DFRF.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 24,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "rsf",
                    description: Some(
                        "desc RSF.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 25,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "dtcoe",
                    description: Some(
                        "desc DTCOE.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 26,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "DmaRevwdtr",
            extends: None,
            description: Some(
                "desc DMA_REVWDTR.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "riwt",
                    description: Some(
                        "desc RIWT.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 8,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "DmaRfrcntr",
            extends: None,
            description: Some(
                "desc DMA_RFRCNTR.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "unacnt",
                    description: Some(
                        "desc UNACNT.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 16,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "unaovf",
                    description: Some(
                        "desc UNAOVF.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 16,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "ovfcnt",
                    description: Some(
                        "desc OVFCNT.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 17,
                        },
                    ),
                    bit_size: 11,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "ovfovf",
                    description: Some(
                        "desc OVFOVF.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 28,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "MacConfigr",
            extends: None,
            description: Some(
                "desc MAC_CONFIGR.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "re",
                    description: Some(
                        "desc RE.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 2,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "te",
                    description: Some(
                        "desc TE.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 3,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "dc",
                    description: Some(
                        "desc DC.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 4,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "bl",
                    description: Some(
                        "desc BL.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 5,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "acs",
                    description: Some(
                        "desc ACS.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 7,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "drty",
                    description: Some(
                        "desc DRTY.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 9,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "ipco",
                    description: Some(
                        "desc IPCO.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 10,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "dm",
                    description: Some(
                        "desc DM.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 11,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "lm",
                    description: Some(
                        "desc LM.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 12,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "do_",
                    description: Some(
                        "desc DO.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 13,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "fes",
                    description: Some(
                        "desc FES.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 14,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "dcrs",
                    description: Some(
                        "desc DCRS.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 16,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "ifg",
                    description: Some(
                        "desc IFG.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 17,
                        },
                    ),
                    bit_size: 3,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "mjb",
                    description: Some(
                        "desc MJB.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 22,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "mwd",
                    description: Some(
                        "desc MWD.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 23,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "cst",
                    description: Some(
                        "desc CST.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 25,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "sairc",
                    description: Some(
                        "desc SAIRC.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 28,
                        },
                    ),
                    bit_size: 3,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "MacFloctlr",
            extends: None,
            description: Some(
                "desc MAC_FLOCTLR.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "fca_bpa",
                    description: Some(
                        "desc FCA_BPA.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "tfe",
                    description: Some(
                        "desc TFE.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 1,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "rfe",
                    description: Some(
                        "desc RFE.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 2,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "unp",
                    description: Some(
                        "desc UNP.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 3,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "plt",
                    description: Some(
                        "desc PLT.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 4,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "dzpq",
                    description: Some(
                        "desc DZPQ.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 7,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "pauset",
                    description: Some(
                        "desc PAUSET.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 16,
                        },
                    ),
                    bit_size: 16,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "MacFltctlr",
            extends: None,
            description: Some(
                "desc MAC_FLTCTLR.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "pr",
                    description: Some(
                        "desc PR.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "huc",
                    description: Some(
                        "desc HUC.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 1,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "hmc",
                    description: Some(
                        "desc HMC.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 2,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "daif",
                    description: Some(
                        "desc DAIF.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 3,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "pmf",
                    description: Some(
                        "desc PMF.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 4,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "dbf",
                    description: Some(
                        "desc DBF.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 5,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "pcf",
                    description: Some(
                        "desc PCF.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 6,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "saif",
                    description: Some(
                        "desc SAIF.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 8,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "saf",
                    description: Some(
                        "desc SAF.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 9,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "hpf",
                    description: Some(
                        "desc HPF.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 10,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "vtfe",
                    description: Some(
                        "desc VTFE.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 16,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "ipfe",
                    description: Some(
                        "desc IPFE.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 20,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "dntu",
                    description: Some(
                        "desc DNTU.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 21,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "ra",
                    description: Some(
                        "desc RA.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 31,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "MacIntmskr",
            extends: None,
            description: Some(
                "desc MAC_INTMSKR.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "pmtim",
                    description: Some(
                        "desc PMTIM.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 3,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "tspim",
                    description: Some(
                        "desc TSPIM.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 9,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "MacIntstsr",
            extends: None,
            description: Some(
                "desc MAC_INTSTSR.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "pmtis",
                    description: Some(
                        "desc PMTIS.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 3,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "mmcis",
                    description: Some(
                        "desc MMCIS.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 4,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "mmcrxis",
                    description: Some(
                        "desc MMCRXIS.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 5,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "mmctxis",
                    description: Some(
                        "desc MMCTXIS.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 6,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "tspis",
                    description: Some(
                        "desc TSPIS.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 9,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "MacL34ctlr",
            extends: None,
            description: Some(
                "desc MAC_L34CTLR.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "l3pen",
                    description: Some(
                        "desc L3PEN.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "l3sam",
                    description: Some(
                        "desc L3SAM.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 2,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "l3saim",
                    description: Some(
                        "desc L3SAIM.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 3,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "l3dam",
                    description: Some(
                        "desc L3DAM.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 4,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "l3daim",
                    description: Some(
                        "desc L3DAIM.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 5,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "l3hsbm",
                    description: Some(
                        "desc L3HSBM.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 6,
                        },
                    ),
                    bit_size: 5,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "l3hdbm",
                    description: Some(
                        "desc L3HDBM.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 11,
                        },
                    ),
                    bit_size: 5,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "l4pen",
                    description: Some(
                        "desc L4PEN.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 16,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "l4spm",
                    description: Some(
                        "desc L4SPM.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 18,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "l4spim",
                    description: Some(
                        "desc L4SPIM.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 19,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "l4dpm",
                    description: Some(
                        "desc L4DPM.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 20,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "l4dpim",
                    description: Some(
                        "desc L4DPIM.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 21,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "MacL4portr",
            extends: None,
            description: Some(
                "desc MAC_L4PORTR.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "l4spval",
                    description: Some(
                        "desc L4SPVAL.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 16,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "l4dpval",
                    description: Some(
                        "desc L4DPVAL.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 16,
                        },
                    ),
                    bit_size: 16,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "MacMacadhr0",
            extends: None,
            description: Some(
                "desc MAC_MACADHR0.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "addrh0",
                    description: Some(
                        "desc ADDRH0.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 16,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "ae0",
                    description: Some(
                        "desc AE0.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 31,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "MacMacadhr1",
            extends: None,
            description: Some(
                "desc MAC_MACADHR1.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "addrh1",
                    description: Some(
                        "desc ADDRH1.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 16,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "mbc1",
                    description: Some(
                        "desc MBC1.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 24,
                        },
                    ),
                    bit_size: 6,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "sa1",
                    description: Some(
                        "desc SA1.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 30,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "ae1",
                    description: Some(
                        "desc AE1.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 31,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "MacMacadhr2",
            extends: None,
            description: Some(
                "desc MAC_MACADHR2.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "addrh2",
                    description: Some(
                        "desc ADDRH2.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 16,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "mbc2",
                    description: Some(
                        "desc MBC2.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 24,
                        },
                    ),
                    bit_size: 6,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "sa2",
                    description: Some(
                        "desc SA2.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 30,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "ae2",
                    description: Some(
                        "desc AE2.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 31,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "MacMacadhr3",
            extends: None,
            description: Some(
                "desc MAC_MACADHR3.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "addrh3",
                    description: Some(
                        "desc ADDRH3.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 16,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "mbc3",
                    description: Some(
                        "desc MBC3.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 24,
                        },
                    ),
                    bit_size: 6,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "sa3",
                    description: Some(
                        "desc SA3.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 30,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "ae3",
                    description: Some(
                        "desc AE3.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 31,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "MacMacadhr4",
            extends: None,
            description: Some(
                "desc MAC_MACADHR4.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "addrh4",
                    description: Some(
                        "desc ADDRH4.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 16,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "mbc4",
                    description: Some(
                        "desc MBC4.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 24,
                        },
                    ),
                    bit_size: 6,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "sa4",
                    description: Some(
                        "desc SA4.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 30,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "ae4",
                    description: Some(
                        "desc AE4.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 31,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "MacMacstsr",
            extends: None,
            description: Some(
                "desc MAC_MACSTSR.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "mrea",
                    description: Some(
                        "desc MREA.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "mrs",
                    description: Some(
                        "desc MRS.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 1,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "rfwa",
                    description: Some(
                        "desc RFWA.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 4,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "rfrs",
                    description: Some(
                        "desc RFRS.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 5,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "rffl",
                    description: Some(
                        "desc RFFL.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 8,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "mtea",
                    description: Some(
                        "desc MTEA.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 16,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "mts",
                    description: Some(
                        "desc MTS.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 17,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "mtp",
                    description: Some(
                        "desc MTP.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 19,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "tfrs",
                    description: Some(
                        "desc TFRS.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 20,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "tfwa",
                    description: Some(
                        "desc TFWA.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 22,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "tfne",
                    description: Some(
                        "desc TFNE.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 24,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "tff",
                    description: Some(
                        "desc TFF.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 25,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "MacPmtctlr",
            extends: None,
            description: Some(
                "desc MAC_PMTCTLR.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "pwdn",
                    description: Some(
                        "desc PWDN.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "mpen",
                    description: Some(
                        "desc MPEN.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 1,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "wken",
                    description: Some(
                        "desc WKEN.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 2,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "mpfr",
                    description: Some(
                        "desc MPFR.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 5,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "wkfr",
                    description: Some(
                        "desc WKFR.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 6,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "glub",
                    description: Some(
                        "desc GLUB.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 9,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "rtwktr",
                    description: Some(
                        "desc RTWKTR.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 10,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "rtwkpt",
                    description: Some(
                        "desc RTWKPT.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 24,
                        },
                    ),
                    bit_size: 3,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "rtwkfr",
                    description: Some(
                        "desc RTWKFR.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 31,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "MacSmiaddr",
            extends: None,
            description: Some(
                "desc MAC_SMIADDR.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "smib",
                    description: Some(
                        "desc SMIB.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "smiw",
                    description: Some(
                        "desc SMIW.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 1,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "smic",
                    description: Some(
                        "desc SMIC.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 2,
                        },
                    ),
                    bit_size: 4,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "smir",
                    description: Some(
                        "desc SMIR.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 6,
                        },
                    ),
                    bit_size: 5,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "smia",
                    description: Some(
                        "desc SMIA.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 11,
                        },
                    ),
                    bit_size: 5,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "MacSmidatr",
            extends: None,
            description: Some(
                "desc MAC_SMIDATR.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "smid",
                    description: Some(
                        "desc SMID.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 16,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "MacVlahtbr",
            extends: None,
            description: Some(
                "desc MAC_VLAHTBR.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "vlht",
                    description: Some(
                        "desc VLHT.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 16,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "MacVtactlr",
            extends: None,
            description: Some(
                "desc MAC_VTACTLR.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "vlanv",
                    description: Some(
                        "desc VLANV.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 16,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "vlanc",
                    description: Some(
                        "desc VLANC.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 16,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "vlans",
                    description: Some(
                        "desc VLANS.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 18,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "MacVtafltr",
            extends: None,
            description: Some(
                "desc MAC_VTAFLTR.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "vlflt",
                    description: Some(
                        "desc VLFLT.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 16,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "vtal",
                    description: Some(
                        "desc VTAL.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 16,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "vtim",
                    description: Some(
                        "desc VTIM.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 17,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "vthm",
                    description: Some(
                        "desc VTHM.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 19,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "MmcMmcctlr",
            extends: None,
            description: Some(
                "desc MMC_MMCCTLR.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "crst",
                    description: Some(
                        "desc CRST.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "cos",
                    description: Some(
                        "desc COS.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 1,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "ror",
                    description: Some(
                        "desc ROR.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 2,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "mcf",
                    description: Some(
                        "desc MCF.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 3,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "mcpset",
                    description: Some(
                        "desc MCPSET.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 4,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "mcpsel",
                    description: Some(
                        "desc MCPSEL.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 5,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "MmcRevstsr",
            extends: None,
            description: Some(
                "desc MMC_REVSTSR.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "rxbgis",
                    description: Some(
                        "desc RXBGIS.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 3,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "rxmgis",
                    description: Some(
                        "desc RXMGIS.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 4,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "rxceis",
                    description: Some(
                        "desc RXCEIS.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 5,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "rxaeis",
                    description: Some(
                        "desc RXAEIS.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 6,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "rxreis",
                    description: Some(
                        "desc RXREIS.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 7,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "rxugis",
                    description: Some(
                        "desc RXUGIS.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 17,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "rxleis",
                    description: Some(
                        "desc RXLEIS.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 18,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "rxoeis",
                    description: Some(
                        "desc RXOEIS.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 19,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "MmcRitctlr",
            extends: None,
            description: Some(
                "desc MMC_RITCTLR.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "rxbgim",
                    description: Some(
                        "desc RXBGIM.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 3,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "rxmgim",
                    description: Some(
                        "desc RXMGIM.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 4,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "rxceim",
                    description: Some(
                        "desc RXCEIM.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 5,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "rxaeim",
                    description: Some(
                        "desc RXAEIM.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 6,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "rxreim",
                    description: Some(
                        "desc RXREIM.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 7,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "rxugim",
                    description: Some(
                        "desc RXUGIM.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 17,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "rxleim",
                    description: Some(
                        "desc RXLEIM.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 18,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "rxoeim",
                    description: Some(
                        "desc RXOEIM.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 19,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "MmcRxalefr",
            extends: None,
            description: Some(
                "desc MMC_RXALEFR.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "rxalecnt",
                    description: Some(
                        "desc RXALECNT.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 16,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "MmcRxbrgfr",
            extends: None,
            description: Some(
                "desc MMC_RXBRGFR.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "rxbrgcnt",
                    description: Some(
                        "desc RXBRGCNT.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 16,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "MmcRxcrefr",
            extends: None,
            description: Some(
                "desc MMC_RXCREFR.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "rxcrecnt",
                    description: Some(
                        "desc RXCRECNT.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 16,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "MmcRxleefr",
            extends: None,
            description: Some(
                "desc MMC_RXLEEFR.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "rxleecnt",
                    description: Some(
                        "desc RXLEECNT.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 16,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "MmcRxmugfr",
            extends: None,
            description: Some(
                "desc MMC_RXMUGFR.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "rxmugcnt",
                    description: Some(
                        "desc RXMUGCNT.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 16,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "MmcRxorefr",
            extends: None,
            description: Some(
                "desc MMC_RXOREFR.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "rxorecnt",
                    description: Some(
                        "desc RXORECNT.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 16,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "MmcRxruefr",
            extends: None,
            description: Some(
                "desc MMC_RXRUEFR.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "rxruecnt",
                    description: Some(
                        "desc RXRUECNT.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 16,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "MmcRxungfr",
            extends: None,
            description: Some(
                "desc MMC_RXUNGFR.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "rxungcnt",
                    description: Some(
                        "desc RXUNGCNT.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 16,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "MmcTitctlr",
            extends: None,
            description: Some(
                "desc MMC_TITCTLR.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "txbgim",
                    description: Some(
                        "desc TXBGIM.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 2,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "txmgim",
                    description: Some(
                        "desc TXMGIM.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 3,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "txdeeim",
                    description: Some(
                        "desc TXDEEIM.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 16,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "txlceim",
                    description: Some(
                        "desc TXLCEIM.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 17,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "txeceim",
                    description: Some(
                        "desc TXECEIM.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 18,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "txcaeim",
                    description: Some(
                        "desc TXCAEIM.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 19,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "txugim",
                    description: Some(
                        "desc TXUGIM.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 21,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "txedeim",
                    description: Some(
                        "desc TXEDEIM.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 22,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "MmcTrsstsr",
            extends: None,
            description: Some(
                "desc MMC_TRSSTSR.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "txbgis",
                    description: Some(
                        "desc TXBGIS.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 2,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "txmgis",
                    description: Some(
                        "desc TXMGIS.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 3,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "txdeeis",
                    description: Some(
                        "desc TXDEEIS.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 16,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "txlceis",
                    description: Some(
                        "desc TXLCEIS.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 17,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "txeceis",
                    description: Some(
                        "desc TXECEIS.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 18,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "txcaeis",
                    description: Some(
                        "desc TXCAEIS.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 19,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "txugis",
                    description: Some(
                        "desc TXUGIS.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 21,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "txedeis",
                    description: Some(
                        "desc TXEDEIS.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 22,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "MmcTxbrgfr",
            extends: None,
            description: Some(
                "desc MMC_TXBRGFR.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "txbrgcnt",
                    description: Some(
                        "desc TXBRGCNT.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 16,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "MmcTxcaefr",
            extends: None,
            description: Some(
                "desc MMC_TXCAEFR.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "txcaecnt",
                    description: Some(
                        "desc TXCAECNT.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 16,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "MmcTxdeefr",
            extends: None,
            description: Some(
                "desc MMC_TXDEEFR.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "txdeecnt",
                    description: Some(
                        "desc TXDEECNT.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 16,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "MmcTxecefr",
            extends: None,
            description: Some(
                "desc MMC_TXECEFR.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "txececnt",
                    description: Some(
                        "desc TXECECNT.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 16,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "MmcTxedefr",
            extends: None,
            description: Some(
                "desc MMC_TXEDEFR.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "txedecnt",
                    description: Some(
                        "desc TXEDECNT.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 16,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "MmcTxlcefr",
            extends: None,
            description: Some(
                "desc MMC_TXLCEFR.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "txlcecnt",
                    description: Some(
                        "desc TXLCECNT.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 16,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "MmcTxmugfr",
            extends: None,
            description: Some(
                "desc MMC_TXMUGFR.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "txmugcnt",
                    description: Some(
                        "desc TXMUGCNT.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 16,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "MmcTxungfr",
            extends: None,
            description: Some(
                "desc MMC_TXUNGFR.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "txungcnt",
                    description: Some(
                        "desc TXUNGCNT.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 16,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "PtpPpsctlr",
            extends: None,
            description: Some(
                "desc PTP_PPSCTLR.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "ppsfre0",
                    description: Some(
                        "desc PPSFRE0.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 4,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "ppsomd",
                    description: Some(
                        "desc PPSOMD.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 4,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "tt0sel",
                    description: Some(
                        "desc TT0SEL.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 5,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "ppsfre1",
                    description: Some(
                        "desc PPSFRE1.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 8,
                        },
                    ),
                    bit_size: 3,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "tt1sel",
                    description: Some(
                        "desc TT1SEL.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 13,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "PtpTmsnser",
            extends: None,
            description: Some(
                "desc PTP_TMSNSER.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "tspsysnsec",
                    description: Some(
                        "desc TSPSYSNSEC.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 31,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "PtpTmtnser0",
            extends: None,
            description: Some(
                "desc PTP_TMTNSER0.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "tsptagnsec0",
                    description: Some(
                        "desc TSPTAGNSEC0.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 31,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "PtpTmtnser1",
            extends: None,
            description: Some(
                "desc PTP_TMTNSER1.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "tsptagnsec1",
                    description: Some(
                        "desc TSPTAGNSEC1.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 31,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "PtpTmunser",
            extends: None,
            description: Some(
                "desc PTP_TMUNSER.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "tspupnsec",
                    description: Some(
                        "desc TSPUPNSEC.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 31,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "tspupns",
                    description: Some(
                        "desc TSPUPNS.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 31,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "PtpTspctlr",
            extends: None,
            description: Some(
                "desc PTP_TSPCTLR.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "tspen",
                    description: Some(
                        "desc TSPEN.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "tspupsel",
                    description: Some(
                        "desc TSPUPSEL.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 1,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "tspini",
                    description: Some(
                        "desc TSPINI.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 2,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "tspup",
                    description: Some(
                        "desc TSPUP.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 3,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "tspint",
                    description: Some(
                        "desc TSPINT.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 4,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "tspadup",
                    description: Some(
                        "desc TSPADUP.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 5,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "tspeall",
                    description: Some(
                        "desc TSPEALL.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 8,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "tspssr",
                    description: Some(
                        "desc TSPSSR.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 9,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "tspver",
                    description: Some(
                        "desc TSPVER.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 10,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "tspoveth",
                    description: Some(
                        "desc TSPOVETH.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 11,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "tspovipv6",
                    description: Some(
                        "desc TSPOVIPV6.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 12,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "tspovipv4",
                    description: Some(
                        "desc TSPOVIPV4.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 13,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "tspmtsel",
                    description: Some(
                        "desc TSPMTSEL.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 14,
                        },
                    ),
                    bit_size: 4,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "tspadf",
                    description: Some(
                        "desc TSPADF.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 18,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "PtpTspnsar",
            extends: None,
            description: Some(
                "desc PTP_TSPNSAR.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "tspnseadd",
                    description: Some(
                        "desc TSPNSEADD.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 8,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "PtpTspstsr",
            extends: None,
            description: Some(
                "desc PTP_TSPSTSR.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "tsovf",
                    description: Some(
                        "desc TSOVF.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "tstar0",
                    description: Some(
                        "desc TSTAR0.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 1,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "tserr0",
                    description: Some(
                        "desc TSERR0.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 3,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "tstar1",
                    description: Some(
                        "desc TSTAR1.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 4,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "tserr1",
                    description: Some(
                        "desc TSERR1.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 5,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
            ],
        },
    ],
    enums: &[],
};
                