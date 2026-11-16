
                    use crate::metadata::ir::*;
                    pub(crate) static REGISTERS: IR = IR {
    blocks: &[
        Block {
            name: "Mdio",
            extends: None,
            description: Some(
                "desc MDIO.",
            ),
            items: &[
                BlockItem {
                    name: "mdcon",
                    description: Some(
                        "desc MDCON.",
                    ),
                    array: None,
                    byte_offset: 0x0,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Mdcon",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "mdcon1",
                    description: Some(
                        "desc MDCON1.",
                    ),
                    array: None,
                    byte_offset: 0x4,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Mdcon1",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "mdfrm",
                    description: Some(
                        "desc MDFRM.",
                    ),
                    array: None,
                    byte_offset: 0x8,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::Read,
                            bit_size: 32,
                            fieldset: Some(
                                "Mdfrm",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "mdrxd",
                    description: Some(
                        "desc MDRXD.",
                    ),
                    array: None,
                    byte_offset: 0xc,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::Read,
                            bit_size: 32,
                            fieldset: Some(
                                "Mdrxd",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "mdadr",
                    description: Some(
                        "desc MDADR.",
                    ),
                    array: None,
                    byte_offset: 0x10,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::Read,
                            bit_size: 32,
                            fieldset: Some(
                                "Mdadr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "mdtxd",
                    description: Some(
                        "desc MDTXD.",
                    ),
                    array: None,
                    byte_offset: 0x14,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Mdtxd",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "mdphy",
                    description: Some(
                        "desc MDPHY.",
                    ),
                    array: None,
                    byte_offset: 0x18,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Mdphy",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "mdsta",
                    description: Some(
                        "desc MDSTA.",
                    ),
                    array: None,
                    byte_offset: 0x1c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Mdsta",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "mdien",
                    description: Some(
                        "desc MDIEN.",
                    ),
                    array: None,
                    byte_offset: 0x20,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Mdien",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "mdpin",
                    description: Some(
                        "desc MDPIN.",
                    ),
                    array: None,
                    byte_offset: 0x24,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::Read,
                            bit_size: 32,
                            fieldset: Some(
                                "Mdpin",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "mdto",
                    description: Some(
                        "desc MDTO.",
                    ),
                    array: None,
                    byte_offset: 0x28,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Mdto",
                            ),
                        },
                    ),
                },
            ],
        },
    ],
    fieldsets: &[
        FieldSet {
            name: "Mdadr",
            extends: None,
            description: Some(
                "desc MDADR.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "md_adr",
                    description: Some(
                        "desc MD_ADR.",
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
            name: "Mdcon",
            extends: None,
            description: Some(
                "desc MDCON.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "md_rst",
                    description: Some(
                        "desc MD_RST.",
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
                    name: "md_phyb",
                    description: Some(
                        "desc MD_PHYB.",
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
            name: "Mdcon1",
            extends: None,
            description: Some(
                "desc MDCON1.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "md_msdev",
                    description: Some(
                        "desc MD_MSDEV.",
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
                    name: "sel_edge",
                    description: Some(
                        "desc SEL_EDGE.",
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
                    name: "md_en",
                    description: Some(
                        "desc MD_EN.",
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
            ],
        },
        FieldSet {
            name: "Mdfrm",
            extends: None,
            description: Some(
                "desc MDFRM.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "md_rop",
                    description: Some(
                        "desc MD_ROP.",
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
                    name: "md_rphyad",
                    description: Some(
                        "desc MD_RPHYAD.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 2,
                        },
                    ),
                    bit_size: 5,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "md_rdevad",
                    description: Some(
                        "desc MD_RDEVAD.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 7,
                        },
                    ),
                    bit_size: 5,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "md_rta",
                    description: Some(
                        "desc MD_RTA.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 12,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Mdien",
            extends: None,
            description: Some(
                "desc MDIEN.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "md_wrfie",
                    description: Some(
                        "desc MD_WRFIE.",
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
                    name: "md_adrfie",
                    description: Some(
                        "desc MD_ADRFIE.",
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
                    name: "md_incfie",
                    description: Some(
                        "desc MD_INCFIE.",
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
                    name: "md_rdfie",
                    description: Some(
                        "desc MD_RDFIE.",
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
                    name: "md_devmie",
                    description: Some(
                        "desc MD_DEVMIE.",
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
                    name: "md_devnie",
                    description: Some(
                        "desc MD_DEVNIE.",
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
                    name: "md_phymie",
                    description: Some(
                        "desc MD_PHYMIE.",
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
                    name: "md_phynie",
                    description: Some(
                        "desc MD_PHYNIE.",
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
                    name: "md_tanmie",
                    description: Some(
                        "desc MD_TANMIE.",
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
                    name: "md_toie",
                    description: Some(
                        "desc MD_TOIE.",
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
                    name: "md_udrie",
                    description: Some(
                        "desc MD_UDRIE.",
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
                    name: "md_ovrie",
                    description: Some(
                        "desc MD_OVRIE.",
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
                    name: "md_rbneie",
                    description: Some(
                        "desc MD_RBNEIE.",
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
            name: "Mdphy",
            extends: None,
            description: Some(
                "desc MDPHY.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "md_physw",
                    description: Some(
                        "desc MD_PHYSW.",
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
                    name: "md_physel",
                    description: Some(
                        "desc MD_PHYSEL.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 5,
                        },
                    ),
                    bit_size: 5,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "md_devadd",
                    description: Some(
                        "desc MD_DEVADD.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 10,
                        },
                    ),
                    bit_size: 5,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Mdpin",
            extends: None,
            description: Some(
                "desc MDPIN.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "md_pin",
                    description: Some(
                        "desc MD_PIN.",
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
            name: "Mdrxd",
            extends: None,
            description: Some(
                "desc MDRXD.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "md_rxd",
                    description: Some(
                        "desc MD_RXD.",
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
            name: "Mdsta",
            extends: None,
            description: Some(
                "desc MDSTA.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "md_wrf",
                    description: Some(
                        "desc MD_WRF.",
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
                    name: "md_adrf",
                    description: Some(
                        "desc MD_ADRF.",
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
                    name: "md_incf",
                    description: Some(
                        "desc MD_INCF.",
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
                    name: "md_rdf",
                    description: Some(
                        "desc MD_RDF.",
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
                    name: "md_devm",
                    description: Some(
                        "desc MD_DEVM.",
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
                    name: "md_devn",
                    description: Some(
                        "desc MD_DEVN.",
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
                    name: "md_phym",
                    description: Some(
                        "desc MD_PHYM.",
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
                    name: "md_phyn",
                    description: Some(
                        "desc MD_PHYN.",
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
                    name: "md_tanm",
                    description: Some(
                        "desc MD_TANM.",
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
                    name: "md_to",
                    description: Some(
                        "desc MD_TO.",
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
                    name: "md_udr",
                    description: Some(
                        "desc MD_UDR.",
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
                    name: "md_ovr",
                    description: Some(
                        "desc MD_OVR.",
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
                    name: "md_rbne",
                    description: Some(
                        "desc MD_RBNE.",
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
                    name: "md_busy",
                    description: Some(
                        "desc MD_BUSY.",
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
            name: "Mdto",
            extends: None,
            description: Some(
                "desc MDTO.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "tocnt",
                    description: Some(
                        "desc TOCNT.",
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
                    name: "toen",
                    description: Some(
                        "desc TOEN.",
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
            name: "Mdtxd",
            extends: None,
            description: Some(
                "desc MDTXD.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "md_txd",
                    description: Some(
                        "desc MD_TXD.",
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
                