
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
                    name: "base1_llp",
                    description: Some(
                        "desc BASE1_LLP.",
                    ),
                    array: None,
                    byte_offset: 0x30,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Base1Llp",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "base2_llp",
                    description: Some(
                        "desc BASE2_LLP.",
                    ),
                    array: None,
                    byte_offset: 0x34,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Base2Llp",
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
                    byte_offset: 0x38,
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
                    name: "ch0ctl0",
                    description: Some(
                        "desc CH0CTL0.",
                    ),
                    array: None,
                    byte_offset: 0x48,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Ch0ctl0",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "ch0ctl1",
                    description: Some(
                        "desc CH0CTL1.",
                    ),
                    array: None,
                    byte_offset: 0x4c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Ch0ctl1",
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
                    name: "ch1ctl0",
                    description: Some(
                        "desc CH1CTL0.",
                    ),
                    array: None,
                    byte_offset: 0x88,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Ch1ctl0",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "ch1ctl1",
                    description: Some(
                        "desc CH1CTL1.",
                    ),
                    array: None,
                    byte_offset: 0x8c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Ch1ctl1",
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
                    name: "ch2ctl0",
                    description: Some(
                        "desc CH2CTL0.",
                    ),
                    array: None,
                    byte_offset: 0xc8,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Ch2ctl0",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "ch2ctl1",
                    description: Some(
                        "desc CH2CTL1.",
                    ),
                    array: None,
                    byte_offset: 0xcc,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Ch2ctl1",
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
                    name: "ch3ctl0",
                    description: Some(
                        "desc CH3CTL0.",
                    ),
                    array: None,
                    byte_offset: 0x108,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Ch3ctl0",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "ch3ctl1",
                    description: Some(
                        "desc CH3CTL1.",
                    ),
                    array: None,
                    byte_offset: 0x10c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Ch3ctl1",
                            ),
                        },
                    ),
                },
            ],
        },
    ],
    fieldsets: &[
        FieldSet {
            name: "Base1Llp",
            extends: None,
            description: Some(
                "desc BASE1_LLP.",
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
            name: "Base2Llp",
            extends: None,
            description: Some(
                "desc BASE2_LLP.",
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
            name: "Ch0ctl0",
            extends: None,
            description: Some(
                "desc CH0CTL0.",
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
                    bit_size: 8,
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
                            offset: 8,
                        },
                    ),
                    bit_size: 10,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "llp",
                    description: Some(
                        "desc LLP.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 18,
                        },
                    ),
                    bit_size: 10,
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
                            offset: 28,
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
                            offset: 29,
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
                            offset: 30,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Ch0ctl1",
            extends: None,
            description: Some(
                "desc CH0CTL1.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "offset",
                    description: Some(
                        "desc OFFSET.",
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
                    name: "rptnscnt",
                    description: Some(
                        "desc RPTNSCNT.",
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
                    name: "rptnsen",
                    description: Some(
                        "desc RPTNSEN.",
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
                    name: "rptnssel",
                    description: Some(
                        "desc RPTNSSEL.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 25,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "llpsel",
                    description: Some(
                        "desc LLPSEL.",
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
                    name: "sinc",
                    description: Some(
                        "desc SINC.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 28,
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
                            offset: 30,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Ch1ctl0",
            extends: None,
            description: Some(
                "desc CH1CTL0.",
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
                    bit_size: 8,
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
                            offset: 8,
                        },
                    ),
                    bit_size: 10,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "llp",
                    description: Some(
                        "desc LLP.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 18,
                        },
                    ),
                    bit_size: 10,
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
                            offset: 28,
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
                            offset: 29,
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
                            offset: 30,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Ch1ctl1",
            extends: None,
            description: Some(
                "desc CH1CTL1.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "offset",
                    description: Some(
                        "desc OFFSET.",
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
                    name: "rptnscnt",
                    description: Some(
                        "desc RPTNSCNT.",
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
                    name: "rptnsen",
                    description: Some(
                        "desc RPTNSEN.",
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
                    name: "rptnssel",
                    description: Some(
                        "desc RPTNSSEL.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 25,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "llpsel",
                    description: Some(
                        "desc LLPSEL.",
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
                    name: "sinc",
                    description: Some(
                        "desc SINC.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 28,
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
                            offset: 30,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Ch2ctl0",
            extends: None,
            description: Some(
                "desc CH2CTL0.",
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
                    bit_size: 8,
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
                            offset: 8,
                        },
                    ),
                    bit_size: 10,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "llp",
                    description: Some(
                        "desc LLP.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 18,
                        },
                    ),
                    bit_size: 10,
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
                            offset: 28,
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
                            offset: 29,
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
                            offset: 30,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Ch2ctl1",
            extends: None,
            description: Some(
                "desc CH2CTL1.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "offset",
                    description: Some(
                        "desc OFFSET.",
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
                    name: "rptnscnt",
                    description: Some(
                        "desc RPTNSCNT.",
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
                    name: "rptnsen",
                    description: Some(
                        "desc RPTNSEN.",
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
                    name: "rptnssel",
                    description: Some(
                        "desc RPTNSSEL.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 25,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "llpsel",
                    description: Some(
                        "desc LLPSEL.",
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
                    name: "sinc",
                    description: Some(
                        "desc SINC.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 28,
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
                            offset: 30,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Ch3ctl0",
            extends: None,
            description: Some(
                "desc CH3CTL0.",
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
                    bit_size: 8,
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
                            offset: 8,
                        },
                    ),
                    bit_size: 10,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "llp",
                    description: Some(
                        "desc LLP.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 18,
                        },
                    ),
                    bit_size: 10,
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
                            offset: 28,
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
                            offset: 29,
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
                            offset: 30,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Ch3ctl1",
            extends: None,
            description: Some(
                "desc CH3CTL1.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "offset",
                    description: Some(
                        "desc OFFSET.",
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
                    name: "rptnscnt",
                    description: Some(
                        "desc RPTNSCNT.",
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
                    name: "rptnsen",
                    description: Some(
                        "desc RPTNSEN.",
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
                    name: "rptnssel",
                    description: Some(
                        "desc RPTNSSEL.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 25,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "llpsel",
                    description: Some(
                        "desc LLPSEL.",
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
                    name: "sinc",
                    description: Some(
                        "desc SINC.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 28,
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
                            offset: 30,
                        },
                    ),
                    bit_size: 2,
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
                    bit_size: 4,
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
                    bit_size: 4,
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
                    name: "chact",
                    description: Some(
                        "desc CHACT.",
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
                    bit_size: 4,
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
                    bit_size: 4,
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
                    bit_size: 4,
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
                    bit_size: 4,
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
                    bit_size: 4,
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
                    bit_size: 4,
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
                    bit_size: 4,
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
                    bit_size: 4,
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
                    bit_size: 4,
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
                    bit_size: 4,
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
                    bit_size: 4,
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
                    bit_size: 4,
                    array: None,
                    enumm: None,
                },
            ],
        },
    ],
    enums: &[],
};
                