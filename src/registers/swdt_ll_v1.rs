use crate::metadata::ir::*;
pub(crate) static REGISTERS: IR = IR {
    blocks: &[Block {
        name: "Swdt",
        extends: None,
        description: Some("desc SWDT."),
        items: &[
            BlockItem {
                name: "sr",
                description: Some("desc SR."),
                array: None,
                byte_offset: 0x4,
                inner: BlockItemInner::Register(Register {
                    access: Access::ReadWrite,
                    bit_size: 32,
                    fieldset: Some("Sr"),
                }),
            },
            BlockItem {
                name: "rr",
                description: Some("desc RR."),
                array: None,
                byte_offset: 0x8,
                inner: BlockItemInner::Register(Register {
                    access: Access::ReadWrite,
                    bit_size: 32,
                    fieldset: Some("Rr"),
                }),
            },
        ],
    }],
    fieldsets: &[
        FieldSet {
            name: "Rr",
            extends: None,
            description: Some("desc RR."),
            bit_size: 32,
            fields: &[Field {
                name: "rf",
                description: Some("desc RF."),
                bit_offset: BitOffset::Regular(RegularBitOffset { offset: 0 }),
                bit_size: 16,
                array: None,
                enumm: None,
            }],
        },
        FieldSet {
            name: "Sr",
            extends: None,
            description: Some("desc SR."),
            bit_size: 32,
            fields: &[
                Field {
                    name: "cnt",
                    description: Some("desc CNT."),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 0 }),
                    bit_size: 16,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "udf",
                    description: Some("desc UDF."),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 16 }),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "ref_",
                    description: Some("desc REF."),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 17 }),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
            ],
        },
    ],
    enums: &[],
};
