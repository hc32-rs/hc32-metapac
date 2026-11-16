
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
                    name: "key1",
                    description: Some(
                        "desc KEY1.",
                    ),
                    array: None,
                    byte_offset: 0x4,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: None,
                        },
                    ),
                },
                BlockItem {
                    name: "key2",
                    description: Some(
                        "desc KEY2.",
                    ),
                    array: None,
                    byte_offset: 0x8,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: None,
                        },
                    ),
                },
                BlockItem {
                    name: "fstp",
                    description: Some(
                        "desc FSTP.",
                    ),
                    array: None,
                    byte_offset: 0x14,
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
                    byte_offset: 0x18,
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
                    byte_offset: 0x1c,
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
                    byte_offset: 0x20,
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
                    byte_offset: 0x24,
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
                    byte_offset: 0x28,
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
                    name: "fswp",
                    description: Some(
                        "desc FSWP.",
                    ),
                    array: None,
                    byte_offset: 0x2c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::Read,
                            bit_size: 32,
                            fieldset: Some(
                                "Fswp",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "chipid",
                    description: Some(
                        "desc CHIPID.",
                    ),
                    array: None,
                    byte_offset: 0x40,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::Read,
                            bit_size: 32,
                            fieldset: None,
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
                            fieldset: Some(
                                "Uqid0",
                            ),
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
                            fieldset: Some(
                                "Uqid1",
                            ),
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
                            fieldset: Some(
                                "Uqid2",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "mmf_remprt",
                    description: Some(
                        "desc MMF_REMPRT.",
                    ),
                    array: None,
                    byte_offset: 0x100,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "MmfRemprt",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "mmf_remcr0",
                    description: Some(
                        "desc MMF_REMCR0.",
                    ),
                    array: None,
                    byte_offset: 0x104,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "MmfRemcr0",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "mmf_remcr1",
                    description: Some(
                        "desc MMF_REMCR1.",
                    ),
                    array: None,
                    byte_offset: 0x108,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "MmfRemcr1",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "wlock",
                    description: Some(
                        "desc WLOCK.",
                    ),
                    array: None,
                    byte_offset: 0x180,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Wlock",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "f0nwprt",
                    description: Some(
                        "desc F0NWPRT.",
                    ),
                    array: None,
                    byte_offset: 0x190,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "F0nwprt",
                            ),
                        },
                    ),
                },
            ],
        },
    ],
    fieldsets: &[
        FieldSet {
            name: "F0nwprt",
            extends: None,
            description: Some(
                "desc F0NWPRT.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "f0nwprt0",
                    description: Some(
                        "desc F0NWPRT0.",
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
                    name: "f0nwprt1",
                    description: Some(
                        "desc F0NWPRT1.",
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
                    name: "f0nwprt2",
                    description: Some(
                        "desc F0NWPRT2.",
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
                    name: "f0nwprt3",
                    description: Some(
                        "desc F0NWPRT3.",
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
                    name: "f0nwprt4",
                    description: Some(
                        "desc F0NWPRT4.",
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
                    name: "f0nwprt5",
                    description: Some(
                        "desc F0NWPRT5.",
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
                    name: "f0nwprt6",
                    description: Some(
                        "desc F0NWPRT6.",
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
                    name: "f0nwprt7",
                    description: Some(
                        "desc F0NWPRT7.",
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
                    name: "f0nwprt8",
                    description: Some(
                        "desc F0NWPRT8.",
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
                    name: "f0nwprt9",
                    description: Some(
                        "desc F0NWPRT9.",
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
                    name: "f0nwprt10",
                    description: Some(
                        "desc F0NWPRT10.",
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
                    name: "f0nwprt11",
                    description: Some(
                        "desc F0NWPRT11.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 11,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "f0nwprt12",
                    description: Some(
                        "desc F0NWPRT12.",
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
                    name: "f0nwprt13",
                    description: Some(
                        "desc F0NWPRT13.",
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
                    name: "f0nwprt14",
                    description: Some(
                        "desc F0NWPRT14.",
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
                    name: "f0nwprt15",
                    description: Some(
                        "desc F0NWPRT15.",
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
                    name: "f0nwprt16",
                    description: Some(
                        "desc F0NWPRT16.",
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
                    name: "f0nwprt17",
                    description: Some(
                        "desc F0NWPRT17.",
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
                    name: "f0nwprt18",
                    description: Some(
                        "desc F0NWPRT18.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 18,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "f0nwprt19",
                    description: Some(
                        "desc F0NWPRT19.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 19,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "f0nwprt20",
                    description: Some(
                        "desc F0NWPRT20.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 20,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "f0nwprt21",
                    description: Some(
                        "desc F0NWPRT21.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 21,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "f0nwprt22",
                    description: Some(
                        "desc F0NWPRT22.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 22,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "f0nwprt23",
                    description: Some(
                        "desc F0NWPRT23.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 23,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "f0nwprt24",
                    description: Some(
                        "desc F0NWPRT24.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 24,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "f0nwprt25",
                    description: Some(
                        "desc F0NWPRT25.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 25,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "f0nwprt26",
                    description: Some(
                        "desc F0NWPRT26.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 26,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "f0nwprt27",
                    description: Some(
                        "desc F0NWPRT27.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 27,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "f0nwprt28",
                    description: Some(
                        "desc F0NWPRT28.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 28,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "f0nwprt29",
                    description: Some(
                        "desc F0NWPRT29.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 29,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "f0nwprt30",
                    description: Some(
                        "desc F0NWPRT30.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 30,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "f0nwprt31",
                    description: Some(
                        "desc F0NWPRT31.",
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
                    bit_size: 4,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "lvm",
                    description: Some(
                        "desc LVM.",
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
                    name: "icache",
                    description: Some(
                        "desc ICACHE.",
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
                    name: "dcache",
                    description: Some(
                        "desc DCACHE.",
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
                    name: "prefete",
                    description: Some(
                        "desc PREFETE.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 18,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "crst",
                    description: Some(
                        "desc CRST.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 19,
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
                    name: "otpwerrclr",
                    description: Some(
                        "desc OTPWERRCLR.",
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
                    name: "prtwerrclr",
                    description: Some(
                        "desc PRTWERRCLR.",
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
                    name: "pgszerrclr",
                    description: Some(
                        "desc PGSZERRCLR.",
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
                    name: "mismtchclr",
                    description: Some(
                        "desc MISMTCHCLR.",
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
                    name: "otpwerr",
                    description: Some(
                        "desc OTPWERR.",
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
                    name: "prtwerr",
                    description: Some(
                        "desc PRTWERR.",
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
                    name: "pgszerr",
                    description: Some(
                        "desc PGSZERR.",
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
                    name: "mismtch",
                    description: Some(
                        "desc MISMTCH.",
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
            name: "Fswp",
            extends: None,
            description: Some(
                "desc FSWP.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "fswp",
                    description: Some(
                        "desc FSWP.",
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
                    name: "pemod",
                    description: Some(
                        "desc PEMOD.",
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
                Field {
                    name: "key1lock",
                    description: Some(
                        "desc KEY1LOCK.",
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
                    name: "key2lock",
                    description: Some(
                        "desc KEY2LOCK.",
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
            name: "MmfRemcr0",
            extends: None,
            description: Some(
                "desc MMF_REMCR0.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "rmsize",
                    description: Some(
                        "desc RMSIZE.",
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
                    name: "rmtaddr",
                    description: Some(
                        "desc RMTADDR.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 12,
                        },
                    ),
                    bit_size: 17,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "en",
                    description: Some(
                        "desc EN.",
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
            name: "MmfRemcr1",
            extends: None,
            description: Some(
                "desc MMF_REMCR1.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "rmsize",
                    description: Some(
                        "desc RMSIZE.",
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
                    name: "rmtaddr",
                    description: Some(
                        "desc RMTADDR.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 12,
                        },
                    ),
                    bit_size: 17,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "en",
                    description: Some(
                        "desc EN.",
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
            name: "MmfRemprt",
            extends: None,
            description: Some(
                "desc MMF_REMPRT.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "mmf_remprt",
                    description: Some(
                        "desc MMF_REMPRT.",
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
            name: "Uqid0",
            extends: None,
            description: Some(
                "desc UQID0.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "y_location",
                    description: Some(
                        "desc Y_LOCATION.",
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
                    name: "x_location",
                    description: Some(
                        "desc X_LOCATION.",
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
                    name: "wafer_id",
                    description: Some(
                        "desc WAFER_ID.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 16,
                        },
                    ),
                    bit_size: 8,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "lot_id",
                    description: Some(
                        "desc LOT_ID.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 24,
                        },
                    ),
                    bit_size: 8,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Uqid1",
            extends: None,
            description: Some(
                "desc UQID1.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "lot_id",
                    description: Some(
                        "desc LOT_ID.",
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
            name: "Uqid2",
            extends: None,
            description: Some(
                "desc UQID2.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "lot_id",
                    description: Some(
                        "desc LOT_ID.",
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
            name: "Wlock",
            extends: None,
            description: Some(
                "desc WLOCK.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "wlock0",
                    description: Some(
                        "desc WLOCK0.",
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
                