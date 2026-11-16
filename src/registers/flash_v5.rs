
                    use crate::metadata::ir::*;
                    pub(crate) static REGISTERS: IR = IR {
    blocks: &[
        Block {
            name: "Flash",
            extends: None,
            description: Some(
                "desc FLASH.",
            ),
            items: &[
                BlockItem {
                    name: "tnvs",
                    description: Some(
                        "desc TNVS.",
                    ),
                    array: None,
                    byte_offset: 0x0,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Tnvs",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "tpgs",
                    description: Some(
                        "desc TPGS.",
                    ),
                    array: None,
                    byte_offset: 0x4,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Tpgs",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "tprog",
                    description: Some(
                        "desc TPROG.",
                    ),
                    array: None,
                    byte_offset: 0x8,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Tprog",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "tserase",
                    description: Some(
                        "desc TSERASE.",
                    ),
                    array: None,
                    byte_offset: 0xc,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Tserase",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "tmerase",
                    description: Some(
                        "desc TMERASE.",
                    ),
                    array: None,
                    byte_offset: 0x10,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Tmerase",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "tprcv",
                    description: Some(
                        "desc TPRCV.",
                    ),
                    array: None,
                    byte_offset: 0x14,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Tprcv",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "tsrcv",
                    description: Some(
                        "desc TSRCV.",
                    ),
                    array: None,
                    byte_offset: 0x18,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Tsrcv",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "tmrcv",
                    description: Some(
                        "desc TMRCV.",
                    ),
                    array: None,
                    byte_offset: 0x1c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Tmrcv",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "cr",
                    description: Some(
                        "desc CR.",
                    ),
                    array: None,
                    byte_offset: 0x20,
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
                    name: "ifr",
                    description: Some(
                        "desc IFR.",
                    ),
                    array: None,
                    byte_offset: 0x24,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::Read,
                            bit_size: 32,
                            fieldset: Some(
                                "Ifr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "iclr",
                    description: Some(
                        "desc ICLR.",
                    ),
                    array: None,
                    byte_offset: 0x28,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::Write,
                            bit_size: 32,
                            fieldset: Some(
                                "Iclr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "bypass",
                    description: Some(
                        "desc BYPASS.",
                    ),
                    array: None,
                    byte_offset: 0x2c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::Write,
                            bit_size: 32,
                            fieldset: Some(
                                "Bypass",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "slock0",
                    description: Some(
                        "desc SLOCK0.",
                    ),
                    array: None,
                    byte_offset: 0x30,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Slock0",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "slock1",
                    description: Some(
                        "desc SLOCK1.",
                    ),
                    array: None,
                    byte_offset: 0x34,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Slock1",
                            ),
                        },
                    ),
                },
            ],
        },
    ],
    fieldsets: &[
        FieldSet {
            name: "Bypass",
            extends: None,
            description: Some(
                "desc BYPASS.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "byseq",
                    description: Some(
                        "desc BYSEQ.",
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
                Field {
                    name: "rsv",
                    description: Some(
                        "desc RSV.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 16,
                        },
                    ),
                    bit_size: 16,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Cr",
            extends: None,
            description: Some(
                "desc CR.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "op",
                    description: Some(
                        "desc OP.",
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
                    name: "wait",
                    description: Some(
                        "desc WAIT.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 2,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "busy",
                    description: Some(
                        "desc BUSY.",
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
                    name: "ie",
                    description: Some(
                        "desc IE.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 5,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "dpstb_en",
                    description: Some(
                        "desc DPSTB_EN.",
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
                    name: "rsv",
                    description: Some(
                        "desc RSV.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 10,
                        },
                    ),
                    bit_size: 22,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Iclr",
            extends: None,
            description: Some(
                "desc ICLR.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "iclr0",
                    description: Some(
                        "desc ICLR0.",
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
                    name: "iclr1",
                    description: Some(
                        "desc ICLR1.",
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
                            offset: 2,
                        },
                    ),
                    bit_size: 30,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Ifr",
            extends: None,
            description: Some(
                "desc IFR.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "if0",
                    description: Some(
                        "desc IF0.",
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
                    name: "if1",
                    description: Some(
                        "desc IF1.",
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
                            offset: 2,
                        },
                    ),
                    bit_size: 30,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Slock0",
            extends: None,
            description: Some(
                "desc SLOCK0.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "slock",
                    description: Some(
                        "desc SLOCK.",
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
            name: "Slock1",
            extends: None,
            description: Some(
                "desc SLOCK1.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "slock",
                    description: Some(
                        "desc SLOCK.",
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
            name: "Tmerase",
            extends: None,
            description: Some(
                "desc TMERASE.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "tmerase",
                    description: Some(
                        "desc TMERASE.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 21,
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
                            offset: 21,
                        },
                    ),
                    bit_size: 11,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Tmrcv",
            extends: None,
            description: Some(
                "desc TMRCV.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "tmrcv",
                    description: Some(
                        "desc TMRCV.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 14,
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
                            offset: 14,
                        },
                    ),
                    bit_size: 18,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Tnvs",
            extends: None,
            description: Some(
                "desc TNVS.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "tnvs",
                    description: Some(
                        "desc TNVS.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 9,
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
                            offset: 9,
                        },
                    ),
                    bit_size: 23,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Tpgs",
            extends: None,
            description: Some(
                "desc TPGS.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "tpgs",
                    description: Some(
                        "desc TPGS.",
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
                Field {
                    name: "rsv",
                    description: Some(
                        "desc RSV.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 8,
                        },
                    ),
                    bit_size: 24,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Tprcv",
            extends: None,
            description: Some(
                "desc TPRCV.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "tprcv",
                    description: Some(
                        "desc TPRCV.",
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
                Field {
                    name: "rsv",
                    description: Some(
                        "desc RSV.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 12,
                        },
                    ),
                    bit_size: 20,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Tprog",
            extends: None,
            description: Some(
                "desc TPROG.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "tprog",
                    description: Some(
                        "desc TPROG.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 9,
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
                            offset: 9,
                        },
                    ),
                    bit_size: 23,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Tserase",
            extends: None,
            description: Some(
                "desc TSERASE.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "tserase",
                    description: Some(
                        "desc TSERASE.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 18,
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
                            offset: 18,
                        },
                    ),
                    bit_size: 14,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Tsrcv",
            extends: None,
            description: Some(
                "desc TSRCV.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "tsrcv",
                    description: Some(
                        "desc TSRCV.",
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
                Field {
                    name: "rsv",
                    description: Some(
                        "desc RSV.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 12,
                        },
                    ),
                    bit_size: 20,
                    array: None,
                    enumm: None,
                },
            ],
        },
    ],
    enums: &[],
};
                