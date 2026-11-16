
                    use crate::metadata::ir::*;
                    pub(crate) static REGISTERS: IR = IR {
    blocks: &[
        Block {
            name: "Peric",
            extends: None,
            description: Some(
                "desc PERIC.",
            ),
            items: &[
                BlockItem {
                    name: "usb_syctlreg",
                    description: Some(
                        "desc USB_SYCTLREG.",
                    ),
                    array: None,
                    byte_offset: 0x0,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "UsbSyctlreg",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "smc_enar",
                    description: Some(
                        "desc SMC_ENAR.",
                    ),
                    array: None,
                    byte_offset: 0xc,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "SmcEnar",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "can_syctlreg",
                    description: Some(
                        "desc CAN_SYCTLREG.",
                    ),
                    array: None,
                    byte_offset: 0x18,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "CanSyctlreg",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "usart1_nfc",
                    description: Some(
                        "desc USART1_NFC.",
                    ),
                    array: None,
                    byte_offset: 0x1c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Usart1Nfc",
                            ),
                        },
                    ),
                },
            ],
        },
    ],
    fieldsets: &[
        FieldSet {
            name: "CanSyctlreg",
            extends: None,
            description: Some(
                "desc CAN_SYCTLREG.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "can1fde",
                    description: Some(
                        "desc CAN1FDE.",
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
                    name: "can2fde",
                    description: Some(
                        "desc CAN2FDE.",
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
                    name: "can3fde",
                    description: Some(
                        "desc CAN3FDE.",
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
            name: "SmcEnar",
            extends: None,
            description: Some(
                "desc SMC_ENAR.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "smcen",
                    description: Some(
                        "desc SMCEN.",
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
            name: "Usart1Nfc",
            extends: None,
            description: Some(
                "desc USART1_NFC.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "usasrt1_nfs",
                    description: Some(
                        "desc USASRT1_NFS.",
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
                    name: "usart1_nfe",
                    description: Some(
                        "desc USART1_NFE.",
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
            name: "UsbSyctlreg",
            extends: None,
            description: Some(
                "desc USB_SYCTLREG.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "usbfs_dfb",
                    description: Some(
                        "desc USBFS_DFB.",
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
                    name: "usbfs_sofen",
                    description: Some(
                        "desc USBFS_SOFEN.",
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
                    name: "usbfs_wkup",
                    description: Some(
                        "desc USBFS_WKUP.",
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
                    name: "usbfs_dmpd",
                    description: Some(
                        "desc USBFS_DMPD.",
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
                    name: "usbfs_dppd",
                    description: Some(
                        "desc USBFS_DPPD.",
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
                    name: "usbfs_pden",
                    description: Some(
                        "desc USBFS_PDEN.",
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
                    name: "usbfs_nfs",
                    description: Some(
                        "desc USBFS_NFS.",
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
                    name: "usbfs_nfe",
                    description: Some(
                        "desc USBFS_NFE.",
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
            ],
        },
    ],
    enums: &[],
};
                