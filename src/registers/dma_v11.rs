
                    use crate::metadata::ir::*;
                    pub(crate) static REGISTERS: IR = IR {
    blocks: &[
        Block {
            name: "Dma",
            extends: None,
            description: Some(
                "desc DMA.",
            ),
            items: &[
                BlockItem {
                    name: "en",
                    description: Some(
                        "desc EN.",
                    ),
                    array: None,
                    byte_offset: 0x0,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "En",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "intstat0",
                    description: Some(
                        "desc INTSTAT0.",
                    ),
                    array: None,
                    byte_offset: 0x4,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::Read,
                            bit_size: 32,
                            fieldset: Some(
                                "Intstat0",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "intstat1",
                    description: Some(
                        "desc INTSTAT1.",
                    ),
                    array: None,
                    byte_offset: 0x8,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::Read,
                            bit_size: 32,
                            fieldset: Some(
                                "Intstat1",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "intmask0",
                    description: Some(
                        "desc INTMASK0.",
                    ),
                    array: None,
                    byte_offset: 0xc,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Intmask0",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "intmask1",
                    description: Some(
                        "desc INTMASK1.",
                    ),
                    array: None,
                    byte_offset: 0x10,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Intmask1",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "intclr0",
                    description: Some(
                        "desc INTCLR0.",
                    ),
                    array: None,
                    byte_offset: 0x14,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::Write,
                            bit_size: 32,
                            fieldset: Some(
                                "Intclr0",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "intclr1",
                    description: Some(
                        "desc INTCLR1.",
                    ),
                    array: None,
                    byte_offset: 0x18,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::Write,
                            bit_size: 32,
                            fieldset: Some(
                                "Intclr1",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "chen",
                    description: Some(
                        "desc CHEN.",
                    ),
                    array: None,
                    byte_offset: 0x1c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Chen",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "reqstat",
                    description: Some(
                        "desc REQSTAT.",
                    ),
                    array: None,
                    byte_offset: 0x20,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::Read,
                            bit_size: 32,
                            fieldset: Some(
                                "Reqstat",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "chstat",
                    description: Some(
                        "desc CHSTAT.",
                    ),
                    array: None,
                    byte_offset: 0x24,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::Read,
                            bit_size: 32,
                            fieldset: Some(
                                "Chstat",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "rcfgctl",
                    description: Some(
                        "desc RCFGCTL.",
                    ),
                    array: None,
                    byte_offset: 0x2c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Rcfgctl",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "swreq",
                    description: Some(
                        "desc SWREQ.",
                    ),
                    array: None,
                    byte_offset: 0x30,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::Write,
                            bit_size: 32,
                            fieldset: Some(
                                "Swreq",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "chenclr",
                    description: Some(
                        "desc CHENCLR.",
                    ),
                    array: None,
                    byte_offset: 0x34,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::Write,
                            bit_size: 32,
                            fieldset: Some(
                                "Chenclr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "sar0",
                    description: Some(
                        "desc SAR0.",
                    ),
                    array: None,
                    byte_offset: 0x40,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: None,
                        },
                    ),
                },
                BlockItem {
                    name: "dar0",
                    description: Some(
                        "desc DAR0.",
                    ),
                    array: None,
                    byte_offset: 0x44,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: None,
                        },
                    ),
                },
                BlockItem {
                    name: "dtctl0",
                    description: Some(
                        "desc DTCTL0.",
                    ),
                    array: None,
                    byte_offset: 0x48,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Dtctl0",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "rpt0",
                    description: Some(
                        "desc RPT0.",
                    ),
                    array: None,
                    byte_offset: 0x4c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Rpt0",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "rptb0",
                    description: Some(
                        "desc RPTB0.",
                    ),
                    array: None,
                    byte_offset: 0x4c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Rptb0",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "snseqctl0",
                    description: Some(
                        "desc SNSEQCTL0.",
                    ),
                    array: None,
                    byte_offset: 0x50,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Snseqctl0",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "snseqctlb0",
                    description: Some(
                        "desc SNSEQCTLB0.",
                    ),
                    array: None,
                    byte_offset: 0x50,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Snseqctlb0",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "dnseqctl0",
                    description: Some(
                        "desc DNSEQCTL0.",
                    ),
                    array: None,
                    byte_offset: 0x54,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Dnseqctl0",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "dnseqctlb0",
                    description: Some(
                        "desc DNSEQCTLB0.",
                    ),
                    array: None,
                    byte_offset: 0x54,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Dnseqctlb0",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "llp0",
                    description: Some(
                        "desc LLP0.",
                    ),
                    array: None,
                    byte_offset: 0x58,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Llp0",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "chctl0",
                    description: Some(
                        "desc CHCTL0.",
                    ),
                    array: None,
                    byte_offset: 0x5c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Chctl0",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "monsar0",
                    description: Some(
                        "desc MONSAR0.",
                    ),
                    array: None,
                    byte_offset: 0x60,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::Read,
                            bit_size: 32,
                            fieldset: None,
                        },
                    ),
                },
                BlockItem {
                    name: "mondar0",
                    description: Some(
                        "desc MONDAR0.",
                    ),
                    array: None,
                    byte_offset: 0x64,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::Read,
                            bit_size: 32,
                            fieldset: None,
                        },
                    ),
                },
                BlockItem {
                    name: "mondtctl0",
                    description: Some(
                        "desc MONDTCTL0.",
                    ),
                    array: None,
                    byte_offset: 0x68,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::Read,
                            bit_size: 32,
                            fieldset: Some(
                                "Mondtctl0",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "monrpt0",
                    description: Some(
                        "desc MONRPT0.",
                    ),
                    array: None,
                    byte_offset: 0x6c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::Read,
                            bit_size: 32,
                            fieldset: Some(
                                "Monrpt0",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "monsnseqctl0",
                    description: Some(
                        "desc MONSNSEQCTL0.",
                    ),
                    array: None,
                    byte_offset: 0x70,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::Read,
                            bit_size: 32,
                            fieldset: Some(
                                "Monsnseqctl0",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "mondnseqctl0",
                    description: Some(
                        "desc MONDNSEQCTL0.",
                    ),
                    array: None,
                    byte_offset: 0x74,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::Read,
                            bit_size: 32,
                            fieldset: Some(
                                "Mondnseqctl0",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "sar1",
                    description: Some(
                        "desc SAR1.",
                    ),
                    array: None,
                    byte_offset: 0x80,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: None,
                        },
                    ),
                },
                BlockItem {
                    name: "dar1",
                    description: Some(
                        "desc DAR1.",
                    ),
                    array: None,
                    byte_offset: 0x84,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: None,
                        },
                    ),
                },
                BlockItem {
                    name: "dtctl1",
                    description: Some(
                        "desc DTCTL1.",
                    ),
                    array: None,
                    byte_offset: 0x88,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Dtctl1",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "rpt1",
                    description: Some(
                        "desc RPT1.",
                    ),
                    array: None,
                    byte_offset: 0x8c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Rpt1",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "rptb1",
                    description: Some(
                        "desc RPTB1.",
                    ),
                    array: None,
                    byte_offset: 0x8c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Rptb1",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "snseqctl1",
                    description: Some(
                        "desc SNSEQCTL1.",
                    ),
                    array: None,
                    byte_offset: 0x90,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Snseqctl1",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "snseqctlb1",
                    description: Some(
                        "desc SNSEQCTLB1.",
                    ),
                    array: None,
                    byte_offset: 0x90,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Snseqctlb1",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "dnseqctl1",
                    description: Some(
                        "desc DNSEQCTL1.",
                    ),
                    array: None,
                    byte_offset: 0x94,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Dnseqctl1",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "dnseqctlb1",
                    description: Some(
                        "desc DNSEQCTLB1.",
                    ),
                    array: None,
                    byte_offset: 0x94,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Dnseqctlb1",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "llp1",
                    description: Some(
                        "desc LLP1.",
                    ),
                    array: None,
                    byte_offset: 0x98,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Llp1",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "chctl1",
                    description: Some(
                        "desc CHCTL1.",
                    ),
                    array: None,
                    byte_offset: 0x9c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Chctl1",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "monsar1",
                    description: Some(
                        "desc MONSAR1.",
                    ),
                    array: None,
                    byte_offset: 0xa0,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::Read,
                            bit_size: 32,
                            fieldset: None,
                        },
                    ),
                },
                BlockItem {
                    name: "mondar1",
                    description: Some(
                        "desc MONDAR1.",
                    ),
                    array: None,
                    byte_offset: 0xa4,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::Read,
                            bit_size: 32,
                            fieldset: None,
                        },
                    ),
                },
                BlockItem {
                    name: "mondtctl1",
                    description: Some(
                        "desc MONDTCTL1.",
                    ),
                    array: None,
                    byte_offset: 0xa8,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::Read,
                            bit_size: 32,
                            fieldset: Some(
                                "Mondtctl1",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "monrpt1",
                    description: Some(
                        "desc MONRPT1.",
                    ),
                    array: None,
                    byte_offset: 0xac,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::Read,
                            bit_size: 32,
                            fieldset: Some(
                                "Monrpt1",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "monsnseqctl1",
                    description: Some(
                        "desc MONSNSEQCTL1.",
                    ),
                    array: None,
                    byte_offset: 0xb0,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::Read,
                            bit_size: 32,
                            fieldset: Some(
                                "Monsnseqctl1",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "mondnseqctl1",
                    description: Some(
                        "desc MONDNSEQCTL1.",
                    ),
                    array: None,
                    byte_offset: 0xb4,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::Read,
                            bit_size: 32,
                            fieldset: Some(
                                "Mondnseqctl1",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "sar2",
                    description: Some(
                        "desc SAR2.",
                    ),
                    array: None,
                    byte_offset: 0xc0,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: None,
                        },
                    ),
                },
                BlockItem {
                    name: "dar2",
                    description: Some(
                        "desc DAR2.",
                    ),
                    array: None,
                    byte_offset: 0xc4,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: None,
                        },
                    ),
                },
                BlockItem {
                    name: "dtctl2",
                    description: Some(
                        "desc DTCTL2.",
                    ),
                    array: None,
                    byte_offset: 0xc8,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Dtctl2",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "rpt2",
                    description: Some(
                        "desc RPT2.",
                    ),
                    array: None,
                    byte_offset: 0xcc,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Rpt2",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "rptb2",
                    description: Some(
                        "desc RPTB2.",
                    ),
                    array: None,
                    byte_offset: 0xcc,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Rptb2",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "snseqctl2",
                    description: Some(
                        "desc SNSEQCTL2.",
                    ),
                    array: None,
                    byte_offset: 0xd0,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Snseqctl2",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "snseqctlb2",
                    description: Some(
                        "desc SNSEQCTLB2.",
                    ),
                    array: None,
                    byte_offset: 0xd0,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Snseqctlb2",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "dnseqctl2",
                    description: Some(
                        "desc DNSEQCTL2.",
                    ),
                    array: None,
                    byte_offset: 0xd4,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Dnseqctl2",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "dnseqctlb2",
                    description: Some(
                        "desc DNSEQCTLB2.",
                    ),
                    array: None,
                    byte_offset: 0xd4,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Dnseqctlb2",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "llp2",
                    description: Some(
                        "desc LLP2.",
                    ),
                    array: None,
                    byte_offset: 0xd8,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Llp2",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "chctl2",
                    description: Some(
                        "desc CHCTL2.",
                    ),
                    array: None,
                    byte_offset: 0xdc,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Chctl2",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "monsar2",
                    description: Some(
                        "desc MONSAR2.",
                    ),
                    array: None,
                    byte_offset: 0xe0,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::Read,
                            bit_size: 32,
                            fieldset: None,
                        },
                    ),
                },
                BlockItem {
                    name: "mondar2",
                    description: Some(
                        "desc MONDAR2.",
                    ),
                    array: None,
                    byte_offset: 0xe4,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::Read,
                            bit_size: 32,
                            fieldset: None,
                        },
                    ),
                },
                BlockItem {
                    name: "mondtctl2",
                    description: Some(
                        "desc MONDTCTL2.",
                    ),
                    array: None,
                    byte_offset: 0xe8,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::Read,
                            bit_size: 32,
                            fieldset: Some(
                                "Mondtctl2",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "monrpt2",
                    description: Some(
                        "desc MONRPT2.",
                    ),
                    array: None,
                    byte_offset: 0xec,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::Read,
                            bit_size: 32,
                            fieldset: Some(
                                "Monrpt2",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "monsnseqctl2",
                    description: Some(
                        "desc MONSNSEQCTL2.",
                    ),
                    array: None,
                    byte_offset: 0xf0,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::Read,
                            bit_size: 32,
                            fieldset: Some(
                                "Monsnseqctl2",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "mondnseqctl2",
                    description: Some(
                        "desc MONDNSEQCTL2.",
                    ),
                    array: None,
                    byte_offset: 0xf4,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::Read,
                            bit_size: 32,
                            fieldset: Some(
                                "Mondnseqctl2",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "sar3",
                    description: Some(
                        "desc SAR3.",
                    ),
                    array: None,
                    byte_offset: 0x100,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: None,
                        },
                    ),
                },
                BlockItem {
                    name: "dar3",
                    description: Some(
                        "desc DAR3.",
                    ),
                    array: None,
                    byte_offset: 0x104,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: None,
                        },
                    ),
                },
                BlockItem {
                    name: "dtctl3",
                    description: Some(
                        "desc DTCTL3.",
                    ),
                    array: None,
                    byte_offset: 0x108,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Dtctl3",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "rpt3",
                    description: Some(
                        "desc RPT3.",
                    ),
                    array: None,
                    byte_offset: 0x10c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Rpt3",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "rptb3",
                    description: Some(
                        "desc RPTB3.",
                    ),
                    array: None,
                    byte_offset: 0x10c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Rptb3",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "snseqctl3",
                    description: Some(
                        "desc SNSEQCTL3.",
                    ),
                    array: None,
                    byte_offset: 0x110,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Snseqctl3",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "snseqctlb3",
                    description: Some(
                        "desc SNSEQCTLB3.",
                    ),
                    array: None,
                    byte_offset: 0x110,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Snseqctlb3",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "dnseqctl3",
                    description: Some(
                        "desc DNSEQCTL3.",
                    ),
                    array: None,
                    byte_offset: 0x114,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Dnseqctl3",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "dnseqctlb3",
                    description: Some(
                        "desc DNSEQCTLB3.",
                    ),
                    array: None,
                    byte_offset: 0x114,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Dnseqctlb3",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "llp3",
                    description: Some(
                        "desc LLP3.",
                    ),
                    array: None,
                    byte_offset: 0x118,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Llp3",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "chctl3",
                    description: Some(
                        "desc CHCTL3.",
                    ),
                    array: None,
                    byte_offset: 0x11c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Chctl3",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "monsar3",
                    description: Some(
                        "desc MONSAR3.",
                    ),
                    array: None,
                    byte_offset: 0x120,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::Read,
                            bit_size: 32,
                            fieldset: None,
                        },
                    ),
                },
                BlockItem {
                    name: "mondar3",
                    description: Some(
                        "desc MONDAR3.",
                    ),
                    array: None,
                    byte_offset: 0x124,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::Read,
                            bit_size: 32,
                            fieldset: None,
                        },
                    ),
                },
                BlockItem {
                    name: "mondtctl3",
                    description: Some(
                        "desc MONDTCTL3.",
                    ),
                    array: None,
                    byte_offset: 0x128,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::Read,
                            bit_size: 32,
                            fieldset: Some(
                                "Mondtctl3",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "monrpt3",
                    description: Some(
                        "desc MONRPT3.",
                    ),
                    array: None,
                    byte_offset: 0x12c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::Read,
                            bit_size: 32,
                            fieldset: Some(
                                "Monrpt3",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "monsnseqctl3",
                    description: Some(
                        "desc MONSNSEQCTL3.",
                    ),
                    array: None,
                    byte_offset: 0x130,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::Read,
                            bit_size: 32,
                            fieldset: Some(
                                "Monsnseqctl3",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "mondnseqctl3",
                    description: Some(
                        "desc MONDNSEQCTL3.",
                    ),
                    array: None,
                    byte_offset: 0x134,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::Read,
                            bit_size: 32,
                            fieldset: Some(
                                "Mondnseqctl3",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "sar4",
                    description: Some(
                        "desc SAR4.",
                    ),
                    array: None,
                    byte_offset: 0x140,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: None,
                        },
                    ),
                },
                BlockItem {
                    name: "dar4",
                    description: Some(
                        "desc DAR4.",
                    ),
                    array: None,
                    byte_offset: 0x144,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: None,
                        },
                    ),
                },
                BlockItem {
                    name: "dtctl4",
                    description: Some(
                        "desc DTCTL4.",
                    ),
                    array: None,
                    byte_offset: 0x148,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Dtctl4",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "rpt4",
                    description: Some(
                        "desc RPT4.",
                    ),
                    array: None,
                    byte_offset: 0x14c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Rpt4",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "rptb4",
                    description: Some(
                        "desc RPTB4.",
                    ),
                    array: None,
                    byte_offset: 0x14c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Rptb4",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "snseqctl4",
                    description: Some(
                        "desc SNSEQCTL4.",
                    ),
                    array: None,
                    byte_offset: 0x150,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Snseqctl4",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "snseqctlb4",
                    description: Some(
                        "desc SNSEQCTLB4.",
                    ),
                    array: None,
                    byte_offset: 0x150,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Snseqctlb4",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "dnseqctl4",
                    description: Some(
                        "desc DNSEQCTL4.",
                    ),
                    array: None,
                    byte_offset: 0x154,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Dnseqctl4",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "dnseqctlb4",
                    description: Some(
                        "desc DNSEQCTLB4.",
                    ),
                    array: None,
                    byte_offset: 0x154,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Dnseqctlb4",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "llp4",
                    description: Some(
                        "desc LLP4.",
                    ),
                    array: None,
                    byte_offset: 0x158,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Llp4",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "chctl4",
                    description: Some(
                        "desc CHCTL4.",
                    ),
                    array: None,
                    byte_offset: 0x15c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Chctl4",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "monsar4",
                    description: Some(
                        "desc MONSAR4.",
                    ),
                    array: None,
                    byte_offset: 0x160,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::Read,
                            bit_size: 32,
                            fieldset: None,
                        },
                    ),
                },
                BlockItem {
                    name: "mondar4",
                    description: Some(
                        "desc MONDAR4.",
                    ),
                    array: None,
                    byte_offset: 0x164,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::Read,
                            bit_size: 32,
                            fieldset: None,
                        },
                    ),
                },
                BlockItem {
                    name: "mondtctl4",
                    description: Some(
                        "desc MONDTCTL4.",
                    ),
                    array: None,
                    byte_offset: 0x168,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::Read,
                            bit_size: 32,
                            fieldset: Some(
                                "Mondtctl4",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "monrpt4",
                    description: Some(
                        "desc MONRPT4.",
                    ),
                    array: None,
                    byte_offset: 0x16c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::Read,
                            bit_size: 32,
                            fieldset: Some(
                                "Monrpt4",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "monsnseqctl4",
                    description: Some(
                        "desc MONSNSEQCTL4.",
                    ),
                    array: None,
                    byte_offset: 0x170,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::Read,
                            bit_size: 32,
                            fieldset: Some(
                                "Monsnseqctl4",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "mondnseqctl4",
                    description: Some(
                        "desc MONDNSEQCTL4.",
                    ),
                    array: None,
                    byte_offset: 0x174,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::Read,
                            bit_size: 32,
                            fieldset: Some(
                                "Mondnseqctl4",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "sar5",
                    description: Some(
                        "desc SAR5.",
                    ),
                    array: None,
                    byte_offset: 0x180,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: None,
                        },
                    ),
                },
                BlockItem {
                    name: "dar5",
                    description: Some(
                        "desc DAR5.",
                    ),
                    array: None,
                    byte_offset: 0x184,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: None,
                        },
                    ),
                },
                BlockItem {
                    name: "dtctl5",
                    description: Some(
                        "desc DTCTL5.",
                    ),
                    array: None,
                    byte_offset: 0x188,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Dtctl5",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "rpt5",
                    description: Some(
                        "desc RPT5.",
                    ),
                    array: None,
                    byte_offset: 0x18c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Rpt5",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "rptb5",
                    description: Some(
                        "desc RPTB5.",
                    ),
                    array: None,
                    byte_offset: 0x18c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Rptb5",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "snseqctl5",
                    description: Some(
                        "desc SNSEQCTL5.",
                    ),
                    array: None,
                    byte_offset: 0x190,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Snseqctl5",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "snseqctlb5",
                    description: Some(
                        "desc SNSEQCTLB5.",
                    ),
                    array: None,
                    byte_offset: 0x190,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Snseqctlb5",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "dnseqctl5",
                    description: Some(
                        "desc DNSEQCTL5.",
                    ),
                    array: None,
                    byte_offset: 0x194,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Dnseqctl5",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "dnseqctlb5",
                    description: Some(
                        "desc DNSEQCTLB5.",
                    ),
                    array: None,
                    byte_offset: 0x194,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Dnseqctlb5",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "llp5",
                    description: Some(
                        "desc LLP5.",
                    ),
                    array: None,
                    byte_offset: 0x198,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Llp5",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "chctl5",
                    description: Some(
                        "desc CHCTL5.",
                    ),
                    array: None,
                    byte_offset: 0x19c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Chctl5",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "monsar5",
                    description: Some(
                        "desc MONSAR5.",
                    ),
                    array: None,
                    byte_offset: 0x1a0,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::Read,
                            bit_size: 32,
                            fieldset: None,
                        },
                    ),
                },
                BlockItem {
                    name: "mondar5",
                    description: Some(
                        "desc MONDAR5.",
                    ),
                    array: None,
                    byte_offset: 0x1a4,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::Read,
                            bit_size: 32,
                            fieldset: None,
                        },
                    ),
                },
                BlockItem {
                    name: "mondtctl5",
                    description: Some(
                        "desc MONDTCTL5.",
                    ),
                    array: None,
                    byte_offset: 0x1a8,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::Read,
                            bit_size: 32,
                            fieldset: Some(
                                "Mondtctl5",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "monrpt5",
                    description: Some(
                        "desc MONRPT5.",
                    ),
                    array: None,
                    byte_offset: 0x1ac,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::Read,
                            bit_size: 32,
                            fieldset: Some(
                                "Monrpt5",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "monsnseqctl5",
                    description: Some(
                        "desc MONSNSEQCTL5.",
                    ),
                    array: None,
                    byte_offset: 0x1b0,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::Read,
                            bit_size: 32,
                            fieldset: Some(
                                "Monsnseqctl5",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "mondnseqctl5",
                    description: Some(
                        "desc MONDNSEQCTL5.",
                    ),
                    array: None,
                    byte_offset: 0x1b4,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::Read,
                            bit_size: 32,
                            fieldset: Some(
                                "Mondnseqctl5",
                            ),
                        },
                    ),
                },
            ],
        },
    ],
    fieldsets: &[
        FieldSet {
            name: "Chctl0",
            extends: None,
            description: Some(
                "desc CHCTL0.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "sinc",
                    description: Some(
                        "desc SINC.",
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
                    name: "dinc",
                    description: Some(
                        "desc DINC.",
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
                    name: "srpten",
                    description: Some(
                        "desc SRPTEN.",
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
                    name: "drpten",
                    description: Some(
                        "desc DRPTEN.",
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
                    name: "snseqen",
                    description: Some(
                        "desc SNSEQEN.",
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
                    name: "dnseqen",
                    description: Some(
                        "desc DNSEQEN.",
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
                    name: "hsize",
                    description: Some(
                        "desc HSIZE.",
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
                    name: "llpen",
                    description: Some(
                        "desc LLPEN.",
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
                    name: "llprun",
                    description: Some(
                        "desc LLPRUN.",
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
                    name: "ie",
                    description: Some(
                        "desc IE.",
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
            ],
        },
        FieldSet {
            name: "Chctl1",
            extends: None,
            description: Some(
                "desc CHCTL1.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "sinc",
                    description: Some(
                        "desc SINC.",
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
                    name: "dinc",
                    description: Some(
                        "desc DINC.",
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
                    name: "srpten",
                    description: Some(
                        "desc SRPTEN.",
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
                    name: "drpten",
                    description: Some(
                        "desc DRPTEN.",
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
                    name: "snseqen",
                    description: Some(
                        "desc SNSEQEN.",
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
                    name: "dnseqen",
                    description: Some(
                        "desc DNSEQEN.",
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
                    name: "hsize",
                    description: Some(
                        "desc HSIZE.",
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
                    name: "llpen",
                    description: Some(
                        "desc LLPEN.",
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
                    name: "llprun",
                    description: Some(
                        "desc LLPRUN.",
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
                    name: "ie",
                    description: Some(
                        "desc IE.",
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
            ],
        },
        FieldSet {
            name: "Chctl2",
            extends: None,
            description: Some(
                "desc CHCTL2.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "sinc",
                    description: Some(
                        "desc SINC.",
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
                    name: "dinc",
                    description: Some(
                        "desc DINC.",
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
                    name: "srpten",
                    description: Some(
                        "desc SRPTEN.",
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
                    name: "drpten",
                    description: Some(
                        "desc DRPTEN.",
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
                    name: "snseqen",
                    description: Some(
                        "desc SNSEQEN.",
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
                    name: "dnseqen",
                    description: Some(
                        "desc DNSEQEN.",
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
                    name: "hsize",
                    description: Some(
                        "desc HSIZE.",
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
                    name: "llpen",
                    description: Some(
                        "desc LLPEN.",
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
                    name: "llprun",
                    description: Some(
                        "desc LLPRUN.",
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
                    name: "ie",
                    description: Some(
                        "desc IE.",
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
            ],
        },
        FieldSet {
            name: "Chctl3",
            extends: None,
            description: Some(
                "desc CHCTL3.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "sinc",
                    description: Some(
                        "desc SINC.",
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
                    name: "dinc",
                    description: Some(
                        "desc DINC.",
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
                    name: "srpten",
                    description: Some(
                        "desc SRPTEN.",
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
                    name: "drpten",
                    description: Some(
                        "desc DRPTEN.",
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
                    name: "snseqen",
                    description: Some(
                        "desc SNSEQEN.",
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
                    name: "dnseqen",
                    description: Some(
                        "desc DNSEQEN.",
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
                    name: "hsize",
                    description: Some(
                        "desc HSIZE.",
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
                    name: "llpen",
                    description: Some(
                        "desc LLPEN.",
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
                    name: "llprun",
                    description: Some(
                        "desc LLPRUN.",
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
                    name: "ie",
                    description: Some(
                        "desc IE.",
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
            ],
        },
        FieldSet {
            name: "Chctl4",
            extends: None,
            description: Some(
                "desc CHCTL4.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "sinc",
                    description: Some(
                        "desc SINC.",
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
                    name: "dinc",
                    description: Some(
                        "desc DINC.",
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
                    name: "srpten",
                    description: Some(
                        "desc SRPTEN.",
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
                    name: "drpten",
                    description: Some(
                        "desc DRPTEN.",
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
                    name: "snseqen",
                    description: Some(
                        "desc SNSEQEN.",
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
                    name: "dnseqen",
                    description: Some(
                        "desc DNSEQEN.",
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
                    name: "hsize",
                    description: Some(
                        "desc HSIZE.",
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
                    name: "llpen",
                    description: Some(
                        "desc LLPEN.",
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
                    name: "llprun",
                    description: Some(
                        "desc LLPRUN.",
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
                    name: "ie",
                    description: Some(
                        "desc IE.",
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
            ],
        },
        FieldSet {
            name: "Chctl5",
            extends: None,
            description: Some(
                "desc CHCTL5.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "sinc",
                    description: Some(
                        "desc SINC.",
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
                    name: "dinc",
                    description: Some(
                        "desc DINC.",
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
                    name: "srpten",
                    description: Some(
                        "desc SRPTEN.",
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
                    name: "drpten",
                    description: Some(
                        "desc DRPTEN.",
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
                    name: "snseqen",
                    description: Some(
                        "desc SNSEQEN.",
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
                    name: "dnseqen",
                    description: Some(
                        "desc DNSEQEN.",
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
                    name: "hsize",
                    description: Some(
                        "desc HSIZE.",
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
                    name: "llpen",
                    description: Some(
                        "desc LLPEN.",
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
                    name: "llprun",
                    description: Some(
                        "desc LLPRUN.",
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
                    name: "ie",
                    description: Some(
                        "desc IE.",
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
            ],
        },
        FieldSet {
            name: "Chen",
            extends: None,
            description: Some(
                "desc CHEN.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "chen",
                    description: Some(
                        "desc CHEN.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 6,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Chenclr",
            extends: None,
            description: Some(
                "desc CHENCLR.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "chenclr",
                    description: Some(
                        "desc CHENCLR.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 6,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Chstat",
            extends: None,
            description: Some(
                "desc CHSTAT.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "dmaact",
                    description: Some(
                        "desc DMAACT.",
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
                    name: "rcfgact",
                    description: Some(
                        "desc RCFGACT.",
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
                    name: "chact",
                    description: Some(
                        "desc CHACT.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 16,
                        },
                    ),
                    bit_size: 6,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Dnseqctl0",
            extends: None,
            description: Some(
                "desc DNSEQCTL0.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "doffset",
                    description: Some(
                        "desc DOFFSET.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 20,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "dnscnt",
                    description: Some(
                        "desc DNSCNT.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 20,
                        },
                    ),
                    bit_size: 12,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Dnseqctl1",
            extends: None,
            description: Some(
                "desc DNSEQCTL1.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "doffset",
                    description: Some(
                        "desc DOFFSET.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 20,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "dnscnt",
                    description: Some(
                        "desc DNSCNT.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 20,
                        },
                    ),
                    bit_size: 12,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Dnseqctl2",
            extends: None,
            description: Some(
                "desc DNSEQCTL2.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "doffset",
                    description: Some(
                        "desc DOFFSET.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 20,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "dnscnt",
                    description: Some(
                        "desc DNSCNT.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 20,
                        },
                    ),
                    bit_size: 12,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Dnseqctl3",
            extends: None,
            description: Some(
                "desc DNSEQCTL3.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "doffset",
                    description: Some(
                        "desc DOFFSET.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 20,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "dnscnt",
                    description: Some(
                        "desc DNSCNT.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 20,
                        },
                    ),
                    bit_size: 12,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Dnseqctl4",
            extends: None,
            description: Some(
                "desc DNSEQCTL4.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "doffset",
                    description: Some(
                        "desc DOFFSET.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 20,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "dnscnt",
                    description: Some(
                        "desc DNSCNT.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 20,
                        },
                    ),
                    bit_size: 12,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Dnseqctl5",
            extends: None,
            description: Some(
                "desc DNSEQCTL5.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "doffset",
                    description: Some(
                        "desc DOFFSET.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 20,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "dnscnt",
                    description: Some(
                        "desc DNSCNT.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 20,
                        },
                    ),
                    bit_size: 12,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Dnseqctlb0",
            extends: None,
            description: Some(
                "desc DNSEQCTLB0.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "dnsdist",
                    description: Some(
                        "desc DNSDIST.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 20,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "dnscntb",
                    description: Some(
                        "desc DNSCNTB.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 20,
                        },
                    ),
                    bit_size: 12,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Dnseqctlb1",
            extends: None,
            description: Some(
                "desc DNSEQCTLB1.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "dnsdist",
                    description: Some(
                        "desc DNSDIST.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 20,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "dnscntb",
                    description: Some(
                        "desc DNSCNTB.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 20,
                        },
                    ),
                    bit_size: 12,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Dnseqctlb2",
            extends: None,
            description: Some(
                "desc DNSEQCTLB2.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "dnsdist",
                    description: Some(
                        "desc DNSDIST.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 20,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "dnscntb",
                    description: Some(
                        "desc DNSCNTB.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 20,
                        },
                    ),
                    bit_size: 12,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Dnseqctlb3",
            extends: None,
            description: Some(
                "desc DNSEQCTLB3.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "dnsdist",
                    description: Some(
                        "desc DNSDIST.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 20,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "dnscntb",
                    description: Some(
                        "desc DNSCNTB.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 20,
                        },
                    ),
                    bit_size: 12,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Dnseqctlb4",
            extends: None,
            description: Some(
                "desc DNSEQCTLB4.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "dnsdist",
                    description: Some(
                        "desc DNSDIST.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 20,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "dnscntb",
                    description: Some(
                        "desc DNSCNTB.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 20,
                        },
                    ),
                    bit_size: 12,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Dnseqctlb5",
            extends: None,
            description: Some(
                "desc DNSEQCTLB5.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "dnsdist",
                    description: Some(
                        "desc DNSDIST.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 20,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "dnscntb",
                    description: Some(
                        "desc DNSCNTB.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 20,
                        },
                    ),
                    bit_size: 12,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Dtctl0",
            extends: None,
            description: Some(
                "desc DTCTL0.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "blksize",
                    description: Some(
                        "desc BLKSIZE.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 10,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "cnt",
                    description: Some(
                        "desc CNT.",
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
            name: "Dtctl1",
            extends: None,
            description: Some(
                "desc DTCTL1.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "blksize",
                    description: Some(
                        "desc BLKSIZE.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 10,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "cnt",
                    description: Some(
                        "desc CNT.",
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
            name: "Dtctl2",
            extends: None,
            description: Some(
                "desc DTCTL2.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "blksize",
                    description: Some(
                        "desc BLKSIZE.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 10,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "cnt",
                    description: Some(
                        "desc CNT.",
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
            name: "Dtctl3",
            extends: None,
            description: Some(
                "desc DTCTL3.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "blksize",
                    description: Some(
                        "desc BLKSIZE.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 10,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "cnt",
                    description: Some(
                        "desc CNT.",
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
            name: "Dtctl4",
            extends: None,
            description: Some(
                "desc DTCTL4.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "blksize",
                    description: Some(
                        "desc BLKSIZE.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 10,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "cnt",
                    description: Some(
                        "desc CNT.",
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
            name: "Dtctl5",
            extends: None,
            description: Some(
                "desc DTCTL5.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "blksize",
                    description: Some(
                        "desc BLKSIZE.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 10,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "cnt",
                    description: Some(
                        "desc CNT.",
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
            name: "En",
            extends: None,
            description: Some(
                "desc EN.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "en",
                    description: Some(
                        "desc EN.",
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
            name: "Intclr0",
            extends: None,
            description: Some(
                "desc INTCLR0.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "clrtrnerr",
                    description: Some(
                        "desc CLRTRNERR.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 6,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "clrreqerr",
                    description: Some(
                        "desc CLRREQERR.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 16,
                        },
                    ),
                    bit_size: 6,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Intclr1",
            extends: None,
            description: Some(
                "desc INTCLR1.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "clrtc",
                    description: Some(
                        "desc CLRTC.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 6,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "clrbtc",
                    description: Some(
                        "desc CLRBTC.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 16,
                        },
                    ),
                    bit_size: 6,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Intmask0",
            extends: None,
            description: Some(
                "desc INTMASK0.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "msktrnerr",
                    description: Some(
                        "desc MSKTRNERR.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 6,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "mskreqerr",
                    description: Some(
                        "desc MSKREQERR.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 16,
                        },
                    ),
                    bit_size: 6,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Intmask1",
            extends: None,
            description: Some(
                "desc INTMASK1.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "msktc",
                    description: Some(
                        "desc MSKTC.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 6,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "mskbtc",
                    description: Some(
                        "desc MSKBTC.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 16,
                        },
                    ),
                    bit_size: 6,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Intstat0",
            extends: None,
            description: Some(
                "desc INTSTAT0.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "trnerr",
                    description: Some(
                        "desc TRNERR.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 6,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "reqerr",
                    description: Some(
                        "desc REQERR.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 16,
                        },
                    ),
                    bit_size: 6,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Intstat1",
            extends: None,
            description: Some(
                "desc INTSTAT1.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "tc",
                    description: Some(
                        "desc TC.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 6,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "btc",
                    description: Some(
                        "desc BTC.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 16,
                        },
                    ),
                    bit_size: 6,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Llp0",
            extends: None,
            description: Some(
                "desc LLP0.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "llp",
                    description: Some(
                        "desc LLP.",
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
            name: "Llp1",
            extends: None,
            description: Some(
                "desc LLP1.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "llp",
                    description: Some(
                        "desc LLP.",
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
            name: "Llp2",
            extends: None,
            description: Some(
                "desc LLP2.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "llp",
                    description: Some(
                        "desc LLP.",
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
            name: "Llp3",
            extends: None,
            description: Some(
                "desc LLP3.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "llp",
                    description: Some(
                        "desc LLP.",
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
            name: "Llp4",
            extends: None,
            description: Some(
                "desc LLP4.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "llp",
                    description: Some(
                        "desc LLP.",
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
            name: "Llp5",
            extends: None,
            description: Some(
                "desc LLP5.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "llp",
                    description: Some(
                        "desc LLP.",
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
            name: "Mondnseqctl0",
            extends: None,
            description: Some(
                "desc MONDNSEQCTL0.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "doffset",
                    description: Some(
                        "desc DOFFSET.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 20,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "dnscnt",
                    description: Some(
                        "desc DNSCNT.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 20,
                        },
                    ),
                    bit_size: 12,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Mondnseqctl1",
            extends: None,
            description: Some(
                "desc MONDNSEQCTL1.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "doffset",
                    description: Some(
                        "desc DOFFSET.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 20,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "dnscnt",
                    description: Some(
                        "desc DNSCNT.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 20,
                        },
                    ),
                    bit_size: 12,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Mondnseqctl2",
            extends: None,
            description: Some(
                "desc MONDNSEQCTL2.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "doffset",
                    description: Some(
                        "desc DOFFSET.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 20,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "dnscnt",
                    description: Some(
                        "desc DNSCNT.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 20,
                        },
                    ),
                    bit_size: 12,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Mondnseqctl3",
            extends: None,
            description: Some(
                "desc MONDNSEQCTL3.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "doffset",
                    description: Some(
                        "desc DOFFSET.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 20,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "dnscnt",
                    description: Some(
                        "desc DNSCNT.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 20,
                        },
                    ),
                    bit_size: 12,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Mondnseqctl4",
            extends: None,
            description: Some(
                "desc MONDNSEQCTL4.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "doffset",
                    description: Some(
                        "desc DOFFSET.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 20,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "dnscnt",
                    description: Some(
                        "desc DNSCNT.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 20,
                        },
                    ),
                    bit_size: 12,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Mondnseqctl5",
            extends: None,
            description: Some(
                "desc MONDNSEQCTL5.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "doffset",
                    description: Some(
                        "desc DOFFSET.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 20,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "dnscnt",
                    description: Some(
                        "desc DNSCNT.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 20,
                        },
                    ),
                    bit_size: 12,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Mondtctl0",
            extends: None,
            description: Some(
                "desc MONDTCTL0.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "blksize",
                    description: Some(
                        "desc BLKSIZE.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 10,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "cnt",
                    description: Some(
                        "desc CNT.",
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
            name: "Mondtctl1",
            extends: None,
            description: Some(
                "desc MONDTCTL1.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "blksize",
                    description: Some(
                        "desc BLKSIZE.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 10,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "cnt",
                    description: Some(
                        "desc CNT.",
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
            name: "Mondtctl2",
            extends: None,
            description: Some(
                "desc MONDTCTL2.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "blksize",
                    description: Some(
                        "desc BLKSIZE.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 10,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "cnt",
                    description: Some(
                        "desc CNT.",
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
            name: "Mondtctl3",
            extends: None,
            description: Some(
                "desc MONDTCTL3.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "blksize",
                    description: Some(
                        "desc BLKSIZE.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 10,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "cnt",
                    description: Some(
                        "desc CNT.",
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
            name: "Mondtctl4",
            extends: None,
            description: Some(
                "desc MONDTCTL4.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "blksize",
                    description: Some(
                        "desc BLKSIZE.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 10,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "cnt",
                    description: Some(
                        "desc CNT.",
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
            name: "Mondtctl5",
            extends: None,
            description: Some(
                "desc MONDTCTL5.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "blksize",
                    description: Some(
                        "desc BLKSIZE.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 10,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "cnt",
                    description: Some(
                        "desc CNT.",
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
            name: "Monrpt0",
            extends: None,
            description: Some(
                "desc MONRPT0.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "srpt",
                    description: Some(
                        "desc SRPT.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 10,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "drpt",
                    description: Some(
                        "desc DRPT.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 16,
                        },
                    ),
                    bit_size: 10,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Monrpt1",
            extends: None,
            description: Some(
                "desc MONRPT1.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "srpt",
                    description: Some(
                        "desc SRPT.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 10,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "drpt",
                    description: Some(
                        "desc DRPT.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 16,
                        },
                    ),
                    bit_size: 10,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Monrpt2",
            extends: None,
            description: Some(
                "desc MONRPT2.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "srpt",
                    description: Some(
                        "desc SRPT.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 10,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "drpt",
                    description: Some(
                        "desc DRPT.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 16,
                        },
                    ),
                    bit_size: 10,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Monrpt3",
            extends: None,
            description: Some(
                "desc MONRPT3.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "srpt",
                    description: Some(
                        "desc SRPT.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 10,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "drpt",
                    description: Some(
                        "desc DRPT.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 16,
                        },
                    ),
                    bit_size: 10,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Monrpt4",
            extends: None,
            description: Some(
                "desc MONRPT4.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "srpt",
                    description: Some(
                        "desc SRPT.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 10,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "drpt",
                    description: Some(
                        "desc DRPT.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 16,
                        },
                    ),
                    bit_size: 10,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Monrpt5",
            extends: None,
            description: Some(
                "desc MONRPT5.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "srpt",
                    description: Some(
                        "desc SRPT.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 10,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "drpt",
                    description: Some(
                        "desc DRPT.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 16,
                        },
                    ),
                    bit_size: 10,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Monsnseqctl0",
            extends: None,
            description: Some(
                "desc MONSNSEQCTL0.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "soffset",
                    description: Some(
                        "desc SOFFSET.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 20,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "snscnt",
                    description: Some(
                        "desc SNSCNT.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 20,
                        },
                    ),
                    bit_size: 12,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Monsnseqctl1",
            extends: None,
            description: Some(
                "desc MONSNSEQCTL1.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "soffset",
                    description: Some(
                        "desc SOFFSET.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 20,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "snscnt",
                    description: Some(
                        "desc SNSCNT.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 20,
                        },
                    ),
                    bit_size: 12,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Monsnseqctl2",
            extends: None,
            description: Some(
                "desc MONSNSEQCTL2.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "soffset",
                    description: Some(
                        "desc SOFFSET.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 20,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "snscnt",
                    description: Some(
                        "desc SNSCNT.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 20,
                        },
                    ),
                    bit_size: 12,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Monsnseqctl3",
            extends: None,
            description: Some(
                "desc MONSNSEQCTL3.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "soffset",
                    description: Some(
                        "desc SOFFSET.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 20,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "snscnt",
                    description: Some(
                        "desc SNSCNT.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 20,
                        },
                    ),
                    bit_size: 12,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Monsnseqctl4",
            extends: None,
            description: Some(
                "desc MONSNSEQCTL4.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "soffset",
                    description: Some(
                        "desc SOFFSET.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 20,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "snscnt",
                    description: Some(
                        "desc SNSCNT.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 20,
                        },
                    ),
                    bit_size: 12,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Monsnseqctl5",
            extends: None,
            description: Some(
                "desc MONSNSEQCTL5.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "soffset",
                    description: Some(
                        "desc SOFFSET.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 20,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "snscnt",
                    description: Some(
                        "desc SNSCNT.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 20,
                        },
                    ),
                    bit_size: 12,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Rcfgctl",
            extends: None,
            description: Some(
                "desc RCFGCTL.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "rcfgen",
                    description: Some(
                        "desc RCFGEN.",
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
                    name: "rcfgllp",
                    description: Some(
                        "desc RCFGLLP.",
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
                    name: "rcfgchs",
                    description: Some(
                        "desc RCFGCHS.",
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
                    name: "sarmd",
                    description: Some(
                        "desc SARMD.",
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
                    name: "darmd",
                    description: Some(
                        "desc DARMD.",
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
                    name: "cntmd",
                    description: Some(
                        "desc CNTMD.",
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
            name: "Reqstat",
            extends: None,
            description: Some(
                "desc REQSTAT.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "chreq",
                    description: Some(
                        "desc CHREQ.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 6,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "rcfgreq",
                    description: Some(
                        "desc RCFGREQ.",
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
            name: "Rpt0",
            extends: None,
            description: Some(
                "desc RPT0.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "srpt",
                    description: Some(
                        "desc SRPT.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 10,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "drpt",
                    description: Some(
                        "desc DRPT.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 16,
                        },
                    ),
                    bit_size: 10,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Rpt1",
            extends: None,
            description: Some(
                "desc RPT1.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "srpt",
                    description: Some(
                        "desc SRPT.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 10,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "drpt",
                    description: Some(
                        "desc DRPT.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 16,
                        },
                    ),
                    bit_size: 10,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Rpt2",
            extends: None,
            description: Some(
                "desc RPT2.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "srpt",
                    description: Some(
                        "desc SRPT.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 10,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "drpt",
                    description: Some(
                        "desc DRPT.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 16,
                        },
                    ),
                    bit_size: 10,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Rpt3",
            extends: None,
            description: Some(
                "desc RPT3.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "srpt",
                    description: Some(
                        "desc SRPT.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 10,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "drpt",
                    description: Some(
                        "desc DRPT.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 16,
                        },
                    ),
                    bit_size: 10,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Rpt4",
            extends: None,
            description: Some(
                "desc RPT4.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "srpt",
                    description: Some(
                        "desc SRPT.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 10,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "drpt",
                    description: Some(
                        "desc DRPT.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 16,
                        },
                    ),
                    bit_size: 10,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Rpt5",
            extends: None,
            description: Some(
                "desc RPT5.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "srpt",
                    description: Some(
                        "desc SRPT.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 10,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "drpt",
                    description: Some(
                        "desc DRPT.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 16,
                        },
                    ),
                    bit_size: 10,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Rptb0",
            extends: None,
            description: Some(
                "desc RPTB0.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "srptb",
                    description: Some(
                        "desc SRPTB.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 10,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "drptb",
                    description: Some(
                        "desc DRPTB.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 16,
                        },
                    ),
                    bit_size: 10,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Rptb1",
            extends: None,
            description: Some(
                "desc RPTB1.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "srptb",
                    description: Some(
                        "desc SRPTB.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 10,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "drptb",
                    description: Some(
                        "desc DRPTB.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 16,
                        },
                    ),
                    bit_size: 10,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Rptb2",
            extends: None,
            description: Some(
                "desc RPTB2.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "srptb",
                    description: Some(
                        "desc SRPTB.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 10,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "drptb",
                    description: Some(
                        "desc DRPTB.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 16,
                        },
                    ),
                    bit_size: 10,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Rptb3",
            extends: None,
            description: Some(
                "desc RPTB3.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "srptb",
                    description: Some(
                        "desc SRPTB.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 10,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "drptb",
                    description: Some(
                        "desc DRPTB.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 16,
                        },
                    ),
                    bit_size: 10,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Rptb4",
            extends: None,
            description: Some(
                "desc RPTB4.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "srptb",
                    description: Some(
                        "desc SRPTB.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 10,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "drptb",
                    description: Some(
                        "desc DRPTB.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 16,
                        },
                    ),
                    bit_size: 10,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Rptb5",
            extends: None,
            description: Some(
                "desc RPTB5.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "srptb",
                    description: Some(
                        "desc SRPTB.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 10,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "drptb",
                    description: Some(
                        "desc DRPTB.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 16,
                        },
                    ),
                    bit_size: 10,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Snseqctl0",
            extends: None,
            description: Some(
                "desc SNSEQCTL0.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "soffset",
                    description: Some(
                        "desc SOFFSET.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 20,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "snscnt",
                    description: Some(
                        "desc SNSCNT.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 20,
                        },
                    ),
                    bit_size: 12,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Snseqctl1",
            extends: None,
            description: Some(
                "desc SNSEQCTL1.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "soffset",
                    description: Some(
                        "desc SOFFSET.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 20,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "snscnt",
                    description: Some(
                        "desc SNSCNT.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 20,
                        },
                    ),
                    bit_size: 12,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Snseqctl2",
            extends: None,
            description: Some(
                "desc SNSEQCTL2.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "soffset",
                    description: Some(
                        "desc SOFFSET.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 20,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "snscnt",
                    description: Some(
                        "desc SNSCNT.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 20,
                        },
                    ),
                    bit_size: 12,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Snseqctl3",
            extends: None,
            description: Some(
                "desc SNSEQCTL3.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "soffset",
                    description: Some(
                        "desc SOFFSET.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 20,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "snscnt",
                    description: Some(
                        "desc SNSCNT.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 20,
                        },
                    ),
                    bit_size: 12,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Snseqctl4",
            extends: None,
            description: Some(
                "desc SNSEQCTL4.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "soffset",
                    description: Some(
                        "desc SOFFSET.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 20,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "snscnt",
                    description: Some(
                        "desc SNSCNT.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 20,
                        },
                    ),
                    bit_size: 12,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Snseqctl5",
            extends: None,
            description: Some(
                "desc SNSEQCTL5.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "soffset",
                    description: Some(
                        "desc SOFFSET.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 20,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "snscnt",
                    description: Some(
                        "desc SNSCNT.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 20,
                        },
                    ),
                    bit_size: 12,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Snseqctlb0",
            extends: None,
            description: Some(
                "desc SNSEQCTLB0.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "snsdist",
                    description: Some(
                        "desc SNSDIST.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 20,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "snscntb",
                    description: Some(
                        "desc SNSCNTB.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 20,
                        },
                    ),
                    bit_size: 12,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Snseqctlb1",
            extends: None,
            description: Some(
                "desc SNSEQCTLB1.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "snsdist",
                    description: Some(
                        "desc SNSDIST.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 20,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "snscntb",
                    description: Some(
                        "desc SNSCNTB.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 20,
                        },
                    ),
                    bit_size: 12,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Snseqctlb2",
            extends: None,
            description: Some(
                "desc SNSEQCTLB2.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "snsdist",
                    description: Some(
                        "desc SNSDIST.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 20,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "snscntb",
                    description: Some(
                        "desc SNSCNTB.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 20,
                        },
                    ),
                    bit_size: 12,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Snseqctlb3",
            extends: None,
            description: Some(
                "desc SNSEQCTLB3.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "snsdist",
                    description: Some(
                        "desc SNSDIST.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 20,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "snscntb",
                    description: Some(
                        "desc SNSCNTB.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 20,
                        },
                    ),
                    bit_size: 12,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Snseqctlb4",
            extends: None,
            description: Some(
                "desc SNSEQCTLB4.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "snsdist",
                    description: Some(
                        "desc SNSDIST.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 20,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "snscntb",
                    description: Some(
                        "desc SNSCNTB.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 20,
                        },
                    ),
                    bit_size: 12,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Snseqctlb5",
            extends: None,
            description: Some(
                "desc SNSEQCTLB5.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "snsdist",
                    description: Some(
                        "desc SNSDIST.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 20,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "snscntb",
                    description: Some(
                        "desc SNSCNTB.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 20,
                        },
                    ),
                    bit_size: 12,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Swreq",
            extends: None,
            description: Some(
                "desc SWREQ.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "swreq",
                    description: Some(
                        "desc SWREQ.",
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
                    name: "swrcfgreq",
                    description: Some(
                        "desc SWRCFGREQ.",
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
                    name: "swreqwp",
                    description: Some(
                        "desc SWREQWP.",
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
                    name: "swrcfgwp",
                    description: Some(
                        "desc SWRCFGWP.",
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
                