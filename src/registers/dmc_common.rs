
                    use crate::metadata::ir::*;
                    pub(crate) static REGISTERS: IR = IR {
    blocks: &[
        Block {
            name: "Dmc",
            extends: None,
            description: Some(
                "desc DMC.",
            ),
            items: &[
                BlockItem {
                    name: "stsr",
                    description: Some(
                        "desc STSR.",
                    ),
                    array: None,
                    byte_offset: 0x0,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::Read,
                            bit_size: 32,
                            fieldset: Some(
                                "Stsr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "stcr",
                    description: Some(
                        "desc STCR.",
                    ),
                    array: None,
                    byte_offset: 0x4,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::Write,
                            bit_size: 32,
                            fieldset: Some(
                                "Stcr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "cmdr",
                    description: Some(
                        "desc CMDR.",
                    ),
                    array: None,
                    byte_offset: 0x8,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::Write,
                            bit_size: 32,
                            fieldset: Some(
                                "Cmdr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "cpcr",
                    description: Some(
                        "desc CPCR.",
                    ),
                    array: None,
                    byte_offset: 0xc,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Cpcr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "rftr",
                    description: Some(
                        "desc RFTR.",
                    ),
                    array: None,
                    byte_offset: 0x10,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Rftr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "tmcr_t_casl",
                    description: Some(
                        "desc TMCR_T_CASL.",
                    ),
                    array: None,
                    byte_offset: 0x14,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "TmcrTCasl",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "tmcr_t_dqss",
                    description: Some(
                        "desc TMCR_T_DQSS.",
                    ),
                    array: None,
                    byte_offset: 0x18,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "TmcrTDqss",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "tmcr_t_mrd",
                    description: Some(
                        "desc TMCR_T_MRD.",
                    ),
                    array: None,
                    byte_offset: 0x1c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "TmcrTMrd",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "tmcr_t_ras",
                    description: Some(
                        "desc TMCR_T_RAS.",
                    ),
                    array: None,
                    byte_offset: 0x20,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "TmcrTRas",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "tmcr_t_rc",
                    description: Some(
                        "desc TMCR_T_RC.",
                    ),
                    array: None,
                    byte_offset: 0x24,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "TmcrTRc",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "tmcr_t_rcd",
                    description: Some(
                        "desc TMCR_T_RCD.",
                    ),
                    array: None,
                    byte_offset: 0x28,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "TmcrTRcd",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "tmcr_t_rfc",
                    description: Some(
                        "desc TMCR_T_RFC.",
                    ),
                    array: None,
                    byte_offset: 0x2c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "TmcrTRfc",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "tmcr_t_rp",
                    description: Some(
                        "desc TMCR_T_RP.",
                    ),
                    array: None,
                    byte_offset: 0x30,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "TmcrTRp",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "tmcr_t_rrd",
                    description: Some(
                        "desc TMCR_T_RRD.",
                    ),
                    array: None,
                    byte_offset: 0x34,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "TmcrTRrd",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "tmcr_t_wr",
                    description: Some(
                        "desc TMCR_T_WR.",
                    ),
                    array: None,
                    byte_offset: 0x38,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "TmcrTWr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "tmcr_t_wtr",
                    description: Some(
                        "desc TMCR_T_WTR.",
                    ),
                    array: None,
                    byte_offset: 0x3c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "TmcrTWtr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "tmcr_t_xp",
                    description: Some(
                        "desc TMCR_T_XP.",
                    ),
                    array: None,
                    byte_offset: 0x40,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "TmcrTXp",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "tmcr_t_xsr",
                    description: Some(
                        "desc TMCR_T_XSR.",
                    ),
                    array: None,
                    byte_offset: 0x44,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "TmcrTXsr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "tmcr_t_esr",
                    description: Some(
                        "desc TMCR_T_ESR.",
                    ),
                    array: None,
                    byte_offset: 0x48,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "TmcrTEsr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "cscr0",
                    description: Some(
                        "desc CSCR0.",
                    ),
                    array: None,
                    byte_offset: 0x200,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Cscr0",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "cscr1",
                    description: Some(
                        "desc CSCR1.",
                    ),
                    array: None,
                    byte_offset: 0x204,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Cscr1",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "cscr2",
                    description: Some(
                        "desc CSCR2.",
                    ),
                    array: None,
                    byte_offset: 0x208,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Cscr2",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "cscr3",
                    description: Some(
                        "desc CSCR3.",
                    ),
                    array: None,
                    byte_offset: 0x20c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Cscr3",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "bacr",
                    description: Some(
                        "desc BACR.",
                    ),
                    array: None,
                    byte_offset: 0x300,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Bacr",
                            ),
                        },
                    ),
                },
            ],
        },
    ],
    fieldsets: &[
        FieldSet {
            name: "Bacr",
            extends: None,
            description: Some(
                "desc BACR.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "dmcmw",
                    description: Some(
                        "desc DMCMW.",
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
                    name: "cksel",
                    description: Some(
                        "desc CKSEL.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 14,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Cmdr",
            extends: None,
            description: Some(
                "desc CMDR.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "cmdadd",
                    description: Some(
                        "desc CMDADD.",
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
                    name: "cmdba",
                    description: Some(
                        "desc CMDBA.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 16,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "cmd",
                    description: Some(
                        "desc CMD.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 18,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "cmdchip",
                    description: Some(
                        "desc CMDCHIP.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 20,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Cpcr",
            extends: None,
            description: Some(
                "desc CPCR.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "colbs",
                    description: Some(
                        "desc COLBS.",
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
                    name: "rowbs",
                    description: Some(
                        "desc ROWBS.",
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
                    name: "apbs",
                    description: Some(
                        "desc APBS.",
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
                    name: "ckedis",
                    description: Some(
                        "desc CKEDIS.",
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
                    name: "ckstop",
                    description: Some(
                        "desc CKSTOP.",
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
                    name: "ckedisprd",
                    description: Some(
                        "desc CKEDISPRD.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 10,
                        },
                    ),
                    bit_size: 6,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "burst",
                    description: Some(
                        "desc BURST.",
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
                    name: "actcp",
                    description: Some(
                        "desc ACTCP.",
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
            ],
        },
        FieldSet {
            name: "Cscr0",
            extends: None,
            description: Some(
                "desc CSCR0.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "addmsk",
                    description: Some(
                        "desc ADDMSK.",
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
                    name: "addmat",
                    description: Some(
                        "desc ADDMAT.",
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
                Field {
                    name: "brc",
                    description: Some(
                        "desc BRC.",
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
            ],
        },
        FieldSet {
            name: "Cscr1",
            extends: None,
            description: Some(
                "desc CSCR1.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "addmsk",
                    description: Some(
                        "desc ADDMSK.",
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
                    name: "addmat",
                    description: Some(
                        "desc ADDMAT.",
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
                Field {
                    name: "brc",
                    description: Some(
                        "desc BRC.",
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
            ],
        },
        FieldSet {
            name: "Cscr2",
            extends: None,
            description: Some(
                "desc CSCR2.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "addmsk",
                    description: Some(
                        "desc ADDMSK.",
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
                    name: "addmat",
                    description: Some(
                        "desc ADDMAT.",
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
                Field {
                    name: "brc",
                    description: Some(
                        "desc BRC.",
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
            ],
        },
        FieldSet {
            name: "Cscr3",
            extends: None,
            description: Some(
                "desc CSCR3.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "addmsk",
                    description: Some(
                        "desc ADDMSK.",
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
                    name: "addmat",
                    description: Some(
                        "desc ADDMAT.",
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
                Field {
                    name: "brc",
                    description: Some(
                        "desc BRC.",
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
            ],
        },
        FieldSet {
            name: "Rftr",
            extends: None,
            description: Some(
                "desc RFTR.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "refprd",
                    description: Some(
                        "desc REFPRD.",
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
            name: "Stcr",
            extends: None,
            description: Some(
                "desc STCR.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "stctl",
                    description: Some(
                        "desc STCTL.",
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
            ],
        },
        FieldSet {
            name: "Stsr",
            extends: None,
            description: Some(
                "desc STSR.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "status",
                    description: Some(
                        "desc STATUS.",
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
                    name: "memw",
                    description: Some(
                        "desc MEMW.",
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
            ],
        },
        FieldSet {
            name: "TmcrTCasl",
            extends: None,
            description: Some(
                "desc TMCR_T_CASL.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "t_casl",
                    description: Some(
                        "desc T_CASL.",
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
            ],
        },
        FieldSet {
            name: "TmcrTDqss",
            extends: None,
            description: Some(
                "desc TMCR_T_DQSS.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "t_dqss",
                    description: Some(
                        "desc T_DQSS.",
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
            name: "TmcrTEsr",
            extends: None,
            description: Some(
                "desc TMCR_T_ESR.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "t_esr",
                    description: Some(
                        "desc T_ESR.",
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
            name: "TmcrTMrd",
            extends: None,
            description: Some(
                "desc TMCR_T_MRD.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "t_mrd",
                    description: Some(
                        "desc T_MRD.",
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
            name: "TmcrTRas",
            extends: None,
            description: Some(
                "desc TMCR_T_RAS.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "t_ras",
                    description: Some(
                        "desc T_RAS.",
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
            name: "TmcrTRc",
            extends: None,
            description: Some(
                "desc TMCR_T_RC.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "t_rc",
                    description: Some(
                        "desc T_RC.",
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
            name: "TmcrTRcd",
            extends: None,
            description: Some(
                "desc TMCR_T_RCD.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "t_rcd_b",
                    description: Some(
                        "desc T_RCD_B.",
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
                    name: "t_rcd_p",
                    description: Some(
                        "desc T_RCD_P.",
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
            ],
        },
        FieldSet {
            name: "TmcrTRfc",
            extends: None,
            description: Some(
                "desc TMCR_T_RFC.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "t_rfc_b",
                    description: Some(
                        "desc T_RFC_B.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 5,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "t_rfc_p",
                    description: Some(
                        "desc T_RFC_P.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 8,
                        },
                    ),
                    bit_size: 5,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "TmcrTRp",
            extends: None,
            description: Some(
                "desc TMCR_T_RP.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "t_rp_b",
                    description: Some(
                        "desc T_RP_B.",
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
                    name: "t_rp_p",
                    description: Some(
                        "desc T_RP_P.",
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
            ],
        },
        FieldSet {
            name: "TmcrTRrd",
            extends: None,
            description: Some(
                "desc TMCR_T_RRD.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "t_rrd",
                    description: Some(
                        "desc T_RRD.",
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
            name: "TmcrTWr",
            extends: None,
            description: Some(
                "desc TMCR_T_WR.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "t_wr",
                    description: Some(
                        "desc T_WR.",
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
            ],
        },
        FieldSet {
            name: "TmcrTWtr",
            extends: None,
            description: Some(
                "desc TMCR_T_WTR.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "t_wtr",
                    description: Some(
                        "desc T_WTR.",
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
            ],
        },
        FieldSet {
            name: "TmcrTXp",
            extends: None,
            description: Some(
                "desc TMCR_T_XP.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "t_xp",
                    description: Some(
                        "desc T_XP.",
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
            name: "TmcrTXsr",
            extends: None,
            description: Some(
                "desc TMCR_T_XSR.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "t_xsr",
                    description: Some(
                        "desc T_XSR.",
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
                