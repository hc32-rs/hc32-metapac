
                    use crate::metadata::ir::*;
                    pub(crate) static REGISTERS: IR = IR {
    blocks: &[
        Block {
            name: "Exti",
            extends: None,
            description: Some(
                "desc EXTI.",
            ),
            items: &[
                BlockItem {
                    name: "intpselr0",
                    description: Some(
                        "desc INTPSELR0.",
                    ),
                    array: None,
                    byte_offset: 0x0,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Intpselr0",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "intpselr1",
                    description: Some(
                        "desc INTPSELR1.",
                    ),
                    array: None,
                    byte_offset: 0x4,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Intpselr1",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "intpdfltcr",
                    description: Some(
                        "desc INTPDFLTCR.",
                    ),
                    array: None,
                    byte_offset: 0x10,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Intpdfltcr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "intpdflter",
                    description: Some(
                        "desc INTPDFLTER.",
                    ),
                    array: None,
                    byte_offset: 0x14,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Intpdflter",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "intpieselr",
                    description: Some(
                        "desc INTPIESELR.",
                    ),
                    array: None,
                    byte_offset: 0x18,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Intpieselr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "intphieer",
                    description: Some(
                        "desc INTPHIEER.",
                    ),
                    array: None,
                    byte_offset: 0x1c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Intphieer",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "intplieer",
                    description: Some(
                        "desc INTPLIEER.",
                    ),
                    array: None,
                    byte_offset: 0x20,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Intplieer",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "intprieer",
                    description: Some(
                        "desc INTPRIEER.",
                    ),
                    array: None,
                    byte_offset: 0x24,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Intprieer",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "intpfieer",
                    description: Some(
                        "desc INTPFIEER.",
                    ),
                    array: None,
                    byte_offset: 0x28,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Intpfieer",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "intpiefr",
                    description: Some(
                        "desc INTPIEFR.",
                    ),
                    array: None,
                    byte_offset: 0x2c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::Read,
                            bit_size: 32,
                            fieldset: Some(
                                "Intpiefr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "intpieclr",
                    description: Some(
                        "desc INTPIECLR.",
                    ),
                    array: None,
                    byte_offset: 0x30,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::Write,
                            bit_size: 32,
                            fieldset: Some(
                                "Intpieclr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "wkuprier",
                    description: Some(
                        "desc WKUPRIER.",
                    ),
                    array: None,
                    byte_offset: 0x34,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Wkuprier",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "wkupfier",
                    description: Some(
                        "desc WKUPFIER.",
                    ),
                    array: None,
                    byte_offset: 0x38,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Wkupfier",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "wkup_ifr",
                    description: Some(
                        "desc WKUP_IFR.",
                    ),
                    array: None,
                    byte_offset: 0x3c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::Read,
                            bit_size: 32,
                            fieldset: Some(
                                "WkupIfr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "wkup_iclr",
                    description: Some(
                        "desc WKUP_ICLR.",
                    ),
                    array: None,
                    byte_offset: 0x40,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::Write,
                            bit_size: 32,
                            fieldset: Some(
                                "WkupIclr",
                            ),
                        },
                    ),
                },
            ],
        },
    ],
    fieldsets: &[
        FieldSet {
            name: "Intpdfltcr",
            extends: None,
            description: Some(
                "desc INTPDFLTCR.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "clksel",
                    description: Some(
                        "desc CLKSEL.",
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
            name: "Intpdflter",
            extends: None,
            description: Some(
                "desc INTPDFLTER.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "en",
                    description: Some(
                        "desc EN.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 16,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Intpfieer",
            extends: None,
            description: Some(
                "desc INTPFIEER.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "en",
                    description: Some(
                        "desc EN.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 16,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Intphieer",
            extends: None,
            description: Some(
                "desc INTPHIEER.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "en",
                    description: Some(
                        "desc EN.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 16,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Intpieclr",
            extends: None,
            description: Some(
                "desc INTPIECLR.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "iecl",
                    description: Some(
                        "desc IECL.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 16,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Intpiefr",
            extends: None,
            description: Some(
                "desc INTPIEFR.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "ief",
                    description: Some(
                        "desc IEF.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 16,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Intpieselr",
            extends: None,
            description: Some(
                "desc INTPIESELR.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "iesel",
                    description: Some(
                        "desc IESEL.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 16,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Intplieer",
            extends: None,
            description: Some(
                "desc INTPLIEER.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "en",
                    description: Some(
                        "desc EN.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 16,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Intprieer",
            extends: None,
            description: Some(
                "desc INTPRIEER.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "en",
                    description: Some(
                        "desc EN.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 16,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Intpselr0",
            extends: None,
            description: Some(
                "desc INTPSELR0.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "insel0",
                    description: Some(
                        "desc INSEL0.",
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
                    name: "insel1",
                    description: Some(
                        "desc INSEL1.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 4,
                        },
                    ),
                    bit_size: 3,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "insel2",
                    description: Some(
                        "desc INSEL2.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 8,
                        },
                    ),
                    bit_size: 3,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "insel3",
                    description: Some(
                        "desc INSEL3.",
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
                Field {
                    name: "insel4",
                    description: Some(
                        "desc INSEL4.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 16,
                        },
                    ),
                    bit_size: 3,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "insel5",
                    description: Some(
                        "desc INSEL5.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 20,
                        },
                    ),
                    bit_size: 3,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "insel6",
                    description: Some(
                        "desc INSEL6.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 24,
                        },
                    ),
                    bit_size: 3,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "insel7",
                    description: Some(
                        "desc INSEL7.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 28,
                        },
                    ),
                    bit_size: 3,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Intpselr1",
            extends: None,
            description: Some(
                "desc INTPSELR1.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "insel8",
                    description: Some(
                        "desc INSEL8.",
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
                    name: "insel9",
                    description: Some(
                        "desc INSEL9.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 4,
                        },
                    ),
                    bit_size: 3,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "insel10",
                    description: Some(
                        "desc INSEL10.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 8,
                        },
                    ),
                    bit_size: 3,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "insel11",
                    description: Some(
                        "desc INSEL11.",
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
                Field {
                    name: "insel12",
                    description: Some(
                        "desc INSEL12.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 16,
                        },
                    ),
                    bit_size: 3,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "insel13",
                    description: Some(
                        "desc INSEL13.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 20,
                        },
                    ),
                    bit_size: 3,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "insel14",
                    description: Some(
                        "desc INSEL14.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 24,
                        },
                    ),
                    bit_size: 3,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "insel15",
                    description: Some(
                        "desc INSEL15.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 28,
                        },
                    ),
                    bit_size: 3,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "WkupIclr",
            extends: None,
            description: Some(
                "desc WKUP_ICLR.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "icl",
                    description: Some(
                        "desc ICL.",
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
            name: "WkupIfr",
            extends: None,
            description: Some(
                "desc WKUP_IFR.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "if_",
                    description: Some(
                        "desc IF.",
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
            name: "Wkupfier",
            extends: None,
            description: Some(
                "desc WKUPFIER.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "en",
                    description: Some(
                        "desc EN.",
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
            name: "Wkuprier",
            extends: None,
            description: Some(
                "desc WKUPRIER.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "en",
                    description: Some(
                        "desc EN.",
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
    ],
    enums: &[],
};
                