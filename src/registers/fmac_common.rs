
                    use crate::metadata::ir::*;
                    pub(crate) static REGISTERS: IR = IR {
    blocks: &[
        Block {
            name: "Fmac",
            extends: None,
            description: Some(
                "desc FMAC.",
            ),
            items: &[
                BlockItem {
                    name: "enr",
                    description: Some(
                        "desc ENR.",
                    ),
                    array: None,
                    byte_offset: 0x0,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Enr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "ctr",
                    description: Some(
                        "desc CTR.",
                    ),
                    array: None,
                    byte_offset: 0x4,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Ctr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "ier",
                    description: Some(
                        "desc IER.",
                    ),
                    array: None,
                    byte_offset: 0x8,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Ier",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "dtr",
                    description: Some(
                        "desc DTR.",
                    ),
                    array: None,
                    byte_offset: 0xc,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Dtr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "rtr0",
                    description: Some(
                        "desc RTR0.",
                    ),
                    array: None,
                    byte_offset: 0x10,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: None,
                        },
                    ),
                },
                BlockItem {
                    name: "rtr1",
                    description: Some(
                        "desc RTR1.",
                    ),
                    array: None,
                    byte_offset: 0x14,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: None,
                        },
                    ),
                },
                BlockItem {
                    name: "str",
                    description: Some(
                        "desc STR.",
                    ),
                    array: None,
                    byte_offset: 0x18,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Str",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "cor0",
                    description: Some(
                        "desc COR0.",
                    ),
                    array: None,
                    byte_offset: 0x20,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Cor0",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "cor1",
                    description: Some(
                        "desc COR1.",
                    ),
                    array: None,
                    byte_offset: 0x24,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Cor1",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "cor2",
                    description: Some(
                        "desc COR2.",
                    ),
                    array: None,
                    byte_offset: 0x28,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Cor2",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "cor3",
                    description: Some(
                        "desc COR3.",
                    ),
                    array: None,
                    byte_offset: 0x2c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Cor3",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "cor4",
                    description: Some(
                        "desc COR4.",
                    ),
                    array: None,
                    byte_offset: 0x30,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Cor4",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "cor5",
                    description: Some(
                        "desc COR5.",
                    ),
                    array: None,
                    byte_offset: 0x34,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Cor5",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "cor6",
                    description: Some(
                        "desc COR6.",
                    ),
                    array: None,
                    byte_offset: 0x38,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Cor6",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "cor7",
                    description: Some(
                        "desc COR7.",
                    ),
                    array: None,
                    byte_offset: 0x3c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Cor7",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "cor8",
                    description: Some(
                        "desc COR8.",
                    ),
                    array: None,
                    byte_offset: 0x40,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Cor8",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "cor9",
                    description: Some(
                        "desc COR9.",
                    ),
                    array: None,
                    byte_offset: 0x44,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Cor9",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "cor10",
                    description: Some(
                        "desc COR10.",
                    ),
                    array: None,
                    byte_offset: 0x48,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Cor10",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "cor11",
                    description: Some(
                        "desc COR11.",
                    ),
                    array: None,
                    byte_offset: 0x4c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Cor11",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "cor12",
                    description: Some(
                        "desc COR12.",
                    ),
                    array: None,
                    byte_offset: 0x50,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Cor12",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "cor13",
                    description: Some(
                        "desc COR13.",
                    ),
                    array: None,
                    byte_offset: 0x54,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Cor13",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "cor14",
                    description: Some(
                        "desc COR14.",
                    ),
                    array: None,
                    byte_offset: 0x58,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Cor14",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "cor15",
                    description: Some(
                        "desc COR15.",
                    ),
                    array: None,
                    byte_offset: 0x5c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Cor15",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "cor16",
                    description: Some(
                        "desc COR16.",
                    ),
                    array: None,
                    byte_offset: 0x60,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Cor16",
                            ),
                        },
                    ),
                },
            ],
        },
    ],
    fieldsets: &[
        FieldSet {
            name: "Cor0",
            extends: None,
            description: Some(
                "desc COR0.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "cin",
                    description: Some(
                        "desc CIN.",
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
            name: "Cor1",
            extends: None,
            description: Some(
                "desc COR1.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "cin",
                    description: Some(
                        "desc CIN.",
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
            name: "Cor10",
            extends: None,
            description: Some(
                "desc COR10.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "cin",
                    description: Some(
                        "desc CIN.",
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
            name: "Cor11",
            extends: None,
            description: Some(
                "desc COR11.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "cin",
                    description: Some(
                        "desc CIN.",
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
            name: "Cor12",
            extends: None,
            description: Some(
                "desc COR12.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "cin",
                    description: Some(
                        "desc CIN.",
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
            name: "Cor13",
            extends: None,
            description: Some(
                "desc COR13.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "cin",
                    description: Some(
                        "desc CIN.",
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
            name: "Cor14",
            extends: None,
            description: Some(
                "desc COR14.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "cin",
                    description: Some(
                        "desc CIN.",
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
            name: "Cor15",
            extends: None,
            description: Some(
                "desc COR15.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "cin",
                    description: Some(
                        "desc CIN.",
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
            name: "Cor16",
            extends: None,
            description: Some(
                "desc COR16.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "cin",
                    description: Some(
                        "desc CIN.",
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
            name: "Cor2",
            extends: None,
            description: Some(
                "desc COR2.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "cin",
                    description: Some(
                        "desc CIN.",
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
            name: "Cor3",
            extends: None,
            description: Some(
                "desc COR3.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "cin",
                    description: Some(
                        "desc CIN.",
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
            name: "Cor4",
            extends: None,
            description: Some(
                "desc COR4.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "cin",
                    description: Some(
                        "desc CIN.",
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
            name: "Cor5",
            extends: None,
            description: Some(
                "desc COR5.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "cin",
                    description: Some(
                        "desc CIN.",
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
            name: "Cor6",
            extends: None,
            description: Some(
                "desc COR6.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "cin",
                    description: Some(
                        "desc CIN.",
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
            name: "Cor7",
            extends: None,
            description: Some(
                "desc COR7.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "cin",
                    description: Some(
                        "desc CIN.",
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
            name: "Cor8",
            extends: None,
            description: Some(
                "desc COR8.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "cin",
                    description: Some(
                        "desc CIN.",
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
            name: "Cor9",
            extends: None,
            description: Some(
                "desc COR9.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "cin",
                    description: Some(
                        "desc CIN.",
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
            name: "Ctr",
            extends: None,
            description: Some(
                "desc CTR.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "stage_num",
                    description: Some(
                        "desc STAGE_NUM.",
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
                    name: "shift",
                    description: Some(
                        "desc SHIFT.",
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
            name: "Dtr",
            extends: None,
            description: Some(
                "desc DTR.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "din",
                    description: Some(
                        "desc DIN.",
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
            name: "Enr",
            extends: None,
            description: Some(
                "desc ENR.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "fmacen",
                    description: Some(
                        "desc FMACEN.",
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
            name: "Ier",
            extends: None,
            description: Some(
                "desc IER.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "inten",
                    description: Some(
                        "desc INTEN.",
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
            name: "Str",
            extends: None,
            description: Some(
                "desc STR.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "ready",
                    description: Some(
                        "desc READY.",
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
    ],
    enums: &[],
};
                