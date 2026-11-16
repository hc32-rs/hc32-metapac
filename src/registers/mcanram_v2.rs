
                    use crate::metadata::ir::*;
                    pub(crate) static REGISTERS: IR = IR {
    blocks: &[
        Block {
            name: "Mcanram",
            extends: None,
            description: Some(
                "desc MCANRAM.",
            ),
            items: &[
                BlockItem {
                    name: "ckcr",
                    description: Some(
                        "desc CKCR.",
                    ),
                    array: None,
                    byte_offset: 0x0,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Ckcr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "cksr",
                    description: Some(
                        "desc CKSR.",
                    ),
                    array: None,
                    byte_offset: 0x4,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Cksr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "eien",
                    description: Some(
                        "desc EIEN.",
                    ),
                    array: None,
                    byte_offset: 0x8,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Eien",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "eibit0",
                    description: Some(
                        "desc EIBIT0.",
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
                    name: "eibit1",
                    description: Some(
                        "desc EIBIT1.",
                    ),
                    array: None,
                    byte_offset: 0x10,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Eibit1",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "eccerraddr",
                    description: Some(
                        "desc ECCERRADDR.",
                    ),
                    array: None,
                    byte_offset: 0x14,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::Read,
                            bit_size: 32,
                            fieldset: Some(
                                "Eccerraddr",
                            ),
                        },
                    ),
                },
            ],
        },
    ],
    fieldsets: &[
        FieldSet {
            name: "Ckcr",
            extends: None,
            description: Some(
                "desc CKCR.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "eccmod",
                    description: Some(
                        "desc ECCMOD.",
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
            name: "Cksr",
            extends: None,
            description: Some(
                "desc CKSR.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "ram_1err",
                    description: Some(
                        "desc RAM_1ERR.",
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
                    name: "ram_2err",
                    description: Some(
                        "desc RAM_2ERR.",
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
            name: "Eccerraddr",
            extends: None,
            description: Some(
                "desc ECCERRADDR.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "eccerraddr",
                    description: Some(
                        "desc ECCERRADDR.",
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
            name: "Eibit1",
            extends: None,
            description: Some(
                "desc EIBIT1.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "eibit",
                    description: Some(
                        "desc EIBIT.",
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
            name: "Eien",
            extends: None,
            description: Some(
                "desc EIEN.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "eien",
                    description: Some(
                        "desc EIEN.",
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
    ],
    enums: &[],
};
                