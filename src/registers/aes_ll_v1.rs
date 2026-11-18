use crate::metadata::ir::*;
pub(crate) static REGISTERS: IR = IR {
    blocks: &[Block {
        name: "Aes",
        extends: None,
        description: Some("desc AES."),
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
                name: "dr0",
                description: Some("desc DR0."),
                array: None,
                byte_offset: 0x10,
                inner: BlockItemInner::Register(Register {
                    access: Access::ReadWrite,
                    bit_size: 32,
                    fieldset: None,
                }),
            },
            BlockItem {
                name: "dr1",
                description: Some("desc DR1."),
                array: None,
                byte_offset: 0x14,
                inner: BlockItemInner::Register(Register {
                    access: Access::ReadWrite,
                    bit_size: 32,
                    fieldset: None,
                }),
            },
            BlockItem {
                name: "dr2",
                description: Some("desc DR2."),
                array: None,
                byte_offset: 0x18,
                inner: BlockItemInner::Register(Register {
                    access: Access::ReadWrite,
                    bit_size: 32,
                    fieldset: None,
                }),
            },
            BlockItem {
                name: "dr3",
                description: Some("desc DR3."),
                array: None,
                byte_offset: 0x1c,
                inner: BlockItemInner::Register(Register {
                    access: Access::ReadWrite,
                    bit_size: 32,
                    fieldset: None,
                }),
            },
            BlockItem {
                name: "kr0",
                description: Some("desc KR0."),
                array: None,
                byte_offset: 0x20,
                inner: BlockItemInner::Register(Register {
                    access: Access::ReadWrite,
                    bit_size: 32,
                    fieldset: None,
                }),
            },
            BlockItem {
                name: "kr1",
                description: Some("desc KR1."),
                array: None,
                byte_offset: 0x24,
                inner: BlockItemInner::Register(Register {
                    access: Access::ReadWrite,
                    bit_size: 32,
                    fieldset: None,
                }),
            },
            BlockItem {
                name: "kr2",
                description: Some("desc KR2."),
                array: None,
                byte_offset: 0x28,
                inner: BlockItemInner::Register(Register {
                    access: Access::ReadWrite,
                    bit_size: 32,
                    fieldset: None,
                }),
            },
            BlockItem {
                name: "kr3",
                description: Some("desc KR3."),
                array: None,
                byte_offset: 0x2c,
                inner: BlockItemInner::Register(Register {
                    access: Access::ReadWrite,
                    bit_size: 32,
                    fieldset: None,
                }),
            },
        ],
    }],
    fieldsets: &[FieldSet {
        name: "Cr",
        extends: None,
        description: Some("desc CR."),
        bit_size: 32,
        fields: &[
            Field {
                name: "start",
                description: Some("desc START."),
                bit_offset: BitOffset::Regular(RegularBitOffset { offset: 0 }),
                bit_size: 1,
                array: None,
                enumm: None,
            },
            Field {
                name: "mode",
                description: Some("desc MODE."),
                bit_offset: BitOffset::Regular(RegularBitOffset { offset: 1 }),
                bit_size: 1,
                array: None,
                enumm: None,
            },
        ],
    }],
    enums: &[],
};
