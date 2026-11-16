
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
                    name: "dma_trgsel0",
                    description: Some(
                        "desc DMA_TRGSEL0.",
                    ),
                    array: None,
                    byte_offset: 0x4,
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
                    byte_offset: 0x8,
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
                    byte_offset: 0xc,
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
                    byte_offset: 0x10,
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
                    name: "dma_rc_trgsel",
                    description: Some(
                        "desc DMA_RC_TRGSEL.",
                    ),
                    array: None,
                    byte_offset: 0x14,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "DmaRcTrgsel",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "dcu_trgsel",
                    description: Some(
                        "desc DCU_TRGSEL.",
                    ),
                    array: None,
                    byte_offset: 0x18,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "DcuTrgsel",
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
                    byte_offset: 0x1c,
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
                    name: "tmra_trgsel0",
                    description: Some(
                        "desc TMRA_TRGSEL0.",
                    ),
                    array: None,
                    byte_offset: 0x20,
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
                    byte_offset: 0x24,
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
                    name: "tmr4_trgsel",
                    description: Some(
                        "desc TMR4_TRGSEL.",
                    ),
                    array: None,
                    byte_offset: 0x28,
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
                    name: "adc_trgsel0",
                    description: Some(
                        "desc ADC_TRGSEL0.",
                    ),
                    array: None,
                    byte_offset: 0x2c,
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
                    byte_offset: 0x30,
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
                    name: "adc_trgsel2",
                    description: Some(
                        "desc ADC_TRGSEL2.",
                    ),
                    array: None,
                    byte_offset: 0x34,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "AdcTrgsel2",
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
                    byte_offset: 0x38,
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
                    byte_offset: 0x3c,
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
                    bit_size: 9,
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
                    bit_size: 9,
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
            name: "AdcTrgsel2",
            extends: None,
            description: Some(
                "desc ADC_TRGSEL2.",
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
                    bit_size: 9,
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
                    name: "comtrg",
                    description: Some(
                        "desc COMTRG.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 9,
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
                    name: "comtrg",
                    description: Some(
                        "desc COMTRG.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 9,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "DcuTrgsel",
            extends: None,
            description: Some(
                "desc DCU_TRGSEL.",
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
                    bit_size: 9,
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
            name: "DmaRcTrgsel",
            extends: None,
            description: Some(
                "desc DMA_RC_TRGSEL.",
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
                    bit_size: 9,
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
                    bit_size: 9,
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
                    bit_size: 9,
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
                    bit_size: 9,
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
                    bit_size: 9,
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
                    bit_size: 9,
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
                    bit_size: 9,
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
                    bit_size: 9,
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
                    bit_size: 9,
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
                