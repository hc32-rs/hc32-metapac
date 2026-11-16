
                    use crate::metadata::ir::*;
                    pub(crate) static REGISTERS: IR = IR {
    blocks: &[
        Block {
            name: "Efm",
            extends: None,
            description: Some(
                "desc EFM.",
            ),
            items: &[
                BlockItem {
                    name: "faprt",
                    description: Some(
                        "desc FAPRT.",
                    ),
                    array: None,
                    byte_offset: 0x0,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Faprt",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "fstp",
                    description: Some(
                        "desc FSTP.",
                    ),
                    array: None,
                    byte_offset: 0x4,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Fstp",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "frmc",
                    description: Some(
                        "desc FRMC.",
                    ),
                    array: None,
                    byte_offset: 0x8,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Frmc",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "fwmc",
                    description: Some(
                        "desc FWMC.",
                    ),
                    array: None,
                    byte_offset: 0xc,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Fwmc",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "fsr",
                    description: Some(
                        "desc FSR.",
                    ),
                    array: None,
                    byte_offset: 0x10,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::Read,
                            bit_size: 32,
                            fieldset: Some(
                                "Fsr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "fsclr",
                    description: Some(
                        "desc FSCLR.",
                    ),
                    array: None,
                    byte_offset: 0x14,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Fsclr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "fite",
                    description: Some(
                        "desc FITE.",
                    ),
                    array: None,
                    byte_offset: 0x18,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Fite",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "fpmtsw",
                    description: Some(
                        "desc FPMTSW.",
                    ),
                    array: None,
                    byte_offset: 0x20,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Fpmtsw",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "fpmtew",
                    description: Some(
                        "desc FPMTEW.",
                    ),
                    array: None,
                    byte_offset: 0x24,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Fpmtew",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "uqid0",
                    description: Some(
                        "desc UQID0.",
                    ),
                    array: None,
                    byte_offset: 0x50,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::Read,
                            bit_size: 32,
                            fieldset: None,
                        },
                    ),
                },
                BlockItem {
                    name: "uqid1",
                    description: Some(
                        "desc UQID1.",
                    ),
                    array: None,
                    byte_offset: 0x54,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::Read,
                            bit_size: 32,
                            fieldset: None,
                        },
                    ),
                },
                BlockItem {
                    name: "uqid2",
                    description: Some(
                        "desc UQID2.",
                    ),
                    array: None,
                    byte_offset: 0x58,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::Read,
                            bit_size: 32,
                            fieldset: None,
                        },
                    ),
                },
                BlockItem {
                    name: "hrccfgr",
                    description: Some(
                        "desc HRCCFGR.",
                    ),
                    array: None,
                    byte_offset: 0x282,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 8,
                            fieldset: Some(
                                "Hrccfgr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "lvdicgcr",
                    description: Some(
                        "desc LVDICGCR.",
                    ),
                    array: None,
                    byte_offset: 0x284,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 16,
                            fieldset: Some(
                                "Lvdicgcr",
                            ),
                        },
                    ),
                },
            ],
        },
    ],
    fieldsets: &[
        FieldSet {
            name: "Faprt",
            extends: None,
            description: Some(
                "desc FAPRT.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "faprt",
                    description: Some(
                        "desc FAPRT.",
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
            name: "Fite",
            extends: None,
            description: Some(
                "desc FITE.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "peerrite",
                    description: Some(
                        "desc PEERRITE.",
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
                    name: "optendite",
                    description: Some(
                        "desc OPTENDITE.",
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
                    name: "colerrite",
                    description: Some(
                        "desc COLERRITE.",
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
            name: "Fpmtew",
            extends: None,
            description: Some(
                "desc FPMTEW.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "fpmtew",
                    description: Some(
                        "desc FPMTEW.",
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
            name: "Fpmtsw",
            extends: None,
            description: Some(
                "desc FPMTSW.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "fpmtsw",
                    description: Some(
                        "desc FPMTSW.",
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
            name: "Frmc",
            extends: None,
            description: Some(
                "desc FRMC.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "flwt",
                    description: Some(
                        "desc FLWT.",
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
                    name: "prefete",
                    description: Some(
                        "desc PREFETE.",
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
            ],
        },
        FieldSet {
            name: "Fsclr",
            extends: None,
            description: Some(
                "desc FSCLR.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "pewerrclr",
                    description: Some(
                        "desc PEWERRCLR.",
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
                    name: "peprterrclr",
                    description: Some(
                        "desc PEPRTERRCLR.",
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
                    name: "pgmismtchclr",
                    description: Some(
                        "desc PGMISMTCHCLR.",
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
                    name: "optendclr",
                    description: Some(
                        "desc OPTENDCLR.",
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
                    name: "colerrclr",
                    description: Some(
                        "desc COLERRCLR.",
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
            name: "Fsr",
            extends: None,
            description: Some(
                "desc FSR.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "pewerr",
                    description: Some(
                        "desc PEWERR.",
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
                    name: "peprterr",
                    description: Some(
                        "desc PEPRTERR.",
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
                    name: "pgmismtch",
                    description: Some(
                        "desc PGMISMTCH.",
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
                    name: "optend",
                    description: Some(
                        "desc OPTEND.",
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
                    name: "colerr",
                    description: Some(
                        "desc COLERR.",
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
                    name: "rdy",
                    description: Some(
                        "desc RDY.",
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
            name: "Fstp",
            extends: None,
            description: Some(
                "desc FSTP.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "fstp",
                    description: Some(
                        "desc FSTP.",
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
            name: "Fwmc",
            extends: None,
            description: Some(
                "desc FWMC.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "pemode",
                    description: Some(
                        "desc PEMODE.",
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
                    name: "pemod",
                    description: Some(
                        "desc PEMOD.",
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
                    name: "bushldctl",
                    description: Some(
                        "desc BUSHLDCTL.",
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
            name: "Hrccfgr",
            extends: None,
            description: Some(
                "desc HRCCFGR.",
            ),
            bit_size: 8,
            fields: &[
                Field {
                    name: "hrcfreqs",
                    description: Some(
                        "desc HRCFREQS.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 4,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Lvdicgcr",
            extends: None,
            description: Some(
                "desc LVDICGCR.",
            ),
            bit_size: 16,
            fields: &[
                Field {
                    name: "dfs",
                    description: Some(
                        "desc DFS.",
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
                    name: "dfdis",
                    description: Some(
                        "desc DFDIS.",
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
                    name: "lvdlvl",
                    description: Some(
                        "desc LVDLVL.",
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
                    name: "nmis",
                    description: Some(
                        "desc NMIS.",
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
                    name: "irs",
                    description: Some(
                        "desc IRS.",
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
                    name: "irdis",
                    description: Some(
                        "desc IRDIS.",
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
                    name: "lvddis",
                    description: Some(
                        "desc LVDDIS.",
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
                