use crate::metadata::ir::*;
pub(crate) static REGISTERS: IR = IR {
    blocks: &[Block {
        name: "Ram",
        extends: None,
        description: Some("desc RAM."),
        items: &[
            BlockItem {
                name: "cr",
                description: Some("desc CR."),
                array: None,
                byte_offset: 0x0,
                inner: BlockItemInner::Register(Register {
                    access: Access::ReadWrite,
                    bit_size: 32,
                    fieldset: Some("Cr"),
                }),
            },
            BlockItem {
                name: "erraddr",
                description: Some("desc ERRADDR."),
                array: None,
                byte_offset: 0x4,
                inner: BlockItemInner::Register(Register {
                    access: Access::Read,
                    bit_size: 32,
                    fieldset: Some("Erraddr"),
                }),
            },
            BlockItem {
                name: "ifr",
                description: Some("desc IFR."),
                array: None,
                byte_offset: 0x8,
                inner: BlockItemInner::Register(Register {
                    access: Access::Read,
                    bit_size: 32,
                    fieldset: Some("Ifr"),
                }),
            },
            BlockItem {
                name: "iclr",
                description: Some("desc ICLR."),
                array: None,
                byte_offset: 0xc,
                inner: BlockItemInner::Register(Register {
                    access: Access::Write,
                    bit_size: 32,
                    fieldset: Some("Iclr"),
                }),
            },
        ],
    }],
    fieldsets: &[
        FieldSet {
            name: "Cr",
            extends: None,
            description: Some("desc CR."),
            bit_size: 32,
            fields: &[
                Field {
                    name: "ie",
                    description: Some("desc IE."),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 1 }),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "rsv",
                    description: Some("desc RSV."),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 31 }),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Erraddr",
            extends: None,
            description: Some("desc ERRADDR."),
            bit_size: 32,
            fields: &[
                Field {
                    name: "erraddr",
                    description: Some("desc ERRADDR."),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 0 }),
                    bit_size: 13,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "rsv",
                    description: Some("desc RSV."),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 31 }),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Iclr",
            extends: None,
            description: Some("desc ICLR."),
            bit_size: 32,
            fields: &[
                Field {
                    name: "errclr",
                    description: Some("desc ERRCLR."),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 0 }),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "rsv",
                    description: Some("desc RSV."),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 31 }),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Ifr",
            extends: None,
            description: Some("desc IFR."),
            bit_size: 32,
            fields: &[
                Field {
                    name: "err",
                    description: Some("desc ERR."),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 0 }),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "rsv",
                    description: Some("desc RSV."),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 31 }),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
            ],
        },
    ],
    enums: &[],
};
