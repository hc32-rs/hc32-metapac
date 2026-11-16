
                    use crate::metadata::ir::*;
                    pub(crate) static REGISTERS: IR = IR {
    blocks: &[
        Block {
            name: "Fcm",
            extends: None,
            description: Some(
                "desc FCM.",
            ),
            items: &[
                BlockItem {
                    name: "lvr",
                    description: Some(
                        "desc LVR.",
                    ),
                    array: None,
                    byte_offset: 0x0,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Lvr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "uvr",
                    description: Some(
                        "desc UVR.",
                    ),
                    array: None,
                    byte_offset: 0x4,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Uvr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "cntr",
                    description: Some(
                        "desc CNTR.",
                    ),
                    array: None,
                    byte_offset: 0x8,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::Read,
                            bit_size: 32,
                            fieldset: Some(
                                "Cntr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "mccr",
                    description: Some(
                        "desc MCCR.",
                    ),
                    array: None,
                    byte_offset: 0x10,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Mccr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "rccr",
                    description: Some(
                        "desc RCCR.",
                    ),
                    array: None,
                    byte_offset: 0x14,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Rccr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "rier",
                    description: Some(
                        "desc RIER.",
                    ),
                    array: None,
                    byte_offset: 0x18,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Rier",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "clr",
                    description: Some(
                        "desc CLR.",
                    ),
                    array: None,
                    byte_offset: 0x20,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::Write,
                            bit_size: 32,
                            fieldset: Some(
                                "Clr",
                            ),
                        },
                    ),
                },
            ],
        },
    ],
    fieldsets: &[
        FieldSet {
            name: "Clr",
            extends: None,
            description: Some(
                "desc CLR.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "errfclr",
                    description: Some(
                        "desc ERRFCLR.",
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
                    name: "mendfclr",
                    description: Some(
                        "desc MENDFCLR.",
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
                    name: "ovfclr",
                    description: Some(
                        "desc OVFCLR.",
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
            ],
        },
        FieldSet {
            name: "Cntr",
            extends: None,
            description: Some(
                "desc CNTR.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "cntr",
                    description: Some(
                        "desc CNTR.",
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
            name: "Lvr",
            extends: None,
            description: Some(
                "desc LVR.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "lvr",
                    description: Some(
                        "desc LVR.",
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
            name: "Mccr",
            extends: None,
            description: Some(
                "desc MCCR.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "mdivs",
                    description: Some(
                        "desc MDIVS.",
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
                    name: "mcks",
                    description: Some(
                        "desc MCKS.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 4,
                        },
                    ),
                    bit_size: 4,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Rccr",
            extends: None,
            description: Some(
                "desc RCCR.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "rdivs",
                    description: Some(
                        "desc RDIVS.",
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
                    name: "rcks",
                    description: Some(
                        "desc RCKS.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 3,
                        },
                    ),
                    bit_size: 4,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "inexs",
                    description: Some(
                        "desc INEXS.",
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
                    name: "dnfs",
                    description: Some(
                        "desc DNFS.",
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
                    name: "edges",
                    description: Some(
                        "desc EDGES.",
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
                Field {
                    name: "exrefe",
                    description: Some(
                        "desc EXREFE.",
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
            name: "Rier",
            extends: None,
            description: Some(
                "desc RIER.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "errie",
                    description: Some(
                        "desc ERRIE.",
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
                    name: "mendie",
                    description: Some(
                        "desc MENDIE.",
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
                    name: "ovfie",
                    description: Some(
                        "desc OVFIE.",
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
                    name: "errintrs",
                    description: Some(
                        "desc ERRINTRS.",
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
                    name: "errre",
                    description: Some(
                        "desc ERRRE.",
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
            name: "Uvr",
            extends: None,
            description: Some(
                "desc UVR.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "uvr",
                    description: Some(
                        "desc UVR.",
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
    ],
    enums: &[],
};
                