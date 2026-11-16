
                    use crate::metadata::ir::*;
                    pub(crate) static REGISTERS: IR = IR {
    blocks: &[
        Block {
            name: "Usbfs",
            extends: None,
            description: Some(
                "desc USBFS.",
            ),
            items: &[
                BlockItem {
                    name: "gahbcfg",
                    description: Some(
                        "desc GAHBCFG.",
                    ),
                    array: None,
                    byte_offset: 0x8,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Gahbcfg",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "gusbcfg",
                    description: Some(
                        "desc GUSBCFG.",
                    ),
                    array: None,
                    byte_offset: 0xc,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Gusbcfg",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "grstctl",
                    description: Some(
                        "desc GRSTCTL.",
                    ),
                    array: None,
                    byte_offset: 0x10,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Grstctl",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "gintsts",
                    description: Some(
                        "desc GINTSTS.",
                    ),
                    array: None,
                    byte_offset: 0x14,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::Read,
                            bit_size: 32,
                            fieldset: Some(
                                "Gintsts",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "gintmsk",
                    description: Some(
                        "desc GINTMSK.",
                    ),
                    array: None,
                    byte_offset: 0x18,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Gintmsk",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "grxstsr",
                    description: Some(
                        "desc GRXSTSR.",
                    ),
                    array: None,
                    byte_offset: 0x1c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::Read,
                            bit_size: 32,
                            fieldset: Some(
                                "Grxstsr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "grxstsp",
                    description: Some(
                        "desc GRXSTSP.",
                    ),
                    array: None,
                    byte_offset: 0x20,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::Read,
                            bit_size: 32,
                            fieldset: Some(
                                "Grxstsp",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "grxfsiz",
                    description: Some(
                        "desc GRXFSIZ.",
                    ),
                    array: None,
                    byte_offset: 0x24,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Grxfsiz",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "cid",
                    description: Some(
                        "desc CID.",
                    ),
                    array: None,
                    byte_offset: 0x3c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::Read,
                            bit_size: 32,
                            fieldset: None,
                        },
                    ),
                },
                BlockItem {
                    name: "dieptxf1",
                    description: Some(
                        "desc DIEPTXF1.",
                    ),
                    array: None,
                    byte_offset: 0x104,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Dieptxf1",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "dieptxf2",
                    description: Some(
                        "desc DIEPTXF2.",
                    ),
                    array: None,
                    byte_offset: 0x108,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Dieptxf2",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "dieptxf3",
                    description: Some(
                        "desc DIEPTXF3.",
                    ),
                    array: None,
                    byte_offset: 0x10c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Dieptxf3",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "dieptxf4",
                    description: Some(
                        "desc DIEPTXF4.",
                    ),
                    array: None,
                    byte_offset: 0x110,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Dieptxf4",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "dcfg",
                    description: Some(
                        "desc DCFG.",
                    ),
                    array: None,
                    byte_offset: 0x800,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Dcfg",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "dctl",
                    description: Some(
                        "desc DCTL.",
                    ),
                    array: None,
                    byte_offset: 0x804,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Dctl",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "dsts",
                    description: Some(
                        "desc DSTS.",
                    ),
                    array: None,
                    byte_offset: 0x808,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::Read,
                            bit_size: 32,
                            fieldset: Some(
                                "Dsts",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "diepmsk",
                    description: Some(
                        "desc DIEPMSK.",
                    ),
                    array: None,
                    byte_offset: 0x810,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Diepmsk",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "doepmsk",
                    description: Some(
                        "desc DOEPMSK.",
                    ),
                    array: None,
                    byte_offset: 0x814,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Doepmsk",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "daint",
                    description: Some(
                        "desc DAINT.",
                    ),
                    array: None,
                    byte_offset: 0x818,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::Read,
                            bit_size: 32,
                            fieldset: Some(
                                "Daint",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "daintmsk",
                    description: Some(
                        "desc DAINTMSK.",
                    ),
                    array: None,
                    byte_offset: 0x81c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::Read,
                            bit_size: 32,
                            fieldset: Some(
                                "Daintmsk",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "diepempmsk",
                    description: Some(
                        "desc DIEPEMPMSK.",
                    ),
                    array: None,
                    byte_offset: 0x834,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Diepempmsk",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "diepctl0",
                    description: Some(
                        "desc DIEPCTL0.",
                    ),
                    array: None,
                    byte_offset: 0x900,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::Read,
                            bit_size: 32,
                            fieldset: Some(
                                "Diepctl0",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "diepint0",
                    description: Some(
                        "desc DIEPINT0.",
                    ),
                    array: None,
                    byte_offset: 0x908,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Diepint0",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "dieptsiz0",
                    description: Some(
                        "desc DIEPTSIZ0.",
                    ),
                    array: None,
                    byte_offset: 0x910,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Dieptsiz0",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "diepdma0",
                    description: Some(
                        "desc DIEPDMA0.",
                    ),
                    array: None,
                    byte_offset: 0x914,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: None,
                        },
                    ),
                },
                BlockItem {
                    name: "dtxfsts0",
                    description: Some(
                        "desc DTXFSTS0.",
                    ),
                    array: None,
                    byte_offset: 0x918,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::Read,
                            bit_size: 32,
                            fieldset: Some(
                                "Dtxfsts0",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "diepctl1",
                    description: Some(
                        "desc DIEPCTL1.",
                    ),
                    array: None,
                    byte_offset: 0x920,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Diepctl1",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "diepint1",
                    description: Some(
                        "desc DIEPINT1.",
                    ),
                    array: None,
                    byte_offset: 0x928,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Diepint1",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "dieptsiz1",
                    description: Some(
                        "desc DIEPTSIZ1.",
                    ),
                    array: None,
                    byte_offset: 0x930,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Dieptsiz1",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "diepdma1",
                    description: Some(
                        "desc DIEPDMA1.",
                    ),
                    array: None,
                    byte_offset: 0x934,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: None,
                        },
                    ),
                },
                BlockItem {
                    name: "dtxfsts1",
                    description: Some(
                        "desc DTXFSTS1.",
                    ),
                    array: None,
                    byte_offset: 0x938,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::Read,
                            bit_size: 32,
                            fieldset: Some(
                                "Dtxfsts1",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "diepctl2",
                    description: Some(
                        "desc DIEPCTL2.",
                    ),
                    array: None,
                    byte_offset: 0x940,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Diepctl2",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "diepint2",
                    description: Some(
                        "desc DIEPINT2.",
                    ),
                    array: None,
                    byte_offset: 0x948,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Diepint2",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "dieptsiz2",
                    description: Some(
                        "desc DIEPTSIZ2.",
                    ),
                    array: None,
                    byte_offset: 0x950,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Dieptsiz2",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "diepdma2",
                    description: Some(
                        "desc DIEPDMA2.",
                    ),
                    array: None,
                    byte_offset: 0x954,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: None,
                        },
                    ),
                },
                BlockItem {
                    name: "dtxfsts2",
                    description: Some(
                        "desc DTXFSTS2.",
                    ),
                    array: None,
                    byte_offset: 0x958,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::Read,
                            bit_size: 32,
                            fieldset: Some(
                                "Dtxfsts2",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "diepctl3",
                    description: Some(
                        "desc DIEPCTL3.",
                    ),
                    array: None,
                    byte_offset: 0x960,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Diepctl3",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "diepint3",
                    description: Some(
                        "desc DIEPINT3.",
                    ),
                    array: None,
                    byte_offset: 0x968,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Diepint3",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "dieptsiz3",
                    description: Some(
                        "desc DIEPTSIZ3.",
                    ),
                    array: None,
                    byte_offset: 0x970,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Dieptsiz3",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "diepdma3",
                    description: Some(
                        "desc DIEPDMA3.",
                    ),
                    array: None,
                    byte_offset: 0x974,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: None,
                        },
                    ),
                },
                BlockItem {
                    name: "dtxfsts3",
                    description: Some(
                        "desc DTXFSTS3.",
                    ),
                    array: None,
                    byte_offset: 0x978,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::Read,
                            bit_size: 32,
                            fieldset: Some(
                                "Dtxfsts3",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "diepctl4",
                    description: Some(
                        "desc DIEPCTL4.",
                    ),
                    array: None,
                    byte_offset: 0x980,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Diepctl4",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "diepint4",
                    description: Some(
                        "desc DIEPINT4.",
                    ),
                    array: None,
                    byte_offset: 0x988,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Diepint4",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "dieptsiz4",
                    description: Some(
                        "desc DIEPTSIZ4.",
                    ),
                    array: None,
                    byte_offset: 0x990,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Dieptsiz4",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "diepdma4",
                    description: Some(
                        "desc DIEPDMA4.",
                    ),
                    array: None,
                    byte_offset: 0x994,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: None,
                        },
                    ),
                },
                BlockItem {
                    name: "dtxfsts4",
                    description: Some(
                        "desc DTXFSTS4.",
                    ),
                    array: None,
                    byte_offset: 0x998,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::Read,
                            bit_size: 32,
                            fieldset: Some(
                                "Dtxfsts4",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "doepctl0",
                    description: Some(
                        "desc DOEPCTL0.",
                    ),
                    array: None,
                    byte_offset: 0xb00,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::Read,
                            bit_size: 32,
                            fieldset: Some(
                                "Doepctl0",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "doepint0",
                    description: Some(
                        "desc DOEPINT0.",
                    ),
                    array: None,
                    byte_offset: 0xb08,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Doepint0",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "doeptsiz0",
                    description: Some(
                        "desc DOEPTSIZ0.",
                    ),
                    array: None,
                    byte_offset: 0xb10,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Doeptsiz0",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "doepdma0",
                    description: Some(
                        "desc DOEPDMA0.",
                    ),
                    array: None,
                    byte_offset: 0xb14,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: None,
                        },
                    ),
                },
                BlockItem {
                    name: "doepctl1",
                    description: Some(
                        "desc DOEPCTL1.",
                    ),
                    array: None,
                    byte_offset: 0xb20,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Doepctl1",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "doepint1",
                    description: Some(
                        "desc DOEPINT1.",
                    ),
                    array: None,
                    byte_offset: 0xb28,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Doepint1",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "doeptsiz1",
                    description: Some(
                        "desc DOEPTSIZ1.",
                    ),
                    array: None,
                    byte_offset: 0xb30,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Doeptsiz1",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "doepdma1",
                    description: Some(
                        "desc DOEPDMA1.",
                    ),
                    array: None,
                    byte_offset: 0xb34,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: None,
                        },
                    ),
                },
                BlockItem {
                    name: "doepctl2",
                    description: Some(
                        "desc DOEPCTL2.",
                    ),
                    array: None,
                    byte_offset: 0xb40,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Doepctl2",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "doepint2",
                    description: Some(
                        "desc DOEPINT2.",
                    ),
                    array: None,
                    byte_offset: 0xb48,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Doepint2",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "doeptsiz2",
                    description: Some(
                        "desc DOEPTSIZ2.",
                    ),
                    array: None,
                    byte_offset: 0xb50,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Doeptsiz2",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "doepdma2",
                    description: Some(
                        "desc DOEPDMA2.",
                    ),
                    array: None,
                    byte_offset: 0xb54,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: None,
                        },
                    ),
                },
                BlockItem {
                    name: "doepctl3",
                    description: Some(
                        "desc DOEPCTL3.",
                    ),
                    array: None,
                    byte_offset: 0xb60,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Doepctl3",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "doepint3",
                    description: Some(
                        "desc DOEPINT3.",
                    ),
                    array: None,
                    byte_offset: 0xb68,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Doepint3",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "doeptsiz3",
                    description: Some(
                        "desc DOEPTSIZ3.",
                    ),
                    array: None,
                    byte_offset: 0xb70,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Doeptsiz3",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "doepdma3",
                    description: Some(
                        "desc DOEPDMA3.",
                    ),
                    array: None,
                    byte_offset: 0xb74,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: None,
                        },
                    ),
                },
                BlockItem {
                    name: "doepctl4",
                    description: Some(
                        "desc DOEPCTL4.",
                    ),
                    array: None,
                    byte_offset: 0xb80,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Doepctl4",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "doepint4",
                    description: Some(
                        "desc DOEPINT4.",
                    ),
                    array: None,
                    byte_offset: 0xb88,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Doepint4",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "doeptsiz4",
                    description: Some(
                        "desc DOEPTSIZ4.",
                    ),
                    array: None,
                    byte_offset: 0xb90,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Doeptsiz4",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "doepdma4",
                    description: Some(
                        "desc DOEPDMA4.",
                    ),
                    array: None,
                    byte_offset: 0xb94,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: None,
                        },
                    ),
                },
                BlockItem {
                    name: "pcgcctl",
                    description: Some(
                        "desc PCGCCTL.",
                    ),
                    array: None,
                    byte_offset: 0xe00,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Pcgcctl",
                            ),
                        },
                    ),
                },
            ],
        },
    ],
    fieldsets: &[
        FieldSet {
            name: "Daint",
            extends: None,
            description: Some(
                "desc DAINT.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "iepint",
                    description: Some(
                        "desc IEPINT.",
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
                    name: "oepint",
                    description: Some(
                        "desc OEPINT.",
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
            name: "Daintmsk",
            extends: None,
            description: Some(
                "desc DAINTMSK.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "iepintm",
                    description: Some(
                        "desc IEPINTM.",
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
                    name: "oepintm",
                    description: Some(
                        "desc OEPINTM.",
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
            name: "Dcfg",
            extends: None,
            description: Some(
                "desc DCFG.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "dspd",
                    description: Some(
                        "desc DSPD.",
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
                    name: "nzlsohsk",
                    description: Some(
                        "desc NZLSOHSK.",
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
                    name: "dad",
                    description: Some(
                        "desc DAD.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 4,
                        },
                    ),
                    bit_size: 7,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "pfivl",
                    description: Some(
                        "desc PFIVL.",
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
            ],
        },
        FieldSet {
            name: "Dctl",
            extends: None,
            description: Some(
                "desc DCTL.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "rwusig",
                    description: Some(
                        "desc RWUSIG.",
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
                    name: "sdis",
                    description: Some(
                        "desc SDIS.",
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
                    name: "ginsts",
                    description: Some(
                        "desc GINSTS.",
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
                    name: "gonsts",
                    description: Some(
                        "desc GONSTS.",
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
                    name: "tctl",
                    description: Some(
                        "desc TCTL.",
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
                    name: "sginak",
                    description: Some(
                        "desc SGINAK.",
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
                    name: "cginak",
                    description: Some(
                        "desc CGINAK.",
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
                    name: "sgonak",
                    description: Some(
                        "desc SGONAK.",
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
                    name: "cgonak",
                    description: Some(
                        "desc CGONAK.",
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
                    name: "poprgdne",
                    description: Some(
                        "desc POPRGDNE.",
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
            ],
        },
        FieldSet {
            name: "Diepctl0",
            extends: None,
            description: Some(
                "desc DIEPCTL0.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "mpsiz",
                    description: Some(
                        "desc MPSIZ.",
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
                    name: "usbaep",
                    description: Some(
                        "desc USBAEP.",
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
                    name: "eonum_dpid",
                    description: Some(
                        "desc EONUM_DPID.",
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
                    name: "naksts",
                    description: Some(
                        "desc NAKSTS.",
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
                    name: "eptyp",
                    description: Some(
                        "desc EPTYP.",
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
                    name: "stall",
                    description: Some(
                        "desc STALL.",
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
                    name: "txfnum",
                    description: Some(
                        "desc TXFNUM.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 22,
                        },
                    ),
                    bit_size: 4,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "cnak",
                    description: Some(
                        "desc CNAK.",
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
                    name: "snak",
                    description: Some(
                        "desc SNAK.",
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
                    name: "sd0pid_sevnfrm",
                    description: Some(
                        "desc SD0PID_SEVNFRM.",
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
                    name: "soddfrm",
                    description: Some(
                        "desc SODDFRM.",
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
                    name: "epdis",
                    description: Some(
                        "desc EPDIS.",
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
                    name: "epena",
                    description: Some(
                        "desc EPENA.",
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
            name: "Diepctl1",
            extends: None,
            description: Some(
                "desc DIEPCTL1.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "mpsiz",
                    description: Some(
                        "desc MPSIZ.",
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
                    name: "usbaep",
                    description: Some(
                        "desc USBAEP.",
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
                    name: "eonum_dpid",
                    description: Some(
                        "desc EONUM_DPID.",
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
                    name: "naksts",
                    description: Some(
                        "desc NAKSTS.",
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
                    name: "eptyp",
                    description: Some(
                        "desc EPTYP.",
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
                    name: "stall",
                    description: Some(
                        "desc STALL.",
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
                    name: "txfnum",
                    description: Some(
                        "desc TXFNUM.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 22,
                        },
                    ),
                    bit_size: 4,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "cnak",
                    description: Some(
                        "desc CNAK.",
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
                    name: "snak",
                    description: Some(
                        "desc SNAK.",
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
                    name: "sd0pid_sevnfrm",
                    description: Some(
                        "desc SD0PID_SEVNFRM.",
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
                    name: "soddfrm",
                    description: Some(
                        "desc SODDFRM.",
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
                    name: "epdis",
                    description: Some(
                        "desc EPDIS.",
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
                    name: "epena",
                    description: Some(
                        "desc EPENA.",
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
            name: "Diepctl2",
            extends: None,
            description: Some(
                "desc DIEPCTL2.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "mpsiz",
                    description: Some(
                        "desc MPSIZ.",
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
                    name: "usbaep",
                    description: Some(
                        "desc USBAEP.",
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
                    name: "eonum_dpid",
                    description: Some(
                        "desc EONUM_DPID.",
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
                    name: "naksts",
                    description: Some(
                        "desc NAKSTS.",
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
                    name: "eptyp",
                    description: Some(
                        "desc EPTYP.",
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
                    name: "stall",
                    description: Some(
                        "desc STALL.",
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
                    name: "txfnum",
                    description: Some(
                        "desc TXFNUM.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 22,
                        },
                    ),
                    bit_size: 4,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "cnak",
                    description: Some(
                        "desc CNAK.",
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
                    name: "snak",
                    description: Some(
                        "desc SNAK.",
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
                    name: "sd0pid_sevnfrm",
                    description: Some(
                        "desc SD0PID_SEVNFRM.",
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
                    name: "soddfrm",
                    description: Some(
                        "desc SODDFRM.",
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
                    name: "epdis",
                    description: Some(
                        "desc EPDIS.",
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
                    name: "epena",
                    description: Some(
                        "desc EPENA.",
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
            name: "Diepctl3",
            extends: None,
            description: Some(
                "desc DIEPCTL3.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "mpsiz",
                    description: Some(
                        "desc MPSIZ.",
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
                    name: "usbaep",
                    description: Some(
                        "desc USBAEP.",
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
                    name: "eonum_dpid",
                    description: Some(
                        "desc EONUM_DPID.",
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
                    name: "naksts",
                    description: Some(
                        "desc NAKSTS.",
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
                    name: "eptyp",
                    description: Some(
                        "desc EPTYP.",
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
                    name: "stall",
                    description: Some(
                        "desc STALL.",
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
                    name: "txfnum",
                    description: Some(
                        "desc TXFNUM.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 22,
                        },
                    ),
                    bit_size: 4,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "cnak",
                    description: Some(
                        "desc CNAK.",
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
                    name: "snak",
                    description: Some(
                        "desc SNAK.",
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
                    name: "sd0pid_sevnfrm",
                    description: Some(
                        "desc SD0PID_SEVNFRM.",
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
                    name: "soddfrm",
                    description: Some(
                        "desc SODDFRM.",
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
                    name: "epdis",
                    description: Some(
                        "desc EPDIS.",
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
                    name: "epena",
                    description: Some(
                        "desc EPENA.",
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
            name: "Diepctl4",
            extends: None,
            description: Some(
                "desc DIEPCTL4.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "mpsiz",
                    description: Some(
                        "desc MPSIZ.",
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
                    name: "usbaep",
                    description: Some(
                        "desc USBAEP.",
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
                    name: "eonum_dpid",
                    description: Some(
                        "desc EONUM_DPID.",
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
                    name: "naksts",
                    description: Some(
                        "desc NAKSTS.",
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
                    name: "eptyp",
                    description: Some(
                        "desc EPTYP.",
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
                    name: "stall",
                    description: Some(
                        "desc STALL.",
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
                    name: "txfnum",
                    description: Some(
                        "desc TXFNUM.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 22,
                        },
                    ),
                    bit_size: 4,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "cnak",
                    description: Some(
                        "desc CNAK.",
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
                    name: "snak",
                    description: Some(
                        "desc SNAK.",
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
                    name: "sd0pid_sevnfrm",
                    description: Some(
                        "desc SD0PID_SEVNFRM.",
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
                    name: "soddfrm",
                    description: Some(
                        "desc SODDFRM.",
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
                    name: "epdis",
                    description: Some(
                        "desc EPDIS.",
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
                    name: "epena",
                    description: Some(
                        "desc EPENA.",
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
            name: "Diepempmsk",
            extends: None,
            description: Some(
                "desc DIEPEMPMSK.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "ineptxfem",
                    description: Some(
                        "desc INEPTXFEM.",
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
            name: "Diepint0",
            extends: None,
            description: Some(
                "desc DIEPINT0.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "xfrc",
                    description: Some(
                        "desc XFRC.",
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
                    name: "epdisd",
                    description: Some(
                        "desc EPDISD.",
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
                    name: "toc",
                    description: Some(
                        "desc TOC.",
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
                    name: "ttxfe",
                    description: Some(
                        "desc TTXFE.",
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
                    name: "nepne",
                    description: Some(
                        "desc NEPNE.",
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
                    name: "txfe",
                    description: Some(
                        "desc TXFE.",
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
            name: "Diepint1",
            extends: None,
            description: Some(
                "desc DIEPINT1.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "xfrc",
                    description: Some(
                        "desc XFRC.",
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
                    name: "epdisd",
                    description: Some(
                        "desc EPDISD.",
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
                    name: "toc",
                    description: Some(
                        "desc TOC.",
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
                    name: "ttxfe",
                    description: Some(
                        "desc TTXFE.",
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
                    name: "nepne",
                    description: Some(
                        "desc NEPNE.",
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
                    name: "txfe",
                    description: Some(
                        "desc TXFE.",
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
            name: "Diepint2",
            extends: None,
            description: Some(
                "desc DIEPINT2.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "xfrc",
                    description: Some(
                        "desc XFRC.",
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
                    name: "epdisd",
                    description: Some(
                        "desc EPDISD.",
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
                    name: "toc",
                    description: Some(
                        "desc TOC.",
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
                    name: "ttxfe",
                    description: Some(
                        "desc TTXFE.",
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
                    name: "nepne",
                    description: Some(
                        "desc NEPNE.",
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
                    name: "txfe",
                    description: Some(
                        "desc TXFE.",
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
            name: "Diepint3",
            extends: None,
            description: Some(
                "desc DIEPINT3.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "xfrc",
                    description: Some(
                        "desc XFRC.",
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
                    name: "epdisd",
                    description: Some(
                        "desc EPDISD.",
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
                    name: "toc",
                    description: Some(
                        "desc TOC.",
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
                    name: "ttxfe",
                    description: Some(
                        "desc TTXFE.",
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
                    name: "nepne",
                    description: Some(
                        "desc NEPNE.",
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
                    name: "txfe",
                    description: Some(
                        "desc TXFE.",
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
            name: "Diepint4",
            extends: None,
            description: Some(
                "desc DIEPINT4.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "xfrc",
                    description: Some(
                        "desc XFRC.",
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
                    name: "epdisd",
                    description: Some(
                        "desc EPDISD.",
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
                    name: "toc",
                    description: Some(
                        "desc TOC.",
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
                    name: "ttxfe",
                    description: Some(
                        "desc TTXFE.",
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
                    name: "nepne",
                    description: Some(
                        "desc NEPNE.",
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
                    name: "txfe",
                    description: Some(
                        "desc TXFE.",
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
            name: "Diepmsk",
            extends: None,
            description: Some(
                "desc DIEPMSK.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "xfrcm",
                    description: Some(
                        "desc XFRCM.",
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
                    name: "epdm",
                    description: Some(
                        "desc EPDM.",
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
                    name: "tom",
                    description: Some(
                        "desc TOM.",
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
                    name: "ittxfemsk",
                    description: Some(
                        "desc ITTXFEMSK.",
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
                    name: "inepnmm",
                    description: Some(
                        "desc INEPNMM.",
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
                    name: "iinepnem",
                    description: Some(
                        "desc IINEPNEM.",
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
            ],
        },
        FieldSet {
            name: "Dieptsiz0",
            extends: None,
            description: Some(
                "desc DIEPTSIZ0.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "xfrsiz",
                    description: Some(
                        "desc XFRSIZ.",
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
                Field {
                    name: "pktcnt",
                    description: Some(
                        "desc PKTCNT.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 19,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Dieptsiz1",
            extends: None,
            description: Some(
                "desc DIEPTSIZ1.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "xfrsiz",
                    description: Some(
                        "desc XFRSIZ.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 19,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "pktcnt",
                    description: Some(
                        "desc PKTCNT.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 19,
                        },
                    ),
                    bit_size: 10,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "mc",
                    description: Some(
                        "desc MC.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 29,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Dieptsiz2",
            extends: None,
            description: Some(
                "desc DIEPTSIZ2.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "xfrsiz",
                    description: Some(
                        "desc XFRSIZ.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 19,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "pktcnt",
                    description: Some(
                        "desc PKTCNT.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 19,
                        },
                    ),
                    bit_size: 10,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "mc",
                    description: Some(
                        "desc MC.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 29,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Dieptsiz3",
            extends: None,
            description: Some(
                "desc DIEPTSIZ3.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "xfrsiz",
                    description: Some(
                        "desc XFRSIZ.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 19,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "pktcnt",
                    description: Some(
                        "desc PKTCNT.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 19,
                        },
                    ),
                    bit_size: 10,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "mc",
                    description: Some(
                        "desc MC.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 29,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Dieptsiz4",
            extends: None,
            description: Some(
                "desc DIEPTSIZ4.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "xfrsiz",
                    description: Some(
                        "desc XFRSIZ.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 19,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "pktcnt",
                    description: Some(
                        "desc PKTCNT.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 19,
                        },
                    ),
                    bit_size: 10,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "mc",
                    description: Some(
                        "desc MC.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 29,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Dieptxf1",
            extends: None,
            description: Some(
                "desc DIEPTXF1.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "ineptxsa",
                    description: Some(
                        "desc INEPTXSA.",
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
                    name: "ineptxfd",
                    description: Some(
                        "desc INEPTXFD.",
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
            name: "Dieptxf2",
            extends: None,
            description: Some(
                "desc DIEPTXF2.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "ineptxsa",
                    description: Some(
                        "desc INEPTXSA.",
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
                    name: "ineptxfd",
                    description: Some(
                        "desc INEPTXFD.",
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
            name: "Dieptxf3",
            extends: None,
            description: Some(
                "desc DIEPTXF3.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "ineptxsa",
                    description: Some(
                        "desc INEPTXSA.",
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
                    name: "ineptxfd",
                    description: Some(
                        "desc INEPTXFD.",
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
            name: "Dieptxf4",
            extends: None,
            description: Some(
                "desc DIEPTXF4.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "ineptxsa",
                    description: Some(
                        "desc INEPTXSA.",
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
                    name: "ineptxfd",
                    description: Some(
                        "desc INEPTXFD.",
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
            name: "Doepctl0",
            extends: None,
            description: Some(
                "desc DOEPCTL0.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "mpsiz",
                    description: Some(
                        "desc MPSIZ.",
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
                    name: "usbaep",
                    description: Some(
                        "desc USBAEP.",
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
                    name: "naksts",
                    description: Some(
                        "desc NAKSTS.",
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
                    name: "eptyp",
                    description: Some(
                        "desc EPTYP.",
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
                    name: "snpm",
                    description: Some(
                        "desc SNPM.",
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
                    name: "stall",
                    description: Some(
                        "desc STALL.",
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
                    name: "cnak",
                    description: Some(
                        "desc CNAK.",
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
                    name: "snak",
                    description: Some(
                        "desc SNAK.",
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
                    name: "epdis",
                    description: Some(
                        "desc EPDIS.",
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
                    name: "epena",
                    description: Some(
                        "desc EPENA.",
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
            name: "Doepctl1",
            extends: None,
            description: Some(
                "desc DOEPCTL1.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "mpsiz",
                    description: Some(
                        "desc MPSIZ.",
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
                    name: "usbaep",
                    description: Some(
                        "desc USBAEP.",
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
                    name: "dpid",
                    description: Some(
                        "desc DPID.",
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
                    name: "naksts",
                    description: Some(
                        "desc NAKSTS.",
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
                    name: "eptyp",
                    description: Some(
                        "desc EPTYP.",
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
                    name: "snpm",
                    description: Some(
                        "desc SNPM.",
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
                    name: "stall",
                    description: Some(
                        "desc STALL.",
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
                    name: "cnak",
                    description: Some(
                        "desc CNAK.",
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
                    name: "snak",
                    description: Some(
                        "desc SNAK.",
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
                    name: "sd0pid",
                    description: Some(
                        "desc SD0PID.",
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
                    name: "sd1pid",
                    description: Some(
                        "desc SD1PID.",
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
                    name: "epdis",
                    description: Some(
                        "desc EPDIS.",
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
                    name: "epena",
                    description: Some(
                        "desc EPENA.",
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
            name: "Doepctl2",
            extends: None,
            description: Some(
                "desc DOEPCTL2.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "mpsiz",
                    description: Some(
                        "desc MPSIZ.",
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
                    name: "usbaep",
                    description: Some(
                        "desc USBAEP.",
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
                    name: "dpid",
                    description: Some(
                        "desc DPID.",
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
                    name: "naksts",
                    description: Some(
                        "desc NAKSTS.",
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
                    name: "eptyp",
                    description: Some(
                        "desc EPTYP.",
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
                    name: "snpm",
                    description: Some(
                        "desc SNPM.",
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
                    name: "stall",
                    description: Some(
                        "desc STALL.",
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
                    name: "cnak",
                    description: Some(
                        "desc CNAK.",
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
                    name: "snak",
                    description: Some(
                        "desc SNAK.",
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
                    name: "sd0pid",
                    description: Some(
                        "desc SD0PID.",
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
                    name: "sd1pid",
                    description: Some(
                        "desc SD1PID.",
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
                    name: "epdis",
                    description: Some(
                        "desc EPDIS.",
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
                    name: "epena",
                    description: Some(
                        "desc EPENA.",
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
            name: "Doepctl3",
            extends: None,
            description: Some(
                "desc DOEPCTL3.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "mpsiz",
                    description: Some(
                        "desc MPSIZ.",
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
                    name: "usbaep",
                    description: Some(
                        "desc USBAEP.",
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
                    name: "dpid",
                    description: Some(
                        "desc DPID.",
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
                    name: "naksts",
                    description: Some(
                        "desc NAKSTS.",
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
                    name: "eptyp",
                    description: Some(
                        "desc EPTYP.",
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
                    name: "snpm",
                    description: Some(
                        "desc SNPM.",
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
                    name: "stall",
                    description: Some(
                        "desc STALL.",
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
                    name: "cnak",
                    description: Some(
                        "desc CNAK.",
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
                    name: "snak",
                    description: Some(
                        "desc SNAK.",
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
                    name: "sd0pid",
                    description: Some(
                        "desc SD0PID.",
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
                    name: "sd1pid",
                    description: Some(
                        "desc SD1PID.",
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
                    name: "epdis",
                    description: Some(
                        "desc EPDIS.",
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
                    name: "epena",
                    description: Some(
                        "desc EPENA.",
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
            name: "Doepctl4",
            extends: None,
            description: Some(
                "desc DOEPCTL4.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "mpsiz",
                    description: Some(
                        "desc MPSIZ.",
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
                    name: "usbaep",
                    description: Some(
                        "desc USBAEP.",
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
                    name: "dpid",
                    description: Some(
                        "desc DPID.",
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
                    name: "naksts",
                    description: Some(
                        "desc NAKSTS.",
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
                    name: "eptyp",
                    description: Some(
                        "desc EPTYP.",
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
                    name: "snpm",
                    description: Some(
                        "desc SNPM.",
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
                    name: "stall",
                    description: Some(
                        "desc STALL.",
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
                    name: "cnak",
                    description: Some(
                        "desc CNAK.",
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
                    name: "snak",
                    description: Some(
                        "desc SNAK.",
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
                    name: "sd0pid",
                    description: Some(
                        "desc SD0PID.",
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
                    name: "sd1pid",
                    description: Some(
                        "desc SD1PID.",
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
                    name: "epdis",
                    description: Some(
                        "desc EPDIS.",
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
                    name: "epena",
                    description: Some(
                        "desc EPENA.",
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
            name: "Doepint0",
            extends: None,
            description: Some(
                "desc DOEPINT0.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "xfrc",
                    description: Some(
                        "desc XFRC.",
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
                    name: "epdisd",
                    description: Some(
                        "desc EPDISD.",
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
                    name: "stup",
                    description: Some(
                        "desc STUP.",
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
                    name: "otepdis",
                    description: Some(
                        "desc OTEPDIS.",
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
                    name: "b2bstup",
                    description: Some(
                        "desc B2BSTUP.",
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
            ],
        },
        FieldSet {
            name: "Doepint1",
            extends: None,
            description: Some(
                "desc DOEPINT1.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "xfrc",
                    description: Some(
                        "desc XFRC.",
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
                    name: "epdisd",
                    description: Some(
                        "desc EPDISD.",
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
                    name: "stup",
                    description: Some(
                        "desc STUP.",
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
                    name: "otepdis",
                    description: Some(
                        "desc OTEPDIS.",
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
                    name: "b2bstup",
                    description: Some(
                        "desc B2BSTUP.",
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
            ],
        },
        FieldSet {
            name: "Doepint2",
            extends: None,
            description: Some(
                "desc DOEPINT2.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "xfrc",
                    description: Some(
                        "desc XFRC.",
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
                    name: "epdisd",
                    description: Some(
                        "desc EPDISD.",
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
                    name: "stup",
                    description: Some(
                        "desc STUP.",
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
                    name: "otepdis",
                    description: Some(
                        "desc OTEPDIS.",
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
                    name: "b2bstup",
                    description: Some(
                        "desc B2BSTUP.",
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
            ],
        },
        FieldSet {
            name: "Doepint3",
            extends: None,
            description: Some(
                "desc DOEPINT3.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "xfrc",
                    description: Some(
                        "desc XFRC.",
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
                    name: "epdisd",
                    description: Some(
                        "desc EPDISD.",
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
                    name: "stup",
                    description: Some(
                        "desc STUP.",
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
                    name: "otepdis",
                    description: Some(
                        "desc OTEPDIS.",
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
                    name: "b2bstup",
                    description: Some(
                        "desc B2BSTUP.",
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
            ],
        },
        FieldSet {
            name: "Doepint4",
            extends: None,
            description: Some(
                "desc DOEPINT4.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "xfrc",
                    description: Some(
                        "desc XFRC.",
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
                    name: "epdisd",
                    description: Some(
                        "desc EPDISD.",
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
                    name: "stup",
                    description: Some(
                        "desc STUP.",
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
                    name: "otepdis",
                    description: Some(
                        "desc OTEPDIS.",
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
                    name: "b2bstup",
                    description: Some(
                        "desc B2BSTUP.",
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
            ],
        },
        FieldSet {
            name: "Doepmsk",
            extends: None,
            description: Some(
                "desc DOEPMSK.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "xfrcm",
                    description: Some(
                        "desc XFRCM.",
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
                    name: "epdm",
                    description: Some(
                        "desc EPDM.",
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
                    name: "stupm",
                    description: Some(
                        "desc STUPM.",
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
                    name: "otepdm",
                    description: Some(
                        "desc OTEPDM.",
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
            ],
        },
        FieldSet {
            name: "Doeptsiz0",
            extends: None,
            description: Some(
                "desc DOEPTSIZ0.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "xfrsiz",
                    description: Some(
                        "desc XFRSIZ.",
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
                Field {
                    name: "pktcnt",
                    description: Some(
                        "desc PKTCNT.",
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
                    name: "stupcnt",
                    description: Some(
                        "desc STUPCNT.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 29,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Doeptsiz1",
            extends: None,
            description: Some(
                "desc DOEPTSIZ1.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "xfrsiz",
                    description: Some(
                        "desc XFRSIZ.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 19,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "pktcnt",
                    description: Some(
                        "desc PKTCNT.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 19,
                        },
                    ),
                    bit_size: 10,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Doeptsiz2",
            extends: None,
            description: Some(
                "desc DOEPTSIZ2.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "xfrsiz",
                    description: Some(
                        "desc XFRSIZ.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 19,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "pktcnt",
                    description: Some(
                        "desc PKTCNT.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 19,
                        },
                    ),
                    bit_size: 10,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Doeptsiz3",
            extends: None,
            description: Some(
                "desc DOEPTSIZ3.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "xfrsiz",
                    description: Some(
                        "desc XFRSIZ.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 19,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "pktcnt",
                    description: Some(
                        "desc PKTCNT.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 19,
                        },
                    ),
                    bit_size: 10,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Doeptsiz4",
            extends: None,
            description: Some(
                "desc DOEPTSIZ4.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "xfrsiz",
                    description: Some(
                        "desc XFRSIZ.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 19,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "pktcnt",
                    description: Some(
                        "desc PKTCNT.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 19,
                        },
                    ),
                    bit_size: 10,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Dsts",
            extends: None,
            description: Some(
                "desc DSTS.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "suspsts",
                    description: Some(
                        "desc SUSPSTS.",
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
                    name: "enumspd",
                    description: Some(
                        "desc ENUMSPD.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 1,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "eerr",
                    description: Some(
                        "desc EERR.",
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
                    name: "fnsof",
                    description: Some(
                        "desc FNSOF.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 8,
                        },
                    ),
                    bit_size: 14,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Dtxfsts0",
            extends: None,
            description: Some(
                "desc DTXFSTS0.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "ineptfsav",
                    description: Some(
                        "desc INEPTFSAV.",
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
            name: "Dtxfsts1",
            extends: None,
            description: Some(
                "desc DTXFSTS1.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "ineptfsav",
                    description: Some(
                        "desc INEPTFSAV.",
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
            name: "Dtxfsts2",
            extends: None,
            description: Some(
                "desc DTXFSTS2.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "ineptfsav",
                    description: Some(
                        "desc INEPTFSAV.",
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
            name: "Dtxfsts3",
            extends: None,
            description: Some(
                "desc DTXFSTS3.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "ineptfsav",
                    description: Some(
                        "desc INEPTFSAV.",
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
            name: "Dtxfsts4",
            extends: None,
            description: Some(
                "desc DTXFSTS4.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "ineptfsav",
                    description: Some(
                        "desc INEPTFSAV.",
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
            name: "Gahbcfg",
            extends: None,
            description: Some(
                "desc GAHBCFG.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "gintmsk",
                    description: Some(
                        "desc GINTMSK.",
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
                    name: "hbstlen",
                    description: Some(
                        "desc HBSTLEN.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 1,
                        },
                    ),
                    bit_size: 4,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "dmaen",
                    description: Some(
                        "desc DMAEN.",
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
            name: "Gintmsk",
            extends: None,
            description: Some(
                "desc GINTMSK.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "sofm",
                    description: Some(
                        "desc SOFM.",
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
                    name: "rxfnem",
                    description: Some(
                        "desc RXFNEM.",
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
                    name: "ginakeffm",
                    description: Some(
                        "desc GINAKEFFM.",
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
                    name: "goutnakeffm",
                    description: Some(
                        "desc GOUTNAKEFFM.",
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
                    name: "esuspm",
                    description: Some(
                        "desc ESUSPM.",
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
                    name: "usbsuspm",
                    description: Some(
                        "desc USBSUSPM.",
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
                    name: "usbrstm",
                    description: Some(
                        "desc USBRSTM.",
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
                    name: "enumdnem",
                    description: Some(
                        "desc ENUMDNEM.",
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
                    name: "isoodrpm",
                    description: Some(
                        "desc ISOODRPM.",
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
                    name: "eopfm",
                    description: Some(
                        "desc EOPFM.",
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
                    name: "iepim",
                    description: Some(
                        "desc IEPIM.",
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
                    name: "oepim",
                    description: Some(
                        "desc OEPIM.",
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
                    name: "iisoixfrm",
                    description: Some(
                        "desc IISOIXFRM.",
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
                    name: "ipxfrm_incompisooutm",
                    description: Some(
                        "desc IPXFRM_INCOMPISOOUTM.",
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
                    name: "datafsuspm",
                    description: Some(
                        "desc DATAFSUSPM.",
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
                    name: "vbusvim",
                    description: Some(
                        "desc VBUSVIM.",
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
                    name: "wkuim",
                    description: Some(
                        "desc WKUIM.",
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
            name: "Gintsts",
            extends: None,
            description: Some(
                "desc GINTSTS.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "sof",
                    description: Some(
                        "desc SOF.",
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
                    name: "rxflne",
                    description: Some(
                        "desc RXFLNE.",
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
                    name: "nptxfe",
                    description: Some(
                        "desc NPTXFE.",
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
                    name: "ginakeff",
                    description: Some(
                        "desc GINAKEFF.",
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
                    name: "goutnakeff",
                    description: Some(
                        "desc GOUTNAKEFF.",
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
                    name: "esusp",
                    description: Some(
                        "desc ESUSP.",
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
                    name: "usbsusp",
                    description: Some(
                        "desc USBSUSP.",
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
                    name: "usbrst",
                    description: Some(
                        "desc USBRST.",
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
                    name: "enumdne",
                    description: Some(
                        "desc ENUMDNE.",
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
                    name: "isoodrp",
                    description: Some(
                        "desc ISOODRP.",
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
                    name: "eopf",
                    description: Some(
                        "desc EOPF.",
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
                    name: "iepint",
                    description: Some(
                        "desc IEPINT.",
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
                    name: "oepint",
                    description: Some(
                        "desc OEPINT.",
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
                    name: "iisoixfr",
                    description: Some(
                        "desc IISOIXFR.",
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
                    name: "ipxfr_incompiso",
                    description: Some(
                        "desc IPXFR_INCOMPISO.",
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
                    name: "datafsusp",
                    description: Some(
                        "desc DATAFSUSP.",
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
                    name: "vbusvint",
                    description: Some(
                        "desc VBUSVINT.",
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
                    name: "wkuint",
                    description: Some(
                        "desc WKUINT.",
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
            name: "Grstctl",
            extends: None,
            description: Some(
                "desc GRSTCTL.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "csrst",
                    description: Some(
                        "desc CSRST.",
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
                    name: "hsrst",
                    description: Some(
                        "desc HSRST.",
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
                    name: "rxfflsh",
                    description: Some(
                        "desc RXFFLSH.",
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
                    name: "txfflsh",
                    description: Some(
                        "desc TXFFLSH.",
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
                    name: "txfnum",
                    description: Some(
                        "desc TXFNUM.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 6,
                        },
                    ),
                    bit_size: 5,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "dmareq",
                    description: Some(
                        "desc DMAREQ.",
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
                    name: "ahbidl",
                    description: Some(
                        "desc AHBIDL.",
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
            name: "Grxfsiz",
            extends: None,
            description: Some(
                "desc GRXFSIZ.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "rxfd",
                    description: Some(
                        "desc RXFD.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 11,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Grxstsp",
            extends: None,
            description: Some(
                "desc GRXSTSP.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "chnum_epnum",
                    description: Some(
                        "desc CHNUM_EPNUM.",
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
                    name: "bcnt",
                    description: Some(
                        "desc BCNT.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 4,
                        },
                    ),
                    bit_size: 11,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "dpid",
                    description: Some(
                        "desc DPID.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 15,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "pktsts",
                    description: Some(
                        "desc PKTSTS.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 17,
                        },
                    ),
                    bit_size: 4,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Grxstsr",
            extends: None,
            description: Some(
                "desc GRXSTSR.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "chnum_epnum",
                    description: Some(
                        "desc CHNUM_EPNUM.",
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
                    name: "bcnt",
                    description: Some(
                        "desc BCNT.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 4,
                        },
                    ),
                    bit_size: 11,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "dpid",
                    description: Some(
                        "desc DPID.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 15,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "pktsts",
                    description: Some(
                        "desc PKTSTS.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 17,
                        },
                    ),
                    bit_size: 4,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Gusbcfg",
            extends: None,
            description: Some(
                "desc GUSBCFG.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "tocal",
                    description: Some(
                        "desc TOCAL.",
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
                    name: "physel",
                    description: Some(
                        "desc PHYSEL.",
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
                    name: "trdt",
                    description: Some(
                        "desc TRDT.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 10,
                        },
                    ),
                    bit_size: 4,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Pcgcctl",
            extends: None,
            description: Some(
                "desc PCGCCTL.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "stppclk",
                    description: Some(
                        "desc STPPCLK.",
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
                    name: "gatehclk",
                    description: Some(
                        "desc GATEHCLK.",
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
            ],
        },
    ],
    enums: &[],
};
                