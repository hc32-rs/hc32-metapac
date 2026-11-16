
                    use crate::metadata::ir::*;
                    pub(crate) static REGISTERS: IR = IR {
    blocks: &[
        Block {
            name: "Dac",
            extends: None,
            description: Some(
                "desc DAC.",
            ),
            items: &[
                BlockItem {
                    name: "dadr1",
                    description: Some(
                        "desc DADR1.",
                    ),
                    array: None,
                    byte_offset: 0x0,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 16,
                            fieldset: None,
                        },
                    ),
                },
                BlockItem {
                    name: "dadr2",
                    description: Some(
                        "desc DADR2.",
                    ),
                    array: None,
                    byte_offset: 0x2,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 16,
                            fieldset: None,
                        },
                    ),
                },
                BlockItem {
                    name: "dacr",
                    description: Some(
                        "desc DACR.",
                    ),
                    array: None,
                    byte_offset: 0x4,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 16,
                            fieldset: Some(
                                "Dacr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "daadpcr",
                    description: Some(
                        "desc DAADPCR.",
                    ),
                    array: None,
                    byte_offset: 0x6,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 16,
                            fieldset: Some(
                                "Daadpcr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "daocr",
                    description: Some(
                        "desc DAOCR.",
                    ),
                    array: None,
                    byte_offset: 0x1c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 16,
                            fieldset: Some(
                                "Daocr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "dacr2",
                    description: Some(
                        "desc DACR2.",
                    ),
                    array: None,
                    byte_offset: 0x20,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 16,
                            fieldset: Some(
                                "Dacr2",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "dadactr1",
                    description: Some(
                        "desc DADACTR1.",
                    ),
                    array: None,
                    byte_offset: 0x2c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::Read,
                            bit_size: 16,
                            fieldset: Some(
                                "Dadactr1",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "dadactr2",
                    description: Some(
                        "desc DADACTR2.",
                    ),
                    array: None,
                    byte_offset: 0x2e,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::Read,
                            bit_size: 16,
                            fieldset: Some(
                                "Dadactr2",
                            ),
                        },
                    ),
                },
            ],
        },
    ],
    fieldsets: &[
        FieldSet {
            name: "Daadpcr",
            extends: None,
            description: Some(
                "desc DAADPCR.",
            ),
            bit_size: 16,
            fields: &[
                Field {
                    name: "adcsl1",
                    description: Some(
                        "desc ADCSL1.",
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
                    name: "adcsl2",
                    description: Some(
                        "desc ADCSL2.",
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
                    name: "adcsl3",
                    description: Some(
                        "desc ADCSL3.",
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
                    name: "da1sf",
                    description: Some(
                        "desc DA1SF.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 8,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "da2sf",
                    description: Some(
                        "desc DA2SF.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 9,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "adpen",
                    description: Some(
                        "desc ADPEN.",
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
            name: "Dacr",
            extends: None,
            description: Some(
                "desc DACR.",
            ),
            bit_size: 16,
            fields: &[
                Field {
                    name: "dae",
                    description: Some(
                        "desc DAE.",
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
                    name: "da1e",
                    description: Some(
                        "desc DA1E.",
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
                    name: "da2e",
                    description: Some(
                        "desc DA2E.",
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
                    name: "dpsel",
                    description: Some(
                        "desc DPSEL.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 8,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "daamp1",
                    description: Some(
                        "desc DAAMP1.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 9,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "daamp2",
                    description: Some(
                        "desc DAAMP2.",
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
            ],
        },
        FieldSet {
            name: "Dacr2",
            extends: None,
            description: Some(
                "desc DACR2.",
            ),
            bit_size: 16,
            fields: &[
                Field {
                    name: "ldmd1",
                    description: Some(
                        "desc LDMD1.",
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
                    name: "ldmd2",
                    description: Some(
                        "desc LDMD2.",
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
            name: "Dadactr1",
            extends: None,
            description: Some(
                "desc DADACTR1.",
            ),
            bit_size: 16,
            fields: &[
                Field {
                    name: "dataact",
                    description: Some(
                        "desc DATAACT.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 12,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Dadactr2",
            extends: None,
            description: Some(
                "desc DADACTR2.",
            ),
            bit_size: 16,
            fields: &[
                Field {
                    name: "dataact",
                    description: Some(
                        "desc DATAACT.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 12,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Daocr",
            extends: None,
            description: Some(
                "desc DAOCR.",
            ),
            bit_size: 16,
            fields: &[
                Field {
                    name: "daodis1",
                    description: Some(
                        "desc DAODIS1.",
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
                    name: "daodis2",
                    description: Some(
                        "desc DAODIS2.",
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
    ],
    enums: &[],
};
                