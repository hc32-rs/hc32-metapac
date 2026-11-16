
                    use crate::metadata::ir::*;
                    pub(crate) static REGISTERS: IR = IR {
    blocks: &[
        Block {
            name: "Peric",
            extends: None,
            description: Some(
                "desc PERIC.",
            ),
            items: &[
                BlockItem {
                    name: "tmr_synenr",
                    description: Some(
                        "desc TMR_SYNENR.",
                    ),
                    array: None,
                    byte_offset: 0x0,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "TmrSynenr",
                            ),
                        },
                    ),
                },
            ],
        },
    ],
    fieldsets: &[
        FieldSet {
            name: "TmrSynenr",
            extends: None,
            description: Some(
                "desc TMR_SYNENR.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "tmra",
                    description: Some(
                        "desc TMRA.",
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
                    name: "tmr4",
                    description: Some(
                        "desc TMR4.",
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
            ],
        },
    ],
    enums: &[],
};
                