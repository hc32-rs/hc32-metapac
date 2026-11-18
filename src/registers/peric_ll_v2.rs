use crate::metadata::ir::*;
pub(crate) static REGISTERS: IR = IR {
    blocks: &[Block {
        name: "Peric",
        extends: None,
        description: Some("desc PERIC."),
        items: &[
            BlockItem {
                name: "usbfs_syctlreg",
                description: Some("desc USBFS_SYCTLREG."),
                array: None,
                byte_offset: 0x0,
                inner: BlockItemInner::Register(Register {
                    access: Access::ReadWrite,
                    bit_size: 32,
                    fieldset: Some("UsbfsSyctlreg"),
                }),
            },
            BlockItem {
                name: "sdioc_syctlreg",
                description: Some("desc SDIOC_SYCTLREG."),
                array: None,
                byte_offset: 0x4,
                inner: BlockItemInner::Register(Register {
                    access: Access::ReadWrite,
                    bit_size: 32,
                    fieldset: Some("SdiocSyctlreg"),
                }),
            },
        ],
    }],
    fieldsets: &[
        FieldSet {
            name: "SdiocSyctlreg",
            extends: None,
            description: Some("desc SDIOC_SYCTLREG."),
            bit_size: 32,
            fields: &[
                Field {
                    name: "selmmc1",
                    description: Some("desc SELMMC1."),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 1 }),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "selmmc2",
                    description: Some("desc SELMMC2."),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 3 }),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "UsbfsSyctlreg",
            extends: None,
            description: Some("desc USBFS_SYCTLREG."),
            bit_size: 32,
            fields: &[
                Field {
                    name: "dfb",
                    description: Some("desc DFB."),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 0 }),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "sofen",
                    description: Some("desc SOFEN."),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 1 }),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
            ],
        },
    ],
    enums: &[],
};
