
                    use crate::metadata::ir::*;
                    pub(crate) static REGISTERS: IR = IR {
    blocks: &[
        Block {
            name: "Tmr",
            extends: None,
            description: Some(
                "desc TMR4_ECER.",
            ),
            items: &[
                BlockItem {
                    name: "ecer1",
                    description: Some(
                        "desc ECER1.",
                    ),
                    array: None,
                    byte_offset: 0x0,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Ecer1",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "ecer2",
                    description: Some(
                        "desc ECER2.",
                    ),
                    array: None,
                    byte_offset: 0x4,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Ecer2",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "ecer3",
                    description: Some(
                        "desc ECER3.",
                    ),
                    array: None,
                    byte_offset: 0x8,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Ecer3",
                            ),
                        },
                    ),
                },
            ],
        },
    ],
    fieldsets: &[
        FieldSet {
            name: "Ecer1",
            extends: None,
            description: Some(
                "desc ECER1.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "embval",
                    description: Some(
                        "desc EMBVAL.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Ecer2",
            extends: None,
            description: Some(
                "desc ECER2.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "embval",
                    description: Some(
                        "desc EMBVAL.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Ecer3",
            extends: None,
            description: Some(
                "desc ECER3.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "embval",
                    description: Some(
                        "desc EMBVAL.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
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
                