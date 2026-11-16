
                    use crate::metadata::ir::*;
                    pub(crate) static REGISTERS: IR = IR {
    blocks: &[
        Block {
            name: "Pwc",
            extends: None,
            description: Some(
                "desc PWC.",
            ),
            items: &[
                BlockItem {
                    name: "stpmcr",
                    description: Some(
                        "desc STPMCR.",
                    ),
                    array: None,
                    byte_offset: 0x0,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 8,
                            fieldset: Some(
                                "Stpmcr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "pwrc",
                    description: Some(
                        "desc PWRC.",
                    ),
                    array: None,
                    byte_offset: 0x4,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 8,
                            fieldset: Some(
                                "Pwrc",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "pwrmon",
                    description: Some(
                        "desc PWRMON.",
                    ),
                    array: None,
                    byte_offset: 0x8,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 8,
                            fieldset: Some(
                                "Pwrmon",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "ramcr",
                    description: Some(
                        "desc RAMCR.",
                    ),
                    array: None,
                    byte_offset: 0xc,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 8,
                            fieldset: Some(
                                "Ramcr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "lvdcsr",
                    description: Some(
                        "desc LVDCSR.",
                    ),
                    array: None,
                    byte_offset: 0x10,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 8,
                            fieldset: Some(
                                "Lvdcsr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "lvd2cr",
                    description: Some(
                        "desc LVD2CR.",
                    ),
                    array: None,
                    byte_offset: 0x14,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 16,
                            fieldset: Some(
                                "Lvd2cr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "lpmcsr",
                    description: Some(
                        "desc LPMCSR.",
                    ),
                    array: None,
                    byte_offset: 0x18,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::Read,
                            bit_size: 16,
                            fieldset: Some(
                                "Lpmcsr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "lpmcscr",
                    description: Some(
                        "desc LPMCSCR.",
                    ),
                    array: None,
                    byte_offset: 0x1a,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 16,
                            fieldset: Some(
                                "Lpmcscr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "fprc",
                    description: Some(
                        "desc FPRC.",
                    ),
                    array: None,
                    byte_offset: 0x20,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 16,
                            fieldset: Some(
                                "Fprc",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "dbgc",
                    description: Some(
                        "desc DBGC.",
                    ),
                    array: None,
                    byte_offset: 0x40,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 8,
                            fieldset: Some(
                                "Dbgc",
                            ),
                        },
                    ),
                },
            ],
        },
    ],
    fieldsets: &[
        FieldSet {
            name: "Dbgc",
            extends: None,
            description: Some(
                "desc DBGC.",
            ),
            bit_size: 8,
            fields: &[
                Field {
                    name: "dbgen",
                    description: Some(
                        "desc DBGEN.",
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
        FieldSet {
            name: "Fprc",
            extends: None,
            description: Some(
                "desc FPRC.",
            ),
            bit_size: 16,
            fields: &[
                Field {
                    name: "ckrwe",
                    description: Some(
                        "desc CKRWE.",
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
                    name: "pwrwe",
                    description: Some(
                        "desc PWRWE.",
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
                    name: "fprcb2",
                    description: Some(
                        "desc FPRCB2.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 2,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "lvrwe",
                    description: Some(
                        "desc LVRWE.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 3,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "fprcb4",
                    description: Some(
                        "desc FPRCB4.",
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
                    name: "fprcb5",
                    description: Some(
                        "desc FPRCB5.",
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
                    name: "fprcb6",
                    description: Some(
                        "desc FPRCB6.",
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
                    name: "fprcb7",
                    description: Some(
                        "desc FPRCB7.",
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
                    name: "fprcwe",
                    description: Some(
                        "desc FPRCWE.",
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
        FieldSet {
            name: "Lpmcscr",
            extends: None,
            description: Some(
                "desc LPMCSCR.",
            ),
            bit_size: 16,
            fields: &[
                Field {
                    name: "lpmfc",
                    description: Some(
                        "desc LPMFC.",
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
        FieldSet {
            name: "Lpmcsr",
            extends: None,
            description: Some(
                "desc LPMCSR.",
            ),
            bit_size: 16,
            fields: &[
                Field {
                    name: "lpmf",
                    description: Some(
                        "desc LPMF.",
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
        FieldSet {
            name: "Lvd2cr",
            extends: None,
            description: Some(
                "desc LVD2CR.",
            ),
            bit_size: 16,
            fields: &[
                Field {
                    name: "lvd2dfs",
                    description: Some(
                        "desc LVD2DFS.",
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
                    name: "lvd2dfdis",
                    description: Some(
                        "desc LVD2DFDIS.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 2,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "lvd2lvl",
                    description: Some(
                        "desc LVD2LVL.",
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
                    name: "lvd2nmis",
                    description: Some(
                        "desc LVD2NMIS.",
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
                    name: "lvd2irs",
                    description: Some(
                        "desc LVD2IRS.",
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
                    name: "lvd2irdis",
                    description: Some(
                        "desc LVD2IRDIS.",
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
                    name: "lvd2dis",
                    description: Some(
                        "desc LVD2DIS.",
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
            name: "Lvdcsr",
            extends: None,
            description: Some(
                "desc LVDCSR.",
            ),
            bit_size: 8,
            fields: &[
                Field {
                    name: "exvccinen",
                    description: Some(
                        "desc EXVCCINEN.",
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
                    name: "lvi1f",
                    description: Some(
                        "desc LVI1F.",
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
                    name: "det1f",
                    description: Some(
                        "desc DET1F.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 2,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "lvi2f",
                    description: Some(
                        "desc LVI2F.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 3,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "det2f",
                    description: Some(
                        "desc DET2F.",
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
                    name: "cmp1oe",
                    description: Some(
                        "desc CMP1OE.",
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
                    name: "cmp2oe",
                    description: Some(
                        "desc CMP2OE.",
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
            ],
        },
        FieldSet {
            name: "Pwrc",
            extends: None,
            description: Some(
                "desc PWRC.",
            ),
            bit_size: 8,
            fields: &[
                Field {
                    name: "vhrce",
                    description: Some(
                        "desc VHRCE.",
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
                    name: "hrcpwrdy",
                    description: Some(
                        "desc HRCPWRDY.",
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
            ],
        },
        FieldSet {
            name: "Pwrmon",
            extends: None,
            description: Some(
                "desc PWRMON.",
            ),
            bit_size: 8,
            fields: &[
                Field {
                    name: "pwmonsel",
                    description: Some(
                        "desc PWMONSEL.",
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
                    name: "pwmone",
                    description: Some(
                        "desc PWMONE.",
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
            name: "Ramcr",
            extends: None,
            description: Some(
                "desc RAMCR.",
            ),
            bit_size: 8,
            fields: &[
                Field {
                    name: "rprta",
                    description: Some(
                        "desc RPRTA.",
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
            name: "Stpmcr",
            extends: None,
            description: Some(
                "desc STPMCR.",
            ),
            bit_size: 8,
            fields: &[
                Field {
                    name: "ckshrc",
                    description: Some(
                        "desc CKSHRC.",
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
                    name: "hapordis",
                    description: Some(
                        "desc HAPORDIS.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 3,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "stop",
                    description: Some(
                        "desc STOP.",
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
            ],
        },
    ],
    enums: &[],
};
                