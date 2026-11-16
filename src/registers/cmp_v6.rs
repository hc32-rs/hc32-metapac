
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
                    name: "dadr1",
                    description: Some(
                        "desc DADR1.",
                    ),
                    array: None,
                    byte_offset: 0x100,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 16,
                            fieldset: Some(
                                "Dadr1",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "dadr2",
                    description: Some(
                        "desc DADR2.",
                    ),
                    array: None,
                    byte_offset: 0x102,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 16,
                            fieldset: Some(
                                "Dadr2",
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
                    byte_offset: 0x108,
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
                BlockItem {
                    name: "rvadc",
                    description: Some(
                        "desc RVADC.",
                    ),
                    array: None,
                    byte_offset: 0x10c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 16,
                            fieldset: Some(
                                "Rvadc",
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
                    name: "da1en",
                    description: Some(
                        "desc DA1EN.",
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
                    name: "da2en",
                    description: Some(
                        "desc DA2EN.",
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
        FieldSet {
            name: "Dadr1",
            extends: None,
            description: Some(
                "desc DADR1.",
            ),
            bit_size: 16,
            fields: &[
                Field {
                    name: "data",
                    description: Some(
                        "desc DATA.",
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
            name: "Dadr2",
            extends: None,
            description: Some(
                "desc DADR2.",
            ),
            bit_size: 16,
            fields: &[
                Field {
                    name: "data",
                    description: Some(
                        "desc DATA.",
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
            name: "Rvadc",
            extends: None,
            description: Some(
                "desc RVADC.",
            ),
            bit_size: 16,
            fields: &[
                Field {
                    name: "da1sw",
                    description: Some(
                        "desc DA1SW.",
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
                    name: "da2sw",
                    description: Some(
                        "desc DA2SW.",
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
                    name: "vrefsw",
                    description: Some(
                        "desc VREFSW.",
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
                    name: "wprt",
                    description: Some(
                        "desc WPRT.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 8,
                        },
                    ),
                    bit_size: 8,
                    array: None,
                    enumm: None,
                },
            ],
        },
    ],
    enums: &[],
};
                