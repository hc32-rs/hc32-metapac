
                    use crate::metadata::ir::*;
                    pub(crate) static REGISTERS: IR = IR {
    blocks: &[
        Block {
            name: "Adc",
            extends: None,
            description: Some(
                "desc ADC1.",
            ),
            items: &[
                BlockItem {
                    name: "str",
                    description: Some(
                        "desc STR.",
                    ),
                    array: None,
                    byte_offset: 0x0,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 8,
                            fieldset: Some(
                                "Str",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "cr0",
                    description: Some(
                        "desc CR0.",
                    ),
                    array: None,
                    byte_offset: 0x2,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 16,
                            fieldset: Some(
                                "Cr0",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "cr1",
                    description: Some(
                        "desc CR1.",
                    ),
                    array: None,
                    byte_offset: 0x4,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 16,
                            fieldset: Some(
                                "Cr1",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "cr2",
                    description: Some(
                        "desc CR2.",
                    ),
                    array: None,
                    byte_offset: 0x6,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 16,
                            fieldset: Some(
                                "Cr2",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "chselra",
                    description: Some(
                        "desc CHSELRA.",
                    ),
                    array: None,
                    byte_offset: 0xc,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Chselra",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "chselrb",
                    description: Some(
                        "desc CHSELRB.",
                    ),
                    array: None,
                    byte_offset: 0x10,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Chselrb",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "avchselr",
                    description: Some(
                        "desc AVCHSELR.",
                    ),
                    array: None,
                    byte_offset: 0x14,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Avchselr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "exchselr",
                    description: Some(
                        "desc EXCHSELR.",
                    ),
                    array: None,
                    byte_offset: 0x18,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 8,
                            fieldset: Some(
                                "Exchselr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "trgsr",
                    description: Some(
                        "desc TRGSR.",
                    ),
                    array: None,
                    byte_offset: 0x1c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Trgsr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "sstr0",
                    description: Some(
                        "desc SSTR0.",
                    ),
                    array: None,
                    byte_offset: 0x20,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 8,
                            fieldset: None,
                        },
                    ),
                },
                BlockItem {
                    name: "sstr1",
                    description: Some(
                        "desc SSTR1.",
                    ),
                    array: None,
                    byte_offset: 0x21,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 8,
                            fieldset: None,
                        },
                    ),
                },
                BlockItem {
                    name: "sstr2",
                    description: Some(
                        "desc SSTR2.",
                    ),
                    array: None,
                    byte_offset: 0x22,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 8,
                            fieldset: None,
                        },
                    ),
                },
                BlockItem {
                    name: "sstr3",
                    description: Some(
                        "desc SSTR3.",
                    ),
                    array: None,
                    byte_offset: 0x23,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 8,
                            fieldset: None,
                        },
                    ),
                },
                BlockItem {
                    name: "sstr4",
                    description: Some(
                        "desc SSTR4.",
                    ),
                    array: None,
                    byte_offset: 0x24,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 8,
                            fieldset: None,
                        },
                    ),
                },
                BlockItem {
                    name: "sstr5",
                    description: Some(
                        "desc SSTR5.",
                    ),
                    array: None,
                    byte_offset: 0x25,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 8,
                            fieldset: None,
                        },
                    ),
                },
                BlockItem {
                    name: "sstr6",
                    description: Some(
                        "desc SSTR6.",
                    ),
                    array: None,
                    byte_offset: 0x26,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 8,
                            fieldset: None,
                        },
                    ),
                },
                BlockItem {
                    name: "sstr7",
                    description: Some(
                        "desc SSTR7.",
                    ),
                    array: None,
                    byte_offset: 0x27,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 8,
                            fieldset: None,
                        },
                    ),
                },
                BlockItem {
                    name: "sstr8",
                    description: Some(
                        "desc SSTR8.",
                    ),
                    array: None,
                    byte_offset: 0x28,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 8,
                            fieldset: None,
                        },
                    ),
                },
                BlockItem {
                    name: "sstr9",
                    description: Some(
                        "desc SSTR9.",
                    ),
                    array: None,
                    byte_offset: 0x29,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 8,
                            fieldset: None,
                        },
                    ),
                },
                BlockItem {
                    name: "sstr10",
                    description: Some(
                        "desc SSTR10.",
                    ),
                    array: None,
                    byte_offset: 0x2a,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 8,
                            fieldset: None,
                        },
                    ),
                },
                BlockItem {
                    name: "sstr11",
                    description: Some(
                        "desc SSTR11.",
                    ),
                    array: None,
                    byte_offset: 0x2b,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 8,
                            fieldset: None,
                        },
                    ),
                },
                BlockItem {
                    name: "sstr12",
                    description: Some(
                        "desc SSTR12.",
                    ),
                    array: None,
                    byte_offset: 0x2c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 8,
                            fieldset: None,
                        },
                    ),
                },
                BlockItem {
                    name: "sstr13",
                    description: Some(
                        "desc SSTR13.",
                    ),
                    array: None,
                    byte_offset: 0x2d,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 8,
                            fieldset: None,
                        },
                    ),
                },
                BlockItem {
                    name: "sstr14",
                    description: Some(
                        "desc SSTR14.",
                    ),
                    array: None,
                    byte_offset: 0x2e,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 8,
                            fieldset: None,
                        },
                    ),
                },
                BlockItem {
                    name: "sstr15",
                    description: Some(
                        "desc SSTR15.",
                    ),
                    array: None,
                    byte_offset: 0x2f,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 8,
                            fieldset: None,
                        },
                    ),
                },
                BlockItem {
                    name: "chmuxr0",
                    description: Some(
                        "desc CHMUXR0.",
                    ),
                    array: None,
                    byte_offset: 0x38,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 16,
                            fieldset: Some(
                                "Chmuxr0",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "chmuxr1",
                    description: Some(
                        "desc CHMUXR1.",
                    ),
                    array: None,
                    byte_offset: 0x3a,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 16,
                            fieldset: Some(
                                "Chmuxr1",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "chmuxr2",
                    description: Some(
                        "desc CHMUXR2.",
                    ),
                    array: None,
                    byte_offset: 0x3c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 16,
                            fieldset: Some(
                                "Chmuxr2",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "chmuxr3",
                    description: Some(
                        "desc CHMUXR3.",
                    ),
                    array: None,
                    byte_offset: 0x3e,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 16,
                            fieldset: Some(
                                "Chmuxr3",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "chselrc",
                    description: Some(
                        "desc CHSELRC.",
                    ),
                    array: None,
                    byte_offset: 0x40,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Chselrc",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "isr",
                    description: Some(
                        "desc ISR.",
                    ),
                    array: None,
                    byte_offset: 0x44,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::Read,
                            bit_size: 8,
                            fieldset: Some(
                                "Isr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "icr",
                    description: Some(
                        "desc ICR.",
                    ),
                    array: None,
                    byte_offset: 0x45,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 8,
                            fieldset: Some(
                                "Icr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "isclrr",
                    description: Some(
                        "desc ISCLRR.",
                    ),
                    array: None,
                    byte_offset: 0x46,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::Write,
                            bit_size: 8,
                            fieldset: Some(
                                "Isclrr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "dr0",
                    description: Some(
                        "desc DR0.",
                    ),
                    array: None,
                    byte_offset: 0x50,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::Read,
                            bit_size: 16,
                            fieldset: None,
                        },
                    ),
                },
                BlockItem {
                    name: "dr1",
                    description: Some(
                        "desc DR1.",
                    ),
                    array: None,
                    byte_offset: 0x52,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::Read,
                            bit_size: 16,
                            fieldset: None,
                        },
                    ),
                },
                BlockItem {
                    name: "dr2",
                    description: Some(
                        "desc DR2.",
                    ),
                    array: None,
                    byte_offset: 0x54,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::Read,
                            bit_size: 16,
                            fieldset: None,
                        },
                    ),
                },
                BlockItem {
                    name: "dr3",
                    description: Some(
                        "desc DR3.",
                    ),
                    array: None,
                    byte_offset: 0x56,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::Read,
                            bit_size: 16,
                            fieldset: None,
                        },
                    ),
                },
                BlockItem {
                    name: "dr4",
                    description: Some(
                        "desc DR4.",
                    ),
                    array: None,
                    byte_offset: 0x58,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::Read,
                            bit_size: 16,
                            fieldset: None,
                        },
                    ),
                },
                BlockItem {
                    name: "dr5",
                    description: Some(
                        "desc DR5.",
                    ),
                    array: None,
                    byte_offset: 0x5a,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::Read,
                            bit_size: 16,
                            fieldset: None,
                        },
                    ),
                },
                BlockItem {
                    name: "dr6",
                    description: Some(
                        "desc DR6.",
                    ),
                    array: None,
                    byte_offset: 0x5c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::Read,
                            bit_size: 16,
                            fieldset: None,
                        },
                    ),
                },
                BlockItem {
                    name: "dr7",
                    description: Some(
                        "desc DR7.",
                    ),
                    array: None,
                    byte_offset: 0x5e,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::Read,
                            bit_size: 16,
                            fieldset: None,
                        },
                    ),
                },
                BlockItem {
                    name: "dr8",
                    description: Some(
                        "desc DR8.",
                    ),
                    array: None,
                    byte_offset: 0x60,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::Read,
                            bit_size: 16,
                            fieldset: None,
                        },
                    ),
                },
                BlockItem {
                    name: "dr9",
                    description: Some(
                        "desc DR9.",
                    ),
                    array: None,
                    byte_offset: 0x62,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::Read,
                            bit_size: 16,
                            fieldset: None,
                        },
                    ),
                },
                BlockItem {
                    name: "dr10",
                    description: Some(
                        "desc DR10.",
                    ),
                    array: None,
                    byte_offset: 0x64,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::Read,
                            bit_size: 16,
                            fieldset: None,
                        },
                    ),
                },
                BlockItem {
                    name: "dr11",
                    description: Some(
                        "desc DR11.",
                    ),
                    array: None,
                    byte_offset: 0x66,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::Read,
                            bit_size: 16,
                            fieldset: None,
                        },
                    ),
                },
                BlockItem {
                    name: "dr12",
                    description: Some(
                        "desc DR12.",
                    ),
                    array: None,
                    byte_offset: 0x68,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::Read,
                            bit_size: 16,
                            fieldset: None,
                        },
                    ),
                },
                BlockItem {
                    name: "dr13",
                    description: Some(
                        "desc DR13.",
                    ),
                    array: None,
                    byte_offset: 0x6a,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::Read,
                            bit_size: 16,
                            fieldset: None,
                        },
                    ),
                },
                BlockItem {
                    name: "dr14",
                    description: Some(
                        "desc DR14.",
                    ),
                    array: None,
                    byte_offset: 0x6c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::Read,
                            bit_size: 16,
                            fieldset: None,
                        },
                    ),
                },
                BlockItem {
                    name: "dr15",
                    description: Some(
                        "desc DR15.",
                    ),
                    array: None,
                    byte_offset: 0x6e,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::Read,
                            bit_size: 16,
                            fieldset: None,
                        },
                    ),
                },
                BlockItem {
                    name: "awdcr",
                    description: Some(
                        "desc AWDCR.",
                    ),
                    array: None,
                    byte_offset: 0xa0,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 16,
                            fieldset: Some(
                                "Awdcr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "awdsr",
                    description: Some(
                        "desc AWDSR.",
                    ),
                    array: None,
                    byte_offset: 0xa2,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::Read,
                            bit_size: 8,
                            fieldset: Some(
                                "Awdsr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "awdsclrr",
                    description: Some(
                        "desc AWDSCLRR.",
                    ),
                    array: None,
                    byte_offset: 0xa3,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::Write,
                            bit_size: 8,
                            fieldset: Some(
                                "Awdsclrr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "awd0dr0",
                    description: Some(
                        "desc AWD0DR0.",
                    ),
                    array: None,
                    byte_offset: 0xa4,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 16,
                            fieldset: None,
                        },
                    ),
                },
                BlockItem {
                    name: "awd0dr1",
                    description: Some(
                        "desc AWD0DR1.",
                    ),
                    array: None,
                    byte_offset: 0xa6,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 16,
                            fieldset: None,
                        },
                    ),
                },
                BlockItem {
                    name: "awd0chsr",
                    description: Some(
                        "desc AWD0CHSR.",
                    ),
                    array: None,
                    byte_offset: 0xa8,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 8,
                            fieldset: Some(
                                "Awd0chsr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "awd1dr0",
                    description: Some(
                        "desc AWD1DR0.",
                    ),
                    array: None,
                    byte_offset: 0xac,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 16,
                            fieldset: None,
                        },
                    ),
                },
                BlockItem {
                    name: "awd1dr1",
                    description: Some(
                        "desc AWD1DR1.",
                    ),
                    array: None,
                    byte_offset: 0xae,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 16,
                            fieldset: None,
                        },
                    ),
                },
                BlockItem {
                    name: "awd1chsr",
                    description: Some(
                        "desc AWD1CHSR.",
                    ),
                    array: None,
                    byte_offset: 0xb0,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 8,
                            fieldset: Some(
                                "Awd1chsr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "opacr",
                    description: Some(
                        "desc OPACR.",
                    ),
                    array: None,
                    byte_offset: 0xc0,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Opacr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "dir0",
                    description: Some(
                        "desc DIR0.",
                    ),
                    array: None,
                    byte_offset: 0x100,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::Read,
                            bit_size: 32,
                            fieldset: Some(
                                "Dir0",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "dir1",
                    description: Some(
                        "desc DIR1.",
                    ),
                    array: None,
                    byte_offset: 0x104,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::Read,
                            bit_size: 32,
                            fieldset: Some(
                                "Dir1",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "dir2",
                    description: Some(
                        "desc DIR2.",
                    ),
                    array: None,
                    byte_offset: 0x108,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::Read,
                            bit_size: 32,
                            fieldset: Some(
                                "Dir2",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "dir3",
                    description: Some(
                        "desc DIR3.",
                    ),
                    array: None,
                    byte_offset: 0x10c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::Read,
                            bit_size: 32,
                            fieldset: Some(
                                "Dir3",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "dir4",
                    description: Some(
                        "desc DIR4.",
                    ),
                    array: None,
                    byte_offset: 0x110,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::Read,
                            bit_size: 32,
                            fieldset: Some(
                                "Dir4",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "dir5",
                    description: Some(
                        "desc DIR5.",
                    ),
                    array: None,
                    byte_offset: 0x114,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::Read,
                            bit_size: 32,
                            fieldset: Some(
                                "Dir5",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "dir6",
                    description: Some(
                        "desc DIR6.",
                    ),
                    array: None,
                    byte_offset: 0x118,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::Read,
                            bit_size: 32,
                            fieldset: Some(
                                "Dir6",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "dir7",
                    description: Some(
                        "desc DIR7.",
                    ),
                    array: None,
                    byte_offset: 0x11c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::Read,
                            bit_size: 32,
                            fieldset: Some(
                                "Dir7",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "dir8",
                    description: Some(
                        "desc DIR8.",
                    ),
                    array: None,
                    byte_offset: 0x120,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::Read,
                            bit_size: 32,
                            fieldset: Some(
                                "Dir8",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "dir9",
                    description: Some(
                        "desc DIR9.",
                    ),
                    array: None,
                    byte_offset: 0x124,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::Read,
                            bit_size: 32,
                            fieldset: Some(
                                "Dir9",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "dir10",
                    description: Some(
                        "desc DIR10.",
                    ),
                    array: None,
                    byte_offset: 0x128,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::Read,
                            bit_size: 32,
                            fieldset: Some(
                                "Dir10",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "dir11",
                    description: Some(
                        "desc DIR11.",
                    ),
                    array: None,
                    byte_offset: 0x12c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::Read,
                            bit_size: 32,
                            fieldset: Some(
                                "Dir11",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "dir12",
                    description: Some(
                        "desc DIR12.",
                    ),
                    array: None,
                    byte_offset: 0x130,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::Read,
                            bit_size: 32,
                            fieldset: Some(
                                "Dir12",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "dir13",
                    description: Some(
                        "desc DIR13.",
                    ),
                    array: None,
                    byte_offset: 0x134,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::Read,
                            bit_size: 32,
                            fieldset: Some(
                                "Dir13",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "dir14",
                    description: Some(
                        "desc DIR14.",
                    ),
                    array: None,
                    byte_offset: 0x138,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::Read,
                            bit_size: 32,
                            fieldset: Some(
                                "Dir14",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "dir15",
                    description: Some(
                        "desc DIR15.",
                    ),
                    array: None,
                    byte_offset: 0x13c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::Read,
                            bit_size: 32,
                            fieldset: Some(
                                "Dir15",
                            ),
                        },
                    ),
                },
            ],
        },
    ],
    fieldsets: &[
        FieldSet {
            name: "Avchselr",
            extends: None,
            description: Some(
                "desc AVCHSELR.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "avchsel",
                    description: Some(
                        "desc AVCHSEL.",
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
            name: "Awd0chsr",
            extends: None,
            description: Some(
                "desc AWD0CHSR.",
            ),
            bit_size: 8,
            fields: &[
                Field {
                    name: "awdch",
                    description: Some(
                        "desc AWDCH.",
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
            ],
        },
        FieldSet {
            name: "Awd1chsr",
            extends: None,
            description: Some(
                "desc AWD1CHSR.",
            ),
            bit_size: 8,
            fields: &[
                Field {
                    name: "awdch",
                    description: Some(
                        "desc AWDCH.",
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
            ],
        },
        FieldSet {
            name: "Awdcr",
            extends: None,
            description: Some(
                "desc AWDCR.",
            ),
            bit_size: 16,
            fields: &[
                Field {
                    name: "awd0en",
                    description: Some(
                        "desc AWD0EN.",
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
                    name: "awd0ien",
                    description: Some(
                        "desc AWD0IEN.",
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
                    name: "awd0md",
                    description: Some(
                        "desc AWD0MD.",
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
                    name: "awd1en",
                    description: Some(
                        "desc AWD1EN.",
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
                    name: "awd1ien",
                    description: Some(
                        "desc AWD1IEN.",
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
                    name: "awd1md",
                    description: Some(
                        "desc AWD1MD.",
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
                    name: "awdcm",
                    description: Some(
                        "desc AWDCM.",
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
            ],
        },
        FieldSet {
            name: "Awdsclrr",
            extends: None,
            description: Some(
                "desc AWDSCLRR.",
            ),
            bit_size: 8,
            fields: &[
                Field {
                    name: "clrawd0f",
                    description: Some(
                        "desc CLRAWD0F.",
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
                    name: "clrawd1f",
                    description: Some(
                        "desc CLRAWD1F.",
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
                    name: "clrawdcmf",
                    description: Some(
                        "desc CLRAWDCMF.",
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
            name: "Awdsr",
            extends: None,
            description: Some(
                "desc AWDSR.",
            ),
            bit_size: 8,
            fields: &[
                Field {
                    name: "awd0f",
                    description: Some(
                        "desc AWD0F.",
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
                    name: "awd1f",
                    description: Some(
                        "desc AWD1F.",
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
                    name: "awdcmf",
                    description: Some(
                        "desc AWDCMF.",
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
            name: "Chmuxr0",
            extends: None,
            description: Some(
                "desc CHMUXR0.",
            ),
            bit_size: 16,
            fields: &[
                Field {
                    name: "ch00mux",
                    description: Some(
                        "desc CH00MUX.",
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
                    name: "ch01mux",
                    description: Some(
                        "desc CH01MUX.",
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
                Field {
                    name: "ch02mux",
                    description: Some(
                        "desc CH02MUX.",
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
                    name: "ch03mux",
                    description: Some(
                        "desc CH03MUX.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 12,
                        },
                    ),
                    bit_size: 4,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Chmuxr1",
            extends: None,
            description: Some(
                "desc CHMUXR1.",
            ),
            bit_size: 16,
            fields: &[
                Field {
                    name: "ch04mux",
                    description: Some(
                        "desc CH04MUX.",
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
                    name: "ch05mux",
                    description: Some(
                        "desc CH05MUX.",
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
                Field {
                    name: "ch06mux",
                    description: Some(
                        "desc CH06MUX.",
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
                    name: "ch07mux",
                    description: Some(
                        "desc CH07MUX.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 12,
                        },
                    ),
                    bit_size: 4,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Chmuxr2",
            extends: None,
            description: Some(
                "desc CHMUXR2.",
            ),
            bit_size: 16,
            fields: &[
                Field {
                    name: "ch08mux",
                    description: Some(
                        "desc CH08MUX.",
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
                    name: "ch09mux",
                    description: Some(
                        "desc CH09MUX.",
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
                Field {
                    name: "ch10mux",
                    description: Some(
                        "desc CH10MUX.",
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
                    name: "ch11mux",
                    description: Some(
                        "desc CH11MUX.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 12,
                        },
                    ),
                    bit_size: 4,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Chmuxr3",
            extends: None,
            description: Some(
                "desc CHMUXR3.",
            ),
            bit_size: 16,
            fields: &[
                Field {
                    name: "ch12mux",
                    description: Some(
                        "desc CH12MUX.",
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
                    name: "ch13mux",
                    description: Some(
                        "desc CH13MUX.",
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
                Field {
                    name: "ch14mux",
                    description: Some(
                        "desc CH14MUX.",
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
                    name: "ch15mux",
                    description: Some(
                        "desc CH15MUX.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 12,
                        },
                    ),
                    bit_size: 4,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Chselra",
            extends: None,
            description: Some(
                "desc CHSELRA.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "chsela",
                    description: Some(
                        "desc CHSELA.",
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
            name: "Chselrb",
            extends: None,
            description: Some(
                "desc CHSELRB.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "chselb",
                    description: Some(
                        "desc CHSELB.",
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
            name: "Chselrc",
            extends: None,
            description: Some(
                "desc CHSELRC.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "chselc",
                    description: Some(
                        "desc CHSELC.",
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
            name: "Cr0",
            extends: None,
            description: Some(
                "desc CR0.",
            ),
            bit_size: 16,
            fields: &[
                Field {
                    name: "ms",
                    description: Some(
                        "desc MS.",
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
                    name: "clren",
                    description: Some(
                        "desc CLREN.",
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
                    name: "dfmt",
                    description: Some(
                        "desc DFMT.",
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
                    name: "avcnt",
                    description: Some(
                        "desc AVCNT.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 8,
                        },
                    ),
                    bit_size: 3,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Cr1",
            extends: None,
            description: Some(
                "desc CR1.",
            ),
            bit_size: 16,
            fields: &[
                Field {
                    name: "rschsel",
                    description: Some(
                        "desc RSCHSEL.",
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
            name: "Cr2",
            extends: None,
            description: Some(
                "desc CR2.",
            ),
            bit_size: 16,
            fields: &[
                Field {
                    name: "ovss",
                    description: Some(
                        "desc OVSS.",
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
                    name: "ovsmod",
                    description: Some(
                        "desc OVSMOD.",
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
            name: "Dir0",
            extends: None,
            description: Some(
                "desc DIR0.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "ad",
                    description: Some(
                        "desc AD.",
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
                    name: "pchs",
                    description: Some(
                        "desc PCHS.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 16,
                        },
                    ),
                    bit_size: 4,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Dir1",
            extends: None,
            description: Some(
                "desc DIR1.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "ad",
                    description: Some(
                        "desc AD.",
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
                    name: "pchs",
                    description: Some(
                        "desc PCHS.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 16,
                        },
                    ),
                    bit_size: 4,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Dir10",
            extends: None,
            description: Some(
                "desc DIR10.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "ad",
                    description: Some(
                        "desc AD.",
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
                    name: "pchs",
                    description: Some(
                        "desc PCHS.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 16,
                        },
                    ),
                    bit_size: 4,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Dir11",
            extends: None,
            description: Some(
                "desc DIR11.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "ad",
                    description: Some(
                        "desc AD.",
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
                    name: "pchs",
                    description: Some(
                        "desc PCHS.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 16,
                        },
                    ),
                    bit_size: 4,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Dir12",
            extends: None,
            description: Some(
                "desc DIR12.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "ad",
                    description: Some(
                        "desc AD.",
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
                    name: "pchs",
                    description: Some(
                        "desc PCHS.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 16,
                        },
                    ),
                    bit_size: 4,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Dir13",
            extends: None,
            description: Some(
                "desc DIR13.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "ad",
                    description: Some(
                        "desc AD.",
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
                    name: "pchs",
                    description: Some(
                        "desc PCHS.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 16,
                        },
                    ),
                    bit_size: 4,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Dir14",
            extends: None,
            description: Some(
                "desc DIR14.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "ad",
                    description: Some(
                        "desc AD.",
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
                    name: "pchs",
                    description: Some(
                        "desc PCHS.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 16,
                        },
                    ),
                    bit_size: 4,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Dir15",
            extends: None,
            description: Some(
                "desc DIR15.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "ad",
                    description: Some(
                        "desc AD.",
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
                    name: "pchs",
                    description: Some(
                        "desc PCHS.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 16,
                        },
                    ),
                    bit_size: 4,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Dir2",
            extends: None,
            description: Some(
                "desc DIR2.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "ad",
                    description: Some(
                        "desc AD.",
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
                    name: "pchs",
                    description: Some(
                        "desc PCHS.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 16,
                        },
                    ),
                    bit_size: 4,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Dir3",
            extends: None,
            description: Some(
                "desc DIR3.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "ad",
                    description: Some(
                        "desc AD.",
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
                    name: "pchs",
                    description: Some(
                        "desc PCHS.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 16,
                        },
                    ),
                    bit_size: 4,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Dir4",
            extends: None,
            description: Some(
                "desc DIR4.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "ad",
                    description: Some(
                        "desc AD.",
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
                    name: "pchs",
                    description: Some(
                        "desc PCHS.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 16,
                        },
                    ),
                    bit_size: 4,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Dir5",
            extends: None,
            description: Some(
                "desc DIR5.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "ad",
                    description: Some(
                        "desc AD.",
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
                    name: "pchs",
                    description: Some(
                        "desc PCHS.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 16,
                        },
                    ),
                    bit_size: 4,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Dir6",
            extends: None,
            description: Some(
                "desc DIR6.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "ad",
                    description: Some(
                        "desc AD.",
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
                    name: "pchs",
                    description: Some(
                        "desc PCHS.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 16,
                        },
                    ),
                    bit_size: 4,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Dir7",
            extends: None,
            description: Some(
                "desc DIR7.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "ad",
                    description: Some(
                        "desc AD.",
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
                    name: "pchs",
                    description: Some(
                        "desc PCHS.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 16,
                        },
                    ),
                    bit_size: 4,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Dir8",
            extends: None,
            description: Some(
                "desc DIR8.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "ad",
                    description: Some(
                        "desc AD.",
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
                    name: "pchs",
                    description: Some(
                        "desc PCHS.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 16,
                        },
                    ),
                    bit_size: 4,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Dir9",
            extends: None,
            description: Some(
                "desc DIR9.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "ad",
                    description: Some(
                        "desc AD.",
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
                    name: "pchs",
                    description: Some(
                        "desc PCHS.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 16,
                        },
                    ),
                    bit_size: 4,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Exchselr",
            extends: None,
            description: Some(
                "desc EXCHSELR.",
            ),
            bit_size: 8,
            fields: &[
                Field {
                    name: "exchsel",
                    description: Some(
                        "desc EXCHSEL.",
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
            name: "Icr",
            extends: None,
            description: Some(
                "desc ICR.",
            ),
            bit_size: 8,
            fields: &[
                Field {
                    name: "eocaien",
                    description: Some(
                        "desc EOCAIEN.",
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
                    name: "eocbien",
                    description: Some(
                        "desc EOCBIEN.",
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
                    name: "eoccien",
                    description: Some(
                        "desc EOCCIEN.",
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
            name: "Isclrr",
            extends: None,
            description: Some(
                "desc ISCLRR.",
            ),
            bit_size: 8,
            fields: &[
                Field {
                    name: "clreocaf",
                    description: Some(
                        "desc CLREOCAF.",
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
                    name: "clreocbf",
                    description: Some(
                        "desc CLREOCBF.",
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
                    name: "clreoccf",
                    description: Some(
                        "desc CLREOCCF.",
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
                    name: "clrsastpdf",
                    description: Some(
                        "desc CLRSASTPDF.",
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
                    name: "clrsbstpdf",
                    description: Some(
                        "desc CLRSBSTPDF.",
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
            name: "Isr",
            extends: None,
            description: Some(
                "desc ISR.",
            ),
            bit_size: 8,
            fields: &[
                Field {
                    name: "eocaf",
                    description: Some(
                        "desc EOCAF.",
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
                    name: "eocbf",
                    description: Some(
                        "desc EOCBF.",
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
                    name: "eoccf",
                    description: Some(
                        "desc EOCCF.",
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
                    name: "sastpdf",
                    description: Some(
                        "desc SASTPDF.",
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
                    name: "sbstpdf",
                    description: Some(
                        "desc SBSTPDF.",
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
            name: "Opacr",
            extends: None,
            description: Some(
                "desc OPACR.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "opa1en",
                    description: Some(
                        "desc OPA1EN.",
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
                    name: "opa2en",
                    description: Some(
                        "desc OPA2EN.",
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
        FieldSet {
            name: "Str",
            extends: None,
            description: Some(
                "desc STR.",
            ),
            bit_size: 8,
            fields: &[
                Field {
                    name: "strt",
                    description: Some(
                        "desc STRT.",
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
            name: "Trgsr",
            extends: None,
            description: Some(
                "desc TRGSR.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "trgsela",
                    description: Some(
                        "desc TRGSELA.",
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
                    name: "trgena",
                    description: Some(
                        "desc TRGENA.",
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
                    name: "trgselb",
                    description: Some(
                        "desc TRGSELB.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 8,
                        },
                    ),
                    bit_size: 7,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "trgenb",
                    description: Some(
                        "desc TRGENB.",
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
                    name: "trgselc",
                    description: Some(
                        "desc TRGSELC.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 16,
                        },
                    ),
                    bit_size: 7,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "trgenc",
                    description: Some(
                        "desc TRGENC.",
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
            ],
        },
    ],
    enums: &[],
};
                