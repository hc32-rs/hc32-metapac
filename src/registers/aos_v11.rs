
                    use crate::metadata::ir::*;
                    pub(crate) static REGISTERS: IR = IR {
    blocks: &[
        Block {
            name: "Aos",
            extends: None,
            description: Some(
                "desc AOS.",
            ),
            items: &[
                BlockItem {
                    name: "intsfttrg",
                    description: Some(
                        "desc INTSFTTRG.",
                    ),
                    array: None,
                    byte_offset: 0x0,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::Write,
                            bit_size: 32,
                            fieldset: Some(
                                "Intsfttrg",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "tmr0_trgsel",
                    description: Some(
                        "desc TMR0_TRGSEL.",
                    ),
                    array: None,
                    byte_offset: 0x4,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Tmr0Trgsel",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "tmr2_trgsel0",
                    description: Some(
                        "desc TMR2_TRGSEL0.",
                    ),
                    array: None,
                    byte_offset: 0x8,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Tmr2Trgsel0",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "tmr2_trgsel1",
                    description: Some(
                        "desc TMR2_TRGSEL1.",
                    ),
                    array: None,
                    byte_offset: 0xc,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Tmr2Trgsel1",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "tmr4_trgsel",
                    description: Some(
                        "desc TMR4_TRGSEL.",
                    ),
                    array: None,
                    byte_offset: 0x10,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Tmr4Trgsel",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "tmra_trgsel0",
                    description: Some(
                        "desc TMRA_TRGSEL0.",
                    ),
                    array: None,
                    byte_offset: 0x14,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "TmraTrgsel0",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "tmra_trgsel1",
                    description: Some(
                        "desc TMRA_TRGSEL1.",
                    ),
                    array: None,
                    byte_offset: 0x18,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "TmraTrgsel1",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "adc_trgsel0",
                    description: Some(
                        "desc ADC_TRGSEL0.",
                    ),
                    array: None,
                    byte_offset: 0x1c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "AdcTrgsel0",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "adc_trgsel1",
                    description: Some(
                        "desc ADC_TRGSEL1.",
                    ),
                    array: None,
                    byte_offset: 0x20,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "AdcTrgsel1",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "dma_trgsel0",
                    description: Some(
                        "desc DMA_TRGSEL0.",
                    ),
                    array: None,
                    byte_offset: 0x24,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "DmaTrgsel0",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "dma_trgsel1",
                    description: Some(
                        "desc DMA_TRGSEL1.",
                    ),
                    array: None,
                    byte_offset: 0x28,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "DmaTrgsel1",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "dma_trgsel2",
                    description: Some(
                        "desc DMA_TRGSEL2.",
                    ),
                    array: None,
                    byte_offset: 0x2c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "DmaTrgsel2",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "dma_trgsel3",
                    description: Some(
                        "desc DMA_TRGSEL3.",
                    ),
                    array: None,
                    byte_offset: 0x30,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "DmaTrgsel3",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "pevnt_trgsel12",
                    description: Some(
                        "desc PEVNT_TRGSEL12.",
                    ),
                    array: None,
                    byte_offset: 0x34,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "PevntTrgsel12",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "pevnt_trgsel34",
                    description: Some(
                        "desc PEVNT_TRGSEL34.",
                    ),
                    array: None,
                    byte_offset: 0x38,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "PevntTrgsel34",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "comtrgsel1",
                    description: Some(
                        "desc COMTRGSEL1.",
                    ),
                    array: None,
                    byte_offset: 0x3c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Comtrgsel1",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "comtrgsel2",
                    description: Some(
                        "desc COMTRGSEL2.",
                    ),
                    array: None,
                    byte_offset: 0x40,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Comtrgsel2",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "pevntdirr1",
                    description: Some(
                        "desc PEVNTDIRR1.",
                    ),
                    array: None,
                    byte_offset: 0x100,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Pevntdirr1",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "pevntidr1",
                    description: Some(
                        "desc PEVNTIDR1.",
                    ),
                    array: None,
                    byte_offset: 0x104,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::Read,
                            bit_size: 32,
                            fieldset: Some(
                                "Pevntidr1",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "pevntodr1",
                    description: Some(
                        "desc PEVNTODR1.",
                    ),
                    array: None,
                    byte_offset: 0x108,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Pevntodr1",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "pevntorr1",
                    description: Some(
                        "desc PEVNTORR1.",
                    ),
                    array: None,
                    byte_offset: 0x10c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Pevntorr1",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "pevntosr1",
                    description: Some(
                        "desc PEVNTOSR1.",
                    ),
                    array: None,
                    byte_offset: 0x110,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Pevntosr1",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "pevntrisr1",
                    description: Some(
                        "desc PEVNTRISR1.",
                    ),
                    array: None,
                    byte_offset: 0x114,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Pevntrisr1",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "pevntfalr1",
                    description: Some(
                        "desc PEVNTFALR1.",
                    ),
                    array: None,
                    byte_offset: 0x118,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Pevntfalr1",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "pevntdirr2",
                    description: Some(
                        "desc PEVNTDIRR2.",
                    ),
                    array: None,
                    byte_offset: 0x11c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Pevntdirr2",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "pevntidr2",
                    description: Some(
                        "desc PEVNTIDR2.",
                    ),
                    array: None,
                    byte_offset: 0x120,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::Read,
                            bit_size: 32,
                            fieldset: Some(
                                "Pevntidr2",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "pevntodr2",
                    description: Some(
                        "desc PEVNTODR2.",
                    ),
                    array: None,
                    byte_offset: 0x124,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Pevntodr2",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "pevntorr2",
                    description: Some(
                        "desc PEVNTORR2.",
                    ),
                    array: None,
                    byte_offset: 0x128,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Pevntorr2",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "pevntosr2",
                    description: Some(
                        "desc PEVNTOSR2.",
                    ),
                    array: None,
                    byte_offset: 0x12c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Pevntosr2",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "pevntrisr2",
                    description: Some(
                        "desc PEVNTRISR2.",
                    ),
                    array: None,
                    byte_offset: 0x130,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Pevntrisr2",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "pevntfalr2",
                    description: Some(
                        "desc PEVNTFALR2.",
                    ),
                    array: None,
                    byte_offset: 0x134,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Pevntfalr2",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "pevntdirr3",
                    description: Some(
                        "desc PEVNTDIRR3.",
                    ),
                    array: None,
                    byte_offset: 0x138,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Pevntdirr3",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "pevntidr3",
                    description: Some(
                        "desc PEVNTIDR3.",
                    ),
                    array: None,
                    byte_offset: 0x13c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::Read,
                            bit_size: 32,
                            fieldset: Some(
                                "Pevntidr3",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "pevntodr3",
                    description: Some(
                        "desc PEVNTODR3.",
                    ),
                    array: None,
                    byte_offset: 0x140,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Pevntodr3",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "pevntorr3",
                    description: Some(
                        "desc PEVNTORR3.",
                    ),
                    array: None,
                    byte_offset: 0x144,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Pevntorr3",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "pevntosr3",
                    description: Some(
                        "desc PEVNTOSR3.",
                    ),
                    array: None,
                    byte_offset: 0x148,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Pevntosr3",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "pevntrisr3",
                    description: Some(
                        "desc PEVNTRISR3.",
                    ),
                    array: None,
                    byte_offset: 0x14c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Pevntrisr3",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "pevntfalr3",
                    description: Some(
                        "desc PEVNTFALR3.",
                    ),
                    array: None,
                    byte_offset: 0x150,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Pevntfalr3",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "pevntdirr4",
                    description: Some(
                        "desc PEVNTDIRR4.",
                    ),
                    array: None,
                    byte_offset: 0x154,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Pevntdirr4",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "pevntidr4",
                    description: Some(
                        "desc PEVNTIDR4.",
                    ),
                    array: None,
                    byte_offset: 0x158,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::Read,
                            bit_size: 32,
                            fieldset: Some(
                                "Pevntidr4",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "pevntodr4",
                    description: Some(
                        "desc PEVNTODR4.",
                    ),
                    array: None,
                    byte_offset: 0x15c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Pevntodr4",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "pevntorr4",
                    description: Some(
                        "desc PEVNTORR4.",
                    ),
                    array: None,
                    byte_offset: 0x160,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Pevntorr4",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "pevntosr4",
                    description: Some(
                        "desc PEVNTOSR4.",
                    ),
                    array: None,
                    byte_offset: 0x164,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Pevntosr4",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "pevntrisr4",
                    description: Some(
                        "desc PEVNTRISR4.",
                    ),
                    array: None,
                    byte_offset: 0x168,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Pevntrisr4",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "pevntfalr4",
                    description: Some(
                        "desc PEVNTFALR4.",
                    ),
                    array: None,
                    byte_offset: 0x16c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Pevntfalr4",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "pevntnfcr",
                    description: Some(
                        "desc PEVNTNFCR.",
                    ),
                    array: None,
                    byte_offset: 0x170,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Pevntnfcr",
                            ),
                        },
                    ),
                },
            ],
        },
    ],
    fieldsets: &[
        FieldSet {
            name: "AdcTrgsel0",
            extends: None,
            description: Some(
                "desc ADC_TRGSEL0.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "trgsel",
                    description: Some(
                        "desc TRGSEL.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 7,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "comen",
                    description: Some(
                        "desc COMEN.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 30,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "AdcTrgsel1",
            extends: None,
            description: Some(
                "desc ADC_TRGSEL1.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "trgsel",
                    description: Some(
                        "desc TRGSEL.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 7,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "comen",
                    description: Some(
                        "desc COMEN.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 30,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Comtrgsel1",
            extends: None,
            description: Some(
                "desc COMTRGSEL1.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "trgsel",
                    description: Some(
                        "desc TRGSEL.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 7,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "comen",
                    description: Some(
                        "desc COMEN.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 30,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Comtrgsel2",
            extends: None,
            description: Some(
                "desc COMTRGSEL2.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "trgsel",
                    description: Some(
                        "desc TRGSEL.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 7,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "comen",
                    description: Some(
                        "desc COMEN.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 30,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "DmaTrgsel0",
            extends: None,
            description: Some(
                "desc DMA_TRGSEL0.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "trgsel",
                    description: Some(
                        "desc TRGSEL.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 7,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "comen",
                    description: Some(
                        "desc COMEN.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 30,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "DmaTrgsel1",
            extends: None,
            description: Some(
                "desc DMA_TRGSEL1.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "trgsel",
                    description: Some(
                        "desc TRGSEL.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 7,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "comen",
                    description: Some(
                        "desc COMEN.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 30,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "DmaTrgsel2",
            extends: None,
            description: Some(
                "desc DMA_TRGSEL2.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "trgsel",
                    description: Some(
                        "desc TRGSEL.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 7,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "comen",
                    description: Some(
                        "desc COMEN.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 30,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "DmaTrgsel3",
            extends: None,
            description: Some(
                "desc DMA_TRGSEL3.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "trgsel",
                    description: Some(
                        "desc TRGSEL.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 7,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "comen",
                    description: Some(
                        "desc COMEN.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 30,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Intsfttrg",
            extends: None,
            description: Some(
                "desc INTSFTTRG.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "strg",
                    description: Some(
                        "desc STRG.",
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
            ],
        },
        FieldSet {
            name: "PevntTrgsel12",
            extends: None,
            description: Some(
                "desc PEVNT_TRGSEL12.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "trgsel",
                    description: Some(
                        "desc TRGSEL.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 7,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "comen",
                    description: Some(
                        "desc COMEN.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 30,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "PevntTrgsel34",
            extends: None,
            description: Some(
                "desc PEVNT_TRGSEL34.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "trgsel",
                    description: Some(
                        "desc TRGSEL.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 7,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "comen",
                    description: Some(
                        "desc COMEN.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 30,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Pevntdirr1",
            extends: None,
            description: Some(
                "desc PEVNTDIRR1.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "pdir",
                    description: Some(
                        "desc PDIR.",
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
            name: "Pevntdirr2",
            extends: None,
            description: Some(
                "desc PEVNTDIRR2.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "pdir",
                    description: Some(
                        "desc PDIR.",
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
            name: "Pevntdirr3",
            extends: None,
            description: Some(
                "desc PEVNTDIRR3.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "pdir",
                    description: Some(
                        "desc PDIR.",
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
            name: "Pevntdirr4",
            extends: None,
            description: Some(
                "desc PEVNTDIRR4.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "pdir",
                    description: Some(
                        "desc PDIR.",
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
            name: "Pevntfalr1",
            extends: None,
            description: Some(
                "desc PEVNTFALR1.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "fal",
                    description: Some(
                        "desc FAL.",
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
            name: "Pevntfalr2",
            extends: None,
            description: Some(
                "desc PEVNTFALR2.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "fal",
                    description: Some(
                        "desc FAL.",
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
            name: "Pevntfalr3",
            extends: None,
            description: Some(
                "desc PEVNTFALR3.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "fal",
                    description: Some(
                        "desc FAL.",
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
            name: "Pevntfalr4",
            extends: None,
            description: Some(
                "desc PEVNTFALR4.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "fal",
                    description: Some(
                        "desc FAL.",
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
            name: "Pevntidr1",
            extends: None,
            description: Some(
                "desc PEVNTIDR1.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "pin",
                    description: Some(
                        "desc PIN.",
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
            name: "Pevntidr2",
            extends: None,
            description: Some(
                "desc PEVNTIDR2.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "pin",
                    description: Some(
                        "desc PIN.",
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
            name: "Pevntidr3",
            extends: None,
            description: Some(
                "desc PEVNTIDR3.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "pin",
                    description: Some(
                        "desc PIN.",
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
            name: "Pevntidr4",
            extends: None,
            description: Some(
                "desc PEVNTIDR4.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "pin",
                    description: Some(
                        "desc PIN.",
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
            name: "Pevntnfcr",
            extends: None,
            description: Some(
                "desc PEVNTNFCR.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "nfen1",
                    description: Some(
                        "desc NFEN1.",
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
                    name: "divs1",
                    description: Some(
                        "desc DIVS1.",
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
                    name: "nfen2",
                    description: Some(
                        "desc NFEN2.",
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
                    name: "divs2",
                    description: Some(
                        "desc DIVS2.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 9,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "nfen3",
                    description: Some(
                        "desc NFEN3.",
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
                    name: "divs3",
                    description: Some(
                        "desc DIVS3.",
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
                    name: "nfen4",
                    description: Some(
                        "desc NFEN4.",
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
                    name: "divs4",
                    description: Some(
                        "desc DIVS4.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 25,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Pevntodr1",
            extends: None,
            description: Some(
                "desc PEVNTODR1.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "pout",
                    description: Some(
                        "desc POUT.",
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
            name: "Pevntodr2",
            extends: None,
            description: Some(
                "desc PEVNTODR2.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "pout",
                    description: Some(
                        "desc POUT.",
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
            name: "Pevntodr3",
            extends: None,
            description: Some(
                "desc PEVNTODR3.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "pout",
                    description: Some(
                        "desc POUT.",
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
            name: "Pevntodr4",
            extends: None,
            description: Some(
                "desc PEVNTODR4.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "pout",
                    description: Some(
                        "desc POUT.",
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
            name: "Pevntorr1",
            extends: None,
            description: Some(
                "desc PEVNTORR1.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "por",
                    description: Some(
                        "desc POR.",
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
            name: "Pevntorr2",
            extends: None,
            description: Some(
                "desc PEVNTORR2.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "por",
                    description: Some(
                        "desc POR.",
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
            name: "Pevntorr3",
            extends: None,
            description: Some(
                "desc PEVNTORR3.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "por",
                    description: Some(
                        "desc POR.",
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
            name: "Pevntorr4",
            extends: None,
            description: Some(
                "desc PEVNTORR4.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "por",
                    description: Some(
                        "desc POR.",
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
            name: "Pevntosr1",
            extends: None,
            description: Some(
                "desc PEVNTOSR1.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "pos",
                    description: Some(
                        "desc POS.",
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
            name: "Pevntosr2",
            extends: None,
            description: Some(
                "desc PEVNTOSR2.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "pos",
                    description: Some(
                        "desc POS.",
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
            name: "Pevntosr3",
            extends: None,
            description: Some(
                "desc PEVNTOSR3.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "pos",
                    description: Some(
                        "desc POS.",
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
            name: "Pevntosr4",
            extends: None,
            description: Some(
                "desc PEVNTOSR4.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "pos",
                    description: Some(
                        "desc POS.",
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
            name: "Pevntrisr1",
            extends: None,
            description: Some(
                "desc PEVNTRISR1.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "ris",
                    description: Some(
                        "desc RIS.",
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
            name: "Pevntrisr2",
            extends: None,
            description: Some(
                "desc PEVNTRISR2.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "ris",
                    description: Some(
                        "desc RIS.",
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
            name: "Pevntrisr3",
            extends: None,
            description: Some(
                "desc PEVNTRISR3.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "ris",
                    description: Some(
                        "desc RIS.",
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
            name: "Pevntrisr4",
            extends: None,
            description: Some(
                "desc PEVNTRISR4.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "ris",
                    description: Some(
                        "desc RIS.",
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
            name: "Tmr0Trgsel",
            extends: None,
            description: Some(
                "desc TMR0_TRGSEL.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "trgsel",
                    description: Some(
                        "desc TRGSEL.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 7,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "comen",
                    description: Some(
                        "desc COMEN.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 30,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Tmr2Trgsel0",
            extends: None,
            description: Some(
                "desc TMR2_TRGSEL0.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "trgsel",
                    description: Some(
                        "desc TRGSEL.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 7,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "comen",
                    description: Some(
                        "desc COMEN.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 30,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Tmr2Trgsel1",
            extends: None,
            description: Some(
                "desc TMR2_TRGSEL1.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "trgsel",
                    description: Some(
                        "desc TRGSEL.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 7,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "comen",
                    description: Some(
                        "desc COMEN.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 30,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Tmr4Trgsel",
            extends: None,
            description: Some(
                "desc TMR4_TRGSEL.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "trgsel",
                    description: Some(
                        "desc TRGSEL.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 7,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "comen",
                    description: Some(
                        "desc COMEN.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 30,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "TmraTrgsel0",
            extends: None,
            description: Some(
                "desc TMRA_TRGSEL0.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "trgsel",
                    description: Some(
                        "desc TRGSEL.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 7,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "comen",
                    description: Some(
                        "desc COMEN.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 30,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "TmraTrgsel1",
            extends: None,
            description: Some(
                "desc TMRA_TRGSEL1.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "trgsel",
                    description: Some(
                        "desc TRGSEL.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 7,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "comen",
                    description: Some(
                        "desc COMEN.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 30,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: None,
                },
            ],
        },
    ],
    enums: &[],
};
                