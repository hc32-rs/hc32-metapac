use crate::metadata::ir::*;
pub(crate) static REGISTERS: IR = IR {
    blocks: &[Block {
        name: "Crc",
        extends: None,
        description: Some("desc CRC."),
        items: &[
            BlockItem {
                name: "result",
                description: Some("desc RESULT."),
                array: None,
                byte_offset: 0x4,
                inner: BlockItemInner::Register(Register {
                    access: Access::ReadWrite,
                    bit_size: 32,
                    fieldset: Some("Result"),
                }),
            },
            BlockItem {
                name: "data",
                description: Some("desc DATA."),
                array: None,
                byte_offset: 0x80,
                inner: BlockItemInner::Register(Register {
                    access: Access::Write,
                    bit_size: 32,
                    fieldset: Some("Data"),
                }),
            },
        ],
    }],
    fieldsets: &[
        FieldSet {
            name: "Data",
            extends: None,
            description: Some("desc DATA."),
            bit_size: 32,
            fields: &[Field {
                name: "data",
                description: Some("desc DATA."),
                bit_offset: BitOffset::Regular(RegularBitOffset { offset: 0 }),
                bit_size: 32,
                array: None,
                enumm: None,
            }],
        },
        FieldSet {
            name: "Result",
            extends: None,
            description: Some("desc RESULT."),
            bit_size: 32,
            fields: &[
                Field {
                    name: "result",
                    description: Some("desc RESULT."),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 0 }),
                    bit_size: 16,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "flag",
                    description: Some("desc FLAG."),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 16 }),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
            ],
        },
    ],
    enums: &[],
};
