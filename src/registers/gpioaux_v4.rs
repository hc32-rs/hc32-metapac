
                    use crate::metadata::ir::*;
                    pub(crate) static REGISTERS: IR = IR {
    blocks: &[
        Block {
            name: "Gpioaux",
            extends: None,
            description: Some(
                "desc GPIOAUX.",
            ),
            items: &[
                BlockItem {
                    name: "ctrl1",
                    description: Some(
                        "desc CTRL1.",
                    ),
                    array: None,
                    byte_offset: 0x304,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Ctrl1",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "ctrl2",
                    description: Some(
                        "desc CTRL2.",
                    ),
                    array: None,
                    byte_offset: 0x308,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Ctrl2",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "times",
                    description: Some(
                        "desc TIMES.",
                    ),
                    array: None,
                    byte_offset: 0x310,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Times",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "timcps",
                    description: Some(
                        "desc TIMCPS.",
                    ),
                    array: None,
                    byte_offset: 0x314,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Timcps",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "fltcfg",
                    description: Some(
                        "desc FLTCFG.",
                    ),
                    array: None,
                    byte_offset: 0x330,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Fltcfg",
                            ),
                        },
                    ),
                },
            ],
        },
    ],
    fieldsets: &[
        FieldSet {
            name: "Ctrl1",
            extends: None,
            description: Some(
                "desc CTRL1.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "ir_pol",
                    description: Some(
                        "desc IR_POL.",
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
                    name: "hsi2c_lsel",
                    description: Some(
                        "desc HSI2C_LSEL.",
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
            name: "Ctrl2",
            extends: None,
            description: Some(
                "desc CTRL2.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "tclk_sel",
                    description: Some(
                        "desc TCLK_SEL.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 4,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "tclk_div",
                    description: Some(
                        "desc TCLK_DIV.",
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
            name: "Fltcfg",
            extends: None,
            description: Some(
                "desc FLTCFG.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "pb0flt_en",
                    description: Some(
                        "desc PB0FLT_EN.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 5,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "pa8flt_en",
                    description: Some(
                        "desc PA8FLT_EN.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 6,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "pa0flt_en",
                    description: Some(
                        "desc PA0FLT_EN.",
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
                    name: "fclk_div",
                    description: Some(
                        "desc FCLK_DIV.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 8,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "fclk_sel",
                    description: Some(
                        "desc FCLK_SEL.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 10,
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
            name: "Timcps",
            extends: None,
            description: Some(
                "desc TIMCPS.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "ctim0_c0",
                    description: Some(
                        "desc CTIM0_C0.",
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
                Field {
                    name: "ctim1_c0",
                    description: Some(
                        "desc CTIM1_C0.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 3,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "atim3_c0a",
                    description: Some(
                        "desc ATIM3_C0A.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 9,
                        },
                    ),
                    bit_size: 2,
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
            name: "Times",
            extends: None,
            description: Some(
                "desc TIMES.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "ctim0_e",
                    description: Some(
                        "desc CTIM0_E.",
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
                Field {
                    name: "ctim1_e",
                    description: Some(
                        "desc CTIM1_E.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 3,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "atim3_e",
                    description: Some(
                        "desc ATIM3_E.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 9,
                        },
                    ),
                    bit_size: 3,
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
    ],
    enums: &[],
};
                