
                    use crate::metadata::ir::*;
                    pub(crate) static REGISTERS: IR = IR {
    blocks: &[
        Block {
            name: "Tmr",
            extends: None,
            description: Some(
                "desc TMR6_COMMON.",
            ),
            items: &[
                BlockItem {
                    name: "sstar",
                    description: Some(
                        "desc SSTAR.",
                    ),
                    array: None,
                    byte_offset: 0xf4,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Sstar",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "sstpr",
                    description: Some(
                        "desc SSTPR.",
                    ),
                    array: None,
                    byte_offset: 0xf8,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Sstpr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "sclrr",
                    description: Some(
                        "desc SCLRR.",
                    ),
                    array: None,
                    byte_offset: 0xfc,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Sclrr",
                            ),
                        },
                    ),
                },
            ],
        },
    ],
    fieldsets: &[
        FieldSet {
            name: "Sclrr",
            extends: None,
            description: Some(
                "desc SCLRR.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "scle1",
                    description: Some(
                        "desc SCLE1.",
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
                    name: "scle2",
                    description: Some(
                        "desc SCLE2.",
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
                    name: "scle3",
                    description: Some(
                        "desc SCLE3.",
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
            ],
        },
        FieldSet {
            name: "Sstar",
            extends: None,
            description: Some(
                "desc SSTAR.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "ssta1",
                    description: Some(
                        "desc SSTA1.",
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
                    name: "ssta2",
                    description: Some(
                        "desc SSTA2.",
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
                    name: "ssta3",
                    description: Some(
                        "desc SSTA3.",
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
            ],
        },
        FieldSet {
            name: "Sstpr",
            extends: None,
            description: Some(
                "desc SSTPR.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "sstp1",
                    description: Some(
                        "desc SSTP1.",
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
                    name: "sstp2",
                    description: Some(
                        "desc SSTP2.",
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
                    name: "sstp3",
                    description: Some(
                        "desc SSTP3.",
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
            ],
        },
    ],
    enums: &[],
};
                