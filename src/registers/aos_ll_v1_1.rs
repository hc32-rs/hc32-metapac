
use crate::metadata::ir::*;
pub(crate) static REGISTERS: IR = IR {
    blocks: &[Block {
        name: "Aos",
        extends: None,
        description: Some("desc AOS."),
        items: &[
            BlockItem {
                name: "intc_strgcr",
                description: Some("desc INTC_STRGCR."),
                array: None,
                byte_offset: 0x0,
                inner: BlockItemInner::Register(Register {
                    access: Access::Write,
                    bit_size: 32,
                    fieldset: Some("IntcStrgcr"),
                }),
            },
            BlockItem {
                name: "tmr0_htssr",
                description: Some("desc TMR0_HTSSR."),
                array: None,
                byte_offset: 0x20,
                inner: BlockItemInner::Register(Register {
                    access: Access::ReadWrite,
                    bit_size: 32,
                    fieldset: Some("Tmr0Htssr"),
                }),
            },
            BlockItem {
                name: "tmrb_htssr",
                description: Some("desc TMRB_HTSSR."),
                array: None,
                byte_offset: 0x50,
                inner: BlockItemInner::Register(Register {
                    access: Access::ReadWrite,
                    bit_size: 32,
                    fieldset: Some("TmrbHtssr"),
                }),
            },
            BlockItem {
                name: "adc_itrgselr0",
                description: Some("desc ADC_ITRGSELR0."),
                array: None,
                byte_offset: 0x60,
                inner: BlockItemInner::Register(Register {
                    access: Access::ReadWrite,
                    bit_size: 32,
                    fieldset: Some("AdcItrgselr0"),
                }),
            },
            BlockItem {
                name: "adc_itrgselr1",
                description: Some("desc ADC_ITRGSELR1."),
                array: None,
                byte_offset: 0x64,
                inner: BlockItemInner::Register(Register {
                    access: Access::ReadWrite,
                    bit_size: 32,
                    fieldset: Some("AdcItrgselr1"),
                }),
            },
        ],
    }],
    fieldsets: &[
        FieldSet {
            name: "AdcItrgselr0",
            extends: None,
            description: Some("desc ADC_ITRGSELR0."),
            bit_size: 32,
            fields: &[Field {
                name: "trgsel",
                description: Some("desc TRGSEL."),
                bit_offset: BitOffset::Regular(RegularBitOffset { offset: 0 }),
                bit_size: 7,
                array: None,
                enumm: None,
            }],
        },
        FieldSet {
            name: "AdcItrgselr1",
            extends: None,
            description: Some("desc ADC_ITRGSELR1."),
            bit_size: 32,
            fields: &[Field {
                name: "trgsel",
                description: Some("desc TRGSEL."),
                bit_offset: BitOffset::Regular(RegularBitOffset { offset: 0 }),
                bit_size: 7,
                array: None,
                enumm: None,
            }],
        },
        FieldSet {
            name: "IntcStrgcr",
            extends: None,
            description: Some("desc INTC_STRGCR."),
            bit_size: 32,
            fields: &[Field {
                name: "strg",
                description: Some("desc STRG."),
                bit_offset: BitOffset::Regular(RegularBitOffset { offset: 0 }),
                bit_size: 1,
                array: None,
                enumm: None,
            }],
        },
        FieldSet {
            name: "Tmr0Htssr",
            extends: None,
            description: Some("desc TMR0_HTSSR."),
            bit_size: 32,
            fields: &[Field {
                name: "trgsel",
                description: Some("desc TRGSEL."),
                bit_offset: BitOffset::Regular(RegularBitOffset { offset: 0 }),
                bit_size: 7,
                array: None,
                enumm: None,
            }],
        },
        FieldSet {
            name: "TmrbHtssr",
            extends: None,
            description: Some("desc TMRB_HTSSR."),
            bit_size: 32,
            fields: &[Field {
                name: "trgsel",
                description: Some("desc TRGSEL."),
                bit_offset: BitOffset::Regular(RegularBitOffset { offset: 0 }),
                bit_size: 7,
                array: None,
                enumm: None,
            }],
        },
    ],
    enums: &[],
};
