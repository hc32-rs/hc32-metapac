
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
                    name: "intc_sfttrg",
                    description: Some(
                        "desc INTC_SFTTRG.",
                    ),
                    array: None,
                    byte_offset: 0x0,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::Write,
                            bit_size: 32,
                            fieldset: Some(
                                "IntcSfttrg",
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
                    byte_offset: 0x20,
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
                    name: "tmrb_trgsel",
                    description: Some(
                        "desc TMRB_TRGSEL.",
                    ),
                    array: None,
                    byte_offset: 0x50,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "TmrbTrgsel",
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
                    byte_offset: 0x60,
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
                    byte_offset: 0x64,
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
            ],
        },
        FieldSet {
            name: "IntcSfttrg",
            extends: None,
            description: Some(
                "desc INTC_SFTTRG.",
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
                    bit_size: 7,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "TmrbTrgsel",
            extends: None,
            description: Some(
                "desc TMRB_TRGSEL.",
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
            ],
        },
    ],
    enums: &[],
};
                