use crate::metadata::ir::*;
pub(crate) static REGISTERS: IR = IR {
    blocks: &[Block {
        name: "Trng",
        extends: None,
        description: Some("desc TRNG."),
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
                name: "mr",
                description: Some("desc MR."),
                array: None,
                byte_offset: 0x4,
                inner: BlockItemInner::Register(Register {
                    access: Access::ReadWrite,
                    bit_size: 32,
                    fieldset: Some("Mr"),
                }),
            },
            BlockItem {
                name: "dr0",
                description: Some("desc DR0."),
                array: None,
                byte_offset: 0xc,
                inner: BlockItemInner::Register(Register {
                    access: Access::Read,
                    bit_size: 32,
                    fieldset: None,
                }),
            },
            BlockItem {
                name: "dr1",
                description: Some("desc DR1."),
                array: None,
                byte_offset: 0x10,
                inner: BlockItemInner::Register(Register {
                    access: Access::Read,
                    bit_size: 32,
                    fieldset: None,
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
                    name: "en",
                    description: Some("desc EN."),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 0 }),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "run",
                    description: Some("desc RUN."),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 1 }),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "endie",
                    description: Some("desc ENDIE."),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 3 }),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Mr",
            extends: None,
            description: Some("desc MR."),
            bit_size: 32,
            fields: &[
                Field {
                    name: "load",
                    description: Some("desc LOAD."),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 0 }),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "cnt",
                    description: Some("desc CNT."),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 2 }),
                    bit_size: 3,
                    array: None,
                    enumm: None,
                },
            ],
        },
    ],
    enums: &[],
};
