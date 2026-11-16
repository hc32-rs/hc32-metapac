
                    use crate::metadata::ir::*;
                    pub(crate) static REGISTERS: IR = IR {
    blocks: &[
        Block {
            name: "Nfc",
            extends: None,
            description: Some(
                "desc NFC.",
            ),
            items: &[
                BlockItem {
                    name: "datr_base",
                    description: Some(
                        "desc DATR_BASE.",
                    ),
                    array: None,
                    byte_offset: 0x0,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: None,
                        },
                    ),
                },
                BlockItem {
                    name: "cmdr",
                    description: Some(
                        "desc CMDR.",
                    ),
                    array: None,
                    byte_offset: 0x8000,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Cmdr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "idxr0",
                    description: Some(
                        "desc IDXR0.",
                    ),
                    array: None,
                    byte_offset: 0x8004,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: None,
                        },
                    ),
                },
                BlockItem {
                    name: "ienr",
                    description: Some(
                        "desc IENR.",
                    ),
                    array: None,
                    byte_offset: 0x8030,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Ienr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "istr",
                    description: Some(
                        "desc ISTR.",
                    ),
                    array: None,
                    byte_offset: 0x8034,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Istr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "irsr",
                    description: Some(
                        "desc IRSR.",
                    ),
                    array: None,
                    byte_offset: 0x8038,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Irsr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "idxr1",
                    description: Some(
                        "desc IDXR1.",
                    ),
                    array: None,
                    byte_offset: 0x8048,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Idxr1",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "tmcr0",
                    description: Some(
                        "desc TMCR0.",
                    ),
                    array: None,
                    byte_offset: 0x804c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Tmcr0",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "tmcr1",
                    description: Some(
                        "desc TMCR1.",
                    ),
                    array: None,
                    byte_offset: 0x8050,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Tmcr1",
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
                    byte_offset: 0x8054,
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
                BlockItem {
                    name: "tmcr2",
                    description: Some(
                        "desc TMCR2.",
                    ),
                    array: None,
                    byte_offset: 0x805c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Tmcr2",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "eccr0",
                    description: Some(
                        "desc ECCR0.",
                    ),
                    array: None,
                    byte_offset: 0x8060,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::Read,
                            bit_size: 32,
                            fieldset: Some(
                                "Eccr0",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "eccr1",
                    description: Some(
                        "desc ECCR1.",
                    ),
                    array: None,
                    byte_offset: 0x8064,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::Read,
                            bit_size: 32,
                            fieldset: Some(
                                "Eccr1",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "eccr2",
                    description: Some(
                        "desc ECCR2.",
                    ),
                    array: None,
                    byte_offset: 0x8068,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::Read,
                            bit_size: 32,
                            fieldset: Some(
                                "Eccr2",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "eccr3",
                    description: Some(
                        "desc ECCR3.",
                    ),
                    array: None,
                    byte_offset: 0x806c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::Read,
                            bit_size: 32,
                            fieldset: Some(
                                "Eccr3",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "eccr4",
                    description: Some(
                        "desc ECCR4.",
                    ),
                    array: None,
                    byte_offset: 0x8070,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::Read,
                            bit_size: 32,
                            fieldset: Some(
                                "Eccr4",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "eccr5",
                    description: Some(
                        "desc ECCR5.",
                    ),
                    array: None,
                    byte_offset: 0x8074,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::Read,
                            bit_size: 32,
                            fieldset: Some(
                                "Eccr5",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "eccr6",
                    description: Some(
                        "desc ECCR6.",
                    ),
                    array: None,
                    byte_offset: 0x8078,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::Read,
                            bit_size: 32,
                            fieldset: Some(
                                "Eccr6",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "eccr7",
                    description: Some(
                        "desc ECCR7.",
                    ),
                    array: None,
                    byte_offset: 0x807c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::Read,
                            bit_size: 32,
                            fieldset: Some(
                                "Eccr7",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "ecc_stat",
                    description: Some(
                        "desc ECC_STAT.",
                    ),
                    array: None,
                    byte_offset: 0x807c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::Read,
                            bit_size: 32,
                            fieldset: Some(
                                "EccStat",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "eccr8",
                    description: Some(
                        "desc ECCR8.",
                    ),
                    array: None,
                    byte_offset: 0x8080,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::Read,
                            bit_size: 32,
                            fieldset: Some(
                                "Eccr8",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "ecc_synd0_0",
                    description: Some(
                        "desc ECC_SYND0_0.",
                    ),
                    array: None,
                    byte_offset: 0x8080,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::Read,
                            bit_size: 32,
                            fieldset: Some(
                                "EccSynd00",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "eccr9",
                    description: Some(
                        "desc ECCR9.",
                    ),
                    array: None,
                    byte_offset: 0x8084,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::Read,
                            bit_size: 32,
                            fieldset: Some(
                                "Eccr9",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "ecc_synd0_1",
                    description: Some(
                        "desc ECC_SYND0_1.",
                    ),
                    array: None,
                    byte_offset: 0x8084,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::Read,
                            bit_size: 32,
                            fieldset: Some(
                                "EccSynd01",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "eccr10",
                    description: Some(
                        "desc ECCR10.",
                    ),
                    array: None,
                    byte_offset: 0x8088,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::Read,
                            bit_size: 32,
                            fieldset: Some(
                                "Eccr10",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "ecc_synd0_2",
                    description: Some(
                        "desc ECC_SYND0_2.",
                    ),
                    array: None,
                    byte_offset: 0x8088,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::Read,
                            bit_size: 32,
                            fieldset: Some(
                                "EccSynd02",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "eccr11",
                    description: Some(
                        "desc ECCR11.",
                    ),
                    array: None,
                    byte_offset: 0x808c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::Read,
                            bit_size: 32,
                            fieldset: Some(
                                "Eccr11",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "ecc_synd0_3",
                    description: Some(
                        "desc ECC_SYND0_3.",
                    ),
                    array: None,
                    byte_offset: 0x808c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::Read,
                            bit_size: 32,
                            fieldset: Some(
                                "EccSynd03",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "eccr12",
                    description: Some(
                        "desc ECCR12.",
                    ),
                    array: None,
                    byte_offset: 0x8090,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::Read,
                            bit_size: 32,
                            fieldset: Some(
                                "Eccr12",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "ecc_synd1_0",
                    description: Some(
                        "desc ECC_SYND1_0.",
                    ),
                    array: None,
                    byte_offset: 0x8090,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::Read,
                            bit_size: 32,
                            fieldset: Some(
                                "EccSynd10",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "eccr13",
                    description: Some(
                        "desc ECCR13.",
                    ),
                    array: None,
                    byte_offset: 0x8094,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::Read,
                            bit_size: 32,
                            fieldset: Some(
                                "Eccr13",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "ecc_synd1_1",
                    description: Some(
                        "desc ECC_SYND1_1.",
                    ),
                    array: None,
                    byte_offset: 0x8094,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::Read,
                            bit_size: 32,
                            fieldset: Some(
                                "EccSynd11",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "eccr14",
                    description: Some(
                        "desc ECCR14.",
                    ),
                    array: None,
                    byte_offset: 0x8098,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::Read,
                            bit_size: 32,
                            fieldset: Some(
                                "Eccr14",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "ecc_synd1_2",
                    description: Some(
                        "desc ECC_SYND1_2.",
                    ),
                    array: None,
                    byte_offset: 0x8098,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::Read,
                            bit_size: 32,
                            fieldset: Some(
                                "EccSynd12",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "eccr15",
                    description: Some(
                        "desc ECCR15.",
                    ),
                    array: None,
                    byte_offset: 0x809c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::Read,
                            bit_size: 32,
                            fieldset: Some(
                                "Eccr15",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "ecc_synd1_3",
                    description: Some(
                        "desc ECC_SYND1_3.",
                    ),
                    array: None,
                    byte_offset: 0x809c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::Read,
                            bit_size: 32,
                            fieldset: Some(
                                "EccSynd13",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "ecc_synd2_0",
                    description: Some(
                        "desc ECC_SYND2_0.",
                    ),
                    array: None,
                    byte_offset: 0x80a0,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::Read,
                            bit_size: 32,
                            fieldset: Some(
                                "EccSynd20",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "ecc_synd2_1",
                    description: Some(
                        "desc ECC_SYND2_1.",
                    ),
                    array: None,
                    byte_offset: 0x80a4,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::Read,
                            bit_size: 32,
                            fieldset: Some(
                                "EccSynd21",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "ecc_synd2_2",
                    description: Some(
                        "desc ECC_SYND2_2.",
                    ),
                    array: None,
                    byte_offset: 0x80a8,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::Read,
                            bit_size: 32,
                            fieldset: Some(
                                "EccSynd22",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "ecc_synd2_3",
                    description: Some(
                        "desc ECC_SYND2_3.",
                    ),
                    array: None,
                    byte_offset: 0x80ac,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::Read,
                            bit_size: 32,
                            fieldset: Some(
                                "EccSynd23",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "ecc_synd3_0",
                    description: Some(
                        "desc ECC_SYND3_0.",
                    ),
                    array: None,
                    byte_offset: 0x80b0,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::Read,
                            bit_size: 32,
                            fieldset: Some(
                                "EccSynd30",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "ecc_synd3_1",
                    description: Some(
                        "desc ECC_SYND3_1.",
                    ),
                    array: None,
                    byte_offset: 0x80b4,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::Read,
                            bit_size: 32,
                            fieldset: Some(
                                "EccSynd31",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "ecc_synd3_2",
                    description: Some(
                        "desc ECC_SYND3_2.",
                    ),
                    array: None,
                    byte_offset: 0x80b8,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::Read,
                            bit_size: 32,
                            fieldset: Some(
                                "EccSynd32",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "ecc_synd3_3",
                    description: Some(
                        "desc ECC_SYND3_3.",
                    ),
                    array: None,
                    byte_offset: 0x80bc,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::Read,
                            bit_size: 32,
                            fieldset: Some(
                                "EccSynd33",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "ecc_synd4_0",
                    description: Some(
                        "desc ECC_SYND4_0.",
                    ),
                    array: None,
                    byte_offset: 0x80c0,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::Read,
                            bit_size: 32,
                            fieldset: Some(
                                "EccSynd40",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "ecc_synd4_1",
                    description: Some(
                        "desc ECC_SYND4_1.",
                    ),
                    array: None,
                    byte_offset: 0x80c4,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::Read,
                            bit_size: 32,
                            fieldset: Some(
                                "EccSynd41",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "ecc_synd4_2",
                    description: Some(
                        "desc ECC_SYND4_2.",
                    ),
                    array: None,
                    byte_offset: 0x80c8,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::Read,
                            bit_size: 32,
                            fieldset: Some(
                                "EccSynd42",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "ecc_synd4_3",
                    description: Some(
                        "desc ECC_SYND4_3.",
                    ),
                    array: None,
                    byte_offset: 0x80cc,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::Read,
                            bit_size: 32,
                            fieldset: Some(
                                "EccSynd43",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "ecc_synd5_0",
                    description: Some(
                        "desc ECC_SYND5_0.",
                    ),
                    array: None,
                    byte_offset: 0x80d0,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::Read,
                            bit_size: 32,
                            fieldset: Some(
                                "EccSynd50",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "ecc_synd5_1",
                    description: Some(
                        "desc ECC_SYND5_1.",
                    ),
                    array: None,
                    byte_offset: 0x80d4,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::Read,
                            bit_size: 32,
                            fieldset: Some(
                                "EccSynd51",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "ecc_synd5_2",
                    description: Some(
                        "desc ECC_SYND5_2.",
                    ),
                    array: None,
                    byte_offset: 0x80d8,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::Read,
                            bit_size: 32,
                            fieldset: Some(
                                "EccSynd52",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "ecc_synd5_3",
                    description: Some(
                        "desc ECC_SYND5_3.",
                    ),
                    array: None,
                    byte_offset: 0x80dc,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::Read,
                            bit_size: 32,
                            fieldset: Some(
                                "EccSynd53",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "ecc_synd6_0",
                    description: Some(
                        "desc ECC_SYND6_0.",
                    ),
                    array: None,
                    byte_offset: 0x80e0,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::Read,
                            bit_size: 32,
                            fieldset: Some(
                                "EccSynd60",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "ecc_synd6_1",
                    description: Some(
                        "desc ECC_SYND6_1.",
                    ),
                    array: None,
                    byte_offset: 0x80e4,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::Read,
                            bit_size: 32,
                            fieldset: Some(
                                "EccSynd61",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "ecc_synd6_2",
                    description: Some(
                        "desc ECC_SYND6_2.",
                    ),
                    array: None,
                    byte_offset: 0x80e8,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::Read,
                            bit_size: 32,
                            fieldset: Some(
                                "EccSynd62",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "ecc_synd6_3",
                    description: Some(
                        "desc ECC_SYND6_3.",
                    ),
                    array: None,
                    byte_offset: 0x80ec,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::Read,
                            bit_size: 32,
                            fieldset: Some(
                                "EccSynd63",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "ecc_synd7_0",
                    description: Some(
                        "desc ECC_SYND7_0.",
                    ),
                    array: None,
                    byte_offset: 0x80f0,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::Read,
                            bit_size: 32,
                            fieldset: Some(
                                "EccSynd70",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "ecc_synd7_1",
                    description: Some(
                        "desc ECC_SYND7_1.",
                    ),
                    array: None,
                    byte_offset: 0x80f4,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::Read,
                            bit_size: 32,
                            fieldset: Some(
                                "EccSynd71",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "ecc_synd7_2",
                    description: Some(
                        "desc ECC_SYND7_2.",
                    ),
                    array: None,
                    byte_offset: 0x80f8,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::Read,
                            bit_size: 32,
                            fieldset: Some(
                                "EccSynd72",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "ecc_synd7_3",
                    description: Some(
                        "desc ECC_SYND7_3.",
                    ),
                    array: None,
                    byte_offset: 0x80fc,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::Read,
                            bit_size: 32,
                            fieldset: Some(
                                "EccSynd73",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "ecc_synd8_0",
                    description: Some(
                        "desc ECC_SYND8_0.",
                    ),
                    array: None,
                    byte_offset: 0x8100,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::Read,
                            bit_size: 32,
                            fieldset: Some(
                                "EccSynd80",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "ecc_synd8_1",
                    description: Some(
                        "desc ECC_SYND8_1.",
                    ),
                    array: None,
                    byte_offset: 0x8104,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::Read,
                            bit_size: 32,
                            fieldset: Some(
                                "EccSynd81",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "ecc_synd8_2",
                    description: Some(
                        "desc ECC_SYND8_2.",
                    ),
                    array: None,
                    byte_offset: 0x8108,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::Read,
                            bit_size: 32,
                            fieldset: Some(
                                "EccSynd82",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "ecc_synd8_3",
                    description: Some(
                        "desc ECC_SYND8_3.",
                    ),
                    array: None,
                    byte_offset: 0x810c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::Read,
                            bit_size: 32,
                            fieldset: Some(
                                "EccSynd83",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "ecc_synd9_0",
                    description: Some(
                        "desc ECC_SYND9_0.",
                    ),
                    array: None,
                    byte_offset: 0x8110,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::Read,
                            bit_size: 32,
                            fieldset: Some(
                                "EccSynd90",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "ecc_synd9_1",
                    description: Some(
                        "desc ECC_SYND9_1.",
                    ),
                    array: None,
                    byte_offset: 0x8114,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::Read,
                            bit_size: 32,
                            fieldset: Some(
                                "EccSynd91",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "ecc_synd9_2",
                    description: Some(
                        "desc ECC_SYND9_2.",
                    ),
                    array: None,
                    byte_offset: 0x8118,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::Read,
                            bit_size: 32,
                            fieldset: Some(
                                "EccSynd92",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "ecc_synd9_3",
                    description: Some(
                        "desc ECC_SYND9_3.",
                    ),
                    array: None,
                    byte_offset: 0x811c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::Read,
                            bit_size: 32,
                            fieldset: Some(
                                "EccSynd93",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "ecc_synd10_0",
                    description: Some(
                        "desc ECC_SYND10_0.",
                    ),
                    array: None,
                    byte_offset: 0x8120,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::Read,
                            bit_size: 32,
                            fieldset: Some(
                                "EccSynd100",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "ecc_synd10_1",
                    description: Some(
                        "desc ECC_SYND10_1.",
                    ),
                    array: None,
                    byte_offset: 0x8124,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::Read,
                            bit_size: 32,
                            fieldset: Some(
                                "EccSynd101",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "ecc_synd10_2",
                    description: Some(
                        "desc ECC_SYND10_2.",
                    ),
                    array: None,
                    byte_offset: 0x8128,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::Read,
                            bit_size: 32,
                            fieldset: Some(
                                "EccSynd102",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "ecc_synd10_3",
                    description: Some(
                        "desc ECC_SYND10_3.",
                    ),
                    array: None,
                    byte_offset: 0x812c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::Read,
                            bit_size: 32,
                            fieldset: Some(
                                "EccSynd103",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "ecc_synd11_0",
                    description: Some(
                        "desc ECC_SYND11_0.",
                    ),
                    array: None,
                    byte_offset: 0x8130,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::Read,
                            bit_size: 32,
                            fieldset: Some(
                                "EccSynd110",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "ecc_synd11_1",
                    description: Some(
                        "desc ECC_SYND11_1.",
                    ),
                    array: None,
                    byte_offset: 0x8134,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::Read,
                            bit_size: 32,
                            fieldset: Some(
                                "EccSynd111",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "ecc_synd11_2",
                    description: Some(
                        "desc ECC_SYND11_2.",
                    ),
                    array: None,
                    byte_offset: 0x8138,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::Read,
                            bit_size: 32,
                            fieldset: Some(
                                "EccSynd112",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "ecc_synd11_3",
                    description: Some(
                        "desc ECC_SYND11_3.",
                    ),
                    array: None,
                    byte_offset: 0x813c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::Read,
                            bit_size: 32,
                            fieldset: Some(
                                "EccSynd113",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "ecc_synd12_0",
                    description: Some(
                        "desc ECC_SYND12_0.",
                    ),
                    array: None,
                    byte_offset: 0x8140,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::Read,
                            bit_size: 32,
                            fieldset: Some(
                                "EccSynd120",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "ecc_synd12_1",
                    description: Some(
                        "desc ECC_SYND12_1.",
                    ),
                    array: None,
                    byte_offset: 0x8144,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::Read,
                            bit_size: 32,
                            fieldset: Some(
                                "EccSynd121",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "ecc_synd12_2",
                    description: Some(
                        "desc ECC_SYND12_2.",
                    ),
                    array: None,
                    byte_offset: 0x8148,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::Read,
                            bit_size: 32,
                            fieldset: Some(
                                "EccSynd122",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "ecc_synd12_3",
                    description: Some(
                        "desc ECC_SYND12_3.",
                    ),
                    array: None,
                    byte_offset: 0x814c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::Read,
                            bit_size: 32,
                            fieldset: Some(
                                "EccSynd123",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "ecc_synd13_0",
                    description: Some(
                        "desc ECC_SYND13_0.",
                    ),
                    array: None,
                    byte_offset: 0x8150,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::Read,
                            bit_size: 32,
                            fieldset: Some(
                                "EccSynd130",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "ecc_synd13_1",
                    description: Some(
                        "desc ECC_SYND13_1.",
                    ),
                    array: None,
                    byte_offset: 0x8154,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::Read,
                            bit_size: 32,
                            fieldset: Some(
                                "EccSynd131",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "ecc_synd13_2",
                    description: Some(
                        "desc ECC_SYND13_2.",
                    ),
                    array: None,
                    byte_offset: 0x8158,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::Read,
                            bit_size: 32,
                            fieldset: Some(
                                "EccSynd132",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "ecc_synd13_3",
                    description: Some(
                        "desc ECC_SYND13_3.",
                    ),
                    array: None,
                    byte_offset: 0x815c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::Read,
                            bit_size: 32,
                            fieldset: Some(
                                "EccSynd133",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "ecc_synd14_0",
                    description: Some(
                        "desc ECC_SYND14_0.",
                    ),
                    array: None,
                    byte_offset: 0x8160,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::Read,
                            bit_size: 32,
                            fieldset: Some(
                                "EccSynd140",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "ecc_synd14_1",
                    description: Some(
                        "desc ECC_SYND14_1.",
                    ),
                    array: None,
                    byte_offset: 0x8164,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::Read,
                            bit_size: 32,
                            fieldset: Some(
                                "EccSynd141",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "ecc_synd14_2",
                    description: Some(
                        "desc ECC_SYND14_2.",
                    ),
                    array: None,
                    byte_offset: 0x8168,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::Read,
                            bit_size: 32,
                            fieldset: Some(
                                "EccSynd142",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "ecc_synd14_3",
                    description: Some(
                        "desc ECC_SYND14_3.",
                    ),
                    array: None,
                    byte_offset: 0x816c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::Read,
                            bit_size: 32,
                            fieldset: Some(
                                "EccSynd143",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "ecc_synd15_0",
                    description: Some(
                        "desc ECC_SYND15_0.",
                    ),
                    array: None,
                    byte_offset: 0x8170,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::Read,
                            bit_size: 32,
                            fieldset: Some(
                                "EccSynd150",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "ecc_synd15_1",
                    description: Some(
                        "desc ECC_SYND15_1.",
                    ),
                    array: None,
                    byte_offset: 0x8174,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::Read,
                            bit_size: 32,
                            fieldset: Some(
                                "EccSynd151",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "ecc_synd15_2",
                    description: Some(
                        "desc ECC_SYND15_2.",
                    ),
                    array: None,
                    byte_offset: 0x8178,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::Read,
                            bit_size: 32,
                            fieldset: Some(
                                "EccSynd152",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "ecc_synd15_3",
                    description: Some(
                        "desc ECC_SYND15_3.",
                    ),
                    array: None,
                    byte_offset: 0x817c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::Read,
                            bit_size: 32,
                            fieldset: Some(
                                "EccSynd153",
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
                    name: "size",
                    description: Some(
                        "desc SIZE.",
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
                    name: "b16bit",
                    description: Some(
                        "desc B16BIT.",
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
                    name: "bank",
                    description: Some(
                        "desc BANK.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 6,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "page",
                    description: Some(
                        "desc PAGE.",
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
                    name: "wp",
                    description: Some(
                        "desc WP.",
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
                    name: "eccm",
                    description: Some(
                        "desc ECCM.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 11,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "rac",
                    description: Some(
                        "desc RAC.",
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
                    name: "scs",
                    description: Some(
                        "desc SCS.",
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
                    name: "cmd",
                    description: Some(
                        "desc CMD.",
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
                    name: "arg",
                    description: Some(
                        "desc ARG.",
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
            name: "EccStat",
            extends: None,
            description: Some(
                "desc ECC_STAT.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "errsec",
                    description: Some(
                        "desc ERRSEC.",
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
            name: "EccSynd00",
            extends: None,
            description: Some(
                "desc ECC_SYND0_0.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "s1",
                    description: Some(
                        "desc S1.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 13,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "s2",
                    description: Some(
                        "desc S2.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 16,
                        },
                    ),
                    bit_size: 13,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "EccSynd01",
            extends: None,
            description: Some(
                "desc ECC_SYND0_1.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "s3",
                    description: Some(
                        "desc S3.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 13,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "s4",
                    description: Some(
                        "desc S4.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 16,
                        },
                    ),
                    bit_size: 13,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "EccSynd02",
            extends: None,
            description: Some(
                "desc ECC_SYND0_2.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "s5",
                    description: Some(
                        "desc S5.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 13,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "s6",
                    description: Some(
                        "desc S6.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 16,
                        },
                    ),
                    bit_size: 13,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "EccSynd03",
            extends: None,
            description: Some(
                "desc ECC_SYND0_3.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "s7",
                    description: Some(
                        "desc S7.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 13,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "s8",
                    description: Some(
                        "desc S8.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 16,
                        },
                    ),
                    bit_size: 13,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "EccSynd10",
            extends: None,
            description: Some(
                "desc ECC_SYND1_0.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "s1",
                    description: Some(
                        "desc S1.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 13,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "s2",
                    description: Some(
                        "desc S2.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 16,
                        },
                    ),
                    bit_size: 13,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "EccSynd100",
            extends: None,
            description: Some(
                "desc ECC_SYND10_0.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "s1",
                    description: Some(
                        "desc S1.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 13,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "s2",
                    description: Some(
                        "desc S2.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 16,
                        },
                    ),
                    bit_size: 13,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "EccSynd101",
            extends: None,
            description: Some(
                "desc ECC_SYND10_1.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "s3",
                    description: Some(
                        "desc S3.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 13,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "s4",
                    description: Some(
                        "desc S4.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 16,
                        },
                    ),
                    bit_size: 13,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "EccSynd102",
            extends: None,
            description: Some(
                "desc ECC_SYND10_2.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "s5",
                    description: Some(
                        "desc S5.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 13,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "s6",
                    description: Some(
                        "desc S6.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 16,
                        },
                    ),
                    bit_size: 13,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "EccSynd103",
            extends: None,
            description: Some(
                "desc ECC_SYND10_3.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "s7",
                    description: Some(
                        "desc S7.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 13,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "s8",
                    description: Some(
                        "desc S8.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 16,
                        },
                    ),
                    bit_size: 13,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "EccSynd11",
            extends: None,
            description: Some(
                "desc ECC_SYND1_1.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "s3",
                    description: Some(
                        "desc S3.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 13,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "s4",
                    description: Some(
                        "desc S4.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 16,
                        },
                    ),
                    bit_size: 13,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "EccSynd110",
            extends: None,
            description: Some(
                "desc ECC_SYND11_0.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "s1",
                    description: Some(
                        "desc S1.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 13,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "s2",
                    description: Some(
                        "desc S2.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 16,
                        },
                    ),
                    bit_size: 13,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "EccSynd111",
            extends: None,
            description: Some(
                "desc ECC_SYND11_1.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "s3",
                    description: Some(
                        "desc S3.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 13,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "s4",
                    description: Some(
                        "desc S4.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 16,
                        },
                    ),
                    bit_size: 13,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "EccSynd112",
            extends: None,
            description: Some(
                "desc ECC_SYND11_2.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "s5",
                    description: Some(
                        "desc S5.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 13,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "s6",
                    description: Some(
                        "desc S6.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 16,
                        },
                    ),
                    bit_size: 13,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "EccSynd113",
            extends: None,
            description: Some(
                "desc ECC_SYND11_3.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "s7",
                    description: Some(
                        "desc S7.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 13,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "s8",
                    description: Some(
                        "desc S8.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 16,
                        },
                    ),
                    bit_size: 13,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "EccSynd12",
            extends: None,
            description: Some(
                "desc ECC_SYND1_2.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "s5",
                    description: Some(
                        "desc S5.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 13,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "s6",
                    description: Some(
                        "desc S6.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 16,
                        },
                    ),
                    bit_size: 13,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "EccSynd120",
            extends: None,
            description: Some(
                "desc ECC_SYND12_0.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "s1",
                    description: Some(
                        "desc S1.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 13,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "s2",
                    description: Some(
                        "desc S2.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 16,
                        },
                    ),
                    bit_size: 13,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "EccSynd121",
            extends: None,
            description: Some(
                "desc ECC_SYND12_1.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "s3",
                    description: Some(
                        "desc S3.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 13,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "s4",
                    description: Some(
                        "desc S4.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 16,
                        },
                    ),
                    bit_size: 13,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "EccSynd122",
            extends: None,
            description: Some(
                "desc ECC_SYND12_2.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "s5",
                    description: Some(
                        "desc S5.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 13,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "s6",
                    description: Some(
                        "desc S6.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 16,
                        },
                    ),
                    bit_size: 13,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "EccSynd123",
            extends: None,
            description: Some(
                "desc ECC_SYND12_3.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "s7",
                    description: Some(
                        "desc S7.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 13,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "s8",
                    description: Some(
                        "desc S8.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 16,
                        },
                    ),
                    bit_size: 13,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "EccSynd13",
            extends: None,
            description: Some(
                "desc ECC_SYND1_3.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "s7",
                    description: Some(
                        "desc S7.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 13,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "s8",
                    description: Some(
                        "desc S8.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 16,
                        },
                    ),
                    bit_size: 13,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "EccSynd130",
            extends: None,
            description: Some(
                "desc ECC_SYND13_0.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "s1",
                    description: Some(
                        "desc S1.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 13,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "s2",
                    description: Some(
                        "desc S2.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 16,
                        },
                    ),
                    bit_size: 13,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "EccSynd131",
            extends: None,
            description: Some(
                "desc ECC_SYND13_1.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "s3",
                    description: Some(
                        "desc S3.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 13,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "s4",
                    description: Some(
                        "desc S4.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 16,
                        },
                    ),
                    bit_size: 13,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "EccSynd132",
            extends: None,
            description: Some(
                "desc ECC_SYND13_2.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "s5",
                    description: Some(
                        "desc S5.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 13,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "s6",
                    description: Some(
                        "desc S6.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 16,
                        },
                    ),
                    bit_size: 13,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "EccSynd133",
            extends: None,
            description: Some(
                "desc ECC_SYND13_3.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "s7",
                    description: Some(
                        "desc S7.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 13,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "s8",
                    description: Some(
                        "desc S8.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 16,
                        },
                    ),
                    bit_size: 13,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "EccSynd140",
            extends: None,
            description: Some(
                "desc ECC_SYND14_0.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "s1",
                    description: Some(
                        "desc S1.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 13,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "s2",
                    description: Some(
                        "desc S2.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 16,
                        },
                    ),
                    bit_size: 13,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "EccSynd141",
            extends: None,
            description: Some(
                "desc ECC_SYND14_1.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "s3",
                    description: Some(
                        "desc S3.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 13,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "s4",
                    description: Some(
                        "desc S4.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 16,
                        },
                    ),
                    bit_size: 13,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "EccSynd142",
            extends: None,
            description: Some(
                "desc ECC_SYND14_2.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "s5",
                    description: Some(
                        "desc S5.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 13,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "s6",
                    description: Some(
                        "desc S6.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 16,
                        },
                    ),
                    bit_size: 13,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "EccSynd143",
            extends: None,
            description: Some(
                "desc ECC_SYND14_3.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "s7",
                    description: Some(
                        "desc S7.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 13,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "s8",
                    description: Some(
                        "desc S8.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 16,
                        },
                    ),
                    bit_size: 13,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "EccSynd150",
            extends: None,
            description: Some(
                "desc ECC_SYND15_0.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "s1",
                    description: Some(
                        "desc S1.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 13,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "s2",
                    description: Some(
                        "desc S2.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 16,
                        },
                    ),
                    bit_size: 13,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "EccSynd151",
            extends: None,
            description: Some(
                "desc ECC_SYND15_1.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "s3",
                    description: Some(
                        "desc S3.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 13,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "s4",
                    description: Some(
                        "desc S4.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 16,
                        },
                    ),
                    bit_size: 13,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "EccSynd152",
            extends: None,
            description: Some(
                "desc ECC_SYND15_2.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "s5",
                    description: Some(
                        "desc S5.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 13,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "s6",
                    description: Some(
                        "desc S6.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 16,
                        },
                    ),
                    bit_size: 13,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "EccSynd153",
            extends: None,
            description: Some(
                "desc ECC_SYND15_3.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "s7",
                    description: Some(
                        "desc S7.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 13,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "s8",
                    description: Some(
                        "desc S8.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 16,
                        },
                    ),
                    bit_size: 13,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "EccSynd20",
            extends: None,
            description: Some(
                "desc ECC_SYND2_0.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "s1",
                    description: Some(
                        "desc S1.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 13,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "s2",
                    description: Some(
                        "desc S2.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 16,
                        },
                    ),
                    bit_size: 13,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "EccSynd21",
            extends: None,
            description: Some(
                "desc ECC_SYND2_1.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "s3",
                    description: Some(
                        "desc S3.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 13,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "s4",
                    description: Some(
                        "desc S4.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 16,
                        },
                    ),
                    bit_size: 13,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "EccSynd22",
            extends: None,
            description: Some(
                "desc ECC_SYND2_2.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "s5",
                    description: Some(
                        "desc S5.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 13,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "s6",
                    description: Some(
                        "desc S6.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 16,
                        },
                    ),
                    bit_size: 13,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "EccSynd23",
            extends: None,
            description: Some(
                "desc ECC_SYND2_3.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "s7",
                    description: Some(
                        "desc S7.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 13,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "s8",
                    description: Some(
                        "desc S8.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 16,
                        },
                    ),
                    bit_size: 13,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "EccSynd30",
            extends: None,
            description: Some(
                "desc ECC_SYND3_0.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "s1",
                    description: Some(
                        "desc S1.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 13,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "s2",
                    description: Some(
                        "desc S2.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 16,
                        },
                    ),
                    bit_size: 13,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "EccSynd31",
            extends: None,
            description: Some(
                "desc ECC_SYND3_1.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "s3",
                    description: Some(
                        "desc S3.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 13,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "s4",
                    description: Some(
                        "desc S4.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 16,
                        },
                    ),
                    bit_size: 13,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "EccSynd32",
            extends: None,
            description: Some(
                "desc ECC_SYND3_2.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "s5",
                    description: Some(
                        "desc S5.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 13,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "s6",
                    description: Some(
                        "desc S6.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 16,
                        },
                    ),
                    bit_size: 13,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "EccSynd33",
            extends: None,
            description: Some(
                "desc ECC_SYND3_3.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "s7",
                    description: Some(
                        "desc S7.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 13,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "s8",
                    description: Some(
                        "desc S8.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 16,
                        },
                    ),
                    bit_size: 13,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "EccSynd40",
            extends: None,
            description: Some(
                "desc ECC_SYND4_0.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "s1",
                    description: Some(
                        "desc S1.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 13,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "s2",
                    description: Some(
                        "desc S2.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 16,
                        },
                    ),
                    bit_size: 13,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "EccSynd41",
            extends: None,
            description: Some(
                "desc ECC_SYND4_1.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "s3",
                    description: Some(
                        "desc S3.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 13,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "s4",
                    description: Some(
                        "desc S4.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 16,
                        },
                    ),
                    bit_size: 13,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "EccSynd42",
            extends: None,
            description: Some(
                "desc ECC_SYND4_2.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "s5",
                    description: Some(
                        "desc S5.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 13,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "s6",
                    description: Some(
                        "desc S6.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 16,
                        },
                    ),
                    bit_size: 13,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "EccSynd43",
            extends: None,
            description: Some(
                "desc ECC_SYND4_3.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "s7",
                    description: Some(
                        "desc S7.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 13,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "s8",
                    description: Some(
                        "desc S8.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 16,
                        },
                    ),
                    bit_size: 13,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "EccSynd50",
            extends: None,
            description: Some(
                "desc ECC_SYND5_0.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "s1",
                    description: Some(
                        "desc S1.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 13,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "s2",
                    description: Some(
                        "desc S2.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 16,
                        },
                    ),
                    bit_size: 13,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "EccSynd51",
            extends: None,
            description: Some(
                "desc ECC_SYND5_1.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "s3",
                    description: Some(
                        "desc S3.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 13,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "s4",
                    description: Some(
                        "desc S4.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 16,
                        },
                    ),
                    bit_size: 13,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "EccSynd52",
            extends: None,
            description: Some(
                "desc ECC_SYND5_2.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "s5",
                    description: Some(
                        "desc S5.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 13,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "s6",
                    description: Some(
                        "desc S6.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 16,
                        },
                    ),
                    bit_size: 13,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "EccSynd53",
            extends: None,
            description: Some(
                "desc ECC_SYND5_3.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "s7",
                    description: Some(
                        "desc S7.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 13,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "s8",
                    description: Some(
                        "desc S8.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 16,
                        },
                    ),
                    bit_size: 13,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "EccSynd60",
            extends: None,
            description: Some(
                "desc ECC_SYND6_0.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "s1",
                    description: Some(
                        "desc S1.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 13,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "s2",
                    description: Some(
                        "desc S2.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 16,
                        },
                    ),
                    bit_size: 13,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "EccSynd61",
            extends: None,
            description: Some(
                "desc ECC_SYND6_1.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "s3",
                    description: Some(
                        "desc S3.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 13,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "s4",
                    description: Some(
                        "desc S4.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 16,
                        },
                    ),
                    bit_size: 13,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "EccSynd62",
            extends: None,
            description: Some(
                "desc ECC_SYND6_2.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "s5",
                    description: Some(
                        "desc S5.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 13,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "s6",
                    description: Some(
                        "desc S6.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 16,
                        },
                    ),
                    bit_size: 13,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "EccSynd63",
            extends: None,
            description: Some(
                "desc ECC_SYND6_3.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "s7",
                    description: Some(
                        "desc S7.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 13,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "s8",
                    description: Some(
                        "desc S8.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 16,
                        },
                    ),
                    bit_size: 13,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "EccSynd70",
            extends: None,
            description: Some(
                "desc ECC_SYND7_0.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "s1",
                    description: Some(
                        "desc S1.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 13,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "s2",
                    description: Some(
                        "desc S2.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 16,
                        },
                    ),
                    bit_size: 13,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "EccSynd71",
            extends: None,
            description: Some(
                "desc ECC_SYND7_1.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "s3",
                    description: Some(
                        "desc S3.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 13,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "s4",
                    description: Some(
                        "desc S4.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 16,
                        },
                    ),
                    bit_size: 13,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "EccSynd72",
            extends: None,
            description: Some(
                "desc ECC_SYND7_2.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "s5",
                    description: Some(
                        "desc S5.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 13,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "s6",
                    description: Some(
                        "desc S6.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 16,
                        },
                    ),
                    bit_size: 13,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "EccSynd73",
            extends: None,
            description: Some(
                "desc ECC_SYND7_3.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "s7",
                    description: Some(
                        "desc S7.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 13,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "s8",
                    description: Some(
                        "desc S8.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 16,
                        },
                    ),
                    bit_size: 13,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "EccSynd80",
            extends: None,
            description: Some(
                "desc ECC_SYND8_0.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "s1",
                    description: Some(
                        "desc S1.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 13,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "s2",
                    description: Some(
                        "desc S2.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 16,
                        },
                    ),
                    bit_size: 13,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "EccSynd81",
            extends: None,
            description: Some(
                "desc ECC_SYND8_1.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "s3",
                    description: Some(
                        "desc S3.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 13,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "s4",
                    description: Some(
                        "desc S4.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 16,
                        },
                    ),
                    bit_size: 13,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "EccSynd82",
            extends: None,
            description: Some(
                "desc ECC_SYND8_2.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "s5",
                    description: Some(
                        "desc S5.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 13,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "s6",
                    description: Some(
                        "desc S6.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 16,
                        },
                    ),
                    bit_size: 13,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "EccSynd83",
            extends: None,
            description: Some(
                "desc ECC_SYND8_3.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "s7",
                    description: Some(
                        "desc S7.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 13,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "s8",
                    description: Some(
                        "desc S8.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 16,
                        },
                    ),
                    bit_size: 13,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "EccSynd90",
            extends: None,
            description: Some(
                "desc ECC_SYND9_0.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "s1",
                    description: Some(
                        "desc S1.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 13,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "s2",
                    description: Some(
                        "desc S2.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 16,
                        },
                    ),
                    bit_size: 13,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "EccSynd91",
            extends: None,
            description: Some(
                "desc ECC_SYND9_1.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "s3",
                    description: Some(
                        "desc S3.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 13,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "s4",
                    description: Some(
                        "desc S4.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 16,
                        },
                    ),
                    bit_size: 13,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "EccSynd92",
            extends: None,
            description: Some(
                "desc ECC_SYND9_2.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "s5",
                    description: Some(
                        "desc S5.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 13,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "s6",
                    description: Some(
                        "desc S6.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 16,
                        },
                    ),
                    bit_size: 13,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "EccSynd93",
            extends: None,
            description: Some(
                "desc ECC_SYND9_3.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "s7",
                    description: Some(
                        "desc S7.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 13,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "s8",
                    description: Some(
                        "desc S8.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 16,
                        },
                    ),
                    bit_size: 13,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Eccr0",
            extends: None,
            description: Some(
                "desc ECCR0.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "errloc",
                    description: Some(
                        "desc ERRLOC.",
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
                    name: "se",
                    description: Some(
                        "desc SE.",
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
                    name: "me",
                    description: Some(
                        "desc ME.",
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
            ],
        },
        FieldSet {
            name: "Eccr1",
            extends: None,
            description: Some(
                "desc ECCR1.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "errloc",
                    description: Some(
                        "desc ERRLOC.",
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
                    name: "se",
                    description: Some(
                        "desc SE.",
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
                    name: "me",
                    description: Some(
                        "desc ME.",
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
            ],
        },
        FieldSet {
            name: "Eccr10",
            extends: None,
            description: Some(
                "desc ECCR10.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "errloc",
                    description: Some(
                        "desc ERRLOC.",
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
                    name: "se",
                    description: Some(
                        "desc SE.",
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
                    name: "me",
                    description: Some(
                        "desc ME.",
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
            ],
        },
        FieldSet {
            name: "Eccr11",
            extends: None,
            description: Some(
                "desc ECCR11.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "errloc",
                    description: Some(
                        "desc ERRLOC.",
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
                    name: "se",
                    description: Some(
                        "desc SE.",
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
                    name: "me",
                    description: Some(
                        "desc ME.",
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
            ],
        },
        FieldSet {
            name: "Eccr12",
            extends: None,
            description: Some(
                "desc ECCR12.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "errloc",
                    description: Some(
                        "desc ERRLOC.",
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
                    name: "se",
                    description: Some(
                        "desc SE.",
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
                    name: "me",
                    description: Some(
                        "desc ME.",
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
            ],
        },
        FieldSet {
            name: "Eccr13",
            extends: None,
            description: Some(
                "desc ECCR13.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "errloc",
                    description: Some(
                        "desc ERRLOC.",
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
                    name: "se",
                    description: Some(
                        "desc SE.",
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
                    name: "me",
                    description: Some(
                        "desc ME.",
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
            ],
        },
        FieldSet {
            name: "Eccr14",
            extends: None,
            description: Some(
                "desc ECCR14.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "errloc",
                    description: Some(
                        "desc ERRLOC.",
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
                    name: "se",
                    description: Some(
                        "desc SE.",
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
                    name: "me",
                    description: Some(
                        "desc ME.",
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
            ],
        },
        FieldSet {
            name: "Eccr15",
            extends: None,
            description: Some(
                "desc ECCR15.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "errloc",
                    description: Some(
                        "desc ERRLOC.",
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
                    name: "se",
                    description: Some(
                        "desc SE.",
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
                    name: "me",
                    description: Some(
                        "desc ME.",
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
            ],
        },
        FieldSet {
            name: "Eccr2",
            extends: None,
            description: Some(
                "desc ECCR2.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "errloc",
                    description: Some(
                        "desc ERRLOC.",
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
                    name: "se",
                    description: Some(
                        "desc SE.",
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
                    name: "me",
                    description: Some(
                        "desc ME.",
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
            ],
        },
        FieldSet {
            name: "Eccr3",
            extends: None,
            description: Some(
                "desc ECCR3.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "errloc",
                    description: Some(
                        "desc ERRLOC.",
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
                    name: "se",
                    description: Some(
                        "desc SE.",
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
                    name: "me",
                    description: Some(
                        "desc ME.",
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
            ],
        },
        FieldSet {
            name: "Eccr4",
            extends: None,
            description: Some(
                "desc ECCR4.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "errloc",
                    description: Some(
                        "desc ERRLOC.",
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
                    name: "se",
                    description: Some(
                        "desc SE.",
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
                    name: "me",
                    description: Some(
                        "desc ME.",
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
            ],
        },
        FieldSet {
            name: "Eccr5",
            extends: None,
            description: Some(
                "desc ECCR5.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "errloc",
                    description: Some(
                        "desc ERRLOC.",
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
                    name: "se",
                    description: Some(
                        "desc SE.",
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
                    name: "me",
                    description: Some(
                        "desc ME.",
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
            ],
        },
        FieldSet {
            name: "Eccr6",
            extends: None,
            description: Some(
                "desc ECCR6.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "errloc",
                    description: Some(
                        "desc ERRLOC.",
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
                    name: "se",
                    description: Some(
                        "desc SE.",
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
                    name: "me",
                    description: Some(
                        "desc ME.",
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
            ],
        },
        FieldSet {
            name: "Eccr7",
            extends: None,
            description: Some(
                "desc ECCR7.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "errloc",
                    description: Some(
                        "desc ERRLOC.",
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
                    name: "se",
                    description: Some(
                        "desc SE.",
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
                    name: "me",
                    description: Some(
                        "desc ME.",
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
            ],
        },
        FieldSet {
            name: "Eccr8",
            extends: None,
            description: Some(
                "desc ECCR8.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "errloc",
                    description: Some(
                        "desc ERRLOC.",
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
                    name: "se",
                    description: Some(
                        "desc SE.",
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
                    name: "me",
                    description: Some(
                        "desc ME.",
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
            ],
        },
        FieldSet {
            name: "Eccr9",
            extends: None,
            description: Some(
                "desc ECCR9.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "errloc",
                    description: Some(
                        "desc ERRLOC.",
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
                    name: "se",
                    description: Some(
                        "desc SE.",
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
                    name: "me",
                    description: Some(
                        "desc ME.",
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
            ],
        },
        FieldSet {
            name: "Idxr1",
            extends: None,
            description: Some(
                "desc IDXR1.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "idx1",
                    description: Some(
                        "desc IDX1.",
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
            name: "Ienr",
            extends: None,
            description: Some(
                "desc IENR.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "ecceuen",
                    description: Some(
                        "desc ECCEUEN.",
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
                    name: "eccecen",
                    description: Some(
                        "desc ECCECEN.",
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
                    name: "ecccen",
                    description: Some(
                        "desc ECCCEN.",
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
                    name: "ecceen",
                    description: Some(
                        "desc ECCEEN.",
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
                    name: "eccdis",
                    description: Some(
                        "desc ECCDIS.",
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
                    name: "rben",
                    description: Some(
                        "desc RBEN.",
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
            ],
        },
        FieldSet {
            name: "Irsr",
            extends: None,
            description: Some(
                "desc IRSR.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "ecceurs",
                    description: Some(
                        "desc ECCEURS.",
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
                    name: "eccecrs",
                    description: Some(
                        "desc ECCECRS.",
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
                    name: "ecccrs",
                    description: Some(
                        "desc ECCCRS.",
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
                    name: "eccers",
                    description: Some(
                        "desc ECCERS.",
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
                    name: "rbrs",
                    description: Some(
                        "desc RBRS.",
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
            ],
        },
        FieldSet {
            name: "Istr",
            extends: None,
            description: Some(
                "desc ISTR.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "ecceust",
                    description: Some(
                        "desc ECCEUST.",
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
                    name: "eccecst",
                    description: Some(
                        "desc ECCECST.",
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
                    name: "ecccst",
                    description: Some(
                        "desc ECCCST.",
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
                    name: "eccest",
                    description: Some(
                        "desc ECCEST.",
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
                    name: "rbst",
                    description: Some(
                        "desc RBST.",
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
            ],
        },
        FieldSet {
            name: "Tmcr0",
            extends: None,
            description: Some(
                "desc TMCR0.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "ts",
                    description: Some(
                        "desc TS.",
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
                    name: "twp",
                    description: Some(
                        "desc TWP.",
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
                    name: "trp",
                    description: Some(
                        "desc TRP.",
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
                    name: "th",
                    description: Some(
                        "desc TH.",
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
            name: "Tmcr1",
            extends: None,
            description: Some(
                "desc TMCR1.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "twh",
                    description: Some(
                        "desc TWH.",
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
                    name: "trh",
                    description: Some(
                        "desc TRH.",
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
                    name: "trr",
                    description: Some(
                        "desc TRR.",
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
                    name: "twb",
                    description: Some(
                        "desc TWB.",
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
            name: "Tmcr2",
            extends: None,
            description: Some(
                "desc TMCR2.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "tccs",
                    description: Some(
                        "desc TCCS.",
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
                    name: "twtr",
                    description: Some(
                        "desc TWTR.",
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
                    name: "trtw",
                    description: Some(
                        "desc TRTW.",
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
                    name: "tadl",
                    description: Some(
                        "desc TADL.",
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
    ],
    enums: &[],
};
                