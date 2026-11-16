
                    use crate::metadata::ir::*;
                    pub(crate) static REGISTERS: IR = IR {
    blocks: &[
        Block {
            name: "Pla",
            extends: None,
            description: Some(
                "desc PLA.",
            ),
            items: &[
                BlockItem {
                    name: "gctl",
                    description: Some(
                        "desc GCTL.",
                    ),
                    array: None,
                    byte_offset: 0x0,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Gctl",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "stat",
                    description: Some(
                        "desc STAT.",
                    ),
                    array: None,
                    byte_offset: 0xc,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::Read,
                            bit_size: 32,
                            fieldset: Some(
                                "Stat",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "muxs0",
                    description: Some(
                        "desc MUXS0.",
                    ),
                    array: None,
                    byte_offset: 0x10,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Muxs0",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "lutctl0",
                    description: Some(
                        "desc LUTCTL0.",
                    ),
                    array: None,
                    byte_offset: 0x14,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Lutctl0",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "ctl0",
                    description: Some(
                        "desc CTL0.",
                    ),
                    array: None,
                    byte_offset: 0x18,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Ctl0",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "muxs1",
                    description: Some(
                        "desc MUXS1.",
                    ),
                    array: None,
                    byte_offset: 0x1c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Muxs1",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "lutctl1",
                    description: Some(
                        "desc LUTCTL1.",
                    ),
                    array: None,
                    byte_offset: 0x20,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Lutctl1",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "ctl1",
                    description: Some(
                        "desc CTL1.",
                    ),
                    array: None,
                    byte_offset: 0x24,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Ctl1",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "muxs2",
                    description: Some(
                        "desc MUXS2.",
                    ),
                    array: None,
                    byte_offset: 0x28,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Muxs2",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "lutctl2",
                    description: Some(
                        "desc LUTCTL2.",
                    ),
                    array: None,
                    byte_offset: 0x2c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Lutctl2",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "ctl2",
                    description: Some(
                        "desc CTL2.",
                    ),
                    array: None,
                    byte_offset: 0x30,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Ctl2",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "muxs3",
                    description: Some(
                        "desc MUXS3.",
                    ),
                    array: None,
                    byte_offset: 0x34,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Muxs3",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "lutctl3",
                    description: Some(
                        "desc LUTCTL3.",
                    ),
                    array: None,
                    byte_offset: 0x38,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Lutctl3",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "ctl3",
                    description: Some(
                        "desc CTL3.",
                    ),
                    array: None,
                    byte_offset: 0x3c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Ctl3",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "muxs4",
                    description: Some(
                        "desc MUXS4.",
                    ),
                    array: None,
                    byte_offset: 0x40,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Muxs4",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "lutctl4",
                    description: Some(
                        "desc LUTCTL4.",
                    ),
                    array: None,
                    byte_offset: 0x44,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Lutctl4",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "ctl4",
                    description: Some(
                        "desc CTL4.",
                    ),
                    array: None,
                    byte_offset: 0x48,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Ctl4",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "muxs5",
                    description: Some(
                        "desc MUXS5.",
                    ),
                    array: None,
                    byte_offset: 0x4c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Muxs5",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "lutctl5",
                    description: Some(
                        "desc LUTCTL5.",
                    ),
                    array: None,
                    byte_offset: 0x50,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Lutctl5",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "ctl5",
                    description: Some(
                        "desc CTL5.",
                    ),
                    array: None,
                    byte_offset: 0x54,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Ctl5",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "muxs6",
                    description: Some(
                        "desc MUXS6.",
                    ),
                    array: None,
                    byte_offset: 0x58,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Muxs6",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "lutctl6",
                    description: Some(
                        "desc LUTCTL6.",
                    ),
                    array: None,
                    byte_offset: 0x5c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Lutctl6",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "ctl6",
                    description: Some(
                        "desc CTL6.",
                    ),
                    array: None,
                    byte_offset: 0x60,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Ctl6",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "muxs7",
                    description: Some(
                        "desc MUXS7.",
                    ),
                    array: None,
                    byte_offset: 0x64,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Muxs7",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "lutctl7",
                    description: Some(
                        "desc LUTCTL7.",
                    ),
                    array: None,
                    byte_offset: 0x68,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Lutctl7",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "ctl7",
                    description: Some(
                        "desc CTL7.",
                    ),
                    array: None,
                    byte_offset: 0x6c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Ctl7",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "muxs8",
                    description: Some(
                        "desc MUXS8.",
                    ),
                    array: None,
                    byte_offset: 0x70,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Muxs8",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "lutctl8",
                    description: Some(
                        "desc LUTCTL8.",
                    ),
                    array: None,
                    byte_offset: 0x74,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Lutctl8",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "ctl8",
                    description: Some(
                        "desc CTL8.",
                    ),
                    array: None,
                    byte_offset: 0x78,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Ctl8",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "muxs9",
                    description: Some(
                        "desc MUXS9.",
                    ),
                    array: None,
                    byte_offset: 0x7c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Muxs9",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "lutctl9",
                    description: Some(
                        "desc LUTCTL9.",
                    ),
                    array: None,
                    byte_offset: 0x80,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Lutctl9",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "ctl9",
                    description: Some(
                        "desc CTL9.",
                    ),
                    array: None,
                    byte_offset: 0x84,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Ctl9",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "muxs10",
                    description: Some(
                        "desc MUXS10.",
                    ),
                    array: None,
                    byte_offset: 0x88,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Muxs10",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "lutctl10",
                    description: Some(
                        "desc LUTCTL10.",
                    ),
                    array: None,
                    byte_offset: 0x8c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Lutctl10",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "ctl10",
                    description: Some(
                        "desc CTL10.",
                    ),
                    array: None,
                    byte_offset: 0x90,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Ctl10",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "muxs11",
                    description: Some(
                        "desc MUXS11.",
                    ),
                    array: None,
                    byte_offset: 0x94,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Muxs11",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "lutctl11",
                    description: Some(
                        "desc LUTCTL11.",
                    ),
                    array: None,
                    byte_offset: 0x98,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Lutctl11",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "ctl11",
                    description: Some(
                        "desc CTL11.",
                    ),
                    array: None,
                    byte_offset: 0x9c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Ctl11",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "muxs12",
                    description: Some(
                        "desc MUXS12.",
                    ),
                    array: None,
                    byte_offset: 0xa0,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Muxs12",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "lutctl12",
                    description: Some(
                        "desc LUTCTL12.",
                    ),
                    array: None,
                    byte_offset: 0xa4,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Lutctl12",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "ctl12",
                    description: Some(
                        "desc CTL12.",
                    ),
                    array: None,
                    byte_offset: 0xa8,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Ctl12",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "muxs13",
                    description: Some(
                        "desc MUXS13.",
                    ),
                    array: None,
                    byte_offset: 0xac,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Muxs13",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "lutctl13",
                    description: Some(
                        "desc LUTCTL13.",
                    ),
                    array: None,
                    byte_offset: 0xb0,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Lutctl13",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "ctl13",
                    description: Some(
                        "desc CTL13.",
                    ),
                    array: None,
                    byte_offset: 0xb4,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Ctl13",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "muxs14",
                    description: Some(
                        "desc MUXS14.",
                    ),
                    array: None,
                    byte_offset: 0xb8,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Muxs14",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "lutctl14",
                    description: Some(
                        "desc LUTCTL14.",
                    ),
                    array: None,
                    byte_offset: 0xbc,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Lutctl14",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "ctl14",
                    description: Some(
                        "desc CTL14.",
                    ),
                    array: None,
                    byte_offset: 0xc0,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Ctl14",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "muxs15",
                    description: Some(
                        "desc MUXS15.",
                    ),
                    array: None,
                    byte_offset: 0xc4,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Muxs15",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "lutctl15",
                    description: Some(
                        "desc LUTCTL15.",
                    ),
                    array: None,
                    byte_offset: 0xc8,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Lutctl15",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "ctl15",
                    description: Some(
                        "desc CTL15.",
                    ),
                    array: None,
                    byte_offset: 0xcc,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Ctl15",
                            ),
                        },
                    ),
                },
            ],
        },
    ],
    fieldsets: &[
        FieldSet {
            name: "Ctl0",
            extends: None,
            description: Some(
                "desc CTL0.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "oen",
                    description: Some(
                        "desc OEN.",
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
                    name: "osel",
                    description: Some(
                        "desc OSEL.",
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
            name: "Ctl1",
            extends: None,
            description: Some(
                "desc CTL1.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "oen",
                    description: Some(
                        "desc OEN.",
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
                    name: "osel",
                    description: Some(
                        "desc OSEL.",
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
            name: "Ctl10",
            extends: None,
            description: Some(
                "desc CTL10.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "oen",
                    description: Some(
                        "desc OEN.",
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
                    name: "osel",
                    description: Some(
                        "desc OSEL.",
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
            name: "Ctl11",
            extends: None,
            description: Some(
                "desc CTL11.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "oen",
                    description: Some(
                        "desc OEN.",
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
                    name: "osel",
                    description: Some(
                        "desc OSEL.",
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
            name: "Ctl12",
            extends: None,
            description: Some(
                "desc CTL12.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "oen",
                    description: Some(
                        "desc OEN.",
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
                    name: "osel",
                    description: Some(
                        "desc OSEL.",
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
            name: "Ctl13",
            extends: None,
            description: Some(
                "desc CTL13.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "oen",
                    description: Some(
                        "desc OEN.",
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
                    name: "osel",
                    description: Some(
                        "desc OSEL.",
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
            name: "Ctl14",
            extends: None,
            description: Some(
                "desc CTL14.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "oen",
                    description: Some(
                        "desc OEN.",
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
                    name: "osel",
                    description: Some(
                        "desc OSEL.",
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
            name: "Ctl15",
            extends: None,
            description: Some(
                "desc CTL15.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "oen",
                    description: Some(
                        "desc OEN.",
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
                    name: "osel",
                    description: Some(
                        "desc OSEL.",
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
            name: "Ctl2",
            extends: None,
            description: Some(
                "desc CTL2.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "oen",
                    description: Some(
                        "desc OEN.",
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
                    name: "osel",
                    description: Some(
                        "desc OSEL.",
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
            name: "Ctl3",
            extends: None,
            description: Some(
                "desc CTL3.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "oen",
                    description: Some(
                        "desc OEN.",
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
                    name: "osel",
                    description: Some(
                        "desc OSEL.",
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
            name: "Ctl4",
            extends: None,
            description: Some(
                "desc CTL4.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "oen",
                    description: Some(
                        "desc OEN.",
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
                    name: "osel",
                    description: Some(
                        "desc OSEL.",
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
            name: "Ctl5",
            extends: None,
            description: Some(
                "desc CTL5.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "oen",
                    description: Some(
                        "desc OEN.",
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
                    name: "osel",
                    description: Some(
                        "desc OSEL.",
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
            name: "Ctl6",
            extends: None,
            description: Some(
                "desc CTL6.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "oen",
                    description: Some(
                        "desc OEN.",
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
                    name: "osel",
                    description: Some(
                        "desc OSEL.",
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
            name: "Ctl7",
            extends: None,
            description: Some(
                "desc CTL7.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "oen",
                    description: Some(
                        "desc OEN.",
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
                    name: "osel",
                    description: Some(
                        "desc OSEL.",
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
            name: "Ctl8",
            extends: None,
            description: Some(
                "desc CTL8.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "oen",
                    description: Some(
                        "desc OEN.",
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
                    name: "osel",
                    description: Some(
                        "desc OSEL.",
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
            name: "Ctl9",
            extends: None,
            description: Some(
                "desc CTL9.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "oen",
                    description: Some(
                        "desc OEN.",
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
                    name: "osel",
                    description: Some(
                        "desc OSEL.",
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
            name: "Gctl",
            extends: None,
            description: Some(
                "desc GCTL.",
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
                    bit_size: 16,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Lutctl0",
            extends: None,
            description: Some(
                "desc LUTCTL0.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "lutctl",
                    description: Some(
                        "desc LUTCTL.",
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
            name: "Lutctl1",
            extends: None,
            description: Some(
                "desc LUTCTL1.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "lutctl",
                    description: Some(
                        "desc LUTCTL.",
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
            name: "Lutctl10",
            extends: None,
            description: Some(
                "desc LUTCTL10.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "lutctl",
                    description: Some(
                        "desc LUTCTL.",
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
            name: "Lutctl11",
            extends: None,
            description: Some(
                "desc LUTCTL11.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "lutctl",
                    description: Some(
                        "desc LUTCTL.",
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
            name: "Lutctl12",
            extends: None,
            description: Some(
                "desc LUTCTL12.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "lutctl",
                    description: Some(
                        "desc LUTCTL.",
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
            name: "Lutctl13",
            extends: None,
            description: Some(
                "desc LUTCTL13.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "lutctl",
                    description: Some(
                        "desc LUTCTL.",
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
            name: "Lutctl14",
            extends: None,
            description: Some(
                "desc LUTCTL14.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "lutctl",
                    description: Some(
                        "desc LUTCTL.",
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
            name: "Lutctl15",
            extends: None,
            description: Some(
                "desc LUTCTL15.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "lutctl",
                    description: Some(
                        "desc LUTCTL.",
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
            name: "Lutctl2",
            extends: None,
            description: Some(
                "desc LUTCTL2.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "lutctl",
                    description: Some(
                        "desc LUTCTL.",
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
            name: "Lutctl3",
            extends: None,
            description: Some(
                "desc LUTCTL3.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "lutctl",
                    description: Some(
                        "desc LUTCTL.",
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
            name: "Lutctl4",
            extends: None,
            description: Some(
                "desc LUTCTL4.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "lutctl",
                    description: Some(
                        "desc LUTCTL.",
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
            name: "Lutctl5",
            extends: None,
            description: Some(
                "desc LUTCTL5.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "lutctl",
                    description: Some(
                        "desc LUTCTL.",
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
            name: "Lutctl6",
            extends: None,
            description: Some(
                "desc LUTCTL6.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "lutctl",
                    description: Some(
                        "desc LUTCTL.",
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
            name: "Lutctl7",
            extends: None,
            description: Some(
                "desc LUTCTL7.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "lutctl",
                    description: Some(
                        "desc LUTCTL.",
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
            name: "Lutctl8",
            extends: None,
            description: Some(
                "desc LUTCTL8.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "lutctl",
                    description: Some(
                        "desc LUTCTL.",
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
            name: "Lutctl9",
            extends: None,
            description: Some(
                "desc LUTCTL9.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "lutctl",
                    description: Some(
                        "desc LUTCTL.",
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
            name: "Muxs0",
            extends: None,
            description: Some(
                "desc MUXS0.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "mux1",
                    description: Some(
                        "desc MUX1.",
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
                    name: "mux0",
                    description: Some(
                        "desc MUX0.",
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
                    name: "mux1_4",
                    description: Some(
                        "desc MUX1_4.",
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
                    name: "mux0_4",
                    description: Some(
                        "desc MUX0_4.",
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
            name: "Muxs1",
            extends: None,
            description: Some(
                "desc MUXS1.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "mux1",
                    description: Some(
                        "desc MUX1.",
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
                    name: "mux0",
                    description: Some(
                        "desc MUX0.",
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
                    name: "mux1_4",
                    description: Some(
                        "desc MUX1_4.",
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
                    name: "mux0_4",
                    description: Some(
                        "desc MUX0_4.",
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
            name: "Muxs10",
            extends: None,
            description: Some(
                "desc MUXS10.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "mux1",
                    description: Some(
                        "desc MUX1.",
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
                    name: "mux0",
                    description: Some(
                        "desc MUX0.",
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
                    name: "mux1_4",
                    description: Some(
                        "desc MUX1_4.",
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
                    name: "mux0_4",
                    description: Some(
                        "desc MUX0_4.",
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
            name: "Muxs11",
            extends: None,
            description: Some(
                "desc MUXS11.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "mux1",
                    description: Some(
                        "desc MUX1.",
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
                    name: "mux0",
                    description: Some(
                        "desc MUX0.",
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
                    name: "mux1_4",
                    description: Some(
                        "desc MUX1_4.",
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
                    name: "mux0_4",
                    description: Some(
                        "desc MUX0_4.",
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
            name: "Muxs12",
            extends: None,
            description: Some(
                "desc MUXS12.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "mux1",
                    description: Some(
                        "desc MUX1.",
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
                    name: "mux0",
                    description: Some(
                        "desc MUX0.",
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
                    name: "mux1_4",
                    description: Some(
                        "desc MUX1_4.",
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
                    name: "mux0_4",
                    description: Some(
                        "desc MUX0_4.",
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
            name: "Muxs13",
            extends: None,
            description: Some(
                "desc MUXS13.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "mux1",
                    description: Some(
                        "desc MUX1.",
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
                    name: "mux0",
                    description: Some(
                        "desc MUX0.",
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
                    name: "mux1_4",
                    description: Some(
                        "desc MUX1_4.",
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
                    name: "mux0_4",
                    description: Some(
                        "desc MUX0_4.",
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
            name: "Muxs14",
            extends: None,
            description: Some(
                "desc MUXS14.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "mux1",
                    description: Some(
                        "desc MUX1.",
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
                    name: "mux0",
                    description: Some(
                        "desc MUX0.",
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
                    name: "mux1_4",
                    description: Some(
                        "desc MUX1_4.",
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
                    name: "mux0_4",
                    description: Some(
                        "desc MUX0_4.",
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
            name: "Muxs15",
            extends: None,
            description: Some(
                "desc MUXS15.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "mux1",
                    description: Some(
                        "desc MUX1.",
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
                    name: "mux0",
                    description: Some(
                        "desc MUX0.",
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
                    name: "mux1_4",
                    description: Some(
                        "desc MUX1_4.",
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
                    name: "mux0_4",
                    description: Some(
                        "desc MUX0_4.",
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
            name: "Muxs2",
            extends: None,
            description: Some(
                "desc MUXS2.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "mux1",
                    description: Some(
                        "desc MUX1.",
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
                    name: "mux0",
                    description: Some(
                        "desc MUX0.",
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
                    name: "mux1_4",
                    description: Some(
                        "desc MUX1_4.",
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
                    name: "mux0_4",
                    description: Some(
                        "desc MUX0_4.",
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
            name: "Muxs3",
            extends: None,
            description: Some(
                "desc MUXS3.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "mux1",
                    description: Some(
                        "desc MUX1.",
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
                    name: "mux0",
                    description: Some(
                        "desc MUX0.",
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
                    name: "mux1_4",
                    description: Some(
                        "desc MUX1_4.",
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
                    name: "mux0_4",
                    description: Some(
                        "desc MUX0_4.",
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
            name: "Muxs4",
            extends: None,
            description: Some(
                "desc MUXS4.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "mux1",
                    description: Some(
                        "desc MUX1.",
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
                    name: "mux0",
                    description: Some(
                        "desc MUX0.",
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
                    name: "mux1_4",
                    description: Some(
                        "desc MUX1_4.",
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
                    name: "mux0_4",
                    description: Some(
                        "desc MUX0_4.",
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
            name: "Muxs5",
            extends: None,
            description: Some(
                "desc MUXS5.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "mux1",
                    description: Some(
                        "desc MUX1.",
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
                    name: "mux0",
                    description: Some(
                        "desc MUX0.",
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
                    name: "mux1_4",
                    description: Some(
                        "desc MUX1_4.",
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
                    name: "mux0_4",
                    description: Some(
                        "desc MUX0_4.",
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
            name: "Muxs6",
            extends: None,
            description: Some(
                "desc MUXS6.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "mux1",
                    description: Some(
                        "desc MUX1.",
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
                    name: "mux0",
                    description: Some(
                        "desc MUX0.",
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
                    name: "mux1_4",
                    description: Some(
                        "desc MUX1_4.",
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
                    name: "mux0_4",
                    description: Some(
                        "desc MUX0_4.",
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
            name: "Muxs7",
            extends: None,
            description: Some(
                "desc MUXS7.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "mux1",
                    description: Some(
                        "desc MUX1.",
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
                    name: "mux0",
                    description: Some(
                        "desc MUX0.",
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
                    name: "mux1_4",
                    description: Some(
                        "desc MUX1_4.",
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
                    name: "mux0_4",
                    description: Some(
                        "desc MUX0_4.",
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
            name: "Muxs8",
            extends: None,
            description: Some(
                "desc MUXS8.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "mux1",
                    description: Some(
                        "desc MUX1.",
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
                    name: "mux0",
                    description: Some(
                        "desc MUX0.",
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
                    name: "mux1_4",
                    description: Some(
                        "desc MUX1_4.",
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
                    name: "mux0_4",
                    description: Some(
                        "desc MUX0_4.",
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
            name: "Muxs9",
            extends: None,
            description: Some(
                "desc MUXS9.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "mux1",
                    description: Some(
                        "desc MUX1.",
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
                    name: "mux0",
                    description: Some(
                        "desc MUX0.",
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
                    name: "mux1_4",
                    description: Some(
                        "desc MUX1_4.",
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
                    name: "mux0_4",
                    description: Some(
                        "desc MUX0_4.",
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
            name: "Stat",
            extends: None,
            description: Some(
                "desc STAT.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "out",
                    description: Some(
                        "desc OUT.",
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
    ],
    enums: &[],
};
                