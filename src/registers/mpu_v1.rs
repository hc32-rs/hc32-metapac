
                    use crate::metadata::ir::*;
                    pub(crate) static REGISTERS: IR = IR {
    blocks: &[
        Block {
            name: "Mpu",
            extends: None,
            description: Some(
                "desc MPU.",
            ),
            items: &[
                BlockItem {
                    name: "rgd0",
                    description: Some(
                        "desc RGD0.",
                    ),
                    array: None,
                    byte_offset: 0x0,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Rgd0",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "rgd1",
                    description: Some(
                        "desc RGD1.",
                    ),
                    array: None,
                    byte_offset: 0x4,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Rgd1",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "rgd2",
                    description: Some(
                        "desc RGD2.",
                    ),
                    array: None,
                    byte_offset: 0x8,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Rgd2",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "rgd3",
                    description: Some(
                        "desc RGD3.",
                    ),
                    array: None,
                    byte_offset: 0xc,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Rgd3",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "rgd4",
                    description: Some(
                        "desc RGD4.",
                    ),
                    array: None,
                    byte_offset: 0x10,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Rgd4",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "rgd5",
                    description: Some(
                        "desc RGD5.",
                    ),
                    array: None,
                    byte_offset: 0x14,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Rgd5",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "rgd6",
                    description: Some(
                        "desc RGD6.",
                    ),
                    array: None,
                    byte_offset: 0x18,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Rgd6",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "rgd7",
                    description: Some(
                        "desc RGD7.",
                    ),
                    array: None,
                    byte_offset: 0x1c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Rgd7",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "rgd8",
                    description: Some(
                        "desc RGD8.",
                    ),
                    array: None,
                    byte_offset: 0x20,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Rgd8",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "rgd9",
                    description: Some(
                        "desc RGD9.",
                    ),
                    array: None,
                    byte_offset: 0x24,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Rgd9",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "rgd10",
                    description: Some(
                        "desc RGD10.",
                    ),
                    array: None,
                    byte_offset: 0x28,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Rgd10",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "rgd11",
                    description: Some(
                        "desc RGD11.",
                    ),
                    array: None,
                    byte_offset: 0x2c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Rgd11",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "rgd12",
                    description: Some(
                        "desc RGD12.",
                    ),
                    array: None,
                    byte_offset: 0x30,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Rgd12",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "rgd13",
                    description: Some(
                        "desc RGD13.",
                    ),
                    array: None,
                    byte_offset: 0x34,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Rgd13",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "rgd14",
                    description: Some(
                        "desc RGD14.",
                    ),
                    array: None,
                    byte_offset: 0x38,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Rgd14",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "rgd15",
                    description: Some(
                        "desc RGD15.",
                    ),
                    array: None,
                    byte_offset: 0x3c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Rgd15",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "sr",
                    description: Some(
                        "desc SR.",
                    ),
                    array: None,
                    byte_offset: 0x40,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::Read,
                            bit_size: 32,
                            fieldset: Some(
                                "Sr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "eclr",
                    description: Some(
                        "desc ECLR.",
                    ),
                    array: None,
                    byte_offset: 0x44,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::Write,
                            bit_size: 32,
                            fieldset: Some(
                                "Eclr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "wp",
                    description: Some(
                        "desc WP.",
                    ),
                    array: None,
                    byte_offset: 0x48,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Wp",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "ippr",
                    description: Some(
                        "desc IPPR.",
                    ),
                    array: None,
                    byte_offset: 0x4c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Ippr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "msppba",
                    description: Some(
                        "desc MSPPBA.",
                    ),
                    array: None,
                    byte_offset: 0x50,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Msppba",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "msppctl",
                    description: Some(
                        "desc MSPPCTL.",
                    ),
                    array: None,
                    byte_offset: 0x54,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Msppctl",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "psppba",
                    description: Some(
                        "desc PSPPBA.",
                    ),
                    array: None,
                    byte_offset: 0x58,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Psppba",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "psppctl",
                    description: Some(
                        "desc PSPPCTL.",
                    ),
                    array: None,
                    byte_offset: 0x5c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Psppctl",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "s1rge",
                    description: Some(
                        "desc S1RGE.",
                    ),
                    array: None,
                    byte_offset: 0x60,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "S1rge",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "s1rgwp",
                    description: Some(
                        "desc S1RGWP.",
                    ),
                    array: None,
                    byte_offset: 0x64,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "S1rgwp",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "s1rgrp",
                    description: Some(
                        "desc S1RGRP.",
                    ),
                    array: None,
                    byte_offset: 0x68,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "S1rgrp",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "s1cr",
                    description: Some(
                        "desc S1CR.",
                    ),
                    array: None,
                    byte_offset: 0x6c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "S1cr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "s2rge",
                    description: Some(
                        "desc S2RGE.",
                    ),
                    array: None,
                    byte_offset: 0x70,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "S2rge",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "s2rgwp",
                    description: Some(
                        "desc S2RGWP.",
                    ),
                    array: None,
                    byte_offset: 0x74,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "S2rgwp",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "s2rgrp",
                    description: Some(
                        "desc S2RGRP.",
                    ),
                    array: None,
                    byte_offset: 0x78,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "S2rgrp",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "s2cr",
                    description: Some(
                        "desc S2CR.",
                    ),
                    array: None,
                    byte_offset: 0x7c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "S2cr",
                            ),
                        },
                    ),
                },
            ],
        },
    ],
    fieldsets: &[
        FieldSet {
            name: "Eclr",
            extends: None,
            description: Some(
                "desc ECLR.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "smpu1eclr",
                    description: Some(
                        "desc SMPU1ECLR.",
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
                    name: "smpu2eclr",
                    description: Some(
                        "desc SMPU2ECLR.",
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
                    name: "pspeclr",
                    description: Some(
                        "desc PSPECLR.",
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
                    name: "mspeclr",
                    description: Some(
                        "desc MSPECLR.",
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
            ],
        },
        FieldSet {
            name: "Ippr",
            extends: None,
            description: Some(
                "desc IPPR.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "aesrdp",
                    description: Some(
                        "desc AESRDP.",
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
                    name: "aeswrp",
                    description: Some(
                        "desc AESWRP.",
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
                    name: "hashrdp",
                    description: Some(
                        "desc HASHRDP.",
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
                    name: "hashwrp",
                    description: Some(
                        "desc HASHWRP.",
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
                    name: "trngrdp",
                    description: Some(
                        "desc TRNGRDP.",
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
                    name: "trngwrp",
                    description: Some(
                        "desc TRNGWRP.",
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
                    name: "crcrdp",
                    description: Some(
                        "desc CRCRDP.",
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
                    name: "crcwrp",
                    description: Some(
                        "desc CRCWRP.",
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
                    name: "efmrdp",
                    description: Some(
                        "desc EFMRDP.",
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
                    name: "efmwrp",
                    description: Some(
                        "desc EFMWRP.",
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
                    name: "wdtrdp",
                    description: Some(
                        "desc WDTRDP.",
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
                    name: "wdtwrp",
                    description: Some(
                        "desc WDTWRP.",
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
                    name: "swdtrdp",
                    description: Some(
                        "desc SWDTRDP.",
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
                    name: "swdtwrp",
                    description: Some(
                        "desc SWDTWRP.",
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
                    name: "bksramrdp",
                    description: Some(
                        "desc BKSRAMRDP.",
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
                    name: "bksramwrp",
                    description: Some(
                        "desc BKSRAMWRP.",
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
                    name: "rtcrdp",
                    description: Some(
                        "desc RTCRDP.",
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
                    name: "rtcwrp",
                    description: Some(
                        "desc RTCWRP.",
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
                    name: "dmpurdp",
                    description: Some(
                        "desc DMPURDP.",
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
                    name: "dmpuwrp",
                    description: Some(
                        "desc DMPUWRP.",
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
                    name: "sramcrdp",
                    description: Some(
                        "desc SRAMCRDP.",
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
                    name: "sramcwrp",
                    description: Some(
                        "desc SRAMCWRP.",
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
                    name: "intcrdp",
                    description: Some(
                        "desc INTCRDP.",
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
                    name: "intcwrp",
                    description: Some(
                        "desc INTCWRP.",
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
                    name: "syscrdp",
                    description: Some(
                        "desc SYSCRDP.",
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
                    name: "syscwrp",
                    description: Some(
                        "desc SYSCWRP.",
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
                    name: "mstprdp",
                    description: Some(
                        "desc MSTPRDP.",
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
                    name: "msptwrp",
                    description: Some(
                        "desc MSPTWRP.",
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
                    name: "buserre",
                    description: Some(
                        "desc BUSERRE.",
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
            name: "Msppba",
            extends: None,
            description: Some(
                "desc MSPPBA.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "msppba",
                    description: Some(
                        "desc MSPPBA.",
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
            name: "Msppctl",
            extends: None,
            description: Some(
                "desc MSPPCTL.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "msppsize",
                    description: Some(
                        "desc MSPPSIZE.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 2,
                        },
                    ),
                    bit_size: 14,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "msppact",
                    description: Some(
                        "desc MSPPACT.",
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
                    name: "msppe",
                    description: Some(
                        "desc MSPPE.",
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
            name: "Psppba",
            extends: None,
            description: Some(
                "desc PSPPBA.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "psppba",
                    description: Some(
                        "desc PSPPBA.",
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
            name: "Psppctl",
            extends: None,
            description: Some(
                "desc PSPPCTL.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "psppsize",
                    description: Some(
                        "desc PSPPSIZE.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 2,
                        },
                    ),
                    bit_size: 14,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "psppact",
                    description: Some(
                        "desc PSPPACT.",
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
                    name: "psppe",
                    description: Some(
                        "desc PSPPE.",
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
            name: "Rgd0",
            extends: None,
            description: Some(
                "desc RGD0.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "mpurgsize",
                    description: Some(
                        "desc MPURGSIZE.",
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
                    name: "mpurgaddr",
                    description: Some(
                        "desc MPURGADDR.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 5,
                        },
                    ),
                    bit_size: 27,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Rgd1",
            extends: None,
            description: Some(
                "desc RGD1.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "mpurgsize",
                    description: Some(
                        "desc MPURGSIZE.",
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
                    name: "mpurgaddr",
                    description: Some(
                        "desc MPURGADDR.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 5,
                        },
                    ),
                    bit_size: 27,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Rgd10",
            extends: None,
            description: Some(
                "desc RGD10.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "mpurgsize",
                    description: Some(
                        "desc MPURGSIZE.",
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
                    name: "mpurgaddr",
                    description: Some(
                        "desc MPURGADDR.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 5,
                        },
                    ),
                    bit_size: 27,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Rgd11",
            extends: None,
            description: Some(
                "desc RGD11.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "mpurgsize",
                    description: Some(
                        "desc MPURGSIZE.",
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
                    name: "mpurgaddr",
                    description: Some(
                        "desc MPURGADDR.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 5,
                        },
                    ),
                    bit_size: 27,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Rgd12",
            extends: None,
            description: Some(
                "desc RGD12.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "mpurgsize",
                    description: Some(
                        "desc MPURGSIZE.",
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
                    name: "mpurgaddr",
                    description: Some(
                        "desc MPURGADDR.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 5,
                        },
                    ),
                    bit_size: 27,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Rgd13",
            extends: None,
            description: Some(
                "desc RGD13.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "mpurgsize",
                    description: Some(
                        "desc MPURGSIZE.",
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
                    name: "mpurgaddr",
                    description: Some(
                        "desc MPURGADDR.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 5,
                        },
                    ),
                    bit_size: 27,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Rgd14",
            extends: None,
            description: Some(
                "desc RGD14.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "mpurgsize",
                    description: Some(
                        "desc MPURGSIZE.",
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
                    name: "mpurgaddr",
                    description: Some(
                        "desc MPURGADDR.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 5,
                        },
                    ),
                    bit_size: 27,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Rgd15",
            extends: None,
            description: Some(
                "desc RGD15.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "mpurgsize",
                    description: Some(
                        "desc MPURGSIZE.",
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
                    name: "mpurgaddr",
                    description: Some(
                        "desc MPURGADDR.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 5,
                        },
                    ),
                    bit_size: 27,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Rgd2",
            extends: None,
            description: Some(
                "desc RGD2.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "mpurgsize",
                    description: Some(
                        "desc MPURGSIZE.",
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
                    name: "mpurgaddr",
                    description: Some(
                        "desc MPURGADDR.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 5,
                        },
                    ),
                    bit_size: 27,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Rgd3",
            extends: None,
            description: Some(
                "desc RGD3.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "mpurgsize",
                    description: Some(
                        "desc MPURGSIZE.",
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
                    name: "mpurgaddr",
                    description: Some(
                        "desc MPURGADDR.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 5,
                        },
                    ),
                    bit_size: 27,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Rgd4",
            extends: None,
            description: Some(
                "desc RGD4.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "mpurgsize",
                    description: Some(
                        "desc MPURGSIZE.",
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
                    name: "mpurgaddr",
                    description: Some(
                        "desc MPURGADDR.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 5,
                        },
                    ),
                    bit_size: 27,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Rgd5",
            extends: None,
            description: Some(
                "desc RGD5.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "mpurgsize",
                    description: Some(
                        "desc MPURGSIZE.",
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
                    name: "mpurgaddr",
                    description: Some(
                        "desc MPURGADDR.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 5,
                        },
                    ),
                    bit_size: 27,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Rgd6",
            extends: None,
            description: Some(
                "desc RGD6.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "mpurgsize",
                    description: Some(
                        "desc MPURGSIZE.",
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
                    name: "mpurgaddr",
                    description: Some(
                        "desc MPURGADDR.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 5,
                        },
                    ),
                    bit_size: 27,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Rgd7",
            extends: None,
            description: Some(
                "desc RGD7.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "mpurgsize",
                    description: Some(
                        "desc MPURGSIZE.",
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
                    name: "mpurgaddr",
                    description: Some(
                        "desc MPURGADDR.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 5,
                        },
                    ),
                    bit_size: 27,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Rgd8",
            extends: None,
            description: Some(
                "desc RGD8.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "mpurgsize",
                    description: Some(
                        "desc MPURGSIZE.",
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
                    name: "mpurgaddr",
                    description: Some(
                        "desc MPURGADDR.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 5,
                        },
                    ),
                    bit_size: 27,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Rgd9",
            extends: None,
            description: Some(
                "desc RGD9.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "mpurgsize",
                    description: Some(
                        "desc MPURGSIZE.",
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
                    name: "mpurgaddr",
                    description: Some(
                        "desc MPURGADDR.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 5,
                        },
                    ),
                    bit_size: 27,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "S1cr",
            extends: None,
            description: Some(
                "desc S1CR.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "smpubrp",
                    description: Some(
                        "desc SMPUBRP.",
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
                    name: "smpubwp",
                    description: Some(
                        "desc SMPUBWP.",
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
                    name: "smpuact",
                    description: Some(
                        "desc SMPUACT.",
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
                    name: "smpue",
                    description: Some(
                        "desc SMPUE.",
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
            name: "S1rge",
            extends: None,
            description: Some(
                "desc S1RGE.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "rg0e",
                    description: Some(
                        "desc RG0E.",
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
                    name: "rg1e",
                    description: Some(
                        "desc RG1E.",
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
                    name: "rg2e",
                    description: Some(
                        "desc RG2E.",
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
                    name: "rg3e",
                    description: Some(
                        "desc RG3E.",
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
                    name: "rg4e",
                    description: Some(
                        "desc RG4E.",
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
                    name: "rg5e",
                    description: Some(
                        "desc RG5E.",
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
                    name: "rg6e",
                    description: Some(
                        "desc RG6E.",
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
                    name: "rg7e",
                    description: Some(
                        "desc RG7E.",
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
                    name: "rg8e",
                    description: Some(
                        "desc RG8E.",
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
                    name: "rg9e",
                    description: Some(
                        "desc RG9E.",
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
                    name: "rg10e",
                    description: Some(
                        "desc RG10E.",
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
                    name: "rg11e",
                    description: Some(
                        "desc RG11E.",
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
                    name: "rg12e",
                    description: Some(
                        "desc RG12E.",
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
                    name: "rg13e",
                    description: Some(
                        "desc RG13E.",
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
                    name: "rg14e",
                    description: Some(
                        "desc RG14E.",
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
                    name: "rg15e",
                    description: Some(
                        "desc RG15E.",
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
            name: "S1rgrp",
            extends: None,
            description: Some(
                "desc S1RGRP.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "rg0rp",
                    description: Some(
                        "desc RG0RP.",
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
                    name: "rg1rp",
                    description: Some(
                        "desc RG1RP.",
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
                    name: "rg2rp",
                    description: Some(
                        "desc RG2RP.",
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
                    name: "rg3rp",
                    description: Some(
                        "desc RG3RP.",
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
                    name: "rg4rp",
                    description: Some(
                        "desc RG4RP.",
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
                    name: "rg5rp",
                    description: Some(
                        "desc RG5RP.",
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
                    name: "rg6rp",
                    description: Some(
                        "desc RG6RP.",
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
                    name: "rg7rp",
                    description: Some(
                        "desc RG7RP.",
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
                    name: "rg8rp",
                    description: Some(
                        "desc RG8RP.",
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
                    name: "rg9rp",
                    description: Some(
                        "desc RG9RP.",
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
                    name: "rg10rp",
                    description: Some(
                        "desc RG10RP.",
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
                    name: "rg11rp",
                    description: Some(
                        "desc RG11RP.",
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
                    name: "rg12rp",
                    description: Some(
                        "desc RG12RP.",
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
                    name: "rg13rp",
                    description: Some(
                        "desc RG13RP.",
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
                    name: "rg14rp",
                    description: Some(
                        "desc RG14RP.",
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
                    name: "rg15rp",
                    description: Some(
                        "desc RG15RP.",
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
            name: "S1rgwp",
            extends: None,
            description: Some(
                "desc S1RGWP.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "rg0wp",
                    description: Some(
                        "desc RG0WP.",
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
                    name: "rg1wp",
                    description: Some(
                        "desc RG1WP.",
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
                    name: "rg2wp",
                    description: Some(
                        "desc RG2WP.",
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
                    name: "rg3wp",
                    description: Some(
                        "desc RG3WP.",
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
                    name: "rg4wp",
                    description: Some(
                        "desc RG4WP.",
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
                    name: "rg5wp",
                    description: Some(
                        "desc RG5WP.",
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
                    name: "rg6wp",
                    description: Some(
                        "desc RG6WP.",
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
                    name: "rg7wp",
                    description: Some(
                        "desc RG7WP.",
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
                    name: "rg8wp",
                    description: Some(
                        "desc RG8WP.",
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
                    name: "rg9wp",
                    description: Some(
                        "desc RG9WP.",
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
                    name: "rg10wp",
                    description: Some(
                        "desc RG10WP.",
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
                    name: "rg11wp",
                    description: Some(
                        "desc RG11WP.",
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
                    name: "rg12wp",
                    description: Some(
                        "desc RG12WP.",
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
                    name: "rg13wp",
                    description: Some(
                        "desc RG13WP.",
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
                    name: "rg14wp",
                    description: Some(
                        "desc RG14WP.",
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
                    name: "rg15wp",
                    description: Some(
                        "desc RG15WP.",
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
            name: "S2cr",
            extends: None,
            description: Some(
                "desc S2CR.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "smpubrp",
                    description: Some(
                        "desc SMPUBRP.",
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
                    name: "smpubwp",
                    description: Some(
                        "desc SMPUBWP.",
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
                    name: "smpuact",
                    description: Some(
                        "desc SMPUACT.",
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
                    name: "smpue",
                    description: Some(
                        "desc SMPUE.",
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
            name: "S2rge",
            extends: None,
            description: Some(
                "desc S2RGE.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "rg0e",
                    description: Some(
                        "desc RG0E.",
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
                    name: "rg1e",
                    description: Some(
                        "desc RG1E.",
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
                    name: "rg2e",
                    description: Some(
                        "desc RG2E.",
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
                    name: "rg3e",
                    description: Some(
                        "desc RG3E.",
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
                    name: "rg4e",
                    description: Some(
                        "desc RG4E.",
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
                    name: "rg5e",
                    description: Some(
                        "desc RG5E.",
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
                    name: "rg6e",
                    description: Some(
                        "desc RG6E.",
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
                    name: "rg7e",
                    description: Some(
                        "desc RG7E.",
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
                    name: "rg8e",
                    description: Some(
                        "desc RG8E.",
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
                    name: "rg9e",
                    description: Some(
                        "desc RG9E.",
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
                    name: "rg10e",
                    description: Some(
                        "desc RG10E.",
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
                    name: "rg11e",
                    description: Some(
                        "desc RG11E.",
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
                    name: "rg12e",
                    description: Some(
                        "desc RG12E.",
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
                    name: "rg13e",
                    description: Some(
                        "desc RG13E.",
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
                    name: "rg14e",
                    description: Some(
                        "desc RG14E.",
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
                    name: "rg15e",
                    description: Some(
                        "desc RG15E.",
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
            name: "S2rgrp",
            extends: None,
            description: Some(
                "desc S2RGRP.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "rg0rp",
                    description: Some(
                        "desc RG0RP.",
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
                    name: "rg1rp",
                    description: Some(
                        "desc RG1RP.",
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
                    name: "rg2rp",
                    description: Some(
                        "desc RG2RP.",
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
                    name: "rg3rp",
                    description: Some(
                        "desc RG3RP.",
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
                    name: "rg4rp",
                    description: Some(
                        "desc RG4RP.",
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
                    name: "rg5rp",
                    description: Some(
                        "desc RG5RP.",
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
                    name: "rg6rp",
                    description: Some(
                        "desc RG6RP.",
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
                    name: "rg7rp",
                    description: Some(
                        "desc RG7RP.",
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
                    name: "rg8rp",
                    description: Some(
                        "desc RG8RP.",
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
                    name: "rg9rp",
                    description: Some(
                        "desc RG9RP.",
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
                    name: "rg10rp",
                    description: Some(
                        "desc RG10RP.",
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
                    name: "rg11rp",
                    description: Some(
                        "desc RG11RP.",
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
                    name: "rg12rp",
                    description: Some(
                        "desc RG12RP.",
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
                    name: "rg13rp",
                    description: Some(
                        "desc RG13RP.",
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
                    name: "rg14rp",
                    description: Some(
                        "desc RG14RP.",
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
                    name: "rg15rp",
                    description: Some(
                        "desc RG15RP.",
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
            name: "S2rgwp",
            extends: None,
            description: Some(
                "desc S2RGWP.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "rg0wp",
                    description: Some(
                        "desc RG0WP.",
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
                    name: "rg1wp",
                    description: Some(
                        "desc RG1WP.",
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
                    name: "rg2wp",
                    description: Some(
                        "desc RG2WP.",
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
                    name: "rg3wp",
                    description: Some(
                        "desc RG3WP.",
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
                    name: "rg4wp",
                    description: Some(
                        "desc RG4WP.",
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
                    name: "rg5wp",
                    description: Some(
                        "desc RG5WP.",
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
                    name: "rg6wp",
                    description: Some(
                        "desc RG6WP.",
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
                    name: "rg7wp",
                    description: Some(
                        "desc RG7WP.",
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
                    name: "rg8wp",
                    description: Some(
                        "desc RG8WP.",
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
                    name: "rg9wp",
                    description: Some(
                        "desc RG9WP.",
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
                    name: "rg10wp",
                    description: Some(
                        "desc RG10WP.",
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
                    name: "rg11wp",
                    description: Some(
                        "desc RG11WP.",
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
                    name: "rg12wp",
                    description: Some(
                        "desc RG12WP.",
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
                    name: "rg13wp",
                    description: Some(
                        "desc RG13WP.",
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
                    name: "rg14wp",
                    description: Some(
                        "desc RG14WP.",
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
                    name: "rg15wp",
                    description: Some(
                        "desc RG15WP.",
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
            name: "Sr",
            extends: None,
            description: Some(
                "desc SR.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "smpu1eaf",
                    description: Some(
                        "desc SMPU1EAF.",
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
                    name: "smpu2eaf",
                    description: Some(
                        "desc SMPU2EAF.",
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
                    name: "pspef",
                    description: Some(
                        "desc PSPEF.",
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
                    name: "mspef",
                    description: Some(
                        "desc MSPEF.",
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
            ],
        },
        FieldSet {
            name: "Wp",
            extends: None,
            description: Some(
                "desc WP.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "mpuwe",
                    description: Some(
                        "desc MPUWE.",
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
                    name: "wkey",
                    description: Some(
                        "desc WKEY.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 1,
                        },
                    ),
                    bit_size: 15,
                    array: None,
                    enumm: None,
                },
            ],
        },
    ],
    enums: &[],
};
                