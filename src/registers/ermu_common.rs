
                    use crate::metadata::ir::*;
                    pub(crate) static REGISTERS: IR = IR {
    blocks: &[
        Block {
            name: "Ermu",
            extends: None,
            description: Some(
                "desc ERMU.",
            ),
            items: &[
                BlockItem {
                    name: "eo0c",
                    description: Some(
                        "desc EO0C.",
                    ),
                    array: None,
                    byte_offset: 0x0,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Eo0c",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "eo0s",
                    description: Some(
                        "desc EO0S.",
                    ),
                    array: None,
                    byte_offset: 0x4,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::Read,
                            bit_size: 32,
                            fieldset: Some(
                                "Eo0s",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "eo0ctcnt",
                    description: Some(
                        "desc EO0CTCNT.",
                    ),
                    array: None,
                    byte_offset: 0x8,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::Read,
                            bit_size: 32,
                            fieldset: Some(
                                "Eo0ctcnt",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "eo0ctcmp",
                    description: Some(
                        "desc EO0CTCMP.",
                    ),
                    array: None,
                    byte_offset: 0xc,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Eo0ctcmp",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "eo0ttc",
                    description: Some(
                        "desc EO0TTC.",
                    ),
                    array: None,
                    byte_offset: 0x10,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Eo0ttc",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "eo0ttcmp",
                    description: Some(
                        "desc EO0TTCMP.",
                    ),
                    array: None,
                    byte_offset: 0x18,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Eo0ttcmp",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "eo0om0",
                    description: Some(
                        "desc EO0OM0.",
                    ),
                    array: None,
                    byte_offset: 0x40,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Eo0om0",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "eo0om1",
                    description: Some(
                        "desc EO0OM1.",
                    ),
                    array: None,
                    byte_offset: 0x44,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Eo0om1",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "eo1c",
                    description: Some(
                        "desc EO1C.",
                    ),
                    array: None,
                    byte_offset: 0x80,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Eo1c",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "eo1s",
                    description: Some(
                        "desc EO1S.",
                    ),
                    array: None,
                    byte_offset: 0x84,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::Read,
                            bit_size: 32,
                            fieldset: Some(
                                "Eo1s",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "eo1ctcnt",
                    description: Some(
                        "desc EO1CTCNT.",
                    ),
                    array: None,
                    byte_offset: 0x88,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::Read,
                            bit_size: 32,
                            fieldset: Some(
                                "Eo1ctcnt",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "eo1ctcmp",
                    description: Some(
                        "desc EO1CTCMP.",
                    ),
                    array: None,
                    byte_offset: 0x8c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Eo1ctcmp",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "eo1ttc",
                    description: Some(
                        "desc EO1TTC.",
                    ),
                    array: None,
                    byte_offset: 0x90,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Eo1ttc",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "eo1ttcmp",
                    description: Some(
                        "desc EO1TTCMP.",
                    ),
                    array: None,
                    byte_offset: 0x98,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Eo1ttcmp",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "eo1om0",
                    description: Some(
                        "desc EO1OM0.",
                    ),
                    array: None,
                    byte_offset: 0xc0,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Eo1om0",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "eo1om1",
                    description: Some(
                        "desc EO1OM1.",
                    ),
                    array: None,
                    byte_offset: 0xc4,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Eo1om1",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "eo2c",
                    description: Some(
                        "desc EO2C.",
                    ),
                    array: None,
                    byte_offset: 0x100,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Eo2c",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "eo2s",
                    description: Some(
                        "desc EO2S.",
                    ),
                    array: None,
                    byte_offset: 0x104,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::Read,
                            bit_size: 32,
                            fieldset: Some(
                                "Eo2s",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "eo2ctcnt",
                    description: Some(
                        "desc EO2CTCNT.",
                    ),
                    array: None,
                    byte_offset: 0x108,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::Read,
                            bit_size: 32,
                            fieldset: Some(
                                "Eo2ctcnt",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "eo2ctcmp",
                    description: Some(
                        "desc EO2CTCMP.",
                    ),
                    array: None,
                    byte_offset: 0x10c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Eo2ctcmp",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "eo2ttc",
                    description: Some(
                        "desc EO2TTC.",
                    ),
                    array: None,
                    byte_offset: 0x110,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Eo2ttc",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "eo2ttcmp",
                    description: Some(
                        "desc EO2TTCMP.",
                    ),
                    array: None,
                    byte_offset: 0x118,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Eo2ttcmp",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "eo2om0",
                    description: Some(
                        "desc EO2OM0.",
                    ),
                    array: None,
                    byte_offset: 0x140,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Eo2om0",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "eo2om1",
                    description: Some(
                        "desc EO2OM1.",
                    ),
                    array: None,
                    byte_offset: 0x144,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Eo2om1",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "eo3c",
                    description: Some(
                        "desc EO3C.",
                    ),
                    array: None,
                    byte_offset: 0x180,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Eo3c",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "eo3s",
                    description: Some(
                        "desc EO3S.",
                    ),
                    array: None,
                    byte_offset: 0x184,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::Read,
                            bit_size: 32,
                            fieldset: Some(
                                "Eo3s",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "eo3ctcnt",
                    description: Some(
                        "desc EO3CTCNT.",
                    ),
                    array: None,
                    byte_offset: 0x188,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::Read,
                            bit_size: 32,
                            fieldset: Some(
                                "Eo3ctcnt",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "eo3ctcmp",
                    description: Some(
                        "desc EO3CTCMP.",
                    ),
                    array: None,
                    byte_offset: 0x18c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Eo3ctcmp",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "eo3ttc",
                    description: Some(
                        "desc EO3TTC.",
                    ),
                    array: None,
                    byte_offset: 0x190,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Eo3ttc",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "eo3ttcmp",
                    description: Some(
                        "desc EO3TTCMP.",
                    ),
                    array: None,
                    byte_offset: 0x198,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Eo3ttcmp",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "eo3om0",
                    description: Some(
                        "desc EO3OM0.",
                    ),
                    array: None,
                    byte_offset: 0x1c0,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Eo3om0",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "eo3om1",
                    description: Some(
                        "desc EO3OM1.",
                    ),
                    array: None,
                    byte_offset: 0x1c4,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Eo3om1",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "wt0c",
                    description: Some(
                        "desc WT0C.",
                    ),
                    array: None,
                    byte_offset: 0x200,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Wt0c",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "wt0s",
                    description: Some(
                        "desc WT0S.",
                    ),
                    array: None,
                    byte_offset: 0x204,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::Read,
                            bit_size: 32,
                            fieldset: Some(
                                "Wt0s",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "wt0cnt",
                    description: Some(
                        "desc WT0CNT.",
                    ),
                    array: None,
                    byte_offset: 0x208,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::Read,
                            bit_size: 32,
                            fieldset: Some(
                                "Wt0cnt",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "wt0cmp",
                    description: Some(
                        "desc WT0CMP.",
                    ),
                    array: None,
                    byte_offset: 0x20c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Wt0cmp",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "wt0se",
                    description: Some(
                        "desc WT0SE.",
                    ),
                    array: None,
                    byte_offset: 0x210,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Wt0se",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "wt1c",
                    description: Some(
                        "desc WT1C.",
                    ),
                    array: None,
                    byte_offset: 0x220,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Wt1c",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "wt1s",
                    description: Some(
                        "desc WT1S.",
                    ),
                    array: None,
                    byte_offset: 0x224,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::Read,
                            bit_size: 32,
                            fieldset: Some(
                                "Wt1s",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "wt1cnt",
                    description: Some(
                        "desc WT1CNT.",
                    ),
                    array: None,
                    byte_offset: 0x228,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::Read,
                            bit_size: 32,
                            fieldset: Some(
                                "Wt1cnt",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "wt1cmp",
                    description: Some(
                        "desc WT1CMP.",
                    ),
                    array: None,
                    byte_offset: 0x22c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Wt1cmp",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "wt1se",
                    description: Some(
                        "desc WT1SE.",
                    ),
                    array: None,
                    byte_offset: 0x230,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Wt1se",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "wt2c",
                    description: Some(
                        "desc WT2C.",
                    ),
                    array: None,
                    byte_offset: 0x240,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Wt2c",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "wt2s",
                    description: Some(
                        "desc WT2S.",
                    ),
                    array: None,
                    byte_offset: 0x244,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::Read,
                            bit_size: 32,
                            fieldset: Some(
                                "Wt2s",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "wt2cnt",
                    description: Some(
                        "desc WT2CNT.",
                    ),
                    array: None,
                    byte_offset: 0x248,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::Read,
                            bit_size: 32,
                            fieldset: Some(
                                "Wt2cnt",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "wt2cmp",
                    description: Some(
                        "desc WT2CMP.",
                    ),
                    array: None,
                    byte_offset: 0x24c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Wt2cmp",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "wt2se",
                    description: Some(
                        "desc WT2SE.",
                    ),
                    array: None,
                    byte_offset: 0x250,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Wt2se",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "wt3c",
                    description: Some(
                        "desc WT3C.",
                    ),
                    array: None,
                    byte_offset: 0x260,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Wt3c",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "wt3s",
                    description: Some(
                        "desc WT3S.",
                    ),
                    array: None,
                    byte_offset: 0x264,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::Read,
                            bit_size: 32,
                            fieldset: Some(
                                "Wt3s",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "wt3cnt",
                    description: Some(
                        "desc WT3CNT.",
                    ),
                    array: None,
                    byte_offset: 0x268,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::Read,
                            bit_size: 32,
                            fieldset: Some(
                                "Wt3cnt",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "wt3cmp",
                    description: Some(
                        "desc WT3CMP.",
                    ),
                    array: None,
                    byte_offset: 0x26c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Wt3cmp",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "wt3se",
                    description: Some(
                        "desc WT3SE.",
                    ),
                    array: None,
                    byte_offset: 0x270,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Wt3se",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "ess0",
                    description: Some(
                        "desc ESS0.",
                    ),
                    array: None,
                    byte_offset: 0x300,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::Read,
                            bit_size: 32,
                            fieldset: Some(
                                "Ess0",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "ess1",
                    description: Some(
                        "desc ESS1.",
                    ),
                    array: None,
                    byte_offset: 0x304,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::Read,
                            bit_size: 32,
                            fieldset: Some(
                                "Ess1",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "essc0",
                    description: Some(
                        "desc ESSC0.",
                    ),
                    array: None,
                    byte_offset: 0x340,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::Write,
                            bit_size: 32,
                            fieldset: Some(
                                "Essc0",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "essc1",
                    description: Some(
                        "desc ESSC1.",
                    ),
                    array: None,
                    byte_offset: 0x344,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::Write,
                            bit_size: 32,
                            fieldset: Some(
                                "Essc1",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "pet0",
                    description: Some(
                        "desc PET0.",
                    ),
                    array: None,
                    byte_offset: 0x380,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::Write,
                            bit_size: 32,
                            fieldset: Some(
                                "Pet0",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "pet1",
                    description: Some(
                        "desc PET1.",
                    ),
                    array: None,
                    byte_offset: 0x384,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::Write,
                            bit_size: 32,
                            fieldset: Some(
                                "Pet1",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "re0",
                    description: Some(
                        "desc RE0.",
                    ),
                    array: None,
                    byte_offset: 0x3c0,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Re0",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "re1",
                    description: Some(
                        "desc RE1.",
                    ),
                    array: None,
                    byte_offset: 0x3c4,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Re1",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "lpie0",
                    description: Some(
                        "desc LPIE0.",
                    ),
                    array: None,
                    byte_offset: 0x400,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Lpie0",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "lpie1",
                    description: Some(
                        "desc LPIE1.",
                    ),
                    array: None,
                    byte_offset: 0x404,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Lpie1",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "hpie0",
                    description: Some(
                        "desc HPIE0.",
                    ),
                    array: None,
                    byte_offset: 0x500,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Hpie0",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "hpie1",
                    description: Some(
                        "desc HPIE1.",
                    ),
                    array: None,
                    byte_offset: 0x504,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Hpie1",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "ccps",
                    description: Some(
                        "desc CCPS.",
                    ),
                    array: None,
                    byte_offset: 0x600,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Ccps",
                            ),
                        },
                    ),
                },
            ],
        },
    ],
    fieldsets: &[
        FieldSet {
            name: "Ccps",
            extends: None,
            description: Some(
                "desc CCPS.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "pse",
                    description: Some(
                        "desc PSE.",
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
                    name: "pss",
                    description: Some(
                        "desc PSS.",
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
            name: "Eo0c",
            extends: None,
            description: Some(
                "desc EO0C.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "set",
                    description: Some(
                        "desc SET.",
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
                    name: "clr",
                    description: Some(
                        "desc CLR.",
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
                    name: "cte",
                    description: Some(
                        "desc CTE.",
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
            name: "Eo0ctcmp",
            extends: None,
            description: Some(
                "desc EO0CTCMP.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "cmp",
                    description: Some(
                        "desc CMP.",
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
            name: "Eo0ctcnt",
            extends: None,
            description: Some(
                "desc EO0CTCNT.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "cnt",
                    description: Some(
                        "desc CNT.",
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
            name: "Eo0om0",
            extends: None,
            description: Some(
                "desc EO0OM0.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "om",
                    description: Some(
                        "desc OM.",
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
            name: "Eo0om1",
            extends: None,
            description: Some(
                "desc EO0OM1.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "om",
                    description: Some(
                        "desc OM.",
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
            name: "Eo0s",
            extends: None,
            description: Some(
                "desc EO0S.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "eos",
                    description: Some(
                        "desc EOS.",
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
                    name: "cts",
                    description: Some(
                        "desc CTS.",
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
            name: "Eo0ttc",
            extends: None,
            description: Some(
                "desc EO0TTC.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "tte",
                    description: Some(
                        "desc TTE.",
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
            name: "Eo0ttcmp",
            extends: None,
            description: Some(
                "desc EO0TTCMP.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "cmp",
                    description: Some(
                        "desc CMP.",
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
            name: "Eo1c",
            extends: None,
            description: Some(
                "desc EO1C.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "set",
                    description: Some(
                        "desc SET.",
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
                    name: "clr",
                    description: Some(
                        "desc CLR.",
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
                    name: "cte",
                    description: Some(
                        "desc CTE.",
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
            name: "Eo1ctcmp",
            extends: None,
            description: Some(
                "desc EO1CTCMP.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "cmp",
                    description: Some(
                        "desc CMP.",
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
            name: "Eo1ctcnt",
            extends: None,
            description: Some(
                "desc EO1CTCNT.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "cnt",
                    description: Some(
                        "desc CNT.",
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
            name: "Eo1om0",
            extends: None,
            description: Some(
                "desc EO1OM0.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "om",
                    description: Some(
                        "desc OM.",
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
            name: "Eo1om1",
            extends: None,
            description: Some(
                "desc EO1OM1.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "om",
                    description: Some(
                        "desc OM.",
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
            name: "Eo1s",
            extends: None,
            description: Some(
                "desc EO1S.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "eos",
                    description: Some(
                        "desc EOS.",
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
                    name: "cts",
                    description: Some(
                        "desc CTS.",
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
            name: "Eo1ttc",
            extends: None,
            description: Some(
                "desc EO1TTC.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "tte",
                    description: Some(
                        "desc TTE.",
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
            name: "Eo1ttcmp",
            extends: None,
            description: Some(
                "desc EO1TTCMP.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "cmp",
                    description: Some(
                        "desc CMP.",
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
            name: "Eo2c",
            extends: None,
            description: Some(
                "desc EO2C.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "set",
                    description: Some(
                        "desc SET.",
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
                    name: "clr",
                    description: Some(
                        "desc CLR.",
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
                    name: "cte",
                    description: Some(
                        "desc CTE.",
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
            name: "Eo2ctcmp",
            extends: None,
            description: Some(
                "desc EO2CTCMP.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "cmp",
                    description: Some(
                        "desc CMP.",
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
            name: "Eo2ctcnt",
            extends: None,
            description: Some(
                "desc EO2CTCNT.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "cnt",
                    description: Some(
                        "desc CNT.",
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
            name: "Eo2om0",
            extends: None,
            description: Some(
                "desc EO2OM0.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "om",
                    description: Some(
                        "desc OM.",
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
            name: "Eo2om1",
            extends: None,
            description: Some(
                "desc EO2OM1.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "om",
                    description: Some(
                        "desc OM.",
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
            name: "Eo2s",
            extends: None,
            description: Some(
                "desc EO2S.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "eos",
                    description: Some(
                        "desc EOS.",
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
                    name: "cts",
                    description: Some(
                        "desc CTS.",
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
            name: "Eo2ttc",
            extends: None,
            description: Some(
                "desc EO2TTC.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "tte",
                    description: Some(
                        "desc TTE.",
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
            name: "Eo2ttcmp",
            extends: None,
            description: Some(
                "desc EO2TTCMP.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "cmp",
                    description: Some(
                        "desc CMP.",
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
            name: "Eo3c",
            extends: None,
            description: Some(
                "desc EO3C.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "set",
                    description: Some(
                        "desc SET.",
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
                    name: "clr",
                    description: Some(
                        "desc CLR.",
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
                    name: "cte",
                    description: Some(
                        "desc CTE.",
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
            name: "Eo3ctcmp",
            extends: None,
            description: Some(
                "desc EO3CTCMP.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "cmp",
                    description: Some(
                        "desc CMP.",
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
            name: "Eo3ctcnt",
            extends: None,
            description: Some(
                "desc EO3CTCNT.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "cnt",
                    description: Some(
                        "desc CNT.",
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
            name: "Eo3om0",
            extends: None,
            description: Some(
                "desc EO3OM0.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "om",
                    description: Some(
                        "desc OM.",
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
            name: "Eo3om1",
            extends: None,
            description: Some(
                "desc EO3OM1.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "om",
                    description: Some(
                        "desc OM.",
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
            name: "Eo3s",
            extends: None,
            description: Some(
                "desc EO3S.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "eos",
                    description: Some(
                        "desc EOS.",
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
                    name: "cts",
                    description: Some(
                        "desc CTS.",
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
            name: "Eo3ttc",
            extends: None,
            description: Some(
                "desc EO3TTC.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "tte",
                    description: Some(
                        "desc TTE.",
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
            name: "Eo3ttcmp",
            extends: None,
            description: Some(
                "desc EO3TTCMP.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "cmp",
                    description: Some(
                        "desc CMP.",
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
            name: "Ess0",
            extends: None,
            description: Some(
                "desc ESS0.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "ess",
                    description: Some(
                        "desc ESS.",
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
            name: "Ess1",
            extends: None,
            description: Some(
                "desc ESS1.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "ess",
                    description: Some(
                        "desc ESS.",
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
            name: "Essc0",
            extends: None,
            description: Some(
                "desc ESSC0.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "essc",
                    description: Some(
                        "desc ESSC.",
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
            name: "Essc1",
            extends: None,
            description: Some(
                "desc ESSC1.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "essc",
                    description: Some(
                        "desc ESSC.",
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
            name: "Hpie0",
            extends: None,
            description: Some(
                "desc HPIE0.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "hpie",
                    description: Some(
                        "desc HPIE.",
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
            name: "Hpie1",
            extends: None,
            description: Some(
                "desc HPIE1.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "hpie",
                    description: Some(
                        "desc HPIE.",
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
            name: "Lpie0",
            extends: None,
            description: Some(
                "desc LPIE0.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "lpie",
                    description: Some(
                        "desc LPIE.",
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
            name: "Lpie1",
            extends: None,
            description: Some(
                "desc LPIE1.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "lpie",
                    description: Some(
                        "desc LPIE.",
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
            name: "Pet0",
            extends: None,
            description: Some(
                "desc PET0.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "pet",
                    description: Some(
                        "desc PET.",
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
            name: "Pet1",
            extends: None,
            description: Some(
                "desc PET1.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "pet",
                    description: Some(
                        "desc PET.",
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
            name: "Re0",
            extends: None,
            description: Some(
                "desc RE0.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "re",
                    description: Some(
                        "desc RE.",
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
            name: "Re1",
            extends: None,
            description: Some(
                "desc RE1.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "re",
                    description: Some(
                        "desc RE.",
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
            name: "Wt0c",
            extends: None,
            description: Some(
                "desc WT0C.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "wte",
                    description: Some(
                        "desc WTE.",
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
                    name: "stp",
                    description: Some(
                        "desc STP.",
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
            name: "Wt0cmp",
            extends: None,
            description: Some(
                "desc WT0CMP.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "cmp",
                    description: Some(
                        "desc CMP.",
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
            name: "Wt0cnt",
            extends: None,
            description: Some(
                "desc WT0CNT.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "cnt",
                    description: Some(
                        "desc CNT.",
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
            name: "Wt0s",
            extends: None,
            description: Some(
                "desc WT0S.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "wts",
                    description: Some(
                        "desc WTS.",
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
            name: "Wt0se",
            extends: None,
            description: Some(
                "desc WT0SE.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "lpise",
                    description: Some(
                        "desc LPISE.",
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
                    name: "hpise",
                    description: Some(
                        "desc HPISE.",
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
            name: "Wt1c",
            extends: None,
            description: Some(
                "desc WT1C.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "wte",
                    description: Some(
                        "desc WTE.",
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
                    name: "stp",
                    description: Some(
                        "desc STP.",
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
            name: "Wt1cmp",
            extends: None,
            description: Some(
                "desc WT1CMP.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "cmp",
                    description: Some(
                        "desc CMP.",
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
            name: "Wt1cnt",
            extends: None,
            description: Some(
                "desc WT1CNT.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "cnt",
                    description: Some(
                        "desc CNT.",
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
            name: "Wt1s",
            extends: None,
            description: Some(
                "desc WT1S.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "wts",
                    description: Some(
                        "desc WTS.",
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
            name: "Wt1se",
            extends: None,
            description: Some(
                "desc WT1SE.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "lpise",
                    description: Some(
                        "desc LPISE.",
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
                    name: "hpise",
                    description: Some(
                        "desc HPISE.",
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
            name: "Wt2c",
            extends: None,
            description: Some(
                "desc WT2C.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "wte",
                    description: Some(
                        "desc WTE.",
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
                    name: "stp",
                    description: Some(
                        "desc STP.",
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
            name: "Wt2cmp",
            extends: None,
            description: Some(
                "desc WT2CMP.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "cmp",
                    description: Some(
                        "desc CMP.",
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
            name: "Wt2cnt",
            extends: None,
            description: Some(
                "desc WT2CNT.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "cnt",
                    description: Some(
                        "desc CNT.",
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
            name: "Wt2s",
            extends: None,
            description: Some(
                "desc WT2S.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "wts",
                    description: Some(
                        "desc WTS.",
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
            name: "Wt2se",
            extends: None,
            description: Some(
                "desc WT2SE.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "lpise",
                    description: Some(
                        "desc LPISE.",
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
                    name: "hpise",
                    description: Some(
                        "desc HPISE.",
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
            name: "Wt3c",
            extends: None,
            description: Some(
                "desc WT3C.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "wte",
                    description: Some(
                        "desc WTE.",
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
                    name: "stp",
                    description: Some(
                        "desc STP.",
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
            name: "Wt3cmp",
            extends: None,
            description: Some(
                "desc WT3CMP.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "cmp",
                    description: Some(
                        "desc CMP.",
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
            name: "Wt3cnt",
            extends: None,
            description: Some(
                "desc WT3CNT.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "cnt",
                    description: Some(
                        "desc CNT.",
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
            name: "Wt3s",
            extends: None,
            description: Some(
                "desc WT3S.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "wts",
                    description: Some(
                        "desc WTS.",
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
            name: "Wt3se",
            extends: None,
            description: Some(
                "desc WT3SE.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "lpise",
                    description: Some(
                        "desc LPISE.",
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
                    name: "hpise",
                    description: Some(
                        "desc HPISE.",
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
    ],
    enums: &[],
};
                