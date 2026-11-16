
                    use crate::metadata::ir::*;
                    pub(crate) static REGISTERS: IR = IR {
    blocks: &[
        Block {
            name: "Sramc",
            extends: None,
            description: Some(
                "desc SRAMC.",
            ),
            items: &[
                BlockItem {
                    name: "wtcr",
                    description: Some(
                        "desc WTCR.",
                    ),
                    array: None,
                    byte_offset: 0x0,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Wtcr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "wtpr",
                    description: Some(
                        "desc WTPR.",
                    ),
                    array: None,
                    byte_offset: 0x4,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Wtpr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "ckcr",
                    description: Some(
                        "desc CKCR.",
                    ),
                    array: None,
                    byte_offset: 0x8,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Ckcr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "ckpr",
                    description: Some(
                        "desc CKPR.",
                    ),
                    array: None,
                    byte_offset: 0xc,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Ckpr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "cksr",
                    description: Some(
                        "desc CKSR.",
                    ),
                    array: None,
                    byte_offset: 0x10,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Cksr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "sram1_eien",
                    description: Some(
                        "desc SRAM1_EIEN.",
                    ),
                    array: None,
                    byte_offset: 0x14,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Sram1Eien",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "sram1_eibit0",
                    description: Some(
                        "desc SRAM1_EIBIT0.",
                    ),
                    array: None,
                    byte_offset: 0x18,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: None,
                        },
                    ),
                },
                BlockItem {
                    name: "sram1_eibit1",
                    description: Some(
                        "desc SRAM1_EIBIT1.",
                    ),
                    array: None,
                    byte_offset: 0x1c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Sram1Eibit1",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "sram1_eccerraddr",
                    description: Some(
                        "desc SRAM1_ECCERRADDR.",
                    ),
                    array: None,
                    byte_offset: 0x20,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Sram1Eccerraddr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "sramh_eien",
                    description: Some(
                        "desc SRAMH_EIEN.",
                    ),
                    array: None,
                    byte_offset: 0x24,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "SramhEien",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "sramh_eibit0",
                    description: Some(
                        "desc SRAMH_EIBIT0.",
                    ),
                    array: None,
                    byte_offset: 0x28,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: None,
                        },
                    ),
                },
                BlockItem {
                    name: "sramh_eibit1",
                    description: Some(
                        "desc SRAMH_EIBIT1.",
                    ),
                    array: None,
                    byte_offset: 0x2c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "SramhEibit1",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "sramh_eccerraddr",
                    description: Some(
                        "desc SRAMH_ECCERRADDR.",
                    ),
                    array: None,
                    byte_offset: 0x30,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "SramhEccerraddr",
                            ),
                        },
                    ),
                },
            ],
        },
    ],
    fieldsets: &[
        FieldSet {
            name: "Ckcr",
            extends: None,
            description: Some(
                "desc CKCR.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "cache_eccoad",
                    description: Some(
                        "desc CACHE_ECCOAD.",
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
                    name: "sram1_eccoad",
                    description: Some(
                        "desc SRAM1_ECCOAD.",
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
                    name: "sramh_eccoad",
                    description: Some(
                        "desc SRAMH_ECCOAD.",
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
                    name: "cache_eccmod",
                    description: Some(
                        "desc CACHE_ECCMOD.",
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
                    name: "sram1_eccmod",
                    description: Some(
                        "desc SRAM1_ECCMOD.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 10,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "sramh_eccmod",
                    description: Some(
                        "desc SRAMH_ECCMOD.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 12,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Ckpr",
            extends: None,
            description: Some(
                "desc CKPR.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "ckprc",
                    description: Some(
                        "desc CKPRC.",
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
                    name: "ckprkw",
                    description: Some(
                        "desc CKPRKW.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 1,
                        },
                    ),
                    bit_size: 7,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Cksr",
            extends: None,
            description: Some(
                "desc CKSR.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "cache_1err",
                    description: Some(
                        "desc CACHE_1ERR.",
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
                    name: "cache_2err",
                    description: Some(
                        "desc CACHE_2ERR.",
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
                    name: "sram1_1err",
                    description: Some(
                        "desc SRAM1_1ERR.",
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
                    name: "sram1_2err",
                    description: Some(
                        "desc SRAM1_2ERR.",
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
                    name: "sramh_1err",
                    description: Some(
                        "desc SRAMH_1ERR.",
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
                    name: "sramh_2err",
                    description: Some(
                        "desc SRAMH_2ERR.",
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
            ],
        },
        FieldSet {
            name: "Sram1Eccerraddr",
            extends: None,
            description: Some(
                "desc SRAM1_ECCERRADDR.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "eccerraddr",
                    description: Some(
                        "desc ECCERRADDR.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 17,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Sram1Eibit1",
            extends: None,
            description: Some(
                "desc SRAM1_EIBIT1.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "eibit",
                    description: Some(
                        "desc EIBIT.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 7,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Sram1Eien",
            extends: None,
            description: Some(
                "desc SRAM1_EIEN.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "eien",
                    description: Some(
                        "desc EIEN.",
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
            name: "SramhEccerraddr",
            extends: None,
            description: Some(
                "desc SRAMH_ECCERRADDR.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "eccerraddr",
                    description: Some(
                        "desc ECCERRADDR.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 17,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "SramhEibit1",
            extends: None,
            description: Some(
                "desc SRAMH_EIBIT1.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "eibit",
                    description: Some(
                        "desc EIBIT.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 7,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "SramhEien",
            extends: None,
            description: Some(
                "desc SRAMH_EIEN.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "eien",
                    description: Some(
                        "desc EIEN.",
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
            name: "Wtcr",
            extends: None,
            description: Some(
                "desc WTCR.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "sram1rwt",
                    description: Some(
                        "desc SRAM1RWT.",
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
                    name: "sram1wwt",
                    description: Some(
                        "desc SRAM1WWT.",
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
                    name: "sramhrwt",
                    description: Some(
                        "desc SRAMHRWT.",
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
                    name: "sramhwwt",
                    description: Some(
                        "desc SRAMHWWT.",
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
            ],
        },
        FieldSet {
            name: "Wtpr",
            extends: None,
            description: Some(
                "desc WTPR.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "wtprc",
                    description: Some(
                        "desc WTPRC.",
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
                    name: "wtprkw",
                    description: Some(
                        "desc WTPRKW.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 1,
                        },
                    ),
                    bit_size: 7,
                    array: None,
                    enumm: None,
                },
            ],
        },
    ],
    enums: &[],
};
                