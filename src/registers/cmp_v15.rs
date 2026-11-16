
                    use crate::metadata::ir::*;
                    pub(crate) static REGISTERS: IR = IR {
    blocks: &[
        Block {
            name: "Cmp",
            extends: None,
            description: Some(
                "desc CMP_COMMON.",
            ),
            items: &[
                BlockItem {
                    name: "dadr",
                    description: Some(
                        "desc DADR.",
                    ),
                    array: None,
                    byte_offset: 0xc,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 16,
                            fieldset: Some(
                                "Dadr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "dacr",
                    description: Some(
                        "desc DACR.",
                    ),
                    array: None,
                    byte_offset: 0xe,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 16,
                            fieldset: Some(
                                "Dacr",
                            ),
                        },
                    ),
                },
            ],
        },
    ],
    fieldsets: &[
        FieldSet {
            name: "Dacr",
            extends: None,
            description: Some(
                "desc DACR.",
            ),
            bit_size: 16,
            fields: &[
                Field {
                    name: "daen",
                    description: Some(
                        "desc DAEN.",
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
                    name: "da2poe",
                    description: Some(
                        "desc DA2POE.",
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
                    name: "refsel",
                    description: Some(
                        "desc REFSEL.",
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
            ],
        },
        FieldSet {
            name: "Dadr",
            extends: None,
            description: Some(
                "desc DADR.",
            ),
            bit_size: 16,
            fields: &[
                Field {
                    name: "dadr",
                    description: Some(
                        "desc DADR.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 12,
                    array: None,
                    enumm: None,
                },
            ],
        },
    ],
    enums: &[],
};
                