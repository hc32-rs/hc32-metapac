
                    use crate::metadata::ir::*;
                    pub(crate) static REGISTERS: IR = IR {
    blocks: &[
        Block {
            name: "Aes",
            extends: None,
            description: Some(
                "desc AES.",
            ),
            items: &[
                BlockItem {
                    name: "cr",
                    description: Some(
                        "desc CR.",
                    ),
                    array: None,
                    byte_offset: 0x0,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Cr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "data0",
                    description: Some(
                        "desc DATA0.",
                    ),
                    array: None,
                    byte_offset: 0x10,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Data0",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "data1",
                    description: Some(
                        "desc DATA1.",
                    ),
                    array: None,
                    byte_offset: 0x14,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Data1",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "data2",
                    description: Some(
                        "desc DATA2.",
                    ),
                    array: None,
                    byte_offset: 0x18,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Data2",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "data3",
                    description: Some(
                        "desc DATA3.",
                    ),
                    array: None,
                    byte_offset: 0x1c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Data3",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "key0",
                    description: Some(
                        "desc KEY0.",
                    ),
                    array: None,
                    byte_offset: 0x20,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Key0",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "key1",
                    description: Some(
                        "desc KEY1.",
                    ),
                    array: None,
                    byte_offset: 0x24,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Key1",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "key2",
                    description: Some(
                        "desc KEY2.",
                    ),
                    array: None,
                    byte_offset: 0x28,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Key2",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "key3",
                    description: Some(
                        "desc KEY3.",
                    ),
                    array: None,
                    byte_offset: 0x2c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Key3",
                            ),
                        },
                    ),
                },
            ],
        },
    ],
    fieldsets: &[
        FieldSet {
            name: "Cr",
            extends: None,
            description: Some(
                "desc CR.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "start",
                    description: Some(
                        "desc START.",
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
                    name: "mode",
                    description: Some(
                        "desc MODE.",
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
                    name: "rsv",
                    description: Some(
                        "desc RSV.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 31,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Data0",
            extends: None,
            description: Some(
                "desc DATA0.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "data0",
                    description: Some(
                        "desc DATA0.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 32,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Data1",
            extends: None,
            description: Some(
                "desc DATA1.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "data0",
                    description: Some(
                        "desc DATA0.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 32,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Data2",
            extends: None,
            description: Some(
                "desc DATA2.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "data0",
                    description: Some(
                        "desc DATA0.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 32,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Data3",
            extends: None,
            description: Some(
                "desc DATA3.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "data0",
                    description: Some(
                        "desc DATA0.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 32,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Key0",
            extends: None,
            description: Some(
                "desc KEY0.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "key0",
                    description: Some(
                        "desc KEY0.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 32,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Key1",
            extends: None,
            description: Some(
                "desc KEY1.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "key0",
                    description: Some(
                        "desc KEY0.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 32,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Key2",
            extends: None,
            description: Some(
                "desc KEY2.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "key0",
                    description: Some(
                        "desc KEY0.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 32,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Key3",
            extends: None,
            description: Some(
                "desc KEY3.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "key0",
                    description: Some(
                        "desc KEY0.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 32,
                    array: None,
                    enumm: None,
                },
            ],
        },
    ],
    enums: &[],
};
                