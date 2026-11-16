
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
                    name: "sram0_eien",
                    description: Some(
                        "desc SRAM0_EIEN.",
                    ),
                    array: None,
                    byte_offset: 0x14,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Sram0Eien",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "sram0_eibit0",
                    description: Some(
                        "desc SRAM0_EIBIT0.",
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
                    name: "sram0_eibit1",
                    description: Some(
                        "desc SRAM0_EIBIT1.",
                    ),
                    array: None,
                    byte_offset: 0x1c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Sram0Eibit1",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "sram0_eccerraddr",
                    description: Some(
                        "desc SRAM0_ECCERRADDR.",
                    ),
                    array: None,
                    byte_offset: 0x20,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::Read,
                            bit_size: 32,
                            fieldset: Some(
                                "Sram0Eccerraddr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "sramb_eien",
                    description: Some(
                        "desc SRAMB_EIEN.",
                    ),
                    array: None,
                    byte_offset: 0x24,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "SrambEien",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "sramb_eibit0",
                    description: Some(
                        "desc SRAMB_EIBIT0.",
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
                    name: "sramb_eibit1",
                    description: Some(
                        "desc SRAMB_EIBIT1.",
                    ),
                    array: None,
                    byte_offset: 0x2c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "SrambEibit1",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "sramb_eccerraddr",
                    description: Some(
                        "desc SRAMB_ECCERRADDR.",
                    ),
                    array: None,
                    byte_offset: 0x30,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::Read,
                            bit_size: 32,
                            fieldset: Some(
                                "SrambEccerraddr",
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
                    name: "pyoad",
                    description: Some(
                        "desc PYOAD.",
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
                    name: "eccoad",
                    description: Some(
                        "desc ECCOAD.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 16,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "beccoad",
                    description: Some(
                        "desc BECCOAD.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 17,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "eccmod",
                    description: Some(
                        "desc ECCMOD.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 24,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "beccmod",
                    description: Some(
                        "desc BECCMOD.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 26,
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
                    name: "sramh_pyerr",
                    description: Some(
                        "desc SRAMH_PYERR.",
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
                    name: "sram0_1err",
                    description: Some(
                        "desc SRAM0_1ERR.",
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
                    name: "sram0_2err",
                    description: Some(
                        "desc SRAM0_2ERR.",
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
                    name: "sramb_1err",
                    description: Some(
                        "desc SRAMB_1ERR.",
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
                    name: "sramb_2err",
                    description: Some(
                        "desc SRAMB_2ERR.",
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
                    name: "cache_pyerr",
                    description: Some(
                        "desc CACHE_PYERR.",
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
            ],
        },
        FieldSet {
            name: "Sram0Eccerraddr",
            extends: None,
            description: Some(
                "desc SRAM0_ECCERRADDR.",
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
                    bit_size: 15,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Sram0Eibit1",
            extends: None,
            description: Some(
                "desc SRAM0_EIBIT1.",
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
            name: "Sram0Eien",
            extends: None,
            description: Some(
                "desc SRAM0_EIEN.",
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
            name: "SrambEccerraddr",
            extends: None,
            description: Some(
                "desc SRAMB_ECCERRADDR.",
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
                    bit_size: 12,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "SrambEibit1",
            extends: None,
            description: Some(
                "desc SRAMB_EIBIT1.",
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
            name: "SrambEien",
            extends: None,
            description: Some(
                "desc SRAMB_EIEN.",
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
    ],
    enums: &[],
};
                