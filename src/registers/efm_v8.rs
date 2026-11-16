
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
                    name: "cr",
                    description: Some(
                        "desc CR.",
                    ),
                    array: None,
                    byte_offset: 0x0,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Cr",
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
                    byte_offset: 0x10,
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
                    name: "ifr",
                    description: Some(
                        "desc IFR.",
                    ),
                    array: None,
                    byte_offset: 0x14,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::Read,
                            bit_size: 32,
                            fieldset: Some(
                                "Ifr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "iclr",
                    description: Some(
                        "desc ICLR.",
                    ),
                    array: None,
                    byte_offset: 0x18,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::Write,
                            bit_size: 32,
                            fieldset: Some(
                                "Iclr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "bypass",
                    description: Some(
                        "desc BYPASS.",
                    ),
                    array: None,
                    byte_offset: 0x2c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Bypass",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "slock0",
                    description: Some(
                        "desc SLOCK0.",
                    ),
                    array: None,
                    byte_offset: 0x30,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Slock0",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "slock1",
                    description: Some(
                        "desc SLOCK1.",
                    ),
                    array: None,
                    byte_offset: 0x34,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Slock1",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "slock2",
                    description: Some(
                        "desc SLOCK2.",
                    ),
                    array: None,
                    byte_offset: 0x38,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Slock2",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "slock3",
                    description: Some(
                        "desc SLOCK3.",
                    ),
                    array: None,
                    byte_offset: 0x3c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Slock3",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "slock4",
                    description: Some(
                        "desc SLOCK4.",
                    ),
                    array: None,
                    byte_offset: 0x40,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Slock4",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "slock5",
                    description: Some(
                        "desc SLOCK5.",
                    ),
                    array: None,
                    byte_offset: 0x44,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Slock5",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "slock6",
                    description: Some(
                        "desc SLOCK6.",
                    ),
                    array: None,
                    byte_offset: 0x48,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Slock6",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "slock7",
                    description: Some(
                        "desc SLOCK7.",
                    ),
                    array: None,
                    byte_offset: 0x4c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Slock7",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "lockst",
                    description: Some(
                        "desc LOCKST.",
                    ),
                    array: None,
                    byte_offset: 0x74,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::Read,
                            bit_size: 32,
                            fieldset: Some(
                                "Lockst",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "lock2st",
                    description: Some(
                        "desc LOCK2ST.",
                    ),
                    array: None,
                    byte_offset: 0x78,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::Read,
                            bit_size: 32,
                            fieldset: Some(
                                "Lock2st",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "ecccr",
                    description: Some(
                        "desc ECCCR.",
                    ),
                    array: None,
                    byte_offset: 0x80,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Ecccr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "eccdr",
                    description: Some(
                        "desc ECCDR.",
                    ),
                    array: None,
                    byte_offset: 0x84,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Eccdr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "eccadr0",
                    description: Some(
                        "desc ECCADR0.",
                    ),
                    array: None,
                    byte_offset: 0x88,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::Read,
                            bit_size: 32,
                            fieldset: Some(
                                "Eccadr0",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "eccadr1",
                    description: Some(
                        "desc ECCADR1.",
                    ),
                    array: None,
                    byte_offset: 0x8c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::Read,
                            bit_size: 32,
                            fieldset: Some(
                                "Eccadr1",
                            ),
                        },
                    ),
                },
            ],
        },
    ],
    fieldsets: &[
        FieldSet {
            name: "Bypass",
            extends: None,
            description: Some(
                "desc BYPASS.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "byseq",
                    description: Some(
                        "desc BYSEQ.",
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
            name: "Cr",
            extends: None,
            description: Some(
                "desc CR.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "op",
                    description: Some(
                        "desc OP.",
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
                    name: "wait",
                    description: Some(
                        "desc WAIT.",
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
                    name: "dpstben",
                    description: Some(
                        "desc DPSTBEN.",
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
                    name: "prefetchen",
                    description: Some(
                        "desc PREFETCHEN.",
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
                    name: "readonly",
                    description: Some(
                        "desc READONLY.",
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
                    name: "pcropcrcen",
                    description: Some(
                        "desc PCROPCRCEN.",
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
            ],
        },
        FieldSet {
            name: "Eccadr0",
            extends: None,
            description: Some(
                "desc ECCADR0.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "index",
                    description: Some(
                        "desc INDEX.",
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
                    name: "flashsel",
                    description: Some(
                        "desc FLASHSEL.",
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
            ],
        },
        FieldSet {
            name: "Eccadr1",
            extends: None,
            description: Some(
                "desc ECCADR1.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "index",
                    description: Some(
                        "desc INDEX.",
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
                    name: "flashsel",
                    description: Some(
                        "desc FLASHSEL.",
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
            ],
        },
        FieldSet {
            name: "Ecccr",
            extends: None,
            description: Some(
                "desc ECCCR.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "eccen",
                    description: Some(
                        "desc ECCEN.",
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
                    name: "gdis",
                    description: Some(
                        "desc GDIS.",
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
                    name: "key",
                    description: Some(
                        "desc KEY.",
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
            name: "Eccdr",
            extends: None,
            description: Some(
                "desc ECCDR.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "eccwd",
                    description: Some(
                        "desc ECCWD.",
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
            name: "Iclr",
            extends: None,
            description: Some(
                "desc ICLR.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "pc",
                    description: Some(
                        "desc PC.",
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
                    name: "prot",
                    description: Some(
                        "desc PROT.",
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
                    name: "pcrop",
                    description: Some(
                        "desc PCROP.",
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
                    name: "progsize",
                    description: Some(
                        "desc PROGSIZE.",
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
                    name: "ecc1err0",
                    description: Some(
                        "desc ECC1ERR0.",
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
                    name: "ecc1err1",
                    description: Some(
                        "desc ECC1ERR1.",
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
                    name: "ecc2err0",
                    description: Some(
                        "desc ECC2ERR0.",
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
                    name: "ecc2err1",
                    description: Some(
                        "desc ECC2ERR1.",
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
                    name: "eccovf",
                    description: Some(
                        "desc ECCOVF.",
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
                    name: "pc",
                    description: Some(
                        "desc PC.",
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
                    name: "prot",
                    description: Some(
                        "desc PROT.",
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
                    name: "pcrop",
                    description: Some(
                        "desc PCROP.",
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
                    name: "progsize",
                    description: Some(
                        "desc PROGSIZE.",
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
                    name: "ecc1err0",
                    description: Some(
                        "desc ECC1ERR0.",
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
                    name: "ecc1err1",
                    description: Some(
                        "desc ECC1ERR1.",
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
                    name: "ecc2err0",
                    description: Some(
                        "desc ECC2ERR0.",
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
                    name: "ecc2err1",
                    description: Some(
                        "desc ECC2ERR1.",
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
                    name: "eccovf",
                    description: Some(
                        "desc ECCOVF.",
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
            ],
        },
        FieldSet {
            name: "Ifr",
            extends: None,
            description: Some(
                "desc IFR.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "pc",
                    description: Some(
                        "desc PC.",
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
                    name: "prot",
                    description: Some(
                        "desc PROT.",
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
                    name: "busy",
                    description: Some(
                        "desc BUSY.",
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
                    name: "pcrop",
                    description: Some(
                        "desc PCROP.",
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
                    name: "progsize",
                    description: Some(
                        "desc PROGSIZE.",
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
                    name: "ecc1err0",
                    description: Some(
                        "desc ECC1ERR0.",
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
                    name: "ecc1err1",
                    description: Some(
                        "desc ECC1ERR1.",
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
                    name: "ecc2err0",
                    description: Some(
                        "desc ECC2ERR0.",
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
                    name: "ecc2err1",
                    description: Some(
                        "desc ECC2ERR1.",
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
                    name: "eccovf",
                    description: Some(
                        "desc ECCOVF.",
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
            ],
        },
        FieldSet {
            name: "Lock2st",
            extends: None,
            description: Some(
                "desc LOCK2ST.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "pcropstart",
                    description: Some(
                        "desc PCROPSTART.",
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
                    name: "pcropend",
                    description: Some(
                        "desc PCROPEND.",
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
            name: "Lockst",
            extends: None,
            description: Some(
                "desc LOCKST.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "state",
                    description: Some(
                        "desc STATE.",
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
            name: "Slock0",
            extends: None,
            description: Some(
                "desc SLOCK0.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "slock",
                    description: Some(
                        "desc SLOCK.",
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
            name: "Slock1",
            extends: None,
            description: Some(
                "desc SLOCK1.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "slock",
                    description: Some(
                        "desc SLOCK.",
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
            name: "Slock2",
            extends: None,
            description: Some(
                "desc SLOCK2.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "slock",
                    description: Some(
                        "desc SLOCK.",
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
            name: "Slock3",
            extends: None,
            description: Some(
                "desc SLOCK3.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "slock",
                    description: Some(
                        "desc SLOCK.",
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
            name: "Slock4",
            extends: None,
            description: Some(
                "desc SLOCK4.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "slock",
                    description: Some(
                        "desc SLOCK.",
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
            name: "Slock5",
            extends: None,
            description: Some(
                "desc SLOCK5.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "slock",
                    description: Some(
                        "desc SLOCK.",
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
            name: "Slock6",
            extends: None,
            description: Some(
                "desc SLOCK6.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "slock",
                    description: Some(
                        "desc SLOCK.",
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
            name: "Slock7",
            extends: None,
            description: Some(
                "desc SLOCK7.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "slock",
                    description: Some(
                        "desc SLOCK.",
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
    ],
    enums: &[],
};
                