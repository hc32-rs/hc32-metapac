
                    use crate::metadata::ir::*;
                    pub(crate) static REGISTERS: IR = IR {
    blocks: &[
        Block {
            name: "Cmp",
            extends: None,
            description: Some(
                "desc CMP.",
            ),
            items: &[
                BlockItem {
                    name: "ctrl",
                    description: Some(
                        "desc CTRL.",
                    ),
                    array: None,
                    byte_offset: 0x0,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 16,
                            fieldset: Some(
                                "Ctrl",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "vltsel",
                    description: Some(
                        "desc VLTSEL.",
                    ),
                    array: None,
                    byte_offset: 0x2,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 16,
                            fieldset: Some(
                                "Vltsel",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "outmon",
                    description: Some(
                        "desc OUTMON.",
                    ),
                    array: None,
                    byte_offset: 0x4,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::Read,
                            bit_size: 16,
                            fieldset: Some(
                                "Outmon",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "cvsstb",
                    description: Some(
                        "desc CVSSTB.",
                    ),
                    array: None,
                    byte_offset: 0x6,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 16,
                            fieldset: Some(
                                "Cvsstb",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "cvsprd",
                    description: Some(
                        "desc CVSPRD.",
                    ),
                    array: None,
                    byte_offset: 0x8,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 16,
                            fieldset: Some(
                                "Cvsprd",
                            ),
                        },
                    ),
                },
            ],
        },
    ],
    fieldsets: &[
        FieldSet {
            name: "Ctrl",
            extends: None,
            description: Some(
                "desc CTRL.",
            ),
            bit_size: 16,
            fields: &[
                Field {
                    name: "fltsl",
                    description: Some(
                        "desc FLTSL.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 3,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "edgsl",
                    description: Some(
                        "desc EDGSL.",
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
                    name: "ien",
                    description: Some(
                        "desc IEN.",
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
                    name: "cvsen",
                    description: Some(
                        "desc CVSEN.",
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
                    name: "outen",
                    description: Some(
                        "desc OUTEN.",
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
                    name: "inv",
                    description: Some(
                        "desc INV.",
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
                    name: "cmpoe",
                    description: Some(
                        "desc CMPOE.",
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
                    name: "cmpon",
                    description: Some(
                        "desc CMPON.",
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
            ],
        },
        FieldSet {
            name: "Cvsprd",
            extends: None,
            description: Some(
                "desc CVSPRD.",
            ),
            bit_size: 16,
            fields: &[
                Field {
                    name: "prd",
                    description: Some(
                        "desc PRD.",
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
            name: "Cvsstb",
            extends: None,
            description: Some(
                "desc CVSSTB.",
            ),
            bit_size: 16,
            fields: &[
                Field {
                    name: "stb",
                    description: Some(
                        "desc STB.",
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
            ],
        },
        FieldSet {
            name: "Outmon",
            extends: None,
            description: Some(
                "desc OUTMON.",
            ),
            bit_size: 16,
            fields: &[
                Field {
                    name: "omon",
                    description: Some(
                        "desc OMON.",
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
                    name: "cvst",
                    description: Some(
                        "desc CVST.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 8,
                        },
                    ),
                    bit_size: 4,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Vltsel",
            extends: None,
            description: Some(
                "desc VLTSEL.",
            ),
            bit_size: 16,
            fields: &[
                Field {
                    name: "rvsl",
                    description: Some(
                        "desc RVSL.",
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
                    name: "cvsl",
                    description: Some(
                        "desc CVSL.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 8,
                        },
                    ),
                    bit_size: 4,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "c4sl",
                    description: Some(
                        "desc C4SL.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 12,
                        },
                    ),
                    bit_size: 3,
                    array: None,
                    enumm: None,
                },
            ],
        },
    ],
    enums: &[],
};
                